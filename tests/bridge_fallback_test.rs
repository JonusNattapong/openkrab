use krabkrab::commands::bridge_command;

#[test]
fn bridge_layer_override_js_is_rejected() {
    let err =
        bridge_command("imessage_native", Some("status"), Some("{}"), Some("js")).unwrap_err();
    assert!(err.to_string().contains("removed"));
}

#[test]
fn bridge_auto_mode_without_fallback_returns_error() {
    let err =
        bridge_command("canvas_host", Some("unsupported_action"), Some("{}"), None).unwrap_err();
    assert!(err.to_string().contains("unsupported"));
}
