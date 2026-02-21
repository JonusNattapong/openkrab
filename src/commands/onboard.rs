//! Wizard Onboarding Module
//!
//! Provides an interactive step-by-step wizard for first-time users to configure
//! their krabkrab assistant, including agent identity, channels, memory, and providers.

use crate::config_io;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
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
    pub workspace: Option<String>,
    pub agent_dir: Option<String>,
    pub list: Vec<AgentDefinition>,
    pub subagents: SubagentsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    pub name: String,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub tools: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentsConfig {
    pub enabled: bool,
    pub max_concurrent: Option<u32>,
    pub agents: Vec<AgentDefinition>,
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

/// Auth profile for LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProfile {
    pub name: String,
    pub provider: String,
    pub auth_type: AuthType,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

/// Auth type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthType {
    ApiKey,
    OAuth,
    Keychain,
    Environment,
}

impl Default for AuthType {
    fn default() -> Self {
        Self::ApiKey
    }
}

/// Channel plugin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPlugin {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub config: Option<serde_json::Value>,
}

impl Default for AgentDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            model: None,
            system_prompt: None,
            tools: None,
        }
    }
}

impl Default for SubagentsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_concurrent: Some(3),
            agents: Vec::new(),
        }
    }
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
                workspace: None,
                agent_dir: None,
                list: Vec::new(),
                subagents: SubagentsConfig::default(),
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

/// Onboarding mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardMode {
    QuickStart,
    Manual,
}

impl Default for OnboardMode {
    fn default() -> Self {
        Self::QuickStart
    }
}

impl std::str::FromStr for OnboardMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "quick" | "quickstart" => Ok(Self::QuickStart),
            "manual" | "advanced" => Ok(Self::Manual),
            _ => Ok(Self::QuickStart),
        }
    }
}

/// Config handling choice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigHandling {
    Keep,
    Modify,
    Reset,
}

/// Reset scope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetScope {
    Config,
    ConfigCredsSessions,
    Full,
}

/// Gateway auth mode
#[derive(Debug, Clone)]
pub struct GatewaySettings {
    pub port: u16,
    pub bind: String,
    pub auth_mode: String,
    pub token: Option<String>,
    pub password: Option<String>,
    pub tailscale_mode: String,
    pub custom_bind_host: Option<String>,
}

impl Default for GatewaySettings {
    fn default() -> Self {
        Self {
            port: 18789,
            bind: "loopback".to_string(),
            auth_mode: "token".to_string(),
            token: None,
            password: None,
            tailscale_mode: "off".to_string(),
            custom_bind_host: None,
        }
    }
}

/// Onboarding mode (local or remote gateway)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnboardGatewayMode {
    #[default]
    Local,
    Remote,
}

/// Run the interactive wizard onboarding flow
pub fn onboard_wizard() -> anyhow::Result<OnboardingConfig> {
    let theme = ColorfulTheme::default();
    let mut config = OnboardingConfig::default();

    // ─────────────────────────────────────────────────────────────────────
    // WELCOME SCREEN
    // ─────────────────────────────────────────────────────────────────────
    print_welcome_banner();

    if cfg!(target_os = "windows") {
        println!("Windows detected — krabkrab runs great on WSL2!");
        println!("Native Windows might be trickier.");
        println!("Quick setup: wsl --install (one command, one reboot)");
        println!("Guide: https://docs.openclaw.ai/windows\n");
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP 0: FLOW SELECTION
    // ─────────────────────────────────────────────────────────────────────
    println!("{}", "━".repeat(60));
    println!("📋 STEP 0: Onboarding Mode");
    println!("{}", "━".repeat(60));
    println!("Select the onboarding mode that suits your needs.\n");

    let flow_options = vec!["QuickStart", "Manual"];
    let flow_selection = Select::with_theme(&theme)
        .with_prompt("Select onboarding mode")
        .default(0)
        .items(&flow_options)
        .interact()?;

    let mode = if flow_selection == 0 {
        OnboardMode::QuickStart
    } else {
        OnboardMode::Manual
    };

    // ─────────────────────────────────────────────────────────────────────
    // RISK ACKNOWLEDGMENT
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("⚠️  Security Warning");
    println!("{}", "━".repeat(60));
    println!("Security warning — please read.");
    println!();
    println!(" is a hobby projectkrabkrab and still in beta. Expect sharp edges.");
    println!("This bot can read files and run actions if tools are enabled.");
    println!("A bad prompt can trick it into doing unsafe things.");
    println!();
    println!(
        "If you're not comfortable with basic security and access control, don't run krabkrab."
    );
    println!(
        "Ask someone experienced to help before enabling tools or exposing it to the internet."
    );
    println!();
    println!("Recommended baseline:");
    println!("- Pairing/allowlists + mention gating.");
    println!("- Sandbox + least-privilege tools.");
    println!("- Keep secrets out of the agent's reachable filesystem.");
    println!("- Use the strongest available model for any bot with tools or untrusted inboxes.");
    println!();
    println!("Run regularly:");
    println!("krabkrab security audit --deep");
    println!("krabkrab security audit --fix");
    println!();
    println!("Must read: https://docs.openclaw.ai/gateway/security\n");

    let ready = Confirm::with_theme(&theme)
        .with_prompt("I understand this is powerful and inherently risky. Continue?")
        .default(false)
        .interact()?;

    if !ready {
        println!(
            "Onboarding cancelled. Run 'krabkrab onboard' when you're ready to accept the risks."
        );
        return Ok(config);
    }

    // Check existing config validity
    let config_path = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("krabkrab")
        .join("default.toml");

    if config_path.exists() {
        println!("\n{}", "━".repeat(60));
        println!("📋 Checking existing configuration...");
        println!("{}", "━".repeat(60));

        match config_io::load_config() {
            Ok(cfg) => {
                if let Err(e) = config_io::validate_config(&cfg) {
                    println!("\n⚠️  Configuration is invalid!");
                    println!("Error: {}", e);
                    println!("\nRun 'krabkrab doctor' to fix, then re-run onboarding.");
                    return Err(anyhow::anyhow!("Invalid configuration"));
                }
                println!("  ✅ Configuration is valid");
            }
            Err(e) => {
                println!("\n⚠️  Could not load configuration: {}", e);
                println!("  Will create a new configuration.");
            }
        }
    }

    // Probe existing config
    let existing_cfg = config_io::load_config().ok();
    if let Some(cfg) = &existing_cfg {
        println!("\n{}", "━".repeat(60));
        println!("📁 Existing Configuration Detected");
        println!("{}", "━".repeat(60));

        let port = cfg.gateway.as_ref().and_then(|g| g.port).unwrap_or(18789);
        let bind = cfg
            .gateway
            .as_ref()
            .and_then(|g| g.bind_address.clone())
            .unwrap_or_else(|| "loopback".to_string());
        println!("Gateway port: {}", port);
        println!("Gateway bind: {}", bind);

        println!("\nChoose how to handle existing configuration:\n");

        let handling_options = vec![
            "Keep existing values",
            "Update values",
            "Reset configuration",
        ];
        let handling_selection = Select::with_theme(&theme)
            .with_prompt("Config handling")
            .default(0)
            .items(&handling_options)
            .interact()?;

        match handling_selection {
            0 => {
                // Keep
                if cfg.gateway.is_some() {
                    config.dashboard.enabled = true;
                    config.dashboard.bind = format!("127.0.0.1:{}", port);
                }
            }
            1 => {
                // Modify - continue with wizard
                println!("\n→ Proceeding with wizard to update values...\n");
            }
            2 => {
                // Reset
                let scope_options = vec![
                    "Config only",
                    "Config + credentials + sessions",
                    "Full reset (config + creds + sessions + workspace)",
                ];
                let scope_selection = Select::with_theme(&theme)
                    .with_prompt("Reset scope")
                    .default(0)
                    .items(&scope_options)
                    .interact()?;

                let scope = match scope_selection {
                    0 => ResetScope::Config,
                    1 => ResetScope::ConfigCredsSessions,
                    _ => ResetScope::Full,
                };

                handle_reset_scope(&scope)?;
                config = OnboardingConfig::default();
                println!("\n✅ Configuration reset complete.\n");
            }
            _ => {}
        }
    }

    // For QuickStart mode, apply sensible defaults
    if mode == OnboardMode::QuickStart {
        println!("\n{}", "━".repeat(60));
        println!("🚀 Applying QuickStart defaults");
        println!("{}", "━".repeat(60));

        // Use defaults for QuickStart
        config.profile = "default".to_string();
        config.agent.name = "krabkrab".to_string();
        config.agent.emoji = "🦀".to_string();
        config.agent.personality = "A helpful and precise AI assistant.".to_string();
        config.llm.provider = "openai".to_string();
        config.llm.model = "gpt-4".to_string();
        config.memory.enabled = true;
        config.memory.provider = "openai".to_string();
        config.dashboard.enabled = true;
        config.dashboard.bind = "127.0.0.1:3000".to_string();

        // Check for API key in environment
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            if !key.trim().is_empty() {
                config.llm.api_key = Some(key);
            }
        }

        println!("✅ QuickStart defaults applied.");
        println!("   - Provider: {}", config.llm.provider);
        println!("   - Model: {}", config.llm.model);
        println!("   - Dashboard: http://{}\n", config.dashboard.bind);

        // QuickStart gateway settings
        println!("{}", "━".repeat(60));
        println!("🚀 QuickStart Gateway Settings");
        println!("{}", "━".repeat(60));
        println!("Gateway port: 18789");
        println!("Gateway bind: Loopback (127.0.0.1)");
        println!("Gateway auth: Token");
        println!("Tailscale: Off");
        println!();
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP X: GATEWAY MODE (local or remote)
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🌐 STEP X: Gateway Mode");
    println!("{}", "━".repeat(60));

    let mode_options = vec!["Local gateway (this machine)", "Remote gateway (info-only)"];
    let mode_selection = Select::with_theme(&theme)
        .with_prompt("What do you want to set up?")
        .default(0)
        .items(&mode_options)
        .interact()?;

    let is_remote = mode_selection == 1;

    if is_remote {
        println!("\n📡 Remote gateway mode selected.");
        println!("   You'll need to configure the remote gateway URL separately.");
        println!("   Run 'krabkrab configure' after onboarding to add remote gateway.");
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP X: GATEWAY CONFIGURATION (for Manual mode + local)
    // ─────────────────────────────────────────────────────────────────────
    if mode == OnboardMode::Manual && !is_remote {
        println!("\n{}", "━".repeat(60));
        println!("🌐 STEP X: Gateway Configuration");
        println!("{}", "━".repeat(60));

        // Port
        let port_str = Input::with_theme(&theme)
            .with_prompt("Gateway port")
            .default("18789".to_string())
            .interact_text()?;
        let port: u16 = port_str.parse().unwrap_or(18789);

        // Bind
        let bind_options = vec!["loopback", "lan", "custom"];
        let bind_selection = Select::with_theme(&theme)
            .with_prompt("Gateway bind")
            .default(0)
            .items(&bind_options)
            .interact()?;
        let bind = bind_options[bind_selection].to_string();

        // Auth mode
        let auth_options = vec!["Token", "Password"];
        let auth_selection = Select::with_theme(&theme)
            .with_prompt("Gateway auth mode")
            .default(0)
            .items(&auth_options)
            .interact()?;
        let auth_mode = if auth_selection == 0 {
            "token"
        } else {
            "password"
        }
        .to_string();

        // Tailscale
        let ts_options = vec!["Off", "Serve", "Funnel"];
        let ts_selection = Select::with_theme(&theme)
            .with_prompt("Tailscale exposure")
            .default(0)
            .items(&ts_options)
            .interact()?;
        let tailscale_mode = match ts_selection {
            0 => "off",
            1 => "serve",
            _ => "funnel",
        };

        println!("\n✅ Gateway configured:");
        println!("   - Port: {}", port);
        println!("   - Bind: {}", bind);
        println!("   - Auth: {}", auth_mode);
        println!("   - Tailscale: {}", tailscale_mode);
    }

    // ─────────────────────────────────────────────────────────────────────
    // PROBE GATEWAY REACHABILITY
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🔍 Checking gateway reachability...");
    println!("{}", "━".repeat(60));

    let gateway_url = "ws://127.0.0.1:18789";
    let gateway_reachable = probe_gateway(gateway_url);

    if gateway_reachable {
        println!("✅ Gateway reachable at {}", gateway_url);
    } else {
        println!("⚠️  Gateway not detected at {}", gateway_url);
        println!("   It will be started when you run 'krabkrab gateway start'");
    }

    println!("Security warning — please read.");
    println!();
    println!("krabkrab is a hobby project and still in beta. Expect sharp edges.");
    println!("This bot can read files and run actions if tools are enabled.");
    println!("A bad prompt can trick it into doing unsafe things.");
    println!();
    println!(
        "If you’re not comfortable with basic security and access control, don’t run krabkrab."
    );
    println!(
        "Ask someone experienced to help before enabling tools or exposing it to the internet."
    );
    println!();
    println!("Recommended baseline:");
    println!("- Pairing/allowlists + mention gating.");
    println!("- Sandbox + least-privilege tools.");
    println!("- Keep secrets out of the agent’s reachable filesystem.");
    println!("- Use the strongest available model for any bot with tools or untrusted inboxes.");
    println!();
    println!("Run regularly:");
    println!("krabkrab security audit --deep");
    println!("krabkrab security audit --fix");
    println!();
    println!("Must read: https://docs.openclaw.ai/gateway/security\n");

    let ready = Confirm::with_theme(&theme)
        .with_prompt("I understand this is powerful and inherently risky. Continue?")
        .default(false)
        .interact()?;

    if !ready {
        println!(
            "Onboarding cancelled. Run 'krabkrab onboard' when you're ready to accept the risks."
        );
        return Ok(config);
    }

    // Probe existing config
    let existing_cfg = config_io::load_config().ok();
    if let Some(cfg) = &existing_cfg {
        println!("\n{}", "━".repeat(60));
        println!("Existing config detected");
        println!("{}", "━".repeat(60));

        let ws = "none".to_string(); // Workspace might not be in config directly, or handled differently
        let port = cfg.gateway.as_ref().and_then(|g| g.port).unwrap_or(18789);
        println!("workspace: {}", ws);
        println!("gateway.port: {}", port);

        let use_existing = Confirm::with_theme(&theme)
            .with_prompt("Use existing values where applicable?")
            .default(true)
            .interact()?;

        if use_existing {
            if cfg.gateway.is_some() {
                config.dashboard.enabled = true;
                config.dashboard.bind = format!("127.0.0.1:{}", port);
            }
        }
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
                .interact_text()?,
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
    }
    .to_string();

    // Model selection based on provider
    let default_model = match config.llm.provider.as_str() {
        "openai" => "gpt-4",
        "anthropic" => "claude-3-opus-20240229",
        "gemini" => "gemini-pro",
        "ollama" => "llama2",
        "custom" => "gpt-4",
        _ => "gpt-4",
    }
    .to_string();

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
                        .interact_text()?,
                );
            }
        } else {
            config.llm.api_key = Some(
                Input::with_theme(&theme)
                    .with_prompt(format!(
                        "{} API Key (or press Enter to skip)",
                        config.llm.provider
                    ))
                    .allow_empty(true)
                    .interact_text()?,
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
                .interact_text()?,
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP 3.5: AUTH PROFILES
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🔐 STEP 3.5: Auth Profile Setup");
    println!("{}", "━".repeat(60));
    println!("Auth profiles allow you to manage multiple provider credentials.\n");

    let auth_type_options = vec![
        "API Key (direct input)",
        "Environment variable",
        "Keychain (macOS/Windows)",
    ];
    let auth_selection = Select::with_theme(&theme)
        .with_prompt("Authentication method")
        .default(0)
        .items(&auth_type_options)
        .interact()?;

    let auth_type = match auth_selection {
        0 => "api_key",
        1 => "environment",
        2 => "keychain",
        _ => "api_key",
    };

    match auth_type {
        "environment" => {
            println!("  Using environment variable for authentication.");
            println!(
                "  Set {} before starting the gateway.",
                match config.llm.provider.as_str() {
                    "openai" => "OPENAI_API_KEY",
                    "anthropic" => "ANTHROPIC_API_KEY",
                    "gemini" => "GOOGLE_API_KEY",
                    _ => "API_KEY",
                }
            );
        }
        "keychain" => {
            #[cfg(target_os = "macos")]
            {
                println!("  macOS Keychain selected.");
                println!(
                    "  Run 'krabkrab auth store --provider {}' after onboarding.",
                    config.llm.provider
                );
            }
            #[cfg(target_os = "windows")]
            {
                println!("  Windows Credential Manager selected.");
                println!(
                    "  Run 'krabkrab auth store --provider {}' after onboarding.",
                    config.llm.provider
                );
            }
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            {
                println!("  Keychain not available on this platform. Using API key instead.");
            }
        }
        _ => {}
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
        }
        .to_string();

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

    let channels = vec!["Telegram", "Slack", "Discord", "LINE", "WhatsApp"];

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
                        .interact_text()?,
                );
            }
            "slack" => {
                println!("\n💼 Slack Configuration:");
                println!("   Get a Bot User OAuth Token from your Slack App.");
                config.channels.slack_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("Slack Bot Token (xoxb-...)")
                        .allow_empty(true)
                        .interact_text()?,
                );
            }
            "discord" => {
                println!("\n🎮 Discord Configuration:");
                println!("   Get a Bot Token from the Discord Developer Portal.");
                config.channels.discord_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("Discord Bot Token")
                        .allow_empty(true)
                        .interact_text()?,
                );
            }
            "line" => {
                println!("\n💚 LINE Configuration:");
                println!("   Get a Channel Access Token from LINE Developers Console.");
                config.channels.line_token = Some(
                    Input::with_theme(&theme)
                        .with_prompt("LINE Channel Access Token")
                        .allow_empty(true)
                        .interact_text()?,
                );
            }
            "whatsapp" => {
                println!("\n💬 WhatsApp Configuration:");
                println!("   Enter your WhatsApp Business phone number ID.");
                config.channels.whatsapp_phone = Some(
                    Input::with_theme(&theme)
                        .with_prompt("WhatsApp Phone Number ID")
                        .allow_empty(true)
                        .interact_text()?,
                );
            }
            _ => {}
        }
    }

    // Channel plugins info
    println!("\n📦 Available channel plugins:");
    println!("  - telegram: Telegram messaging");
    println!("  - discord: Discord server");
    println!("  - slack: Slack workspace");
    println!("  - whatsapp: WhatsApp Business");
    println!("  - line: LINE messaging");
    println!("  - matrix: Matrix protocol");
    println!("  - signal: Signal messaging");
    println!("\n  Install more: krabkrab channels install <plugin>");

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
    // STEP 8: WORKSPACE SETUP
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("📁 STEP 8: Workspace Setup");
    println!("{}", "━".repeat(60));

    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let base_dir = home.join(".openkrab");
    let workspace_dir = base_dir.join("krabkrab-workspace");
    let sessions_dir = base_dir.join("sessions");

    // Create workspace directory
    if let Err(e) = std::fs::create_dir_all(&workspace_dir) {
        println!("  ⚠️  Could not create workspace: {}", e);
    } else {
        println!("  ✅ Workspace created: {}", workspace_dir.display());
    }

    // Create sessions directory
    if let Err(e) = std::fs::create_dir_all(&sessions_dir) {
        println!("  ⚠️  Could not create sessions dir: {}", e);
    } else {
        println!(
            "  ✅ Sessions directory created: {}",
            sessions_dir.display()
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP 9: BOOTSTRAP SETUP
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🐣 STEP 9: Bootstrap Message");
    println!("{}", "━".repeat(60));
    println!("Set a message to wake up your agent on first launch.\n");

    let use_bootstrap = Confirm::with_theme(&theme)
        .with_prompt("Enable bootstrap message?")
        .default(true)
        .interact()?;

    if use_bootstrap {
        let bootstrap_message = Input::with_theme(&theme)
            .with_prompt("Wake-up message")
            .default("Wake up, my friend!".to_string())
            .interact_text()?;

        // Save bootstrap to workspace
        let bootstrap_file = workspace_dir.join("bootstrap.txt");
        if let Err(e) = std::fs::write(&bootstrap_file, &bootstrap_message) {
            println!("  ⚠️  Could not save bootstrap: {}", e);
        } else {
            println!("  ✅ Bootstrap message saved");
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP 10: SKILLS SETUP
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🧩 STEP 10: Skills Setup");
    println!("{}", "━".repeat(60));

    let skills_dir = base_dir.join("skills");
    if let Err(e) = std::fs::create_dir_all(&skills_dir) {
        println!("  ⚠️  Could not create skills dir: {}", e);
    } else {
        println!("  ✅ Skills directory created: {}", skills_dir.display());
    }

    println!("\n  Skills can extend your agent's capabilities.");
    println!("  Add skills by placing skill files in the skills directory.");
    println!("  Learn more: https://docs.openclaw.ai/tools/skills\n");

    // ─────────────────────────────────────────────────────────────────────
    // STEP 11: HOOKS SETUP
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🪝 STEP 11: Internal Hooks Setup");
    println!("{}", "━".repeat(60));
    println!("Hooks allow your agent to respond to specific events.\n");

    let hooks_dir = base_dir.join("hooks");
    if let Err(e) = std::fs::create_dir_all(&hooks_dir) {
        println!("  ⚠️  Could not create hooks dir: {}", e);
    } else {
        println!("  ✅ Hooks directory created: {}", hooks_dir.display());
    }

    // Create default hooks
    let new_session_hook = hooks_dir.join("on_new_session.rs");
    if !new_session_hook.exists() {
        let default_hook = r#"// On new session hook
// This runs when a new session is created

fn on_new_session(session: &Session) {
    // Send a welcome message to new sessions
    session.send("Hello! I'm ready to help you.");
}
"#;
        if let Err(e) = std::fs::write(&new_session_hook, default_hook) {
            println!("  ⚠️  Could not create default hook: {}", e);
        } else {
            println!("  ✅ Default session hook created");
        }
    }

    println!("\n  Built-in hooks:");
    println!("  - on_new_session: Triggered when a new session starts");
    println!("  - on_message: Triggered on every message");
    println!("  - on_tool_call: Triggered when a tool is called\n");

    // ─────────────────────────────────────────────────────────────────────
    // STEP 12: GATEWAY SERVICE INSTALL
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("⚙️  STEP 12: Gateway Service Setup");
    println!("{}", "━".repeat(60));

    #[cfg(target_os = "windows")]
    {
        println!("Windows detected.");
        println!("Gateway service on Windows requires manual setup.");
        println!("Run 'krabkrab gateway start' to start the gateway.\n");
    }

    #[cfg(target_os = "macos")]
    {
        let install_service = Confirm::with_theme(&theme)
            .with_prompt("Install Gateway as a launchd service? (recommended)")
            .default(true)
            .interact()?;

        if install_service {
            println!("\n  macOS service setup:");
            println!("  To install manually, run:");
            println!("    brew services start krabkrab");
            println!("  Or use: krabkrab gateway start");
        }
    }

    #[cfg(target_os = "linux")]
    {
        let install_service = Confirm::with_theme(&theme)
            .with_prompt("Install Gateway as a systemd service? (recommended)")
            .default(true)
            .interact()?;

        if install_service {
            println!("\n  Linux service setup:");
            println!("  To install manually, run:");
            println!("    sudo systemctl enable krabkrab");
            println!("    sudo systemctl start krabkrab");
            println!("  Or use: krabkrab gateway start");
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // STEP 13: GATEWAY TOKEN GENERATION
    // ─────────────────────────────────────────────────────────────────────
    println!("\n{}", "━".repeat(60));
    println!("🔑 STEP 13: Gateway Token Setup");
    println!("{}", "━".repeat(60));

    let generate_token = Confirm::with_theme(&theme)
        .with_prompt("Generate a secure gateway token? (recommended)")
        .default(true)
        .interact()?;

    let gateway_token = if generate_token {
        let token = generate_gateway_token();
        println!("  ✅ Generated gateway token: {}...", &token[..8]);
        println!("  Token will be saved to configuration.");
        Some(token)
    } else {
        let enter_token = Confirm::with_theme(&theme)
            .with_prompt("Enter a custom token?")
            .default(false)
            .interact()?;

        if enter_token {
            Some(
                Input::with_theme(&theme)
                    .with_prompt("Enter custom token")
                    .interact_text()?,
            )
        } else {
            None
        }
    };

    // Save token to credentials directory
    if let Some(token) = &gateway_token {
        save_gateway_token(token)?;
        println!("  ✅ Token saved to ~/.openkrab/credentials/gateway_token");
    }

    // ─────────────────────────────────────────────────────────────────────
    // COMPLETION
    // ─────────────────────────────────────────────────────────────────────
    let _hatch_choice = print_completion_banner_and_choice(&config)?;

    // Try to check gateway health if it's running
    check_gateway_health(&config);

    // Setup shell completion
    setup_shell_completion()?;

    Ok(config)
}

/// Handle reset scope for existing configuration
fn handle_reset_scope(scope: &ResetScope) -> anyhow::Result<()> {
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let base_dir = home.join(".openkrab");
    let credentials = base_dir.join("credentials");
    let sessions = base_dir.join("sessions");
    let workspace = base_dir.join("krabkrab-workspace");

    println!("\n🔄 Resetting configuration...");

    match scope {
        ResetScope::Config => {
            if let Ok(_cfg) = config_io::load_config() {
                if let Some(path) = config_io::resolve_config_path().ok() {
                    std::fs::remove_file(&path).ok();
                    println!("  ✅ Removed config file: {}", path.display());
                }
            }
        }
        ResetScope::ConfigCredsSessions => {
            // Config
            if let Ok(_cfg) = config_io::load_config() {
                if let Some(path) = config_io::resolve_config_path().ok() {
                    std::fs::remove_file(&path).ok();
                }
            }
            // Credentials
            if credentials.exists() {
                std::fs::remove_dir_all(&credentials).ok();
                println!("  ✅ Removed credentials dir");
            }
            // Sessions
            if sessions.exists() {
                std::fs::remove_dir_all(&sessions).ok();
                println!("  ✅ Removed sessions dir");
            }
        }
        ResetScope::Full => {
            // Config
            if let Ok(_cfg) = config_io::load_config() {
                if let Some(path) = config_io::resolve_config_path().ok() {
                    std::fs::remove_file(&path).ok();
                }
            }
            // Credentials
            if credentials.exists() {
                std::fs::remove_dir_all(&credentials).ok();
                println!("  ✅ Removed credentials dir");
            }
            // Sessions
            if sessions.exists() {
                std::fs::remove_dir_all(&sessions).ok();
                println!("  ✅ Removed sessions dir");
            }
            // Workspace
            if workspace.exists() {
                std::fs::remove_dir_all(&workspace).ok();
                println!("  ✅ Removed workspace dir");
            }
        }
    }

    println!("  ✅ Reset complete.\n");
    Ok(())
}

/// Check if gateway is running and healthy
fn check_gateway_health(_config: &OnboardingConfig) {
    let port = 18789u16;
    let url = format!("http://127.0.0.1:{}/health", port);

    println!("\n🔍 Checking gateway health...");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build();

    match client {
        Ok(c) => {
            if let Ok(resp) = c.get(&url).send() {
                if resp.status().is_success() {
                    println!("  ✅ Gateway is running and healthy!");
                    return;
                }
            }
            println!("  ⚠️  Gateway not reachable at http://127.0.0.1:{}", port);
            println!("     Start it with: krabkrab gateway start");
        }
        Err(_) => {
            println!("  ⚠️  Could not connect to gateway");
            println!("     Start it with: krabkrab gateway start");
        }
    }
}

/// Probe if gateway is reachable at the given URL
fn probe_gateway(url: &str) -> bool {
    use std::net::TcpStream;
    use std::time::Duration;

    let addr = url
        .strip_prefix("ws://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    if let Ok(_stream) = TcpStream::connect_timeout(
        &addr
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:18789".parse().unwrap()),
        Duration::from_secs(2),
    ) {
        return true;
    }
    false
}

/// Generate a secure random gateway token
fn generate_gateway_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let random: u64 = rand_simple();

    format!("krab_{:x}{:x}", timestamp, random)
}

/// Simple pseudo-random number generator
fn rand_simple() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // Linear congruential generator
    const A: u64 = 6364136223846793005;
    const C: u64 = 1;
    (A.wrapping_mul(nanos) ^ (nanos >> 17)).wrapping_add(C)
}

/// Save gateway token to credentials directory
fn save_gateway_token(token: &str) -> anyhow::Result<()> {
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let creds_dir = home.join(".openkrab").join("credentials");

    std::fs::create_dir_all(&creds_dir)?;

    let token_file = creds_dir.join("gateway_token");
    std::fs::write(token_file, token)?;

    Ok(())
}

/// Setup shell completion for krabkrab
fn setup_shell_completion() -> anyhow::Result<()> {
    println!("\n{}", "🐚 Setting up shell completion...");

    let shell = std::env::var("SHELL")
        .ok()
        .map(|s| {
            if s.contains("zsh") {
                "zsh"
            } else if s.contains("bash") {
                "bash"
            } else if s.contains("fish") {
                "fish"
            } else {
                "unknown"
            }
        })
        .unwrap_or("unknown");

    if shell == "unknown" {
        println!("  ⚠️  Could not detect shell. Skipping completion setup.");
        println!("  To set up manually, run:");
        println!("    krabkrab completion --shell <bash|zsh|fish>");
        return Ok(());
    }

    println!("  ✅ Shell completion available for {}", shell);
    println!("  To enable, add to your shell config:");
    println!("    source $(krabkrab completion --shell {})", shell);

    Ok(())
}

/// Print the welcome banner
fn print_welcome_banner() {
    println!();
    println!("{}", "▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄");
    println!("{}", "██░▄▄▄░██░▄▄░██░▄▄▄██░▀██░██░▄▄▀██░████░▄▄▀██░███░██");
    println!("{}", "██░███░██░▀▀░██░▄▄▄██░█░█░██░█████░████░▀▀░██░█░█░██");
    println!("{}", "██░▀▀▀░██░█████░▀▀▀██░██▄░██░▀▀▄██░▀▀░█░██░██▄▀▄▀▄██");
    println!("{}", "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    println!("{}", "                  🦀 OPENKRAB 🦀                    ");
    println!();
}

/// Print the completion banner and offer hatch choice
pub fn print_completion_banner_and_choice(config: &OnboardingConfig) -> anyhow::Result<String> {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", "════════════════════════════════════════════════════");
    println!("{}", "Onboarding complete!");
    println!("{}", "════════════════════════════════════════════════════");
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
        let url = format!("http://{}", config.dashboard.bind);
        println!("     {}", url);
    }
    println!();
    println!("  4. Test your agent:");
    println!("     krabkrab ask \"Hello, {}!\"", config.agent.name);
    println!();

    // Hatch choice
    println!("{}", "━".repeat(60));
    println!("🐣 How do you want to start your agent?");
    println!("{}", "━".repeat(60));

    let hatch_options = vec!["Launch TUI (recommended)", "Open Web UI", "Do this later"];

    let hatch_selection = Select::with_theme(&theme)
        .with_prompt("Choose startup option")
        .default(0)
        .items(&hatch_options)
        .interact()?;

    let choice = match hatch_selection {
        0 => "tui",
        1 => "web",
        _ => "later",
    };

    // Handle user choice
    match choice {
        "tui" => {
            println!("\n🐣 Launching TUI...");
            println!("   Starting your agent with: 'Wake up, my friend!'\n");

            // Try to launch TUI
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/c", "start", "krabkrab", "tui"])
                    .spawn()
                    .ok();
            }
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .args(["-a", "Terminal", "--args", "-c", "krabkrab tui"])
                    .spawn()
                    .ok();
            }
            #[cfg(target_os = "linux")]
            {
                std::process::Command::new("x-terminal-emulator")
                    .args(["-e", "krabkrab tui"])
                    .spawn()
                    .ok();
            }

            // Fallback: show instructions
            println!("   If TUI didn't start, run manually:");
            println!("   krabkrab tui");
        }
        "web" => {
            let url = format!("http://{}", config.dashboard.bind);
            println!("\n🌐 Opening Web UI...");

            #[cfg(target_os = "windows")]
            std::process::Command::new("cmd")
                .args(["/c", "start", &url])
                .spawn()
                .ok();
            #[cfg(target_os = "macos")]
            std::process::Command::new("open").arg(&url).spawn().ok();
            #[cfg(target_os = "linux")]
            std::process::Command::new("xdg-open")
                .arg(&url)
                .spawn()
                .ok();
        }
        _ => {}
    }

    println!();
    println!("For more help, run: krabkrab --help");
    println!();

    Ok(choice.to_string())
}

/// Generate TOML configuration from OnboardingConfig
fn generate_toml_config(config: &OnboardingConfig) -> String {
    let mut lines = vec![];

    lines.push("# krabkrab Configuration".to_string());
    lines.push(format!(
        "# Generated by onboarding wizard on {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
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
    lines.push(format!(
        "enabled = [{}]",
        config
            .channels
            .enabled_channels
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect::<Vec<_>>()
            .join(", ")
    ));

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
    }
    .to_string();

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
                    .interact_text()?,
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
