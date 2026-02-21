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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CronConfig {
    #[serde(default)]
    pub jobs: Vec<CronJob>,
}

pub fn cron_add_command(schedule: &str, command: &str) -> String {
    let cron_path = get_cron_path();
    if let Some(parent) = cron_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut config: CronConfig = if cron_path.exists() {
        fs::read_to_string(&cron_path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        CronConfig::default()
    };

    let id = uuid::Uuid::new_v4().to_string();
    config.jobs.push(CronJob {
        id: id.clone(),
        schedule: schedule.to_string(),
        command: command.to_string(),
        enabled: true,
        last_run: None,
        next_run: None,
    });

    if let Ok(content) = toml::to_string(&config) {
        if fs::write(&cron_path, &content).is_ok() {
            return format!("Added cron job [{}]: {} -> {}", id, schedule, command);
        }
    }
    "Failed to add cron job".to_string()
}

pub fn cron_remove_command(id: &str) -> String {
    let cron_path = get_cron_path();
    let mut config: CronConfig = fs::read_to_string(&cron_path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();

    let initial_len = config.jobs.len();
    config.jobs.retain(|j| j.id != id);

    if config.jobs.len() < initial_len {
        if let Ok(content) = toml::to_string(&config) {
            let _ = fs::write(&cron_path, &content);
            return format!("Removed cron job: {}", id);
        }
    }
    format!("Cron job not found: {}", id)
}

pub fn cron_enable_command(id: &str) -> String {
    let cron_path = get_cron_path();
    let mut config: CronConfig = fs::read_to_string(&cron_path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();

    for job in &mut config.jobs {
        if job.id == id {
            job.enabled = true;
            if let Ok(content) = toml::to_string(&config) {
                let _ = fs::write(&cron_path, &content);
                return format!("Enabled cron job: {}", id);
            }
            return "Failed to save cron configuration".to_string();
        }
    }
    format!("Cron job not found: {}", id)
}

pub fn cron_disable_command(id: &str) -> String {
    let cron_path = get_cron_path();
    let mut config: CronConfig = fs::read_to_string(&cron_path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();

    for job in &mut config.jobs {
        if job.id == id {
            job.enabled = false;
            if let Ok(content) = toml::to_string(&config) {
                let _ = fs::write(&cron_path, &content);
                return format!("Disabled cron job: {}", id);
            }
            return "Failed to save cron configuration".to_string();
        }
    }
    format!("Cron job not found: {}", id)
}
