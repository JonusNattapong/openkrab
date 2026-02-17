use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use openclaw_core::Config;
use openclaw_gateway::GatewayServer;

#[derive(Parser)]
#[command(name = "openclaw")]
#[command(about = "OpenClaw - Multi-channel AI gateway")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value = "info")]
    log_level: LogLevel,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Onboard,
    Configure,
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    Status,
    Health,
    Sessions,
    Message {
        #[command(subcommand)]
        command: MessageCommands,
    },
    Memory,
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },
    Gateway {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "18789")]
        port: u16,
    },
    Channels {
        #[command(subcommand)]
        command: ChannelCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    Show,
    Set { key: String, value: String },
}

#[derive(Subcommand)]
enum MessageCommands {
    Send { channel: String, chat_id: String, content: String },
    Read { channel: String, chat_id: String, #[arg(default_value = "10")] limit: usize },
}

#[derive(Subcommand)]
enum AgentCommands {
    List,
    Start { name: String },
    Stop { name: String },
}

#[derive(Subcommand)]
enum ChannelCommands {
    List,
    Add { channel_type: String, config: String },
    Remove { channel_id: String },
}

fn get_log_level(level: &LogLevel) -> tracing::Level {
    match level {
        LogLevel::Error => tracing::Level::ERROR,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Trace => tracing::Level::TRACE,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(get_log_level(&cli.log_level))
        .init();

    let config = load_config(cli.config.as_ref().map(|p| p.as_path()))?;

    match cli.command {
        Commands::Setup => {
            println!("Running setup...");
            println!("1. Creating config directory: ~/.openclaw");
            println!("2. Generating default configuration");
            println!("Setup complete!");
        }
        Commands::Onboard => {
            println!("Starting onboarding wizard...");
            println!("Use 'openclaw configure' for manual configuration.");
        }
        Commands::Configure => {
            println!("Configuration wizard not yet implemented");
        }
        Commands::Config { command } => {
            handle_config_command(config, command).await?;
        }
        Commands::Status => {
            println!("Gateway Status: Running");
            println!("Host: {}", config.server.host);
            println!("Port: {}", config.server.port);
            println!("Channels: {}", config.channels.len());
        }
        Commands::Health => {
            println!("Gateway Health: OK");
        }
        Commands::Sessions => {
            println!("Sessions: (none)");
        }
        Commands::Message { command } => {
            handle_message_command(config, command).await?;
        }
        Commands::Memory => {
            println!("Memory: 0 MB used");
        }
        Commands::Agent { command } => {
            handle_agent_command(config, command).await?;
        }
        Commands::Gateway { host, port } => {
            run_gateway(config, host, port).await?;
        }
        Commands::Channels { command } => {
            handle_channel_command(config, command).await?;
        }
    }

    Ok(())
}

fn load_config(config_path: Option<&Path>) -> anyhow::Result<Config> {
    let config_path = config_path
        .and_then(|p| p.to_str())
        .unwrap_or("openclaw.toml");

    if std::path::Path::new(config_path).exists() {
        Config::load(config_path)
    } else {
        tracing::warn!("Config file not found, using defaults");
        Ok(Config::default())
    }
}

async fn run_gateway(mut config: Config, host: String, port: u16) -> anyhow::Result<()> {
    config.server.host = host;
    config.server.port = port;

    let server = GatewayServer::new(config);

    tracing::info!("Starting OpenClaw Gateway Server...");
    server.start().await?;

    Ok(())
}

async fn handle_config_command(config: Config, command: ConfigCommands) -> anyhow::Result<()> {
    match command {
        ConfigCommands::Show => {
            let json = serde_json::to_string_pretty(&config)?;
            println!("{}", json);
        }
        ConfigCommands::Set { key, value } => {
            println!("Setting {} = {} (not persisted)", key, value);
        }
    }
    Ok(())
}

async fn handle_message_command(_config: Config, command: MessageCommands) -> anyhow::Result<()> {
    match command {
        MessageCommands::Send { channel, chat_id, content } => {
            println!("Sending to {}:{}: {}", channel, chat_id, content);
            println!("(Gateway not connected - start with 'openclaw gateway')");
        }
        MessageCommands::Read { channel, chat_id, limit } => {
            println!("Reading {} messages from {}:{}", limit, channel, chat_id);
        }
    }
    Ok(())
}

async fn handle_agent_command(_config: Config, command: AgentCommands) -> anyhow::Result<()> {
    match command {
        AgentCommands::List => {
            println!("Available agents: []");
        }
        AgentCommands::Start { name } => {
            println!("Starting agent {} (not implemented)", name);
        }
        AgentCommands::Stop { name } => {
            println!("Stopping agent {} (not implemented)", name);
        }
    }
    Ok(())
}

async fn handle_channel_command(config: Config, command: ChannelCommands) -> anyhow::Result<()> {
    match command {
        ChannelCommands::List => {
            println!("Configured channels:");
            if config.channels.is_empty() {
                println!("  (none)");
            } else {
                for ch in &config.channels {
                    println!("  - {} ({})", ch.name, ch.channel_type);
                }
            }
            println!("\nAvailable: telegram, discord, slack, whatsapp, signal, web");
        }
        ChannelCommands::Add { channel_type, .. } => {
            println!("Adding {} channel (not implemented)", channel_type);
        }
        ChannelCommands::Remove { channel_id } => {
            println!("Removing channel {} (not implemented)", channel_id);
        }
    }
    Ok(())
}
