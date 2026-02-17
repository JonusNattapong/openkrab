use krabkrab::channels::registry::*;

#[test]
fn list_and_normalize() {
    let list = list_chat_channels();
    assert!(!list.is_empty());
    assert_eq!(normalize_chat_channel_id(Some("imsg")), Some("imessage".to_string()));
    assert_eq!(normalize_chat_channel_id(Some("telegram")), Some("telegram".to_string()));
    assert_eq!(normalize_chat_channel_id(Some("unknown")), None);
}
