//! matrix — Matrix channel connector.
//! Ported from `openclaw/extensions/matrix/` (Phase 5-6).

use crate::common::{Message, UserId};
use serde::{Deserialize, Serialize};

/// Matrix message event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixMessageEvent {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "origin_server_ts")]
    pub origin_server_ts: u64,
    #[serde(rename = "content")]
    pub content: MatrixMessageContent,
    #[serde(rename = "room_id")]
    pub room_id: String,
}

/// Matrix message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixMessageContent {
    #[serde(rename = "msgtype")]
    pub msgtype: String,
    pub body: String,
    #[serde(rename = "formatted_body")]
    pub formatted_body: Option<String>,
    #[serde(rename = "format")]
    pub format: Option<String>,
}

/// Matrix room message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixRoomMessage {
    pub room_id: String,
    pub event: MatrixMessageEvent,
}

/// Parse Matrix message from JSON.
pub fn parse_matrix_message(json: &serde_json::Value) -> Option<MatrixMessageEvent> {
    serde_json::from_value(json.clone()).ok()
}

/// Extract text from Matrix message.
pub fn extract_matrix_text(event: &MatrixMessageEvent) -> Option<String> {
    if event.content.msgtype == "m.text" {
        Some(event.content.body.clone())
    } else {
        None
    }
}

/// Normalize Matrix message to common format.
pub fn normalize_inbound(text: &str, sender: &str, room_id: &str) -> Message {
    Message {
        id: format!("matrix:{}:{}", room_id, uuid::Uuid::new_v4()),
        text: text.to_string(),
        from: Some(UserId(format!("matrix:{}", sender))),
    }
}

/// Format outbound Matrix message.
pub fn format_outbound(text: &str) -> String {
    format!("[matrix] {}", text)
}

/// Build Matrix text message payload.
pub fn build_text_message(text: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": text
    })
}

/// Build Matrix HTML message payload.
pub fn build_html_message(text: &str, html: &str) -> serde_json::Value {
    serde_json::json!({
        "msgtype": "m.text",
        "body": text,
        "format": "org.matrix.custom.html",
        "formatted_body": html
    })
}

/// Matrix sync response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixSyncResponse {
    #[serde(rename = "next_batch")]
    pub next_batch: String,
    pub rooms: Option<MatrixRooms>,
}

/// Matrix rooms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixRooms {
    pub join: Option<std::collections::HashMap<String, MatrixJoinedRoom>>,
}

/// Matrix joined room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixJoinedRoom {
    pub timeline: Option<MatrixTimeline>,
}

/// Matrix timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixTimeline {
    pub events: Vec<serde_json::Value>,
}

/// Parse Matrix sync response.
pub fn parse_sync_response(json: &serde_json::Value) -> Option<MatrixSyncResponse> {
    serde_json::from_value(json.clone()).ok()
}

/// Extract messages from sync response.
pub fn extract_messages_from_sync(sync: &MatrixSyncResponse) -> Vec<MatrixRoomMessage> {
    let mut messages = Vec::new();

    if let Some(ref rooms) = sync.rooms {
        if let Some(ref join) = rooms.join {
            for (room_id, room) in join {
                if let Some(ref timeline) = room.timeline {
                    for event in &timeline.events {
                        if let Some(msg_event) = parse_matrix_message(event) {
                            messages.push(MatrixRoomMessage {
                                room_id: room_id.clone(),
                                event: msg_event,
                            });
                        }
                    }
                }
            }
        }
    }

    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_matrix_text() {
        let event = MatrixMessageEvent {
            event_id: "$1234567890".to_string(),
            sender: "@user:example.com".to_string(),
            origin_server_ts: 1234567890,
            content: MatrixMessageContent {
                msgtype: "m.text".to_string(),
                body: "Hello world".to_string(),
                formatted_body: None,
                format: None,
            },
            room_id: "!room:example.com".to_string(),
        };

        assert_eq!(extract_matrix_text(&event), Some("Hello world".to_string()));
    }

    #[test]
    fn test_normalize_inbound() {
        let msg = normalize_inbound("Hello", "@user:example.com", "!room:example.com");
        assert!(msg.id.starts_with("matrix:!room:"));
        assert_eq!(msg.text, "Hello");
        assert_eq!(msg.from.as_ref().unwrap().0, "matrix:@user:example.com");
    }

    #[test]
    fn test_build_text_message() {
        let payload = build_text_message("Hello");
        assert_eq!(payload["msgtype"], "m.text");
        assert_eq!(payload["body"], "Hello");
    }

    #[test]
    fn test_build_html_message() {
        let payload = build_html_message("Hello", "<b>Hello</b>");
        assert_eq!(payload["msgtype"], "m.text");
        assert_eq!(payload["body"], "Hello");
        assert_eq!(payload["formatted_body"], "<b>Hello</b>");
    }
}
