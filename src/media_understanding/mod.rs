//! media_understanding — AI-powered media analysis abstraction.
//! Ported from `openclaw/src/media-understanding/` (Phase 7).
//!
//! Provides a unified trait for analysing image, audio and video content
//! using an underlying vision / multimodal LLM.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ─── Analysis result ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAnalysis {
    /// Natural-language description of the media.
    pub description: String,
    /// Detected labels / tags (e.g. "cat", "outdoor", "sunset").
    pub labels: Vec<String>,
    /// Transcribed text (for images with text, audio, video).
    pub transcript: Option<String>,
    /// Detected language (ISO 639-1, if applicable).
    pub language: Option<String>,
    /// Confidence score 0.0–1.0.
    pub confidence: f32,
    /// Source modality that was analysed.
    pub modality: MediaModality,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaModality {
    Image,
    Audio,
    Video,
    Document,
}

// ─── Provider trait ───────────────────────────────────────────────────────────

#[async_trait]
pub trait MediaUnderstandingProvider: Send + Sync {
    fn name(&self) -> &str;

    /// Analyse an image given its URL or base64 data URI.
    async fn analyse_image(
        &self,
        image_url_or_data: &str,
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis>;

    /// Transcribe audio from a URL.
    async fn transcribe_audio(&self, audio_url: &str) -> Result<MediaAnalysis>;
}

// ─── OpenAI vision provider ───────────────────────────────────────────────────

pub struct OpenAiVisionProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiVisionProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: "gpt-4o".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Option<Self> {
        std::env::var("OPENAI_API_KEY").ok().map(|k| Self::new(k))
    }
}

#[async_trait]
impl MediaUnderstandingProvider for OpenAiVisionProvider {
    fn name(&self) -> &str {
        "openai-vision"
    }

    async fn analyse_image(
        &self,
        image_url_or_data: &str,
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis> {
        let prompt_text = prompt.unwrap_or(
            "Describe this image concisely. List the main objects, scene, and any text present.",
        );

        let image_content = if image_url_or_data.starts_with("data:") {
            serde_json::json!({ "type": "image_url", "image_url": { "url": image_url_or_data } })
        } else {
            serde_json::json!({ "type": "image_url", "image_url": { "url": image_url_or_data } })
        };

        let body = serde_json::json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": prompt_text },
                    image_content
                ]
            }],
            "max_tokens": 512
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let json: serde_json::Value = resp.json().await?;
        let description = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(MediaAnalysis {
            description,
            labels: Vec::new(),
            transcript: None,
            language: None,
            confidence: 1.0,
            modality: MediaModality::Image,
        })
    }

    async fn transcribe_audio(&self, audio_url: &str) -> Result<MediaAnalysis> {
        // Download audio then call Whisper
        let audio_bytes = self.client.get(audio_url).send().await?.bytes().await?;

        let part = reqwest::multipart::Part::bytes(audio_bytes.to_vec())
            .file_name("audio.mp3")
            .mime_str("audio/mpeg")?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "whisper-1");

        let resp = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        let json: serde_json::Value = resp.json().await?;
        let text = json["text"].as_str().unwrap_or("").to_string();

        Ok(MediaAnalysis {
            description: format!("Audio transcript: {}", text),
            labels: Vec::new(),
            transcript: Some(text),
            language: json["language"].as_str().map(|s| s.to_string()),
            confidence: 1.0,
            modality: MediaModality::Audio,
        })
    }
}

// ─── Mock provider (for testing) ─────────────────────────────────────────────

pub struct MockMediaProvider;

#[async_trait]
impl MediaUnderstandingProvider for MockMediaProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn analyse_image(
        &self,
        _image_url_or_data: &str,
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis> {
        Ok(MediaAnalysis {
            description: "A test image containing various objects.".to_string(),
            labels: vec!["test".to_string(), "mock".to_string()],
            transcript: None,
            language: None,
            confidence: 0.95,
            modality: MediaModality::Image,
        })
    }

    async fn transcribe_audio(&self, _audio_url: &str) -> Result<MediaAnalysis> {
        Ok(MediaAnalysis {
            description: "Audio transcript: Hello, this is a test.".to_string(),
            labels: Vec::new(),
            transcript: Some("Hello, this is a test.".to_string()),
            language: Some("en".to_string()),
            confidence: 0.99,
            modality: MediaModality::Audio,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_image_analysis() {
        let p = MockMediaProvider;
        let result = p.analyse_image("https://example.com/img.jpg", None).await.unwrap();
        assert!(!result.description.is_empty());
        assert_eq!(result.modality, MediaModality::Image);
        assert!(result.confidence > 0.0);
    }

    #[tokio::test]
    async fn mock_audio_transcription() {
        let p = MockMediaProvider;
        let result = p.transcribe_audio("https://example.com/audio.mp3").await.unwrap();
        assert!(result.transcript.is_some());
        assert_eq!(result.modality, MediaModality::Audio);
    }

    #[test]
    fn provider_name() {
        let p = MockMediaProvider;
        assert_eq!(p.name(), "mock");
    }
}
