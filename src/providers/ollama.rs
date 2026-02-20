//! Ollama local provider — chat completion + text embeddings via Ollama API.

use super::LlmProvider;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

// ─── Response types ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}

#[derive(Deserialize)]
struct OllamaMessage {
    content: String,
}

#[derive(Deserialize)]
struct OllamaEmbedResponse {
    embedding: Vec<f32>,
}

// ─── Provider implementation ──────────────────────────────────────────────────

/// Ollama local LLM provider.
/// Connects to an Ollama server (default: http://localhost:11434).
pub struct OllamaProvider {
    base_url: String,
    chat_model: String,
    embedding_model: String,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl OllamaProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            chat_model: "llama3".to_string(),
            embedding_model: "nomic-embed-text".to_string(),
            client: crate::infra::retry_http::build_retrying_client(),
        }
    }

    pub fn with_models(
        base_url: impl Into<String>,
        chat_model: impl Into<String>,
        embedding_model: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            chat_model: chat_model.into(),
            embedding_model: embedding_model.into(),
            client: crate::infra::retry_http::build_retrying_client(),
        }
    }

    /// Build from env vars: `OLLAMA_HOST` (default localhost:11434).
    pub fn from_env() -> Self {
        let host =
            std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
        let chat_model =
            std::env::var("OLLAMA_CHAT_MODEL").unwrap_or_else(|_| "llama3".to_string());
        let embed_model =
            std::env::var("OLLAMA_EMBED_MODEL").unwrap_or_else(|_| "nomic-embed-text".to_string());
        Self::with_models(host, chat_model, embed_model)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn complete(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/chat", self.base_url);
        let body = json!({
            "model": self.chat_model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": false
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let parsed: OllamaChatResponse = resp.json().await?;
        Ok(parsed.message.content)
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url);
        let body = json!({
            "model": self.embedding_model,
            "prompt": text
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        let parsed: OllamaEmbedResponse = resp.json().await?;
        Ok(parsed.embedding)
    }
}

/// Check if an Ollama server is reachable at the given base URL.
pub async fn probe_ollama(base_url: &str) -> bool {
    let client = reqwest::Client::new();
    client
        .get(&format!("{}/api/tags", base_url))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_name() {
        let p = OllamaProvider::new("http://localhost:11434");
        assert_eq!(p.name(), "ollama");
        assert_eq!(p.base_url(), "http://localhost:11434");
    }

    #[test]
    fn from_env_defaults() {
        let p = OllamaProvider::from_env();
        assert!(p.base_url().starts_with("http://"));
    }
}
