//! Llama.cpp local provider — chat completion + text embeddings via OpenAI-compatible API.

use super::LlmProvider;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

// ─── Request / Response types ─────────────────────────────────────────────────

#[derive(Serialize)]
#[allow(dead_code)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    input: &'a str,
    model: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

// ─── Provider implementation ──────────────────────────────────────────────────

/// Llama.cpp local LLM provider.
/// Connects to a llama-server instance using its OpenAI-compatible API endpoints.
pub struct LlamaCppProvider {
    base_url: String,
    chat_model: String,
    embedding_model: String,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl LlamaCppProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            // llama.cpp often ignores the model name if only one is loaded
            chat_model: "local-model".to_string(),
            embedding_model: "local-model".to_string(),
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

    /// Try to build from the `LLAMA_CPP_HOST` environment variable.
    pub fn from_env() -> Self {
        let host = std::env::var("LLAMA_CPP_HOST").unwrap_or_else(|_| "http://localhost:8080/v1".to_string());
        let chat_model = std::env::var("LLAMA_CPP_CHAT_MODEL").unwrap_or_else(|_| "local-model".to_string());
        let embed_model = std::env::var("LLAMA_CPP_EMBED_MODEL").unwrap_or_else(|_| "local-model".to_string());
        Self::with_models(host, chat_model, embed_model)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl LlmProvider for LlamaCppProvider {
    fn name(&self) -> &str {
        "llama-cpp"
    }

    async fn complete(&self, prompt: &str) -> Result<String> {
        let body = json!({
            "model": self.chat_model,
            "messages": [{"role": "user", "content": prompt}]
        });

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let parsed: ChatCompletionResponse = resp.json().await?;
        let text = parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .unwrap_or_default();
        Ok(text)
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let body = EmbeddingRequest {
            input: text,
            model: &self.embedding_model,
        };

        let url = format!("{}/embeddings", self.base_url.trim_end_matches('/'));

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let parsed: EmbeddingResponse = resp.json().await?;
        let vec = parsed
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .unwrap_or_default();
        Ok(vec)
    }
}

/// Check if a llama.cpp server is reachable at the given base URL.
pub async fn probe_llama_cpp(base_url: &str) -> bool {
    let client = reqwest::Client::new();
    client
        .get(&format!("{}/models", base_url.trim_end_matches('/')))
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
        let p = LlamaCppProvider::new("http://localhost:8080/v1");
        assert_eq!(p.name(), "llama-cpp");
        assert_eq!(p.base_url(), "http://localhost:8080/v1");
    }

    #[test]
    fn from_env_defaults() {
        let p = LlamaCppProvider::from_env();
        assert!(p.base_url().starts_with("http://"));
    }
}
