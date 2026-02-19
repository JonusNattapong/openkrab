use anyhow::{anyhow, Result};

/// Prepare a Discord outbound send request without network I/O.
pub fn discord_send_dry_run_command(to: &str, text: &str) -> Result<String> {
    let normalized = crate::connectors::discord::normalize_outbound_target(Some(to))?;
    let preview = crate::connectors::discord::format_outbound(text);
    Ok(format!("discord to={} payload={}", normalized, preview))
}

/// Send an outbound Discord message via HTTP API path.
///
/// Requires `DISCORD_BOT_TOKEN` in environment.
pub async fn discord_send_command(to: &str, text: &str) -> Result<String> {
    let token = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| anyhow!("Missing DISCORD_BOT_TOKEN environment variable"))?;
    let client = reqwest::Client::new();
    let response =
        crate::connectors::discord::send_outbound_message(&client, &token, Some(to), text).await?;
    Ok(format!(
        "sent discord message id={} channel={}",
        response.message_id, response.channel_id
    ))
}
