use krabkrab::commands::bridge_command;

fn set_bridge_cmd_for_test() -> Option<String> {
    let key = "KRABKRAB_JS_BRIDGE_CMD";
    let previous = std::env::var(key).ok();
    #[cfg(target_os = "windows")]
    let cmd = "echo {\"ok\":true,\"layer\":\"js\",\"message\":\"integration-fallback\"}";
    #[cfg(not(target_os = "windows"))]
    let cmd = "printf '{\"ok\":true,\"layer\":\"js\",\"message\":\"integration-fallback\"}'";
    std::env::set_var(key, cmd);
    previous
}

fn restore_bridge_cmd(previous: Option<String>) {
    let key = "KRABKRAB_JS_BRIDGE_CMD";
    if let Some(value) = previous {
        std::env::set_var(key, value);
    } else {
        std::env::remove_var(key);
    }
}

#[test]
fn bridge_auto_mode_uses_js_for_js_primary_features() {
    let prev = set_bridge_cmd_for_test();
    let out = bridge_command("browser", Some("snapshot"), Some("{}"), None).unwrap();
    restore_bridge_cmd(prev);

    assert!(out.contains("feature=browser_automation"));
    assert!(out.contains("layer=js"));
}

#[test]
fn bridge_auto_mode_falls_back_from_rust_to_js() {
    let prev = set_bridge_cmd_for_test();
    let out = bridge_command("line_full", Some("sync_full"), Some("{}"), None).unwrap();
    restore_bridge_cmd(prev);

    assert!(out.contains("feature=line_full"));
    assert!(out.contains("layer=js"));
}
