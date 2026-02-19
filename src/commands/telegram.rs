use anyhow::{anyhow, Result};

fn normalize_target(to: &str) -> Result<String> {
    let normalized = to.trim();
    if normalized.is_empty() {
        return Err(anyhow!("recipient is required"));
    }
    Ok(normalized.to_string())
}

/// Prepare a Telegram outbound send request without network I/O.
pub fn telegram_send_dry_run_command(
    to: &str,
    text: &str,
    reply_to_message_id: Option<i64>,
) -> Result<String> {
    let chat_id = normalize_target(to)?;
    let payload = crate::connectors::telegram_client::build_telegram_http_payload(
        &chat_id,
        text,
        reply_to_message_id,
    );
    Ok(format!("telegram to={} payload={}", chat_id, payload))
}

/// Send an outbound Telegram message via Bot API.
///
/// Requires `TELEGRAM_BOT_TOKEN` in environment.
pub async fn telegram_send_command(
    to: &str,
    text: &str,
    reply_to_message_id: Option<i64>,
) -> Result<String> {
    let chat_id = normalize_target(to)?;
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| anyhow!("Missing TELEGRAM_BOT_TOKEN environment variable"))?;
    let client = reqwest::Client::new();
    let response = crate::connectors::telegram_client::send_message(
        &client,
        &token,
        &chat_id,
        text,
        reply_to_message_id,
    )
    .await?;

    if response
        .get("ok")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        let message_id = response
            .get("result")
            .and_then(|v| v.get("message_id"))
            .and_then(|v| v.as_i64())
            .map(|v| v.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        return Ok(format!(
            "sent telegram message chat={} message_id={}",
            chat_id, message_id
        ));
    }

    let description = response
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown Telegram API error");
    Err(anyhow!("Telegram API error: {}", description))
}
