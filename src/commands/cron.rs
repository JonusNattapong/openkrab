//! Cron command - Manage scheduled tasks

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub id: String,
    pub schedule: String,
    pub command: String,
    pub enabled: bool,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
}

fn get_cron_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("cron.toml")
}

pub fn cron_list_command() -> String {
    let cron_path = get_cron_path();

    if !cron_path.exists() {
        return r#"No cron jobs configured.
Run 'krabkrab cron add <schedule> <command>' to add one.

Example:
  krabkrab cron add "0 9 * * *" "Send daily summary"
"#
        .to_string();
    }

    match fs::read_to_string(&cron_path) {
        Ok(content) => {
            let mut output = String::from("Cron Jobs:\n");
            output.push_str("==========\n\n");
            output.push_str(&content);
            output
        }
        Err(e) => format!("Failed to read cron jobs: {}", e),
    }
}

pub fn cron_add_command(schedule: &str, command: &str) -> String {
    let cron_path = get_cron_path();

    if let Some(parent) = cron_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let job = format!(
        "[[jobs]]\nid = \"{}\"\nschedule = \"{}\"\ncommand = \"{}\"\nenabled = true\n",
        uuid::Uuid::new_v4(),
        schedule,
        command
    );

    let content = if cron_path.exists() {
        let mut existing = fs::read_to_string(&cron_path).unwrap_or_default();
        existing.push_str("\n");
        existing.push_str(&job);
        existing
    } else {
        job
    };

    match fs::write(&cron_path, &content) {
        Ok(_) => format!("Added cron job: {} -> {}", schedule, command),
        Err(e) => format!("Failed to add cron job: {}", e),
    }
}

pub fn cron_remove_command(id: &str) -> String {
    format!("Removing cron job: {} (not yet implemented)", id)
}

pub fn cron_enable_command(id: &str) -> String {
    format!("Enabling cron job: {} (not yet implemented)", id)
}

pub fn cron_disable_command(id: &str) -> String {
    format!("Disabling cron job: {} (not yet implemented)", id)
}
