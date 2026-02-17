use crate::prompts::{WizardPrompter, WizardSelectOption};
use crate::types::{GatewayWizardSettings, OnboardingResult, WizardFlow};
use anyhow::Result;

pub struct WizardSession<'a> {
    prompter: &'a mut dyn WizardPrompter,
}

impl<'a> WizardSession<'a> {
    pub fn new(prompter: &'a mut dyn WizardPrompter) -> Self {
        Self { prompter }
    }

    pub fn run(&mut self) -> Result<OnboardingResult> {
        self.prompter.note(
            "KrabKrab Onboarding",
            "Welcome. This wizard prepares a starter configuration.",
        )?;

        let flow = self.prompter.select(
            "Choose flow",
            &[
                WizardSelectOption {
                    label: "Quickstart (recommended)".to_string(),
                    value: "quickstart".to_string(),
                },
                WizardSelectOption {
                    label: "Advanced".to_string(),
                    value: "advanced".to_string(),
                },
            ],
        )?;

        let flow = if flow == "advanced" {
            WizardFlow::Advanced
        } else {
            WizardFlow::Quickstart
        };

        let profile_name = self
            .prompter
            .text("Profile name", Some("default"))?
            .chars()
            .take(64)
            .collect::<String>();

        let profile_name = if profile_name.is_empty() {
            "default".to_string()
        } else {
            profile_name
        };

        let host = self.prompter.text("Gateway host", Some("127.0.0.1"))?;
        let host = if host.is_empty() {
            "127.0.0.1".to_string()
        } else {
            host
        };

        let port_raw = self.prompter.text("Gateway port", Some("18789"))?;
        let port = port_raw.parse::<u16>().unwrap_or(18789);

        let auth_required = self.prompter.confirm("Enable gateway auth?", true)?;

        Ok(OnboardingResult {
            flow,
            profile_name,
            gateway: GatewayWizardSettings {
                host,
                port,
                auth_required,
            },
        })
    }
}
