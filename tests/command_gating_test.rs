use krabkrab::channels::command_gating::*;

#[test]
fn basic_authorization() {
    let auths = [CommandAuthorizer {
        configured: true,
        allowed: true,
    }];
    let res = resolve_command_authorized_from_authorizers(false, &auths, None);
    assert!(res);

    let auths2 = [CommandAuthorizer {
        configured: true,
        allowed: false,
    }];
    let res2 = resolve_command_authorized_from_authorizers(
        false,
        &auths2,
        Some(CommandGatingModeWhenAccessGroupsOff::Configured),
    );
    assert!(!res2);
}

#[test]
fn control_gate_blocking() {
    let auths = [CommandAuthorizer {
        configured: true,
        allowed: false,
    }];
    let out = resolve_control_command_gate(
        false,
        &auths,
        true,
        true,
        Some(CommandGatingModeWhenAccessGroupsOff::Configured),
    );
    assert!(!out.command_authorized);
    assert!(out.should_block);
}
