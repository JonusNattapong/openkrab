use krabkrab::connectors::{slack, telegram};

#[test]
fn telegram_normalize_inbound() {
    let msg = telegram::normalize_inbound("hello", 1001, 42);
    assert!(msg.id.starts_with("tg:"));
    assert_eq!(msg.text, "hello");
}

#[test]
fn slack_normalize_inbound() {
    let msg = slack::normalize_inbound("ping", "C123", "U555");
    assert!(msg.id.starts_with("slack:"));
    assert_eq!(msg.text, "ping");
}
