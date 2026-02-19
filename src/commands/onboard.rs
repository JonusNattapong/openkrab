//! Wizard Onboarding Module
//! 
//! Provides an interactive step-by-step wizard for first-time users to configure
//! their krabkrab assistant, including agent identity, channels, memory, and providers.

use dialoguer::{Input, Select, MultiSelect, Confirm, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Onboarding configuration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingConfig {
    pub profile: String,
    pub agent: AgentConfig,
    pub channels: ChannelsConfig,
    pub memory: MemoryOnboardConfig,
    pub llm: LlmConfig,
    pub dashboard: DashboardConfig,
    pub config_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub emoji: String,
    pub personality: String,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    pub enabled_channels: Vec<String>,
    pub telegram_token: Option<String>,
    pub slack_token: Option<String>,
    pub discord_token: Option<String>,
    pub line_token: Option<String>,
    pub whatsapp_phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOnboardConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub bind: String,
}

impl Default for OnboardingConfig {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            agent: AgentConfig {
                name: "krabkrab".to_string(),
                emoji: "🦀".to_string(),
                personality: "A helpful and precise AI assistant.".to_string(),
                system_prompt: None,
            },
            channels: ChannelsConfig {
                enabled_channels: vec![],
                telegram_token: None,
                slack_token: None,
                discord_token: None,
                line_token: None,
                whatsapp_phone: None,
            },
            memory: MemoryOnboardConfig {
                enabled: true,
                provider: "openai".to_string(),
                model: None,
                api_key: None,
            },
            llm: LlmConfig {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                api_key: None,
                base_url: None,
            },
            dashboard: DashboardConfig {
                enabled: true,
                bind: "0.0.0.0:3000".to_string(),
            },
            config_path: String::new(),
        }
    }
}

/// Non-interactive onboarding for scripting/testing
pub fn onboard_command(profile: &str) -> String {
    format!("onboarded profile={profile}")
}

/// Run the interactive wizard onboarding flow
pub fn onboard_wizard() -> anyhow::Result<OnboardingConfig> {
    let theme = ColorfulTheme::default();
    let mut config = OnboardingConfig::default();
    
    // ─────────────────────────────────────────────────────────────────────
    // WELCOME SCREEN
    // ─────────────────────────────────────────────────────────────────────
    print_welcome_banner();
    
    println!("This wizard will guide you through setting up your krabkrab assistant.\n");
    
    let ready = Confirm::with_theme(&theme)
        .with_prompt("Ready to begin?")
        .default(true)
        .interact()?;
    
    if !ready {
        println!("Onboarding cancelled. Run 'krabkrab onboard' when you're ready.");
        return Ok(config);
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 1: PROFILE SELECTION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("📋 STEP 1: Profile Selection");
    println!("{}", "━".repeat(60));
    println!("Profiles allow you to have multiple configurations (e.g., work, personal).\n");
    
    config.profile = Input::with_theme(&theme)
        .with_prompt("Profile name")
        .default("default".to_string())
        .interact_text()?;
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 2: AGENT IDENTITY
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🤖 STEP 2: Agent Identity");
    println!("{}", "━".repeat(60));
    println!("Define your assistant's personality and how it identifies itself.\n");
    
    config.agent.name = Input::with_theme(&theme)
        .with_prompt("Agent name")
        .default(config.agent.name)
        .interact_text()?;
    
    config.agent.emoji = Input::with_theme(&theme)
        .with_prompt("Agent emoji")
        .default(config.agent.emoji)
        .interact_text()?;
    
    config.agent.personality = Input::with_theme(&theme)
        .with_prompt("Personality description")
        .default(config.agent.personality)
        .interact_text()?;
    
    let custom_prompt = Confirm::with_theme(&theme)
        .with_prompt("Set a custom system prompt? (advanced)")
        .default(false)
        .interact()?;
    
    if custom_prompt {
        config.agent.system_prompt = Some(
            Input::with_theme(&theme)
                .with_prompt("Custom system prompt")
                .default(format!(
                    "You are {}. {}. Always identify yourself with {}.",
                    config.agent.name, config.agent.personality, config.agent.emoji
                ))
                .interact_text()?
        );
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 3: LLM PROVIDER
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🧠 STEP 3: LLM Provider");
    println!("{}", "━".repeat(60));
    println!("Select the AI model provider for conversations.\n");
    
    let llm_providers = vec![
        "OpenAI (GPT-4, GPT-3.5)",
        "Anthropic (Claude)",
        "Google (Gemini)",
        "Ollama (Local models)",
        "Custom OpenAI-compatible API",
    ];
    
    let llm_selection = Select::with_theme(&theme)
        .with_prompt("Select LLM provider")
        .default(0)
        .items(&llm_providers)
        .interact()?;
    
    config.llm.provider = match llm_selection {
        0 => "openai",
        1 => "anthropic",
        2 => "gemini",
        3 => "ollama",
        4 => "custom",
        _ => "openai",
    }.to_string();
    
    // Model selection based on provider
    let default_model = match config.llm.provider.as_str() {
        "openai" => "gpt-4",
        "anthropic" => "claude-3-opus-20240229",
        "gemini" => "gemini-pro",
        "ollama" => "llama2",
        "custom" => "gpt-4",
        _ => "gpt-4",
    }.to_string();
    
    config.llm.model = Input::with_theme(&theme)
        .with_prompt("Model name")
        .default(default_model)
        .interact_text()?;
    
    // API Key for cloud providers
    if !["ollama"].contains(&config.llm.provider.as_str()) {
        let env_var = match config.llm.provider.as_str() {
            "openai" => "OPENAI_API_KEY",
            "anthropic" => "ANTHROPIC_API_KEY",
            "gemini" => "GOOGLE_API_KEY",
            _ => "API_KEY",
        };
        
        let has_env_key = std::env::var(env_var).is_ok();
        
        if has_env_key {
            println!("✅ Found {} in environment", env_var);
            let use_env = Confirm::with_theme(&theme)
                .with_prompt("Use the API key from environment?")
                .default(true)
                .interact()?;
            if !use_env {
                config.llm.api_key = Some(
                    Input::with_theme(&theme)
                        .with_prompt(format!("{} API Key", config.llm.provider))
                        .interact_text()?
                );
            }
        } else {
            config.llm.api_key = Some(
                Input::with_theme(&theme)
                    .with_prompt(format!("{} API Key (or press Enter to skip)", config.llm.provider))
                    .allow_empty(true)
                    .interact_text()?
            );
        }
    }
    
    // Base URL for custom/ollama
    if ["ollama", "custom"].contains(&config.llm.provider.as_str()) {
        let default_url = if config.llm.provider == "ollama" {
            "http://localhost:11434"
        } else {
            "http://localhost:8080/v1"
        };
        
        config.llm.base_url = Some(
            Input::with_theme(&theme)
                .with_prompt("API Base URL")
                .default(default_url.to_string())
                .interact_text()?
        );
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 4: MEMORY CONFIGURATION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("💾 STEP 4: Memory Configuration");
    println!("{}", "━".repeat(60));
    println!("Memory allows your assistant to remember past conversations and context.\n");
    
    config.memory.enabled = Confirm::with_theme(&theme)
        .with_prompt("Enable long-term memory?")
        .default(true)
        .interact()?;
    
    if config.memory.enabled {
        let memory_providers = vec![
            "OpenAI Embeddings",
            "Google Gemini Embeddings",
            "Ollama (Local embeddings)",
        ];
        
        let mem_selection = Select::with_theme(&theme)
            .with_prompt("Select embedding provider")
            .default(0)
            .items(&memory_providers)
            .interact()?;
        
        config.memory.provider = match mem_selection {
            0 => "openai",
            1 => "gemini",
            2 => "ollama",
            _ => "openai",
        }.to_string();
        
        if config.memory.provider != "ollama" {
            config.memory.api_key = config.llm.api_key.clone();
        }
    } else {
        config.memory.provider = "disabled".to_string();
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 5: CHANNEL CONFIGURATION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("📡 STEP 5: Channel Configuration");
    println!("{}", "━".repeat(60));
    println!("Select which messaging platforms to enable.\n");
    
    let channels = vec![
        "Telegram",
        "Slack",
        "Discord",
        "LINE",
        "WhatsApp",
    ];
    
    let selected_channels = MultiSelect::with_theme(&theme)
        .with_prompt("Select channels to enable (Space to select, Enter to confirm)")
        .items(&channels)
        .interact()?;
    
    config.channels.enabled_channels = selected_channels
        .into_iter()
        .map(|i| channels[i].to_lowercase())
        .collect();
    
    // Collect credentials for selected channels
    for channel in &config.channels.enabled_channels {
        match channel.as_str() {
            "telegram" => {
                println!("\n📱 Telegram Configuration:");
                println!("   Get a bot token from @BotFather on Telegram.");
                config.channels.telegram_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("Telegram Bot Token")
                        .allow_empty(true)
                        .interact_text()?
                );
            }
            "slack" => {
                println!("\n💼 Slack Configuration:");
                println!("   Get a Bot User OAuth Token from your Slack App.");
                config.channels.slack_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("Slack Bot Token (xoxb-...)")
                        .allow_empty(true)
                        .interact_text()?
                );
            }
            "discord" => {
                println!("\n🎮 Discord Configuration:");
                println!("   Get a Bot Token from the Discord Developer Portal.");
                config.channels.discord_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("Discord Bot Token")
                        .allow_empty(true)
                        .interact_text()?
                );
            }
            "line" => {
                println!("\n💚 LINE Configuration:");
                println!("   Get a Channel Access Token from LINE Developers Console.");
                config.channels.line_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("LINE Channel Access Token")
                        .allow_empty(true)
                        .interact_text()?
                );
            }
            "whatsapp" => {
                println!("\n💬 WhatsApp Configuration:");
                println!("   Enter your WhatsApp Business phone number ID.");
                config.channels.whatsapp_phone = Some(
                    Input::with_theme(&theme)
                        .with_prompt("WhatsApp Phone Number ID")
                        .allow_empty(true)
                        .interact_text()?
                );
            }
            _ => {}
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 6: DASHBOARD CONFIGURATION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("📊 STEP 6: Dashboard Configuration");
    println!("{}", "━".repeat(60));
    println!("The web dashboard provides a UI for monitoring and control.\n");
    
    config.dashboard.enabled = Confirm::with_theme(&theme)
        .with_prompt("Enable web dashboard?")
        .default(true)
        .interact()?;
    
    if config.dashboard.enabled {
        config.dashboard.bind = Input::with_theme(&theme)
            .with_prompt("Dashboard bind address")
            .default(config.dashboard.bind)
            .interact_text()?;
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // STEP 7: SAVE CONFIGURATION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("💾 STEP 7: Save Configuration");
    println!("{}", "━".repeat(60));
    
    // Determine config path
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("krabkrab");
    
    let default_path = config_dir.join(format!("{}.toml", config.profile));
    config.config_path = default_path.to_string_lossy().to_string();
    
    println!("\nConfiguration will be saved to:");
    println!("  {}", config.config_path);
    
    let save_config = Confirm::with_theme(&theme)
        .with_prompt("Save configuration?")
        .default(true)
        .interact()?;
    
    if save_config {
        // Create config directory if needed
        if let Some(parent) = PathBuf::from(&config.config_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        // Generate TOML config
        let toml_content = generate_toml_config(&config);
        
        match std::fs::write(&config.config_path, &toml_content) {
            Ok(_) => {
                println!("\n✅ Configuration saved successfully!");
                println!("\nGenerated configuration:\n");
                println!("{}", "─".repeat(50));
                println!("{}", toml_content);
                println!("{}", "─".repeat(50));
            }
            Err(e) => {
                println!("\n❌ Failed to save configuration: {}", e);
                println!("\nHere's your configuration to save manually:\n");
                println!("{}", "─".repeat(50));
                println!("{}", toml_content);
                println!("{}", "─".repeat(50));
            }
        }
    } else {
        println!("\nConfiguration not saved. Here's a preview:\n");
        let toml_content = generate_toml_config(&config);
        println!("{}", "─".repeat(50));
        println!("{}", toml_content);
        println!("{}", "─".repeat(50));
    }
    
    // ─────────────────────────────────────────────────────────────────────
    // COMPLETION
    // ─────────────────────────────────────────────────────────────────────
    print_completion_banner(&config);
    
    Ok(config)
}

/// Print the welcome banner
fn print_welcome_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗");
    println!("{}", "║                                                           ║");
    println!("{}", "║     🦀 Welcome to krabkrab Onboarding Wizard! 🦀          ║");
    println!("{}", "║                                                           ║");
    println!("{}", "║     Your Personal AI Assistant - Rust Edition             ║");
    println!("{}", "║                                                           ║");
    println!("{}", "╚═══════════════════════════════════════════════════════════╝");
    println!();
}

/// Print the completion banner
fn print_completion_banner(config: &OnboardingConfig) {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗");
    println!("{}", "║                                                           ║");
    println!("{}", "║              🎉 Onboarding Complete! 🎉                   ║");
    println!("{}", "║                                                           ║");
    println!("{}", "╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Next steps:");
    println!();
    println!("  1. Review your configuration:");
    println!("     krabkrab config show");
    println!();
    println!("  2. Start the daemon:");
    println!("     krabkrab gateway start");
    println!();
    println!("  3. Open the dashboard:");
    if config.dashboard.enabled {
        println!("     http://{}", config.dashboard.bind);
    }
    println!();
    println!("  4. Test your agent:");
    println!("     krabkrab ask \"Hello, {}!\"", config.agent.name);
    println!();
    println!("For more help, run: krabkrab --help");
    println!();
}

/// Generate TOML configuration from OnboardingConfig
fn generate_toml_config(config: &OnboardingConfig) -> String {
    let mut lines = vec![];
    
    lines.push("# krabkrab Configuration".to_string());
    lines.push(format!("# Generated by onboarding wizard on {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    lines.push("".to_string());
    
    // Profile settings
    lines.push(format!("profile = \"{}\"", config.profile));
    lines.push("log_level = \"info\"".to_string());
    lines.push("".to_string());
    
    // Agent identity
    lines.push("[agent]".to_string());
    lines.push(format!("name = \"{}\"", config.agent.name));
    lines.push(format!("emoji = \"{}\"", config.agent.emoji));
    lines.push(format!("personality = \"{}\"", config.agent.personality));
    if let Some(ref prompt) = config.agent.system_prompt {
        lines.push(format!("system_prompt = \"{}\"", prompt));
    }
    lines.push("".to_string());
    
    // LLM Provider
    lines.push("[llm]".to_string());
    lines.push(format!("provider = \"{}\"", config.llm.provider));
    lines.push(format!("model = \"{}\"", config.llm.model));
    if let Some(ref key) = config.llm.api_key {
        if !key.is_empty() {
            lines.push("# API key (consider using environment variable instead)".to_string());
            lines.push(format!("api_key = \"{}\"", key));
        }
    }
    if let Some(ref url) = config.llm.base_url {
        lines.push(format!("base_url = \"{}\"", url));
    }
    lines.push("".to_string());
    
    // Memory
    lines.push("[memory]".to_string());
    lines.push(format!("enabled = {}", config.memory.enabled));
    lines.push(format!("provider = \"{}\"", config.memory.provider));
    if config.memory.enabled {
        if let Some(ref model) = config.memory.model {
            lines.push(format!("model = \"{}\"", model));
        }
    }
    lines.push("".to_string());
    
    // Channels
    lines.push("[channels]".to_string());
    lines.push(format!("enabled = [{}]", config.channels.enabled_channels
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ")));
    
    if let Some(ref token) = config.channels.telegram_token {
        if !token.is_empty() {
            lines.push(format!("telegram_token = \"{}\"", token));
        }
    }
    if let Some(ref token) = config.channels.slack_token {
        if !token.is_empty() {
            lines.push(format!("slack_token = \"{}\"", token));
        }
    }
    if let Some(ref token) = config.channels.discord_token {
        if !token.is_empty() {
            lines.push(format!("discord_token = \"{}\"", token));
        }
    }
    if let Some(ref token) = config.channels.line_token {
        if !token.is_empty() {
            lines.push(format!("line_token = \"{}\"", token));
        }
    }
    if let Some(ref phone) = config.channels.whatsapp_phone {
        if !phone.is_empty() {
            lines.push(format!("whatsapp_phone = \"{}\"", phone));
        }
    }
    lines.push("".to_string());
    
    // Dashboard
    lines.push("[dashboard]".to_string());
    lines.push(format!("enabled = {}", config.dashboard.enabled));
    lines.push(format!("bind = \"{}\"", config.dashboard.bind));
    
    lines.join("\n")
}

/// Quick onboarding with minimal prompts for experienced users
pub fn onboard_quick() -> anyhow::Result<OnboardingConfig> {
    let theme = ColorfulTheme::default();
    let mut config = OnboardingConfig::default();
    
    println!("🦀 krabkrab Quick Setup\n");
    
    // Just ask for essentials
    config.agent.name = Input::with_theme(&theme)
        .with_prompt("Agent name")
        .default("krabkrab".to_string())
        .interact_text()?;
    
    let llm_providers = vec!["OpenAI", "Anthropic", "Google", "Ollama"];
    let llm_selection = Select::with_theme(&theme)
        .with_prompt("LLM Provider")
        .default(0)
        .items(&llm_providers)
        .interact()?;
    
    config.llm.provider = match llm_selection {
        0 => "openai",
        1 => "anthropic",
        2 => "gemini",
        3 => "ollama",
        _ => "openai",
    }.to_string();
    
    // Auto-detect API key from environment
    let env_var = match config.llm.provider.as_str() {
        "openai" => Some("OPENAI_API_KEY"),
        "anthropic" => Some("ANTHROPIC_API_KEY"),
        "gemini" => Some("GOOGLE_API_KEY"),
        _ => None,
    };
    
    if let Some(var) = env_var {
        if std::env::var(var).is_ok() {
            println!("✅ Found {} in environment", var);
        } else {
            config.llm.api_key = Some(
                Input::with_theme(&theme)
                    .with_prompt(format!("{} API Key", config.llm.provider))
                    .allow_empty(true)
                    .interact_text()?
            );
        }
    }
    
    // Save configuration
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("krabkrab");
    
    let config_path = config_dir.join("default.toml");
    config.config_path = config_path.to_string_lossy().to_string();
    
    if let Some(parent) = config_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    
    let toml_content = generate_toml_config(&config);
    std::fs::write(&config_path, &toml_content)?;
    
    println!("\n✅ Quick setup complete! Run 'krabkrab onboard' for full configuration.");
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OnboardingConfig::default();
        assert_eq!(config.profile, "default");
        assert_eq!(config.agent.name, "krabkrab");
        assert!(config.memory.enabled);
        assert!(config.dashboard.enabled);
    }

    #[test]
    fn test_generate_toml() {
        let config = OnboardingConfig::default();
        let toml = generate_toml_config(&config);
        assert!(toml.contains("[agent]"));
        assert!(toml.contains("[llm]"));
        assert!(toml.contains("[memory]"));
        assert!(toml.contains("[dashboard]"));
    }
}