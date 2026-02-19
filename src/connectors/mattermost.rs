//! mattermost — Mattermost connector.
//! Ported from `openclaw/extensions/mattermost/` (Phase 11).
//!
//! Uses the Mattermost REST API v4 and Incoming Webhooks.

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MattermostConfig {
    /// Mattermost server URL, e.g. "https://mattermost.example.com".
    pub server_url: String,
    /// Bot user personal access token.
    pub access_token: String,
    /// Default channel ID or name to post in.
    pub default_channel: Option<String>,
    /// Incoming Webhook URL (alternative to token-based API).
    pub webhook_url: Option<String>,
    /// Team name.
    pub team_name: Option<String>,
}

impl Default for MattermostConfig {
    fn default() -> Self {
        Self {
            server_url: std::env::var("MATTERMOST_URL").unwrap_or_default(),
            access_token: std::env::var("MATTERMOST_TOKEN").unwrap_or_default(),
            default_channel: std::env::var("MATTERMOST_CHANNEL").ok(),
            webhook_url: std::env::var("MATTERMOST_WEBHOOK_URL").ok(),
            team_name: std::env::var("MATTERMOST_TEAM").ok(),
        }
    }
}

impl MattermostConfig {
    pub fn from_env() -> Self { Self::default() }
    pub fn validate(&self) -> Result<()> {
        if self.server_url.is_empty() { bail!("MATTERMOST_URL is required"); }
        if self.access_token.is_empty() && self.webhook_url.is_none() {
            bail!("MATTERMOST_TOKEN or MATTERMOST_WEBHOOK_URL is required");
        }
        Ok(())
    }
}

// ─── Webhook event ────────────────────────────────────────────────────────────

/// Mattermost outgoing webhook / slash command payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MattermostWebhookPayload {
    pub token: Option<String>,
    pub team_id: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub post_id: Option<String>,
    pub text: Option<String>,
    pub trigger_word: Option<String>,
}

/// Parsed message from Mattermost webhook.
#[derive(Debug, Clone)]
pub struct ParsedMattermostMessage {
    pub user_id: String,
    pub user_name: String,
    pub channel_id: String,
    pub channel_name: String,
    pub post_id: String,
    pub text: String,
}

pub fn parse_webhook(payload: &MattermostWebhookPayload) -> Option<ParsedMattermostMessage> {
    let text = payload.text.as_deref()?.trim().to_string();
    if text.is_empty() { return None; }

    Some(ParsedMattermostMessage {
        user_id: payload.user_id.clone().unwrap_or_default(),
        user_name: payload.user_name.clone().unwrap_or_default(),
        channel_id: payload.channel_id.clone().unwrap_or_default(),
        channel_name: payload.channel_name.clone().unwrap_or_default(),
        post_id: payload.post_id.clone().unwrap_or_default(),
        text,
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build an incoming webhook payload.
pub fn build_webhook_payload(text: &str, channel: Option<&str>) -> serde_json::Value {
    let mut v = serde_json::json!({ "text": text });
    if let Some(ch) = channel {
        v["channel"] = serde_json::json!(ch);
    }
    v
}

/// Build a REST API post payload.
pub fn build_post_payload(channel_id: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "channel_id": channel_id,
        "message": message
    })
}

/// Send a message via incoming webhook.
pub async fn send_webhook(
    client: &reqwest::Client,
    webhook_url: &str,
    text: &str,
    channel: Option<&str>,
) -> Result<()> {
    let payload = build_webhook_payload(text, channel);
    client.post(webhook_url).json(&payload).send().await?.error_for_status()?;
    Ok(())
}

/// Send a message via REST API.
pub async fn send_post(
    client: &reqwest::Client,
    cfg: &MattermostConfig,
    channel_id: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/api/v4/posts", cfg.server_url);
    let payload = build_post_payload(channel_id, text);
    let resp = client
        .post(&url)
        .bearer_auth(&cfg.access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

pub fn normalize_inbound(msg: &ParsedMattermostMessage) -> Message {
    Message {
        id: format!("mattermost:{}", msg.post_id),
        text: msg.text.clone(),
        from: Some(UserId(format!("mattermost:{}", msg.user_id))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[mattermost] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validate_missing_url() {
        let cfg = MattermostConfig { server_url: "".into(), ..Default::default() };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validate_missing_token_and_webhook() {
        let cfg = MattermostConfig {
            server_url: "https://mm.example.com".into(),
            access_token: "".into(),
            webhook_url: None,
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn parse_webhook_ok() {
        let p = MattermostWebhookPayload {
            token: None, team_id: None,
            channel_id: Some("ch1".into()), channel_name: Some("general".into()),
            user_id: Some("u1".into()), user_name: Some("alice".into()),
            post_id: Some("post1".into()), text: Some("  hello  ".into()),
            trigger_word: None,
        };
        let msg = parse_webhook(&p).unwrap();
        assert_eq!(msg.text, "hello");
        assert_eq!(msg.user_name, "alice");
    }

    #[test]
    fn build_webhook_payload_with_channel() {
        let v = build_webhook_payload("hi", Some("#general"));
        assert_eq!(v["text"].as_str(), Some("hi"));
        assert_eq!(v["channel"].as_str(), Some("#general"));
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedMattermostMessage {
            user_id: "u1".into(), user_name: "alice".into(),
            channel_id: "ch1".into(), channel_name: "general".into(),
            post_id: "p1".into(), text: "hi".into(),
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("mattermost:"));
    }
}
