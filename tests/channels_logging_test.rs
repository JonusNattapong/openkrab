use krabkrab::channels::logging::*;
use serde_json::json;

#[test]
fn test_log_inbound_drop_formats() {
    use std::sync::{Arc, Mutex};
    let captured = Arc::new(Mutex::new(String::new()));
    let cap = captured.clone();
    let logger: LogFn = Box::new(move |s: &str| {
        let mut g = cap.lock().unwrap();
        *g = s.to_string();
    });
    let params = InboundDropParams {
        log: logger,
        channel: "whatsapp".to_string(),
        reason: "no-route".to_string(),
        target: Some("+123".to_string()),
    };
    log_inbound_drop(&params);
    let out = captured.lock().unwrap();
    assert!(out.contains("drop no-route"));
}

#[test]
fn test_log_typing_failure_and_ack() {
    let logger2: LogFn = Box::new(|_s: &str| {});
    let params = TypingFailureParams {
        log: logger2,
        channel: "tg".to_string(),
        target: None,
        action: Some("start".to_string()),
        error: json!("boom"),
    };
    log_typing_failure(&params);

    let logger3: LogFn = Box::new(|_s: &str| {});
    let params2 = AckFailureParams {
        log: logger3,
        channel: "tg".to_string(),
        target: Some("@chan".to_string()),
        error: json!({ "code": 500 }),
    };
    log_ack_failure(&params2);
}
