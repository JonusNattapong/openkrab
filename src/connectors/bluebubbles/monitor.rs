//! BlueBubbles webhook monitor.
//! Ported from openclaw/extensions/bluebubbles/src/monitor.ts

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use super::config::{resolve_account, resolve_webhook_path, ResolvedBlueBubblesAccount};
use super::types::{
    BlueBubblesAttachment, BlueBubblesConfig, NormalizedWebhookMessage, NormalizedWebhookReaction,
    WebhookPayload, DEFAULT_TIMEOUT_MS,
};

const DEFAULT_WEBHOOK_PATH: &str = "/webhook/bluebubbles";
const DEFAULT_INBOUND_DEBOUNCE_MS: u64 = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTarget {
    pub account: ResolvedBlueBubblesAccount,
    pub config: BlueBubblesConfig,
    pub path: String,
    pub status: Option<RuntimeStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeStatus {
    pub running: bool,
    pub last_inbound_at: Option<i64>,
    pub last_outbound_at: Option<i64>,
    pub last_error: Option<String>,
}

pub struct Monitor {
    targets: Arc<Mutex<HashMap<String, Vec<WebhookTarget>>>>,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            targets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, target: WebhookTarget) -> impl Fn() {
        let path = target.path.clone();
        let targets = Arc::clone(&self.targets);

        {
            let mut targets_lock = targets.lock().unwrap();
            targets_lock
                .entry(path.clone())
                .or_insert_with(Vec::new)
                .push(target);
        }

        let targets_for_closure = Arc::clone(&targets);
        Box::new(move || {
            let mut targets_lock = targets_for_closure.lock().unwrap();
            if let Some(vec) = targets_lock.get_mut(&path) {
                vec.retain(|t| t.path != path);
            }
        })
    }

    pub fn resolve_target(&self, path: &str) -> Option<WebhookTarget> {
        let targets = self.targets.lock().unwrap();
        targets.get(path).and_then(|v| v.first().cloned())
    }

    pub fn list_paths(&self) -> Vec<String> {
        let targets = self.targets.lock().unwrap();
        targets.keys().cloned().collect()
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_webhook_payload(body: &[u8]) -> Result<WebhookPayload, String> {
    let raw = String::from_utf8(body.to_vec()).map_err(|e| format!("invalid UTF-8: {}", e))?;

    serde_json::from_str(&raw).map_err(|e| format!("invalid JSON: {}", e))
}

pub fn normalize_webhook_message(payload: &serde_json::Value) -> Option<NormalizedWebhookMessage> {
    let data = payload.get("data")?;

    let text = data
        .get("text")
        .or_else(|| payload.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let sender_id = data
        .get("handleId")
        .or_else(|| data.get("sender"))
        .or_else(|| payload.get("sender"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let chat_guid = data
        .get("chatGuid")
        .or_else(|| data.get("chat_guid"))
        .or_else(|| payload.get("chatGuid"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let message_id = data
        .get("guid")
        .or_else(|| data.get("messageGuid"))
        .or_else(|| payload.get("guid"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let is_group = chat_guid
        .as_ref()
        .map(|g| g.contains(";+;"))
        .unwrap_or(false);

    let timestamp = data
        .get("dateCreated")
        .or_else(|| data.get("timestamp"))
        .and_then(|v| v.as_i64());

    let from_me = data
        .get("fromMe")
        .or_else(|| payload.get("fromMe"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let attachments = data
        .get("attachments")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    let att = a.as_object()?;
                    Some(BlueBubblesAttachment {
                        guid: att
                            .get("guid")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        uti: att
                            .get("uti")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        mime_type: att
                            .get("mimeType")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        transfer_name: att
                            .get("transferName")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        total_bytes: att.get("totalBytes").and_then(|v| v.as_i64()),
                        height: att.get("height").and_then(|v| v.as_i64()).map(|v| v as i32),
                        width: att.get("width").and_then(|v| v.as_i64()).map(|v| v as i32),
                        original_rowid: att
                            .get("originalROWID")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32),
                    })
                })
                .collect()
        });

    let reply_to_id = data
        .get("replyTo")
        .or_else(|| payload.get("replyTo"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Some(NormalizedWebhookMessage {
        message_id,
        chat_guid,
        chat_id: None,
        chat_identifier: None,
        sender_id,
        text,
        is_group,
        from_me,
        timestamp,
        attachments,
        reply_to_id: reply_to_id.clone(),
        reply_to_body: None,
        reply_to_sender: None,
        balloon_bundle_id: data
            .get("balloonBundleId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        associated_message_guid: data
            .get("associatedMessageGuid")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    })
}

pub fn normalize_webhook_reaction(
    payload: &serde_json::Value,
) -> Option<NormalizedWebhookReaction> {
    let data = payload.get("data")?;

    let message_id = data
        .get("messageGuid")
        .or_else(|| payload.get("messageGuid"))
        .and_then(|v| v.as_str())?
        .to_string();

    let sender_id = data
        .get("handleId")
        .or_else(|| data.get("sender"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let action = data
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("add")
        .to_string();

    let reaction = data
        .get("reaction")
        .or_else(|| payload.get("reaction"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Some(NormalizedWebhookReaction {
        message_id,
        sender_id,
        action,
        reaction,
    })
}

pub fn is_event_type_allowed(event_type: &str) -> bool {
    matches!(
        event_type,
        "new-message" | "updated-message" | "message-reaction" | "reaction"
    )
}

pub fn extract_event_type(payload: &WebhookPayload) -> Option<String> {
    payload.event_type.as_ref().map(|s| s.trim().to_string())
}

pub fn validate_auth_token(provided: Option<&str>, expected: Option<&str>) -> bool {
    let provided = provided.unwrap_or("").trim();
    let expected = expected.unwrap_or("").trim();

    if expected.is_empty() {
        return false;
    }

    if provided.is_empty() {
        return false;
    }

    if provided == expected {
        return true;
    }

    if let Some(stripped) = provided.strip_prefix("Bearer ") {
        return stripped.trim() == expected;
    }

    false
}

pub fn is_local_request(remote_addr: Option<&str>, host: Option<&str>) -> bool {
    let remote = remote_addr.unwrap_or("").trim().to_lowercase();
    let remote_is_loopback =
        remote == "127.0.0.1" || remote == "::1" || remote == "::ffff:127.0.0.1";

    if !remote_is_loopback {
        return false;
    }

    let host = host.unwrap_or("").trim().to_lowercase();
    let host_is_local = host == "localhost" || host == "127.0.0.1" || host == "::1";

    host_is_local
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_new() {
        let monitor = Monitor::new();
        let paths = monitor.list_paths();
        assert!(paths.is_empty());
    }

    #[test]
    fn test_is_event_type_allowed() {
        assert!(is_event_type_allowed("new-message"));
        assert!(is_event_type_allowed("message-reaction"));
        assert!(!is_event_type_allowed("unknown"));
    }

    #[test]
    fn test_validate_auth_token() {
        assert!(validate_auth_token(Some("secret"), Some("secret")));
        assert!(validate_auth_token(Some("Bearer secret"), Some("secret")));
        assert!(!validate_auth_token(Some("wrong"), Some("secret")));
        assert!(!validate_auth_token(None, Some("secret")));
    }

    #[test]
    fn test_is_local_request() {
        assert!(is_local_request(Some("127.0.0.1"), Some("localhost")));
        assert!(is_local_request(Some("::1"), Some("127.0.0.1")));
        assert!(!is_local_request(Some("192.168.1.1"), Some("localhost")));
    }
}
