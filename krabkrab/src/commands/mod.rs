//! krabkrab commands module — port of `openclaw/src/commands`

pub mod configure;
pub mod doctor;
pub mod onboard;
pub mod slack;
pub mod status;
pub mod telegram;

pub use configure::configure_command;
pub use doctor::doctor_command;
pub use onboard::onboard_command;
pub use slack::slack_send_command;
pub use status::status_command;
pub use telegram::telegram_send_command;
