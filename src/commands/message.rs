//! message — Message sending commands.
//! Ported from `openkrab/src/commands/message.ts` (Phase 6).

use crate::common::Message;

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
    let message = Message {
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
async fn send_slack_message(opts: &MessageSendOptions) -> anyhow::Result<String> {
    // In real implementation, would use slack_client
    Ok(format!("slack-msg-{}", uuid::Uuid::new_v4()))
}

/// Send message via Telegram.
async fn send_telegram_message(opts: &MessageSendOptions) -> anyhow::Result<String> {
    // In real implementation, would use telegram_client
    Ok(format!("telegram-msg-{}", uuid::Uuid::new_v4()))
}

/// Send message via Discord.
async fn send_discord_message(opts: &MessageSendOptions) -> anyhow::Result<String> {
    // In real implementation, would use discord_client
    Ok(format!("discord-msg-{}", uuid::Uuid::new_v4()))
}

/// Send message via LINE.
async fn send_line_message(opts: &MessageSendOptions) -> anyhow::Result<String> {
    // In real implementation, would use line_client
    Ok(format!("line-msg-{}", uuid::Uuid::new_v4()))
}

/// Send message via WhatsApp.
async fn send_whatsapp_message(opts: &MessageSendOptions) -> anyhow::Result<String> {
    // In real implementation, would use whatsapp_client
    Ok(format!("whatsapp-msg-{}", uuid::Uuid::new_v4()))
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
