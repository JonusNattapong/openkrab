//! gateway — WebSocket + HTTP server for real-time communication.
//! Ported from `openclaw/src/gateway/` (Phase 7-8).
//!
//! Core infrastructure for real-time communication and API services.

pub mod server;
pub mod client;
pub mod auth;
pub mod types;
pub mod constants;
pub mod monitor_manager;

// Re-exports for convenience
pub use constants::*;
pub use types::*;
pub use server::{GatewayServer, GatewayServerOptions, ClientConnection};
pub use client::*;

/// Gateway state wrapper (for compatibility with old code)
pub type GatewayState = GatewayServer;

/// Start the gateway server
pub async fn start_gateway(opts: GatewayServerOptions) -> anyhow::Result<GatewayServer> {
    let port = opts.port.unwrap_or(18789);
    let bind_host = opts.bind_host.unwrap_or_else(|| "127.0.0.1".to_string());
    let server = GatewayServer::new(port, bind_host);
    Ok(server)
}