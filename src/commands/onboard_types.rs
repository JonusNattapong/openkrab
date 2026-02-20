//! onboard_types — Types for onboard command.
//! Ported from `openkrab/src/commands/onboard-types.ts` (Phase 6).

use serde::{Deserialize, Serialize};

/// Onboarding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingConfig {
    pub mode: OnboardMode,
    pub channels: Vec<ChannelConfig>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub workspace_dir: Option<String>,
    pub enable_memory: bool,
    pub enable_sandbox: bool,
}

impl Default for OnboardingConfig {
    fn default() -> Self {
        Self {
            mode: OnboardMode::Interactive,
            channels: vec![],
            provider: None,
            model: None,
            workspace_dir: None,
            enable_memory: true,
            enable_sandbox: false,
        }
    }
}

/// Onboarding mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OnboardMode {
    Interactive,
    NonInteractive,
    Remote,
}

/// Channel configuration for onboarding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub channel_type: String,
    pub account_id: String,
    pub token: Option<String>,
    pub enabled: bool,
}

/// Onboarding result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardResult {
    pub success: bool,
    pub config_path: Option<String>,
    pub messages: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Wizard step for interactive onboarding.
#[derive(Debug, Clone)]
pub enum WizardStep {
    Welcome,
    SelectChannels,
    ConfigureChannel { channel_type: String },
    SelectProvider,
    ConfigureAuth,
    SelectModel,
    ConfigureMemory,
    ConfigureSandbox,
    Review,
    Complete,
}

/// Onboarding wizard state.
#[derive(Debug, Clone)]
pub struct WizardState {
    pub current_step: WizardStep,
    pub config: OnboardingConfig,
    pub can_go_back: bool,
    pub can_continue: bool,
}

impl WizardState {
    pub fn new() -> Self {
        Self {
            current_step: WizardStep::Welcome,
            config: OnboardingConfig::default(),
            can_go_back: false,
            can_continue: true,
        }
    }

    pub fn next_step(&mut self) {
        self.current_step = match &self.current_step {
            WizardStep::Welcome => WizardStep::SelectChannels,
            WizardStep::SelectChannels => WizardStep::SelectProvider,
            WizardStep::ConfigureChannel { .. } => WizardStep::SelectChannels,
            WizardStep::SelectProvider => WizardStep::ConfigureAuth,
            WizardStep::ConfigureAuth => WizardStep::SelectModel,
            WizardStep::SelectModel => WizardStep::ConfigureMemory,
            WizardStep::ConfigureMemory => WizardStep::ConfigureSandbox,
            WizardStep::ConfigureSandbox => WizardStep::Review,
            WizardStep::Review => WizardStep::Complete,
            WizardStep::Complete => WizardStep::Complete,
        };
        self.update_navigation();
    }

    pub fn previous_step(&mut self) {
        self.current_step = match &self.current_step {
            WizardStep::Welcome => WizardStep::Welcome,
            WizardStep::SelectChannels => WizardStep::Welcome,
            WizardStep::ConfigureChannel { .. } => WizardStep::SelectChannels,
            WizardStep::SelectProvider => WizardStep::SelectChannels,
            WizardStep::ConfigureAuth => WizardStep::SelectProvider,
            WizardStep::SelectModel => WizardStep::ConfigureAuth,
            WizardStep::ConfigureMemory => WizardStep::SelectModel,
            WizardStep::ConfigureSandbox => WizardStep::ConfigureMemory,
            WizardStep::Review => WizardStep::ConfigureSandbox,
            WizardStep::Complete => WizardStep::Review,
        };
        self.update_navigation();
    }

    fn update_navigation(&mut self) {
        self.can_go_back = !matches!(
            self.current_step,
            WizardStep::Welcome | WizardStep::Complete
        );
        self.can_continue = !matches!(self.current_step, WizardStep::Complete);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_navigation() {
        let mut state = WizardState::new();
        assert!(matches!(state.current_step, WizardStep::Welcome));
        assert!(!state.can_go_back);

        state.next_step();
        assert!(matches!(state.current_step, WizardStep::SelectChannels));
        assert!(state.can_go_back);

        state.next_step();
        assert!(matches!(state.current_step, WizardStep::SelectProvider));

        state.previous_step();
        assert!(matches!(state.current_step, WizardStep::SelectChannels));
    }

    #[test]
    fn test_onboarding_config_default() {
        let config = OnboardingConfig::default();
        assert!(config.enable_memory);
        assert!(!config.enable_sandbox);
        assert!(matches!(config.mode, OnboardMode::Interactive));
    }
}
