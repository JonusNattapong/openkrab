//! feishu — Feishu/Lark connector.
//! Ported from `openclaw/extensions/feishu/` (Phase 12).
//!
//! Uses the Feishu Open Platform API.
//! Reference: https://open.feishu.cn/document/home/index

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuConfig {
    /// App ID (AppKey).
    pub app_id: String,
    /// App Secret.
    pub app_secret: String,
    /// Webhook Verification Token (for webhook security).
    pub verification_token: Option<String>,
    /// Encrypt key for payload decryption.
    pub encrypt_key: Option<String>,
}

impl Default for FeishuConfig {
    fn default() -> Self {
        Self {
            app_id: std::env::var("FEISHU_APP_ID").unwrap_or_default(),
            app_secret: std::env::var("FEISHU_APP_SECRET").unwrap_or_default(),
            verification_token: std::env::var("FEISHU_VERIFICATION_TOKEN").ok(),
            encrypt_key: std::env::var("FEISHU_ENCRYPT_KEY").ok(),
        }
    }
}

impl FeishuConfig {
    pub fn from_env() -> Self {
        Self::default()
    }
    pub fn validate(&self) -> Result<()> {
        if self.app_id.is_empty() {
            bail!("FEISHU_APP_ID is required");
        }
        if self.app_secret.is_empty() {
            bail!("FEISHU_APP_SECRET is required");
        }
        Ok(())
    }
}

// ─── Event types ──────────────────────────────────────────────────────────────

/// Feishu event callback (v2.0 format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuEventCallback {
    pub schema: Option<String>,
    pub header: Option<FeishuEventHeader>,
    pub event: Option<FeishuMessageEvent>,
    /// For URL verification challenge.
    pub challenge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuEventHeader {
    pub event_id: Option<String>,
    pub event_type: Option<String>,
    pub create_time: Option<String>,
    pub token: Option<String>,
    pub app_id: Option<String>,
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuMessageEvent {
    pub message: Option<FeishuMessage>,
    pub sender: Option<FeishuSender>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuMessage {
    pub message_id: Option<String>,
    pub parent_id: Option<String>,
    pub thread_id: Option<String>,
    pub create_time: Option<String>,
    pub chat_id: Option<String>,
    pub chat_type: Option<String>,    // "p2p" | "group"
    pub message_type: Option<String>, // "text" | "image" | etc.
    pub content: Option<String>,      // JSON string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuSender {
    pub sender_id: Option<FeishuSenderId>,
    pub sender_type: Option<String>,
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuSenderId {
    pub open_id: Option<String>,
    pub user_id: Option<String>,
    pub union_id: Option<String>,
}

/// Parsed Feishu text message.
#[derive(Debug, Clone)]
pub struct ParsedFeishuMessage {
    pub message_id: String,
    pub chat_id: String,
    pub sender_open_id: String,
    pub text: String,
    pub is_p2p: bool,
    pub thread_id: Option<String>,
}

pub fn parse_event(cb: &FeishuEventCallback) -> Option<ParsedFeishuMessage> {
    // URL verification — not a real message
    if cb.challenge.is_some() {
        return None;
    }

    let header = cb.header.as_ref()?;
    if header.event_type.as_deref() != Some("im.message.receive_v1") {
        return None;
    }

    let ev = cb.event.as_ref()?;
    let msg = ev.message.as_ref()?;
    if msg.message_type.as_deref() != Some("text") {
        return None;
    }

    // content is a JSON string: {"text": "hello"}
    let content_str = msg.content.as_deref().unwrap_or("{}");
    let content: serde_json::Value = serde_json::from_str(content_str).unwrap_or_default();
    let text = content["text"].as_str().unwrap_or("").trim().to_string();
    if text.is_empty() {
        return None;
    }

    let sender = ev.sender.as_ref()?;
    let sender_open_id = sender
        .sender_id
        .as_ref()
        .and_then(|id| id.open_id.as_deref())
        .unwrap_or("")
        .to_string();

    Some(ParsedFeishuMessage {
        message_id: msg.message_id.clone().unwrap_or_default(),
        chat_id: msg.chat_id.clone().unwrap_or_default(),
        sender_open_id,
        text,
        is_p2p: msg.chat_type.as_deref() == Some("p2p"),
        thread_id: msg.thread_id.clone(),
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

pub const FEISHU_API_BASE: &str = "https://open.feishu.cn/open-apis";

/// Build a text message payload.
pub fn build_text_payload(
    receive_id: &str,
    text: &str,
    receive_id_type: &str,
) -> serde_json::Value {
    serde_json::json!({
        "receive_id": receive_id,
        "msg_type": "text",
        "content": serde_json::json!({"text": text}).to_string()
    })
}

/// Send a text message via Feishu API.
pub async fn send_message(
    client: &reqwest::Client,
    access_token: &str,
    receive_id: &str,
    text: &str,
    receive_id_type: &str,
) -> Result<serde_json::Value> {
    let url = format!(
        "{}/im/v1/messages?receive_id_type={}",
        FEISHU_API_BASE, receive_id_type
    );
    let payload = build_text_payload(receive_id, text, receive_id_type);
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json().await?)
}

/// Get a tenant access token.
pub async fn get_tenant_access_token(
    client: &reqwest::Client,
    cfg: &FeishuConfig,
) -> Result<String> {
    let url = format!("{}/auth/v3/tenant_access_token/internal", FEISHU_API_BASE);
    let payload = serde_json::json!({
        "app_id": cfg.app_id,
        "app_secret": cfg.app_secret
    });
    let resp: serde_json::Value = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    resp["tenant_access_token"]
        .as_str()
        .map(|t| t.to_string())
        .ok_or_else(|| anyhow::anyhow!("missing tenant_access_token in response"))
}

pub fn normalize_inbound(msg: &ParsedFeishuMessage) -> Message {
    Message {
        id: format!("feishu:{}", msg.message_id),
        text: msg.text.clone(),
        from: Some(UserId(format!("feishu:{}", msg.sender_open_id))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[feishu] {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_callback(text: &str) -> FeishuEventCallback {
        FeishuEventCallback {
            schema: Some("2.0".into()),
            challenge: None,
            header: Some(FeishuEventHeader {
                event_id: Some("ev1".into()),
                event_type: Some("im.message.receive_v1".into()),
                create_time: None,
                token: None,
                app_id: Some("app1".into()),
                tenant_key: None,
            }),
            event: Some(FeishuMessageEvent {
                sender: Some(FeishuSender {
                    sender_id: Some(FeishuSenderId {
                        open_id: Some("ou_abc".into()),
                        user_id: None,
                        union_id: None,
                    }),
                    sender_type: Some("user".into()),
                    tenant_key: None,
                }),
                message: Some(FeishuMessage {
                    message_id: Some("om_msg1".into()),
                    parent_id: None,
                    thread_id: None,
                    create_time: None,
                    chat_id: Some("oc_chat1".into()),
                    chat_type: Some("p2p".into()),
                    message_type: Some("text".into()),
                    content: Some(format!("{{\"text\":\"{}\"}}", text)),
                }),
            }),
        }
    }

    #[test]
    fn parse_text_message() {
        let cb = make_callback("สวัสดี");
        let msg = parse_event(&cb).unwrap();
        assert_eq!(msg.text, "สวัสดี");
        assert_eq!(msg.sender_open_id, "ou_abc");
        assert!(msg.is_p2p);
    }

    #[test]
    fn parse_challenge_returns_none() {
        let cb = FeishuEventCallback {
            schema: None,
            header: None,
            event: None,
            challenge: Some("abc123".into()),
        };
        assert!(parse_event(&cb).is_none());
    }

    #[test]
    fn config_validate_missing_app_id() {
        let cfg = FeishuConfig {
            app_id: "".into(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn build_text_payload_json() {
        let p = build_text_payload("oc_abc", "hello", "chat_id");
        assert_eq!(p["msg_type"].as_str(), Some("text"));
        assert_eq!(p["receive_id"].as_str(), Some("oc_abc"));
    }

    #[test]
    fn normalize_inbound_test() {
        let msg = ParsedFeishuMessage {
            message_id: "m1".into(),
            chat_id: "c1".into(),
            sender_open_id: "ou_1".into(),
            text: "hi".into(),
            is_p2p: true,
            thread_id: None,
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("feishu:"));
    }
}
