use krabkrab::channels::sender_identity::*;

#[test]
fn test_validate_missing_identity_in_group() {
    let ctx = MsgContext {
        ChatType: Some("group".to_string()),
        SenderId: None,
        SenderName: None,
        SenderUsername: None,
        SenderE164: None,
    };
    let issues = validate_sender_identity(&ctx);
    assert!(issues.iter().any(|s| s.contains("missing sender identity")));
}

#[test]
fn test_validate_e164_invalid() {
    let ctx = MsgContext {
        ChatType: Some("group".to_string()),
        SenderId: None,
        SenderName: None,
        SenderUsername: None,
        SenderE164: Some("+12".to_string()),
    };
    let issues = validate_sender_identity(&ctx);
    assert!(issues.iter().any(|s| s.starts_with("invalid SenderE164")));
}

#[test]
fn test_validate_username_constraints() {
    let ctx = MsgContext {
        ChatType: Some("group".to_string()),
        SenderId: None,
        SenderName: None,
        SenderUsername: Some("bad @name".to_string()),
        SenderE164: None,
    };
    let issues = validate_sender_identity(&ctx);
    assert!(issues.iter().any(|s| s.contains("should not include \"@\"")));
    assert!(issues.iter().any(|s| s.contains("whitespace")));
}
