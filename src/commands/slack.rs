use anyhow::{anyhow, Result};

fn normalize_target(to: &str) -> Result<String> {
    let normalized = to.trim();
    if normalized.is_empty() {
        return Err(anyhow!("recipient is required"));
    }
    Ok(normalized.to_string())
}

/// Prepare a Slack outbound send request without network I/O.
pub fn slack_send_dry_run_command(to: &str, text: &str, thread_ts: Option<&str>) -> Result<String> {
    let channel = normalize_target(to)?;
    let payload = crate::connectors::slack_client::build_slack_http_payload(&channel, text, thread_ts);
    Ok(format!("slack to={} payload={}", channel, payload))
}

/// Send an outbound Slack message via Slack Web API.
///
/// Requires `SLACK_BOT_TOKEN` in environment.
pub async fn slack_send_command(to: &str, text: &str, thread_ts: Option<&str>) -> Result<String> {
    let channel = normalize_target(to)?;
    let token = std::env::var("SLACK_BOT_TOKEN")
        .map_err(|_| anyhow!("Missing SLACK_BOT_TOKEN environment variable"))?;
    let client = reqwest::Client::new();
    let response = crate::connectors::slack_client::send_message(
        &client,
        &token,
        &channel,
        text,
        thread_ts,
    )
    .await?;

    if response
        .get("ok")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        let ts = response
            .get("ts")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        return Ok(format!("sent slack message channel={} ts={}", channel, ts));
    }

    let error = response
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown Slack API error");
    Err(anyhow!("Slack API error: {}", error))
}

