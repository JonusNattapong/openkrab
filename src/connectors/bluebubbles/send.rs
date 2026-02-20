//! BlueBubbles send functionality.
//! Ported from openclaw/extensions/bluebubbles/src/send.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::config::{resolve_account, ResolvedBlueBubblesAccount};
use super::probe::probe_server;
use super::targets::{extract_handle_from_chat_guid, is_dm_chat_guid, ParsedTarget};
use super::types::{
    build_api_url, extract_message_id, normalize_handle, BlueBubblesConfig, BlueBubblesSendResult,
    BlueBubblesSendTarget, DEFAULT_TIMEOUT_MS,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendOptions {
    pub server_url: Option<String>,
    pub password: Option<String>,
    pub account_id: Option<String>,
    pub timeout_ms: Option<u64>,
    pub reply_to_message_guid: Option<String>,
    pub reply_to_part_index: Option<i32>,
    pub effect_id: Option<String>,
}

pub fn send_message(
    to: &str,
    text: &str,
    config: &BlueBubblesConfig,
    opts: SendOptions,
) -> Result<BlueBubblesSendResult, String> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return Err("BlueBubbles send requires text".to_string());
    }

    let account = resolve_account(config, opts.account_id.as_deref());
    if !account.configured {
        return Err("BlueBubbles account not configured".to_string());
    }

    let base_url = opts
        .server_url
        .as_ref()
        .or(account.config.server_url.as_ref())
        .ok_or("server_url required")?;

    let password = opts
        .password
        .as_ref()
        .or(account.config.password.as_ref())
        .ok_or("password required")?;

    let target = parse_target(to);
    let timeout = opts.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);

    let chat_guid = resolve_chat_guid(base_url, password, &target, timeout)?;

    send_to_chat(
        base_url,
        password,
        &chat_guid,
        trimmed_text,
        opts.reply_to_message_guid.as_deref(),
        opts.reply_to_part_index,
        opts.effect_id.as_deref(),
        timeout,
    )
}

fn parse_target(raw: &str) -> BlueBubblesSendTarget {
    ParsedTarget::parse(raw).to_send_target()
}

fn resolve_chat_guid(
    base_url: &str,
    password: &str,
    target: &BlueBubblesSendTarget,
    timeout_ms: u64,
) -> Result<String, String> {
    match target {
        BlueBubblesSendTarget::ChatGuid { chat_guid } => Ok(chat_guid.clone()),
        BlueBubblesSendTarget::ChatId { chat_id } => {
            query_chat_guid_by_id(base_url, password, *chat_id, timeout_ms)
        }
        BlueBubblesSendTarget::ChatIdentifier { chat_identifier } => {
            query_chat_guid_by_identifier(base_url, password, chat_identifier, timeout_ms)
        }
        BlueBubblesSendTarget::Handle {
            address,
            service: _,
        } => resolve_chat_guid_by_handle(base_url, password, address, timeout_ms),
    }
}

fn query_chat_guid_by_id(
    base_url: &str,
    password: &str,
    chat_id: i64,
    timeout_ms: u64,
) -> Result<String, String> {
    let chats = query_chats(base_url, password, 0, 100, timeout_ms)?;
    for chat in chats {
        if let Some(id) = chat
            .get("chatId")
            .or(chat.get("id"))
            .or(chat.get("chat_id"))
        {
            if let Some(n) = id.as_i64() {
                if n == chat_id {
                    return extract_chat_guid(&chat).ok_or("chat not found".to_string());
                }
            }
        }
    }
    Err(format!("chat with id {} not found", chat_id))
}

fn query_chat_guid_by_identifier(
    base_url: &str,
    password: &str,
    identifier: &str,
    timeout_ms: u64,
) -> Result<String, String> {
    let chats = query_chats(base_url, password, 0, 500, timeout_ms)?;

    for chat in &chats {
        if let Some(guid) = extract_chat_guid(&chat) {
            if let Some(extracted) = extract_handle_from_chat_guid(&guid) {
                if extracted == normalize_handle(identifier) || extracted == identifier {
                    return Ok(guid);
                }
            }
            if let Some(id) = chat
                .get("identifier")
                .or(chat.get("chatIdentifier"))
                .or(chat.get("chat_identifier"))
            {
                if let Some(s) = id.as_str() {
                    if s == identifier {
                        return Ok(guid);
                    }
                }
            }
        }
    }

    for chat in &chats {
        let participants = chat
            .get("participants")
            .and_then(|p| p.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        p.get("address")
                            .or(p.get("handle"))
                            .or(p.get("id"))
                            .and_then(|v| v.as_str())
                            .map(|s| normalize_handle(s))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let normalized = normalize_handle(identifier);
        if participants.contains(&normalized) {
            return extract_chat_guid(&chat).ok_or("chat not found".to_string());
        }
    }

    Err(format!("chat with identifier {} not found", identifier))
}

fn resolve_chat_guid_by_handle(
    base_url: &str,
    password: &str,
    handle: &str,
    timeout_ms: u64,
) -> Result<String, String> {
    let normalized = normalize_handle(handle);
    let limit = 500;
    let mut offset = 0;

    loop {
        let chats = query_chats(base_url, password, offset, limit, timeout_ms)?;
        if chats.is_empty() {
            break;
        }

        for chat in &chats {
            if let Some(guid) = extract_chat_guid(chat) {
                if is_dm_chat_guid(&guid) {
                    if let Some(sender) = extract_handle_from_chat_guid(&guid) {
                        if sender == normalized {
                            return Ok(guid);
                        }
                    }
                }

                let participants = chat
                    .get("participants")
                    .and_then(|p| p.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|p| {
                                p.get("address")
                                    .or(p.get("handle"))
                                    .or(p.get("id"))
                                    .and_then(|v| v.as_str())
                                    .map(|s| normalize_handle(s))
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                if participants.contains(&normalized) {
                    return Ok(guid);
                }
            }
        }

        offset += limit;
    }

    Err(format!("no chat found for handle {}", handle))
}

fn query_chats(
    base_url: &str,
    password: &str,
    offset: usize,
    limit: usize,
    timeout_ms: u64,
) -> Result<Vec<HashMap<String, serde_json::Value>>, String> {
    let url = build_api_url(base_url, "/api/v1/chat/query", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "offset": offset,
        "limit": limit,
        "with": ["participants"]
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let data = json
        .get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();

    Ok(data
        .into_iter()
        .filter_map(|v| {
            if let serde_json::Value::Object(m) = v {
                Some(m.into_iter().collect::<HashMap<_, _>>())
            } else {
                None
            }
        })
        .collect())
}

fn extract_chat_guid(chat: &HashMap<String, serde_json::Value>) -> Option<String> {
    let keys = [
        "chatGuid",
        "guid",
        "chat_guid",
        "identifier",
        "chatIdentifier",
        "chat_identifier",
    ];
    for key in keys {
        if let Some(v) = chat.get(key) {
            if let Some(s) = v.as_str() {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn send_to_chat(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    text: &str,
    reply_to_guid: Option<&str>,
    reply_to_part_index: Option<i32>,
    effect_id: Option<&str>,
    timeout_ms: u64,
) -> Result<BlueBubblesSendResult, String> {
    let url = build_api_url(base_url, "/api/v1/message/text", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let mut payload = serde_json::json!({
        "chatGuid": chat_guid,
        "tempGuid": uuid::Uuid::new_v4().to_string(),
        "message": text,
    });

    if reply_to_guid.is_some() {
        payload["method"] = serde_json::json!("private-api");
        if let Some(guid) = reply_to_guid {
            payload["selectedMessageGuid"] = serde_json::json!(guid);
        }
        if let Some(part_idx) = reply_to_part_index {
            payload["partIndex"] = serde_json::json!(part_idx);
        } else {
            payload["partIndex"] = serde_json::json!(0);
        }
    }

    if effect_id.is_some() {
        payload["method"] = serde_json::json!("private-api");
        payload["effectId"] = serde_json::json!(effect_id.unwrap());
    }

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        return Err(format!(
            "send failed ({}): {}",
            status,
            error_text
        ));
    }

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let message_id = extract_message_id(&json);

    Ok(BlueBubblesSendResult { message_id })
}

pub fn create_new_chat(
    base_url: &str,
    password: &str,
    address: &str,
    message: &str,
    timeout_ms: u64,
) -> Result<BlueBubblesSendResult, String> {
    let url = build_api_url(base_url, "/api/v1/chat/new", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "addresses": [address],
        "message": message,
        "tempGuid": format!("temp-{}", uuid::Uuid::new_v4()),
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        if error_text.to_lowercase().contains("private api") {
            return Err("Cannot create new chat: Private API must be enabled".to_string());
        }
        return Err(format!(
            "create chat failed ({}): {}",
            status,
            error_text
        ));
    }

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let message_id = extract_message_id(&json);

    Ok(BlueBubblesSendResult { message_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_target() {
        let target = parse_target("+15551234567");
        match target {
            BlueBubblesSendTarget::Handle { address, service } => {
                assert_eq!(address, "+15551234567");
            }
            _ => panic!("expected Handle"),
        }
    }

    #[test]
    fn test_parse_target_chat_guid() {
        let target = parse_target("chat_guid:iMessage;-;+15551234567");
        match target {
            BlueBubblesSendTarget::ChatGuid { chat_guid } => {
                assert_eq!(chat_guid, "iMessage;-;+15551234567");
            }
            _ => panic!("expected ChatGuid"),
        }
    }

    #[test]
    fn test_normalize_handle() {
        assert_eq!(normalize_handle("+1 (555) 123-4567"), "+15551234567");
    }
}
