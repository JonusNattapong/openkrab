//! Chutes OAuth flow — port of `openkrab/src/commands/chutes-oauth.ts`

use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;

const CHUTES_OAUTH_ISSUER: &str = "https://api.chutes.ai";
const CHUTES_AUTHORIZE_ENDPOINT: &str = "https://api.chutes.ai/idp/authorize";
const CHUTES_TOKEN_ENDPOINT: &str = "https://api.chutes.ai/idp/token";
const CHUTES_USERINFO_ENDPOINT: &str = "https://api.chutes.ai/idp/userinfo";
const DEFAULT_REDIRECT_URI: &str = "http://127.0.0.1:1456/oauth/callback";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChutesCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: String,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChutesPkce {
    pub verifier: String,
    pub challenge: String,
}

impl ChutesPkce {
    pub fn generate() -> Self {
        let verifier = generate_random_hex(32);
        let challenge = base64url_encode(&sha256(&verifier));
        Self {
            verifier,
            challenge,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    token_type: String,
}

pub fn generate_chutes_pkce() -> ChutesPkce {
    ChutesPkce::generate()
}

pub fn parse_oauth_callback_input(input: &str, expected_state: &str) -> Result<(String, String)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("No input provided");
    }

    // Try to parse as URL
    if let Ok(url) = url::Url::parse(trimmed) {
        if let Some((code, state)) = extract_code_and_state_from_url(&url) {
            if state != expected_state {
                bail!("Invalid OAuth state");
            }
            return Ok((code, state));
        }
    }

    // Check if it looks like a redirect URL but failed to parse
    if trimmed.contains("://") || trimmed.contains("?") {
        bail!("Invalid OAuth redirect URL format");
    }

    // Legacy: treat as raw code (not recommended)
    bail!("Raw authorization code input is not supported. Please paste the full redirect URL.");
}

fn extract_code_and_state_from_url(url: &url::Url) -> Option<(String, String)> {
    let query_pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();

    let code = query_pairs.get("code")?;
    let state = query_pairs.get("state")?;

    Some((code.to_string(), state.to_string()))
}

pub fn build_chutes_authorize_url(
    client_id: &str,
    redirect_uri: &str,
    scopes: &[String],
    state: &str,
    challenge: &str,
) -> String {
    let scope_str = scopes.join(" ");
    let params = [
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("response_type", "code"),
        ("scope", &scope_str),
        ("state", state),
        ("code_challenge", challenge),
        ("code_challenge_method", "S256"),
    ];

    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", CHUTES_AUTHORIZE_ENDPOINT, query)
}

pub fn exchange_chutes_code_for_tokens(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
    verifier: &str,
) -> Result<ChutesCredentials> {
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_uri),
        ("code_verifier", verifier),
    ];

    let client = Client::new();
    let response = client.post(CHUTES_TOKEN_ENDPOINT).form(&params).send()?;

    if !response.status().is_success() {
        bail!("Chutes token exchange failed: HTTP {}", response.status());
    }

    let token_resp: TokenResponse = response.json()?;

    Ok(ChutesCredentials {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        expires_in: token_resp.expires_in,
        token_type: token_resp.token_type,
        client_id: Some(client_id.to_string()),
    })
}

pub fn login_chutes_oauth(
    client_id: &str,
    client_secret: &str,
    redirect_uri: Option<&str>,
    scopes: &[String],
) -> Result<ChutesCredentials> {
    let redirect_uri = redirect_uri.unwrap_or(DEFAULT_REDIRECT_URI);
    let pkce = generate_chutes_pkce();
    let state = generate_random_hex(16);

    // Build authorization URL
    let auth_url =
        build_chutes_authorize_url(client_id, redirect_uri, scopes, &state, &pkce.challenge);

    println!("Open this URL in your browser:");
    println!("{}", auth_url);
    println!();

    // Try to start local server for callback
    if redirect_uri.starts_with("http://127.0.0.1:")
        || redirect_uri.starts_with("http://localhost:")
    {
        if let Ok(code) = wait_for_local_callback(redirect_uri, &state, 300_000) {
            return exchange_chutes_code_for_tokens(
                client_id,
                client_secret,
                &code,
                redirect_uri,
                &pkce.verifier,
            );
        }
    }

    // Fall back to manual input
    println!("Paste the redirect URL here:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let (code, _) = parse_oauth_callback_input(input.trim(), &state)?;

    exchange_chutes_code_for_tokens(
        client_id,
        client_secret,
        &code,
        redirect_uri,
        &pkce.verifier,
    )
}

fn wait_for_local_callback(
    redirect_uri: &str,
    expected_state: &str,
    timeout_ms: u64,
) -> Result<String> {
    let url = url::Url::parse(redirect_uri)?;
    let hostname = url.host_str().unwrap_or("127.0.0.1");
    let port = url.port().unwrap_or(1456);

    if hostname != "127.0.0.1" && hostname != "localhost" {
        bail!("Chutes OAuth redirect URI must use loopback host (127.0.0.1 or localhost)");
    }

    let listener = TcpListener::bind(format!("{}:{}", hostname, port))?;
    listener.set_nonblocking(true)?;

    println!("Waiting for OAuth callback on {}...", redirect_uri);

    let start = std::time::Instant::now();
    let timeout = Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut buf = [0u8; 1024];
                match stream.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let request = String::from_utf8_lossy(&buf[..n]);
                        if request.contains("GET ")
                            && request.contains("code=")
                            && request.contains("state=")
                        {
                            let url_line = request
                                .lines()
                                .find(|line| line.starts_with("GET "))
                                .and_then(|line| line.split_whitespace().nth(1))
                                .ok_or_else(|| anyhow!("No URL found in request"))?;

                            let full_url = format!("http://{}{}", hostname, url_line);
                            let (code, state) =
                                parse_oauth_callback_input(&full_url, expected_state)?;

                            // Constant-time state comparison (defense in depth)
                            if !constant_time_compare(&state, expected_state) {
                                let response = "HTTP/1.1 400 Bad Request\r\n\
                                    Content-Type: text/html\r\n\
                                    \r\n\
                                    <!DOCTYPE html><html><head><meta charset='utf-8'/></head>\
                                    <body><h2>Invalid State</h2><p>State mismatch. Please retry.</p></body></html>";
                                let _ = stream.write_all(response.as_bytes());
                                continue;
                            }

                            // Send success response
                            let response = "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html\r\n\
                                \r\n\
                                <!DOCTYPE html><html><head><meta charset='utf-8'/></head>\
                                <body><h2>Authentication Complete</h2><p>You may close this window.</p></body></html>";
                            stream.write_all(response.as_bytes())?;

                            return Ok(code);
                        }
                    }
                    _ => {}
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(_) => break,
        }
    }

    Err(anyhow!("OAuth callback timeout after {}ms", timeout_ms))
}

fn constant_time_compare(a: &str, b: &str) -> bool {
    use subtle::ConstantTimeEq;
    a.as_bytes().ct_eq(b.as_bytes()).unwrap_u8() != 0
}

fn generate_random_hex(len: usize) -> String {
    use rand::RngCore;
    let mut bytes = vec![0u8; len];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pkce() {
        let pkce = generate_chutes_pkce();
        assert!(!pkce.verifier.is_empty());
        assert!(!pkce.challenge.is_empty());
        assert_ne!(pkce.verifier, pkce.challenge);
    }

    #[test]
    fn build_authorize_url() {
        let url = build_chutes_authorize_url(
            "client123",
            "http://localhost:1456/callback",
            &["read".to_string(), "write".to_string()],
            "state123",
            "challenge123",
        );
        assert!(url.contains("client_id=client123"));
        assert!(url.contains("scope=read%20write"));
        assert!(url.contains("state=state123"));
        assert!(url.contains("code_challenge=challenge123"));
    }

    #[test]
    fn parse_callback_input_valid() {
        let url = "http://localhost:1456/callback?code=abc123&state=xyz789";
        let (code, state) = parse_oauth_callback_input(url, "xyz789").unwrap();
        assert_eq!(code, "abc123");
        assert_eq!(state, "xyz789");
    }

    #[test]
    fn parse_callback_input_wrong_state() {
        let url = "http://localhost:1456/callback?code=abc123&state=wrong";
        assert!(parse_oauth_callback_input(url, "xyz789").is_err());
    }

    #[test]
    fn parse_callback_input_no_url() {
        assert!(parse_oauth_callback_input("justcode", "state").is_err());
    }
}
