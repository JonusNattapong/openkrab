//! Config I/O — port of `openkrab/src/config/io.ts` (Phase 1-4 config loading)

use crate::openkrab_config::OpenKrabConfig;
use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::RwLock;
use std::time::SystemTime;

static CONFIG_CACHE: Lazy<RwLock<Option<CachedConfig>>> = Lazy::new(|| RwLock::new(None));

/// Cached configuration with metadata
#[derive(Debug, Clone)]
struct CachedConfig {
    config: OpenKrabConfig,
    path: PathBuf,
    hash: String,
    mtime: SystemTime,
}

/// Configuration snapshot for read operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    pub config: OpenKrabConfig,
    pub hash: String,
    pub path: PathBuf,
    pub mtime: SystemTime,
}

/// Load configuration from file with caching
pub fn load_config() -> Result<OpenKrabConfig> {
    let path = resolve_config_path()?;
    load_config_from_path(&path)
}

/// Load configuration from specific path with caching
pub fn load_config_from_path(path: &Path) -> Result<OpenKrabConfig> {
    if !path.exists() {
        return Ok(OpenKrabConfig::default());
    }

    let metadata = fs::metadata(path)?;
    let mtime = metadata.modified()?;
    let current_hash = compute_file_hash(path)?;

    // Check cache
    if let Some(cached) = CONFIG_CACHE.read().unwrap().as_ref() {
        if cached.path == path && cached.hash == current_hash && cached.mtime == mtime {
            return Ok(cached.config.clone());
        }
    }

    // Load fresh config
    let config = load_config_file(path)?;

    // Update cache
    let cached = CachedConfig {
        config: config.clone(),
        path: path.to_path_buf(),
        hash: current_hash,
        mtime,
    };
    *CONFIG_CACHE.write().unwrap() = Some(cached);

    Ok(config)
}

/// Load configuration file without caching
pub fn load_config_file(path: &Path) -> Result<OpenKrabConfig> {
    if !path.exists() {
        return Ok(OpenKrabConfig::default());
    }

    let content = fs::read_to_string(&path)?;
    let config = parse_config_json5(&content, Some(path))?;
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &OpenKrabConfig) -> Result<()> {
    let path = resolve_config_path()?;
    save_config_to_path(config, &path)
}

/// Save configuration to specific path
pub fn save_config_to_path(config: &OpenKrabConfig, path: &Path) -> Result<()> {
    // Update metadata
    let mut config_with_meta = config.clone();
    config_with_meta.meta = Some(crate::openkrab_config::ConfigMeta {
        last_touched_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        last_touched_at: Some(chrono::Utc::now().to_rfc3339()),
    });

    // Create parent directories
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write file
    let json = serde_json::to_string_pretty(&config_with_meta)?;
    fs::write(path, json)?;

    // Invalidate cache
    *CONFIG_CACHE.write().unwrap() = None;

    Ok(())
}

/// Get configuration snapshot (read-only)
pub fn read_config_snapshot() -> Result<ConfigSnapshot> {
    let path = resolve_config_path()?;
    let config = load_config_from_path(&path)?;
    let hash = compute_file_hash(&path)?;
    let metadata = fs::metadata(&path)?;
    let mtime = metadata.modified()?;

    Ok(ConfigSnapshot {
        config,
        hash,
        path,
        mtime,
    })
}

/// Clear configuration cache
pub fn clear_config_cache() {
    *CONFIG_CACHE.write().unwrap() = None;
}

/// Resolve configuration file path
pub fn resolve_config_path() -> Result<PathBuf> {
    let home = dirs::config_dir().ok_or_else(|| anyhow!("Could not find config directory"))?;
    Ok(home.join("krabkrab").join("config.json"))
}

/// Parse JSON5 configuration content
fn parse_config_json5(content: &str, base_path: Option<&Path>) -> Result<OpenKrabConfig> {
    let parsed: Value = json5::from_str(content)?;
    let resolved = if let Some(path) = base_path {
        resolve_config_includes(parsed, path)?
    } else {
        parsed
    };
    let config: OpenKrabConfig = serde_json::from_value(resolved)?;
    Ok(config)
}

/// Load configuration as raw JSON value (used for hot-reloading)
pub fn load_config_value() -> Result<serde_json::Value> {
    let path = resolve_config_path()?;
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let content = fs::read_to_string(&path)?;
    let parsed: Value = json5::from_str(&content)?;
    resolve_config_includes(parsed, &path)
}

const INCLUDE_KEY: &str = "$include";
const MAX_INCLUDE_DEPTH: usize = 10;

fn resolve_config_includes(root: Value, config_path: &Path) -> Result<Value> {
    let root_dir = config_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    resolve_includes_recursive(root, config_path, &root_dir, &mut vec![], 0)
}

fn resolve_includes_recursive(
    node: Value,
    current_path: &Path,
    root_dir: &Path,
    visited: &mut Vec<PathBuf>,
    depth: usize,
) -> Result<Value> {
    if depth > MAX_INCLUDE_DEPTH {
        bail!("Maximum include depth exceeded ({})", MAX_INCLUDE_DEPTH);
    }

    match node {
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(resolve_includes_recursive(
                    item,
                    current_path,
                    root_dir,
                    visited,
                    depth,
                )?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(mut obj) => {
            let include_value = obj.remove(INCLUDE_KEY);
            let mut processed_obj = serde_json::Map::new();
            for (k, v) in obj {
                processed_obj.insert(
                    k,
                    resolve_includes_recursive(v, current_path, root_dir, visited, depth)?,
                );
            }

            if let Some(include_value) = include_value {
                let included = resolve_include_value(
                    include_value,
                    current_path,
                    root_dir,
                    visited,
                    depth + 1,
                )?;
                Ok(deep_merge(included, Value::Object(processed_obj)))
            } else {
                Ok(Value::Object(processed_obj))
            }
        }
        other => Ok(other),
    }
}

fn resolve_include_value(
    include_value: Value,
    current_path: &Path,
    root_dir: &Path,
    visited: &mut Vec<PathBuf>,
    depth: usize,
) -> Result<Value> {
    match include_value {
        Value::String(path) => load_included_file(&path, current_path, root_dir, visited, depth),
        Value::Array(items) => {
            let mut merged = Value::Object(serde_json::Map::new());
            for item in items {
                let Value::String(path) = item else {
                    bail!("$include array entries must be strings");
                };
                let included = load_included_file(&path, current_path, root_dir, visited, depth)?;
                merged = deep_merge(merged, included);
            }
            Ok(merged)
        }
        _ => bail!("$include must be a string or an array of strings"),
    }
}

fn load_included_file(
    include_path: &str,
    current_path: &Path,
    root_dir: &Path,
    visited: &mut Vec<PathBuf>,
    depth: usize,
) -> Result<Value> {
    let base_dir = current_path.parent().unwrap_or_else(|| Path::new("."));
    let resolved = if Path::new(include_path).is_absolute() {
        PathBuf::from(include_path)
    } else {
        base_dir.join(include_path)
    };
    let normalized = resolved.clean();

    if !normalized.starts_with(root_dir) {
        bail!(
            "Include path escapes config directory: {} (root: {})",
            normalized.display(),
            root_dir.display()
        );
    }

    if visited.iter().any(|p| p == &normalized) {
        bail!("Circular include detected at {}", normalized.display());
    }

    let raw = fs::read_to_string(&normalized)
        .map_err(|e| anyhow!("Failed to read include {}: {}", normalized.display(), e))?;
    let parsed: Value = json5::from_str(&raw)
        .map_err(|e| anyhow!("Failed to parse include {}: {}", normalized.display(), e))?;

    visited.push(normalized.clone());
    let resolved = resolve_includes_recursive(parsed, &normalized, root_dir, visited, depth)?;
    visited.pop();
    Ok(resolved)
}

fn deep_merge(target: Value, source: Value) -> Value {
    match (target, source) {
        (Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Value::Array(a)
        }
        (Value::Object(mut a), Value::Object(b)) => {
            for (k, v) in b {
                let merged = match a.remove(&k) {
                    Some(existing) => deep_merge(existing, v),
                    None => v,
                };
                a.insert(k, merged);
            }
            Value::Object(a)
        }
        (_, b) => b,
    }
}

/// Compute file hash for cache validation
fn compute_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(hex::encode(hasher.finalize()))
}

/// Apply environment variable substitution
pub fn apply_env_substitution(config: &mut OpenKrabConfig) -> Result<()> {
    // Apply shell env if configured
    if let Some(env_config) = &config.env {
        let shell_env_timeout = env_config
            .shell_env
            .as_ref()
            .and_then(|shell_env| shell_env.enabled.then_some(shell_env.timeout_ms));
        let inline_vars = env_config.vars.clone();

        if let Some(timeout_ms) = shell_env_timeout {
            apply_shell_env(config, timeout_ms)?;
        }

        // Apply inline env vars
        apply_inline_env_vars(config, &inline_vars)?;
    }

    Ok(())
}

/// Apply shell environment variables
fn apply_shell_env(config: &mut OpenKrabConfig, timeout_ms: u64) -> Result<()> {
    let Some(env_cfg) = &config.env else {
        return Ok(());
    };

    let mut expected_keys: Vec<String> = env_cfg.vars.keys().cloned().collect();
    for (k, v) in &env_cfg.extra_vars {
        if v.is_string() {
            expected_keys.push(k.clone());
        }
    }
    expected_keys.sort();
    expected_keys.dedup();

    if expected_keys.is_empty() {
        return Ok(());
    }

    let shell_env = load_shell_env(timeout_ms)?;
    for key in expected_keys {
        if std::env::var(&key).is_ok() {
            continue;
        }
        if let Some(value) = shell_env.get(&key) {
            std::env::set_var(&key, value);
        }
    }

    Ok(())
}

fn load_shell_env(timeout_ms: u64) -> Result<HashMap<String, String>> {
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.args(["/C", "set"]);
        c
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let mut c = Command::new(shell);
        c.args(["-lc", "env"]);
        c
    };

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let started = std::time::Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if started.elapsed().as_millis() as u64 > timeout_ms {
            let _ = child.kill();
            let _ = child.wait();
            bail!("Shell env import timed out after {}ms", timeout_ms);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let mut stdout = String::new();
    if let Some(mut s) = child.stdout.take() {
        let _ = s.read_to_string(&mut stdout);
    }

    let mut out = HashMap::new();
    for line in stdout.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if !k.trim().is_empty() {
                out.insert(k.trim().to_string(), v.to_string());
            }
        }
    }
    Ok(out)
}

/// Apply inline environment variables
fn apply_inline_env_vars(
    _config: &mut OpenKrabConfig,
    vars: &HashMap<String, String>,
) -> Result<()> {
    for (key, value) in vars {
        if std::env::var(key).is_err() {
            std::env::set_var(key, value);
        }
    }
    Ok(())
}

/// Validate configuration
pub fn validate_config(config: &OpenKrabConfig) -> Result<()> {
    // Basic validation
    if let Some(logging) = &config.logging {
        if logging.level.is_empty() {
            bail!("logging.level cannot be empty");
        }
    }

    if let Some(gateway) = &config.gateway {
        if let Some(port) = gateway.port {
            if port == 0 {
                bail!("gateway.port cannot be 0");
            }
        }
    }

    Ok(())
}

/// Get default configuration values
pub fn get_default_config() -> OpenKrabConfig {
    OpenKrabConfig {
        gateway: Some(crate::openkrab_config::GatewayConfig {
            enabled: true,
            port: Some(18789),
            bind_address: Some("127.0.0.1".to_string()),
        }),
        logging: Some(crate::openkrab_config::LoggingConfig {
            level: "info".to_string(),
            file: None,
            ..Default::default()
        }),
        diagnostics: Some(crate::openkrab_config::DiagnosticsConfig { enabled: true }),
        ..Default::default()
    }
}

/// Migrate legacy configuration format
pub fn migrate_legacy_config(legacy_content: &str) -> Result<OpenKrabConfig> {
    let raw: Value = json5::from_str(legacy_content)?;

    if let Ok(cfg) = serde_json::from_value::<OpenKrabConfig>(raw.clone()) {
        return Ok(cfg);
    }

    if let Ok(app_cfg) = serde_json::from_value::<crate::config::AppConfig>(raw.clone()) {
        return Ok(crate::config::app_to_openkrab_config(&app_cfg));
    }

    let mut app_cfg = crate::config::AppConfig::default();
    if let Value::Object(obj) = raw {
        if let Some(v) = obj.get("profile").and_then(|v| v.as_str()) {
            app_cfg.profile = v.to_string();
        }
        if let Some(v) = obj.get("log_level").and_then(|v| v.as_str()) {
            app_cfg.log_level = v.to_string();
        }
        if let Some(v) = obj.get("enable_telegram").and_then(|v| v.as_bool()) {
            app_cfg.enable_telegram = v;
        }
        if let Some(v) = obj.get("enable_slack").and_then(|v| v.as_bool()) {
            app_cfg.enable_slack = v;
        }
        if let Some(v) = obj.get("enable_discord").and_then(|v| v.as_bool()) {
            app_cfg.enable_discord = v;
        }
        if let Some(v) = obj.get("enable_line").and_then(|v| v.as_bool()) {
            app_cfg.enable_line = v;
        }
        if let Some(v) = obj.get("enable_whatsapp").and_then(|v| v.as_bool()) {
            app_cfg.enable_whatsapp = v;
        }
        if let Some(v) = obj.get("enable_dashboard").and_then(|v| v.as_bool()) {
            app_cfg.enable_dashboard = v;
        }
        if let Some(v) = obj.get("dashboard_bind").and_then(|v| v.as_str()) {
            app_cfg.dashboard_bind = v.to_string();
        }
        return Ok(crate::config::app_to_openkrab_config(&app_cfg));
    }

    bail!("Unrecognized legacy config format")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn load_nonexistent_config_returns_default() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let config = load_config_file(&config_path).unwrap();
        assert!(config.meta.is_none());
    }

    #[test]
    fn save_and_load_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_path = temp_file.path();

        let mut config = OpenKrabConfig::default();
        config.gateway = Some(crate::openkrab_config::GatewayConfig {
            enabled: true,
            port: Some(8080),
            bind_address: Some("localhost".to_string()),
        });

        save_config_to_path(&config, config_path).unwrap();
        let loaded = load_config_file(config_path).unwrap();

        assert_eq!(loaded.gateway.as_ref().unwrap().port, Some(8080));
    }

    #[test]
    fn config_hash_changes_on_content_change() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_path = temp_file.path();

        let config1 = OpenKrabConfig::default();
        save_config_to_path(&config1, config_path).unwrap();
        let hash1 = compute_file_hash(config_path).unwrap();

        let mut config2 = OpenKrabConfig::default();
        config2.gateway = Some(crate::openkrab_config::GatewayConfig {
            enabled: true,
            port: Some(9000),
            bind_address: None,
        });
        save_config_to_path(&config2, config_path).unwrap();
        let hash2 = compute_file_hash(config_path).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn validate_config_basic_checks() {
        let mut config = OpenKrabConfig::default();
        config.logging = Some(crate::openkrab_config::LoggingConfig {
            level: "".to_string(),
            file: None,
        });

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn get_default_config_has_gateway() {
        let config = get_default_config();
        assert!(config.gateway.is_some());
        assert_eq!(config.gateway.as_ref().unwrap().port, Some(18789));
    }
}
