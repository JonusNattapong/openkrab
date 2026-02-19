//! nextcloud_talk — Nextcloud Talk connector.
//! Ported from `openclaw/extensions/nextcloud-talk/` (Phase 12).
//!
//! Uses the Nextcloud Talk API (spreed).
//! Reference: https://nextcloud-talk.readthedocs.io/en/latest/

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextcloudTalkConfig {
    /// Nextcloud server URL (e.g. "https://cloud.example.com").
    pub server_url: String,
    /// Bot username.
    pub username: String,
    /// Bot password or app token.
    pub password: String,
    /// Poll interval in seconds.
    pub poll_interval_secs: u64,
}

impl Default for NextcloudTalkConfig {
    fn default() -> Self {
        Self {
            server_url: std::env::var("NEXTCLOUD_URL").unwrap_or_default(),
            username: std::env::var("NEXTCLOUD_USER").unwrap_or_default(),
            password: std::env::var("NEXTCLOUD_PASSWORD").unwrap_or_default(),
            poll_interval_secs: std::env::var("NEXTCLOUD_POLL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
        }
    }
}

impl NextcloudTalkConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.server_url.is_empty() {
            bail!("NEXTCLOUD_URL is required");
        }
        if self.username.is_empty() {
            bail!("NEXTCLOUD_USER is required");
        }
        if self.password.is_empty() {
            bail!("NEXTCLOUD_PASSWORD is required");
        }
        Ok(())
    }

    pub fn api_base(&self) -> String {
        format!("{}/ocs/v2.php/apps/spreed/api/v1", self.server_url)
    }
}

// ─── Message types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTalkMessage {
    pub id: i64,
    pub token: String,
    pub actor_type: String,
    #[serde(rename = "actorId")]
    pub actor_id: String,
    #[serde(rename = "actorDisplayName")]
    pub actor_display_name: String,
    pub message: String,
    pub timestamp: i64,
    #[serde(rename = "messageType")]
    pub message_type: String, // "comment" | "system" | "command"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTalkMessagesResponse {
    pub ocs: NcTalkOcs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTalkOcs {
    pub data: Vec<NcTalkMessage>,
}

/// Parsed message from Nextcloud Talk.
#[derive(Debug, Clone)]
pub struct ParsedNcTalkMessage {
    pub id: i64,
    pub room_token: String,
    pub actor_id: String,
    pub actor_display_name: String,
    pub text: String,
    pub timestamp: i64,
}

pub fn parse_message(msg: &NcTalkMessage) -> Option<ParsedNcTalkMessage> {
    if msg.message_type != "comment" {
        return None;
    }
    let text = msg.message.trim().to_string();
    if text.is_empty() {
        return None;
    }

    Some(ParsedNcTalkMessage {
        id: msg.id,
        room_token: msg.token.clone(),
        actor_id: msg.actor_id.clone(),
        actor_display_name: msg.actor_display_name.clone(),
        text,
        timestamp: msg.timestamp,
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build a chat message payload.
pub fn build_message_payload(message: &str) -> Vec<(&str, &str)> {
    vec![("message", message)]
}

/// Send a message to a room via REST API.
pub async fn send_message(
    client: &reqwest::Client,
    cfg: &NextcloudTalkConfig,
    room_token: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/chat/{}", cfg.api_base(), room_token);
    let resp = client
        .post(&url)
        .basic_auth(&cfg.username, Some(&cfg.password))
        .header("OCS-APIRequest", "true")
        .header("Accept", "application/json")
        .form(&[("message", text)])
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

/// Fetch messages from a room (long polling).
pub async fn get_messages(
    client: &reqwest::Client,
    cfg: &NextcloudTalkConfig,
    room_token: &str,
    last_known_id: Option<i64>,
) -> Result<Vec<NcTalkMessage>> {
    let mut url = format!("{}/chat/{}", cfg.api_base(), room_token);
    if let Some(id) = last_known_id {
        url += &format!("?lastKnownMessageId={}&lookIntoFuture=1&limit=50", id);
    }
    let resp: NcTalkMessagesResponse = client
        .get(&url)
        .basic_auth(&cfg.username, Some(&cfg.password))
        .header("OCS-APIRequest", "true")
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(resp.ocs.data)
}

pub fn normalize_inbound(msg: &ParsedNcTalkMessage) -> Message {
    Message {
        id: format!("nextcloud:{}", msg.id),
        text: msg.text.clone(),
        from: Some(UserId(format!("nextcloud:{}", msg.actor_id))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[nextcloud] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(text: &str, msg_type: &str) -> NcTalkMessage {
        NcTalkMessage {
            id: 42,
            token: "room1".into(),
            actor_type: "users".into(),
            actor_id: "alice".into(),
            actor_display_name: "Alice".into(),
            message: text.into(),
            timestamp: 1_700_000,
            message_type: msg_type.into(),
        }
    }

    #[test]
    fn parse_comment_message() {
        let msg = parse_message(&make_msg("hello", "comment")).unwrap();
        assert_eq!(msg.text, "hello");
        assert_eq!(msg.actor_id, "alice");
        assert_eq!(msg.room_token, "room1");
    }

    #[test]
    fn parse_system_message_returns_none() {
        assert!(parse_message(&make_msg("user joined", "system")).is_none());
    }

    #[test]
    fn parse_empty_text_returns_none() {
        assert!(parse_message(&make_msg("   ", "comment")).is_none());
    }

    #[test]
    fn config_validate_missing_url() {
        let cfg = NextcloudTalkConfig {
            server_url: "".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn api_base_format() {
        let cfg = NextcloudTalkConfig {
            server_url: "https://cloud.example.com".into(),
            ..Default::default()
        };
        assert!(cfg.api_base().contains("/apps/spreed/api"));
    }
}
