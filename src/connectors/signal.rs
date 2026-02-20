//! signal — Signal channel connector.
//! Ported from `openkrab/extensions/signal/` (Phase 5-6).

use crate::common::{Message, UserId};
use serde::{Deserialize, Serialize};

/// Signal message format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalMessage {
    pub envelope: SignalEnvelope,
}

/// Signal envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEnvelope {
    pub source: String,
    #[serde(rename = "sourceNumber")]
    pub source_number: Option<String>,
    #[serde(rename = "sourceUuid")]
    pub source_uuid: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    pub timestamp: u64,
    #[serde(rename = "dataMessage")]
    pub data_message: Option<SignalDataMessage>,
    #[serde(rename = "syncMessage")]
    pub sync_message: Option<SignalSyncMessage>,
}

/// Signal data message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDataMessage {
    pub message: Option<String>,
    pub attachments: Vec<SignalAttachment>,
}

/// Signal sync message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalSyncMessage {
    #[serde(rename = "sentMessage")]
    pub sent_message: Option<SignalSentMessage>,
}

/// Signal sent message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalSentMessage {
    pub message: Option<String>,
    #[serde(rename = "groupInfo")]
    pub group_info: Option<SignalGroupInfo>,
}

/// Signal group info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalGroupInfo {
    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// Signal attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalAttachment {
    pub id: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub filename: String,
}

/// Parse Signal message from JSON.
pub fn parse_signal_message(json: &serde_json::Value) -> Option<SignalMessage> {
    serde_json::from_value(json.clone()).ok()
}

/// Extract text from Signal message.
pub fn extract_signal_text(msg: &SignalMessage) -> Option<String> {
    // Try data message first
    if let Some(ref data) = msg.envelope.data_message {
        if let Some(ref text) = data.message {
            return Some(text.clone());
        }
    }

    // Try sync message
    if let Some(ref sync) = msg.envelope.sync_message {
        if let Some(ref sent) = sync.sent_message {
            if let Some(ref text) = sent.message {
                return Some(text.clone());
            }
        }
    }

    None
}

/// Normalize Signal message to common format.
pub fn normalize_inbound(text: &str, source: &str) -> Message {
    Message {
        id: format!("signal:{}", uuid::Uuid::new_v4()),
        text: text.to_string(),
        from: Some(UserId(format!("signal:{}", source))),
    }
}

/// Format outbound Signal message.
pub fn format_outbound(text: &str) -> String {
    format!("[signal] {}", text)
}

/// Build Signal text message payload.
pub fn build_text_payload(recipient: &str, text: &str) -> serde_json::Value {
    serde_json::json!({
        "recipient": recipient,
        "message": text
    })
}

/// Check if Signal CLI is available.
pub fn is_signal_cli_available() -> bool {
    std::process::Command::new("signal-cli")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_signal_text_data_message() {
        let msg = SignalMessage {
            envelope: SignalEnvelope {
                source: "+1234567890".to_string(),
                source_number: Some("+1234567890".to_string()),
                source_uuid: None,
                source_name: None,
                timestamp: 1234567890,
                data_message: Some(SignalDataMessage {
                    message: Some("Hello world".to_string()),
                    attachments: vec![],
                }),
                sync_message: None,
            },
        };

        assert_eq!(extract_signal_text(&msg), Some("Hello world".to_string()));
    }

    #[test]
    fn test_normalize_inbound() {
        let msg = normalize_inbound("Hello", "+1234567890");
        assert!(msg.id.starts_with("signal:"));
        assert_eq!(msg.text, "Hello");
        assert_eq!(msg.from.as_ref().unwrap().0, "signal:+1234567890");
    }

    #[test]
    fn test_build_text_payload() {
        let payload = build_text_payload("+1234567890", "Hello");
        assert_eq!(payload["recipient"], "+1234567890");
        assert_eq!(payload["message"], "Hello");
    }
}
