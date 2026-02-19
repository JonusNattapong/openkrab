use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeLayer {
    Rust,
    Js,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureRoute {
    pub primary: RuntimeLayer,
    pub fallback: Option<RuntimeLayer>,
}

impl FeatureRoute {
    pub const fn rust_only() -> Self {
        Self {
            primary: RuntimeLayer::Rust,
            fallback: None,
        }
    }

    pub const fn js_only() -> Self {
        Self {
            primary: RuntimeLayer::Js,
            fallback: None,
        }
    }

    pub const fn rust_then_js() -> Self {
        Self {
            primary: RuntimeLayer::Rust,
            fallback: Some(RuntimeLayer::Js),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeatureMatrix {
    pub browser_automation: FeatureRoute,
    pub canvas_host: FeatureRoute,
    pub voice_wake_talk: FeatureRoute,
    pub macos_native: FeatureRoute,
    pub node_host: FeatureRoute,
    pub imessage_native: FeatureRoute,
    pub whatsapp_full: FeatureRoute,
    pub line_full: FeatureRoute,
}

impl Default for FeatureMatrix {
    fn default() -> Self {
        Self {
            browser_automation: FeatureRoute::js_only(),
            canvas_host: FeatureRoute::rust_then_js(),
            voice_wake_talk: FeatureRoute::rust_then_js(),
            macos_native: FeatureRoute::js_only(),
            node_host: FeatureRoute::rust_then_js(),
            imessage_native: FeatureRoute::js_only(),
            whatsapp_full: FeatureRoute::rust_then_js(),
            line_full: FeatureRoute::rust_then_js(),
        }
    }
}

impl FeatureMatrix {
    pub fn route_for(&self, feature: &str) -> Option<&FeatureRoute> {
        match feature {
            "browser" | "browser_automation" => Some(&self.browser_automation),
            "canvas" | "canvas_host" => Some(&self.canvas_host),
            "voice" | "voice_wake_talk" | "wake_talk" => Some(&self.voice_wake_talk),
            "macos" | "macos_native" => Some(&self.macos_native),
            "node_host" | "node" => Some(&self.node_host),
            "imessage" | "imessage_native" => Some(&self.imessage_native),
            "whatsapp" | "whatsapp_full" => Some(&self.whatsapp_full),
            "line" | "line_full" => Some(&self.line_full),
            _ => None,
        }
    }
}

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
    pub feature_matrix: FeatureMatrix,
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
            feature_matrix: FeatureMatrix::default(),
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
    let all = [
        &cfg.feature_matrix.browser_automation,
        &cfg.feature_matrix.canvas_host,
        &cfg.feature_matrix.voice_wake_talk,
        &cfg.feature_matrix.macos_native,
        &cfg.feature_matrix.node_host,
        &cfg.feature_matrix.imessage_native,
        &cfg.feature_matrix.whatsapp_full,
        &cfg.feature_matrix.line_full,
    ];
    for route in all {
        if route.fallback == Some(route.primary) {
            return Err("feature_matrix fallback must differ from primary".to_string());
        }
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
    fn feature_matrix_defaults_to_js_for_non_ported_areas() {
        let cfg = AppConfig::default();
        assert_eq!(
            cfg.feature_matrix.browser_automation.primary,
            RuntimeLayer::Js
        );
        assert_eq!(cfg.feature_matrix.canvas_host.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.voice_wake_talk.primary,
            RuntimeLayer::Rust
        );
        assert_eq!(cfg.feature_matrix.node_host.primary, RuntimeLayer::Rust);
        assert_eq!(cfg.feature_matrix.whatsapp_full.primary, RuntimeLayer::Rust);
        assert_eq!(
            cfg.feature_matrix.whatsapp_full.fallback,
            Some(RuntimeLayer::Js)
        );
    }

    #[test]
    fn invalid_matrix_rejects_same_primary_and_fallback() {
        let mut cfg = AppConfig::default();
        cfg.feature_matrix.browser_automation.fallback = Some(RuntimeLayer::Js);
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn empty_profile_is_invalid() {
        let mut cfg = AppConfig::default();
        cfg.profile = "   ".to_string();
        assert!(validate_config(&cfg).is_err());
    }
}
