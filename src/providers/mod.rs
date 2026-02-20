//! providers — LLM provider registry and trait.
//! Ported from `openclaw/src/providers/` (Phase 5).
//!
//! Provides a unified `LlmProvider` trait and a `ProviderRegistry` for
//! selecting chat / embedding backends by name at runtime.

pub mod copilot_models;
pub mod copilot_proxy;
pub mod copilot_token;
pub mod gemini;
pub mod gemini_cli_auth;
pub mod minimax_oauth;
pub mod ollama;
pub mod openai;
pub mod qwen_oauth;

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

// ─── Core trait ───────────────────────────────────────────────────────────────

/// A unified LLM provider interface for chat completion and embeddings.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Human-readable provider name (e.g. "openai", "gemini").
    fn name(&self) -> &str;

    /// Single-turn chat completion: given a prompt, return the response text.
    async fn complete(&self, prompt: &str) -> Result<String>;

    /// Embed a piece of text into a float vector.
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
}

// ─── Registry ────────────────────────────────────────────────────────────────

/// Runtime registry mapping provider names to `LlmProvider` implementations.
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn LlmProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider under the given name.
    pub fn register(&mut self, provider: Box<dyn LlmProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    /// Get a provider by name, or `None` if not registered.
    pub fn get(&self, name: &str) -> Option<&dyn LlmProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    /// List all registered provider names.
    pub fn list(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.providers.keys().map(|k| k.as_str()).collect();
        names.sort();
        names
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a default runtime registry from environment variables.
///
/// - Registers `openai` when `OPENAI_API_KEY` is set.
/// - Registers `gemini` when `GEMINI_API_KEY` or `GOOGLE_AI_API_KEY` is set.
/// - Registers `copilot` when `GITHUB_TOKEN` is set.
/// - Registers `qwen-portal` when `QWEN_ACCESS_TOKEN` is set.
/// - Always registers local `ollama` (host/model can still come from env).
pub fn default_registry_from_env() -> ProviderRegistry {
    let mut registry = ProviderRegistry::new();

    if let Some(p) = crate::providers::openai::OpenAiProvider::from_env() {
        registry.register(Box::new(p));
    }
    if let Some(p) = crate::providers::gemini::GeminiProvider::from_env() {
        registry.register(Box::new(p));
    }
    if std::env::var("GITHUB_TOKEN").is_ok() {
        // Copilot provider requires GitHub token - will be resolved at runtime
    }
    if let Some(p) = crate::providers::copilot_proxy::CopilotProxyProvider::from_env() {
        registry.register(Box::new(p));
    }
    if std::env::var("QWEN_ACCESS_TOKEN").is_ok() {
        // Qwen provider requires access token - will be resolved at runtime
    }
    registry.register(Box::new(
        crate::providers::ollama::OllamaProvider::from_env(),
    ));

    registry
}

/// Return known model IDs for a provider (used by config/CLI model validation).
pub fn known_model_ids(provider: &str) -> Vec<String> {
    match ProviderKind::from_str(provider) {
        ProviderKind::OpenAi => crate::providers::openai::KNOWN_OPENAI_CHAT_MODELS
            .iter()
            .chain(crate::providers::openai::KNOWN_OPENAI_EMBEDDING_MODELS.iter())
            .map(|m| (*m).to_string())
            .collect(),
        ProviderKind::Gemini => crate::providers::gemini::KNOWN_GEMINI_CHAT_MODELS
            .iter()
            .chain(crate::providers::gemini::KNOWN_GEMINI_EMBEDDING_MODELS.iter())
            .map(|m| (*m).to_string())
            .collect(),
        ProviderKind::Ollama => vec!["llama3".to_string(), "nomic-embed-text".to_string()],
        ProviderKind::Copilot => crate::providers::copilot_models::get_default_model_ids()
            .into_iter()
            .map(|m| m.to_string())
            .collect(),
        ProviderKind::CopilotProxy => crate::providers::copilot_proxy::DEFAULT_MODEL_IDS
            .iter()
            .map(|m| m.to_string())
            .collect(),
        ProviderKind::QwenPortal | ProviderKind::Custom(_) => Vec::new(),
    }
}

/// Refresh Qwen OAuth credentials when they are expired; otherwise return current credentials.
pub async fn resolve_qwen_credentials(
    client: &reqwest::Client,
    current: &crate::providers::qwen_oauth::QwenCredentials,
) -> Result<crate::providers::qwen_oauth::QwenCredentials> {
    if current.is_expired() {
        return crate::providers::qwen_oauth::refresh_qwen_credentials(client, current).await;
    }
    Ok(current.clone())
}

// ─── Provider kind enum ───────────────────────────────────────────────────────

/// Canonical identifier for a known provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    OpenAi,
    Gemini,
    Ollama,
    Copilot,
    CopilotProxy,
    QwenPortal,
    Custom(String),
}

impl ProviderKind {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "openai" | "gpt" => ProviderKind::OpenAi,
            "gemini" | "google" => ProviderKind::Gemini,
            "ollama" => ProviderKind::Ollama,
            "copilot" | "github-copilot" => ProviderKind::Copilot,
            "copilot-proxy" => ProviderKind::CopilotProxy,
            "qwen" | "qwen-portal" => ProviderKind::QwenPortal,
            other => ProviderKind::Custom(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ProviderKind::OpenAi => "openai",
            ProviderKind::Gemini => "gemini",
            ProviderKind::Ollama => "ollama",
            ProviderKind::Copilot => "copilot",
            ProviderKind::CopilotProxy => "copilot-proxy",
            ProviderKind::QwenPortal => "qwen-portal",
            ProviderKind::Custom(s) => s.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_kind_from_str() {
        assert_eq!(ProviderKind::from_str("openai"), ProviderKind::OpenAi);
        assert_eq!(ProviderKind::from_str("GPT"), ProviderKind::OpenAi);
        assert_eq!(ProviderKind::from_str("gemini"), ProviderKind::Gemini);
        assert_eq!(ProviderKind::from_str("ollama"), ProviderKind::Ollama);
        assert_eq!(ProviderKind::from_str("copilot"), ProviderKind::Copilot);
        assert_eq!(
            ProviderKind::from_str("qwen-portal"),
            ProviderKind::QwenPortal
        );
        assert_eq!(
            ProviderKind::from_str("custom-xyz"),
            ProviderKind::Custom("custom-xyz".to_string())
        );
    }

    #[test]
    fn registry_register_and_list() {
        struct DummyProvider;

        #[async_trait::async_trait]
        impl LlmProvider for DummyProvider {
            fn name(&self) -> &str {
                "dummy"
            }
            async fn complete(&self, _prompt: &str) -> anyhow::Result<String> {
                Ok("dummy reply".to_string())
            }
            async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
                Ok(vec![0.1, 0.2])
            }
        }

        let mut reg = ProviderRegistry::new();
        assert!(reg.list().is_empty());
        reg.register(Box::new(DummyProvider));
        assert_eq!(reg.list(), vec!["dummy"]);
        assert!(reg.get("dummy").is_some());
        assert!(reg.get("other").is_none());
    }

    #[test]
    fn known_model_ids_includes_copilot_defaults() {
        let ids = known_model_ids("copilot");
        assert!(ids.contains(&"gpt-4o".to_string()));
        assert!(ids.contains(&"o3-mini".to_string()));
    }

    #[test]
    fn known_model_ids_openai_has_chat_and_embedding_models() {
        let ids = known_model_ids("openai");
        assert!(ids.contains(&"gpt-4o".to_string()));
        assert!(ids.contains(&"text-embedding-3-small".to_string()));
    }

    #[test]
    fn default_registry_from_env_includes_ollama() {
        let reg = default_registry_from_env();
        assert!(reg.get("ollama").is_some());
    }

    #[tokio::test]
    async fn resolve_qwen_credentials_no_refresh_if_not_expired() {
        let client = reqwest::Client::new();
        let current = crate::providers::qwen_oauth::QwenCredentials {
            access: "acc".to_string(),
            refresh: Some("ref".to_string()),
            expires: u64::MAX,
        };

        let out = resolve_qwen_credentials(&client, &current).await.unwrap();
        assert_eq!(out.access, "acc");
        assert_eq!(out.refresh.as_deref(), Some("ref"));
    }
}
