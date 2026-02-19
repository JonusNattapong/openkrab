use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State, Json,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use crate::memory::{MemoryManager, HybridSearchOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::connectors::telegram;
use crate::connectors::discord;
use crate::connectors::line;
use crate::connectors::whatsapp;
use crate::dashboard;

// ─── Gateway status ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub healthy: bool,
    pub endpoint: String,
}

pub fn gateway_status() -> GatewayStatus {
    GatewayStatus {
        healthy: true,
        endpoint: "http://127.0.0.1:3000".to_string(),
    }
}

// ─── Shared gateway state ──────────────────────────────────────────────────────

pub struct GatewayState {
    pub memory: Arc<MemoryManager>,
    pub agent: Arc<crate::agents::Agent>,
}

// ─── WebSocket request / response types ───────────────────────────────────────

#[derive(Deserialize)]
#[serde(tag = "type")]
enum GatewayRequest {
    #[serde(rename = "memory/search")]
    MemorySearch { query: String },
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum GatewayResponse {
    #[serde(rename = "memory/results")]
    MemoryResults { results: Vec<crate::memory::store::SearchResult> },
    #[serde(rename = "error")]
    Error { message: String },
}

// ─── WhatsApp webhook verification query params ────────────────────────────────

#[derive(Deserialize)]
pub struct WhatsAppVerifyParams {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

// ─── Gateway startup ───────────────────────────────────────────────────────────

pub async fn start_gateway(state: Arc<GatewayState>) -> anyhow::Result<()> {
    // ── Spawn Telegram monitor ─────────────────────────────────────────────────
    if let Ok(token) = std::env::var("TELEGRAM_BOT_TOKEN") {
        let s = state.clone();
        tokio::spawn(async move {
            telegram::monitor(s, token).await;
        });
        tracing::info!("[gateway] Telegram monitor spawned.");
    }

    // ── Spawn Discord monitor ──────────────────────────────────────────────────
    if let Ok(token) = std::env::var("DISCORD_BOT_TOKEN") {
        let s = state.clone();
        tokio::spawn(async move {
            discord::monitor(s, token).await;
        });
        tracing::info!("[gateway] Discord monitor spawned.");
    }

    // ── Build router ───────────────────────────────────────────────────────────
    let app = Router::new()
        // ── Web Dashboard ──────────────────────────────────────────────────────
        .route("/", get(dashboard::dashboard_handler))
        .route("/api/status", get(dashboard::api_status_handler))
        .route("/api/chat", post(dashboard::api_chat_handler))
        .route("/api/memory", get(dashboard::api_memory_handler))
        .route("/health", get(dashboard::health_handler))
        // ── WebSocket (existing) ───────────────────────────────────────────────
        .route("/ws", get(ws_handler))
        // ── Platform webhooks ──────────────────────────────────────────────────
        .route("/slack/events", post(slack_events_handler))
        .route("/line/webhook", post(line_webhook_handler))
        .route("/whatsapp/webhook", get(whatsapp_verify_handler))
        .route("/whatsapp/webhook", post(whatsapp_events_handler))
        .route("/whatsapp/web/ws", get(whatsapp_web_ws_handler))
        // ── Middleware ─────────────────────────────────────────────────────────
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Gateway listening on http://{}", listener.local_addr()?);
    println!("🦀 Krabkrab dashboard: http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

// ─── Slack events handler (existing) ──────────────────────────────────────────

async fn slack_events_handler(
    State(state): State<Arc<GatewayState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Handle Slack URL Verification challenge
    if let Some(challenge) = payload.get("challenge").and_then(|v| v.as_str()) {
        return Json(json!({ "challenge": challenge }));
    }

    // Handle event callback
    if let Some(event) = payload.get("event") {
        let type_ = event.get("type").and_then(|v| v.as_str());
        if type_ == Some("message")
            && event.get("bot_id").is_none()
            && event.get("subtype").is_none()
        {
            if let Some(text) = event.get("text").and_then(|v| v.as_str()) {
                let channel = event
                    .get("channel")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                let state_clone = state.clone();
                let text_owned = text.to_string();
                let channel_owned = channel.clone();

                tokio::spawn(async move {
                    if let Ok(token) = std::env::var("SLACK_BOT_TOKEN") {
                        match state_clone.agent.answer(&text_owned).await {
                            Ok(answer) => {
                                let client = reqwest::Client::new();
                                let _ = crate::connectors::slack_client::send_message(
                                    &client,
                                    &token,
                                    &channel_owned,
                                    &answer,
                                    None,
                                )
                                .await;
                            }
                            Err(e) => eprintln!("[slack] Agent error: {}", e),
                        }
                    }
                });
            }
        }
    }

    Json(json!({ "ok": true }))
}

// ─── Line webhook handler ──────────────────────────────────────────────────────

async fn line_webhook_handler(
    State(state): State<Arc<GatewayState>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    use crate::connectors::line::signature::validate_line_signature;

    let channel_secret = std::env::var("LINE_CHANNEL_SECRET").unwrap_or_default();
    if channel_secret.is_empty() {
        tracing::warn!("LINE_CHANNEL_SECRET not set - skipping signature verification");
        return (StatusCode::OK, Json(json!({ "ok": true })));
    }

    let body_str = String::from_utf8_lossy(&body);
    let payload: serde_json::Value = match serde_json::from_str(&body_str) {
        Ok(p) => p,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid JSON payload" })));
        }
    };

    // LINE webhook verification sends POST {"events":[]} without a
    // signature header. Return 200 immediately so the LINE Developers
    // Console "Verify" button succeeds.
    let events = payload.get("events").and_then(|e| e.as_array());
    let is_verification = events.map(|a| a.is_empty()).unwrap_or(false);

    if is_verification {
        return (StatusCode::OK, Json(json!({ "status": "ok" })));
    }

    let signature = match headers.get("x-line-signature") {
        Some(v) => v.to_str().unwrap_or(""),
        None => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Missing X-Line-Signature header" })));
        }
    };

    if !validate_line_signature(&body_str, signature, &channel_secret) {
        tracing::warn!("line: webhook signature validation failed");
        return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Invalid signature" })));
    }

    line::handle_events(state, payload).await;
    (StatusCode::OK, Json(json!({ "ok": true })))
}

// ─── WhatsApp webhook verification (GET) ──────────────────────────────────────

async fn whatsapp_verify_handler(
    Query(params): Query<WhatsAppVerifyParams>,
) -> impl IntoResponse {
    let expected_token = std::env::var("WHATSAPP_VERIFY_TOKEN").unwrap_or_default();

    if params.mode.as_deref() == Some("subscribe")
        && params.verify_token.as_deref() == Some(&expected_token)
    {
        if let Some(challenge) = &params.challenge {
            return (StatusCode::OK, challenge.clone());
        }
    }
    (StatusCode::FORBIDDEN, "Verification failed".to_string())
}

// ─── WhatsApp events handler (POST) ───────────────────────────────────────────

async fn whatsapp_events_handler(
    State(state): State<Arc<GatewayState>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body);
    let payload: serde_json::Value = match serde_json::from_str(&body_str) {
        Ok(p) => p,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid JSON payload" })));
        }
    };

    let app_secret = std::env::var("WHATSAPP_APP_SECRET").unwrap_or_default();
    if !app_secret.is_empty() {
        let signature = match headers.get("x-hub-signature-256") {
            Some(v) => v.to_str().unwrap_or(""),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Missing X-Hub-Signature-256 header" })),
                );
            }
        };

        if !whatsapp::signature::validate_whatsapp_signature(&body_str, signature, &app_secret) {
            tracing::warn!("whatsapp: webhook signature validation failed");
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Invalid signature" })));
        }
    }

    whatsapp::handle_events(state, payload).await;
    (StatusCode::OK, Json(json!({ "ok": true })))
}

// ─── WhatsApp Web bridge websocket ─────────────────────────────────────────────

async fn whatsapp_web_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<GatewayState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_whatsapp_web_socket(socket, state))
}

async fn handle_whatsapp_web_socket(mut socket: WebSocket, state: Arc<GatewayState>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let payload: serde_json::Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) => {
                    let _ = socket
                        .send(Message::Text(
                            json!({ "type": "error", "error": format!("invalid-json: {e}") })
                                .to_string(),
                        ))
                        .await;
                    continue;
                }
            };

            let outbound = whatsapp::handle_web_bridge_event(state.clone(), payload).await;
            let _ = socket.send(Message::Text(outbound.to_string())).await;
        }
    }
}

// ─── WebSocket handler (existing) ─────────────────────────────────────────────

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<GatewayState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<GatewayState>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let req: Result<GatewayRequest, _> = serde_json::from_str(&text);
            match req {
                Ok(GatewayRequest::MemorySearch { query }) => {
                    let results = state
                        .memory
                        .search_hybrid(&query, HybridSearchOptions::default())
                        .await;
                    match results {
                        Ok(res) => {
                            let resp = GatewayResponse::MemoryResults { results: res };
                            let _ = socket
                                .send(Message::Text(serde_json::to_string(&resp).unwrap()))
                                .await;
                        }
                        Err(e) => {
                            let resp = GatewayResponse::Error { message: e.to_string() };
                            let _ = socket
                                .send(Message::Text(serde_json::to_string(&resp).unwrap()))
                                .await;
                        }
                    }
                }
                Err(e) => {
                    let resp = GatewayResponse::Error {
                        message: format!("Invalid request: {}", e),
                    };
                    let _ = socket
                        .send(Message::Text(serde_json::to_string(&resp).unwrap()))
                        .await;
                }
            }
        }
    }
}
