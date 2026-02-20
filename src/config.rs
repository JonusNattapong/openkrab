use serde::{Deserialize, Serialize};

fn map_openkrab_channel_config(c: &crate::openkrab_config::ChannelConfig) -> ChannelConfig {
    ChannelConfig {
        enabled: c.enabled,
        allowlist: c.allowlist.clone(),
        token: c.token.clone(),
        token_encrypted: c.token_encrypted.clone(),
        webhook_secret: c.webhook_secret.clone(),
        webhook_secret_encrypted: c.webhook_secret_encrypted.clone(),
    }
}

fn map_openkrab_channels(
    channels: Option<&crate::openkrab_config::ChannelsConfig>,
) -> ChannelsConfig {
    let Some(channels) = channels else {
        return ChannelsConfig::default();
    };

    // Telegram: flatten accounts from TelegramConfig into HashMap
    let telegram = channels
        .telegram
        .as_ref()
        .map(|tc| {
            tc.accounts
                .iter()
                .map(|(k, acct)| {
                    (
                        k.clone(),
                        ChannelConfig {
                            enabled: acct.enabled,
                            allowlist: acct.allowlist.clone(),
                            token: acct.token.clone(),
                            token_encrypted: acct.token_encrypted.clone(),
                            webhook_secret: acct.webhook_secret.clone(),
                            webhook_secret_encrypted: acct.webhook_secret_encrypted.clone(),
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // Discord: flatten accounts from DiscordConfig into HashMap
    let discord = channels
        .discord
        .as_ref()
        .map(|dc| {
            dc.accounts
                .iter()
                .map(|(k, acct)| {
                    (
                        k.clone(),
                        ChannelConfig {
                            enabled: acct.enabled,
                            allowlist: acct.allowlist.clone(),
                            token: acct.token.clone(),
                            token_encrypted: acct.token_encrypted.clone(),
                            webhook_secret: None,
                            webhook_secret_encrypted: None,
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    ChannelsConfig {
        telegram,
        slack: channels
            .slack
            .iter()
            .map(|(k, v)| (k.clone(), map_openkrab_channel_config(v)))
            .collect(),
        discord,
        whatsapp: channels
            .whatsapp
            .iter()
            .map(|(k, v)| (k.clone(), map_openkrab_channel_config(v)))
            .collect(),
        accounts: channels
            .accounts
            .iter()
            .map(|(k, v)| (k.clone(), map_openkrab_channel_config(v)))
            .collect(),
    }
}

fn map_openkrab_agents(agents: Option<&crate::openkrab_config::AgentsConfig>) -> AgentsConfig {
    let Some(agents) = agents else {
        return AgentsConfig::default();
    };

    let defaults = agents
        .defaults
        .as_ref()
        .map_or_else(AgentDefaults::default, |d| AgentDefaults {
            model: d.model.as_ref().map(|m| m.primary.clone()),
            sandbox: d
                .sandbox
                .as_ref()
                .map_or_else(SandboxConfig::default, |s| SandboxConfig {
                    mode: Some(s.mode.clone()),
                    ..SandboxConfig::default()
                }),
            memory_search: None,
            tools: std::collections::HashMap::new(),
        });

    AgentsConfig {
        defaults,
        list: Vec::new(),
    }
}

fn map_openkrab_auth(auth: Option<&crate::openkrab_config::AuthConfig>) -> AuthConfig {
    let Some(auth) = auth else {
        return AuthConfig::default();
    };

    let profiles = auth
        .profiles
        .iter()
        .map(|(key, profile)| {
            let value = serde_json::to_value(profile).unwrap_or(serde_json::Value::Null);
            (key.clone(), value)
        })
        .collect();

    AuthConfig { profiles }
}

/// Re-export OpenKrabConfig as the main config type
pub use crate::openkrab_config::OpenKrabConfig;

/// Re-export config I/O functions
pub use crate::config_io::{
    apply_env_substitution, clear_config_cache, get_default_config, load_config, load_config_file,
    load_config_from_path, migrate_legacy_config, read_config_snapshot, resolve_config_path,
    save_config, save_config_to_path, validate_config as validate_openkrab_config,
};

/// Re-export validation functions
pub use crate::config_validation::{
    format_validation_errors, validate_config_json, validate_config_object_raw,
    validate_config_object_with_plugins, validate_config_schema, ValidationError, ValidationResult,
};

/// Convert OpenKrabConfig to AppConfig (for backward compatibility)
pub fn openkrab_to_app_config(openkrab: &OpenKrabConfig) -> AppConfig {
    AppConfig {
        profile: "default".to_string(),
        log_level: openkrab
            .logging
            .as_ref()
            .map(|l| l.level.clone())
            .unwrap_or_else(|| "info".to_string()),
        enable_telegram: openkrab
            .channels
            .as_ref()
            .and_then(|c| c.telegram.as_ref())
            .and_then(|tc| tc.accounts.get("default"))
            .map(|a| a.enabled)
            .unwrap_or(false),
        enable_slack: openkrab
            .channels
            .as_ref()
            .and_then(|c| c.slack.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_discord: openkrab
            .channels
            .as_ref()
            .and_then(|c| c.discord.as_ref())
            .and_then(|dc| dc.accounts.get("default"))
            .map(|a| a.enabled)
            .unwrap_or(false),
        enable_line: false,
        enable_whatsapp: openkrab
            .channels
            .as_ref()
            .and_then(|c| c.whatsapp.get("default"))
            .map(|c| c.enabled)
            .unwrap_or(false),
        enable_dashboard: openkrab.web.as_ref().map(|w| w.enabled).unwrap_or(false),
        dashboard_bind: format!(
            "0.0.0.0:{}",
            openkrab.web.as_ref().and_then(|w| w.port).unwrap_or(3000)
        ),
        memory: crate::memory::config::MemoryConfig::default(),
        agent: crate::agents::AgentIdentity::default(),
        feature_matrix: FeatureMatrix::default(),
        agents: map_openkrab_agents(openkrab.agents.as_ref()),
        auth: map_openkrab_auth(openkrab.auth.as_ref()),
        channels: map_openkrab_channels(openkrab.channels.as_ref()),
        gateway: GatewayConfig::default(),
        logging: LoggingConfig::default(),
    }
}

/// Convert AppConfig to OpenKrabConfig (for forward compatibility)
pub fn app_to_openkrab_config(app: &AppConfig) -> OpenKrabConfig {
    let mut channels = crate::openkrab_config::ChannelsConfig::default();

    let to_openkrab_channel = |cfg: Option<&ChannelConfig>| crate::openkrab_config::ChannelConfig {
        enabled: cfg.map(|c| c.enabled).unwrap_or(true),
        allowlist: cfg.map(|c| c.allowlist.clone()).unwrap_or_default(),
        token: cfg.and_then(|c| c.token.clone()),
        token_encrypted: cfg.and_then(|c| c.token_encrypted.clone()),
        webhook_secret: cfg.and_then(|c| c.webhook_secret.clone()),
        webhook_secret_encrypted: cfg.and_then(|c| c.webhook_secret_encrypted.clone()),
    };

    if app.enable_telegram {
        let acct_cfg = app.channels.telegram.get("default");
        let mut tc = crate::openkrab_config::TelegramConfig::default();
        tc.accounts.insert(
            "default".to_string(),
            crate::openkrab_config::TelegramAccountConfig {
                enabled: acct_cfg.map(|c| c.enabled).unwrap_or(true),
                allowlist: acct_cfg.map(|c| c.allowlist.clone()).unwrap_or_default(),
                token: acct_cfg.and_then(|c| c.token.clone()),
                token_encrypted: acct_cfg.and_then(|c| c.token_encrypted.clone()),
                webhook_secret: acct_cfg.and_then(|c| c.webhook_secret.clone()),
                webhook_secret_encrypted: acct_cfg.and_then(|c| c.webhook_secret_encrypted.clone()),
            },
        );
        channels.telegram = Some(tc);
    }

    if app.enable_slack {
        channels.slack.insert(
            "default".to_string(),
            to_openkrab_channel(app.channels.slack.get("default")),
        );
    }

    if app.enable_discord {
        let acct_cfg = app.channels.discord.get("default");
        let mut dc = crate::openkrab_config::DiscordConfig::default();
        dc.accounts.insert(
            "default".to_string(),
            crate::openkrab_config::DiscordAccountConfig {
                enabled: acct_cfg.map(|c| c.enabled).unwrap_or(true),
                allowlist: acct_cfg.map(|c| c.allowlist.clone()).unwrap_or_default(),
                token: acct_cfg.and_then(|c| c.token.clone()),
                token_encrypted: acct_cfg.and_then(|c| c.token_encrypted.clone()),
            },
        );
        channels.discord = Some(dc);
    }

    if app.enable_whatsapp {
        channels.whatsapp.insert(
            "default".to_string(),
            to_openkrab_channel(app.channels.whatsapp.get("default")),
        );
    }

    let port = app
        .dashboard_bind
        .split(':')
        .nth(1)
        .and_then(|p| p.parse().ok());

    OpenKrabConfig {
        meta: Some(crate::openkrab_config::ConfigMeta {
            last_touched_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            last_touched_at: Some(chrono::Utc::now().to_rfc3339()),
        }),
        logging: Some(crate::openkrab_config::LoggingConfig {
            level: app.log_level.clone(),
            file: None,
            ..Default::default()
        }),
        web: Some(crate::openkrab_config::WebConfig {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureRoute {
    pub primary: RuntimeLayer,
}

impl FeatureRoute {
    pub const fn rust_only() -> Self {
        Self {
            primary: RuntimeLayer::Rust,
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
            browser_automation: FeatureRoute::rust_only(),
            canvas_host: FeatureRoute::rust_only(),
            voice_wake_talk: FeatureRoute::rust_only(),
            macos_native: FeatureRoute::rust_only(),
            node_host: FeatureRoute::rust_only(),
            imessage_native: FeatureRoute::rust_only(),
            whatsapp_full: FeatureRoute::rust_only(),
            line_full: FeatureRoute::rust_only(),
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
    #[serde(default)]
    pub binds: Option<Vec<String>>,
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
    pub token_encrypted: Option<crate::secure::EncryptedValue>,
    #[serde(default)]
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub webhook_secret_encrypted: Option<crate::secure::EncryptedValue>,
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
            agents: AgentsConfig::default(),
            auth: AuthConfig::default(),
            channels: ChannelsConfig::default(),
            gateway: GatewayConfig::default(),
            logging: LoggingConfig::default(),
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
    fn feature_matrix_defaults_to_rust_only() {
        let cfg = AppConfig::default();
        assert_eq!(
            cfg.feature_matrix.browser_automation.primary,
            RuntimeLayer::Rust
        );
        assert_eq!(cfg.feature_matrix.canvas_host.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.voice_wake_talk.primary,
            RuntimeLayer::Rust
        );
        assert_eq!(cfg.feature_matrix.macos_native.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.imessage_native.primary,
            RuntimeLayer::Rust
        );
        assert_eq!(cfg.feature_matrix.node_host.primary, RuntimeLayer::Rust);
        assert_eq!(cfg.feature_matrix.whatsapp_full.primary, RuntimeLayer::Rust);
        assert_eq!(cfg.feature_matrix.line_full.primary, RuntimeLayer::Rust);
    }

    #[test]
    fn empty_profile_is_invalid() {
        let mut cfg = AppConfig::default();
        cfg.profile = "   ".to_string();
        assert!(validate_config(&cfg).is_err());
    }
}
