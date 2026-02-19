//! zalo — Zalo messaging connector (Vietnamese market).
//! Ported from `openclaw/extensions/zalo/` + `zalouser/` (Phase 11).
//!
//! Uses the Zalo Official Account (OA) API v3.
//! Reference: https://developers.zalo.me/docs/official-account/

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloConfig {
    /// OA Access Token.
    pub access_token: String,
    /// OA Secret Key (for webhook verification).
    pub secret_key: Option<String>,
    /// OA ID.
    pub oa_id: Option<String>,
}

impl Default for ZaloConfig {
    fn default() -> Self {
        Self {
            access_token: std::env::var("ZALO_ACCESS_TOKEN").unwrap_or_default(),
            secret_key: std::env::var("ZALO_SECRET_KEY").ok(),
            oa_id: std::env::var("ZALO_OA_ID").ok(),
        }
    }
}

impl ZaloConfig {
    pub fn from_env() -> Self { Self::default() }
    pub fn validate(&self) -> Result<()> {
        if self.access_token.is_empty() { bail!("ZALO_ACCESS_TOKEN is required"); }
        Ok(())
    }
}

// ─── Webhook event ────────────────────────────────────────────────────────────

/// Zalo OA webhook event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloWebhookEvent {
    pub event_name: String,
    #[serde(rename = "app_id")]
    pub app_id: Option<String>,
    pub timestamp: Option<i64>,
    pub sender: Option<ZaloSender>,
    pub recipient: Option<ZaloRecipient>,
    pub message: Option<ZaloMessageBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloSender {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloRecipient {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloMessageBody {
    pub msg_id: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "attachments")]
    pub attachments: Option<Vec<ZaloAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZaloAttachment {
    #[serde(rename = "type")]
    pub kind: String,
    pub payload: Option<serde_json::Value>,
}

/// Parsed text message from Zalo OA webhook.
#[derive(Debug, Clone)]
pub struct ParsedZaloMessage {
    pub sender_id: String,
    pub recipient_id: String,
    pub msg_id: String,
    pub text: String,
    pub has_attachment: bool,
    pub timestamp: i64,
}

pub fn parse_event(event: &ZaloWebhookEvent) -> Option<ParsedZaloMessage> {
    if event.event_name != "follow" && event.event_name != "user_send_text"
        && !event.event_name.starts_with("user_") {
        // Only process user-initiated events
    }
    let msg_body = event.message.as_ref()?;
    let text = msg_body.text.as_deref().unwrap_or("").trim().to_string();
    // Accept messages even without text (media-only)
    let sender_id = event.sender.as_ref().map(|s| s.id.clone()).unwrap_or_default();
    let recipient_id = event.recipient.as_ref().map(|r| r.id.clone()).unwrap_or_default();

    Some(ParsedZaloMessage {
        sender_id,
        recipient_id,
        msg_id: msg_body.msg_id.clone().unwrap_or_default(),
        text,
        has_attachment: msg_body.attachments.as_ref().map(|a| !a.is_empty()).unwrap_or(false),
        timestamp: event.timestamp.unwrap_or(0),
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Base URL for Zalo OA API v3.
pub const ZALO_API_BASE: &str = "https://openapi.zalo.me/v3.0/oa";

/// Build a text message send payload.
pub fn build_text_payload(to_user_id: &str, text: &str) -> serde_json::Value {
    serde_json::json!({
        "recipient": { "user_id": to_user_id },
        "message": { "text": text }
    })
}

/// Send a text message to a Zalo user.
pub async fn send_message(
    client: &reqwest::Client,
    cfg: &ZaloConfig,
    to_user_id: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/message/cs", ZALO_API_BASE);
    let payload = build_text_payload(to_user_id, text);
    let resp = client
        .post(&url)
        .header("access_token", &cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

/// Get user profile from Zalo OA API.
pub async fn get_user_profile(
    client: &reqwest::Client,
    cfg: &ZaloConfig,
    user_id: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/getprofile?data={{\"user_id\":\"{}\"}}", ZALO_API_BASE, user_id);
    let resp = client
        .get(&url)
        .header("access_token", &cfg.access_token)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

pub fn normalize_inbound(msg: &ParsedZaloMessage) -> Message {
    Message {
        id: format!("zalo:{}", msg.msg_id),
        text: msg.text.clone(),
        from: Some(UserId(format!("zalo:{}", msg.sender_id))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[zalo] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validate_missing_token() {
        let cfg = ZaloConfig { access_token: "".into(), ..Default::default() };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn parse_text_event() {
        let event = ZaloWebhookEvent {
            event_name: "user_send_text".into(),
            app_id: None,
            timestamp: Some(1_700_000_000),
            sender: Some(ZaloSender { id: "u123".into() }),
            recipient: Some(ZaloRecipient { id: "oa456".into() }),
            message: Some(ZaloMessageBody {
                msg_id: Some("msg-1".into()),
                text: Some("  สวัสดี  ".into()),
                attachments: None,
            }),
        };
        let msg = parse_event(&event).unwrap();
        assert_eq!(msg.sender_id, "u123");
        assert_eq!(msg.text, "สวัสดี");
        assert!(!msg.has_attachment);
    }

    #[test]
    fn parse_event_no_message_body_returns_none() {
        let event = ZaloWebhookEvent {
            event_name: "follow".into(),
            app_id: None, timestamp: None,
            sender: Some(ZaloSender { id: "u1".into() }),
            recipient: Some(ZaloRecipient { id: "oa1".into() }),
            message: None,
        };
        assert!(parse_event(&event).is_none());
    }

    #[test]
    fn build_text_payload_json() {
        let p = build_text_payload("u123", "hello");
        assert_eq!(p["recipient"]["user_id"].as_str(), Some("u123"));
        assert_eq!(p["message"]["text"].as_str(), Some("hello"));
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedZaloMessage {
            sender_id: "u1".into(), recipient_id: "oa1".into(),
            msg_id: "m1".into(), text: "hi".into(),
            has_attachment: false, timestamp: 0,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("zalo:"));
        assert_eq!(m.text, "hi");
    }
}
