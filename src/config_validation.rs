//! Config validation — port of `openclaw/src/config/validation.ts` (Phase 1-4 schema validation)

use crate::openclaw_config::OpenClawConfig;
use anyhow::{bail, Result};
use serde_json::Value;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

/// Validation result
pub type ValidationResult<T> = Result<T, Vec<ValidationError>>;

/// Validate OpenClawConfig against schema
pub fn validate_config_schema(config: &OpenClawConfig) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Validate meta
    if let Some(meta) = &config.meta {
        if let Some(version) = &meta.last_touched_version {
            if version.trim().is_empty() {
                errors.push(ValidationError {
                    field: "meta.last_touched_version".to_string(),
                    message: "must not be empty".to_string(),
                });
            }
        }
    }

    // Validate auth profiles
    if let Some(auth) = &config.auth {
        for (profile_id, profile) in &auth.profiles {
            if profile_id.trim().is_empty() {
                errors.push(ValidationError {
                    field: format!("auth.profiles.{}", profile_id),
                    message: "profile ID must not be empty".to_string(),
                });
            }
            if profile.provider.trim().is_empty() {
                errors.push(ValidationError {
                    field: format!("auth.profiles.{}.provider", profile_id),
                    message: "provider must not be empty".to_string(),
                });
            }
        }
    }

    // Validate logging
    if let Some(logging) = &config.logging {
        if logging.level.trim().is_empty() {
            errors.push(ValidationError {
                field: "logging.level".to_string(),
                message: "must not be empty".to_string(),
            });
        }
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&logging.level.as_str()) {
            errors.push(ValidationError {
                field: "logging.level".to_string(),
                message: format!("must be one of: {}", valid_levels.join(", ")),
            });
        }
    }

    // Validate gateway
    if let Some(gateway) = &config.gateway {
        if let Some(port) = gateway.port {
            if port == 0 {
                errors.push(ValidationError {
                    field: "gateway.port".to_string(),
                    message: "must not be 0".to_string(),
                });
            }
            if port > 65535 {
                errors.push(ValidationError {
                    field: "gateway.port".to_string(),
                    message: "must be <= 65535".to_string(),
                });
            }
        }
    }

    // Validate channels
    if let Some(channels) = &config.channels {
        validate_channels_config(channels, &mut errors);
    }

    // Validate models
    if let Some(models) = &config.models {
        validate_models_config(models, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate channels configuration
fn validate_channels_config(
    channels: &crate::openclaw_config::ChannelsConfig,
    errors: &mut Vec<ValidationError>,
) {
    let channel_types = [
        ("telegram", &channels.telegram),
        ("slack", &channels.slack),
        ("discord", &channels.discord),
        ("whatsapp", &channels.whatsapp),
        ("signal", &channels.signal),
        ("imessage", &channels.imessage),
        ("irc", &channels.irc),
        ("web", &channels.web),
    ];

    for (channel_type, configs) in channel_types {
        for (name, config) in configs {
            if name.trim().is_empty() {
                errors.push(ValidationError {
                    field: format!("channels.{}.{}.name", channel_type, name),
                    message: "channel name must not be empty".to_string(),
                });
            }

            if config.enabled && config.token.is_none() {
                errors.push(ValidationError {
                    field: format!("channels.{}.{}.token", channel_type, name),
                    message: "token is required when channel is enabled".to_string(),
                });
            }
        }
    }
}

/// Validate models configuration
fn validate_models_config(
    models: &crate::openclaw_config::ModelsConfig,
    errors: &mut Vec<ValidationError>,
) {
    for (provider, config) in &models.providers {
        if provider.trim().is_empty() {
            errors.push(ValidationError {
                field: format!("models.providers.{}", provider),
                message: "provider name must not be empty".to_string(),
            });
        }

        if config.enabled && config.api_key.is_none() {
            errors.push(ValidationError {
                field: format!("models.providers.{}.api_key", provider),
                message: "API key is required when provider is enabled".to_string(),
            });
        }
    }
}

/// Validate config object with plugins (full validation)
pub fn validate_config_object_with_plugins(config: &OpenClawConfig) -> ValidationResult<()> {
    // For now, just do basic schema validation
    // TODO: Add plugin-based validation
    validate_config_schema(config)
}

/// Validate config object (raw validation without plugins)
pub fn validate_config_object_raw(config: &OpenClawConfig) -> ValidationResult<()> {
    validate_config_schema(config)
}

/// Validate config from JSON value
pub fn validate_config_json(json: &Value) -> ValidationResult<()> {
    let config: OpenClawConfig = serde_json::from_value(json.clone()).map_err(|e| {
        vec![ValidationError {
            field: "root".to_string(),
            message: format!("Invalid JSON structure: {}", e),
        }]
    })?;

    validate_config_schema(&config)
}

/// Format validation errors as string
pub fn format_validation_errors(errors: &[ValidationError]) -> String {
    errors
        .iter()
        .map(|e| format!("{}: {}", e.field, e.message))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openclaw_config::*;

    #[test]
    fn validate_valid_config() {
        let config = OpenClawConfig {
            logging: Some(LoggingConfig {
                level: "info".to_string(),
                file: None,
            }),
            gateway: Some(GatewayConfig {
                enabled: true,
                port: Some(8080),
                bind_address: None,
            }),
            ..Default::default()
        };

        assert!(validate_config_schema(&config).is_ok());
    }

    #[test]
    fn validate_invalid_log_level() {
        let config = OpenClawConfig {
            logging: Some(LoggingConfig {
                level: "invalid".to_string(),
                file: None,
            }),
            ..Default::default()
        };

        let errors = validate_config_schema(&config).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].field.contains("logging.level"));
    }

    #[test]
    fn validate_invalid_gateway_port() {
        let config = OpenClawConfig {
            gateway: Some(GatewayConfig {
                enabled: true,
                port: Some(0),
                bind_address: None,
            }),
            ..Default::default()
        };

        let errors = validate_config_schema(&config).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].field.contains("gateway.port"));
    }

    #[test]
    fn validate_channel_requires_token_when_enabled() {
        let mut channels = ChannelsConfig::default();
        channels.telegram.insert(
            "test".to_string(),
            ChannelConfig {
                enabled: true,
                token: None,
                allowlist: vec![],
                webhook_secret: None,
            },
        );

        let config = OpenClawConfig {
            channels: Some(channels),
            ..Default::default()
        };

        let errors = validate_config_schema(&config).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].field.contains("token"));
    }

    #[test]
    fn validate_provider_requires_api_key_when_enabled() {
        let mut providers = std::collections::HashMap::new();
        providers.insert(
            "test".to_string(),
            ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
            },
        );

        let config = OpenClawConfig {
            models: Some(ModelsConfig {
                providers,
                aliases: std::collections::HashMap::new(),
            }),
            ..Default::default()
        };

        let errors = validate_config_schema(&config).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].field.contains("api_key"));
    }

    #[test]
    fn format_validation_errors_readable() {
        let errors = vec![
            ValidationError {
                field: "logging.level".to_string(),
                message: "must be valid".to_string(),
            },
            ValidationError {
                field: "gateway.port".to_string(),
                message: "must be > 0".to_string(),
            },
        ];

        let formatted = format_validation_errors(&errors);
        assert!(formatted.contains("logging.level"));
        assert!(formatted.contains("gateway.port"));
        assert!(formatted.contains("must be valid"));
        assert!(formatted.contains("must be > 0"));
    }
}
