use openclaw_core::Config;
use openclaw_gateway::GatewayServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Respect RUST_LOG if set; otherwise default to INFO
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let config = load_config()?;
    let server = GatewayServer::new(config);

    tracing::info!("Starting OpenClaw Gateway Server...");
    server.start().await?;

    Ok(())
}

fn load_config() -> anyhow::Result<Config> {
    let path = std::path::PathBuf::from("openclaw.toml");

    if path.exists() {
        Config::load("openclaw.toml")
    } else {
        tracing::warn!("Config file not found, using defaults");
        Ok(Config::default())
    }
}
