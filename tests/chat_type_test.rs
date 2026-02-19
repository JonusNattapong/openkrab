use krabkrab::channels::chat_type::*;

#[test]
fn test_normalize_chat_type_direct() {
    assert_eq!(normalize_chat_type(Some("direct")), Some(ChatType::Direct));
    assert_eq!(normalize_chat_type(Some("DM")), Some(ChatType::Direct));
}

#[test]
fn test_normalize_chat_type_group_and_channel() {
    assert_eq!(normalize_chat_type(Some("group")), Some(ChatType::Group));
    assert_eq!(normalize_chat_type(Some("channel")), Some(ChatType::Channel));
}

#[test]
fn test_normalize_chat_type_unknown_or_empty() {
    assert_eq!(normalize_chat_type(Some("unknown")), None);
    assert_eq!(normalize_chat_type(None), None);
    assert_eq!(normalize_chat_type(Some("   ")), None);
}
