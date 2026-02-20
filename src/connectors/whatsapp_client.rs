use anyhow::anyhow;
use anyhow::Result;
use reqwest_middleware::ClientWithMiddleware as Client;
use serde_json::json;

const WHATSAPP_API_BASE: &str = "https://graph.facebook.com/v19.0";

/// Build a WhatsApp Cloud API text message payload.
pub fn build_whatsapp_text_payload(to: &str, body: &str) -> serde_json::Value {
    json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "text",
        "text": {
            "preview_url": false,
            "body": body
        }
    })
}

/// Build a WhatsApp Cloud API template message payload.
pub fn build_whatsapp_template_payload(
    to: &str,
    template_name: &str,
    language_code: &str,
) -> serde_json::Value {
    json!({
        "messaging_product": "whatsapp",
        "to": to,
        "type": "template",
        "template": {
            "name": template_name,
            "language": {
                "code": language_code
            }
        }
    })
}

/// Send a text message via WhatsApp Cloud API.
/// `access_token` is the permanent/temporary system user token from Meta.
/// `phone_number_id` is the WhatsApp Business phone number ID (not the display number).
pub async fn send_message(
    client: &Client,
    access_token: &str,
    phone_number_id: &str,
    to: &str,
    body: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/{}/messages", WHATSAPP_API_BASE, phone_number_id);
    let payload = build_whatsapp_text_payload(to, body);
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "whatsapp send_message failed ({}): {}",
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

/// Mark a received WhatsApp message as read.
pub async fn mark_as_read(
    client: &Client,
    access_token: &str,
    phone_number_id: &str,
    message_id: &str,
) -> Result<serde_json::Value> {
    let url = format!("{}/{}/messages", WHATSAPP_API_BASE, phone_number_id);
    let payload = json!({
        "messaging_product": "whatsapp",
        "status": "read",
        "message_id": message_id
    });
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "whatsapp mark_as_read failed ({}): {}",
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

/// Send a template message via WhatsApp Cloud API.
pub async fn send_template(
    client: &Client,
    access_token: &str,
    phone_number_id: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value> {
    let url = format!("{}/{}/messages", WHATSAPP_API_BASE, phone_number_id);
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    let status = resp.status();
    let raw_body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!(
            "whatsapp send_template failed ({}): {}",
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
    fn whatsapp_text_payload_structure() {
        let p = build_whatsapp_text_payload("+66812345678", "Hello from bot!");
        assert_eq!(p["messaging_product"], "whatsapp");
        assert_eq!(p["to"], "+66812345678");
        assert_eq!(p["type"], "text");
        assert_eq!(p["text"]["body"], "Hello from bot!");
    }

    #[test]
    fn whatsapp_template_payload_structure() {
        let p = build_whatsapp_template_payload("+66812345678", "hello_world", "en_US");
        assert_eq!(p["type"], "template");
        assert_eq!(p["template"]["name"], "hello_world");
        assert_eq!(p["template"]["language"]["code"], "en_US");
    }
}
