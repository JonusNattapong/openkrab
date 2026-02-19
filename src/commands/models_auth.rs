use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const AUTH_PROFILES_PATH: &str = "krabkrab/credentials/auth-profiles.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthProfile {
    pub profile_id: String,
    pub provider: String,
    pub credential: Credential,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Credential {
    #[serde(rename = "token")]
    Token { token: String },
    #[serde(rename = "oauth")]
    OAuth {
        access: String,
        refresh: Option<String>,
        expires: Option<u64>,
    },
}

impl Default for Credential {
    fn default() -> Self {
        Credential::Token {
            token: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthProfiles {
    pub profiles: HashMap<String, AuthProfile>,
}

fn get_auth_profiles_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(AUTH_PROFILES_PATH)
}

pub fn load_auth_profiles() -> AuthProfiles {
    let path = get_auth_profiles_path();
    if !path.exists() {
        return AuthProfiles::default();
    }
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AuthProfiles::default(),
    }
}

pub fn save_auth_profiles(profiles: &AuthProfiles) -> Result<()> {
    let path = get_auth_profiles_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(profiles)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn models_auth_list_command() -> String {
    let profiles = load_auth_profiles();
    if profiles.profiles.is_empty() {
        return "auth profiles: none".to_string();
    }
    let lines: Vec<String> = profiles
        .profiles
        .values()
        .map(|p| {
            let expiry = p
                .expires_at
                .map(|e| format!(" expires={}", e))
                .unwrap_or_default();
            format!(
                "- {} ({}): type={}{}",
                p.profile_id,
                p.provider,
                serde_json::to_string(&p.credential).unwrap_or_default(),
                expiry
            )
        })
        .collect();
    format!("auth profiles:\n{}", lines.join("\n"))
}

pub fn models_auth_add_command(
    profile_id: &str,
    provider: &str,
    token: Option<&str>,
) -> Result<String> {
    if profile_id.trim().is_empty() {
        bail!("profile_id is required");
    }
    if provider.trim().is_empty() {
        bail!("provider is required");
    }

    let mut profiles = load_auth_profiles();
    let credential = if let Some(t) = token {
        Credential::Token {
            token: t.to_string(),
        }
    } else {
        Credential::OAuth {
            access: String::new(),
            refresh: None,
            expires: None,
        }
    };

    let profile = AuthProfile {
        profile_id: profile_id.trim().to_string(),
        provider: provider.trim().to_string(),
        credential,
        expires_at: None,
    };

    profiles
        .profiles
        .insert(profile_id.trim().to_string(), profile);
    save_auth_profiles(&profiles)?;

    Ok(format!(
        "added auth profile: {} (provider={})",
        profile_id, provider
    ))
}

pub fn models_auth_remove_command(profile_id: &str) -> Result<String> {
    let mut profiles = load_auth_profiles();
    if profiles.profiles.remove(profile_id).is_none() {
        bail!("auth profile not found: {}", profile_id);
    }
    save_auth_profiles(&profiles)?;
    Ok(format!("removed auth profile: {}", profile_id))
}

pub fn models_auth_get_command(profile_id: &str) -> Result<String> {
    let profiles = load_auth_profiles();
    let profile = profiles
        .profiles
        .get(profile_id)
        .ok_or_else(|| anyhow::anyhow!("auth profile not found: {}", profile_id))?;

    Ok(format!(
        "profile_id={} provider={} credential={}",
        profile.profile_id,
        profile.provider,
        serde_json::to_string(&profile.credential).unwrap_or_default()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_auth_list_empty() {
        let out = models_auth_list_command();
        assert!(out.contains("none"));
    }

    #[test]
    fn models_auth_add_and_get() {
        let result = models_auth_add_command("test-profile", "openai", Some("sk-test123"));
        assert!(result.is_ok());

        let get_result = models_auth_get_command("test-profile");
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().contains("test-profile"));

        let _ = models_auth_remove_command("test-profile");
    }

    #[test]
    fn models_auth_remove_nonexistent() {
        let result = models_auth_remove_command("nonexistent-profile-xyz");
        assert!(result.is_err());
    }
}
