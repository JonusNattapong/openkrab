use krabkrab::channels::sender_label::*;

#[test]
fn test_resolve_sender_label_display_and_id() {
    let params = SenderLabelParams {
        name: Some("Alice".to_string()),
        username: None,
        tag: None,
        e164: Some("+123456789".to_string()),
        id: None,
    };
    let r = resolve_sender_label(&params).unwrap();
    assert_eq!(r, "Alice (+123456789)");
}

#[test]
fn test_resolve_sender_label_only_id() {
    let params = SenderLabelParams {
        name: None,
        username: None,
        tag: None,
        e164: None,
        id: Some("user-1".to_string()),
    };
    let r = resolve_sender_label(&params).unwrap();
    assert_eq!(r, "user-1");
}

#[test]
fn test_list_sender_label_candidates_includes_resolved() {
    let params = SenderLabelParams {
        name: Some("Bob".to_string()),
        username: Some("bobx".to_string()),
        tag: None,
        e164: None,
        id: Some("+999".to_string()),
    };
    let list = list_sender_label_candidates(&params);
    assert!(list.iter().any(|s| s == "Bob (+999)" || s == "Bob"));
}
