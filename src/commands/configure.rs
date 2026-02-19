use dialoguer::{Input, Select, theme::ColorfulTheme};
use crate::config::AppConfig;

/// Non-interactive configure input for scripting / testing.
#[derive(Debug, Clone, Default)]
pub struct ConfigureInput {
    pub profile: String,
    pub verbose: bool,
}

/// Non-interactive configure command — used in tests and scripted flows.
pub fn configure_command(input: ConfigureInput) -> String {
    let verbose_str = if input.verbose { "on" } else { "off" };
    format!("profile={} verbose={}", input.profile, verbose_str)
}

/// Interactive TUI configure command — used from the CLI.
pub fn configure_command_interactive() -> String {
    let theme = ColorfulTheme::default();
    println!("🦀 krabkrab Configuration Helper 🦀\n");

    let mut config = AppConfig::default();

    // 1. Agent Identity
    println!("--- Agent Identity ---");
    config.agent.name = Input::with_theme(&theme)
        .with_prompt("Agent Name")
        .default(config.agent.name)
        .interact_text()
        .unwrap_or_default();

    config.agent.emoji = Input::with_theme(&theme)
        .with_prompt("Agent Emoji")
        .default(config.agent.emoji)
        .interact_text()
        .unwrap_or_default();

    config.agent.personality = Input::with_theme(&theme)
        .with_prompt("Personality Description")
        .default(config.agent.personality)
        .interact_text()
        .unwrap_or_default();

    // 2. Memory Settings
    println!("\n--- Memory Settings ---");
    let providers = crate::memory::MemoryConfig::supported_embedding_providers();
    let selection = Select::with_theme(&theme)
        .with_prompt("Select Embedding Provider")
        .default(0)
        .items(&providers)
        .interact()
        .unwrap_or(0);

    config.memory.provider = providers[selection].to_string();

    // 3. Save Configuration (Mock for now, would save to file)
    let toml = toml::to_string_pretty(&config).unwrap_or_default();

    format!(
        "✅ Configuration Completed!\n\nHere is your generated config:\n\n{}",
        toml
    )
}

