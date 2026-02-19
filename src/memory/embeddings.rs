use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    fn id(&self) -> &str;
    fn model(&self) -> &str;
    async fn embed_query(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            model: model.unwrap_or_else(|| "text-embedding-3-small".to_string()),
        }
    }
}

#[derive(Serialize)]
struct OpenAiEmbeddingRequest<'a> {
    model: &'a str,
    input: &'a [String],
}

#[derive(Deserialize)]
struct OpenAiEmbeddingResponse {
    data: Vec<OpenAiEmbeddingData>,
}

#[derive(Deserialize)]
struct OpenAiEmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for OpenAiProvider {
    fn id(&self) -> &str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn embed_query(&self, text: &str) -> Result<Vec<f32>> {
        let mut res = self.embed_batch(&[text.to_string()]).await?;
        Ok(res.pop().unwrap_or_default())
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/embeddings", self.base_url.trim_end_matches('/'));
        let response = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&OpenAiEmbeddingRequest {
                model: &self.model,
                input: texts,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", err_text));
        }

        let body: OpenAiEmbeddingResponse = response.json().await?;
        Ok(body.data.into_iter().map(|d| d.embedding).collect())
    }
}

pub struct GeminiProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://generativelanguage.googleapis.com/v1beta".to_string()),
            model: model.unwrap_or_else(|| "text-embedding-004".to_string()),
        }
    }

    fn model_path(&self) -> String {
        if self.model.starts_with("models/") {
            self.model.clone()
        } else {
            format!("models/{}", self.model)
        }
    }
}

#[derive(Serialize)]
struct GeminiEmbedRequest<'a> {
    content: GeminiContent<'a>,
    #[serde(rename = "taskType")]
    task_type: &'a str,
}

#[derive(Serialize)]
struct GeminiContent<'a> {
    parts: Vec<GeminiPart<'a>>,
}

#[derive(Serialize)]
struct GeminiPart<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct GeminiBatchRequest {
    requests: Vec<GeminiBatchItem>,
}

#[derive(Serialize)]
struct GeminiBatchItem {
    model: String,
    content: GeminiContentOwned,
    #[serde(rename = "taskType")]
    task_type: String,
}

#[derive(Serialize)]
struct GeminiContentOwned {
    parts: Vec<GeminiPartOwned>,
}

#[derive(Serialize)]
struct GeminiPartOwned {
    text: String,
}

#[derive(Deserialize)]
struct GeminiEmbedResponse {
    embedding: Option<GeminiEmbedding>,
}

#[derive(Deserialize)]
struct GeminiBatchResponse {
    embeddings: Option<Vec<GeminiEmbedding>>,
}

#[derive(Deserialize)]
struct GeminiEmbedding {
    values: Option<Vec<f32>>,
}

#[async_trait]
impl EmbeddingProvider for GeminiProvider {
    fn id(&self) -> &str {
        "gemini"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn embed_query(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/{}:embedContent", self.base_url.trim_end_matches('/'), self.model_path());
        let response = self.client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&GeminiEmbedRequest {
                content: GeminiContent {
                    parts: vec![GeminiPart { text }],
                },
                task_type: "RETRIEVAL_QUERY",
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow::anyhow!("Gemini API error: {}", err_text));
        }

        let body: GeminiEmbedResponse = response.json().await?;
        Ok(body.embedding.and_then(|e| e.values).unwrap_or_default())
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/{}:batchEmbedContents", self.base_url.trim_end_matches('/'), self.model_path());
        let requests = texts.iter().map(|t| GeminiBatchItem {
            model: self.model_path(),
            content: GeminiContentOwned {
                parts: vec![GeminiPartOwned { text: t.clone() }],
            },
            task_type: "RETRIEVAL_DOCUMENT".to_string(),
        }).collect();

        let response = self.client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&GeminiBatchRequest { requests })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow::anyhow!("Gemini API error: {}", err_text));
        }

        let body: GeminiBatchResponse = response.json().await?;
        Ok(body.embeddings.unwrap_or_default().into_iter().map(|e| e.values.unwrap_or_default()).collect())
    }
}

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model: model.unwrap_or_else(|| "nomic-embed-text".to_string()),
        }
    }
}

#[derive(Serialize)]
struct OllamaEmbedRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct OllamaEmbedResponse {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for OllamaProvider {
    fn id(&self) -> &str {
        "ollama"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn embed_query(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url.trim_end_matches('/'));
        let response = self.client
            .post(&url)
            .json(&OllamaEmbedRequest {
                model: &self.model,
                prompt: text,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow::anyhow!("Ollama API error: {}", err_text));
        }

        let body: OllamaEmbedResponse = response.json().await?;
        Ok(body.embedding)
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // Ollama doesn't have a batch endpoint yet, so we iterate
        let mut results = Vec::new();
        for text in texts {
            results.push(self.embed_query(text).await?);
        }
        Ok(results)
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    
    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot / (norm_a.sqrt() * norm_b.sqrt())
}

pub fn sanitize_and_normalize(vec: &mut Vec<f32>) {
    for val in vec.iter_mut() {
        if !val.is_finite() {
            *val = 0.0;
        }
    }
    
    let magnitude_sq: f32 = vec.iter().map(|v| v * v).sum();
    let magnitude = magnitude_sq.sqrt();
    
    if magnitude > 1e-10 {
        for val in vec.iter_mut() {
            *val /= magnitude;
        }
    }
}
