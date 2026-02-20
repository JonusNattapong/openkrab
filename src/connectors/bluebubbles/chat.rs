//! BlueBubbles chat operations (group management).
//! Ported from openkrab/extensions/bluebubbles/src/chat.ts

use serde::{Deserialize, Serialize};

use super::types::{build_api_url, BlueBubblesSendResult, DEFAULT_TIMEOUT_MS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResult {
    pub chat_guid: Option<String>,
    pub ok: bool,
    pub error: Option<String>,
}

/// Helper: check response status and return Err with formatted message on failure.
fn check_response(
    response: reqwest::blocking::Response,
    action: &str,
) -> Result<reqwest::blocking::Response, String> {
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().unwrap_or_default();
        return Err(format!("{} failed ({}): {}", action, status, error_text));
    }
    Ok(response)
}

pub fn rename_chat(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    new_name: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/chat/rename", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
        "newName": new_name,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "rename")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

pub fn set_group_icon(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    image_path: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/chat/icon", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let image_data =
        std::fs::read(image_path).map_err(|e| format!("failed to read image: {}", e))?;

    let part = reqwest::blocking::multipart::Part::bytes(image_data)
        .file_name("icon.jpg")
        .mime_str("image/jpeg")
        .map_err(|e| format!("mime error: {}", e))?;

    let form = reqwest::blocking::multipart::Form::new()
        .text("chatGuid", chat_guid.to_string())
        .part("icon", part);

    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "set icon")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

pub fn add_participant(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    address: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/chat/participant/add", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
        "address": address,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "add participant")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

pub fn remove_participant(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    address: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/chat/participant/remove", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
        "address": address,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "remove participant")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

pub fn leave_chat(
    base_url: &str,
    password: &str,
    chat_guid: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/chat/leave", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "chatGuid": chat_guid,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "leave chat")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

pub fn edit_message(
    base_url: &str,
    password: &str,
    message_guid: &str,
    new_text: &str,
    timeout_ms: Option<u64>,
) -> Result<BlueBubblesSendResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/message/edit", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "messageGuid": message_guid,
        "newText": new_text,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    let response = check_response(response, "edit message")?;

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let message_id = json
        .get("data")
        .and_then(|d| d.get("guid"))
        .and_then(|g| g.as_str())
        .unwrap_or("ok")
        .to_string();

    Ok(BlueBubblesSendResult { message_id })
}

pub fn unsend_message(
    base_url: &str,
    password: &str,
    message_guid: &str,
    chat_guid: &str,
    timeout_ms: Option<u64>,
) -> Result<ChatResult, String> {
    let timeout = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    let url = build_api_url(base_url, "/api/v1/message/unsend", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "messageGuid": message_guid,
        "chatGuid": chat_guid,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .map_err(|e| format!("request failed: {}", e))?;

    check_response(response, "unsend")?;

    Ok(ChatResult {
        chat_guid: Some(chat_guid.to_string()),
        ok: true,
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_result_default() {
        let result = ChatResult {
            chat_guid: None,
            ok: false,
            error: None,
        };
        assert!(!result.ok);
    }
}
