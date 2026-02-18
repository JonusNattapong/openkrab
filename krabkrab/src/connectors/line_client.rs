use serde_json::json;
use anyhow::Result;
use reqwest::Client;

const LINE_API_BASE: &str = "https://api.line.me/v2/bot";

/// Build the JSON payload for a Line reply message
pub fn build_line_reply_payload(reply_token: &str, text: &str) -> serde_json::Value {
    json!({
        "replyToken": reply_token,
        "messages": [
            {
                "type": "text",
                "text": text
            }
        ]
    })
}

/// Build the JSON payload for a Line push message
pub fn build_line_push_payload(to: &str, text: &str) -> serde_json::Value {
    json!({
        "to": to,
        "messages": [
            {
                "type": "text",
                "text": text
            }
        ]
    })
}

/// Reply to a Line message using replyToken (within 30 seconds of the original event).
/// `token` is the Channel Access Token from Line Developers Console.
pub async fn reply_message(
    client: &Client,
    token: &str,
    reply_token: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/message/reply", LINE_API_BASE);
    let payload = build_line_reply_payload(reply_token, text);
    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    let v: serde_json::Value = resp.json().await?;
    Ok(v)
}

/// Push a message to a Line user/group/room proactively.
/// `to` is the user/group/room ID. Requires the Messaging API push plan.
pub async fn push_message(
    client: &Client,
    token: &str,
    to: &str,
    text: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/message/push", LINE_API_BASE);
    let payload = build_line_push_payload(to, text);
    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    let v: serde_json::Value = resp.json().await?;
    Ok(v)
}

/// Retrieve the bot's profile info.
pub async fn get_bot_info(client: &Client, token: &str) -> Result<serde_json::Value> {
    let url = format!("{}/info", LINE_API_BASE);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?;
    let v: serde_json::Value = resp.json().await?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_reply_payload_structure() {
        let p = build_line_reply_payload("test-reply-token", "hello");
        assert_eq!(p["replyToken"], "test-reply-token");
        let msgs = p["messages"].as_array().unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0]["type"], "text");
        assert_eq!(msgs[0]["text"], "hello");
    }

    #[test]
    fn line_push_payload_structure() {
        let p = build_line_push_payload("U12345", "push message");
        assert_eq!(p["to"], "U12345");
        let msgs = p["messages"].as_array().unwrap();
        assert_eq!(msgs[0]["text"], "push message");
    }
}
