//! OpenAI Codex OAuth flow — port of `openkrab/src/commands/openai-codex-oauth.ts`

use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const OPENAI_OAUTH_AUTHORIZE_URL: &str = "https://api.openai.com/oauth/authorize";
const OPENAI_OAUTH_TOKEN_URL: &str = "https://api.openai.com/oauth/token";
const OPENAI_OAUTH_CLIENT_ID: &str = "openai-codex"; // Would be configured
const OPENAI_OAUTH_REDIRECT_URI: &str = "http://localhost:1455/oauth/callback";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAICredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: String,
}

pub struct OAuthHandlers {
    pub on_auth: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
    pub on_prompt: Box<dyn Fn(&str, Option<&str>) -> Result<String> + Send + Sync>,
}

impl std::fmt::Debug for OAuthHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthHandlers")
            .field("on_auth", &"...")
            .field("on_prompt", &"...")
            .finish()
    }
}

impl Default for OAuthHandlers {
    fn default() -> Self {
        Self {
            on_auth: Box::new(|url| {
                println!("Open URL in browser: {}", url);
                open_url_in_browser(&url)?;
                Ok(())
            }),
            on_prompt: Box::new(|message, placeholder| {
                println!("{}", message);
                if let Some(ph) = placeholder {
                    println!("({})", ph);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                Ok(input.trim().to_string())
            }),
        }
    }
}

pub fn create_vps_aware_oauth_handlers(
    is_remote: bool,
    local_browser_message: &str,
    _manual_prompt_message: &str,
) -> OAuthHandlers {
    if is_remote {
        OAuthHandlers {
            on_auth: Box::new(move |url| {
                println!("\nYou are running in a remote/VPS environment.");
                println!("Open this URL in your LOCAL browser:\n");
                println!("{}", url);
                println!();
                Ok(())
            }),
            on_prompt: Box::new(move |message, _placeholder| {
                println!("{}", message);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                Ok(input.trim().to_string())
            }),
        }
    } else {
        let local_msg = local_browser_message.to_string();
        OAuthHandlers {
            on_auth: Box::new(move |url| {
                println!("{}", local_msg);
                println!("Opening: {}", url);
                open_url_in_browser(&url)?;
                Ok(())
            }),
            on_prompt: Box::new(|message, placeholder| {
                println!("{}", message);
                if let Some(ph) = placeholder {
                    println!("({})", ph);
                }
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                Ok(input.trim().to_string())
            }),
        }
    }
}

fn open_url_in_browser(url: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
        let status = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .status()?;
        if !status.success() {
            bail!("failed to open browser with cmd/start");
        }
        return Ok(());
    }

    if cfg!(target_os = "macos") {
        let status = std::process::Command::new("open").arg(url).status()?;
        if !status.success() {
            bail!("failed to open browser with macOS open");
        }
        return Ok(());
    }

    // Linux/Unix fallback
    let status = std::process::Command::new("xdg-open").arg(url).status()?;
    if !status.success() {
        bail!("failed to open browser with xdg-open");
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    token_type: String,
}

pub fn login_openai_codex_oauth(
    handlers: &OAuthHandlers,
    client_id: Option<&str>,
    client_secret: Option<&str>,
) -> Result<OpenAICredentials> {
    let client_id = client_id.unwrap_or(OPENAI_OAUTH_CLIENT_ID);
    let client_secret = client_secret.ok_or_else(|| anyhow!("OpenAI client secret required"))?;

    // Generate PKCE challenge
    let verifier = generate_pkce_verifier();
    let challenge = base64url_encode(&sha256(&verifier));

    // Build authorization URL
    let auth_url = format!(
        "{}?client_id={}&response_type=code&scope=openai&redirect_uri={}&code_challenge={}&code_challenge_method=S256&state={}",
        OPENAI_OAUTH_AUTHORIZE_URL,
        url_encode(client_id),
        url_encode(OPENAI_OAUTH_REDIRECT_URI),
        challenge,
        generate_state()
    );

    // Call on_auth handler
    (handlers.on_auth)(auth_url)?;

    // Get authorization code
    let code = (handlers.on_prompt)(
        "Paste the authorization code from the redirect URL:",
        Some("code=..."),
    )?;

    // Extract code from input
    let code = extract_code_from_input(&code)?;

    // Exchange code for tokens
    let client = Client::new();
    let token_params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", &code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", OPENAI_OAUTH_REDIRECT_URI),
        ("code_verifier", &verifier),
    ];

    let response = client
        .post(OPENAI_OAUTH_TOKEN_URL)
        .form(&token_params)
        .send()?;

    if !response.status().is_success() {
        bail!("OpenAI token exchange failed: HTTP {}", response.status());
    }

    let token_response: OAuthTokenResponse = response.json()?;

    Ok(OpenAICredentials {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_in: token_response.expires_in,
        token_type: token_response.token_type,
    })
}

pub fn login_openai_codex_oauth_interactive(
    is_remote: bool,
    client_id: Option<&str>,
    client_secret: Option<&str>,
) -> Result<OpenAICredentials> {
    let handlers = create_vps_aware_oauth_handlers(
        is_remote,
        "Browser will open for OpenAI authentication...",
        "Paste the redirect URL:",
    );

    let creds = login_openai_codex_oauth(&handlers, client_id, client_secret)?;

    println!("OpenAI Codex OAuth complete!");
    Ok(creds)
}

fn generate_pkce_verifier() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    base64url_encode(&bytes)
}

fn generate_state() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn sha256(data: &str) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().to_vec()
}

fn base64url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
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

fn extract_code_from_input(input: &str) -> Result<String> {
    let input = input.trim();

    // Check if it's a full URL
    if input.contains("code=") {
        if let Some(code_start) = input.find("code=") {
            let rest = &input[code_start + 5..];
            if let Some(end) = rest.find('&') {
                return Ok(rest[..end].to_string());
            }
            return Ok(rest.to_string());
        }
    }

    // Assume it's just the code
    if !input.is_empty() {
        Ok(input.to_string())
    } else {
        Err(anyhow!("No authorization code found in input"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_code_from_url() {
        let url = "http://localhost:1455/oauth/callback?code=abc123&state=xyz";
        assert_eq!(extract_code_from_input(url).unwrap(), "abc123");
    }

    #[test]
    fn extract_code_plain() {
        assert_eq!(extract_code_from_input("abc123").unwrap(), "abc123");
    }

    #[test]
    fn extract_code_empty() {
        assert!(extract_code_from_input("").is_err());
    }

    #[test]
    fn url_encoding() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("test@example.com"), "test%40example.com");
    }

    #[test]
    fn pkce_generation() {
        let verifier1 = generate_pkce_verifier();
        let verifier2 = generate_pkce_verifier();
        assert_ne!(verifier1, verifier2);
        assert!(!verifier1.is_empty());
    }
}
