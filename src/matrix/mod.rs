//! matrix — Matrix messaging connector.
//! Ported from `openclaw/extensions/matrix/` (Phase 10, 13).
//! Uses Matrix Client-Server API directly instead of the SDK.

pub mod credentials;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

pub type MatrixEventSender = mpsc::Sender<MatrixMonitorEvent>;
pub type MatrixEventReceiver = mpsc::Receiver<MatrixMonitorEvent>;

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    /// The homeserver URL, e.g. "https://matrix.org".
    pub homeserver: String,
    /// The access token for the bot user.
    pub access_token: String,
    /// The bot's full Matrix ID, e.g. "@krabbot:matrix.org".
    pub user_id: String,
    /// Enable E2EE (requires crypto storage)
    pub encryption: Option<bool>,
    /// Sync timeout in ms
    pub sync_timeout_ms: Option<u64>,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            homeserver: std::env::var("MATRIX_HOMESERVER")
                .unwrap_or_else(|_| "https://matrix.org".to_string()),
            access_token: std::env::var("MATRIX_ACCESS_TOKEN").unwrap_or_default(),
            user_id: std::env::var("MATRIX_USER_ID").unwrap_or_default(),
            encryption: None,
            sync_timeout_ms: Some(30_000),
        }
    }
}

impl MatrixConfig {
    pub fn from_env() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        if self.access_token.is_empty() {
            bail!("MATRIX_ACCESS_TOKEN is required");
        }
        if self.user_id.is_empty() {
            bail!("MATRIX_USER_ID is required");
        }
        if !self.user_id.starts_with('@') {
            bail!("MATRIX_USER_ID must be in format @user:server");
        }
        Ok(())
    }
}

// ─── Matrix event types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixEvent {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "room_id")]
    pub room_id: String,
    pub sender: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub content: serde_json::Value,
    #[serde(rename = "origin_server_ts")]
    pub origin_server_ts: i64,
    #[serde(rename = "redacts")]
    pub redacts: Option<String>,
    #[serde(rename = "unsigned")]
    pub unsigned: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixTextContent {
    pub body: String,
    pub msgtype: String,
    #[serde(rename = "m.relates_to")]
    pub relates_to: Option<MatrixRelatesTo>,
    pub format: Option<String>,
    pub formatted_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixRelatesTo {
    #[serde(rename = "m.in_reply_to")]
    pub in_reply_to: Option<MatrixInReplyTo>,
    pub rel_type: Option<String>,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixInReplyTo {
    pub event_id: String,
}

/// Parsed text message from Matrix.
#[derive(Debug, Clone)]
pub struct ParsedMatrixMessage {
    pub event_id: String,
    pub room_id: String,
    pub sender: String,
    pub body: String,
    pub formatted_body: Option<String>,
    pub reply_to_event_id: Option<String>,
    pub timestamp: i64,
}

pub fn parse_text_event(event: &MatrixEvent) -> Option<ParsedMatrixMessage> {
    if event.kind != "m.room.message" {
        return None;
    }
    let content: MatrixTextContent = serde_json::from_value(event.content.clone()).ok()?;
    if content.msgtype != "m.text" && content.msgtype != "m.emote" {
        return None;
    }

    let reply_to = content
        .relates_to
        .as_ref()
        .and_then(|r| r.in_reply_to.as_ref())
        .map(|r| r.event_id.clone());

    Some(ParsedMatrixMessage {
        event_id: event.event_id.clone(),
        room_id: event.room_id.clone(),
        sender: event.sender.clone(),
        body: content.body,
        formatted_body: content.formatted_body,
        reply_to_event_id: reply_to,
        timestamp: event.origin_server_ts,
    })
}

// ─── Sync response ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct MatrixSyncResponse {
    #[serde(rename = "next_batch")]
    pub next_batch: String,
    pub rooms: Option<MatrixSyncRooms>,
    pub presence: Option<serde_json::Value>,
    pub account_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct MatrixSyncRooms {
    pub join: Option<std::collections::HashMap<String, MatrixJoinedRoom>>,
    pub invite: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub leave: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct MatrixJoinedRoom {
    #[serde(rename = "timeline")]
    pub timeline: Option<MatrixTimeline>,
    pub state: Option<MatrixState>,
    pub ephemeral: Option<serde_json::Value>,
    pub account_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct MatrixTimeline {
    pub events: Vec<MatrixEvent>,
    pub limited: Option<bool>,
    #[serde(rename = "prev_batch")]
    pub prev_batch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatrixState {
    pub events: Vec<MatrixEvent>,
}

pub fn extract_text_messages(sync: &MatrixSyncResponse) -> Vec<ParsedMatrixMessage> {
    let mut out = Vec::new();
    if let Some(rooms) = &sync.rooms {
        if let Some(joined) = &rooms.join {
            for room in joined.values() {
                if let Some(timeline) = &room.timeline {
                    for event in &timeline.events {
                        if let Some(msg) = parse_text_event(event) {
                            out.push(msg);
                        }
                    }
                }
            }
        }
    }
    out
}

// ─── Room membership types ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct MatrixMembersResponse {
    pub chunk: Vec<MatrixEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixRoomJoin {
    #[serde(rename = "room_id")]
    pub room_id: String,
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

pub fn build_text_event(body: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": body
    })
}

pub fn build_formatted_text_event(body: &str, formatted: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": body,
        "format": "org.matrix.custom.html",
        "formatted_body": formatted
    })
}

pub fn build_emote_event(body: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.emote",
        "body": body
    })
}

pub fn build_reply_event(body: &str, reply_to_event_id: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": body,
        "m.relates_to": {
            "m.in_reply_to": {
                "event_id": reply_to_event_id
            }
        }
    })
}

pub fn build_reaction_event(emoji: &str, target_event_id: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.reaction",
        "body": emoji,
        "m.relates_to": {
            "rel_type": "m.annotation",
            "event_id": target_event_id
        }
    })
}

/// URL-en forcode room ID Matrix API (replace ! with %21, : with %3A)
pub fn encode_room_id(s: &str) -> String {
    s.replace('!', "%21").replace(':', "%3A")
}

/// Send a text message to a Matrix room.
pub async fn send_message(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    body: &str,
) -> Result<String> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let txn_id = format!("krab_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        txn_id
    );
    let payload = build_text_event(body);
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Send a formatted text message to a Matrix room.
pub async fn send_formatted_message(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    body: &str,
    formatted: &str,
) -> Result<String> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let txn_id = format!("krab_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        txn_id
    );
    let payload = build_formatted_text_event(body, formatted);
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Send an HTML message to a Matrix room.
pub async fn send_html_message(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    plain_text: &str,
    html: &str,
) -> Result<String> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let txn_id = format!("krab_html_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        txn_id
    );
    let payload = build_formatted_text_event(plain_text, html);
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Send a notice message (bot-like, unformatted).
pub async fn send_notice(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    text: &str,
) -> Result<String> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let txn_id = format!("krab_notice_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        txn_id
    );
    let payload = serde_json::json!({
        "msgtype": "m.notice",
        "body": text
    });
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Send a reaction to a message.
pub async fn send_reaction(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    target_event_id: &str,
    emoji: &str,
) -> Result<String> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let txn_id = format!("krab_react_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.reaction/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        txn_id
    );
    let payload = build_reaction_event(emoji, target_event_id);
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Redact (delete) a message.
pub async fn redact_message(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
    event_id: &str,
    reason: Option<&str>,
) -> Result<String> {
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/redact/{}/{}",
        cfg.homeserver,
        encode_room_id(room_id),
        event_id,
        format!(
            "krab_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        )
    );
    let mut payload = serde_json::json!({});
    if let Some(r) = reason {
        payload["reason"] = r.into();
    }
    let resp = client
        .put(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["event_id"].as_str().unwrap_or("").to_string())
}

/// Join a room.
pub async fn join_room(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id_or_alias: &str,
) -> Result<MatrixRoomJoin> {
    let url = format!(
        "{}/_matrix/client/v3/join/{}",
        cfg.homeserver, room_id_or_alias
    );
    let payload = serde_json::json!({});
    let resp = client
        .post(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    let json: serde_json::Value = resp.json().await?;
    let room_id = json["room_id"].as_str().unwrap_or("").to_string();
    Ok(MatrixRoomJoin { room_id })
}

/// Leave a room.
pub async fn leave_room(client: &reqwest::Client, cfg: &MatrixConfig, room_id: &str) -> Result<()> {
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/leave",
        cfg.homeserver,
        encode_room_id(room_id)
    );
    let payload = serde_json::json!({});
    let _resp = client
        .post(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

/// Get room members.
pub async fn get_room_members(
    client: &reqwest::Client,
    cfg: &MatrixConfig,
    room_id: &str,
) -> Result<Vec<String>> {
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/members",
        cfg.homeserver,
        encode_room_id(room_id)
    );
    let resp = client
        .get(&url)
        .bearer_auth(&cfg.access_token)
        .send()
        .await?
        .error_for_status()?;
    let data: MatrixMembersResponse = resp.json().await?;
    let members: Vec<String> = data
        .chunk
        .iter()
        .filter(|e| e.kind == "m.room.member")
        .filter(|e| {
            if let Some(content) = e.content.get("membership") {
                content.as_str() == Some("join")
            } else {
                false
            }
        })
        .map(|e| e.sender.clone())
        .collect();
    Ok(members)
}

// ─── Normalize to common message ─────────────────────────────────────────────

pub fn normalize_inbound(msg: &ParsedMatrixMessage) -> crate::common::Message {
    crate::common::Message {
        id: format!("matrix:{}", msg.event_id),
        text: msg.body.clone(),
        from: Some(crate::common::UserId(format!("matrix:{}", msg.sender))),
    }
}

// ─── Event types for monitor ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum MatrixMonitorEvent {
    Message(ParsedMatrixMessage),
    Reaction {
        room_id: String,
        sender: String,
        target_event_id: String,
        emoji: String,
        timestamp: i64,
    },
    RoomJoin {
        room_id: String,
        user_id: String,
    },
    RoomLeave {
        room_id: String,
        user_id: String,
    },
    Connected,
    Disconnected,
    Error(String),
}

pub fn create_matrix_channel() -> (
    mpsc::Sender<MatrixMonitorEvent>,
    mpsc::Receiver<MatrixMonitorEvent>,
) {
    mpsc::channel(100)
}

// ─── Monitor ─────────────────────────────────────────────────────────────────

pub struct Monitor {
    client: reqwest::Client,
    config: Arc<MatrixConfig>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl Monitor {
    pub fn new(config: MatrixConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Arc::new(config),
            shutdown_tx: None,
        }
    }

    /// Start the sync loop, returning a channel for events.
    pub async fn start(&mut self) -> Result<MatrixEventReceiver> {
        let (event_tx, event_rx) = mpsc::channel::<MatrixMonitorEvent>(100);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);

        let config = self.config.clone();
        let event_tx_clone = event_tx.clone();
        let mut next_batch = String::new();

        tokio::spawn(async move {
            let _ = event_tx_clone.send(MatrixMonitorEvent::Connected).await;

            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        let _ = event_tx_clone.send(MatrixMonitorEvent::Disconnected).await;
                        break;
                    }
                    _ = Self::sync_loop(&config, &mut next_batch, event_tx_clone.clone()) => {
                        // Error in sync, continue loop with backoff
                    }
                }
            }
        });

        Ok(event_rx)
    }

    async fn sync_loop(
        config: &Arc<MatrixConfig>,
        next_batch: &mut String,
        event_tx: MatrixEventSender,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let timeout = config.sync_timeout_ms.unwrap_or(30_000);

        loop {
            let url = if next_batch.is_empty() {
                format!(
                    "{}/_matrix/client/v3/sync?timeout={}",
                    config.homeserver, timeout
                )
            } else {
                format!(
                    "{}/_matrix/client/v3/sync?timeout={}&since={}",
                    config.homeserver, timeout, next_batch
                )
            };

            let resp = client
                .get(&url)
                .bearer_auth(&config.access_token)
                .send()
                .await?
                .error_for_status()?;

            let sync: MatrixSyncResponse = resp.json().await?;
            *next_batch = sync.next_batch.clone();

            // Process messages
            for msg in extract_text_messages(&sync) {
                let _ = event_tx.send(MatrixMonitorEvent::Message(msg)).await;
            }

            // Process room join/leave
            if let Some(rooms) = &sync.rooms {
                if let Some(joined) = &rooms.join {
                    for (room_id, room) in joined {
                        if let Some(state) = &room.state {
                            for event in &state.events {
                                if event.kind == "m.room.member" {
                                    if let Some(content) = event.content.get("membership") {
                                        if content.as_str() == Some("join") {
                                            let _ = event_tx
                                                .send(MatrixMonitorEvent::RoomJoin {
                                                    room_id: room_id.clone(),
                                                    user_id: event.sender.clone(),
                                                })
                                                .await;
                                        } else if content.as_str() == Some("leave") {
                                            let _ = event_tx
                                                .send(MatrixMonitorEvent::RoomLeave {
                                                    room_id: room_id.clone(),
                                                    user_id: event.sender.clone(),
                                                })
                                                .await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Small delay to prevent tight loop
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Stop the monitor.
    pub async fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }
    }
}

/// Run a simple Matrix monitor for testing or CLI use.
pub async fn run_monitor(config: MatrixConfig) -> Result<()> {
    let mut monitor = Monitor::new(config);
    let mut events = monitor.start().await?;

    println!("Matrix monitor started. Press Ctrl+C to stop.");

    while let Some(event) = events.recv().await {
        match event {
            MatrixMonitorEvent::Message(msg) => {
                println!("[Matrix] {} in {}: {}", msg.sender, msg.room_id, msg.body);
            }
            MatrixMonitorEvent::RoomJoin { room_id, user_id } => {
                println!("[Matrix] {} joined {}", user_id, room_id);
            }
            MatrixMonitorEvent::RoomLeave { room_id, user_id } => {
                println!("[Matrix] {} left {}", user_id, room_id);
            }
            MatrixMonitorEvent::Connected => {
                println!("[Matrix] Connected");
            }
            MatrixMonitorEvent::Disconnected => {
                println!("[Matrix] Disconnected");
            }
            MatrixMonitorEvent::Error(e) => {
                eprintln!("[Matrix] Error: {}", e);
            }
            MatrixMonitorEvent::Reaction { .. } => {}
        }
    }

    monitor.stop().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validate_missing_token() {
        let cfg = MatrixConfig {
            homeserver: "https://matrix.org".into(),
            access_token: "".into(),
            user_id: "@bot:matrix.org".into(),
            encryption: None,
            sync_timeout_ms: None,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validate_missing_user_id() {
        let cfg = MatrixConfig {
            homeserver: "https://matrix.org".into(),
            access_token: "token".into(),
            user_id: "".into(),
            encryption: None,
            sync_timeout_ms: None,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validate_invalid_user_id() {
        let cfg = MatrixConfig {
            homeserver: "https://matrix.org".into(),
            access_token: "token".into(),
            user_id: "bot:matrix.org".into(),
            encryption: None,
            sync_timeout_ms: None,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn parse_text_event_ok() {
        let event = MatrixEvent {
            event_id: "$ev1".into(),
            room_id: "!room:matrix.org".into(),
            sender: "@user:matrix.org".into(),
            kind: "m.room.message".into(),
            content: serde_json::json!({ "msgtype": "m.text", "body": "hello" }),
            origin_server_ts: 1_700_000_000_000,
            redacts: None,
            unsigned: None,
        };
        let parsed = parse_text_event(&event).unwrap();
        assert_eq!(parsed.body, "hello");
        assert_eq!(parsed.sender, "@user:matrix.org");
    }

    #[test]
    fn parse_non_message_event_returns_none() {
        let event = MatrixEvent {
            event_id: "$ev2".into(),
            room_id: "!r:m.org".into(),
            sender: "@u:m.org".into(),
            kind: "m.room.member".into(),
            content: serde_json::json!({ "membership": "join" }),
            origin_server_ts: 0,
            redacts: None,
            unsigned: None,
        };
        assert!(parse_text_event(&event).is_none());
    }

    #[test]
    fn build_text_event_json() {
        let ev = build_text_event("hi");
        assert_eq!(ev["msgtype"].as_str(), Some("m.text"));
        assert_eq!(ev["body"].as_str(), Some("hi"));
    }

    #[test]
    fn build_reply_event_has_relates_to() {
        let ev = build_reply_event("hello", "$original");
        assert!(ev["m.relates_to"]["m.in_reply_to"]["event_id"]
            .as_str()
            .is_some());
    }

    #[test]
    fn encode_room_id_test() {
        assert_eq!(encode_room_id("!room:matrix.org"), "%21room%3Amatrix.org");
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedMatrixMessage {
            event_id: "$ev1".into(),
            room_id: "!r:m.org".into(),
            sender: "@u:m.org".into(),
            body: "yo".into(),
            formatted_body: None,
            reply_to_event_id: None,
            timestamp: 0,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("matrix:"));
        assert_eq!(m.text, "yo");
    }
}
