//! gateway — WebSocket + HTTP server for real-time communication.
//! Ported from `openkrab/src/gateway/` (Phase 7-8).
//!
//! Core infrastructure for real-time communication and API services.

pub mod auth;
pub mod client;
pub mod config_reload;
pub mod constants;
pub mod heartbeat;
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

    // Start heartbeat runner
    let heartbeat = crate::gateway::heartbeat::start_heartbeat_runner(
        Default::default(),
        vec![], // Add targets later
        None,
    );
    *server.heartbeat_runner.write().await = Some(heartbeat);

    // Start config reloader
    if let Ok(config_path) = crate::config_io::resolve_config_path() {
        let initial_config = crate::config_io::load_config_value().unwrap_or_default();
        let reloader = crate::gateway::config_reload::start_gateway_config_reloader(
            config_path,
            initial_config,
            std::sync::Arc::new(|| crate::config_io::load_config_value()),
            Box::new(|plan, _next_cfg| {
                tracing::info!("Gateway hot-reloading: {:?}", plan.hot_reasons);
            }),
            Box::new(|_plan, _next_cfg| {
                tracing::warn!("Gateway restart required due to config changes.");
            }),
            Default::default(),
        )?;
        *server.config_reloader.write().await = Some(reloader);
    }

    Ok(server)
}
