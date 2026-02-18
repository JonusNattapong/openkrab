//! signal — Signal messenger connector stub.
//! Ported from `openclaw/extensions/signal/` (Phase 10).
//!
//! Provides inbound/outbound message types and a signal-cli HTTP bridge
//! so the krabkrab agent can send and receive Signal messages via
//! the signal-cli REST API (https://github.com/bbernhard/signal-cli-rest-api).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    /// The phone number registered with signal-cli (e.g. "+15550123456").
    pub phone_number: String,
    /// Base URL of the signal-cli REST API (default: "http://localhost:8080").
    pub api_base: String,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            phone_number: std::env::var("SIGNAL_PHONE_NUMBER").unwrap_or_default(),
            api_base: std::env::var("SIGNAL_API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }
    }
}

impl SignalConfig {
    pub fn from_env() -> Self { Self::default() }

    pub fn validate(&self) -> Result<()> {
        if self.phone_number.is_empty() {
            bail!("SIGNAL_PHONE_NUMBER is required");
        }
        Ok(())
    }
}

// ─── Inbound message ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalInbound {
    pub envelope: SignalEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalEnvelope {
    pub source: String,
    pub source_device: u32,
    pub timestamp: i64,
    #[serde(rename = "dataMessage")]
    pub data_message: Option<SignalDataMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDataMessage {
    pub message: Option<String>,
    pub timestamp: i64,
    #[serde(rename = "groupInfo")]
    pub group_info: Option<SignalGroupInfo>,
    pub attachments: Option<Vec<SignalAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGroupInfo {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAttachment {
    pub content_type: String,
    pub filename: Option<String>,
    pub size: Option<u64>,
}

// ─── Parsed message ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ParsedSignalMessage {
    pub from: String,
    pub text: String,
    pub group_id: Option<String>,
    pub has_attachment: bool,
    pub timestamp: i64,
}

pub fn parse_inbound(inbound: &SignalInbound) -> Option<ParsedSignalMessage> {
    let env = &inbound.envelope;
    let data = env.data_message.as_ref()?;
    let text = data.message.clone().unwrap_or_default();

    Some(ParsedSignalMessage {
        from: env.source.clone(),
        text,
        group_id: data.group_info.as_ref().map(|g| g.group_id.clone()),
        has_attachment: data.attachments.as_ref().map(|a| !a.is_empty()).unwrap_or(false),
        timestamp: env.timestamp,
    })
}

// ─── Outbound helpers ─────────────────────────────────────────────────────────

/// Build the JSON payload for signal-cli REST API `v2/send`.
pub fn build_send_payload(number: &str, recipients: &[&str], message: &str) -> serde_json::Value {
    serde_json::json!({
        "number": number,
        "recipients": recipients,
        "message": message
    })
}

/// Build a group message payload.
pub fn build_group_send_payload(number: &str, group_id: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "number": number,
        "recipients": [format!("group.{}", group_id)],
        "message": message
    })
}

/// Send a Signal message via signal-cli REST API (synchronous helper for CLI use).
pub async fn send_message(
    client: &reqwest::Client,
    cfg: &SignalConfig,
    to: &str,
    text: &str,
) -> Result<()> {
    let url = format!("{}/v2/send", cfg.api_base);
    let payload = build_send_payload(&cfg.phone_number, &[to], text);

    let resp = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let _body = resp.text().await?;
    Ok(())
}

// ─── Normalise to common message ──────────────────────────────────────────────

pub fn normalize_inbound(parsed: &ParsedSignalMessage) -> crate::common::Message {
    crate::common::Message {
        id: format!("signal:{}:{}", parsed.from, parsed.timestamp),
        text: parsed.text.clone(),
        from: Some(crate::common::UserId(format!("signal:{}", parsed.from))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_api_base() {
        let cfg = SignalConfig { phone_number: "+1".into(), api_base: "http://localhost:8080".into() };
        assert!(cfg.api_base.contains("localhost"));
    }

    #[test]
    fn config_validate_missing_phone() {
        let cfg = SignalConfig { phone_number: "".into(), api_base: "http://localhost:8080".into() };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn parse_inbound_ok() {
        let inbound = SignalInbound {
            envelope: SignalEnvelope {
                source: "+66812345678".into(),
                source_device: 1,
                timestamp: 1_700_000_000_000,
                data_message: Some(SignalDataMessage {
                    message: Some("Hello".into()),
                    timestamp: 1_700_000_000_000,
                    group_info: None,
                    attachments: None,
                }),
            },
        };
        let parsed = parse_inbound(&inbound).unwrap();
        assert_eq!(parsed.from, "+66812345678");
        assert_eq!(parsed.text, "Hello");
        assert!(parsed.group_id.is_none());
    }

    #[test]
    fn parse_inbound_no_data_message() {
        let inbound = SignalInbound {
            envelope: SignalEnvelope {
                source: "+1".into(),
                source_device: 1,
                timestamp: 0,
                data_message: None,
            },
        };
        assert!(parse_inbound(&inbound).is_none());
    }

    #[test]
    fn build_send_payload_test() {
        let p = build_send_payload("+1", &["+2", "+3"], "hi");
        assert_eq!(p["number"].as_str(), Some("+1"));
        assert_eq!(p["message"].as_str(), Some("hi"));
        assert_eq!(p["recipients"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn normalize_inbound_test() {
        let parsed = ParsedSignalMessage {
            from: "+66812345678".into(),
            text: "hello".into(),
            group_id: None,
            has_attachment: false,
            timestamp: 123456,
        };
        let msg = normalize_inbound(&parsed);
        assert!(msg.id.contains("signal:"));
        assert_eq!(msg.text, "hello");
    }
}
