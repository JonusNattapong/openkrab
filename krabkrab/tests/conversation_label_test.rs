use krabkrab::channels::conversation_label::*;

#[test]
fn test_resolve_conversation_label_explicit() {
    let ctx = MsgContext { ConversationLabel: Some("Explicit".to_string()), ..Default::default() };
    assert_eq!(resolve_conversation_label(&ctx).unwrap(), "Explicit");
}

#[test]
fn test_resolve_conversation_label_direct() {
    let mut ctx = MsgContext::default();
    ctx.ChatType = Some("direct".to_string());
    ctx.SenderName = Some("Alice".to_string());
    assert_eq!(resolve_conversation_label(&ctx).unwrap(), "Alice");
}

#[test]
fn test_resolve_conversation_label_append_id() {
    let mut ctx = MsgContext::default();
    ctx.GroupChannel = Some("MyGroup".to_string());
    ctx.From = Some("xyz:123456".to_string());
    let res = resolve_conversation_label(&ctx).unwrap();
    assert!(res.contains("id:123456"));
}
