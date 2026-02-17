use krabkrab::channels::mention_gating::*;

#[test]
fn test_resolve_mention_gating_not_mentioned_requires_mention() {
    let params = MentionGateParams {
        require_mention: true,
        can_detect_mention: true,
        was_mentioned: false,
        implicit_mention: None,
        should_bypass_mention: None,
    };
    let res = resolve_mention_gating(params);
    assert_eq!(res.effective_was_mentioned, false);
    assert_eq!(res.should_skip, true);
}

#[test]
fn test_resolve_mention_gating_with_implicit() {
    let params = MentionGateParams {
        require_mention: true,
        can_detect_mention: true,
        was_mentioned: false,
        implicit_mention: Some(true),
        should_bypass_mention: None,
    };
    let res = resolve_mention_gating(params);
    assert_eq!(res.effective_was_mentioned, true);
    assert_eq!(res.should_skip, false);
}

#[test]
fn test_resolve_mention_with_bypass_conditions() {
    let params = MentionGateWithBypassParams {
        is_group: true,
        require_mention: true,
        can_detect_mention: true,
        was_mentioned: false,
        implicit_mention: None,
        has_any_mention: Some(false),
        allow_text_commands: true,
        has_control_command: true,
        command_authorized: true,
    };
    let res = resolve_mention_gating_with_bypass(params);
    assert_eq!(res.should_bypass_mention, true);
    assert_eq!(res.effective_was_mentioned, true);
    assert_eq!(res.should_skip, false);
}
