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
pub mod models;

pub use configure::{configure_command, configure_command_interactive, ConfigureInput};
pub use doctor::doctor_command;
pub use discord::{discord_send_command, discord_send_dry_run_command};
pub use onboard::onboard_command;
pub use slack::slack_send_command;
pub use status::status_command;
pub use telegram::telegram_send_command;
pub use memory::{memory_sync_command, memory_search_command};
pub use gateway::gateway_start_command;
pub use ask::ask_command;
pub use models::models_list_command;
