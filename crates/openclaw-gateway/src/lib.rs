use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tower_http::cors::CorsLayer;

use openclaw_core::{
    ChannelId, ChannelRegistry, Config, OpenClawError, Result, Session, SessionId,
};
use uuid::Uuid;
use openclaw_storage::{Storage, create_storage, StorageConfig, StorageBackend, SessionFilter};

mod auth;
mod protocol;
mod session;

pub use auth::{require_auth, AuthState};
pub use protocol::*;
pub use session::*;

pub struct GatewayServer {
    config: Config,
    channels: Arc<RwLock<ChannelRegistry>>,
    sessions: Arc<RwLock<HashMap<SessionId, mpsc::Sender<String>>>>,
    session_manager: Arc<RwLock<SessionManager>>,
    storage: Option<Arc<Box<dyn Storage>>>,
}

fn parse_session_id(s: &str) -> Result<SessionId> {
    Uuid::parse_str(s)
        .map(SessionId)
        .map_err(|e| OpenClawError::Serialization { message: format!("Invalid session ID: {}", e) })
}

impl GatewayServer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            channels: Arc::new(RwLock::new(ChannelRegistry::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_manager: Arc::new(RwLock::new(SessionManager::new())),
            storage: None,
        }
    }

    pub async fn init_storage(&mut self) -> Result<()> {
        let storage_config = StorageConfig {
            backend: StorageBackend::Sqlite,
            connection_string: format!("sqlite:{}", self.config.database.path),
            max_connections: self.config.database.max_connections,
            timeout_seconds: 30,
        };
        let storage = create_storage(&storage_config).await?;
        self.storage = Some(Arc::from(storage));
        Ok(())
    }

    fn storage(&self) -> Result<&Arc<Box<dyn Storage>>> {
        self.storage.as_ref().ok_or_else(|| OpenClawError::Storage {
            message: "Storage not initialized".to_string(),
        })
    }

    pub async fn start(mut self) -> Result<()> {
        let host = self.config.server.host.clone();
        let port = self.config.server.port;

        self.init_storage().await?;

        let auth_state = AuthState::new(self.config.security.api_keys.clone());

        let app = Router::new()
            .route("/ws", get(ws_handler))
            .route("/health", get(health_handler))
            .layer(CorsLayer::permissive())
            .with_state((Arc::new(self), auth_state));

        let addr: std::net::IpAddr = host.parse().map_err(|e| OpenClawError::Config {
            message: format!("Invalid host: {}", e),
        })?;
        let addr = SocketAddr::from((addr, port));

        tracing::info!("Starting gateway server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| OpenClawError::Network {
                source: Box::new(e),
            })?;

        axum::serve(listener, app)
            .await
            .map_err(|e| OpenClawError::Network {
                source: Box::new(e),
            })?;

        Ok(())
    }

    pub async fn register_channel(&self, channel: Arc<dyn openclaw_core::Channel>) -> Result<()> {
        let channel_id = channel.id();
        let channels = self.channels.write().await;
        channels.register(channel)?;
        tracing::info!("Registered channel: {}", channel_id);
        Ok(())
    }

    pub async fn channels(&self) -> Vec<ChannelId> {
        let channels = self.channels.read().await;
        channels.list()
    }

    pub async fn get_channel(&self, id: &ChannelId) -> Option<Arc<dyn openclaw_core::Channel>> {
        let channels = self.channels.read().await;
        channels.get(id)
    }

    pub async fn channel_status(&self, id: &ChannelId) -> serde_json::Value {
        let channels = self.channels.read().await;
        if let Some(channel) = channels.get(id) {
            serde_json::json!({
                "id": id.to_string(),
                "connected": channel.is_connected(),
                "name": channel.name(),
                "channel_type": format!("{:?}", channel.channel_type()),
            })
        } else {
            serde_json::json!({
                "id": id.to_string(),
                "error": "Channel not found"
            })
        }
    }

    pub async fn health_check(&self) -> serde_json::Value {
        let channels = self.channels.read().await;
        let channel_list = channels.list();
        
        let mut channel_health = Vec::new();
        for ch_id in &channel_list {
            if let Some(ch) = channels.get(ch_id) {
                channel_health.push(serde_json::json!({
                    "id": ch_id.to_string(),
                    "name": ch.name(),
                    "connected": ch.is_connected(),
                }));
            }
        }

        serde_json::json!({
            "status": "healthy",
            "channels": channel_health,
            "sessions": self.sessions.read().await.len(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }

    pub async fn route_message(&self, channel_id: &ChannelId, _message: serde_json::Value) -> Result<Option<SessionId>> {
        let channels = self.channels.read().await;
        
        if channels.get(channel_id).is_some() {
            let session_id = SessionId::new();
            let mut session_manager = self.session_manager.write().await;
            session_manager.add(session_id.clone());
            
                    tracing::trace!("Routed message to session: {}", session_id);
            Ok(Some(session_id))
        } else {
            tracing::warn!("Channel not found: {}", channel_id);
            Ok(None)
        }
    }

    pub async fn session_list(&self) -> Vec<serde_json::Value> {
        let manager = self.session_manager.read().await;
        let memory_sessions: std::collections::HashSet<_> = manager.list().into_iter().collect();
        
        let mut sessions = Vec::new();
        
        // Add memory sessions
        for session_id in &memory_sessions {
            sessions.push(serde_json::json!({
                "session_id": session_id.0.to_string(),
                "active": true
            }));
        }
        
        // Add storage sessions (excluding those already in memory)
        if let Some(storage) = self.storage.as_ref() {
            match storage.list_sessions(SessionFilter::new()).await {
                Ok(storage_sessions) => {
                    for session in storage_sessions {
                        if !memory_sessions.contains(&session.id) {
                            sessions.push(serde_json::json!({
                                "session_id": session.id.0.to_string(),
                                "active": false,
                                "name": session.name,
                                "channel_id": session.channel_id.0.to_string(),
                                "created_at": session.created_at.to_rfc3339(),
                            }));
                        }
                    }
                }
                Err(e) => tracing::warn!("Failed to list sessions from storage: {}", e),
            }
        }
        
        sessions
    }

    pub async fn session_get(&self, session_id: &SessionId) -> Option<serde_json::Value> {
        let manager = self.session_manager.read().await;
        if manager.get(session_id).is_some() {
            return Some(serde_json::json!({
                "session_id": session_id.0.to_string(),
                "active": true
            }));
        }
        
        if let Some(storage) = self.storage.as_ref() {
            match storage.get_session(session_id.clone()).await {
                Ok(Some(session)) => {
                    tracing::trace!("Session loaded from storage: {}", session_id);
                    return Some(serde_json::json!({
                        "session_id": session_id.0.to_string(),
                        "active": false,
                        "name": session.name,
                        "channel_id": session.channel_id.0.to_string(),
                        "created_at": session.created_at.to_rfc3339(),
                    }));
                }
                Ok(None) => {}
                Err(e) => tracing::warn!("Failed to load session from storage: {}", e),
            }
        }
        
        None
    }

    pub async fn session_create(&self) -> SessionId {
        let channel_id = ChannelId::new();
        let session = Session::new(channel_id, None);
        let session_id = session.id.clone();
        
        let mut manager = self.session_manager.write().await;
        manager.add(session_id.clone());
        
        if let Some(storage) = self.storage.as_ref() {
            match storage.save_session(&session).await {
                Ok(_) => tracing::trace!("Session saved to storage: {}", session_id),
                Err(e) => tracing::warn!("Failed to save session to storage: {}", e),
            }
        }
        
        session_id
    }

    pub async fn broadcast(&self, message: serde_json::Value) -> Result<()> {
        let sessions = self.sessions.read().await;
        for sender in sessions.values() {
            let _ = sender.send(serde_json::to_string(&message).unwrap_or_default()).await;
        }
        Ok(())
    }

pub async fn init_channels_from_config(&self) -> Result<()> {
        let mut channels = self.channels.write().await;
        
        for channel_config in &self.config.channels {
            if !channel_config.enabled {
                tracing::info!("Channel {} is disabled, skipping", channel_config.name);
                continue;
            }

            match channel_config.channel_type.as_str() {
                "telegram" => {
                    tracing::info!("Creating Telegram channel '{}'", channel_config.name);
                    
                    let token = channel_config.config.get("token")
                        .and_then(|v: &serde_json::Value| v.as_str())
                        .ok_or_else(|| OpenClawError::Config {
                            message: "Telegram token missing".to_string(),
                        })?;
                    
                    // Create Telegram channel instance using ChannelConfig path from core
                    let telegram_cfg = openclaw_core::channel::ChannelConfig {
                        id: openclaw_core::ChannelId::new(),
                        name: channel_config.name.clone(),
                        channel_type: openclaw_core::ChannelType::Telegram,
                        enabled: channel_config.enabled,
                        config: serde_json::Value::Object(channel_config.config.clone().into_iter().collect()),
                    };

                    let telegram_channel = openclaw_telegram::TelegramChannel::new(token, telegram_cfg);
                    channels.register(Arc::new(telegram_channel) as Arc<dyn openclaw_core::Channel>)?;
                    tracing::info!("Telegram channel '{}' registered", channel_config.name);
                }
                "discord" => {
                    tracing::info!("Creating Discord channel '{}'", channel_config.name);
                    
                    let token = channel_config.config.get("token")
                        .and_then(|v: &serde_json::Value| v.as_str())
                        .ok_or_else(|| OpenClawError::Config {
                            message: "Discord token missing".to_string(),
                        })?;
                    
                    let discord_config = openclaw_core::channel::ChannelConfig {
                        id: openclaw_core::ChannelId::new(),
                        name: channel_config.name.clone(),
                        channel_type: openclaw_core::ChannelType::Discord,
                        enabled: channel_config.enabled,
                        config: serde_json::Value::Object(channel_config.config.clone().into_iter().collect()),
                    };
                    
                    let discord_channel = openclaw_discord::DiscordChannel::new(token, discord_config);
                    channels.register(Arc::new(discord_channel) as Arc<dyn openclaw_core::Channel>)?;
                    tracing::info!("Discord channel '{}' registered", channel_config.name);
                }
                "slack" => {
                    tracing::info!("Slack channel '{}' configured (implementation pending)", channel_config.name);
                }
                _ => {
                    tracing::warn!("Unknown channel type: {}", channel_config.channel_type);
                }
            }
        }
        Ok(())
    }
}

async fn health_handler(
    axum::extract::State(state): axum::extract::State<(Arc<GatewayServer>, AuthState)>,
    headers: axum::http::HeaderMap,
) -> Response {
    let auth_state = &state.1;
    if let Err(_) = require_auth(auth_state, &headers) {
        return (axum::http::StatusCode::UNAUTHORIZED, r#"{"error":"Unauthorized"}"#).into_response();
    }
    axum::response::Json(serde_json::json!({ "status": "ok" })).into_response()
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<(Arc<GatewayServer>, AuthState)>,
    headers: axum::http::HeaderMap,
) -> Response {
    let auth_state = &state.1;
    if let Err(_) = require_auth(auth_state, &headers) {
        return (axum::http::StatusCode::UNAUTHORIZED, r#"{"error":"Unauthorized"}"#).into_response();
    }
    let server = state.0.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, server))
}

async fn handle_socket(socket: WebSocket, server: Arc<GatewayServer>) {
    let session_id = SessionId::new();
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let (mut ws_sender, mut ws_receiver) = socket.split();

    {
        let mut sessions = server.sessions.write().await;
        sessions.insert(session_id.clone(), tx.clone());
    }

    {
        let mut manager = server.session_manager.write().await;
        manager.add(session_id.clone());
    }

    tracing::info!("New WebSocket connection: {}", session_id);

    let server_clone = server.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let response = handle_message(&server_clone, &tx, &text).await;
                if let Some(resp) = response {
                    let _ = tx.send(resp).await;
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket connection closed: {}", session_id);
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    {
        let mut sessions = server.sessions.write().await;
        sessions.remove(&session_id);
    }

    {
        let mut manager = server.session_manager.write().await;
        manager.remove(&session_id);
    }

    tracing::info!("WebSocket connection cleaned up: {}", session_id);
}

async fn handle_message(
    server: &Arc<GatewayServer>,
    _sender: &mpsc::Sender<String>,
    message: &str,
) -> Option<String> {
    let request: serde_json::Value = match serde_json::from_str(message) {
        Ok(v) => v,
        Err(e) => {
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32700, "message": "Parse error", "data": e.to_string() },
                "id": null
            });
            return Some(response.to_string());
        }
    };

    let method = request.get("method").and_then(|v| v.as_str());
    let params = request.get("params").cloned().unwrap_or(serde_json::Value::Null);
    let id = request.get("id").cloned();

    let response = match method {
        Some("ping") => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": { "pong": true, "timestamp": chrono::Utc::now().to_rfc3339() },
                "id": id
            })
        }
        Some("list_channels") => {
            let channels = server.channels().await;
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": { "channels": channels.iter().map(|c| c.0.to_string()).collect::<Vec<_>>() },
                "id": id
            })
        }
        Some("channel_status") => {
            let channel_id = params.get("channel_id").and_then(|v| v.as_str());
            if channel_id.is_some() {
                let status = server.channel_status(&ChannelId::new()).await;
                serde_json::json!({ "jsonrpc": "2.0", "result": status, "id": id })
            } else {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32602, "message": "Missing channel_id" },
                    "id": id
                })
            }
        }
        Some("init_channels") => {
            match server.init_channels_from_config().await {
                Ok(_) => serde_json::json!({ "jsonrpc": "2.0", "result": { "status": "initialized" }, "id": id }),
                Err(e) => serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32603, "message": e.to_string() },
                    "id": id
                })
            }
        }
        Some("session_list") => {
            let sessions = server.session_list().await;
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": { "sessions": sessions },
                "id": id
            })
        }
        Some("session_create") => {
            let session_id = server.session_create().await;
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": { "session_id": session_id.0.to_string() },
                "id": id
            })
        }
        Some("session_get") => {
            let session_id_str = params.get("session_id").and_then(|v| v.as_str());
            if let Some(sid) = session_id_str {
                match parse_session_id(sid) {
                    Ok(session_id) => {
                        let session = server.session_get(&session_id).await;
                        match session {
                            Some(s) => serde_json::json!({ "jsonrpc": "2.0", "result": s, "id": id }),
                            None => serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": { "code": -32602, "message": "Session not found" },
                                "id": id
                            })
                        }
                    }
                    Err(e) => serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": { "code": -32602, "message": e.to_string() },
                        "id": id
                    })
                }
            } else {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32602, "message": "Missing session_id" },
                    "id": id
                })
            }
        }
        Some(m) => {
            tracing::warn!("Unknown method: {}", m);
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32601, "message": "Method not found" },
                "id": id
            })
        }
        None => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32600, "message": "Invalid Request" },
                "id": id
            })
        }
    };

    Some(response.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openclaw_core::Config;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_init_channels_from_config_discord() {
        // Create a mock config with Discord channel
        let mut config = Config::default();
        
        let mut discord_config = HashMap::new();
        discord_config.insert("token".to_string(), serde_json::Value::String("test_token".to_string()));
        discord_config.insert("channel_id".to_string(), serde_json::Value::Number(serde_json::Number::from(123456789)));
        
        let channel_config = openclaw_core::config::ChannelConfig {
            id: "discord-test".to_string(),
            name: "Test Discord".to_string(),
            channel_type: "discord".to_string(),
            enabled: true,
            config: discord_config,
        };
        
        config.channels.push(channel_config);
        
        // Create GatewayServer
        let server = GatewayServer::new(config);
        
        // Initialize channels from config
        let result = server.init_channels_from_config().await;
        assert!(result.is_ok(), "Failed to init channels: {:?}", result.err());
        
        // Check that channel was registered
        let channels = server.channels().await;
        assert!(!channels.is_empty(), "No channels registered");
        
        // Verify channel status
        let channel_status = server.channel_status(&channels[0]).await;
        assert!(channel_status.get("error").is_none(), "Channel error: {:?}", channel_status);
    }
}
