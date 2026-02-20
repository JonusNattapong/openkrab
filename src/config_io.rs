//! Config I/O — port of `openkrab/src/config/io.ts` (Phase 1-4 config loading)

use crate::openkrab_config::OpenKrabConfig;
use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

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
    let processed_config = process_config_includes(&config)?;

    // Update cache
    let cached = CachedConfig {
        config: processed_config.clone(),
        path: path.to_path_buf(),
        hash: current_hash,
        mtime,
    };
    *CONFIG_CACHE.write().unwrap() = Some(cached);

    Ok(processed_config)
}

/// Load configuration file without caching
pub fn load_config_file(path: &Path) -> Result<OpenKrabConfig> {
    if !path.exists() {
        return Ok(OpenKrabConfig::default());
    }

    let content = fs::read_to_string(path)?;
    let config = parse_config_json5(&content)?;
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
fn parse_config_json5(content: &str) -> Result<OpenKrabConfig> {
    // For now, use JSON parsing. JSON5 support would require additional dependency
    let config: OpenKrabConfig = serde_json::from_str(content)?;
    Ok(config)
}

/// Process config includes (#include directives)
fn process_config_includes(config: &OpenKrabConfig) -> Result<OpenKrabConfig> {
    // TODO: Implement #include processing
    Ok(config.clone())
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
    // This would require shell execution, simplified for now
    // TODO: Implement actual shell env import
    Ok(())
}

/// Apply inline environment variables
fn apply_inline_env_vars(
    config: &mut OpenKrabConfig,
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
    // TODO: Implement legacy migration
    // For now, try to parse as JSON
    parse_config_json5(legacy_content)
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
