//! Finalize step of the onboarding wizard.
//! Ported from `openkrab/src/wizard/onboarding.finalize.ts`

use anyhow::Result;
use std::path::Path;

use super::prompts::WizardPrompter;
use super::types::{GatewayWizardSettings, OnboardMode, OnboardOptions, WizardFlow};

/// Options for the finalization step.
pub struct FinalizeOnboardingOptions<'a> {
    pub flow: WizardFlow,
    pub opts: &'a OnboardOptions,
    pub workspace_dir: &'a str,
    pub settings: &'a GatewayWizardSettings,
}

/// Result of the finalization step.
pub struct FinalizeResult {
    pub launched_tui: bool,
}

/// Finalize the onboarding wizard:
/// - Ensure workspace directory exists
/// - Write config file
/// - Display summary and next steps
/// - Optionally launch TUI
pub async fn finalize_onboarding_wizard(
    options: &FinalizeOnboardingOptions<'_>,
    prompter: &dyn WizardPrompter,
) -> Result<FinalizeResult> {
    // Ensure workspace directory
    ensure_workspace_dir(options.workspace_dir)?;

    // Display summary
    let summary = build_summary(options);
    prompter.note(&summary, Some("Setup complete")).await?;

    // Display next steps
    let next_steps = build_next_steps(options);
    prompter.note(&next_steps, Some("Next steps")).await?;

    // In quickstart mode, offer to launch TUI
    let launched_tui = if options.flow == WizardFlow::Quickstart {
        prompter
            .note(
                "Run `krabkrab` to start the gateway and TUI.",
                Some("Ready"),
            )
            .await?;
        false // Actual TUI launch is handled by the caller
    } else {
        false
    };

    prompter.outro("Onboarding complete! 🦀").await?;

    Ok(FinalizeResult { launched_tui })
}

/// Ensure the workspace directory exists, creating it if needed.
fn ensure_workspace_dir(workspace_dir: &str) -> Result<()> {
    let expanded = if workspace_dir.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            workspace_dir.replacen('~', &home.display().to_string(), 1)
        } else {
            workspace_dir.to_string()
        }
    } else {
        workspace_dir.to_string()
    };

    let path = Path::new(&expanded);
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}

/// Build a human-readable summary of the onboarding configuration.
fn build_summary(options: &FinalizeOnboardingOptions<'_>) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "Mode: {}",
        match options.opts.mode {
            Some(OnboardMode::Local) => "Local gateway",
            Some(OnboardMode::Remote) => "Remote gateway",
            None => "Local gateway",
        }
    ));
    lines.push(format!("Flow: {}", options.flow));
    lines.push(format!("Workspace: {}", options.workspace_dir));
    lines.push(format!("Gateway port: {}", options.settings.port));
    lines.push(format!("Gateway bind: {}", options.settings.bind));
    lines.push(format!("Auth mode: {:?}", options.settings.auth_mode));
    lines.push(format!("Tailscale: {}", options.settings.tailscale_mode));

    if let Some(ref token) = options.settings.gateway_token {
        let masked = if token.len() > 8 {
            format!("{}...{}", &token[..4], &token[token.len() - 4..])
        } else {
            "****".to_string()
        };
        lines.push(format!("Gateway token: {}", masked));
    }

    lines.join("\n")
}

/// Build next-steps instructions.
fn build_next_steps(options: &FinalizeOnboardingOptions<'_>) -> String {
    let mut lines = Vec::new();

    lines.push("1. Start the gateway:".to_string());
    lines.push("   krabkrab".to_string());
    lines.push("".to_string());

    lines.push("2. Connect from a client:".to_string());
    lines.push(format!("   ws://127.0.0.1:{}", options.settings.port));
    lines.push("".to_string());

    lines.push("3. Configure channels:".to_string());
    lines.push("   krabkrab configure".to_string());
    lines.push("".to_string());

    lines.push("4. Run security audit:".to_string());
    lines.push("   krabkrab security audit --deep".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wizard::types::{GatewayAuthChoice, GatewayBindMode, TailscaleMode};

    #[test]
    fn build_summary_includes_port() {
        let opts = OnboardOptions::default();
        let settings = GatewayWizardSettings {
            port: 4120,
            bind: GatewayBindMode::Loopback,
            custom_bind_host: None,
            auth_mode: GatewayAuthChoice::Token,
            gateway_token: Some("abcd1234efgh5678".to_string()),
            tailscale_mode: TailscaleMode::Off,
            tailscale_reset_on_exit: false,
        };
        let finalize_opts = FinalizeOnboardingOptions {
            flow: WizardFlow::Quickstart,
            opts: &opts,
            workspace_dir: "~/.openkrab/workspace",
            settings: &settings,
        };
        let summary = build_summary(&finalize_opts);
        assert!(summary.contains("4120"));
        assert!(summary.contains("Loopback"));
    }

    #[test]
    fn next_steps_includes_commands() {
        let opts = OnboardOptions::default();
        let settings = GatewayWizardSettings {
            port: 4120,
            bind: GatewayBindMode::Loopback,
            custom_bind_host: None,
            auth_mode: GatewayAuthChoice::Token,
            gateway_token: None,
            tailscale_mode: TailscaleMode::Off,
            tailscale_reset_on_exit: false,
        };
        let finalize_opts = FinalizeOnboardingOptions {
            flow: WizardFlow::Quickstart,
            opts: &opts,
            workspace_dir: "~/.openkrab/workspace",
            settings: &settings,
        };
        let steps = build_next_steps(&finalize_opts);
        assert!(steps.contains("krabkrab"));
        assert!(steps.contains("security audit"));
    }
}
