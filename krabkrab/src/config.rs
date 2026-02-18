use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub profile: String,
    pub log_level: String,
    // ── Connector toggles ────────────────────────────────────────────────────
    pub enable_telegram: bool,
    pub enable_slack: bool,
    pub enable_discord: bool,
    pub enable_line: bool,
    pub enable_whatsapp: bool,
    // ── Web Dashboard ────────────────────────────────────────────────────────
    pub enable_dashboard: bool,
    pub dashboard_bind: String,
    // ── Sub-configs ──────────────────────────────────────────────────────────
    pub memory: crate::memory::config::MemoryConfig,
    pub agent: crate::agents::AgentIdentity,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
            log_level: "info".to_string(),
            enable_telegram: true,
            enable_slack: true,
            enable_discord: true,
            enable_line: true,
            enable_whatsapp: true,
            enable_dashboard: true,
            dashboard_bind: "0.0.0.0:3000".to_string(),
            memory: crate::memory::config::MemoryConfig::default(),
            agent: crate::agents::AgentIdentity::default(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = AppConfig::default();
        assert!(validate_config(&cfg).is_ok());
    }

    #[test]
    fn default_has_all_connectors_enabled() {
        let cfg = AppConfig::default();
        assert!(cfg.enable_telegram);
        assert!(cfg.enable_slack);
        assert!(cfg.enable_discord);
        assert!(cfg.enable_line);
        assert!(cfg.enable_whatsapp);
        assert!(cfg.enable_dashboard);
    }

    #[test]
    fn empty_profile_is_invalid() {
        let mut cfg = AppConfig::default();
        cfg.profile = "   ".to_string();
        assert!(validate_config(&cfg).is_err());
    }
}
