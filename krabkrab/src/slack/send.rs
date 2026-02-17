use serde_json::json;

pub fn build_slack_send_payload(text: &str, thread_ts: Option<&str>) -> serde_json::Value {
    let mut payload = json!({ "text": text });
    if let Some(ts) = thread_ts {
        payload["thread_ts"] = json!(ts);
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_payload_without_thread() {
        let p = build_slack_send_payload("hello", None);
        assert_eq!(p["text"], "hello");
        assert!(p.get("thread_ts").is_none());
    }

    #[test]
    fn builds_payload_with_thread() {
        let p = build_slack_send_payload("hi", Some("12345.6789"));
        assert_eq!(p["text"], "hi");
        assert_eq!(p["thread_ts"], "12345.6789");
    }
}
