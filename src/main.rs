//! OpenClaw - Multi-channel AI Gateway
//!
//! A production-ready gateway for managing AI conversations across multiple messaging channels.

mod app;

use app::{ensure_config, get_config_path, Application};
use openclaw_core::Config;
use std::process;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    init_logging();
    
    print_banner();
    
    let config_path = match get_config_path() {
        Some(path) if path.exists() => path,
        _ => {
            info!("Configuration not found, creating default...");
            ensure_config().await?
        }
    };
    
    info!("Loading configuration from: {}", config_path.display());
    
    let config = match Config::load(config_path.to_str().unwrap()) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            error!("Please check your configuration file at: {}", config_path.display());
            return Err(e);
        }
    };
    
    info!("Configuration loaded successfully");
    
    let mut app = Application::new(config);
    
    if let Err(e) = app.init().await {
        error!("Failed to initialize application: {}", e);
        return Err(e.into());
    }
    
    if let Err(e) = app.start().await {
        error!("Application error: {}", e);
        return Err(e.into());
    }
    
    Ok(())
}

fn init_logging() {
    // Respect RUST_LOG if provided, otherwise default to INFO
    let level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(Level::INFO);

    let _subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(_subscriber).expect("setting default subscriber failed");
    info!("Logging initialized at level: {:?}", level);
}

fn print_banner() {
    const BANNER: &str = r#"
    ____                   ___________      __         
   / __ \____  ___  ____  / ____/ ____/___  / /_  __  __
  / / / / __ \/ _ \/ __ \/ /   / / __/ __ \/ __ \/ / / /
 / /_/ / /_/ /  __/ / / / /___/ /_/ / /_/ / / / / /_/ / 
/_____/\____/\___/_/ /_/\____/\____/\____/_/ /_/\__, /  
                                               /____/   
    Multi-Channel AI Gateway v0.1.0
"#;
    
    println!("{}", BANNER);
}
