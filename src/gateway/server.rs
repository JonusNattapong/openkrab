use axum::{
    extract::{ws::Message, ws::WebSocketUpgrade, State},
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

use crate::gateway::types::*;

pub type ClientId = String;
pub type ConnectionId = u64;

#[derive(Clone)]
pub struct GatewayServer {
    pub port: u16,
    pub bind_host: String,
    pub clients: Arc<RwLock<HashMap<ConnectionId, ClientConnection>>>,
    pub next_connection_id: Arc<RwLock<ConnectionId>>,
    // Agent for processing messages
    pub agent: Option<Arc<crate::agents::Agent>>,
    pub memory: Option<Arc<crate::memory::MemoryManager>>,
    /// Thread-safe session registry
    pub sessions: Arc<RwLock<crate::sessions::SessionRegistry>>,
    /// Heartbeat runner for channel health monitoring
    pub heartbeat_runner: Arc<RwLock<Option<crate::gateway::heartbeat::HeartbeatRunner>>>,
    /// Handle for config hot-reloading
    pub config_reloader: Arc<RwLock<Option<crate::gateway::config_reload::ConfigReloaderHandle>>>,
}

impl std::fmt::Debug for GatewayServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GatewayServer")
            .field("port", &self.port)
            .field("bind_host", &self.bind_host)
            .field("clients", &"<async RwLock>")
            .field("agent", &self.agent.is_some())
            .field("memory", &self.memory.is_some())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub id: ConnectionId,
    pub client_id: Option<String>,
    pub addr: SocketAddr,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct GatewayServerOptions {
    pub port: Option<u16>,
    pub bind_host: Option<String>,
    pub enable_cors: bool,
}

impl Default for GatewayServerOptions {
    fn default() -> Self {
        Self {
            port: Some(18789),
            bind_host: Some("127.0.0.1".to_string()),
            enable_cors: true,
        }
    }
}

impl GatewayServer {
    pub fn new(port: u16, bind_host: String) -> Self {
        Self {
            port,
            bind_host,
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_connection_id: Arc::new(RwLock::new(1)),
            agent: None,
            memory: None,
            sessions: Arc::new(RwLock::new(crate::sessions::SessionRegistry::new())),
            heartbeat_runner: Arc::new(RwLock::new(None)),
            config_reloader: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn broadcast(
        &self,
        message: GatewayMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(&message)?;
        let clients = self.clients.read().await;

        // In a real implementation, we'd have WebSocket senders stored here
        // For now, just log the broadcast
        tracing::debug!(
            "Broadcasting message to {} clients: {}",
            clients.len(),
            json
        );

        Ok(())
    }

    pub async fn send_to_client(
        &self,
        client_id: &str,
        message: GatewayMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string(&message)?;
        let clients = self.clients.read().await;

        // Find client by client_id
        for client in clients.values() {
            if client.client_id.as_deref() == Some(client_id) {
                tracing::debug!("Sending message to client {}: {}", client_id, json);
                break;
            }
        }

        Ok(())
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("Closing gateway server");
        // Close all client connections
        let mut clients = self.clients.write().await;
        clients.clear();
        Ok(())
    }
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    State(server): State<Arc<GatewayServer>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, server))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, server: Arc<GatewayServer>) {
    let connection_id = {
        let mut next_id = server.next_connection_id.write().await;
        let id = *next_id;
        *next_id += 1;
        id
    };

    let addr = "127.0.0.1:0".parse().unwrap(); // Placeholder
    let client = ClientConnection {
        id: connection_id,
        client_id: None,
        addr,
        connected_at: chrono::Utc::now(),
    };

    // Add to clients map
    {
        let mut clients = server.clients.write().await;
        clients.insert(connection_id, client);
    }

    tracing::info!("WebSocket connection established: {}", connection_id);

    let (mut sender, mut receiver) = socket.split();

    // Send hello message
    let hello = GatewayMessage::Hello {
        client_id: format!("client-{}", connection_id),
        version: env!("CARGO_PKG_VERSION").to_string(),
        capabilities: vec!["chat".to_string(), "status".to_string()],
    };

    if let Ok(json) = serde_json::to_string(&hello) {
        if sender.send(Message::Text(json.into())).await.is_err() {
            tracing::warn!("Failed to send hello message to client {}", connection_id);
        }
    }

    // Message handling loop
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::debug!("Received message from {}: {}", connection_id, text);

                // Parse incoming message
                match serde_json::from_str::<GatewayMessage>(&text) {
                    Ok(GatewayMessage::Chat {
                        session_key,
                        message,
                        attachments: _,
                    }) => {
                        tracing::info!("Chat message from {}: {}", session_key, message);

                        // Emit Inbound Hook
                        let mut payload = crate::hooks::HookPayload::new();
                        payload.set("session_key", session_key.clone());
                        payload.set("message", message.clone());
                        payload.set("connection_id", connection_id as i64);
                        crate::hooks::emit(crate::hooks::events::MESSAGE_INBOUND, &payload);

                        if let Some(agent) = &server.agent {
                            // Get or create session
                            let mut sessions_lock = server.sessions.write().await;
                            let session = sessions_lock.get_or_create(&session_key);
                            session.append_transcript(crate::sessions::TranscriptEntry::user(&message));
                            
                            // Drop lock before async call
                            drop(sessions_lock);

                            // Process message
                            let mut sessions_lock = server.sessions.write().await;
                            let reply = agent.answer_session(sessions_lock.get_or_create(&session_key), None).await;
                            drop(sessions_lock);

                            match reply {
                                Ok(text) => {
                                    // Emit Outbound Hook
                                    let mut out_payload = crate::hooks::HookPayload::new();
                                    out_payload.set("session_key", session_key.clone());
                                    out_payload.set("reply", text.clone());
                                    crate::hooks::emit(crate::hooks::events::MESSAGE_OUTBOUND, &out_payload);

                                    let response = GatewayMessage::Chat {
                                        session_key: session_key.clone(),
                                        message: text,
                                        attachments: None,
                                    };
                                    if let Ok(json) = serde_json::to_string(&response) {
                                        let _ = sender.send(Message::Text(json.into())).await;
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Agent error: {}", e);
                                    let error_msg = GatewayMessage::Error {
                                        code: "agent_error".to_string(),
                                        message: e.to_string(),
                                    };
                                    if let Ok(json) = serde_json::to_string(&error_msg) {
                                        let _ = sender.send(Message::Text(json.into())).await;
                                    }
                                }
                            }
                        }
                    }
                    Ok(GatewayMessage::Status { .. }) => {
                        let sessions_registry = server.sessions.read().await;
                        let sessions: Vec<GatewaySessionRow> = sessions_registry.iter().map(|(_, s)| GatewaySessionRow::from(s)).collect();
                        
                        let agents = if let Some(agent) = &server.agent {
                            vec![GatewayAgentRow {
                                id: agent.identity.name.clone().to_lowercase(),
                                name: Some(agent.identity.name.clone()),
                                identity: Some(crate::gateway::types::AgentIdentity {
                                    name: Some(agent.identity.name.clone()),
                                    theme: None,
                                    emoji: Some(agent.identity.emoji.clone()),
                                    avatar: None,
                                    avatar_url: None,
                                }),
                            }]
                        } else {
                            vec![]
                        };

                        let response = GatewayMessage::Status { sessions, agents };
                        if let Ok(json) = serde_json::to_string(&response) {
                            let _ = sender.send(Message::Text(json.into())).await;
                        }
                    }
                    _ => {
                        tracing::warn!("Unknown message type from client {}", connection_id);
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket connection closed: {}", connection_id);
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error for client {}: {}", connection_id, e);
                break;
            }
            _ => {} // Ignore other message types
        }
    }

    // Remove from clients map
    {
        let mut clients = server.clients.write().await;
        clients.remove(&connection_id);
    }

    tracing::info!("WebSocket connection cleaned up: {}", connection_id);
}

pub async fn start_gateway_server(
    opts: GatewayServerOptions,
) -> Result<GatewayServer, Box<dyn std::error::Error + Send + Sync>> {
    let port = opts.port.unwrap_or(18789);
    let bind_host = opts.bind_host.unwrap_or_else(|| "127.0.0.1".to_string());

    let server = Arc::new(GatewayServer::new(port, bind_host.clone()));

    let app = Router::new()
        .route("/ws", get(handle_websocket))
        .route("/health", get(|| async { "OK" }))
        .with_state(server.clone());

    let app = if opts.enable_cors {
        app.layer(CorsLayer::permissive())
    } else {
        app
    };

    let addr: SocketAddr = format!("{}:{}", bind_host, port).parse()?;
    tracing::info!("Starting gateway server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let _server_handle = tokio::spawn(async move { axum::serve(listener, app).await });

    // In a real implementation, we'd return a handle to stop the server
    // For now, just return the GatewayServer instance
    Ok((*server).clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gateway_server_creation() {
        let server = GatewayServer::new(18789, "127.0.0.1".to_string());
        assert_eq!(server.port, 18789);
        assert_eq!(server.bind_host, "127.0.0.1");
    }

    #[tokio::test]
    async fn test_broadcast() {
        let server = GatewayServer::new(18789, "127.0.0.1".to_string());
        let message = GatewayMessage::Status {
            sessions: vec![],
            agents: vec![],
        };

        let result = server.broadcast(message).await;
        assert!(result.is_ok());
    }
}
