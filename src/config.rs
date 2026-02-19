use serde::{Deserialize, Serialize};

/// Re-export OpenClawConfig as the main config type
pub use crate::openclaw_config::OpenClawConfig;

/// Re-export config I/O functions
pub use crate::config_io::{
    apply_env_substitution, clear_config_cache, get_default_config, load_config, load_config_file,
    load_config_from_path, migrate_legacy_config, read_config_snapshot, resolve_config_path,
    save_config, save_config_to_path, validate_config as validate_openclaw_config,
};

/// Re-export validation functions
pub use crate::config_validation::{
    format_validation_errors, validate_config_json, validate_config_object_raw,
    validate_config_object_with_plugins, validate_config_schema, ValidationError, ValidationResult,
};

/// Convert OpenClawConfig to AppConfig (for backward compatibility)
pub fn openclaw_to_app_config(openclaw: &OpenClawConfig) -> AppConfig {
    AppConfig {
        profile: "default".to_string(), // TODO: derive from openclaw config
        log_level: openclaw
            .logging
            .as_ref()
            .map(|l| l.level.clone())
            .unwrap_or_else(|| "info".to_string()),
        enable_telegram: openclaw
            .channels
            .as_ref()
            .and_then(|c| c.telegram.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_slack: openclaw
            .channels
            .as_ref()
            .and_then(|c| c.slack.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_discord: openclaw
            .channels
            .as_ref()
            .and_then(|c| c.discord.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_line: false, // TODO: add to OpenClawConfig
        enable_whatsapp: openclaw
            .channels
            .as_ref()
            .and_then(|c| c.whatsapp.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_dashboard: openclaw.web.as_ref().map(|w| w.enabled).unwrap_or(false),
        dashboard_bind: format!(
            "0.0.0.0:{}",
            openclaw.web.as_ref().and_then(|w| w.port).unwrap_or(3000)
        ),
        memory: crate::memory::config::MemoryConfig::default(), // TODO: convert from openclaw
        agent: crate::agents::AgentIdentity::default(),         // TODO: convert from openclaw
        feature_matrix: FeatureMatrix::default(),
    }
}

/// Convert AppConfig to OpenClawConfig (for forward compatibility)
pub fn app_to_openclaw_config(app: &AppConfig) -> OpenClawConfig {
    let mut channels = crate::openclaw_config::ChannelsConfig::default();

    if app.enable_telegram {
        channels.telegram.insert(
            "default".to_string(),
            crate::openclaw_config::ChannelConfig {
                enabled: true,
                allowlist: vec![],
                token: None,
                webhook_secret: None,
            },
        );
    }

    if app.enable_slack {
        channels.slack.insert(
            "default".to_string(),
            crate::openclaw_config::ChannelConfig {
                enabled: true,
                allowlist: vec![],
                token: None,
                webhook_secret: None,
            },
        );
    }

    if app.enable_discord {
        channels.discord.insert(
            "default".to_string(),
            crate::openclaw_config::ChannelConfig {
                enabled: true,
                allowlist: vec![],
                token: None,
                webhook_secret: None,
            },
        );
    }

    if app.enable_whatsapp {
        channels.whatsapp.insert(
            "default".to_string(),
            crate::openclaw_config::ChannelConfig {
                enabled: true,
                allowlist: vec![],
                token: None,
                webhook_secret: None,
            },
        );
    }

    let port = app
        .dashboard_bind
        .split(':')
        .nth(1)
        .and_then(|p| p.parse().ok());

    OpenClawConfig {
        meta: Some(crate::openclaw_config::ConfigMeta {
            last_touched_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            last_touched_at: Some(chrono::Utc::now().to_rfc3339()),
        }),
        logging: Some(crate::openclaw_config::LoggingConfig {
            level: app.log_level.clone(),
            file: None,
        }),
        web: Some(crate::openclaw_config::WebConfig {
            enabled: app.enable_dashboard,
            port,
        }),
        channels: Some(channels),
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeLayer {
    Rust,
    Js,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureRoute {
    pub primary: RuntimeLayer,
    pub fallback: Option<RuntimeLayer>,
}

impl FeatureRoute {
    pub const fn rust_only() -> Self {
        Self {
            primary: RuntimeLayer::Rust,
            fallback: None,
        }
    }

    pub const fn js_only() -> Self {
        Self {
            primary: RuntimeLayer::Js,
            fallback: None,
        }
    }

    pub const fn rust_then_js() -> Self {
        Self {
            primary: RuntimeLayer::Rust,
            fallback: Some(RuntimeLayer::Js),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureMatrix {
    pub browser_automation: FeatureRoute,
    pub canvas_host: FeatureRoute,
    pub voice_wake_talk: FeatureRoute,
    pub macos_native: FeatureRoute,
    pub node_host: FeatureRoute,
    pub imessage_native: FeatureRoute,
    pub whatsapp_full: FeatureRoute,
    pub line_full: FeatureRoute,
}

impl Default for FeatureMatrix {
    fn default() -> Self {
        Self {
            browser_automation: FeatureRoute::js_only(),
            canvas_host: FeatureRoute::rust_then_js(),
            voice_wake_talk: FeatureRoute::rust_then_js(),
            macos_native: FeatureRoute::js_only(),
            node_host: FeatureRoute::rust_then_js(),
            imessage_native: FeatureRoute::js_only(),
            whatsapp_full: FeatureRoute::rust_then_js(),
            line_full: FeatureRoute::rust_then_js(),
        }
    }
}

impl FeatureMatrix {
    pub fn route_for(&self, feature: &str) -> Option<&FeatureRoute> {
        match feature {
            "browser" | "browser_automation" => Some(&self.browser_automation),
            "canvas" | "canvas_host" => Some(&self.canvas_host),
            "voice" | "voice_wake_talk" | "wake_talk" => Some(&self.voice_wake_talk),
            "macos" | "macos_native" => Some(&self.macos_native),
            "node_host" | "node" => Some(&self.node_host),
            "imessage" | "imessage_native" => Some(&self.imessage_native),
            "whatsapp" | "whatsapp_full" => Some(&self.whatsapp_full),
            "line" | "line_full" => Some(&self.line_full),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub profile: String,
    pub log_level: String,
    // ── Connector toggles ────────────────────────────────────────────────────
    pub enable_telegram: bool,
    pub enable_slack: bool,
    pub enable_discord: bool,
    pub enable_line: bool,
    pub enable_whatsapp: bool,
    // ── Web Dashboard ───────────────────────────────────────────────────────
    pub enable_dashboard: bool,
    pub dashboard_bind: String,
    // ── Sub-configs ──────────────────────────────────────────────────────────
    pub memory: crate::memory::config::MemoryConfig,
    pub agent: crate::agents::AgentIdentity,
    pub feature_matrix: FeatureMatrix,
    // ── Legacy/OpenClaw fields ───────────────────────────────────────────
    #[serde(default)]
    pub agents: AgentsConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub channels: ChannelsConfig,
    #[serde(default)]
    pub gateway: GatewayConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgentsConfig {
    #[serde(default)]
    pub defaults: AgentDefaults,
    #[serde(default)]
    pub list: Vec<AgentInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgentDefaults {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub sandbox: SandboxConfig,
    #[serde(default)]
    pub memory_search: Option<MemorySearchConfig>,
    #[serde(default)]
    pub tools: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgentInstance {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SandboxConfig {
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub docker: DockerConfig,
    #[serde(default)]
    pub binds: Option<Vec<String>>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub seccomp_profile: Option<String>,
    #[serde(default)]
    pub apparmor_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DockerConfig {
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub browser: Option<BrowserConfig>,
    #[serde(default)]
    pub common: Option<CommonConfig>,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub seccomp_profile: Option<String>,
    #[serde(default)]
    pub apparmor_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BrowserConfig {
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonConfig {
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MemorySearchConfig {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub max_results: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub profiles: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChannelsConfig {
    #[serde(default)]
    pub telegram: std::collections::HashMap<String, ChannelConfig>,
    #[serde(default)]
    pub slack: std::collections::HashMap<String, ChannelConfig>,
    #[serde(default)]
    pub discord: std::collections::HashMap<String, ChannelConfig>,
    #[serde(default)]
    pub whatsapp: std::collections::HashMap<String, ChannelConfig>,
    #[serde(default)]
    pub accounts: std::collections::HashMap<String, ChannelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChannelConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub allowlist: Vec<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub webhook_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GatewayConfig {
    #[serde(default)]
    pub bind: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub directory: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            log_level: "info".to_string(),
            enable_telegram: true,
            enable_slack: true,
            enable_discord: true,
            enable_line: true,
            enable_whatsapp: true,
            enable_dashboard: true,
            dashboard_bind: "0.0.0.0:3000".to_string(),
            memory: crate::memory::config::MemoryConfig::default(),
            agent: crate::agents::AgentIdentity::default(),
            feature_matrix: FeatureMatrix::default(),
        }
    }
}

/// Get the default config file path
pub fn config_path() -> anyhow::Result<std::path::PathBuf> {
    resolve_config_path()
}

pub fn validate_config(cfg: &AppConfig) -> Result<(), String> {
    if cfg.profile.trim().is_empty() {
        return Err("profile must not be empty".to_string());
    }
    if cfg.log_level.trim().is_empty() {
        return Err("log_level must not be empty".to_string());
    }
    let all = [
        &cfg.feature_matrix.browser_automation,
        &cfg.feature_matrix.canvas_host,
        &cfg.feature_matrix.voice_wake_talk,
        &cfg.feature_matrix.macos_native,
        &cfg.feature_matrix.node_host,
        &cfg.feature_matrix.imessage_native,
        &cfg.feature_matrix.whatsapp_full,
        &cfg.feature_matrix.line_full,
    ];
    for route in all {
        if route.fallback == Some(route.primary) {
            return Err("feature_matrix fallback must differ from primary".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = AppConfig::default();
        assert!(validate_config(&cfg).is_ok());
    }

    #[test]
    fn default_has_all_connectors_enabled() {
        let cfg = AppConfig::default();
        assert!(cfg.enable_telegram);
        assert!(cfg.enable_slack);
        assert!(cfg.enable_discord);
        assert!(cfg.enable_line);
        assert!(cfg.enable_whatsapp);
        assert!(cfg.enable_dashboard);
    }

    #[test]
    fn feature_matrix_defaults_to_js_for_non_ported_areas() {
        let cfg = AppConfig::default();
        assert_eq!(
            cfg.feature_matrix.browser_automation.primary,
            RuntimeLayer::Js
        );
        assert_eq!(cfg.feature_matrix.canvas_host.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.voice_wake_talk.primary,
            RuntimeLayer::Rust
        );
        assert_eq!(cfg.feature_matrix.node_host.primary, RuntimeLayer::Rust);
        assert_eq!(cfg.feature_matrix.whatsapp_full.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.whatsapp_full.fallback,
            Some(RuntimeLayer::Js)
        );
    }

    #[test]
    fn invalid_matrix_rejects_same_primary_and_fallback() {
        let mut cfg = AppConfig::default();
        cfg.feature_matrix.browser_automation.fallback = Some(RuntimeLayer::Js);
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn empty_profile_is_invalid() {
        let mut cfg = AppConfig::default();
        cfg.profile = "   ".to_string();
        assert!(validate_config(&cfg).is_err());
    }
}
