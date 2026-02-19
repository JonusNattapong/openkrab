//! krabkrab commands module — port of `openclaw/src/commands`

pub mod configure;
pub mod doctor;
pub mod discord;
pub mod onboard;
pub mod slack;
pub mod status;
pub mod telegram;
pub mod memory;
pub mod gateway;
pub mod ask;
pub mod bridge;
pub mod models;
pub mod channels;
pub mod logs;
pub mod config;
pub mod cron;
pub mod pairing;
pub mod admin;

pub use configure::{configure_command, configure_command_interactive, ConfigureInput};
pub use doctor::doctor_command;
pub use discord::{discord_send_command, discord_send_dry_run_command};
pub use onboard::{onboard_command, onboard_wizard, onboard_quick, OnboardingConfig};
pub use crate::shell::run_interactive_shell;
pub use slack::slack_send_command;
pub use status::status_command;
pub use telegram::telegram_send_command;
pub use memory::{memory_sync_command, memory_search_command};
pub use gateway::gateway_start_command;
pub use ask::ask_command;
pub use bridge::bridge_command;
pub use models::models_list_command;
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
