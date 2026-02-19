//! OpenClawConfig — port of `openclaw/src/config/types.openclaw.ts` (Phase 1-4 core config)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main OpenClaw configuration structure (equivalent to TypeScript OpenClawConfig)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenClawConfig {
    /// Metadata about config version and last modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ConfigMeta>,

    /// Authentication configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,

    /// Environment variable configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvConfig>,

    /// Wizard/onboarding state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wizard: Option<WizardConfig>,

    /// Diagnostics configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsConfig>,

    /// Logging configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingConfig>,

    /// Update configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateConfig>,

    /// Browser automation configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserConfig>,

    /// UI configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<UiConfig>,

    /// Skills configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<SkillsConfig>,

    /// Plugins configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<PluginsConfig>,

    /// Models/LLM configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<ModelsConfig>,

    /// Node host configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_host: Option<NodeHostConfig>,

    /// Agents configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<AgentsConfig>,

    /// Tools configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsConfig>,

    /// Agent bindings
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub bindings: Vec<AgentBinding>,

    /// Broadcast configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<BroadcastConfig>,

    /// Audio configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,

    /// Messages configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<MessagesConfig>,

    /// Commands configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<CommandsConfig>,

    /// Approvals configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals: Option<ApprovalsConfig>,

    /// Session configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionConfig>,

    /// Web configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebConfig>,

    /// Channels configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<ChannelsConfig>,

    /// Cron configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<CronConfig>,

    /// Hooks configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<HooksConfig>,

    /// Discovery configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<DiscoveryConfig>,

    /// Canvas host configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_host: Option<CanvasHostConfig>,

    /// Talk configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talk: Option<TalkConfig>,

    /// Gateway configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<GatewayConfig>,

    /// Memory configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryConfig>,
}

/// Config metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMeta {
    /// Last OpenClaw version that wrote this config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touched_version: Option<String>,
    /// ISO timestamp when this config was last written
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touched_at: Option<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// Authentication profiles
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub profiles: HashMap<String, AuthProfile>,
}

/// Authentication profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProfile {
    /// Profile ID
    pub id: String,
    /// Provider type
    pub provider: String,
    /// Credential data
    pub credential: Credential,
}

/// Credential types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Credential {
    #[serde(rename = "token")]
    Token { token: String },
    #[serde(rename = "oauth")]
    OAuth {
        access: String,
        refresh: Option<String>,
        expires: Option<u64>,
    },
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvConfig {
    /// Shell environment import
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_env: Option<ShellEnvConfig>,
    /// Inline environment variables
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub vars: HashMap<String, String>,
    /// Allow direct env vars under env
    #[serde(flatten)]
    pub extra_vars: HashMap<String, serde_json::Value>,
}

/// Shell environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellEnvConfig {
    /// Whether enabled
    #[serde(default)]
    pub enabled: bool,
    /// Timeout in milliseconds
    #[serde(default = "default_shell_env_timeout")]
    pub timeout_ms: u64,
}

fn default_shell_env_timeout() -> u64 {
    15000
}

/// Wizard/onboarding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_mode: Option<String>,
}

/// Diagnostics configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticsConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

/// Update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_on_start: Option<bool>,
}

/// Browser configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seam_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant: Option<AssistantUiConfig>,
}

/// Assistant UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantUiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// Skills configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillsConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Plugins configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_dirs: Option<Vec<String>>,
}

/// Models/LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<HashMap<String, ProviderConfig>>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub aliases: HashMap<String, String>,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

/// Node host configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeHostConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Agents configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<AgentDefaults>,
}

/// Agent defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelSelection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<SandboxConfig>,
}

/// Model selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelection {
    pub primary: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fallbacks: Vec<String>,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub mode: String,
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Agent binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBinding {
    pub channel: String,
    pub agent: String,
}

/// Broadcast configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Messages configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

/// Commands configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandsConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Approvals configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalsConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_history: Option<u64>,
}

/// Web configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

/// Channels configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub telegram: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub slack: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub discord: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub whatsapp: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub signal: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub imessage: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub irc: HashMap<String, ChannelConfig>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub web: HashMap<String, ChannelConfig>,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allowlist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

/// Cron configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CronConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Hooks configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HooksConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Canvas host configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanvasHostConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Talk configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TalkConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_address: Option<String>,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_store: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openclaw_config_default() {
        let cfg = OpenClawConfig::default();
        assert!(cfg.meta.is_none());
        assert!(cfg.auth.is_none());
    }

    #[test]
    fn openclaw_config_serialize() {
        let cfg = OpenClawConfig {
            meta: Some(ConfigMeta {
                last_touched_version: Some("1.0.0".to_string()),
                last_touched_at: Some("2024-01-01T00:00:00Z".to_string()),
            }),
            gateway: Some(GatewayConfig {
                enabled: true,
                port: Some(18789),
                bind_address: Some("0.0.0.0".to_string()),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&cfg).unwrap();
        assert!(json.contains("last_touched_version"));
        assert!(json.contains("gateway"));
        assert!(json.contains("18789"));
    }

    #[test]
    fn channels_config_telegram() {
        let mut channels = ChannelsConfig::default();
        channels.telegram.insert(
            "default".to_string(),
            ChannelConfig {
                enabled: true,
                token: Some("bot_token".to_string()),
                allowlist: vec!["user1".to_string()],
                webhook_secret: None,
            },
        );

        assert!(channels.telegram.contains_key("default"));
        assert_eq!(
            channels.telegram["default"].token.as_deref(),
            Some("bot_token")
        );
    }
}
