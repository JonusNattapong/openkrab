//! media_understanding — AI-powered media analysis abstraction.
//! Ported from `openclaw/src/media-understanding/` (Phase 7).
//!
//! Provides a unified trait for analysing image, audio and video content
//! using an underlying vision / multimodal LLM.

pub mod apply;
pub mod attachments;
pub mod audio_preflight;
pub mod format;
pub mod providers;
pub mod resolve;
pub mod transcription_hooks;
pub mod types;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub use apply::{apply_media_understanding, ApplyMediaUnderstandingResult};
pub use attachments::{
    is_audio_attachment, is_image_attachment, is_video_attachment, resolve_attachment_kind,
    select_attachments, MediaAttachment, MediaAttachmentCache, MediaAttachmentError,
};
pub use audio_preflight::transcribe_first_audio;
pub use format::{get_text_stats, sanitize_mime_type, xml_escape_attr};
pub use providers::{MediaUnderstandingProvider, MockMediaProvider, OpenAiVisionProvider};
pub use resolve::{
    build_provider_registry, resolve_scope_decision, MediaCapability, MediaModelConfig,
    MediaUnderstandingConfig,
};
pub use transcription_hooks::{
    CleanupEvent, CleanupReason, LoggingTranscriptionHook, MetricsTranscriptionHook,
    PostTranscriptionEvent, PreTranscriptionEvent, TranscriptionErrorEvent, TranscriptionHook,
    TranscriptionHookBuilder, TranscriptionHookRegistry, TranscriptionStats,
};
pub use types::{
    MediaAnalysis, MediaModality, MediaUnderstandingDecision, MediaUnderstandingOutcome,
    MediaUnderstandingOutput, ModelDecision,
};

#[async_trait]
pub trait MediaUnderstandingProviderLegacy: Send + Sync {
    fn name(&self) -> &str;

    async fn analyse_image(
        &self,
        image_url_or_data: &str,
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis>;

    async fn transcribe_audio(&self, audio_url: &str) -> Result<MediaAnalysis>;
}

impl OpenAiVisionProvider {
    pub fn new_legacy(api_key: impl Into<String>) -> Self {
        Self::new(api_key.into())
    }

    pub fn from_env() -> Option<Self> {
        std::env::var("OPENAI_API_KEY").ok().map(Self::new)
    }
}

#[async_trait]
impl MediaUnderstandingProviderLegacy for OpenAiVisionProvider {
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

        let image_content =
            serde_json::json!({ "type": "image_url", "image_url": { "url": image_url_or_data } });

        let body = serde_json::json!({
            "model": "gpt-4o",
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": prompt_text },
                    image_content
                ]
            }],
            "max_tokens": 512
        });

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key())
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
        let client = reqwest::Client::new();
        let audio_bytes = client.get(audio_url).send().await?.bytes().await?;

        let part = reqwest::multipart::Part::bytes(audio_bytes.to_vec())
            .file_name("audio.mp3")
            .mime_str("audio/mpeg")?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "whisper-1");

        let resp = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_image_analysis() {
        let p = MockMediaProvider;
        let result = p.analyse_image(&[], None).await.unwrap();
        assert!(!result.description.is_empty());
        assert_eq!(result.modality, MediaModality::Image);
        assert!(result.confidence > 0.0);
    }

    #[tokio::test]
    async fn mock_audio_transcription() {
        let p = MockMediaProvider;
        let result = p.transcribe_audio(&[]).await.unwrap();
        assert!(result.transcript.is_some());
        assert_eq!(result.modality, MediaModality::Audio);
    }

    #[test]
    fn provider_name() {
        let p = MockMediaProvider;
        assert_eq!(p.id(), "mock");
    }
}
