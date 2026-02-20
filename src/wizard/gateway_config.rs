//! Gateway configuration step of the onboarding wizard.
//! Ported from `openkrab/src/wizard/onboarding.gateway-config.ts`

use anyhow::Result;
use rand::Rng;

use super::prompts::{
    WizardConfirmParams, WizardPrompter, WizardSelectOption, WizardSelectParams,
    WizardTextParams,
};
use super::types::{
    GatewayAuthChoice, GatewayBindMode, GatewayWizardSettings, QuickstartGatewayDefaults,
    TailscaleMode, WizardFlow,
};

/// Default port for the gateway.
const DEFAULT_GATEWAY_PORT: u16 = 4120;

/// High-risk node commands that are denied by default on fresh installs.
#[allow(dead_code)]
const DEFAULT_DANGEROUS_NODE_DENY_COMMANDS: &[&str] = &[
    "camera.snap",
    "camera.clip",
    "screen.record",
    "calendar.add",
    "contacts.add",
    "reminders.add",
];

/// Generate a random auth token.
pub fn random_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
    hex::encode(bytes)
}

/// Validate a gateway password input.
pub fn validate_gateway_password(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    if trimmed.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    Ok(())
}

/// Validate an IPv4 address input.
pub fn validate_ipv4_address(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if trimmed.parse::<std::net::Ipv4Addr>().is_err() {
        return Err("Invalid IPv4 address".to_string());
    }
    Ok(())
}

/// Configure the gateway settings for onboarding.
pub async fn configure_gateway(
    flow: WizardFlow,
    _base_config: &serde_json::Value,
    prompter: &dyn WizardPrompter,
) -> Result<GatewayWizardSettings> {
    let defaults = QuickstartGatewayDefaults::default();

    // Port
    let port = if flow == WizardFlow::Quickstart {
        defaults.port
    } else {
        let input = prompter
            .text(WizardTextParams {
                message: "Gateway port".to_string(),
                initial_value: Some(DEFAULT_GATEWAY_PORT.to_string()),
                placeholder: None,
            })
            .await?;
        input.trim().parse::<u16>().unwrap_or(DEFAULT_GATEWAY_PORT)
    };

    // Bind mode
    let mut bind = if flow == WizardFlow::Quickstart {
        defaults.bind
    } else {
        let choice = prompter
            .select(WizardSelectParams {
                message: "Gateway bind".to_string(),
                options: vec![
                    WizardSelectOption {
                        value: "loopback".to_string(),
                        label: "Loopback (127.0.0.1)".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "lan".to_string(),
                        label: "LAN (0.0.0.0)".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "tailnet".to_string(),
                        label: "Tailnet (Tailscale IP)".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "auto".to_string(),
                        label: "Auto (Loopback → LAN)".to_string(),
                        hint: None,
                    },
                    WizardSelectOption {
                        value: "custom".to_string(),
                        label: "Custom IP".to_string(),
                        hint: None,
                    },
                ],
                initial_value: None,
            })
            .await?;

        match choice.as_str() {
            "lan" => GatewayBindMode::Lan,
            "tailnet" => GatewayBindMode::Tailnet,
            "auto" => GatewayBindMode::Auto,
            "custom" => GatewayBindMode::Custom,
            _ => GatewayBindMode::Loopback,
        }
    };

    // Custom bind host
    let mut custom_bind_host = defaults.custom_bind_host.clone();
    if bind == GatewayBindMode::Custom {
        let input = prompter
            .text(WizardTextParams {
                message: "Custom IP address".to_string(),
                initial_value: custom_bind_host.clone(),
                placeholder: Some("192.168.1.100".to_string()),
            })
            .await?;
        let trimmed = input.trim().to_string();
        if let Err(e) = validate_ipv4_address(&trimmed) {
            prompter.note(&e, Some("Warning")).await?;
        }
        custom_bind_host = Some(trimmed);
    }

    // Auth mode
    let mut auth_mode = if flow == WizardFlow::Quickstart {
        defaults.auth_mode
    } else {
        let choice = prompter
            .select(WizardSelectParams {
                message: "Gateway auth".to_string(),
                options: vec![
                    WizardSelectOption {
                        value: "token".to_string(),
                        label: "Token".to_string(),
                        hint: Some("Recommended default (local + remote)".to_string()),
                    },
                    WizardSelectOption {
                        value: "password".to_string(),
                        label: "Password".to_string(),
                        hint: None,
                    },
                ],
                initial_value: Some("token".to_string()),
            })
            .await?;

        match choice.as_str() {
            "password" => GatewayAuthChoice::Password,
            _ => GatewayAuthChoice::Token,
        }
    };

    // Tailscale mode
    let tailscale_mode = if flow == WizardFlow::Quickstart {
        defaults.tailscale_mode
    } else {
        let choice = prompter
            .select(WizardSelectParams {
                message: "Tailscale exposure".to_string(),
                options: vec![
                    WizardSelectOption {
                        value: "off".to_string(),
                        label: "Off".to_string(),
                        hint: Some("No Tailscale integration".to_string()),
                    },
                    WizardSelectOption {
                        value: "serve".to_string(),
                        label: "Serve".to_string(),
                        hint: Some("Expose to tailnet only".to_string()),
                    },
                    WizardSelectOption {
                        value: "funnel".to_string(),
                        label: "Funnel".to_string(),
                        hint: Some("Expose publicly via Tailscale Funnel".to_string()),
                    },
                ],
                initial_value: Some("off".to_string()),
            })
            .await?;

        match choice.as_str() {
            "serve" => TailscaleMode::Serve,
            "funnel" => TailscaleMode::Funnel,
            _ => TailscaleMode::Off,
        }
    };

    // Tailscale safety constraints:
    // - Tailscale wants bind=loopback
    // - Funnel requires password auth
    if tailscale_mode != TailscaleMode::Off && bind != GatewayBindMode::Loopback {
        prompter
            .note(
                "Tailscale requires bind=loopback. Adjusting bind to loopback.",
                Some("Note"),
            )
            .await?;
        bind = GatewayBindMode::Loopback;
        custom_bind_host = None;
    }

    if tailscale_mode == TailscaleMode::Funnel && auth_mode != GatewayAuthChoice::Password {
        prompter
            .note("Tailscale funnel requires password auth.", Some("Note"))
            .await?;
        auth_mode = GatewayAuthChoice::Password;
    }

    // Reset on exit
    let tailscale_reset_on_exit = if tailscale_mode != TailscaleMode::Off && flow != WizardFlow::Quickstart {
        prompter
            .confirm(WizardConfirmParams {
                message: "Reset Tailscale serve/funnel on exit?".to_string(),
                initial_value: Some(false),
            })
            .await?
    } else {
        false
    };

    // Generate or prompt for token/password
    let gateway_token = if auth_mode == GatewayAuthChoice::Token {
        if flow == WizardFlow::Quickstart {
            Some(defaults.token.unwrap_or_else(random_token))
        } else {
            let input = prompter
                .text(WizardTextParams {
                    message: "Gateway token (blank to generate)".to_string(),
                    initial_value: defaults.token.clone(),
                    placeholder: Some("Needed for multi-machine or non-loopback access".to_string()),
                })
                .await?;
            let trimmed = input.trim().to_string();
            if trimmed.is_empty() {
                Some(random_token())
            } else {
                Some(trimmed)
            }
        }
    } else {
        // Password mode — prompt for password
        let _password = if flow == WizardFlow::Quickstart {
            defaults.password.unwrap_or_default()
        } else {
            prompter
                .text(WizardTextParams {
                    message: "Gateway password".to_string(),
                    initial_value: None,
                    placeholder: Some("At least 8 characters".to_string()),
                })
                .await?
        };
        None
    };

    Ok(GatewayWizardSettings {
        port,
        bind,
        custom_bind_host: if bind == GatewayBindMode::Custom {
            custom_bind_host
        } else {
            None
        },
        auth_mode,
        gateway_token,
        tailscale_mode,
        tailscale_reset_on_exit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_token_is_64_hex_chars() {
        let token = random_token();
        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn validate_password_too_short() {
        assert!(validate_gateway_password("short").is_err());
        assert!(validate_gateway_password("").is_err());
        assert!(validate_gateway_password("long_enough_password").is_ok());
    }

    #[test]
    fn validate_ipv4_valid() {
        assert!(validate_ipv4_address("192.168.1.100").is_ok());
        assert!(validate_ipv4_address("127.0.0.1").is_ok());
        assert!(validate_ipv4_address("not-an-ip").is_err());
    }

    #[test]
    fn deny_commands_are_present() {
        assert!(DEFAULT_DANGEROUS_NODE_DENY_COMMANDS.contains(&"camera.snap"));
        assert!(DEFAULT_DANGEROUS_NODE_DENY_COMMANDS.contains(&"screen.record"));
    }
}
