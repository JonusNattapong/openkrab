//! OAuth commands for CLI.
//! Provides OAuth 2.0 PKCE flow for authenticating with various providers.

use crate::oauth::{
    build_auth_url, exchange_code, parse_callback_url, PkceChallenge, OAuthClientConfig, OAuthTokens,
};
use crate::providers::minimax_oauth::{self, MiniMaxRegion};
use crate::providers::qwen_oauth::{self, QwenCredentials};
use anyhow::{anyhow, bail, Result};
use rand::RngCore;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Check if running in a remote environment where local browser OAuth flow won't work.
pub fn is_remote_environment() -> bool {
    // Check for SSH session
    if env::var("SSH_CLIENT").is_ok() || env::var("SSH_TTY").is_ok() || env::var("SSH_CONNECTION").is_ok() {
        return true;
    }

    // Check for remote container environments
    if env::var("REMOTE_CONTAINERS").is_ok() || env::var("CODESPACES").is_ok() {
        return true;
    }

    // Check for headless Linux (no display, not WSL)
    if cfg!(target_os = "linux") {
        let has_display = env::var("DISPLAY").is_ok();
        let has_wayland = env::var("WAYLAND_DISPLAY").is_ok();

        if !has_display && !has_wayland {
            // Check if we're in WSL
            let is_wsl = env::var("WSL_DISTRO_NAME").is_ok() ||
                        env::var("WSLENV").is_ok() ||
                        std::fs::read_to_string("/proc/version")
                            .map(|v| v.contains("Microsoft") || v.contains("WSL"))
                            .unwrap_or(false);

            if !is_wsl {
                return true;
            }
        }
    }

    false
}

fn get_token_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("oauth_tokens.json")
}

fn load_tokens() -> Result<Option<OAuthTokens>> {
    let path = get_token_path();
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    let tokens: OAuthTokens = serde_json::from_str(&content)?;
    Ok(Some(tokens))
}

fn save_tokens(tokens: &OAuthTokens) -> Result<()> {
    let path = get_token_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(tokens)?;
    fs::write(&path, content)?;
    Ok(())
}

/// Get OAuth flow instructions based on environment.
pub fn get_oauth_instructions() -> String {
    if is_remote_environment() {
        "Open this URL in your LOCAL browser and paste the redirect URL back here:".to_string()
    } else {
        "Opening browser for OAuth authentication...".to_string()
    }
}

/// Build OAuth authorization URL for a provider.
pub async fn build_url(provider: &str, client_id: &str, redirect_uri: &str) -> Result<String> {
    let (auth_url, token_url, scopes) = match provider {
        "google" => (
            "https://accounts.google.com/o/oauth2/v2/auth",
            "https://oauth2.googleapis.com/token",
            vec!["openid".to_string(), "email".to_string(), "https://www.googleapis.com/auth/gmail.readonly".to_string()],
        ),
        "minimax" => (
            "https://platform.minimaxi.com/oauth/code",
            "https://platform.minimaxi.com/oauth/token",
            vec!["api".to_string()],
        ),
        "qwen" => (
            "https://chat.qwen.ai/oauth/authorize",
            "https://chat.qwen.ai/api/v1/oauth2/token",
            vec!["api".to_string()],
        ),
        _ => bail!("Unsupported provider: {}. Use: google, minimax, or qwen", provider),
    };

    let cfg = OAuthClientConfig {
        client_id: client_id.to_string(),
        client_secret: String::new(),
        redirect_uri: redirect_uri.to_string(),
        auth_url: auth_url.to_string(),
        token_url: token_url.to_string(),
        scopes,
    };

    let challenge = PkceChallenge::generate();
    let url = build_auth_url(&cfg, &challenge.challenge, "state123", &[]);
    
    // Store verifier for later token exchange
    let verifier_path = get_token_path().with_extension("verifier");
    fs::write(&verifier_path, &challenge.verifier)?;
    
    Ok(url)
}

/// Complete OAuth flow by exchanging code for tokens.
pub async fn complete_flow(redirect_url: &str) -> Result<OAuthTokens> {
    let verifier_path = get_token_path().with_extension("verifier");
    if !verifier_path.exists() {
        bail!("No pending OAuth flow. Run 'oauth url' first.");
    }
    
    let verifier = fs::read_to_string(&verifier_path)?;
    let callback = parse_callback_url(redirect_url)?;
    
    // Use a default client config - in real usage this would be passed in
    let cfg = OAuthClientConfig {
        client_id: String::new(), // Would be provided by caller
        client_secret: String::new(),
        redirect_uri: String::new(),
        auth_url: String::new(),
        token_url: String::new(),
        scopes: vec![],
    };
    
    let client = Client::new();
    let tokens = exchange_code(&client, &cfg, &callback.code, &verifier).await?;
    
    // Clean up verifier
    let _ = fs::remove_file(&verifier_path);
    
    // Save tokens
    save_tokens(&tokens)?;
    
    Ok(tokens)
}

/// Get stored OAuth tokens if not expired.
pub fn get_stored_tokens() -> Result<Option<OAuthTokens>> {
    let tokens = load_tokens()?;
    if let Some(t) = &tokens {
        if t.is_expired() {
            println!("Tokens expired. Need to refresh.");
            return Ok(None);
        }
    }
    Ok(tokens)
}

/// Refresh OAuth tokens.
pub async fn refresh_tokens(provider: &str) -> Result<OAuthTokens> {
    let tokens = load_tokens()?;
    let tokens = tokens.ok_or_else(|| anyhow::anyhow!("No stored tokens found"))?;
    
    let token_url = match provider {
        "google" => "https://oauth2.googleapis.com/token",
        "minimax" => "https://platform.minimaxi.com/oauth/token",
        "qwen" => "https://chat.qwen.ai/api/v1/oauth2/token",
        _ => bail!("Unsupported provider: {}", provider),
    };
    
    let cfg = OAuthClientConfig {
        client_id: String::new(),
        client_secret: String::new(),
        redirect_uri: String::new(),
        auth_url: String::new(),
        token_url: token_url.to_string(),
        scopes: vec![],
    };
    
    let client = Client::new();
    let new_tokens = crate::oauth::refresh_token(&client, &cfg, &tokens.refresh_token).await?;
    
    save_tokens(&new_tokens)?;
    Ok(new_tokens)
}

// ─── MiniMax Device Code OAuth ─────────────────────────────────────────────────

const MINIMAX_POLL_TIMEOUT_SECS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeProgress {
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub status: String,
}

pub fn login_minimax_oauth(region: Option<&str>) -> Result<QwenCredentials> {
    let region = match region.unwrap_or("global") {
        "cn" => MiniMaxRegion::Cn,
        _ => MiniMaxRegion::Global,
    };

    let mut login = crate::providers::minimax_oauth::MiniMaxPkce::generate();
    
    // Step 1: Request device code
    let body = minimax_oauth::build_code_request_body(
        &login.challenge,
        &login.state,
        region,
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(region.code_endpoint())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(body)
        .send()?;

    if !response.status().is_success() {
        bail!("MiniMax device code request failed: HTTP {}", response.status());
    }

    let body = response.text()?;
    let auth = minimax_oauth::parse_authorization(&body, &login.state)?;

    println!("\nOpen {} to approve access.", auth.verification_uri);
    println!("If prompted, enter the code: {}", auth.user_code);
    println!("Expires in: {} seconds\n", auth.expired_in);

    // Step 2: Poll for token
    let start = std::time::Instant::now();
    let mut interval_ms = auth.interval.unwrap_or(2000);

    while start.elapsed().as_secs() < MINIMAX_POLL_TIMEOUT_SECS {
        let poll_body = minimax_oauth::build_token_poll_body(
            &auth.user_code,
            &login.verifier,
            region,
        );

        let resp = client
            .post(region.token_endpoint())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(poll_body)
            .send()?;

        let poll_text = resp.text()?;
        let result = minimax_oauth::parse_poll_result(&poll_text);

        match result {
            minimax_oauth::PollResult::Success(token) => {
                println!("Authorization successful!");
                // Save to auth profiles
                use crate::commands::models_auth::models_auth_add_command;
                let profile_id = format!("minimax:{}", if region == MiniMaxRegion::Cn { "cn" } else { "global" });
                let _ = models_auth_add_command(&profile_id, "minimax", Some(&token.access));
                return Ok(QwenCredentials {
                    access: token.access,
                    refresh: token.refresh,
                    expires: token.expires,
                });
            }
            minimax_oauth::PollResult::Pending { message } => {
                println!("Waiting... ({})", message.as_deref().unwrap_or("pending"));
            }
            minimax_oauth::PollResult::Error(e) => {
                bail!("MiniMax OAuth error: {}", e);
            }
        }

        interval_ms = minimax_oauth::next_poll_interval_ms(interval_ms);
        std::thread::sleep(Duration::from_millis(interval_ms));
    }

    Err(anyhow!("MiniMax OAuth timed out waiting for authorization."))
}

// ─── Qwen Device Code OAuth ───────────────────────────────────────────────────

pub fn login_qwen_oauth() -> Result<QwenCredentials> {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let verifier = hex::encode(&bytes);
    let challenge = {
        use sha2::{Digest, Sha256};
        use base64::Engine;
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
    };

    // Step 1: Request device code
    let params = [
        ("client_id", "f0304373b74a44d2b584a3fb70ca9e56"),
        ("scope", "openid profile email model.completion"),
        ("code_challenge", &challenge),
        ("code_challenge_method", "S256"),
    ];
    let body: String = params.iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://chat.qwen.ai/api/v1/oauth2/device/code")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .body(body)
        .send()?;

    if !response.status().is_success() {
        bail!("Qwen device code request failed: HTTP {}", response.status());
    }

    #[derive(Deserialize)]
    struct DeviceCodeResponse {
        device_code: String,
        user_code: String,
        verification_uri: String,
        #[serde(rename = "verification_uri_complete")]
        verification_uri_complete: Option<String>,
        expires_in: u64,
        interval: Option<u64>,
    }

    let device: DeviceCodeResponse = response.json()?;
    let verification_uri = device.verification_uri_complete.unwrap_or(device.verification_uri.clone());

    println!("\nOpen {} to approve access.", verification_uri);
    println!("If prompted, enter the code: {}", device.user_code);
    println!("Expires in: {} seconds\n", device.expires_in);

    // Step 2: Poll for token
    let start = std::time::Instant::now();
    let mut interval_ms = device.interval.unwrap_or(2) * 1000;
    let timeout_ms = device.expires_in * 1000;

    while (start.elapsed().as_millis() as u64) < timeout_ms {
        let poll_params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", "f0304373b74a44d2b584a3fb70ca9e56"),
            ("device_code", &device.device_code),
            ("code_verifier", &verifier),
        ];
        let poll_body: String = poll_params.iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let resp = client
            .post("https://chat.qwen.ai/api/v1/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(poll_body)
            .send()?;

        if resp.status().is_success() {
            let token_resp: qwen_oauth::QwenRefreshResponse = resp.json()?;
            if let (Some(access), Some(expires_in)) = (token_resp.access_token, token_resp.expires_in) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                let expires = now + expires_in * 1000;
                
                println!("Authorization successful!");
                
                use crate::commands::models_auth::models_auth_add_command;
                let _ = models_auth_add_command("qwen:default", "qwen-portal", Some(&access));
                
                return Ok(QwenCredentials {
                    access,
                    refresh: token_resp.refresh_token,
                    expires,
                });
            }
        }

        println!("Waiting for authorization...");
        interval_ms = (interval_ms * 3 / 2).min(10_000);
        std::thread::sleep(Duration::from_millis(interval_ms));
    }

    Err(anyhow!("Qwen OAuth timed out waiting for authorization."))
}

// ─── GitHub Copilot Device Code OAuth ───────────────────────────────────────--

pub fn login_github_copilot(profile_id: Option<&str>) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    
    // Step 1: Request device code
    let body = "client_id=Iv1.b507a08c87ecfe98&scope=read:user";
    let response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()?;

    if !response.status().is_success() {
        bail!("GitHub device code request failed: HTTP {}", response.status());
    }

    #[derive(Deserialize)]
    struct DeviceCodeResponse {
        device_code: String,
        user_code: String,
        verification_uri: String,
        expires_in: u64,
        interval: u64,
    }

    let device: DeviceCodeResponse = response.json()?;

    println!("\nAuthorize GitHub Copilot:");
    println!("Visit: {}", device.verification_uri);
    println!("Code: {}", device.user_code);
    println!("\nWaiting for authorization... (expires in {} seconds)\n", device.expires_in);

    // Step 2: Poll for token
    let start = std::time::Instant::now();
    let interval_ms = device.interval * 1000;

    while start.elapsed().as_secs() < device.expires_in as u64 {
        let poll_body = format!(
            "client_id=Iv1.b507a08c87ecfe98&device_code={}&grant_type=urn:ietf:params:oauth:grant-type:device_code",
            device.device_code
        );

        let resp = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(poll_body)
            .send()?;

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: Option<String>,
            error: Option<String>,
        }

        let token_resp: TokenResponse = resp.json()?;

        if let Some(access_token) = token_resp.access_token {
            println!("Authorization successful!");
            
            let profile_id = profile_id.unwrap_or("github-copilot:github");
            use crate::commands::models_auth::models_auth_add_command;
            let _ = models_auth_add_command(profile_id, "github-copilot", Some(&access_token));
            
            return Ok(format!("Added auth profile: {}", profile_id));
        }

        if let Some(error) = token_resp.error {
            if error == "authorization_pending" || error == "slow_down" {
                println!("Waiting...");
            } else if error == "expired_token" {
                bail!("GitHub device code expired. Run login again.");
            } else if error == "access_denied" {
                bail!("GitHub login cancelled.");
            } else {
                bail!("GitHub OAuth error: {}", error);
            }
        }

        std::thread::sleep(Duration::from_millis(interval_ms));
    }

    Err(anyhow!("GitHub OAuth timed out waiting for authorization."))
}

fn url_encode(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || "-._~".contains(c) {
                vec![c]
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| format!("%{:02X}", b).chars().collect::<Vec<_>>())
                    .collect()
            }
        })
        .collect()
}
