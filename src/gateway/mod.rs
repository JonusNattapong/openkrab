//! gateway — WebSocket + HTTP server for real-time communication.
//! Ported from `openkrab/src/gateway/` (Phase 7-8).
//!
//! Core infrastructure for real-time communication and API services.

pub mod auth;
pub mod client;
pub mod constants;
pub mod monitor_manager;
pub mod server;
pub mod types;

// Re-exports for convenience
pub use client::*;
pub use constants::*;
pub use server::{ClientConnection, GatewayServer, GatewayServerOptions};
pub use types::*;

/// Gateway state wrapper (for compatibility with old code)
pub type GatewayState = GatewayServer;

/// Start the gateway server
pub async fn start_gateway(opts: GatewayServerOptions) -> anyhow::Result<GatewayServer> {
    let cfg = crate::config_io::load_config().ok();
    if let Some(summary) = crate::plugins::loader::PluginManager::bootstrap_from_config(
        cfg.as_ref().and_then(|c| c.plugins.as_ref()),
    )
    .await?
    {
        tracing::info!(
            "Plugin bootstrap complete: loaded={}, failed_load={}, initialized={}, failed_init={}",
            summary.load.loaded,
            summary.load.failed,
            summary.init.initialized,
            summary.init.failed
        );
    }

    let port = opts.port.unwrap_or(18789);
    let bind_host = opts.bind_host.unwrap_or_else(|| "127.0.0.1".to_string());
    let server = GatewayServer::new(port, bind_host);
    Ok(server)
}
