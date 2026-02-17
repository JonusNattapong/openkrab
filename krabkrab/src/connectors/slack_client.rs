use serde_json::json;
use anyhow::Result;
use reqwest::blocking::Client;
use std::time::Duration;

/// Build the JSON payload for posting a message to Slack
pub fn build_slack_http_payload(channel: &str, text: &str, thread_ts: Option<&str>) -> serde_json::Value {
    let mut payload = json!({
        "channel": channel,
        "text": text,
    });
    if let Some(ts) = thread_ts {
        payload["thread_ts"] = json!(ts);
    }
    payload
}

/// Send a message to Slack using a blocking reqwest client.
/// This is a thin shim; in tests we avoid calling it.
pub fn send_message(token: &str, channel: &str, text: &str, thread_ts: Option<&str>) -> Result<serde_json::Value> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let url = "https://slack.com/api/chat.postMessage";
    let payload = build_slack_http_payload(channel, text, thread_ts);
    let resp = client.post(url)
        .bearer_auth(token)
        .json(&payload)
        .send()?;
    let v: serde_json::Value = resp.json()?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_contains_channel_and_text() {
        let p = build_slack_http_payload("C1", "hello", None);
        assert_eq!(p["channel"], "C1");
        assert_eq!(p["text"], "hello");
        assert!(p.get("thread_ts").is_none());
    }

    #[test]
    fn payload_includes_thread_ts_when_provided() {
        let p = build_slack_http_payload("C1", "reply", Some("12345.6789"));
        assert_eq!(p["thread_ts"], "12345.6789");
    }
}
