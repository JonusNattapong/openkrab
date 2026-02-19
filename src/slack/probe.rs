use serde_json::json;

pub fn build_probe_request(channel: &str) -> serde_json::Value {
    json!({ "channel": channel, "action": "probe" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_request_contains_channel() {
        let r = build_probe_request("C123");
        assert_eq!(r["channel"], "C123");
        assert_eq!(r["action"], "probe");
    }
}
