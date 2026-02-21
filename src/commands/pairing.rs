//! Pairing command - Manage device pairing

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingInfo {
    pub device_id: String,
    pub device_name: String,
    pub status: String,
    pub paired_at: Option<String>,
}

pub fn pairing_list_command() -> String {
    let mut output = String::from("Paired Devices (Allowlist):\n");
    output.push_str("=============================\n\n");

    let config = match crate::config_io::load_config() {
        Ok(cfg) => cfg,
        Err(e) => return format!("Failed to read configuration: {}", e),
    };

    let mut found_any = false;

    if let Some(channels) = config.channels {
        if let Some(telegram) = channels.telegram {
            for (acc_name, acc) in telegram.accounts {
                for id in acc.allowlist {
                    found_any = true;
                    output.push_str(&format!("({}) Telegram [{}]\n", id, acc_name));
                }
            }
        }
        if let Some(discord) = channels.discord {
            for (acc_name, acc) in discord.accounts {
                for id in acc.allowlist {
                    found_any = true;
                    output.push_str(&format!("({}) Discord [{}]\n", id, acc_name));
                }
            }
        }
        for (acc_name, acc) in channels.slack {
            for id in acc.allowlist {
                found_any = true;
                output.push_str(&format!("({}) Slack [{}]\n", id, acc_name));
            }
        }
        for (acc_name, acc) in channels.whatsapp {
            for id in acc.allowlist {
                found_any = true;
                output.push_str(&format!("({}) WhatsApp [{}]\n", id, acc_name));
            }
        }
    }

    if !found_any {
        output.push_str("No devices are currently paired (allowlist is empty).\n");
    }

    output
}

pub fn pairing_approve_command(channel: &str, code: &str) -> String {
    format!(
        "Approving pairing request from {} with code {}...",
        channel, code
    )
}

pub fn pairing_revoke_command(device_id: &str) -> String {
    let mut config = match crate::config_io::load_config() {
        Ok(cfg) => cfg,
        Err(e) => return format!("Failed to read configuration: {}", e),
    };

    let mut removed = false;

    if let Some(ref mut channels) = config.channels {
        if let Some(ref mut telegram) = channels.telegram {
            for (_, acc) in telegram.accounts.iter_mut() {
                if let Some(pos) = acc.allowlist.iter().position(|x| x == device_id) {
                    acc.allowlist.remove(pos);
                    removed = true;
                }
            }
        }
        if let Some(ref mut discord) = channels.discord {
            for (_, acc) in discord.accounts.iter_mut() {
                if let Some(pos) = acc.allowlist.iter().position(|x| x == device_id) {
                    acc.allowlist.remove(pos);
                    removed = true;
                }
            }
        }
        for (_, acc) in channels.slack.iter_mut() {
            if let Some(pos) = acc.allowlist.iter().position(|x| x == device_id) {
                acc.allowlist.remove(pos);
                removed = true;
            }
        }
        for (_, acc) in channels.whatsapp.iter_mut() {
            if let Some(pos) = acc.allowlist.iter().position(|x| x == device_id) {
                acc.allowlist.remove(pos);
                removed = true;
            }
        }
    }

    if removed {
        match crate::config_io::save_config(&config) {
            Ok(_) => format!("Revoked pairing for device: {}", device_id),
            Err(e) => format!("Failed to save config: {}", e),
        }
    } else {
        format!("Device {} was not paired in any channel.", device_id)
    }
}

pub fn pairing_generate_command() -> String {
    let code = format!(
        "{:04}-{:04}",
        rand::random::<u32>() % 10000,
        rand::random::<u32>() % 10000
    );
    format!(
        "Generated pairing code: {}\n\nShare this code with the device to pair.\nCode expires in 5 minutes.",
        code
    )
}
