//! krabkrab commands module — port of `openkrab/src/commands`

pub mod admin;
pub mod ask;
pub mod bridge;
pub mod channels;
pub mod chutes_oauth;
pub mod config;
pub mod configure;
pub mod cron;
pub mod discord;
pub mod doctor;
pub mod doctor_auth;
pub mod doctor_gateway;
pub mod doctor_sandbox;
pub mod doctor_security;
pub mod gateway;
pub mod health;
pub mod logs;
pub mod memory;
pub mod message;
pub mod mission_control;
pub mod models;
pub mod models_auth;
pub mod oauth;
pub mod onboard;
pub mod onboard_types;
pub mod openai_codex_oauth;
pub mod pairing;
pub mod reset;
pub mod sandbox;
pub mod sessions;
pub mod setup;
pub mod signal_install;
pub mod slack;
pub mod status;
pub mod status_daemon;
pub mod status_summary;
pub mod status_update;
pub mod telegram;
pub mod uninstall;
pub mod whatsapp_send;

pub use crate::shell::run_interactive_shell;
pub use admin::{
    browser_command, daemon_command, devices_command, directory_command, dns_command, docs_command,
    exec_approvals_command, hooks_command, nodes_command, sandbox_command, skills_command,
    system_command, update_command, webhooks_command,
};
pub use ask::ask_command;
pub use bridge::bridge_command;
pub use channels::{
    channels_add_command, channels_list_command, channels_logs_command, channels_remove_command,
    channels_status_command,
};
pub use chutes_oauth::login_chutes_oauth;
pub use config::{
    config_edit_command, config_get_command, config_set_command, config_show_command,
};
pub use configure::{configure_command, configure_command_interactive, ConfigureInput};
pub use cron::{
    cron_add_command, cron_disable_command, cron_enable_command, cron_list_command,
    cron_remove_command,
};
pub use discord::{discord_send_command, discord_send_dry_run_command};
pub use doctor::{doctor_command, doctor_simple, format_doctor_result, DoctorResult};
pub use doctor_auth::{
    build_auth_health_summary, check_anthropic_oauth_profile_repair, format_remaining_short,
    note_auth_profile_health,
};
pub use doctor_gateway::{
    check_gateway_health, collect_channel_status_issues, format_gateway_health, ChannelIssue,
    GatewayHealthResult,
};
pub use doctor_sandbox::{
    check_sandbox_health, docker_image_exists, format_sandbox_health, is_docker_available,
    note_sandbox_scope_warnings, SandboxHealth, DEFAULT_SANDBOX_BROWSER_IMAGE,
    DEFAULT_SANDBOX_COMMON_IMAGE, DEFAULT_SANDBOX_IMAGE,
};
pub use doctor_security::{
    check_security, format_security_check, note_security_warnings, SecurityCheckResult,
};
pub use gateway::gateway_start_command;
pub use health::{
    format_health_check_failure, format_health_result, health_command, CheckStatus, HealthCheck,
    HealthResult,
};
pub use logs::logs_tail_command;
pub use memory::{memory_search_command, memory_sync_command};
pub use message::{format_message, message_send_command, MessageSendOptions};
pub use mission_control::mission_control_command;
pub use models::models_list_command;
pub use models_auth::{
    models_auth_add_command, models_auth_get_command, models_auth_list_command,
    models_auth_remove_command,
};
pub use oauth::{
    is_remote_environment, login_github_copilot, login_minimax_oauth, login_qwen_oauth,
};
pub use onboard::{onboard_command, onboard_quick, onboard_wizard};
pub use onboard_types::{ChannelConfig, OnboardMode, OnboardResult, WizardState, WizardStep};
pub use openai_codex_oauth::{login_openai_codex_oauth, login_openai_codex_oauth_interactive};
pub use pairing::{
    pairing_approve_command, pairing_generate_command, pairing_list_command, pairing_revoke_command,
};
pub use reset::{reset_command, ResetOptions};
pub use sandbox::{
    format_sandbox_status, format_simple_sandbox_status, sandbox_build_command,
    sandbox_explain_command, sandbox_list_command, sandbox_recreate_command, SandboxContainerInfo,
    SandboxListResult,
};
pub use sessions::{
    sessions_archive_command, sessions_delete_command, sessions_get_command, sessions_list_command,
    sessions_lock_command, sessions_stats_command, sessions_unlock_command, SessionInfo,
    SessionListOptions, SessionStats,
};
pub use setup::{setup_command, SetupOptions};
pub use signal_install::{
    get_signal_cli_version, install_signal_cli, is_signal_cli_installed, signal_install_command,
    SignalInstallResult,
};
pub use slack::{slack_send_command, slack_send_dry_run_command};
pub use status::{status_command, status_simple, StatusOptions};
pub use status_daemon::{
    format_agent_local_status, format_daemon_status, get_agent_local_status, get_daemon_status,
    AgentLocalStatus, DaemonStatus,
};
pub use status_summary::{
    build_status_summary, format_duration, format_status_summary, format_tokens_compact,
    redact_sensitive_status_summary, StatusSummary,
};
pub use status_update::{
    check_for_updates, format_update_available_hint, format_update_one_liner, UpdateCheckResult,
};
pub use telegram::{telegram_send_command, telegram_send_dry_run_command};
pub use uninstall::{uninstall_command, UninstallOptions};
pub use whatsapp_send::{send_whatsapp_media, send_whatsapp_message, send_whatsapp_template};
