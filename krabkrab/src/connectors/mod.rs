//! Connectors ported from openclaw channels

pub mod slack;
pub mod telegram;
pub mod slack_client;
pub mod telegram_client;

pub use slack::SlackConnector;
pub use slack_client::build_slack_http_payload;
pub use telegram_client::build_telegram_http_payload;
