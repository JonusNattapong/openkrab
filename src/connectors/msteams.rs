//! msteams — Microsoft Teams connector.
//! Ported from `openclaw/extensions/msteams/` (Phase 11).
//!
//! Uses the Bot Framework / Incoming Webhook API.

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsTeamsConfig {
    /// Incoming Webhook URL for posting messages.
    pub webhook_url: String,
    /// Bot App ID (for Bot Framework).
    pub app_id: Option<String>,
    /// Bot App Password (for Bot Framework).
    pub app_password: Option<String>,
    /// Tenant ID.
    pub tenant_id: Option<String>,
}

impl Default for MsTeamsConfig {
    fn default() -> Self {
        Self {
            webhook_url: std::env::var("MSTEAMS_WEBHOOK_URL").unwrap_or_default(),
            app_id: std::env::var("MSTEAMS_APP_ID").ok(),
            app_password: std::env::var("MSTEAMS_APP_PASSWORD").ok(),
            tenant_id: std::env::var("MSTEAMS_TENANT_ID").ok(),
        }
    }
}

impl MsTeamsConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.webhook_url.is_empty() {
            bail!("MSTEAMS_WEBHOOK_URL is required");
        }
        Ok(())
    }
}

// ─── Activity types ───────────────────────────────────────────────────────────

/// Bot Framework Activity object (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsActivity {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: Option<String>,
    pub text: Option<String>,
    pub from: Option<TeamsAccount>,
    pub conversation: Option<TeamsConversation>,
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    #[serde(rename = "serviceUrl")]
    pub service_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsAccount {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConversation {
    pub id: String,
    #[serde(rename = "isGroup")]
    pub is_group: Option<bool>,
}

/// Parsed message from a Teams Activity.
#[derive(Debug, Clone)]
pub struct ParsedTeamsMessage {
    pub activity_id: String,
    pub from_id: String,
    pub from_name: Option<String>,
    pub conversation_id: String,
    pub text: String,
    pub is_group: bool,
    pub service_url: Option<String>,
}

pub fn parse_activity(activity: &TeamsActivity) -> Option<ParsedTeamsMessage> {
    if activity.kind != "message" {
        return None;
    }
    let text = activity.text.as_deref()?.trim().to_string();
    if text.is_empty() {
        return None;
    }

    let from = activity.from.as_ref()?;
    let conv = activity.conversation.as_ref()?;

    Some(ParsedTeamsMessage {
        activity_id: activity.id.clone().unwrap_or_default(),
        from_id: from.id.clone(),
        from_name: from.name.clone(),
        conversation_id: conv.id.clone(),
        text,
        is_group: conv.is_group.unwrap_or(false),
        service_url: activity.service_url.clone(),
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build an Adaptive Card message payload for Incoming Webhook.
pub fn build_webhook_payload(text: &str) -> serde_json::Value {
    serde_json::json!({
        "@type": "MessageCard",
        "@context": "https://schema.org/extensions",
        "text": text
    })
}

/// Build a simple text reply Activity.
pub fn build_reply_activity(conversation_id: &str, text: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "message",
        "conversation": { "id": conversation_id },
        "text": text
    })
}

/// Send a message via Incoming Webhook.
pub async fn send_webhook(client: &reqwest::Client, webhook_url: &str, text: &str) -> Result<()> {
    let payload = build_webhook_payload(text);
    client
        .post(webhook_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub fn normalize_inbound(msg: &ParsedTeamsMessage) -> Message {
    Message {
        id: format!("msteams:{}", msg.activity_id),
        text: msg.text.clone(),
        from: Some(UserId(format!("msteams:{}", msg.from_id))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[msteams] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validate_missing_webhook() {
        let cfg = MsTeamsConfig {
            webhook_url: "".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn parse_message_activity_ok() {
        let a = TeamsActivity {
            kind: "message".into(),
            id: Some("act-1".into()),
            text: Some("  hello  ".into()),
            from: Some(TeamsAccount {
                id: "u1".into(),
                name: Some("Alice".into()),
            }),
            conversation: Some(TeamsConversation {
                id: "conv-1".into(),
                is_group: Some(false),
            }),
            channel_id: Some("msteams".into()),
            service_url: Some("https://smba.trafficmanager.net/".into()),
        };
        let msg = parse_activity(&a).unwrap();
        assert_eq!(msg.text, "hello");
        assert_eq!(msg.from_id, "u1");
        assert!(!msg.is_group);
    }

    #[test]
    fn parse_non_message_activity_returns_none() {
        let a = TeamsActivity {
            kind: "conversationUpdate".into(),
            id: None,
            text: None,
            from: None,
            conversation: None,
            channel_id: None,
            service_url: None,
        };
        assert!(parse_activity(&a).is_none());
    }

    #[test]
    fn build_webhook_payload_json() {
        let p = build_webhook_payload("Hello Teams!");
        assert_eq!(p["@type"].as_str(), Some("MessageCard"));
        assert_eq!(p["text"].as_str(), Some("Hello Teams!"));
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedTeamsMessage {
            activity_id: "a1".into(),
            from_id: "u1".into(),
            from_name: None,
            conversation_id: "c1".into(),
            text: "hi".into(),
            is_group: false,
            service_url: None,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("msteams:"));
    }
}
