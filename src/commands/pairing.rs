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
    let devices = vec![PairingInfo {
        device_id: "demo-device-1".to_string(),
        device_name: "iPhone".to_string(),
        status: "paired".to_string(),
        paired_at: Some("2024-01-15T10:30:00Z".to_string()),
    }];

    let mut output = String::from("Paired Devices:\n");
    output.push_str("=================\n\n");

    for device in devices {
        output.push_str(&format!(
            "{} ({})\n   Status: {}\n   Paired: {}\n\n",
            device.device_name,
            device.device_id,
            device.status,
            device.paired_at.unwrap_or_else(|| "N/A".to_string())
        ));
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
    format!("Revoking pairing for device: {}", device_id)
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
