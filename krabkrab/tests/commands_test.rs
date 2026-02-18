use krabkrab::commands::configure::ConfigureInput;
use krabkrab::commands::{
    configure_command, discord_send_dry_run_command, models_list_command, slack_send_command,
    status_command, telegram_send_command,
};
use krabkrab::commands::status::get_status_summary;

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
