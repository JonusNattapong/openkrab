//! Wizard types — shared type definitions for the onboarding wizard.
//! Ported from `openkrab/src/wizard/onboarding.types.ts`

use serde::{Deserialize, Serialize};

/// Wizard flow mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WizardFlow {
    /// Minimal prompts, sensible defaults.
    Quickstart,
    /// Full control over every setting.
    Advanced,
}

impl std::fmt::Display for WizardFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WizardFlow::Quickstart => write!(f, "quickstart"),
            WizardFlow::Advanced => write!(f, "advanced"),
        }
    }
}

/// Gateway authentication mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewayAuthChoice {
    Token,
    Password,
}

/// Gateway bind mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewayBindMode {
    Loopback,
    Lan,
    Auto,
    Custom,
    Tailnet,
}

impl std::fmt::Display for GatewayBindMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayBindMode::Loopback => write!(f, "Loopback (127.0.0.1)"),
            GatewayBindMode::Lan => write!(f, "LAN (0.0.0.0)"),
            GatewayBindMode::Auto => write!(f, "Auto"),
            GatewayBindMode::Custom => write!(f, "Custom IP"),
            GatewayBindMode::Tailnet => write!(f, "Tailnet (Tailscale IP)"),
        }
    }
}

/// Tailscale exposure mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TailscaleMode {
    Off,
    Serve,
    Funnel,
}

impl std::fmt::Display for TailscaleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TailscaleMode::Off => write!(f, "Off"),
            TailscaleMode::Serve => write!(f, "Serve"),
            TailscaleMode::Funnel => write!(f, "Funnel"),
        }
    }
}

/// Quickstart gateway defaults — resolved from existing config (if any).
#[derive(Debug, Clone)]
pub struct QuickstartGatewayDefaults {
    pub has_existing: bool,
    pub port: u16,
    pub bind: GatewayBindMode,
    pub auth_mode: GatewayAuthChoice,
    pub tailscale_mode: TailscaleMode,
    pub token: Option<String>,
    pub password: Option<String>,
    pub custom_bind_host: Option<String>,
    pub tailscale_reset_on_exit: bool,
}

impl Default for QuickstartGatewayDefaults {
    fn default() -> Self {
        Self {
            has_existing: false,
            port: 4120,
            bind: GatewayBindMode::Loopback,
            auth_mode: GatewayAuthChoice::Token,
            tailscale_mode: TailscaleMode::Off,
            token: None,
            password: None,
            custom_bind_host: None,
            tailscale_reset_on_exit: false,
        }
    }
}

/// Settings produced by the gateway configuration step of the wizard.
#[derive(Debug, Clone)]
pub struct GatewayWizardSettings {
    pub port: u16,
    pub bind: GatewayBindMode,
    pub custom_bind_host: Option<String>,
    pub auth_mode: GatewayAuthChoice,
    pub gateway_token: Option<String>,
    pub tailscale_mode: TailscaleMode,
    pub tailscale_reset_on_exit: bool,
}

/// Onboarding mode — local or remote gateway.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnboardMode {
    Local,
    Remote,
}

/// Reset scope when user chooses to reset during onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResetScope {
    /// Config file only.
    Config,
    /// Config + credentials + sessions.
    ConfigCredsAndSessions,
    /// Full reset including workspace.
    Full,
}

/// Options passed to the onboarding wizard.
#[derive(Debug, Clone, Default)]
pub struct OnboardOptions {
    pub mode: Option<OnboardMode>,
    pub flow: Option<WizardFlow>,
    pub accept_risk: bool,
    pub workspace: Option<String>,
    pub auth_choice: Option<String>,
    pub token_provider: Option<String>,
    pub token: Option<String>,
    pub skip_channels: bool,
    pub skip_providers: bool,
    pub skip_skills: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wizard_flow_display() {
        assert_eq!(WizardFlow::Quickstart.to_string(), "quickstart");
        assert_eq!(WizardFlow::Advanced.to_string(), "advanced");
    }

    #[test]
    fn gateway_bind_display() {
        assert_eq!(
            GatewayBindMode::Loopback.to_string(),
            "Loopback (127.0.0.1)"
        );
        assert_eq!(GatewayBindMode::Lan.to_string(), "LAN (0.0.0.0)");
    }

    #[test]
    fn defaults_are_sensible() {
        let defaults = QuickstartGatewayDefaults::default();
        assert_eq!(defaults.port, 4120);
        assert_eq!(defaults.bind, GatewayBindMode::Loopback);
        assert_eq!(defaults.auth_mode, GatewayAuthChoice::Token);
        assert_eq!(defaults.tailscale_mode, TailscaleMode::Off);
        assert!(!defaults.has_existing);
    }
}
