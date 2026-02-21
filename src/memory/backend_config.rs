use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackendConfig {
    pub backend: String,   // "builtin" | "qmd"
    pub citations: String, // "auto" | "never" | "always"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmd: Option<QmdConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmdCollection {
    pub name: String,
    pub path: String,
    pub pattern: String,
    pub kind: String, // "memory" | "custom" | "sessions"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmdUpdateConfig {
    pub interval_ms: u64,
    pub debounce_ms: u64,
    pub on_boot: bool,
    pub wait_for_boot_sync: bool,
    pub embed_interval_ms: u64,
    pub command_timeout_ms: u64,
    pub update_timeout_ms: u64,
    pub embed_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmdLimitsConfig {
    pub max_results: usize,
    pub max_snippet_chars: usize,
    pub max_injected_chars: usize,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmdSessionConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QmdConfig {
    pub command: String,
    pub search_mode: String, // "search" | "vsearch" | "query"
    pub collections: Vec<QmdCollection>,
    pub sessions: QmdSessionConfig,
    pub update: QmdUpdateConfig,
    pub limits: QmdLimitsConfig,
    pub include_default_memory: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<SessionSendPolicyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSendPolicyConfig {
    pub default: String, // "allow" | "deny"
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub action: String, // "allow" | "deny"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
}

pub type ResolvedMemoryBackendConfig = MemoryBackendConfig;
pub type ResolvedQmdConfig = QmdConfig;
pub type ResolvedQmdCollection = QmdCollection;
pub type ResolvedQmdUpdateConfig = QmdUpdateConfig;
pub type ResolvedQmdLimitsConfig = QmdLimitsConfig;
pub type ResolvedQmdSessionConfig = QmdSessionConfig;

const DEFAULT_BACKEND: &str = "builtin";
const DEFAULT_CITATIONS: &str = "auto";
const DEFAULT_QMD_INTERVAL_MS: u64 = 5 * 60 * 1000; // 5 minutes
const DEFAULT_QMD_DEBOUNCE_MS: u64 = 15_000;
const DEFAULT_QMD_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_QMD_SEARCH_MODE: &str = "search";
const DEFAULT_QMD_EMBED_INTERVAL_MS: u64 = 60 * 60 * 1000; // 60 minutes
const DEFAULT_QMD_COMMAND_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_QMD_UPDATE_TIMEOUT_MS: u64 = 120_000;
const DEFAULT_QMD_EMBED_TIMEOUT_MS: u64 = 120_000;
const DEFAULT_QMD_LIMITS: QmdLimitsConfig = QmdLimitsConfig {
    max_results: 6,
    max_snippet_chars: 700,
    max_injected_chars: 4_000,
    timeout_ms: DEFAULT_QMD_TIMEOUT_MS,
};

fn default_qmd_scope() -> SessionSendPolicyConfig {
    SessionSendPolicyConfig {
        default: "deny".to_string(),
        rules: vec![PolicyRule {
            action: "allow".to_string(),
            chat_type: Some("direct".to_string()),
            channel: None,
            session_key: None,
        }],
    }
}

fn sanitize_name(input: &str) -> String {
    let lower = input
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
    let trimmed = lower.trim_matches('_');
    if trimmed.is_empty() {
        "collection".to_string()
    } else {
        trimmed.to_string()
    }
}

fn scope_collection_base(base: &str, agent_id: &str) -> String {
    format!("{}-{}", base, sanitize_name(agent_id))
}

fn ensure_unique_name(base: &str, existing: &mut HashMap<String, ()>) -> String {
    let name = sanitize_name(base);
    if !existing.contains_key(&name) {
        existing.insert(name.clone(), ());
        return name;
    }
    for suffix in 2.. {
        let unique = format!("{}-{}", name, suffix);
        if !existing.contains_key(&unique) {
            existing.insert(unique.clone(), ());
            return unique;
        }
    }
    unreachable!()
}

fn resolve_path(raw: &str, workspace_dir: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("path required".to_string());
    }

    if trimmed.starts_with('~') {
        let home = dirs::home_dir().ok_or_else(|| "home directory not found".to_string())?;
        let suffix = trimmed
            .trim_start_matches('~')
            .trim_start_matches(['/', '\\']);
        let expanded = if suffix.is_empty() {
            home
        } else {
            home.join(suffix)
        };
        return Ok(expanded.to_string_lossy().to_string());
    }

    if std::path::Path::new(trimmed).is_absolute() {
        return Ok(trimmed.to_string());
    }

    let resolved = std::path::Path::new(workspace_dir).join(trimmed);
    Ok(resolved.to_string_lossy().to_string())
}

fn resolve_interval_ms(raw: Option<String>) -> u64 {
    let value = raw.as_deref().unwrap_or("").trim();
    if value.is_empty() {
        return DEFAULT_QMD_INTERVAL_MS;
    }
    parse_duration_ms(value).unwrap_or(DEFAULT_QMD_INTERVAL_MS)
}

fn resolve_embed_interval_ms(raw: Option<String>) -> u64 {
    let value = raw.as_deref().unwrap_or("").trim();
    if value.is_empty() {
        return DEFAULT_QMD_EMBED_INTERVAL_MS;
    }
    parse_duration_ms(value).unwrap_or(DEFAULT_QMD_EMBED_INTERVAL_MS)
}

fn parse_duration_ms(input: &str) -> Option<u64> {
    let s = input.trim().to_lowercase();
    if s.is_empty() {
        return None;
    }

    if let Ok(ms) = s.parse::<u64>() {
        return Some(ms);
    }

    let split_idx = s.find(|c: char| !c.is_ascii_digit())?;
    let (num_part, unit_part) = s.split_at(split_idx);
    let n = num_part.parse::<u64>().ok()?;
    let factor = match unit_part.trim() {
        "ms" => 1,
        "s" | "sec" | "secs" => 1_000,
        "m" | "min" | "mins" => 60_000,
        "h" | "hr" | "hrs" => 3_600_000,
        "d" | "day" | "days" => 86_400_000,
        _ => return None,
    };
    n.checked_mul(factor)
}

fn resolve_debounce_ms(raw: Option<u64>) -> u64 {
    raw.filter(|&v| v > 0).unwrap_or(DEFAULT_QMD_DEBOUNCE_MS)
}

fn resolve_timeout_ms(raw: Option<u64>, fallback: u64) -> u64 {
    raw.filter(|&v| v > 0).unwrap_or(fallback)
}

fn resolve_limits(raw: Option<QmdLimitsConfig>) -> QmdLimitsConfig {
    let mut parsed = DEFAULT_QMD_LIMITS.clone();
    if let Some(raw) = raw {
        if raw.max_results > 0 {
            parsed.max_results = raw.max_results;
        }
        if raw.max_snippet_chars > 0 {
            parsed.max_snippet_chars = raw.max_snippet_chars;
        }
        if raw.max_injected_chars > 0 {
            parsed.max_injected_chars = raw.max_injected_chars;
        }
        if raw.timeout_ms > 0 {
            parsed.timeout_ms = raw.timeout_ms;
        }
    }
    parsed
}

fn resolve_search_mode(raw: Option<String>) -> String {
    match raw.as_deref() {
        Some("search" | "vsearch" | "query") => raw.unwrap(),
        _ => DEFAULT_QMD_SEARCH_MODE.to_string(),
    }
}

fn resolve_session_config(cfg: Option<QmdSessionConfig>, workspace_dir: &str) -> QmdSessionConfig {
    let enabled = cfg.as_ref().map(|c| c.enabled).unwrap_or(false);
    let export_dir = cfg
        .as_ref()
        .and_then(|c| c.export_dir.as_ref())
        .and_then(|p| resolve_path(p, workspace_dir).ok());
    let retention_days = cfg
        .as_ref()
        .and_then(|c| c.retention_days)
        .filter(|&d| d > 0);

    QmdSessionConfig {
        enabled,
        export_dir,
        retention_days,
    }
}

fn resolve_custom_paths(
    raw_paths: Option<Vec<QmdCollection>>,
    workspace_dir: &str,
    existing: &mut HashMap<String, ()>,
    agent_id: &str,
) -> Vec<QmdCollection> {
    let mut collections = Vec::new();

    if let Some(paths) = raw_paths {
        for (_index, entry) in paths.into_iter().enumerate() {
            let trimmed_path = entry.path.trim();
            if trimmed_path.is_empty() {
                continue;
            }

            let resolved_path = match resolve_path(trimmed_path, workspace_dir) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let pattern = if entry.pattern.trim().is_empty() {
                "**/*.md".to_string()
            } else {
                entry.pattern
            };

            let base_name = scope_collection_base(&entry.name.trim().to_string(), agent_id);
            let name = ensure_unique_name(&base_name, existing);

            collections.push(QmdCollection {
                name,
                path: resolved_path,
                pattern,
                kind: "custom".to_string(),
            });
        }
    }

    collections
}

fn resolve_default_collections(
    include: bool,
    workspace_dir: &str,
    existing: &mut HashMap<String, ()>,
    agent_id: &str,
) -> Vec<QmdCollection> {
    if !include {
        return Vec::new();
    }

    let entries = vec![
        (
            "memory-root",
            workspace_dir.to_string(),
            "MEMORY.md".to_string(),
        ),
        (
            "memory-alt",
            workspace_dir.to_string(),
            "memory.md".to_string(),
        ),
        (
            "memory-dir",
            format!("{}/memory", workspace_dir),
            "**/*.md".to_string(),
        ),
    ];

    entries
        .into_iter()
        .map(|(base, path, pattern)| QmdCollection {
            name: ensure_unique_name(&scope_collection_base(base, agent_id), existing),
            path,
            pattern,
            kind: "memory".to_string(),
        })
        .collect()
}

pub fn resolve_memory_backend_config(
    cfg: Option<MemoryBackendConfig>,
    agent_id: &str,
    workspace_dir: &str,
) -> ResolvedMemoryBackendConfig {
    let config = cfg.unwrap_or_else(|| MemoryBackendConfig {
        backend: DEFAULT_BACKEND.to_string(),
        citations: DEFAULT_CITATIONS.to_string(),
        qmd: None,
    });

    if config.backend != "qmd" {
        return MemoryBackendConfig {
            backend: "builtin".to_string(),
            citations: config.citations,
            qmd: None,
        };
    }

    let qmd_cfg = config.qmd.unwrap_or_else(|| QmdConfig {
        command: "qmd".to_string(),
        search_mode: DEFAULT_QMD_SEARCH_MODE.to_string(),
        collections: Vec::new(),
        sessions: QmdSessionConfig {
            enabled: false,
            export_dir: None,
            retention_days: None,
        },
        update: QmdUpdateConfig {
            interval_ms: DEFAULT_QMD_INTERVAL_MS,
            debounce_ms: DEFAULT_QMD_DEBOUNCE_MS,
            on_boot: true,
            wait_for_boot_sync: false,
            embed_interval_ms: DEFAULT_QMD_EMBED_INTERVAL_MS,
            command_timeout_ms: DEFAULT_QMD_COMMAND_TIMEOUT_MS,
            update_timeout_ms: DEFAULT_QMD_UPDATE_TIMEOUT_MS,
            embed_timeout_ms: DEFAULT_QMD_EMBED_TIMEOUT_MS,
        },
        limits: DEFAULT_QMD_LIMITS.clone(),
        include_default_memory: true,
        scope: Some(default_qmd_scope()),
    });

    let include_default_memory = qmd_cfg.include_default_memory;
    let mut name_set = HashMap::new();

    let collections = [
        resolve_default_collections(
            include_default_memory,
            workspace_dir,
            &mut name_set,
            agent_id,
        ),
        resolve_custom_paths(
            Some(qmd_cfg.collections),
            workspace_dir,
            &mut name_set,
            agent_id,
        ),
    ]
    .concat();

    let resolved_qmd = QmdConfig {
        command: qmd_cfg.command,
        search_mode: resolve_search_mode(Some(qmd_cfg.search_mode)),
        collections,
        sessions: resolve_session_config(Some(qmd_cfg.sessions), workspace_dir),
        update: QmdUpdateConfig {
            interval_ms: resolve_interval_ms(Some(qmd_cfg.update.interval_ms.to_string())),
            debounce_ms: resolve_debounce_ms(Some(qmd_cfg.update.debounce_ms)),
            on_boot: qmd_cfg.update.on_boot,
            wait_for_boot_sync: qmd_cfg.update.wait_for_boot_sync,
            embed_interval_ms: resolve_embed_interval_ms(Some(
                qmd_cfg.update.embed_interval_ms.to_string(),
            )),
            command_timeout_ms: resolve_timeout_ms(
                Some(qmd_cfg.update.command_timeout_ms),
                DEFAULT_QMD_COMMAND_TIMEOUT_MS,
            ),
            update_timeout_ms: resolve_timeout_ms(
                Some(qmd_cfg.update.update_timeout_ms),
                DEFAULT_QMD_UPDATE_TIMEOUT_MS,
            ),
            embed_timeout_ms: resolve_timeout_ms(
                Some(qmd_cfg.update.embed_timeout_ms),
                DEFAULT_QMD_EMBED_TIMEOUT_MS,
            ),
        },
        limits: resolve_limits(Some(qmd_cfg.limits)),
        include_default_memory,
        scope: qmd_cfg.scope,
    };

    MemoryBackendConfig {
        backend: "qmd".to_string(),
        citations: config.citations,
        qmd: Some(resolved_qmd),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("test file"), "test_file");
        assert_eq!(sanitize_name("../../../etc"), "etc");
        assert_eq!(sanitize_name(""), "collection");
    }

    #[test]
    fn test_scope_collection_base() {
        assert_eq!(scope_collection_base("memory", "agent1"), "memory-agent1");
    }

    #[test]
    fn test_ensure_unique_name() {
        let mut existing = HashMap::new();
        assert_eq!(ensure_unique_name("test", &mut existing), "test");
        assert_eq!(ensure_unique_name("test", &mut existing), "test-2");
    }

    #[test]
    fn test_resolve_search_mode() {
        assert_eq!(resolve_search_mode(Some("query".to_string())), "query");
        assert_eq!(resolve_search_mode(Some("invalid".to_string())), "search");
        assert_eq!(resolve_search_mode(None), "search");
    }

    #[test]
    fn test_parse_duration_ms() {
        assert_eq!(parse_duration_ms("5000"), Some(5000));
        assert_eq!(parse_duration_ms("5m"), Some(300000));
        assert_eq!(parse_duration_ms("60s"), Some(60000));
        assert_eq!(parse_duration_ms("2h"), Some(7200000));
        assert_eq!(parse_duration_ms("invalid"), None);
    }
}
