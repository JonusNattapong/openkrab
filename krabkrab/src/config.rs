use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub profile: String,
    pub log_level: String,
    pub enable_telegram: bool,
    pub enable_slack: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            log_level: "info".to_string(),
            enable_telegram: true,
            enable_slack: true,
        }
    }
}

pub fn validate_config(cfg: &AppConfig) -> Result<(), String> {
    if cfg.profile.trim().is_empty() {
        return Err("profile must not be empty".to_string());
    }
    if cfg.log_level.trim().is_empty() {
        return Err("log_level must not be empty".to_string());
    }
    Ok(())
}

