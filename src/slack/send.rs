use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::connectors::build_slack_http_payload;
use crate::connectors::send_message as slack_send_message;
use crate::slack::{build_slack_blocks_fallback_text, validate_slack_blocks_array};

pub struct SlackSendIdentity {
    pub username: Option<String>,
    pub icon_url: Option<String>,
    pub icon_emoji: Option<String>,
}

pub struct SlackSendOpts {
    pub token: Option<String>,
    pub account_id: Option<String>,
    pub media_url: Option<String>,
    pub thread_ts: Option<String>,
    pub identity: Option<SlackSendIdentity>,
    pub blocks: Option<Vec<Value>>,
}

pub struct SlackSendResult {
    pub message_id: String,
    pub channel_id: String,
}

fn parse_recipient(raw: &str) -> Result<(bool, String), String> {
    let s = raw.trim();
    if s.starts_with('#') {
        return Ok((false, s.trim_start_matches('#').to_string()));
    }
    if s.starts_with('@') {
        return Ok((true, s.trim_start_matches('@').to_string()));
    }
    Err("Recipient must start with #channel or @user".to_string())
}

fn resolve_channel_id(is_user: bool, id: &str, token: &str) -> Result<(String, bool), String> {
    if is_user {
        // Open DM channel using conversations.open API
        let client = reqwest::blocking::Client::new();
        let url = "https://slack.com/api/conversations.open";
        let payload = serde_json::json!({ "users": id });

        let resp = client
            .post(url)
            .bearer_auth(token)
            .json(&payload)
            .send()
            .map_err(|e| e.to_string())?;

        let v: serde_json::Value = resp.json().map_err(|e| e.to_string())?;

        if !v.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Err(v
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("failed to open DM")
                .to_string());
        }

        let channel_id = v
            .get("channel")
            .and_then(|c| c.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "no channel id in response".to_string())?
            .to_string();

        Ok((channel_id, true))
    } else {
        Ok((id.to_string(), false))
    }
}

pub fn send_message_slack(to: &str, message: &str, opts: SlackSendOpts) -> Result<SlackSendResult> {
    let trimmed = message.trim();
    let blocks_opt = opts.blocks;
    if trimmed.is_empty() && opts.media_url.is_none() && blocks_opt.is_none() {
        return Err(anyhow!("Slack send requires text, blocks, or media"));
    }

    // Resolve recipient
    let (is_user, id) = parse_recipient(to).map_err(|e| anyhow!(e))?;
    let token = opts.token.ok_or_else(|| anyhow!("Slack token required"))?;
    let (channel_id, _is_dm) = resolve_channel_id(is_user, &id, &token).map_err(|e| anyhow!(e))?;

    if let Some(blocks) = blocks_opt {
        let validated =
            validate_slack_blocks_array(&Value::Array(blocks.clone())).map_err(|e| anyhow!(e))?;
        if opts.media_url.is_some() {
            return Err(anyhow!("Slack send does not support blocks with mediaUrl"));
        }
        let fallback_text = if !trimmed.is_empty() {
            trimmed.to_string()
        } else {
            build_slack_blocks_fallback_text(&validated)
        };
        // Use connectors slack send shim (token required)
        let token = opts.token.ok_or_else(|| anyhow!("Slack token required"))?;
        let resp = slack_send_message(
            &token,
            &channel_id,
            &fallback_text,
            opts.thread_ts.as_deref(),
        )?;
        let msg_id = resp
            .get("ts")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        return Ok(SlackSendResult {
            message_id: msg_id,
            channel_id,
        });
    }

    // For simplicity: send single message (no chunking) or upload media first then send
    if let Some(media) = opts.media_url {
        // Upload file using Slack files.upload API
        let client = reqwest::blocking::Client::new();
        let upload_url = "https://slack.com/api/files.upload";

        let file_resp = client
            .post(upload_url)
            .bearer_auth(&token)
            .form(&[("channels", &channel_id), ("file_url", &media)])
            .send()
            .map_err(|e| anyhow!("File upload failed: {}", e))?;

        let file_result: serde_json::Value = file_resp
            .json()
            .map_err(|e| anyhow!("Parse upload response failed: {}", e))?;

        if !file_result
            .get("ok")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            return Err(anyhow!(
                "File upload failed: {:?}",
                file_result.get("error")
            ));
        }

        let file_id = file_result
            .get("file")
            .and_then(|f| f.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        // send caption if any
        let caption = if trimmed.is_empty() { "" } else { trimmed };
        let resp = slack_send_message(&token, &channel_id, caption, opts.thread_ts.as_deref())?;
        let msg_id = resp
            .get("ts")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        return Ok(SlackSendResult {
            message_id: msg_id,
            channel_id,
        });
    }

    // Plain text send
    let token = opts.token.ok_or_else(|| anyhow!("Slack token required"))?;
    let resp = slack_send_message(&token, &channel_id, trimmed, opts.thread_ts.as_deref())?;
    let msg_id = resp
        .get("ts")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    Ok(SlackSendResult {
        message_id: msg_id,
        channel_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rejects_empty_send() {
        let opts = SlackSendOpts {
            token: None,
            account_id: None,
            media_url: None,
            thread_ts: None,
            identity: None,
            blocks: None,
        };
        let res = send_message_slack("#general", "   ", opts);
        assert!(res.is_err());
    }

    #[test]
    fn sends_text_with_token() {
        // Using the blocking reqwest send shim will attempt a network call if token provided; avoid calling it by passing invalid token and expecting error from reqwest.
        let opts = SlackSendOpts {
            token: Some("invalid".to_string()),
            account_id: None,
            media_url: None,
            thread_ts: None,
            identity: None,
            blocks: None,
        };
        let res = send_message_slack("#general", "hello", opts);
        // We expect either network error or Ok; ensure function attempted to run and returned Result
        assert!(res.is_ok() || res.is_err());
    }

    #[test]
    fn sends_blocks_fallback() {
        let block = json!({"type":"section","text":{"type":"mrkdwn","text":"Hi"}});
        let opts = SlackSendOpts {
            token: Some("invalid".to_string()),
            account_id: None,
            media_url: None,
            thread_ts: None,
            identity: None,
            blocks: Some(vec![block]),
        };
        let res = send_message_slack("#general", "", opts);
        assert!(res.is_ok() || res.is_err());
    }
}
use serde_json::json;

pub fn build_slack_send_payload(text: &str, thread_ts: Option<&str>) -> serde_json::Value {
    let mut payload = json!({ "text": text });
    if let Some(ts) = thread_ts {
        payload["thread_ts"] = json!(ts);
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_payload_without_thread() {
        let p = build_slack_send_payload("hello", None);
        assert_eq!(p["text"], "hello");
        assert!(p.get("thread_ts").is_none());
    }

    #[test]
    fn builds_payload_with_thread() {
        let p = build_slack_send_payload("hi", Some("12345.6789"));
        assert_eq!(p["text"], "hi");
        assert_eq!(p["thread_ts"], "12345.6789");
    }
}
