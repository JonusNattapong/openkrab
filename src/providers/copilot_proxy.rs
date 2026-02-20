//! providers::copilot_proxy — Local Copilot Proxy (VS Code LM) provider.
//! Ported from `openkrab/extensions/copilot-proxy/index.ts` (Phase 21).
//!
//! Bridges requests to a locally-running Copilot Proxy VS Code extension that
//! exposes an OpenAI-compatible `/v1/chat/completions` endpoint.
//! Base URL is user-configurable; no real auth token is required.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// ─── Defaults ─────────────────────────────────────────────────────────────────

pub const DEFAULT_BASE_URL: &str = "http://localhost:3000/v1";
/// Placeholder API key — Copilot Proxy ignores auth headers.
pub const DEFAULT_API_KEY: &str = "n/a";
pub const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;
pub const DEFAULT_MAX_TOKENS: u64 = 8_192;

/// Default model IDs served by the Copilot Proxy extension.
/// Availability depends on the user's Copilot plan.
pub const DEFAULT_MODEL_IDS: &[&str] = &[
    "gpt-5.2",
    "gpt-5.2-codex",
    "gpt-5.1",
    "gpt-5.1-codex",
    "gpt-5.1-codex-max",
    "gpt-5-mini",
    "claude-opus-4.6",
    "claude-opus-4.5",
    "claude-sonnet-4.5",
    "claude-haiku-4.5",
    "gemini-3-pro",
    "gemini-3-flash",
    "grok-code-fast-1",
];

// ─── Config ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotProxyConfig {
    /// Base URL including `/v1` (e.g. `http://localhost:3000/v1`).
    pub base_url: String,
    /// API key sent in `Authorization: Bearer` header; defaults to `"n/a"`.
    pub api_key: String,
    /// Model IDs available on this proxy instance.
    pub model_ids: Vec<String>,
}

impl Default for CopilotProxyConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: DEFAULT_API_KEY.to_string(),
            model_ids: DEFAULT_MODEL_IDS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl CopilotProxyConfig {
    /// Load from environment variables or return defaults.
    ///
    /// - `COPILOT_PROXY_BASE_URL` — override base URL
    /// - `COPILOT_PROXY_MODELS`   — comma-separated model ID list
    pub fn from_env() -> Self {
        let base_url = std::env::var("COPILOT_PROXY_BASE_URL")
            .ok()
            .map(|v| normalize_base_url(&v))
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        let model_ids = std::env::var("COPILOT_PROXY_MODELS")
            .ok()
            .map(|v| parse_model_ids(&v))
            .unwrap_or_else(|| DEFAULT_MODEL_IDS.iter().map(|s| s.to_string()).collect());

        Self {
            base_url,
            api_key: std::env::var("COPILOT_PROXY_API_KEY")
                .unwrap_or_else(|_| DEFAULT_API_KEY.to_string()),
            model_ids,
        }
    }

    pub fn validate(&self) -> Result<()> {
        let url = validate_base_url(&self.base_url)?;
        drop(url);
        if self.model_ids.is_empty() {
            bail!("copilot-proxy: at least one model ID is required");
        }
        Ok(())
    }

    /// Return the first (default) model reference in `copilot-proxy/<id>` form.
    pub fn default_model_ref(&self) -> Option<String> {
        self.model_ids
            .first()
            .map(|id| format!("copilot-proxy/{}", id))
    }
}

// ─── URL helpers ──────────────────────────────────────────────────────────────

/// Strip trailing slashes; append `/v1` when the path doesn't already end with it.
pub fn normalize_base_url(value: &str) -> String {
    let mut normalized = value.trim().trim_end_matches('/').to_string();
    if !normalized.is_empty() && !normalized.ends_with("/v1") {
        normalized.push_str("/v1");
    }
    if normalized.is_empty() {
        return DEFAULT_BASE_URL.to_string();
    }
    normalized
}

/// Return `Err` when `value` is not a valid URL after normalization.
pub fn validate_base_url(value: &str) -> Result<url::Url> {
    let normalized = normalize_base_url(value);
    url::Url::parse(&normalized).map_err(|e| anyhow::anyhow!("invalid Copilot Proxy URL: {}", e))
}

// ─── Model ID parsing ─────────────────────────────────────────────────────────

/// Split a newline/comma-separated model list and deduplicate.
pub fn parse_model_ids(input: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    input
        .split(['\n', ','])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && seen.insert(s.clone()))
        .collect()
}

// ─── Model definition ─────────────────────────────────────────────────────────

/// OpenAI-compatible model definition for a Copilot Proxy model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyModelDefinition {
    pub id: String,
    pub name: String,
    /// Always `"openai-completions"` (the VS Code extension exposes `/v1/chat/completions`).
    pub api: String,
    pub reasoning: bool,
    pub context_window: u64,
    pub max_tokens: u64,
    /// Zero cost — billed via Copilot subscription.
    pub cost_input: u64,
    pub cost_output: u64,
}

pub fn build_model_definition(model_id: &str) -> ProxyModelDefinition {
    let id = model_id.trim().to_string();
    ProxyModelDefinition {
        name: id.clone(),
        id,
        api: "openai-completions".to_string(),
        reasoning: false,
        context_window: DEFAULT_CONTEXT_WINDOW,
        max_tokens: DEFAULT_MAX_TOKENS,
        cost_input: 0,
        cost_output: 0,
    }
}

/// Build definitions for all models in the given config.
pub fn build_model_definitions(cfg: &CopilotProxyConfig) -> Vec<ProxyModelDefinition> {
    cfg.model_ids
        .iter()
        .map(|id| build_model_definition(id))
        .collect()
}

// ─── HTTP send helper ─────────────────────────────────────────────────────────

/// Send a chat completion request to the Copilot Proxy server.
///
/// Uses the `/chat/completions` endpoint appended to `base_url`.
/// Returns the raw response JSON.
pub async fn send_completion(
    client: &reqwest::Client,
    cfg: &CopilotProxyConfig,
    model_id: &str,
    messages: Vec<serde_json::Value>,
) -> Result<serde_json::Value> {
    let url = format!("{}/chat/completions", cfg.base_url);
    let payload = serde_json::json!({
        "model": model_id,
        "messages": messages,
    });
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", cfg.api_key))
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        bail!(
            "copilot-proxy: HTTP {} from {}",
            resp.status(),
            url
        );
    }

    Ok(resp.json().await?)
}

/// Extract the assistant reply text from an OpenAI-style chat completion response.
pub fn extract_reply(response: &serde_json::Value) -> Option<String> {
    response
        .get("choices")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
}

// ─── LlmProvider implementation ───────────────────────────────────────────────

use async_trait::async_trait;

use crate::providers::LlmProvider;

pub struct CopilotProxyProvider {
    config: CopilotProxyConfig,
    client: reqwest::Client,
    /// The model ID to use for completions (defaults to first in list).
    model_id: String,
}

impl CopilotProxyProvider {
    pub fn new(config: CopilotProxyConfig) -> Self {
        let model_id = config
            .model_ids
            .first()
            .cloned()
            .unwrap_or_else(|| DEFAULT_MODEL_IDS[0].to_string());
        Self {
            config,
            client: reqwest::Client::new(),
            model_id,
        }
    }

    pub fn from_env() -> Option<Self> {
        let cfg = CopilotProxyConfig::from_env();
        if cfg.base_url.is_empty() {
            return None;
        }
        Some(Self::new(cfg))
    }

    pub fn with_model(mut self, model_id: impl Into<String>) -> Self {
        self.model_id = model_id.into();
        self
    }
}

#[async_trait]
impl LlmProvider for CopilotProxyProvider {
    fn name(&self) -> &str {
        "copilot-proxy"
    }

    async fn complete(&self, prompt: &str) -> Result<String> {
        let messages = vec![serde_json::json!({"role": "user", "content": prompt})];
        let resp = send_completion(&self.client, &self.config, &self.model_id, messages).await?;
        extract_reply(&resp)
            .ok_or_else(|| anyhow::anyhow!("copilot-proxy: no reply content in response"))
    }

    async fn embed(&self, _text: &str) -> Result<Vec<f32>> {
        bail!("copilot-proxy: embedding is not supported by the Copilot Proxy extension")
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_base_url_adds_v1() {
        assert_eq!(
            normalize_base_url("http://localhost:3000"),
            "http://localhost:3000/v1"
        );
    }

    #[test]
    fn normalize_base_url_strips_trailing_slash() {
        assert_eq!(
            normalize_base_url("http://localhost:3000/"),
            "http://localhost:3000/v1"
        );
    }

    #[test]
    fn normalize_base_url_keeps_existing_v1() {
        assert_eq!(
            normalize_base_url("http://localhost:3000/v1"),
            "http://localhost:3000/v1"
        );
    }

    #[test]
    fn normalize_base_url_empty_returns_default() {
        assert_eq!(normalize_base_url(""), DEFAULT_BASE_URL);
        assert_eq!(normalize_base_url("   "), DEFAULT_BASE_URL);
    }

    #[test]
    fn validate_base_url_valid() {
        assert!(validate_base_url("http://localhost:3000").is_ok());
        assert!(validate_base_url("http://localhost:3000/v1").is_ok());
    }

    #[test]
    fn validate_base_url_invalid() {
        assert!(validate_base_url("not a url!!").is_err());
    }

    #[test]
    fn parse_model_ids_comma_separated() {
        let ids = parse_model_ids("gpt-4o, gpt-4o-mini, claude-3");
        assert_eq!(ids, vec!["gpt-4o", "gpt-4o-mini", "claude-3"]);
    }

    #[test]
    fn parse_model_ids_newline_separated() {
        let ids = parse_model_ids("gpt-4o\ngpt-4o-mini");
        assert_eq!(ids, vec!["gpt-4o", "gpt-4o-mini"]);
    }

    #[test]
    fn parse_model_ids_deduplicates() {
        let ids = parse_model_ids("gpt-4o, gpt-4o, gpt-4o-mini");
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn parse_model_ids_empty_input() {
        let ids = parse_model_ids("");
        assert!(ids.is_empty());
    }

    #[test]
    fn config_default_model_ref() {
        let cfg = CopilotProxyConfig::default();
        let ref_ = cfg.default_model_ref().unwrap();
        assert!(ref_.starts_with("copilot-proxy/"));
        assert!(ref_.contains("gpt-5.2"));
    }

    #[test]
    fn config_validate_ok() {
        let cfg = CopilotProxyConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn config_validate_no_models() {
        let cfg = CopilotProxyConfig {
            model_ids: vec![],
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn config_validate_bad_url() {
        let cfg = CopilotProxyConfig {
            base_url: "not-a-url".to_string(),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn build_model_definition_fields() {
        let def = build_model_definition("claude-sonnet-4.5");
        assert_eq!(def.id, "claude-sonnet-4.5");
        assert_eq!(def.api, "openai-completions");
        assert_eq!(def.context_window, DEFAULT_CONTEXT_WINDOW);
        assert_eq!(def.max_tokens, DEFAULT_MAX_TOKENS);
        assert_eq!(def.cost_input, 0);
        assert_eq!(def.cost_output, 0);
        assert!(!def.reasoning);
    }

    #[test]
    fn build_model_definitions_count() {
        let cfg = CopilotProxyConfig::default();
        let defs = build_model_definitions(&cfg);
        assert_eq!(defs.len(), cfg.model_ids.len());
    }

    #[test]
    fn default_model_ids_non_empty() {
        assert!(!DEFAULT_MODEL_IDS.is_empty());
        assert!(DEFAULT_MODEL_IDS.contains(&"gpt-5.2"));
        assert!(DEFAULT_MODEL_IDS.contains(&"claude-sonnet-4.5"));
    }

    #[test]
    fn extract_reply_from_well_formed_response() {
        let resp = serde_json::json!({
            "choices": [{"message": {"content": "Hello!"}}]
        });
        assert_eq!(extract_reply(&resp).unwrap(), "Hello!");
    }

    #[test]
    fn extract_reply_missing_choices() {
        let resp = serde_json::json!({});
        assert!(extract_reply(&resp).is_none());
    }
}
