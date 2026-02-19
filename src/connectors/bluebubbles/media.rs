//! BlueBubbles media/attachment sending.
//! Ported from openclaw/extensions/bluebubbles/src/media-send.ts

use serde::{Deserialize, Serialize};

use super::config::resolve_account;
use super::send::send_message;
use super::targets::extract_handle_from_chat_guid;
use super::types::{
    build_api_url, extract_message_id, normalize_handle, BlueBubblesConfig, BlueBubblesSendResult,
    DEFAULT_TIMEOUT_MS,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSendOptions {
    pub server_url: Option<String>,
    pub password: Option<String>,
    pub account_id: Option<String>,
    pub timeout_ms: Option<u64>,
    pub caption: Option<String>,
    pub reply_to_id: Option<String>,
}

pub fn send_media(
    to: &str,
    media_path: Option<&str>,
    media_url: Option<&str>,
    media_bytes: Option<Vec<u8>>,
    content_type: Option<&str>,
    filename: Option<&str>,
    config: &BlueBubblesConfig,
    opts: MediaSendOptions,
) -> Result<BlueBubblesSendResult, String> {
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

    let timeout = opts.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);

    if let Some(path) = media_path {
        send_media_file(
            base_url,
            password,
            to,
            path,
            content_type,
            filename,
            opts.caption.as_deref(),
            timeout,
        )
    } else if let Some(url) = media_url {
        send_media_url(
            base_url,
            password,
            to,
            url,
            content_type,
            filename,
            opts.caption.as_deref(),
            timeout,
        )
    } else if let Some(bytes) = media_bytes {
        send_media_bytes(
            base_url,
            password,
            to,
            &bytes,
            content_type,
            filename,
            opts.caption.as_deref(),
            timeout,
        )
    } else {
        Err("No media provided (path, url, or bytes required)".to_string())
    }
}

fn send_media_file(
    base_url: &str,
    password: &str,
    to: &str,
    file_path: &str,
    content_type: Option<&str>,
    filename: Option<&str>,
    caption: Option<&str>,
    timeout_ms: u64,
) -> Result<BlueBubblesSendResult, String> {
    let file_data = std::fs::read(file_path).map_err(|e| format!("failed to read file: {}", e))?;

    let mime = content_type.or_else(|| {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str());
        ext.map(|e| mime_from_ext(e))
    });

    let name = filename.or_else(|| {
        std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
    });

    send_media_bytes(
        base_url, password, to, &file_data, mime, name, caption, timeout_ms,
    )
}

fn send_media_url(
    base_url: &str,
    password: &str,
    to: &str,
    media_url: &str,
    content_type: Option<&str>,
    filename: Option<&str>,
    caption: Option<&str>,
    timeout_ms: u64,
) -> Result<BlueBubblesSendResult, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let response = client
        .get(media_url)
        .send()
        .map_err(|e| format!("failed to fetch media: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("media fetch failed: {}", response.status()));
    }

    let bytes = response
        .bytes()
        .map_err(|e| format!("failed to read media: {}", e))?
        .to_vec();

    send_media_bytes(
        base_url,
        password,
        to,
        &bytes,
        content_type,
        filename,
        caption,
        timeout_ms,
    )
}

fn send_media_bytes(
    base_url: &str,
    password: &str,
    to: &str,
    data: &[u8],
    content_type: Option<&str>,
    filename: Option<&str>,
    caption: Option<&str>,
    timeout_ms: u64,
) -> Result<BlueBubblesSendResult, String> {
    let target = to.to_string();
    let chat_guid = resolve_chat_guid_for_target(base_url, password, &target, timeout_ms)?;

    let url = build_api_url(base_url, "/api/v1/message/file", Some(password));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let mime = content_type.unwrap_or("application/octet-stream");
    let name = filename.unwrap_or("attachment");

    let part = reqwest::multipart::Part::bytes(data.to_vec())
        .file_name(name.to_string())
        .mime_str(mime)
        .map_err(|e| format!("mime error: {}", e))?;

    let mut form = reqwest::multipart::Form::new()
        .text("chatGuid", &chat_guid)
        .part("file", part);

    if let Some(cap) = caption {
        if !cap.is_empty() {
            form = form.text("message", cap.to_string());
        }
    }

    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .map_err(|e| format!("upload failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().unwrap_or_default();
        return Err(format!(
            "media send failed ({}): {}",
            response.status(),
            error_text
        ));
    }

    let json: serde_json::Value = response.json().map_err(|e| format!("parse error: {}", e))?;

    let message_id = extract_message_id(&json);

    Ok(BlueBubblesSendResult { message_id })
}

fn resolve_chat_guid_for_target(
    base_url: &str,
    password: &str,
    target: &str,
    timeout_ms: u64,
) -> Result<String, String> {
    if target.contains(";-;") || target.contains(";+;") {
        return Ok(target.to_string());
    }

    if target.starts_with("chat_guid:") {
        return Ok(target[10..].trim().to_string());
    }

    let normalized = normalize_handle(target);

    let url = build_api_url(base_url, "/api/v1/chat/query", Some(password));
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| format!("client error: {}", e))?;

    let payload = serde_json::json!({
        "limit": 100,
        "offset": 0,
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

    let chats = json
        .get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default();

    for chat in chats {
        let chat_guid = chat
            .get("chatGuid")
            .or(chat.get("guid"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(guid) = chat_guid {
            if let Some(handle) = extract_handle_from_chat_guid(&guid) {
                if handle == normalized {
                    return Ok(guid);
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

    Err(format!("no chat found for target: {}", target))
}

fn mime_from_ext(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "heic" => "image/heic",
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "wav" => "audio/wav",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_from_ext() {
        assert_eq!(mime_from_ext("jpg"), "image/jpeg");
        assert_eq!(mime_from_ext("png"), "image/png");
        assert_eq!(mime_from_ext("mp4"), "video/mp4");
        assert_eq!(mime_from_ext("unknown"), "application/octet-stream");
    }
}
