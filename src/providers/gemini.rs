//! Gemini (Google) provider — chat completion + text embeddings.
//! Mirrors openclaw's google-shared provider behaviour.

use super::LlmProvider;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

// ─── Response types ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiEmbedResponse {
    embedding: GeminiEmbedding,
}

#[derive(Deserialize)]
struct GeminiEmbedding {
    values: Vec<f32>,
}

// ─── Provider implementation ──────────────────────────────────────────────────

pub struct GeminiProvider {
    api_key: String,
    chat_model: String,
    embedding_model: String,
    client: reqwest::Client,
}

impl GeminiProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            chat_model: "gemini-1.5-flash".to_string(),
            embedding_model: "text-embedding-004".to_string(),
            client: reqwest::Client::new(),
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
            client: reqwest::Client::new(),
        }
    }

    /// Try to build from the `GEMINI_API_KEY` (or `GOOGLE_AI_API_KEY`) env var.
    pub fn from_env() -> Option<Self> {
        let key = std::env::var("GEMINI_API_KEY")
            .or_else(|_| std::env::var("GOOGLE_AI_API_KEY"))
            .ok()?;
        Some(Self::new(key))
    }
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    async fn complete(&self, prompt: &str) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.chat_model, self.api_key
        );
        let body = json!({
            "contents": [{"parts": [{"text": prompt}]}]
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let parsed: GeminiResponse = resp.json().await?;
        let text = parsed
            .candidates
            .into_iter()
            .next()
            .and_then(|c| c.content.parts.into_iter().next())
            .and_then(|p| p.text)
            .unwrap_or_default();
        Ok(text)
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:embedContent?key={}",
            self.embedding_model, self.api_key
        );
        let body = json!({
            "model": format!("models/{}", self.embedding_model),
            "content": {"parts": [{"text": text}]}
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let parsed: GeminiEmbedResponse = resp.json().await?;
        Ok(parsed.embedding.values)
    }
}

// ─── Known models ─────────────────────────────────────────────────────────────

pub const KNOWN_GEMINI_CHAT_MODELS: &[&str] = &[
    "gemini-2.0-flash",
    "gemini-2.0-flash-lite",
    "gemini-1.5-pro",
    "gemini-1.5-flash",
    "gemini-1.5-flash-8b",
];

pub const KNOWN_GEMINI_EMBEDDING_MODELS: &[&str] =
    &["text-embedding-004", "text-multilingual-embedding-002"];

pub fn is_known_gemini_model(model: &str) -> bool {
    KNOWN_GEMINI_CHAT_MODELS.contains(&model) || KNOWN_GEMINI_EMBEDDING_MODELS.contains(&model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_models() {
        assert!(is_known_gemini_model("gemini-1.5-flash"));
        assert!(is_known_gemini_model("text-embedding-004"));
        assert!(!is_known_gemini_model("gpt-4o"));
    }

    #[test]
    fn provider_name() {
        let p = GeminiProvider::new("test-key");
        assert_eq!(p.name(), "gemini");
    }
}
