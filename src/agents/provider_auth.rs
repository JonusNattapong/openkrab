//! agents::provider_auth — Provider authentication wiring.
//! Ported from `openclaw/src/agents/model-auth.ts` (Phase 15).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Provider authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub models: Option<Vec<String>>,
}

/// Auth profile for managing multiple API keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProfile {
    pub name: String,
    pub provider: String,
    pub api_key: String,
    pub base_url: Option<String>,
    pub priority: i32,
    pub last_used: Option<u64>,
    pub cooldown_until: Option<u64>,
}

/// Resolve API key for a provider from various sources.
pub fn resolve_api_key_for_provider(provider: &str) -> Option<String> {
    // 1. Environment variable
    let env_key = match provider {
        "openai" => "OPENAI_API_KEY",
        "gemini" | "google" => "GEMINI_API_KEY",
        "anthropic" => "ANTHROPIC_API_KEY",
        "voyage" => "VOYAGE_API_KEY",
        "deepgram" => "DEEPGRAM_API_KEY",
        "github" | "copilot" => "GITHUB_TOKEN",
        "qwen" => "QWEN_ACCESS_TOKEN",
        _ => return None,
    };

    if let Ok(key) = env::var(env_key) {
        if !key.trim().is_empty() {
            return Some(key);
        }
    }

    // 2. Auth profiles (placeholder - would load from storage)
    // In production, this would load from ~/.krabkrab/auth_profiles.json

    None
}

/// Resolve base URL for a provider.
pub fn resolve_base_url_for_provider(provider: &str) -> Option<String> {
    let env_key = match provider {
        "openai" => "OPENAI_BASE_URL",
        "gemini" | "google" => "GEMINI_BASE_URL",
        "ollama" => "OLLAMA_BASE_URL",
        _ => return None,
    };

    env::var(env_key).ok()
}

/// Get provider configuration with authentication.
pub fn get_provider_config(provider: &str) -> ProviderAuthConfig {
    ProviderAuthConfig {
        provider: provider.to_string(),
        api_key: resolve_api_key_for_provider(provider),
        base_url: resolve_base_url_for_provider(provider),
        models: None, // Would be populated from model catalog
    }
}

/// Check if a provider is configured (has API key).
pub fn is_provider_configured(provider: &str) -> bool {
    resolve_api_key_for_provider(provider).is_some()
}

/// Get all configured providers.
pub fn get_configured_providers() -> Vec<String> {
    let providers = vec![
        "openai",
        "gemini",
        "ollama",
        "copilot",
        "qwen",
        "anthropic",
        "voyage",
        "deepgram",
    ];

    providers
        .into_iter()
        .filter(|p| is_provider_configured(p))
        .map(|s| s.to_string())
        .collect()
}

/// Validate API key format for a provider.
pub fn validate_api_key(provider: &str, api_key: &str) -> Result<(), String> {
    if api_key.trim().is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    match provider {
        "openai" => {
            if !api_key.starts_with("sk-") {
                return Err("OpenAI API key should start with 'sk-'".to_string());
            }
        }
        "gemini" | "google" => {
            // Gemini keys are typically longer and don't have specific prefixes
            if api_key.len() < 20 {
                return Err("Gemini API key seems too short".to_string());
            }
        }
        "anthropic" => {
            if !api_key.starts_with("sk-ant-") {
                return Err("Anthropic API key should start with 'sk-ant-'".to_string());
            }
        }
        _ => {
            // Generic validation for other providers
            if api_key.len() < 10 {
                return Err("API key seems too short".to_string());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_openai_api_key_from_env() {
        env::set_var("OPENAI_API_KEY", "sk-test123");
        assert_eq!(
            resolve_api_key_for_provider("openai"),
            Some("sk-test123".to_string())
        );
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn validate_openai_key_format() {
        assert!(validate_api_key("openai", "sk-valid123").is_ok());
        assert!(validate_api_key("openai", "invalid").is_err());
        assert!(validate_api_key("openai", "").is_err());
    }

    #[test]
    fn validate_gemini_key_format() {
        assert!(validate_api_key("gemini", "AIzaSyDummyKeyWithEnoughLength").is_ok());
        assert!(validate_api_key("gemini", "short").is_err());
    }

    #[test]
    fn get_provider_config_populates_from_env() {
        env::set_var("OPENAI_API_KEY", "sk-test123");
        env::set_var("OPENAI_BASE_URL", "https://custom.openai.com");

        let config = get_provider_config("openai");
        assert_eq!(config.provider, "openai");
        assert_eq!(config.api_key, Some("sk-test123".to_string()));
        assert_eq!(
            config.base_url,
            Some("https://custom.openai.com".to_string())
        );

        env::remove_var("OPENAI_API_KEY");
        env::remove_var("OPENAI_BASE_URL");
    }

    #[test]
    fn is_provider_configured_checks_env() {
        assert!(!is_provider_configured("openai"));

        env::set_var("OPENAI_API_KEY", "sk-test123");
        assert!(is_provider_configured("openai"));

        env::remove_var("OPENAI_API_KEY");
    }
}
