use crate::prompts::WizardPrompter;
use crate::session::WizardSession;
use crate::types::OnboardingResult;
use anyhow::Result;

pub fn run_onboarding_wizard(prompter: &mut dyn WizardPrompter) -> Result<OnboardingResult> {
    let mut session = WizardSession::new(prompter);
    session.run()
}
