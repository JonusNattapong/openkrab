use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

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
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line reply_message failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Build the JSON payload for a Line broadcast message.
pub fn build_line_broadcast_payload(text: &str, notification_disabled: bool) -> serde_json::Value {
    json!({
        "messages": [
            {
                "type": "text",
                "text": text
            }
        ],
        "notificationDisabled": notification_disabled
    })
}

/// Build a minimal rich menu payload.
pub fn build_line_rich_menu_payload(
    name: &str,
    chat_bar_text: &str,
    selected: bool,
) -> serde_json::Value {
    json!({
        "size": {
            "width": 2500,
            "height": 843
        },
        "selected": selected,
        "name": name,
        "chatBarText": chat_bar_text,
        "areas": []
    })
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
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line push_message failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Broadcast a message to all users/followers.
pub async fn broadcast_message(
    client: &Client,
    token: &str,
    text: &str,
    notification_disabled: bool,
) -> Result<serde_json::Value> {
    let url = format!("{}/message/broadcast", LINE_API_BASE);
    let payload = build_line_broadcast_payload(text, notification_disabled);
    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line broadcast_message failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Create a rich menu.
pub async fn create_rich_menu(
    client: &Client,
    token: &str,
    rich_menu: &serde_json::Value,
) -> Result<serde_json::Value> {
    let url = format!("{}/richmenu", LINE_API_BASE);
    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(rich_menu)
        .send()
        .await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line create_rich_menu failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Delete a rich menu by id.
pub async fn delete_rich_menu(client: &Client, token: &str, rich_menu_id: &str) -> Result<()> {
    let url = format!("{}/richmenu/{}", LINE_API_BASE, rich_menu_id);
    let resp = client.delete(&url).bearer_auth(token).send().await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line delete_rich_menu failed ({}): {}",
            status,
            raw_body
        ));
    }
    Ok(())
}

/// Set a default rich menu for all users.
pub async fn set_default_rich_menu(client: &Client, token: &str, rich_menu_id: &str) -> Result<()> {
    let url = format!("{}/user/all/richmenu/{}", LINE_API_BASE, rich_menu_id);
    let resp = client.post(&url).bearer_auth(token).send().await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line set_default_rich_menu failed ({}): {}",
            status,
            raw_body
        ));
    }
    Ok(())
}

/// Get list of rich menus.
pub async fn list_rich_menus(client: &Client, token: &str) -> Result<serde_json::Value> {
    let url = format!("{}/richmenu/list", LINE_API_BASE);
    let resp = client.get(&url).bearer_auth(token).send().await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line list_rich_menus failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Retrieve the bot's profile info.
pub async fn get_bot_info(client: &Client, token: &str) -> Result<serde_json::Value> {
    let url = format!("{}/info", LINE_API_BASE);
    let resp = client.get(&url).bearer_auth(token).send().await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "line get_bot_info failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
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

    #[test]
    fn line_broadcast_payload_structure() {
        let p = build_line_broadcast_payload("hello all", true);
        assert_eq!(p["notificationDisabled"], true);
        let msgs = p["messages"].as_array().expect("messages");
        assert_eq!(msgs[0]["text"], "hello all");
    }

    #[test]
    fn line_rich_menu_payload_structure() {
        let p = build_line_rich_menu_payload("main", "Menu", true);
        assert_eq!(p["name"], "main");
        assert_eq!(p["chatBarText"], "Menu");
        assert_eq!(p["selected"], true);
        assert_eq!(p["size"]["width"], 2500);
    }
}
