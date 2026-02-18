use krabkrab::connectors::{bluebubbles, discord, slack, telegram};

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

#[test]
fn discord_normalize_inbound() {
    let msg = discord::normalize_inbound("pong", 1001, 42, 7);
    assert!(msg.id.starts_with("discord:"));
    assert_eq!(msg.text, "pong");
    assert_eq!(msg.from.as_ref().map(|u| u.0.as_str()), Some("discord:42"));
}

#[test]
fn bluebubbles_normalize_inbound() {
    let msg = bluebubbles::normalize_inbound(
        "hello from imessage",
        "iMessage;-;+15551234567",
        "+15551234567",
        "msg-123",
    );
    assert!(msg.id.starts_with("bluebubbles:"));
    assert_eq!(msg.text, "hello from imessage");
    assert_eq!(
        msg.from.as_ref().map(|u| u.0.as_str()),
        Some("bluebubbles:+15551234567")
    );
}

#[test]
fn bluebubbles_normalize_target() {
    assert_eq!(
        bluebubbles::normalize_target("bluebubbles:CHAT_GUID:iMessage;-;+15551234567"),
        "iMessage;-;+15551234567"
    );
    assert_eq!(
        bluebubbles::normalize_target("bluebubbles:+15550001111"),
        "+15550001111"
    );
}
