//! Config command - View and edit configuration

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub agent: AgentConfig,
    pub channels: ChannelsConfig,
    pub providers: ProvidersConfig,
    pub gateway: GatewayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentConfig {
    pub model: Option<String>,
    pub workspace: Option<String>,
    pub agent_dir: Option<String>,
    pub list: Vec<AgentDefinition>,
    pub subagents: SubagentsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentDefinition {
    pub name: String,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub tools: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubagentsConfig {
    pub enabled: bool,
    pub max_concurrent: Option<u32>,
    pub agents: Vec<AgentDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelsConfig {
    pub telegram: ChannelConfig,
    pub slack: ChannelConfig,
    pub discord: ChannelConfig,
    pub signal: ChannelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelConfig {
    pub enabled: Option<bool>,
    pub bot_token: Option<String>,
    pub bot_token_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersConfig {
    pub openai: ProviderConfig,
    pub anthropic: ProviderConfig,
    pub gemini: ProviderConfig,
    pub ollama: ProviderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GatewayConfig {
    pub port: Option<u16>,
    pub bind: Option<String>,
}

fn get_config_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("krabkrab.toml")
}

pub fn config_show_command() -> String {
    let config_path = get_config_path();

    if !config_path.exists() {
        return r#"# No config file found. Create one at:
# ~/.config/krabkrab/krabkrab.toml
#
# Example configuration:
# 
# [agent]
# model = "anthropic/claude-opus-4-6"
#
# [channels.telegram]
# enabled = true
# bot_token = "your-bot-token"
#
# [providers.openai]
# api_key = "sk-..."
"#
        .to_string();
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => format!("Failed to read config: {}", e),
    }
}

pub fn config_get_command(key: &str) -> String {
    let config_path = get_config_path();

    if !config_path.exists() {
        return format!("Config file not found: {}", config_path.display());
    }

    // Simple key lookup (supports dot notation like "agent.model")
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with(&format!("{} ", key))
                    || trimmed.starts_with(&format!("{}=", key))
                    || trimmed == key
                {
                    return line.to_string();
                }
            }
            format!("Key '{}' not found", key)
        }
        Err(e) => format!("Failed to read config: {}", e),
    }
}

pub fn config_set_command(key: &str, value: &str) -> String {
    let config_path = get_config_path();

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let content = if config_path.exists() {
        fs::read_to_string(&config_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Simple key-value update
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut found = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with(&format!("{} ", key))
            || line.trim().starts_with(&format!("{}=", key))
        {
            *line = format!("{} = \"{}\"", key, value);
            found = true;
            break;
        }
    }

    if !found {
        lines.push(format!("{} = \"{}\"", key, value));
    }

    match fs::write(&config_path, lines.join("\n")) {
        Ok(_) => format!("Set {} = \"{}\"", key, value),
        Err(e) => format!("Failed to write config: {}", e),
    }
}

pub fn config_edit_command() -> String {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !config_path.exists() {
        let default_config = r#"[agent]
model = "anthropic/claude-opus-4-6"

[channels.telegram]
enabled = true

[channels.slack]
enabled = false

[channels.discord]
enabled = false

[providers.openai]
# api_key = "sk-..."

[providers.anthropic]
# api_key = "sk-ant-..."

[providers.gemini]
# api_key = "..."

[providers.ollama]
base_url = "http://localhost:11434"
"#;
        let _ = fs::write(&config_path, default_config);
    }

    // Open in editor
    if let Ok(editor) = std::env::var("EDITOR") {
        let _ = std::process::Command::new(editor)
            .arg(&config_path)
            .status();
        format!("Opened {} in editor", config_path.display())
    } else {
        format!(
            "Config file: {}\nSet $EDITOR to open in your preferred editor",
            config_path.display()
        )
    }
}
