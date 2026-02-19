//! matrix — Matrix messaging connector.
//! Ported from `openclaw/extensions/matrix/` (Phase 10).
//!
//! Provides inbound/outbound message types and HTTP client helpers
//! for the Matrix Client-Server API (https://spec.matrix.org/v1.6/).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    /// The homeserver URL, e.g. "https://matrix.org".
    pub homeserver: String,
    /// The access token for the bot user.
    pub access_token: String,
    /// The bot's full Matrix ID, e.g. "@krabbot:matrix.org".
    pub user_id: String,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            homeserver: std::env::var("MATRIX_HOMESERVER")
                .unwrap_or_else(|_| "https://matrix.org".to_string()),
            access_token: std::env::var("MATRIX_ACCESS_TOKEN").unwrap_or_default(),
            user_id: std::env::var("MATRIX_USER_ID").unwrap_or_default(),
        }
    }
}

impl MatrixConfig {
    pub fn from_env() -> Self { Self::default() }

    pub fn validate(&self) -> Result<()> {
        if self.access_token.is_empty() {
            bail!("MATRIX_ACCESS_TOKEN is required");
        }
        if self.user_id.is_empty() {
            bail!("MATRIX_USER_ID is required");
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
    pub origin_server_ts: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixTextContent {
    pub body: String,
    pub msgtype: String,
    #[serde(rename = "m.relates_to")]
    pub relates_to: Option<MatrixRelatesTo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixRelatesTo {
    #[serde(rename = "m.in_reply_to")]
    pub in_reply_to: Option<MatrixInReplyTo>,
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
    pub reply_to_event_id: Option<String>,
    pub timestamp: i64,
}

pub fn parse_text_event(event: &MatrixEvent) -> Option<ParsedMatrixMessage> {
    if event.kind != "m.room.message" { return None; }
    let content: MatrixTextContent = serde_json::from_value(event.content.clone()).ok()?;
    if content.msgtype != "m.text" { return None; }

    let reply_to = content.relates_to
        .as_ref()
        .and_then(|r| r.in_reply_to.as_ref())
        .map(|r| r.event_id.clone());

    Some(ParsedMatrixMessage {
        event_id: event.event_id.clone(),
        room_id: event.room_id.clone(),
        sender: event.sender.clone(),
        body: content.body,
        reply_to_event_id: reply_to,
        timestamp: event.origin_server_ts,
    })
}

// ─── Sync response ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixSyncResponse {
    pub next_batch: String,
    pub rooms: Option<MatrixSyncRooms>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixSyncRooms {
    pub join: Option<std::collections::HashMap<String, MatrixJoinedRoom>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixJoinedRoom {
    pub timeline: Option<MatrixTimeline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixTimeline {
    pub events: Vec<MatrixEvent>,
    pub limited: Option<bool>,
    pub prev_batch: Option<String>,
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

// ─── Outbound helpers ─────────────────────────────────────────────────────────

pub fn build_text_event(body: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": body
    })
}

pub fn build_reply_event(body: &str, reply_to_event_id: &str, room_id: &str) -> serde_json::Value {
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
    let txn_id = format!("txn_{}", now_ms);
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
        cfg.homeserver, urlencoding(room_id), txn_id
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

fn urlencoding(s: &str) -> String {
    s.replace('!', "%21").replace(':', "%3A").replace('#', "%23")
}

pub fn normalize_inbound(msg: &ParsedMatrixMessage) -> crate::common::Message {
    crate::common::Message {
        id: format!("matrix:{}", msg.event_id),
        text: msg.body.clone(),
        from: Some(crate::common::UserId(format!("matrix:{}", msg.sender))),
    }
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
        let ev = build_reply_event("hello", "$original", "!room:m.org");
        assert!(ev["m.relates_to"]["m.in_reply_to"]["event_id"].as_str().is_some());
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedMatrixMessage {
            event_id: "$ev1".into(),
            room_id: "!r:m.org".into(),
            sender: "@u:m.org".into(),
            body: "yo".into(),
            reply_to_event_id: None,
            timestamp: 0,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("matrix:"));
        assert_eq!(m.text, "yo");
    }
}
