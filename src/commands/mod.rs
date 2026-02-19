//! krabkrab commands module — port of `openclaw/src/commands`

pub mod configure;
pub mod doctor;
pub mod doctor_auth;
pub mod doctor_gateway;
pub mod doctor_sandbox;
pub mod doctor_security;
pub mod discord;
pub mod onboard;
pub mod slack;
pub mod status;
pub mod status_update;
pub mod status_summary;
pub mod status_daemon;
pub mod telegram;
pub mod memory;
pub mod gateway;
pub mod ask;
pub mod bridge;
pub mod models;
pub mod models_auth;
pub mod channels;
pub mod logs;
pub mod config;
pub mod cron;
pub mod pairing;
pub mod admin;
pub mod oauth;
pub mod openai_codex_oauth;
pub mod chutes_oauth;
pub mod whatsapp_send;
pub mod onboard_types;
pub mod sandbox;
pub mod message;
pub mod sessions;
pub mod health;
pub mod signal_install;
pub mod setup;
pub mod reset;
pub mod uninstall;

pub use configure::{configure_command, configure_command_interactive, ConfigureInput};
pub use doctor::{doctor_command, doctor_simple, DoctorResult, format_doctor_result};
pub use doctor_auth::{
    format_remaining_short, 
    build_auth_health_summary, 
    note_auth_profile_health,
    check_anthropic_oauth_profile_repair,
};
pub use doctor_gateway::{
    check_gateway_health, 
    GatewayHealthResult, 
    ChannelIssue,
    collect_channel_status_issues,
    format_gateway_health,
};
pub use doctor_sandbox::{
    is_docker_available,
    docker_image_exists,
    check_sandbox_health,
    SandboxHealth,
    format_sandbox_health,
    note_sandbox_scope_warnings,
    DEFAULT_SANDBOX_IMAGE,
    DEFAULT_SANDBOX_BROWSER_IMAGE,
    DEFAULT_SANDBOX_COMMON_IMAGE,
};
pub use doctor_security::{
    check_security,
    SecurityCheckResult,
    note_security_warnings,
    format_security_check,
};
pub use discord::{discord_send_command, discord_send_dry_run_command};
pub use onboard::{onboard_command, onboard_wizard, onboard_quick};
pub use crate::shell::run_interactive_shell;
pub use slack::{slack_send_command, slack_send_dry_run_command};
pub use status::{status_command, status_simple, StatusOptions};
pub use status_update::{
    check_for_updates,
    format_update_available_hint,
    format_update_one_liner,
    UpdateCheckResult,
};
pub use status_summary::{
    build_status_summary,
    format_status_summary,
    redact_sensitive_status_summary,
    format_tokens_compact,
    format_duration,
    StatusSummary,
};
pub use status_daemon::{
    get_daemon_status,
    get_agent_local_status,
    format_daemon_status,
    format_agent_local_status,
    DaemonStatus,
    AgentLocalStatus,
};
pub use telegram::{telegram_send_command, telegram_send_dry_run_command};
pub use memory::{memory_sync_command, memory_search_command};
pub use gateway::gateway_start_command;
pub use ask::ask_command;
pub use bridge::bridge_command;
pub use models::models_list_command;
pub use models_auth::{
    models_auth_list_command,
    models_auth_add_command,
    models_auth_remove_command,
    models_auth_get_command,
};
pub use channels::{channels_list_command, channels_status_command, channels_add_command, channels_remove_command, channels_logs_command};
pub use logs::logs_tail_command;
pub use config::{config_show_command, config_get_command, config_set_command, config_edit_command};
pub use cron::{cron_list_command, cron_add_command, cron_remove_command, cron_enable_command, cron_disable_command};
pub use pairing::{pairing_list_command, pairing_approve_command, pairing_revoke_command, pairing_generate_command};
pub use admin::{
    update_command,
    skills_command,
    sandbox_command,
    nodes_command,
    browser_command,
    hooks_command,
    webhooks_command,
    exec_approvals_command,
    docs_command,
    dns_command,
    directory_command,
    system_command,
    devices_command,
    daemon_command,
};
pub use oauth::{
    login_minimax_oauth,
    login_qwen_oauth,
    login_github_copilot,
};
pub use openai_codex_oauth::{
    login_openai_codex_oauth,
    login_openai_codex_oauth_interactive,
};
pub use chutes_oauth::login_chutes_oauth;
pub use whatsapp_send::{
    send_whatsapp_message,
    send_whatsapp_media,
    send_whatsapp_template,
};
pub use onboard_types::{
    OnboardMode,
    ChannelConfig,
    OnboardResult,
    WizardStep,
    WizardState,
};
pub use sandbox::{
    sandbox_list_command,
    sandbox_build_command,
    sandbox_recreate_command,
    sandbox_explain_command,
    format_sandbox_status,
    format_simple_sandbox_status,
    SandboxContainerInfo,
    SandboxListResult,
};
pub use message::{
    message_send_command,
    format_message,
    MessageSendOptions,
};
pub use sessions::{
    sessions_list_command,
    sessions_get_command,
    sessions_lock_command,
    sessions_unlock_command,
    sessions_archive_command,
    sessions_delete_command,
    sessions_stats_command,
    SessionInfo,
    SessionListOptions,
    SessionStats,
};
pub use health::{
    health_command,
    format_health_result,
    format_health_check_failure,
    HealthResult,
    HealthCheck,
    CheckStatus,
};
pub use signal_install::{
    signal_install_command,
    is_signal_cli_installed,
    get_signal_cli_version,
    install_signal_cli,
    SignalInstallResult,
};
pub use setup::{
    setup_command,
    SetupOptions,
};
pub use reset::{
    reset_command,
    ResetOptions,
};
pub use uninstall::{
    uninstall_command,
    UninstallOptions,
};
