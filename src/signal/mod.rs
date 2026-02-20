//! signal — Signal messenger connector.
//! Ported from `openkrab/extensions/signal/` (Phase 10, 13).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

pub mod client;
pub mod daemon;
pub mod format;
pub mod identity;
pub mod monitor;
pub mod send;

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    /// The phone number registered with signal-cli (e.g. "+15550123456").
    pub phone_number: String,
    /// Base URL of the signal-cli REST API (default: "http://localhost:8080").
    pub api_base: String,
    /// Account ID for multi-account setups
    pub account: Option<String>,
    /// Path to signal-cli binary (default: "signal-cli")
    pub cli_path: Option<String>,
    /// Auto-start signal-cli daemon (default: true if api_base is localhost)
    pub auto_start: Option<bool>,
    /// Max time to wait for daemon startup in ms (default: 30000)
    pub startup_timeout_ms: Option<u64>,
    /// Send read receipts via daemon
    pub send_read_receipts: Option<bool>,
}

impl Default for SignalConfig {
    fn default() -> Self {
        let phone_number = std::env::var("SIGNAL_PHONE_NUMBER").unwrap_or_default();
        let api_base =
            std::env::var("SIGNAL_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let account = None;
        let cli_path = Some("signal-cli".to_string());
        let auto_start = Some(
            api_base.starts_with("http://localhost") || api_base.starts_with("http://127.0.0.1"),
        );
        let startup_timeout_ms = Some(30_000);
        let send_read_receipts = Some(false);

        Self {
            phone_number,
            api_base,
            account,
            cli_path,
            auto_start,
            startup_timeout_ms,
            send_read_receipts,
        }
    }
}

impl SignalConfig {
    pub fn from_env() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<()> {
        if self.phone_number.is_empty() {
            bail!("SIGNAL_PHONE_NUMBER is required");
        }
        Ok(())
    }

    pub fn resolve_account(&self) -> &str {
        self.account.as_deref().unwrap_or(&self.phone_number)
    }
}

// ─── Inbound message types ────────────────────────────────────────────────────

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
    #[serde(rename = "syncMessage")]
    pub sync_message: Option<SignalSyncMessage>,
    #[serde(rename = "receiptMessage")]
    pub receipt_message: Option<SignalReceiptMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDataMessage {
    pub message: Option<String>,
    pub timestamp: i64,
    #[serde(rename = "groupInfo")]
    pub group_info: Option<SignalGroupInfo>,
    pub attachments: Option<Vec<SignalAttachment>>,
    pub reactions: Option<Vec<SignalReaction>>,
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
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalReaction {
    pub emoji: Option<String>,
    pub target_author_uuid: Option<String>,
    pub target_sent_timestamp: Option<i64>,
    pub target_author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalSyncMessage {
    #[serde(rename = "sentMessage")]
    pub sent_message: Option<SignalSentMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalSentMessage {
    pub destination: Option<String>,
    pub timestamp: i64,
    pub message: Option<String>,
    #[serde(rename = "groupInfo")]
    pub group_info: Option<SignalGroupInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalReceiptMessage {
    pub r#type: Option<i64>,
    pub timestamps: Option<Vec<i64>>,
}

// ─── Parsed message ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ParsedSignalMessage {
    pub from: String,
    pub text: String,
    pub group_id: Option<String>,
    pub has_attachment: bool,
    pub timestamp: i64,
    pub is_sync: bool,
    pub reaction: Option<SignalReactionInfo>,
}

#[derive(Debug, Clone)]
pub struct SignalReactionInfo {
    pub emoji: String,
    pub target_author: Option<String>,
    pub target_timestamp: i64,
}

pub fn parse_inbound(inbound: &SignalInbound) -> Option<ParsedSignalMessage> {
    let env = &inbound.envelope;

    // Handle syncMessage (sent from another device)
    if let Some(sync) = &env.sync_message {
        if let Some(sent) = &sync.sent_message {
            return Some(ParsedSignalMessage {
                from: sent.destination.clone()?,
                text: sent.message.clone().unwrap_or_default(),
                group_id: sent.group_info.as_ref().map(|g| g.group_id.clone()),
                has_attachment: false,
                timestamp: sent.timestamp,
                is_sync: true,
                reaction: None,
            });
        }
    }

    // Handle dataMessage (received from others)
    let data = env.data_message.as_ref()?;
    let text = data.message.clone().unwrap_or_default();

    // Parse reaction if present
    let reaction = data.reactions.as_ref().and_then(|reactions| {
        reactions.first().and_then(|r| {
            let emoji = r.emoji.as_ref()?.clone();
            let target_timestamp = r.target_sent_timestamp?;
            Some(SignalReactionInfo {
                emoji,
                target_author: r.target_author.clone().or(r.target_author_uuid.clone()),
                target_timestamp,
            })
        })
    });

    Some(ParsedSignalMessage {
        from: env.source.clone(),
        text,
        group_id: data.group_info.as_ref().map(|g| g.group_id.clone()),
        has_attachment: data
            .attachments
            .as_ref()
            .map(|a| !a.is_empty())
            .unwrap_or(false),
        timestamp: env.timestamp,
        is_sync: false,
        reaction,
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

/// Normalise to common message
pub fn normalize_inbound(parsed: &ParsedSignalMessage) -> crate::common::Message {
    crate::common::Message {
        id: format!("signal:{}:{}", parsed.from, parsed.timestamp),
        text: parsed.text.clone(),
        from: Some(crate::common::UserId(format!("signal:{}", parsed.from))),
    }
}

// ─── Event types for monitor ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SignalEvent {
    Message(ParsedSignalMessage),
    Reaction {
        from: String,
        emoji: String,
        target_author: Option<String>,
        target_timestamp: i64,
        timestamp: i64,
    },
    Receipt {
        from: String,
        r#type: i64,
        timestamps: Vec<i64>,
    },
    Connected,
    Disconnected,
    Error(String),
}

pub type SignalEventReceiver = mpsc::Receiver<SignalEvent>;
pub type SignalEventSender = mpsc::Sender<SignalEvent>;

pub fn create_signal_channel() -> (SignalEventSender, SignalEventReceiver) {
    mpsc::channel(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_api_base() {
        let cfg = SignalConfig {
            phone_number: "+1".into(),
            api_base: "http://localhost:8080".into(),
            account: None,
        };
        assert!(cfg.api_base.contains("localhost"));
    }

    #[test]
    fn config_validate_missing_phone() {
        let cfg = SignalConfig {
            phone_number: "".into(),
            api_base: "http://localhost:8080".into(),
            account: None,
        };
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
                    reactions: None,
                }),
                sync_message: None,
                receipt_message: None,
            },
        };
        let parsed = parse_inbound(&inbound).unwrap();
        assert_eq!(parsed.from, "+66812345678");
        assert_eq!(parsed.text, "Hello");
        assert!(parsed.group_id.is_none());
        assert!(!parsed.is_sync);
    }

    #[test]
    fn parse_inbound_with_reaction() {
        let inbound = SignalInbound {
            envelope: SignalEnvelope {
                source: "+66812345678".into(),
                source_device: 1,
                timestamp: 1_700_000_001_000,
                data_message: Some(SignalDataMessage {
                    message: None,
                    timestamp: 1_700_000_001_000,
                    group_info: None,
                    attachments: None,
                    reactions: Some(vec![SignalReaction {
                        emoji: Some("👍".into()),
                        target_author_uuid: None,
                        target_sent_timestamp: Some(1_700_000_000_000),
                        target_author: Some("+66800000000".into()),
                    }]),
                }),
                sync_message: None,
                receipt_message: None,
            },
        };
        let parsed = parse_inbound(&inbound).unwrap();
        assert!(parsed.text.is_empty());
        assert!(parsed.reaction.is_some());
        let reaction = parsed.reaction.unwrap();
        assert_eq!(reaction.emoji, "👍");
    }

    #[test]
    fn parse_inbound_sync_message() {
        let inbound = SignalInbound {
            envelope: SignalEnvelope {
                source: "+66812345678".into(),
                source_device: 1,
                timestamp: 1_700_000_000_000,
                data_message: None,
                sync_message: Some(SignalSyncMessage {
                    sent_message: Some(SignalSentMessage {
                        destination: Some("+66900000000".into()),
                        timestamp: 1_700_000_000_000,
                        message: Some("Hello from sync".into()),
                        group_info: None,
                    }),
                }),
                receipt_message: None,
            },
        };
        let parsed = parse_inbound(&inbound).unwrap();
        assert_eq!(parsed.from, "+66900000000");
        assert_eq!(parsed.text, "Hello from sync");
        assert!(parsed.is_sync);
    }

    #[test]
    fn parse_inbound_no_data_message() {
        let inbound = SignalInbound {
            envelope: SignalEnvelope {
                source: "+1".into(),
                source_device: 1,
                timestamp: 0,
                data_message: None,
                sync_message: None,
                receipt_message: None,
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
            is_sync: false,
            reaction: None,
        };
        let msg = normalize_inbound(&parsed);
        assert!(msg.id.contains("signal:"));
        assert_eq!(msg.text, "hello");
    }
}
