use crate::media_understanding::attachments::{MediaAttachment, MediaAttachmentCache};
use crate::media_understanding::resolve::{
    resolve_scope_decision, resolve_max_bytes, resolve_timeout_ms, MediaContext as ResolveMediaContext,
};
use crate::media_understanding::types::MediaUnderstandingError;
use crate::media_understanding::{
    MediaCapability, MediaUnderstandingProvider,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MediaContext {
    pub session_key: Option<String>,
    pub channel: Option<String>,
    pub chat_type: Option<String>,
}

pub async fn transcribe_first_audio(
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    cfg: &crate::media_understanding::resolve::MediaUnderstandingConfig,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
) -> Result<Option<String>, MediaUnderstandingError> {
    // Check if audio transcription is enabled
    if !cfg.enabled.unwrap_or(true) {
        return Ok(None);
    }

    let audio_config = cfg; // Use same config for now

    // Find first audio attachment that hasn't been transcribed
    let first_audio = attachments.iter().find(|att| {
        att.mime
            .as_ref()
            .map_or(false, |mime| mime.starts_with("audio/"))
            && !att.already_transcribed
    });

    let audio_attachment = match first_audio {
        Some(att) => att,
        None => return Ok(None),
    };

    // Check scope decision
    let scope_decision = resolve_scope_decision(&cfg.scope.clone().unwrap_or_default(), ctx);
    if scope_decision == crate::media_understanding::resolve::ScopePolicy::Deny {
        return Ok(None);
    }

    // Get buffer from cache
    let buffer_result = cache
        .get_buffer(
            audio_attachment.index,
        resolve_max_bytes(cfg.max_bytes, 10 * 1024 * 1024), // Default 10MB
        resolve_timeout_ms(cfg.timeout_seconds, 30.0) as u64,
        )
        .await?;

    let mime = buffer_result
        .mime
        .unwrap_or_else(|| "audio/mpeg".to_string());
    if !mime.starts_with("audio/") {
        return Ok(None);
    }

    // Find provider that supports audio transcription
    for provider in provider_registry.values() {
        if provider.capabilities().contains(&MediaCapability::Audio) {
            match provider.transcribe_audio(&buffer_result.buffer).await {
                Ok(analysis) => {
                    // Mark attachment as transcribed to avoid double-processing
                    // Note: In Rust, we can't modify the original attachment, so this would need
                    // to be handled at the call site
                    return Ok(analysis.transcript);
                }
                Err(e) => {
                    // Log error but continue trying other providers
                    eprintln!(
                        "Audio transcription failed with provider {}: {:?}",
                        provider.id(),
                        e
                    );
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media_understanding::providers::MockMediaProvider;

    #[tokio::test]
    async fn test_transcribe_first_audio_no_attachments() {
        let ctx = MediaContext::default();
        let attachments = Vec::new();
        let cache = MediaAttachmentCache::new(attachments);
        let cfg = crate::media_understanding::resolve::MediaUnderstandingConfig {
            enabled: Some(true),
            max_bytes: Some(10 * 1024 * 1024),
            max_chars: Some(1000),
            prompt: Some("Transcribe audio".to_string()),
            timeout_seconds: Some(30.0),
            models: Vec::new(),
        };
        let mut provider_registry = HashMap::new();
        let provider = MockMediaProvider;
        provider_registry.insert(
            "mock".to_string(),
            Box::new(provider) as Box<dyn MediaUnderstandingProvider>,
        );

        let result = transcribe_first_audio(&ctx, &[], &cache, &cfg, &provider_registry).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_transcribe_first_audio_disabled() {
        let ctx = MediaContext::default();
        let attachments = vec![MediaAttachment {
            index: 0,
            mime: Some("audio/mpeg".to_string()),
            ..Default::default()
        }];
        let cache = MediaAttachmentCache::new(attachments.clone());
        let cfg = crate::media_understanding::resolve::MediaUnderstandingConfig {
            enabled: Some(false),
            max_bytes: Some(10 * 1024 * 1024),
            max_chars: Some(1000),
            prompt: Some("Transcribe audio".to_string()),
            timeout_seconds: Some(30.0),
            models: Vec::new(),
        };
        let provider_registry = HashMap::new();

        let result =
            transcribe_first_audio(&ctx, &attachments, &cache, &cfg, &provider_registry).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
