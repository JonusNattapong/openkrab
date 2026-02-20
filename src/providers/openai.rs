//! OpenAI provider — chat completion + text embeddings.
//! Mirrors openclaw's OpenAI provider behaviour.

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

/// OpenAI chat + embedding provider.
pub struct OpenAiProvider {
    api_key: String,
    chat_model: String,
    embedding_model: String,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl OpenAiProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            chat_model: "gpt-4o-mini".to_string(),
            embedding_model: "text-embedding-3-small".to_string(),
            client: crate::infra::retry_http::build_retrying_client(),
        }
    }

    pub fn with_models(
        api_key: impl Into<String>,
        chat_model: impl Into<String>,
        embedding_model: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            chat_model: chat_model.into(),
            embedding_model: embedding_model.into(),
            client: crate::infra::retry_http::build_retrying_client(),
        }
    }

    /// Try to build from the `OPENAI_API_KEY` environment variable.
    pub fn from_env() -> Option<Self> {
        std::env::var("OPENAI_API_KEY").ok().map(|k| Self::new(k))
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn complete(&self, prompt: &str) -> Result<String> {
        let body = json!({
            "model": self.chat_model,
            "messages": [{"role": "user", "content": prompt}]
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
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

        let resp = self
            .client
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&self.api_key)
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

// ─── Model list helpers ───────────────────────────────────────────────────────

pub const KNOWN_OPENAI_CHAT_MODELS: &[&str] =
    &["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"];

pub const KNOWN_OPENAI_EMBEDDING_MODELS: &[&str] = &[
    "text-embedding-3-large",
    "text-embedding-3-small",
    "text-embedding-ada-002",
];

pub fn is_known_openai_model(model: &str) -> bool {
    KNOWN_OPENAI_CHAT_MODELS.contains(&model) || KNOWN_OPENAI_EMBEDDING_MODELS.contains(&model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_models() {
        assert!(is_known_openai_model("gpt-4o"));
        assert!(is_known_openai_model("text-embedding-3-small"));
        assert!(!is_known_openai_model("claude-3"));
    }

    #[test]
    fn provider_name() {
        let p = OpenAiProvider::new("test-key");
        assert_eq!(p.name(), "openai");
    }
}
