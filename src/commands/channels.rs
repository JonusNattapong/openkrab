//! Channels command - Manage connected chat channels and accounts

use crate::config_io;
use crate::openkrab_config::{ChannelConfig, DiscordAccountConfig, TelegramAccountConfig};

pub fn channels_list_command() -> String {
    let mut output = String::from("Configured Channels:\n");
    output.push_str("====================\n\n");

    let config = match config_io::load_config() {
        Ok(cfg) => cfg,
        Err(e) => return format!("Failed to read configuration: {}", e),
    };

    let mut found_any = false;

    if let Some(channels) = config.channels {
        if let Some(telegram) = channels.telegram {
            for (id, acc) in telegram.accounts {
                found_any = true;
                let status_icon = if acc.enabled { "✓" } else { "✗" };
                output.push_str(&format!(
                    "{} Telegram ({})\n   Status: {}\n",
                    status_icon,
                    id,
                    if acc.enabled { "connected" } else { "disabled" }
                ));
            }
        }
        if let Some(discord) = channels.discord {
            for (id, acc) in discord.accounts {
                found_any = true;
                let status_icon = if acc.enabled { "✓" } else { "✗" };
                output.push_str(&format!(
                    "{} Discord ({})\n   Status: {}\n",
                    status_icon,
                    id,
                    if acc.enabled { "connected" } else { "disabled" }
                ));
            }
        }
        for (id, acc) in channels.slack {
            found_any = true;
            let status_icon = if acc.enabled { "✓" } else { "✗" };
            output.push_str(&format!(
                "{} Slack ({})\n   Status: {}\n",
                status_icon,
                id,
                if acc.enabled { "connected" } else { "disabled" }
            ));
        }
        for (id, acc) in channels.whatsapp {
            found_any = true;
            let status_icon = if acc.enabled { "✓" } else { "✗" };
            output.push_str(&format!(
                "{} WhatsApp ({})\n   Status: {}\n",
                status_icon,
                id,
                if acc.enabled { "connected" } else { "disabled" }
            ));
        }
    }

    if !found_any {
        output.push_str("No channels configured.\n");
    }

    output
}

pub fn channels_status_command() -> String {
    channels_list_command()
}

pub fn channels_add_command(channel: &str, token: Option<&str>) -> String {
    let mut config = match config_io::load_config() {
        Ok(cfg) => cfg,
        Err(_) => crate::openkrab_config::OpenKrabConfig::default(),
    };

    let mut channels_cfg = config.channels.take().unwrap_or_default();

    match channel.to_lowercase().as_str() {
        "telegram" => {
            let mut tg = channels_cfg.telegram.unwrap_or_default();
            tg.accounts.insert(
                "default".to_string(),
                TelegramAccountConfig {
                    enabled: true,
                    token: token.map(|s| s.to_string()),
                    ..Default::default()
                },
            );
            channels_cfg.telegram = Some(tg);
        }
        "discord" => {
            let mut dc = channels_cfg.discord.unwrap_or_default();
            dc.accounts.insert(
                "default".to_string(),
                DiscordAccountConfig {
                    enabled: true,
                    token: token.map(|s| s.to_string()),
                    ..Default::default()
                },
            );
            channels_cfg.discord = Some(dc);
        }
        "slack" => {
            channels_cfg.slack.insert(
                "default".to_string(),
                ChannelConfig {
                    enabled: true,
                    token: token.map(|s| s.to_string()),
                    ..Default::default()
                },
            );
        }
        "whatsapp" => {
            channels_cfg.whatsapp.insert(
                "default".to_string(),
                ChannelConfig {
                    enabled: true,
                    token: token.map(|s| s.to_string()),
                    ..Default::default()
                },
            );
        }
        _ => return format!("Unsupported channel: {}", channel),
    }

    config.channels = Some(channels_cfg);

    match config_io::save_config(&config) {
        Ok(_) => format!(
            "Added channel: {} with token {}",
            channel,
            token.unwrap_or("via env")
        ),
        Err(e) => format!("Failed to save config: {}", e),
    }
}

pub fn channels_remove_command(channel: &str) -> String {
    let mut config = match config_io::load_config() {
        Ok(cfg) => cfg,
        Err(_) => return "Failed to load configuration.".to_string(),
    };

    if let Some(mut channels_cfg) = config.channels.take() {
        match channel.to_lowercase().as_str() {
            "telegram" => channels_cfg.telegram = None,
            "discord" => channels_cfg.discord = None,
            "slack" => channels_cfg.slack.clear(),
            "whatsapp" => channels_cfg.whatsapp.clear(),
            _ => return format!("Unsupported channel: {}", channel),
        }
        config.channels = Some(channels_cfg);

        match config_io::save_config(&config) {
            Ok(_) => format!("Removed channel: {}", channel),
            Err(e) => format!("Failed to save config: {}", e),
        }
    } else {
        format!("Channel {} is not configured.", channel)
    }
}

pub fn channels_logs_command(channel: Option<&str>, lines: Option<usize>) -> String {
    let n = lines.unwrap_or(100);
    format!(
        "Fetching last {} lines of logs for channel: {:?} (requires live connection)",
        n, channel
    )
}
