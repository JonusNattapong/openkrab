use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use subtle::ConstantTimeEq;
use tokio::sync::RwLock;

struct RateLimitEntry {
    attempts: u32,
    first_attempt: Instant,
}

pub struct RateLimiter {
    entries: RwLock<HashMap<String, RateLimitEntry>>,
    max_attempts: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_attempts: u32, window_secs: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            max_attempts,
            window: Duration::from_secs(window_secs),
        }
    }

    pub async fn check(&self, key: &str) -> (bool, Option<u64>) {
        let mut entries = self.entries.write().await;
        let now = Instant::now();

        if let Some(entry) = entries.get_mut(key) {
            if now.duration_since(entry.first_attempt) > self.window {
                entry.attempts = 1;
                entry.first_attempt = now;
                return (true, None);
            }

            entry.attempts += 1;
            if entry.attempts > self.max_attempts {
                let remaining = self
                    .window
                    .saturating_sub(now.duration_since(entry.first_attempt));
                return (false, Some(remaining.as_millis() as u64));
            }
            return (true, None);
        }

        entries.insert(
            key.to_string(),
            RateLimitEntry {
                attempts: 1,
                first_attempt: now,
            },
        );
        (true, None)
    }

    pub async fn reset(&self, key: &str) {
        let mut entries = self.entries.write().await;
        entries.remove(key);
    }
}

/// Authentication modes supported by the gateway
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    None,
    Token,
    Password,
    TrustedProxy,
}

/// Resolved authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedAuth {
    pub mode: AuthMode,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub allow_tailscale: bool,
    pub trusted_proxy: Option<TrustedProxyConfig>,
    pub rate_limit_per_user: Option<u32>,
}

/// Trusted proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedProxyConfig {
    pub networks: Vec<String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub ok: bool,
    pub method: Option<String>,
    pub user: Option<String>,
    pub reason: Option<String>,
    pub rate_limited: Option<bool>,
    pub retry_after_ms: Option<u64>,
}

/// Authentication context for a request
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub client_ip: Option<String>,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            client_ip: None,
            headers: HashMap::new(),
            query_params: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    pub fn with_client_ip(mut self, ip: impl Into<String>) -> Self {
        self.client_ip = Some(ip.into());
        self
    }
}

/// Authenticator trait for different authentication methods
pub trait Authenticator: Send + Sync {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult;
}

/// No authentication - allows all requests
pub struct NoAuth;

impl Authenticator for NoAuth {
    fn authenticate(&self, _ctx: &AuthContext) -> AuthResult {
        AuthResult {
            ok: true,
            method: Some("none".to_string()),
            user: None,
            reason: None,
            rate_limited: None,
            retry_after_ms: None,
        }
    }
}

/// Token-based authentication
pub struct TokenAuth {
    token: String,
}

impl TokenAuth {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Authenticator for TokenAuth {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        let auth_header = ctx.headers.get("authorization");
        let token_param = ctx.query_params.get("token");

        let provided_token: Option<&str> = auth_header
            .and_then(|h| h.strip_prefix("Bearer "))
            .or_else(|| token_param.map(|s| s.as_str()));

        match provided_token {
            Some(token) if token == self.token => AuthResult {
                ok: true,
                method: Some("token".to_string()),
                user: None,
                reason: None,
                rate_limited: None,
                retry_after_ms: None,
            },
            _ => AuthResult {
                ok: false,
                method: Some("token".to_string()),
                user: None,
                reason: Some("Invalid token".to_string()),
                rate_limited: None,
                retry_after_ms: None,
            },
        }
    }
}

/// Password-based authentication
pub struct PasswordAuth {
    username: String,
    password: String,
}

impl PasswordAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    fn constant_time_compare(a: &str, b: &str) -> bool {
        a.as_bytes().ct_eq(b.as_bytes()).into()
    }
}

impl Authenticator for PasswordAuth {
    fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        let auth_header = ctx.headers.get("authorization");

        if let Some(header) = auth_header {
            if let Some(credentials) = header.strip_prefix("Basic ") {
                if let Ok(decoded) = STANDARD.decode(credentials) {
                    if let Ok(creds_str) = String::from_utf8(decoded) {
                        let parts: Vec<&str> = creds_str.split(':').collect();
                        if parts.len() == 2 {
                            let username = parts[0];
                            let password = parts[1];

                            let username_ok = Self::constant_time_compare(username, &self.username);
                            let password_ok = Self::constant_time_compare(password, &self.password);

                            if username_ok & password_ok {
                                return AuthResult {
                                    ok: true,
                                    method: Some("password".to_string()),
                                    user: Some(username.to_string()),
                                    reason: None,
                                    rate_limited: None,
                                    retry_after_ms: None,
                                };
                            }
                        }
                    }
                }
            }
        }

        AuthResult {
            ok: false,
            method: Some("password".to_string()),
            user: None,
            reason: Some("Invalid credentials".to_string()),
            rate_limited: None,
            retry_after_ms: None,
        }
    }
}

/// Authentication manager that handles different auth methods
pub struct AuthManager {
    authenticator: Box<dyn Authenticator>,
    rate_limiter: Option<Arc<RateLimiter>>,
}

impl AuthManager {
    pub fn new(auth_config: ResolvedAuth) -> Self {
        let rate_limiter = auth_config
            .rate_limit_per_user
            .map(|max_attempts| Arc::new(RateLimiter::new(max_attempts, 60)));

        let authenticator: Box<dyn Authenticator> = match auth_config.mode {
            AuthMode::None => Box::new(NoAuth),
            AuthMode::Token => {
                let token = auth_config.token.unwrap_or_default();
                Box::new(TokenAuth::new(token))
            }
            AuthMode::Password => {
                let username = auth_config.username.unwrap_or_else(|| "admin".to_string());
                let password = auth_config.password.unwrap_or_default();
                Box::new(PasswordAuth::new(username, password))
            }
            AuthMode::TrustedProxy => Box::new(NoAuth), // Simplified for now
        };

        Self {
            authenticator,
            rate_limiter,
        }
    }

    pub async fn authenticate(&self, ctx: &AuthContext) -> AuthResult {
        if let Some(limiter) = &self.rate_limiter {
            let client_ip = ctx.client_ip.as_deref().unwrap_or("unknown");
            let (allowed, retry_after) = limiter.check(client_ip).await;
            if !allowed {
                return AuthResult {
                    ok: false,
                    method: None,
                    user: None,
                    reason: Some("Rate limit exceeded".to_string()),
                    rate_limited: Some(true),
                    retry_after_ms: retry_after,
                };
            }
        }

        self.authenticator.authenticate(ctx)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new(ResolvedAuth {
            mode: AuthMode::None,
            token: None,
            username: None,
            password: None,
            allow_tailscale: false,
            trusted_proxy: None,
            rate_limit_per_user: None,
        })
    }
}

pub fn is_loopback_address(ip: &str) -> bool {
    ip == "127.0.0.1" || ip == "::1" || ip.starts_with("127.")
}

pub fn is_local_request(client_ip: Option<&str>) -> bool {
    client_ip.map_or(false, |ip| is_loopback_address(ip))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_auth() {
        let auth = NoAuth;
        let ctx = AuthContext::new();
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("none".to_string()));
    }

    #[test]
    fn test_token_auth_success() {
        let auth = TokenAuth::new("secret-token".to_string());
        let ctx = AuthContext::new().with_header("authorization", "Bearer secret-token");
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("token".to_string()));
    }

    #[test]
    fn test_token_auth_failure() {
        let auth = TokenAuth::new("secret-token".to_string());
        let ctx = AuthContext::new().with_header("authorization", "Bearer wrong-token");
        let result = auth.authenticate(&ctx);
        assert!(!result.ok);
        assert_eq!(result.reason, Some("Invalid token".to_string()));
    }

    #[test]
    fn test_password_auth_success() {
        let auth = PasswordAuth::new("admin".to_string(), "password".to_string());
        let credentials = STANDARD.encode("admin:password");
        let ctx = AuthContext::new().with_header("authorization", format!("Basic {}", credentials));
        let result = auth.authenticate(&ctx);
        assert!(result.ok);
        assert_eq!(result.method, Some("password".to_string()));
        assert_eq!(result.user, Some("admin".to_string()));
    }

    #[test]
    fn test_is_loopback_address() {
        assert!(is_loopback_address("127.0.0.1"));
        assert!(is_loopback_address("::1"));
        assert!(!is_loopback_address("192.168.1.1"));
    }
}
