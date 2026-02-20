//! Channels command - Manage connected chat channels and accounts

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub status: String,
}

pub fn channels_list_command() -> String {
    let channels = vec![
        ChannelInfo {
            id: "telegram".to_string(),
            name: "Telegram".to_string(),
            enabled: true,
            status: "connected".to_string(),
        },
        ChannelInfo {
            id: "slack".to_string(),
            name: "Slack".to_string(),
            enabled: true,
            status: "connected".to_string(),
        },
        ChannelInfo {
            id: "discord".to_string(),
            name: "Discord".to_string(),
            enabled: true,
            status: "connected".to_string(),
        },
    ];

    let mut output = String::from("Configured Channels:\n");
    output.push_str("====================\n\n");

    for channel in channels {
        let status_icon = if channel.enabled { "✓" } else { "✗" };
        output.push_str(&format!(
            "{} {} ({})\n   Status: {}\n\n",
            status_icon, channel.name, channel.id, channel.status
        ));
    }

    output
}

pub fn channels_status_command() -> String {
    channels_list_command()
}

pub fn channels_add_command(channel: &str, _token: Option<&str>) -> String {
    format!("Adding channel: {} with token...", channel)
}

pub fn channels_remove_command(channel: &str) -> String {
    format!("Removing channel: {}", channel)
}

pub fn channels_logs_command(channel: Option<&str>, lines: Option<usize>) -> String {
    let n = lines.unwrap_or(100);
    format!(
        "Fetching last {} lines of logs for channel: {:?}",
        n, channel
    )
}
