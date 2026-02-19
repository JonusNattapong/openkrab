use krabkrab::commands::configure::ConfigureInput;
use krabkrab::commands::status::get_status_summary;
use krabkrab::commands::{
    browser_command, configure_command, daemon_command, devices_command, directory_command,
    discord_send_dry_run_command, dns_command, docs_command, exec_approvals_command, hooks_command,
    models_list_command, nodes_command, sandbox_command, skills_command, slack_send_command,
    status_command, system_command, telegram_send_command, update_command, webhooks_command,
};

#[test]
fn status_command_reports_ok() {
    let out = status_command();
    assert!(out.contains("OK"));
}

#[test]
fn status_summary_includes_ollama_provider() {
    let s = get_status_summary();
    assert!(s.providers.iter().any(|p| p == "ollama"));
}

#[test]
fn configure_command_formats_output() {
    let out = configure_command(ConfigureInput {
        profile: "prod".to_string(),
        verbose: true,
    });
    assert!(out.contains("profile=prod"));
    assert!(out.contains("verbose=on"));
}

#[test]
fn telegram_command_formats_channel_prefix() {
    let out = telegram_send_command("ping");
    assert_eq!(out, "[telegram] ping");
}

#[test]
fn slack_command_formats_channel_prefix() {
    let out = slack_send_command("pong");
    assert_eq!(out, "[slack] pong");
}

#[test]
fn models_command_lists_copilot_models() {
    let out = models_list_command("copilot").unwrap();
    assert!(out.contains("provider=copilot"));
    assert!(out.contains("gpt-4o"));
}

#[test]
fn models_command_rejects_unknown_provider() {
    let err = models_list_command("bad-provider").unwrap_err();
    assert!(err.to_string().contains("unsupported provider"));
}

#[test]
fn discord_send_dry_run_normalizes_target() {
    let out = discord_send_dry_run_command(" 123456789012345678 ", "hi").unwrap();
    assert!(out.contains("to=channel:123456789012345678"));
    assert!(out.contains("[discord] hi"));
}

#[test]
fn discord_send_dry_run_rejects_empty_target() {
    let err = discord_send_dry_run_command("   ", "hello").unwrap_err();
    assert!(err.to_string().contains("recipient is required"));
}

#[test]
fn added_admin_commands_return_expected_markers() {
    assert!(update_command(true).contains("channel adapters"));
    assert!(skills_command("list").contains("skills:"));
    assert!(sandbox_command("status").contains("sandbox:"));
    assert!(nodes_command("list").contains("nodes:"));
    assert!(browser_command("status").contains("browser:"));
    assert!(hooks_command().contains("message:inbound"));
    assert!(webhooks_command("list").contains("webhooks:"));
    assert!(exec_approvals_command("list").contains("exec-approvals:"));
    assert!(docs_command(Some("gateway")).contains("docs.molt.bot/gateway"));
    assert!(dns_command("discover").contains("dns:"));
    assert!(directory_command("list").contains("directory:"));
    assert!(system_command().contains("system: os="));
    assert!(devices_command("list").contains("devices:"));
    assert!(daemon_command("status").contains("daemon: running="));
}
