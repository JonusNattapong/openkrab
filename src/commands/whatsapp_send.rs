//! WhatsApp send command — port of `openclaw/extensions/whatsapp/src/send.ts`

use anyhow::{bail, Result};
use reqwest::Client;

use crate::connectors::whatsapp_client;

/// Send WhatsApp message via Cloud API
pub async fn send_whatsapp_message(
    to: &str,
    text: &str,
    access_token: &str,
    phone_number_id: &str,
) -> Result<serde_json::Value> {
    let client = Client::new();
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
    // For now, just send text. In full implementation, would handle media upload
    if let Some(text) = text {
        send_whatsapp_message(to, text, access_token, phone_number_id).await
    } else {
        bail!("Media messages require text content");
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
    let client = Client::new();
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
