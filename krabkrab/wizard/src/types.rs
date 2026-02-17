#[derive(Debug, Clone, Copy)]
pub enum WizardFlow {
    Quickstart,
    Advanced,
}

#[derive(Debug, Clone)]
pub struct GatewayWizardSettings {
    pub host: String,
    pub port: u16,
    pub auth_required: bool,
}

#[derive(Debug, Clone)]
pub struct OnboardingResult {
    pub flow: WizardFlow,
    pub profile_name: String,
    pub gateway: GatewayWizardSettings,
}
