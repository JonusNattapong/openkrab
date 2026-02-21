//! WhatsApp send command — port of `openkrab/extensions/whatsapp/src/send.ts`

use anyhow::{bail, Result};

use crate::connectors::whatsapp_client;

/// Send WhatsApp message via Cloud API
pub async fn send_whatsapp_message(
    to: &str,
    text: &str,
    access_token: &str,
    phone_number_id: &str,
) -> Result<serde_json::Value> {
    let client = crate::infra::retry_http::build_retrying_client();
    whatsapp_client::send_message(&client, access_token, phone_number_id, to, text).await
}

/// Send WhatsApp media message
pub async fn send_whatsapp_media(
    to: &str,
    text: Option<&str>,
    media_url: &str,
    access_token: &str,
    phone_number_id: &str,
) -> Result<serde_json::Value> {
    if media_url.trim().is_empty() {
        bail!("Media URL is required");
    }

    let client = crate::infra::retry_http::build_retrying_client();
    let url = format!(
        "https://graph.facebook.com/v19.0/{}/messages",
        phone_number_id
    );
    let mut payload = serde_json::json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "image",
        "image": {
            "link": media_url
        }
    });

    if let Some(caption) = text {
        payload["image"]["caption"] = serde_json::json!(caption);
    }

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
        return Err(anyhow::anyhow!(
            "whatsapp send_whatsapp_media failed ({}): {}",
            status,
            raw_body
        ));
    }

    if raw_body.trim().is_empty() {
        Ok(serde_json::json!({}))
    } else {
        Ok(serde_json::from_str(&raw_body)?)
    }
}

/// Send WhatsApp template message
pub async fn send_whatsapp_template(
    to: &str,
    template_name: &str,
    language_code: &str,
    access_token: &str,
    phone_number_id: &str,
) -> Result<serde_json::Value> {
    let client = crate::infra::retry_http::build_retrying_client();
    let payload =
        whatsapp_client::build_whatsapp_template_payload(to, template_name, language_code);
    whatsapp_client::send_template(&client, access_token, phone_number_id, payload).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_whatsapp_message_invalid_token() {
        let result = send_whatsapp_message(
            "1234567890",
            "test message",
            "invalid_token",
            "phone_number_id",
        )
        .await;

        // Should fail with auth error
        assert!(result.is_err());
    }
}
