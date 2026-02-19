use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAnalysis {
    pub description: String,
    pub labels: Vec<String>,
    pub transcript: Option<String>,
    pub language: Option<String>,
    pub confidence: f32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUnderstandingOutput {
    pub kind: String,
    pub text: String,
    pub attachment_index: usize,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub confidence: Option<f32>,
}

impl MediaUnderstandingOutput {
    pub fn image_description(text: String, attachment_index: usize) -> Self {
        Self {
            kind: "image.description".to_string(),
            text,
            attachment_index,
            provider: None,
            model: None,
            confidence: None,
        }
    }

    pub fn audio_transcription(text: String, attachment_index: usize) -> Self {
        Self {
            kind: "audio.transcription".to_string(),
            text,
            attachment_index,
            provider: None,
            model: None,
            confidence: None,
        }
    }

    pub fn video_description(text: String, attachment_index: usize) -> Self {
        Self {
            kind: "video.description".to_string(),
            text,
            attachment_index,
            provider: None,
            model: None,
            confidence: None,
        }
    }

    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Some(confidence);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUnderstandingDecision {
    pub capability: String,
    pub outcome: MediaUnderstandingOutcome,
    pub attachments: Vec<AttachmentDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaUnderstandingOutcome {
    Success,
    Skipped,
    Disabled,
    NoAttachment,
    ScopeDeny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentDecision {
    pub attachment_index: usize,
    pub attempts: Vec<ModelDecision>,
    pub chosen: Option<ModelDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub outcome: String,
    pub reason: Option<String>,
}

impl ModelDecision {
    pub fn success_provider(provider: &str, model: Option<&str>) -> Self {
        Self {
            decision_type: "provider".to_string(),
            provider: Some(provider.to_string()),
            model: model.map(|m| m.to_string()),
            outcome: "success".to_string(),
            reason: None,
        }
    }

    pub fn skipped(reason: &str) -> Self {
        Self {
            decision_type: "provider".to_string(),
            provider: None,
            model: None,
            outcome: "skipped".to_string(),
            reason: Some(reason.to_string()),
        }
    }

    pub fn failed(reason: &str) -> Self {
        Self {
            decision_type: "provider".to_string(),
            provider: None,
            model: None,
            outcome: "failed".to_string(),
            reason: Some(reason.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaCapability {
    Image,
    Audio,
    Video,
}

impl std::fmt::Display for MediaCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaCapability::Image => write!(f, "image"),
            MediaCapability::Audio => write!(f, "audio"),
            MediaCapability::Video => write!(f, "video"),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum MediaUnderstandingError {
    #[error("No provider available for {0}")]
    NoProvider(String),

    #[error("Attachment {0} not found")]
    AttachmentNotFound(usize),

    #[error("Failed to fetch media: {0}")]
    FetchError(String),

    #[error("Media too large: {0} bytes")]
    TooLarge(usize),

    #[error("Timeout while processing")]
    Timeout,

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Empty response from provider")]
    EmptyResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_understanding_output_image() {
        let output = MediaUnderstandingOutput::image_description("A cat".to_string(), 0);
        assert_eq!(output.kind, "image.description");
        assert_eq!(output.text, "A cat");
        assert_eq!(output.attachment_index, 0);
    }

    #[test]
    fn test_media_understanding_output_audio() {
        let output = MediaUnderstandingOutput::audio_transcription("Hello world".to_string(), 1);
        assert_eq!(output.kind, "audio.transcription");
        assert_eq!(output.text, "Hello world");
    }

    #[test]
    fn test_model_decision_success() {
        let decision = ModelDecision::success_provider("openai", Some("gpt-4o"));
        assert_eq!(decision.decision_type, "provider");
        assert_eq!(decision.provider, Some("openai".to_string()));
        assert_eq!(decision.outcome, "success");
    }
}
