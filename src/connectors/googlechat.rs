//! googlechat — Google Chat connector.
//! Ported from `openkrab/extensions/googlechat/` (Phase 12).
//!
//! Uses Google Chat Events API (Pub/Sub or HTTP webhook).
//! Reference: https://developers.google.com/chat/api/reference/rest

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatConfig {
    /// Service account JSON key (path or JSON string).
    pub service_account_key: String,
    /// Webhook URL (for simple webhook integration).
    pub webhook_url: Option<String>,
    /// Google Cloud Project ID.
    pub project_id: Option<String>,
    /// Pub/Sub subscription name.
    pub subscription: Option<String>,
}

impl Default for GoogleChatConfig {
    fn default() -> Self {
        Self {
            service_account_key: std::env::var("GOOGLECHAT_SERVICE_ACCOUNT_KEY")
                .unwrap_or_default(),
            webhook_url: std::env::var("GOOGLECHAT_WEBHOOK_URL").ok(),
            project_id: std::env::var("GOOGLECHAT_PROJECT_ID").ok(),
            subscription: std::env::var("GOOGLECHAT_SUBSCRIPTION").ok(),
        }
    }
}

impl GoogleChatConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.service_account_key.is_empty() && self.webhook_url.is_none() {
            bail!("GOOGLECHAT_SERVICE_ACCOUNT_KEY or GOOGLECHAT_WEBHOOK_URL is required");
        }
        Ok(())
    }
}

// ─── Event types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatEvent {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,
    pub message: Option<GoogleChatMessage>,
    pub user: Option<GoogleChatUser>,
    pub space: Option<GoogleChatSpace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatMessage {
    pub name: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "argumentText")]
    pub argument_text: Option<String>,
    pub sender: Option<GoogleChatUser>,
    pub space: Option<GoogleChatSpace>,
    pub thread: Option<GoogleChatThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatUser {
    pub name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatSpace {
    pub name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleChatThread {
    pub name: Option<String>,
}

/// Parsed message from a Google Chat event.
#[derive(Debug, Clone)]
pub struct ParsedGoogleChatMessage {
    pub message_name: String,
    pub sender_name: String,
    pub sender_display_name: String,
    pub space_name: String,
    pub thread_name: Option<String>,
    pub text: String,
    pub is_bot_mentioned: bool,
}

pub fn parse_event(event: &GoogleChatEvent) -> Option<ParsedGoogleChatMessage> {
    if event.kind != "MESSAGE" {
        return None;
    }
    let msg = event.message.as_ref()?;
    let raw_text = msg.text.as_deref().unwrap_or("").trim().to_string();
    let text = msg
        .argument_text
        .as_deref()
        .map(|t| t.trim().to_string())
        .unwrap_or_else(|| raw_text.clone());
    if raw_text.is_empty() {
        return None;
    }

    let sender = msg.sender.as_ref().or(event.user.as_ref());
    let sender_name = sender
        .as_ref()
        .and_then(|u| u.name.as_deref())
        .unwrap_or("")
        .to_string();
    let sender_display = sender
        .as_ref()
        .and_then(|u| u.display_name.as_deref())
        .unwrap_or("")
        .to_string();
    let space_name = msg
        .space
        .as_ref()
        .or(event.space.as_ref())
        .and_then(|s| s.name.as_deref())
        .unwrap_or("")
        .to_string();

    // Detect bot mention (<bot_name> or @bot)
    let is_bot_mentioned = raw_text != text;

    Some(ParsedGoogleChatMessage {
        message_name: msg.name.clone().unwrap_or_default(),
        sender_name,
        sender_display_name: sender_display,
        space_name,
        thread_name: msg.thread.as_ref().and_then(|t| t.name.clone()),
        text,
        is_bot_mentioned,
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build a simple text message payload.
pub fn build_text_message(text: &str) -> serde_json::Value {
    serde_json::json!({ "text": text })
}

/// Build a text message in a specific thread.
pub fn build_thread_message(text: &str, thread_name: &str) -> serde_json::Value {
    serde_json::json!({
        "text": text,
        "thread": { "name": thread_name }
    })
}

/// Send a message via Incoming Webhook.
pub async fn send_webhook(
    client: &reqwest::Client,
    webhook_url: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let payload = build_text_message(text);
    let resp = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

pub fn normalize_inbound(msg: &ParsedGoogleChatMessage) -> Message {
    Message {
        id: format!("googlechat:{}", msg.message_name),
        text: msg.text.clone(),
        from: Some(UserId(format!("googlechat:{}", msg.sender_name))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[googlechat] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(text: &str) -> GoogleChatEvent {
        GoogleChatEvent {
            kind: "MESSAGE".into(),
            event_time: None,
            user: None,
            space: None,
            message: Some(GoogleChatMessage {
                name: Some("spaces/sp1/messages/msg1".into()),
                text: Some(text.into()),
                argument_text: Some(text.into()),
                sender: Some(GoogleChatUser {
                    name: Some("users/u1".into()),
                    display_name: Some("Alice".into()),
                    kind: Some("HUMAN".into()),
                }),
                space: Some(GoogleChatSpace {
                    name: Some("spaces/sp1".into()),
                    display_name: None,
                    kind: Some("ROOM".into()),
                }),
                thread: Some(GoogleChatThread {
                    name: Some("spaces/sp1/threads/t1".into()),
                }),
            }),
        }
    }

    #[test]
    fn parse_message_event() {
        let ev = make_event("hello");
        let msg = parse_event(&ev).unwrap();
        assert_eq!(msg.text, "hello");
        assert_eq!(msg.sender_display_name, "Alice");
    }

    #[test]
    fn parse_non_message_event_returns_none() {
        let ev = GoogleChatEvent {
            kind: "ADDED_TO_SPACE".into(),
            event_time: None,
            message: None,
            user: None,
            space: None,
        };
        assert!(parse_event(&ev).is_none());
    }

    #[test]
    fn build_text_message_json() {
        let p = build_text_message("hi");
        assert_eq!(p["text"].as_str(), Some("hi"));
    }

    #[test]
    fn build_thread_message_json() {
        let p = build_thread_message("reply", "spaces/sp/threads/t1");
        assert_eq!(p["thread"]["name"].as_str(), Some("spaces/sp/threads/t1"));
    }

    #[test]
    fn config_validate_missing_all() {
        let cfg = GoogleChatConfig {
            service_account_key: "".into(),
            webhook_url: None,
            project_id: None,
            subscription: None,
        };
        assert!(cfg.validate().is_err());
    }
}
