//! message — Message sending commands.
//! Ported from `openkrab/src/commands/message.ts` (Phase 6).

use crate::common::Message;
use crate::connectors::{
    discord_client, line_client, slack_client, telegram_client, whatsapp_client,
};

/// Message send options.
#[derive(Debug, Clone)]
pub struct MessageSendOptions {
    pub channel: String,
    pub to: String,
    pub text: String,
    pub reply_to: Option<String>,
    pub silent: bool,
}

/// Send a message to a channel.
pub async fn message_send_command(opts: MessageSendOptions) -> anyhow::Result<String> {
    let _message = Message {
        id: format!("{}:{}", opts.channel, uuid::Uuid::new_v4()),
        text: opts.text.clone(),
        from: None,
    };

    // Route to appropriate connector
    let result = match opts.channel.as_str() {
        "slack" => send_slack_message(&opts).await,
        "telegram" => send_telegram_message(&opts).await,
        "discord" => send_discord_message(&opts).await,
        "line" => send_line_message(&opts).await,
        "whatsapp" => send_whatsapp_message(&opts).await,
        _ => Err(anyhow::anyhow!("Unknown channel: {}", opts.channel)),
    };

    match result {
        Ok(msg_id) => Ok(format!(
            "✓ Sent message to {} ({}): {}",
            opts.channel, opts.to, msg_id
        )),
        Err(e) => Err(anyhow::anyhow!("Failed to send message: {}", e)),
    }
}

/// Send message via Slack.
async fn send_slack_message(_opts: &MessageSendOptions) -> anyhow::Result<String> {
    let token = std::env::var("SLACK_BOT_TOKEN")
        .map_err(|_| anyhow::anyhow!("SLACK_BOT_TOKEN is not set"))?;
    let client = crate::infra::retry_http::build_retrying_client();
    let resp = slack_client::send_message(&client, &token, &_opts.to, &_opts.text, None).await?;

    if resp.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(anyhow::anyhow!("Slack API returned error: {}", resp));
    }

    Ok(resp
        .get("ts")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))
}

/// Send message via Telegram.
async fn send_telegram_message(_opts: &MessageSendOptions) -> anyhow::Result<String> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| anyhow::anyhow!("TELEGRAM_BOT_TOKEN is not set"))?;
    let client = crate::infra::retry_http::build_retrying_client();
    let reply_to_id = _opts.reply_to.as_ref().and_then(|s| s.parse::<i64>().ok());
    let resp =
        telegram_client::send_message(&client, &token, &_opts.to, &_opts.text, reply_to_id).await?;

    if resp.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(anyhow::anyhow!("Telegram API returned error: {}", resp));
    }

    Ok(resp
        .get("result")
        .and_then(|v| v.get("message_id"))
        .and_then(|v| v.as_i64())
        .map(|id| id.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))
}

/// Send message via Discord.
async fn send_discord_message(_opts: &MessageSendOptions) -> anyhow::Result<String> {
    let token = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| anyhow::anyhow!("DISCORD_BOT_TOKEN is not set"))?;
    let client = crate::infra::retry_http::build_retrying_client();
    let opts = discord_client::SendOptions {
        reply_to: _opts.reply_to.clone(),
        embeds: None,
        silent: _opts.silent,
    };
    let res =
        discord_client::send_message(&client, &token, &_opts.to, &_opts.text, Some(opts)).await?;
    Ok(res.message_id)
}

/// Send message via LINE.
async fn send_line_message(_opts: &MessageSendOptions) -> anyhow::Result<String> {
    let token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")
        .map_err(|_| anyhow::anyhow!("LINE_CHANNEL_ACCESS_TOKEN is not set"))?;
    let client = crate::infra::retry_http::build_base_client();
    let resp = line_client::push_message(&client, &token, &_opts.to, &_opts.text).await?;
    Ok(resp
        .get("sentMessages")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))
}

/// Send message via WhatsApp.
async fn send_whatsapp_message(_opts: &MessageSendOptions) -> anyhow::Result<String> {
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .map_err(|_| anyhow::anyhow!("WHATSAPP_ACCESS_TOKEN is not set"))?;
    let phone_number_id = std::env::var("WHATSAPP_PHONE_NUMBER_ID")
        .map_err(|_| anyhow::anyhow!("WHATSAPP_PHONE_NUMBER_ID is not set"))?;
    let client = crate::infra::retry_http::build_retrying_client();
    let resp = whatsapp_client::send_message(
        &client,
        &access_token,
        &phone_number_id,
        &_opts.to,
        &_opts.text,
    )
    .await?;
    Ok(resp
        .get("messages")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))
}

/// Format message for display.
pub fn format_message(message: &Message, channel: &str) -> String {
    format!(
        "[{}] {}: {}",
        channel,
        message
            .from
            .as_ref()
            .map(|f| f.0.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        message.text
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::UserId;

    #[test]
    fn test_format_message() {
        let msg = Message {
            id: "test-123".to_string(),
            text: "Hello world".to_string(),
            from: Some(UserId("user-456".to_string())),
        };

        let formatted = format_message(&msg, "slack");
        assert!(formatted.contains("[slack]"));
        assert!(formatted.contains("user-456"));
        assert!(formatted.contains("Hello world"));
    }
}
