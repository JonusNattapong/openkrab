use crate::media_understanding::types::{MediaAnalysis, MediaUnderstandingError};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use std::collections::HashMap;

#[async_trait]
pub trait MediaUnderstandingProvider: Send + Sync {
    fn id(&self) -> &str;

    fn capabilities(&self) -> &[super::resolve::MediaCapability];

    async fn analyse_image(
        &self,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError>;

    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
    ) -> Result<MediaAnalysis, MediaUnderstandingError>;

    async fn describe_video(
        &self,
        video_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError>;
}

pub struct OpenAiVisionProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiVisionProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4o".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[async_trait]
impl MediaUnderstandingProvider for OpenAiVisionProvider {
    fn id(&self) -> &str {
        "openai"
    }

    fn capabilities(&self) -> &[super::resolve::MediaCapability] {
        &[
            super::resolve::MediaCapability::Image,
            super::resolve::MediaCapability::Audio,
        ]
    }

    async fn analyse_image(
        &self,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        let prompt_text = prompt.unwrap_or(
            "Describe this image concisely. List the main objects, scene, and any text present.",
        );

        let base64_data = STANDARD.encode(image_data);
        let image_url = format!("data:image/jpeg;base64,{}", base64_data);

        let body = serde_json::json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": [
                    { "type": "text", "text": prompt_text },
                    { "type": "image_url", "image_url": { "url": image_url } }
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
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(MediaUnderstandingError::ProviderError(err_text));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

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
            modality: super::types::MediaModality::Image,
        })
    }

    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        let part = reqwest::multipart::Part::bytes(audio_data.to_vec())
            .file_name("audio.mp3")
            .mime_str("audio/mpeg")
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "whisper-1");

        let resp = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(MediaUnderstandingError::ProviderError(err_text));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        let text = json["text"].as_str().unwrap_or("").to_string();

        Ok(MediaAnalysis {
            description: format!("Audio transcript: {}", text),
            labels: Vec::new(),
            transcript: Some(text),
            language: json["language"].as_str().map(|s| s.to_string()),
            confidence: 1.0,
            modality: super::types::MediaModality::Audio,
        })
    }

    async fn describe_video(
        &self,
        _video_data: &[u8],
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Err(MediaUnderstandingError::NoProvider("video".to_string()))
    }
}

pub struct MockMediaProvider;

#[async_trait]
impl MediaUnderstandingProvider for MockMediaProvider {
    fn id(&self) -> &str {
        "mock"
    }

    fn capabilities(&self) -> &[super::resolve::MediaCapability] {
        &[
            super::resolve::MediaCapability::Image,
            super::resolve::MediaCapability::Audio,
        ]
    }

    async fn analyse_image(
        &self,
        _image_data: &[u8],
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Ok(MediaAnalysis {
            description: "A test image containing various objects.".to_string(),
            labels: vec!["test".to_string(), "mock".to_string()],
            transcript: None,
            language: None,
            confidence: 0.95,
            modality: super::types::MediaModality::Image,
        })
    }

    async fn transcribe_audio(
        &self,
        _audio_data: &[u8],
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Ok(MediaAnalysis {
            description: "Audio transcript: Hello, this is a test.".to_string(),
            labels: Vec::new(),
            transcript: Some("Hello, this is a test.".to_string()),
            language: Some("en".to_string()),
            confidence: 0.99,
            modality: super::types::MediaModality::Audio,
        })
    }

    async fn describe_video(
        &self,
        _video_data: &[u8],
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Ok(MediaAnalysis {
            description: "A test video.".to_string(),
            labels: vec!["video".to_string()],
            transcript: None,
            language: None,
            confidence: 0.9,
            modality: super::types::MediaModality::Video,
        })
    }
}

pub struct GeminiProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com".to_string(),
            model: "gemini-1.5-flash".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    async fn generate_content(
        &self,
        parts: Vec<serde_json::Value>,
        prompt: &str,
    ) -> Result<String, MediaUnderstandingError> {
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.base_url.trim_end_matches('/'),
            self.model,
            self.api_key
        );

        let mut parts_vec = vec![serde_json::json!({"text": prompt})];
        parts_vec.extend(parts);
        let body = serde_json::json!({
            "contents": [{
                "role": "user",
                "parts": parts_vec
            }]
        });

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(MediaUnderstandingError::ProviderError(err_text));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        if text.is_empty() {
            return Err(MediaUnderstandingError::ProviderError(
                "Empty response from Gemini".to_string(),
            ));
        }

        Ok(text)
    }
}

#[async_trait]
impl MediaUnderstandingProvider for GeminiProvider {
    fn id(&self) -> &str {
        "gemini"
    }

    fn capabilities(&self) -> &[super::resolve::MediaCapability] {
        &[
            super::resolve::MediaCapability::Image,
            super::resolve::MediaCapability::Video,
        ]
    }

    async fn analyse_image(
        &self,
        image_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        let prompt_text = prompt.unwrap_or(
            "Describe this image concisely. List the main objects, scene, and any text present.",
        );

        let base64_data = STANDARD.encode(image_data);
        let mime_type = "image/jpeg"; // Assume JPEG for now

        let inline_data = serde_json::json!({
            "inline_data": {
                "mime_type": mime_type,
                "data": base64_data
            }
        });

        let description = self
            .generate_content(vec![inline_data], prompt_text)
            .await?;

        Ok(MediaAnalysis {
            description,
            labels: Vec::new(), // Gemini doesn't provide structured labels
            transcript: None,
            language: None,
            confidence: 1.0,
            modality: super::types::MediaModality::Image,
        })
    }

    async fn transcribe_audio(
        &self,
        _audio_data: &[u8],
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Err(MediaUnderstandingError::NoProvider("audio".to_string()))
    }

    async fn describe_video(
        &self,
        video_data: &[u8],
        prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        let prompt_text =
            prompt.unwrap_or("Describe this video concisely. What scenes or activities are shown?");

        let base64_data = STANDARD.encode(video_data);
        let mime_type = "video/mp4"; // Assume MP4 for now

        let inline_data = serde_json::json!({
            "inline_data": {
                "mime_type": mime_type,
                "data": base64_data
            }
        });

        let description = self
            .generate_content(vec![inline_data], prompt_text)
            .await?;

        Ok(MediaAnalysis {
            description,
            labels: Vec::new(),
            transcript: None,
            language: None,
            confidence: 1.0,
            modality: super::types::MediaModality::Video,
        })
    }
}

pub struct DeepgramProvider {
    api_key: String,
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl DeepgramProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.deepgram.com".to_string(),
            model: "nova-3".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[derive(Deserialize)]
struct DeepgramResponse {
    results: Option<DeepgramResults>,
}

#[derive(Deserialize)]
struct DeepgramResults {
    channels: Vec<DeepgramChannel>,
}

#[derive(Deserialize)]
struct DeepgramChannel {
    alternatives: Vec<DeepgramAlternative>,
}

#[derive(Deserialize)]
struct DeepgramAlternative {
    transcript: String,
}

#[async_trait]
impl MediaUnderstandingProvider for DeepgramProvider {
    fn id(&self) -> &str {
        "deepgram"
    }

    fn capabilities(&self) -> &[super::resolve::MediaCapability] {
        &[super::resolve::MediaCapability::Audio]
    }

    async fn analyse_image(
        &self,
        _image_data: &[u8],
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Err(MediaUnderstandingError::NoProvider("image".to_string()))
    }

    async fn transcribe_audio(
        &self,
        audio_data: &[u8],
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        let url = format!(
            "{}/v1/listen?model={}",
            self.base_url.trim_end_matches('/'),
            self.model
        );

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "audio/mpeg") // Assume MP3 for now
            .body(audio_data.to_vec())
            .send()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(MediaUnderstandingError::ProviderError(err_text));
        }

        let data: DeepgramResponse = resp
            .json()
            .await
            .map_err(|e| MediaUnderstandingError::ProviderError(e.to_string()))?;

        let transcript = data
            .results
            .and_then(|r| r.channels.first())
            .and_then(|c| c.alternatives.first())
            .map(|a| a.transcript.trim().to_string())
            .ok_or_else(|| {
                MediaUnderstandingError::ProviderError(
                    "No transcript in Deepgram response".to_string(),
                )
            })?;

        Ok(MediaAnalysis {
            description: format!("Audio transcript: {}", transcript),
            labels: Vec::new(),
            transcript: Some(transcript),
            language: Some("en".to_string()), // Deepgram can detect language, but we'll assume English for now
            confidence: 1.0,
            modality: super::types::MediaModality::Audio,
        })
    }

    async fn describe_video(
        &self,
        _video_data: &[u8],
        _prompt: Option<&str>,
    ) -> Result<MediaAnalysis, MediaUnderstandingError> {
        Err(MediaUnderstandingError::NoProvider("video".to_string()))
    }
}

pub fn build_provider_registry() -> HashMap<String, Box<dyn MediaUnderstandingProvider>> {
    let mut registry: HashMap<String, Box<dyn MediaUnderstandingProvider>> = HashMap::new();

    registry.insert("mock".to_string(), Box::new(MockMediaProvider));

    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        registry.insert(
            "openai".to_string(),
            Box::new(OpenAiVisionProvider::new(api_key)),
        );
    }

    if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
        registry.insert("gemini".to_string(), Box::new(GeminiProvider::new(api_key)));
    }

    if let Ok(api_key) = std::env::var("DEEPGRAM_API_KEY") {
        registry.insert(
            "deepgram".to_string(),
            Box::new(DeepgramProvider::new(api_key)),
        );
    }

    registry
}
