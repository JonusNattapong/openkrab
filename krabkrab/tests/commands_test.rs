use krabkrab::commands::configure::ConfigureInput;
use krabkrab::commands::{configure_command, slack_send_command, status_command, telegram_send_command};

#[test]
fn status_command_reports_ok() {
    let out = status_command();
    assert!(out.contains("OK"));
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
