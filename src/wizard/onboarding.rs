//! Onboarding wizard — the main flow that guides users through first-time setup.
//! Ported from `openkrab/src/wizard/onboarding.ts`

use anyhow::Result;
use std::sync::Arc;

use super::prompts::{
    WizardCancelledError, WizardConfirmParams, WizardPrompter,
    WizardSelectOption, WizardSelectParams, WizardTextParams,
};
use super::types::{
    GatewayAuthChoice, GatewayBindMode, OnboardMode, OnboardOptions,
    QuickstartGatewayDefaults, ResetScope, TailscaleMode, WizardFlow,
};

/// Default gateway port.
pub const DEFAULT_GATEWAY_PORT: u16 = 4120;

/// Default workspace directory.
pub const DEFAULT_WORKSPACE: &str = "~/.openkrab/workspace";

/// Run the risk acknowledgement step.
async fn require_risk_acknowledgement(
    prompter: &dyn WizardPrompter,
    accept_risk: bool,
) -> Result<()> {
    if accept_risk {
        return Ok(());
    }

    prompter
        .note(
            &[
                "Security warning — please read.",
                "",
                "OpenKrab is a hobby project and still in beta. Expect sharp edges.",
                "This bot can read files and run actions if tools are enabled.",
                "A bad prompt can trick it into doing unsafe things.",
                "",
                "If you're not comfortable with basic security and access control,",
                "don't run OpenKrab publicly. Ask someone experienced to help before",
                "enabling tools or exposing it to the internet.",
                "",
                "Recommended baseline:",
                "- Pairing/allowlists + mention gating.",
                "- Sandbox + least-privilege tools.",
                "- Keep secrets out of the agent's reachable filesystem.",
                "- Use the strongest available model for any bot with tools.",
                "",
                "Run regularly:",
                "  krabkrab security audit --deep",
                "  krabkrab security audit --fix",
            ]
            .join("\n"),
            Some("Security"),
        )
        .await?;

    let ok = prompter
        .confirm(WizardConfirmParams {
            message: "I understand this is powerful and inherently risky. Continue?".to_string(),
            initial_value: Some(false),
        })
        .await?;

    if !ok {
        return Err(WizardCancelledError::new("risk not accepted").into());
    }

    Ok(())
}

/// Summarize existing config for user review.
fn summarize_existing_config(config: &serde_json::Value) -> String {
    let mut lines = Vec::new();
    lines.push("Current config summary:".to_string());

    if let Some(gateway) = config.get("gateway") {
        if let Some(port) = gateway.get("port").and_then(|v| v.as_u64()) {
            lines.push(format!("  Gateway port: {}", port));
        }
        if let Some(bind) = gateway.get("bind").and_then(|v| v.as_str()) {
            lines.push(format!("  Gateway bind: {}", bind));
        }
        if let Some(auth) = gateway.get("auth") {
            if let Some(mode) = auth.get("mode").and_then(|v| v.as_str()) {
                lines.push(format!("  Auth mode: {}", mode));
            }
        }
    }

    if let Some(agents) = config.get("agents") {
        if let Some(defaults) = agents.get("defaults") {
            if let Some(workspace) = defaults.get("workspace").and_then(|v| v.as_str()) {
                lines.push(format!("  Workspace: {}", workspace));
            }
            if let Some(model) = defaults.get("model").and_then(|v| v.as_str()) {
                lines.push(format!("  Default model: {}", model));
            }
        }
    }

    if lines.len() == 1 {
        lines.push("  (no settings detected)".to_string());
    }

    lines.join("\n")
}

/// The main onboarding wizard flow.
pub async fn run_onboarding_wizard(
    opts: &OnboardOptions,
    prompter: &dyn WizardPrompter,
) -> Result<OnboardingResult> {
    prompter
        .intro("OpenKrab onboarding")
        .await?;

    // Risk acknowledgement
    require_risk_acknowledgement(prompter, opts.accept_risk).await?;

    // Check existing config
    let config_path = resolve_config_path();
    let existing_config = load_existing_config(&config_path);
    let has_existing = existing_config.is_some();

    let base_config = existing_config.unwrap_or_else(|| serde_json::json!({}));

    // Choose flow
    let flow = match opts.flow {
        Some(f) => f,
        None => {
            let choice = prompter
                .select(WizardSelectParams {
                    message: "Onboarding mode".to_string(),
                    options: vec![
                        WizardSelectOption {
                            value: "quickstart".to_string(),
                            label: "QuickStart".to_string(),
                            hint: Some("Configure details later via `krabkrab configure`.".to_string()),
                        },
                        WizardSelectOption {
                            value: "advanced".to_string(),
                            label: "Manual".to_string(),
                            hint: Some("Configure port, network, Tailscale, and auth options.".to_string()),
                        },
                    ],
                    initial_value: Some("quickstart".to_string()),
                })
                .await?;

            match choice.as_str() {
                "advanced" => WizardFlow::Advanced,
                _ => WizardFlow::Quickstart,
            }
        }
    };

    // If existing config detected, let user choose what to do
    if has_existing {
        prompter
            .note(
                &summarize_existing_config(&base_config),
                Some("Existing config detected"),
            )
            .await?;

        let action = prompter
            .select(WizardSelectParams {
                message: "Config handling".to_string(),
                options: vec![
                    WizardSelectOption {
                        value: "keep".to_string(),
                        label: "Use existing values".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "modify".to_string(),
                        label: "Update values".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "reset".to_string(),
                        label: "Reset".to_string(),
                        hint: None,
                    },
                ],
                initial_value: None,
            })
            .await?;

        if action == "reset" {
            let _reset_scope = prompter
                .select(WizardSelectParams {
                    message: "Reset scope".to_string(),
                    options: vec![
                        WizardSelectOption {
                            value: "config".to_string(),
                            label: "Config only".to_string(),
                            hint: None,
                        },
                        WizardSelectOption {
                            value: "config+creds+sessions".to_string(),
                            label: "Config + creds + sessions".to_string(),
                            hint: None,
                        },
                        WizardSelectOption {
                            value: "full".to_string(),
                            label: "Full reset (config + creds + sessions + workspace)".to_string(),
                            hint: None,
                        },
                    ],
                    initial_value: None,
                })
                .await?;
            // TODO: Actually perform the reset
        }
    }

    // Determine onboard mode
    let mode = match opts.mode {
        Some(m) => m,
        None if flow == WizardFlow::Quickstart => OnboardMode::Local,
        None => {
            let choice = prompter
                .select(WizardSelectParams {
                    message: "What do you want to set up?".to_string(),
                    options: vec![
                        WizardSelectOption {
                            value: "local".to_string(),
                            label: "Local gateway (this machine)".to_string(),
                            hint: None,
                        },
                        WizardSelectOption {
                            value: "remote".to_string(),
                            label: "Remote gateway (info-only)".to_string(),
                            hint: None,
                        },
                    ],
                    initial_value: None,
                })
                .await?;

            match choice.as_str() {
                "remote" => OnboardMode::Remote,
                _ => OnboardMode::Local,
            }
        }
    };

    // Workspace setup
    let workspace = match &opts.workspace {
        Some(w) => w.clone(),
        None if flow == WizardFlow::Quickstart => DEFAULT_WORKSPACE.to_string(),
        None => {
            prompter
                .text(WizardTextParams {
                    message: "Workspace directory".to_string(),
                    initial_value: Some(DEFAULT_WORKSPACE.to_string()),
                    placeholder: None,
                })
                .await?
        }
    };

    // Gateway configuration
    let gateway_settings = if mode == OnboardMode::Local {
        Some(
            super::gateway_config::configure_gateway(flow, &base_config, prompter).await?,
        )
    } else {
        None
    };

    // Quickstart summary
    if flow == WizardFlow::Quickstart {
        let settings = gateway_settings.as_ref().map(|s| {
            format!(
                "Gateway port: {}\nGateway bind: {}\nGateway auth: {}\nTailscale: {}",
                s.port,
                s.bind,
                match s.auth_mode {
                    GatewayAuthChoice::Token => "Token (default)",
                    GatewayAuthChoice::Password => "Password",
                },
                s.tailscale_mode,
            )
        }).unwrap_or_else(|| "Remote mode — no local gateway.".to_string());

        prompter
            .note(&settings, Some("Configuration"))
            .await?;
    }

    prompter
        .outro("Onboarding complete! Run `krabkrab` to start.")
        .await?;

    Ok(OnboardingResult {
        flow,
        mode,
        workspace,
        gateway_settings,
    })
}

/// Result of the onboarding wizard.
#[derive(Debug)]
pub struct OnboardingResult {
    pub flow: WizardFlow,
    pub mode: OnboardMode,
    pub workspace: String,
    pub gateway_settings: Option<super::types::GatewayWizardSettings>,
}

/// Resolve the config file path.
fn resolve_config_path() -> String {
    if let Ok(path) = std::env::var("OPENKRAB_CONFIG") {
        return path;
    }

    let home = dirs::home_dir()
        .map(|h| h.display().to_string())
        .unwrap_or_else(|| ".".to_string());

    format!("{}/.openkrab/config.yaml", home)
}

/// Load existing config if it exists.
fn load_existing_config(path: &str) -> Option<serde_json::Value> {
    let content = std::fs::read_to_string(path).ok()?;
    // Try JSON first, then try YAML-like parsing as JSON
    serde_json::from_str(&content).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_empty_config() {
        let config = serde_json::json!({});
        let summary = summarize_existing_config(&config);
        assert!(summary.contains("no settings detected"));
    }

    #[test]
    fn summarize_gateway_config() {
        let config = serde_json::json!({
            "gateway": {
                "port": 4120,
                "bind": "loopback",
                "auth": { "mode": "token" }
            }
        });
        let summary = summarize_existing_config(&config);
        assert!(summary.contains("4120"));
        assert!(summary.contains("loopback"));
        assert!(summary.contains("token"));
    }

    #[test]
    fn default_config_path_uses_home() {
        std::env::remove_var("OPENKRAB_CONFIG");
        let path = resolve_config_path();
        assert!(path.ends_with(".openkrab/config.yaml"));
    }
}
