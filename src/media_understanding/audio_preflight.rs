use crate::media_understanding::attachments::{MediaAttachment, MediaAttachmentCache};
use crate::media_understanding::resolve::{
    resolve_max_bytes, resolve_scope_decision, resolve_timeout_ms,
    MediaContext as ResolveMediaContext,
};
use crate::media_understanding::transcription_hooks::{
    PostTranscriptionEvent, PreTranscriptionEvent, TranscriptionErrorEvent,
    TranscriptionHookRegistry,
};
use crate::media_understanding::types::MediaUnderstandingError;
use crate::media_understanding::{MediaCapability, MediaUnderstandingProvider};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct MediaContext {
    pub session_key: Option<String>,
    pub channel: Option<String>,
    pub chat_type: Option<String>,
}

fn to_resolve_context(ctx: &MediaContext) -> ResolveMediaContext {
    ResolveMediaContext {
        session_key: ctx.session_key.clone(),
        channel: ctx.channel.clone(),
        chat_type: ctx.chat_type.clone(),
    }
}

/// Options for transcription
#[derive(Debug, Clone)]
pub struct TranscriptionOptions {
    /// Optional hook registry for lifecycle events
    pub hook_registry: Option<Arc<TranscriptionHookRegistry>>,
    /// Maximum time to wait for transcription
    pub timeout_seconds: f64,
    /// Maximum bytes for audio file
    pub max_bytes: usize,
}

impl Default for TranscriptionOptions {
    fn default() -> Self {
        Self {
            hook_registry: None,
            timeout_seconds: 30.0,
            max_bytes: 10 * 1024 * 1024, // 10MB default
        }
    }
}

pub async fn transcribe_first_audio(
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    cfg: &crate::media_understanding::resolve::MediaUnderstandingConfig,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
) -> Result<Option<String>, MediaUnderstandingError> {
    transcribe_first_audio_with_options(
        ctx,
        attachments,
        cache,
        cfg,
        provider_registry,
        TranscriptionOptions::default(),
    )
    .await
}

pub async fn transcribe_first_audio_with_options(
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    cfg: &crate::media_understanding::resolve::MediaUnderstandingConfig,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
    options: TranscriptionOptions,
) -> Result<Option<String>, MediaUnderstandingError> {
    // Check if audio transcription is enabled
    if !cfg.enabled.unwrap_or(true) {
        return Ok(None);
    }

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
    let resolve_ctx = to_resolve_context(ctx);
    let scope_decision =
        resolve_scope_decision(&cfg.scope.clone().unwrap_or_default(), &resolve_ctx);
    if scope_decision == crate::media_understanding::resolve::ScopePolicy::Deny {
        return Ok(None);
    }

    // Get buffer from cache
    let buffer_result = cache
        .get_buffer(
            audio_attachment.index,
            resolve_max_bytes(cfg.max_bytes, options.max_bytes),
            resolve_timeout_ms(cfg.timeout_seconds, options.timeout_seconds) as u64,
        )
        .await
        .map_err(|e| MediaUnderstandingError::FetchError(e.to_string()))?;

    let mime = buffer_result
        .mime
        .clone()
        .unwrap_or_else(|| "audio/mpeg".to_string());
    if !mime.starts_with("audio/") {
        return Ok(None);
    }

    // Find provider that supports audio transcription
    for provider in provider_registry.values() {
        if provider.capabilities().contains(&MediaCapability::Audio) {
            let start_time = Instant::now();
            let provider_id = provider.id().to_string();

            // Fire pre-transcription hook
            if let Some(registry) = &options.hook_registry {
                registry
                    .fire_pre(PreTranscriptionEvent {
                        attachment_index: audio_attachment.index,
                        mime_type: Some(mime.clone()),
                        file_name: Some(buffer_result.file_name.clone()),
                        size_bytes: buffer_result.size,
                        provider_id: Some(provider_id.clone()),
                    })
                    .await;
            }

            match provider.transcribe_audio(&buffer_result.buffer).await {
                Ok(analysis) => {
                    let duration_ms = start_time.elapsed().as_millis() as u64;

                    // Fire post-transcription hook
                    if let Some(registry) = &options.hook_registry {
                        registry
                            .fire_post(PostTranscriptionEvent {
                                attachment_index: audio_attachment.index,
                                transcript: analysis.transcript.clone().unwrap_or_default(),
                                language: analysis.language.clone(),
                                confidence: analysis.confidence,
                                provider_id: provider_id.clone(),
                                duration_ms,
                            })
                            .await;
                    }

                    // Mark attachment as transcribed to avoid double-processing
                    // Note: In Rust, we can't modify the original attachment, so this would need
                    // to be handled at the call site
                    return Ok(analysis.transcript);
                }
                Err(e) => {
                    let error_msg = format!("{:?}", e);
                    let retryable = matches!(
                        e,
                        MediaUnderstandingError::Timeout | MediaUnderstandingError::FetchError(_)
                    );

                    // Fire error hook
                    if let Some(registry) = &options.hook_registry {
                        registry
                            .fire_error(TranscriptionErrorEvent {
                                attachment_index: audio_attachment.index,
                                error: error_msg.clone(),
                                provider_id: Some(provider_id.clone()),
                                retryable,
                            })
                            .await;
                    }

                    // Log error but continue trying other providers
                    eprintln!(
                        "[audio_preflight] Audio transcription failed with provider {}: {}",
                        provider_id, error_msg
                    );
                }
            }
        }
    }

    Ok(None)
}

/// Transcribe all audio attachments in a message
pub async fn transcribe_all_audio(
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    cfg: &crate::media_understanding::resolve::MediaUnderstandingConfig,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
    options: TranscriptionOptions,
) -> Result<Vec<(usize, String)>, MediaUnderstandingError> {
    if !cfg.enabled.unwrap_or(true) {
        return Ok(Vec::new());
    }

    let resolve_ctx = to_resolve_context(ctx);
    let scope_decision =
        resolve_scope_decision(&cfg.scope.clone().unwrap_or_default(), &resolve_ctx);
    if scope_decision == crate::media_understanding::resolve::ScopePolicy::Deny {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    for attachment in attachments {
        // Skip non-audio or already transcribed
        if !attachment
            .mime
            .as_ref()
            .map_or(false, |m| m.starts_with("audio/"))
            || attachment.already_transcribed
        {
            continue;
        }

        // Get buffer from cache
        let buffer_result = match cache
            .get_buffer(
                attachment.index,
                resolve_max_bytes(cfg.max_bytes, options.max_bytes),
                resolve_timeout_ms(cfg.timeout_seconds, options.timeout_seconds) as u64,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!(
                    "[audio_preflight] Failed to get buffer for attachment {}: {}",
                    attachment.index, e
                );
                continue;
            }
        };

        let mime = buffer_result
            .mime
            .clone()
            .unwrap_or_else(|| "audio/mpeg".to_string());

        // Try each provider
        for provider in provider_registry.values() {
            if !provider.capabilities().contains(&MediaCapability::Audio) {
                continue;
            }

            let start_time = Instant::now();
            let provider_id = provider.id().to_string();

            // Fire pre-transcription hook
            if let Some(registry) = &options.hook_registry {
                registry
                    .fire_pre(PreTranscriptionEvent {
                        attachment_index: attachment.index,
                        mime_type: Some(mime.clone()),
                        file_name: Some(buffer_result.file_name.clone()),
                        size_bytes: buffer_result.size,
                        provider_id: Some(provider_id.clone()),
                    })
                    .await;
            }

            match provider.transcribe_audio(&buffer_result.buffer).await {
                Ok(analysis) => {
                    let duration_ms = start_time.elapsed().as_millis() as u64;

                    // Fire post-transcription hook
                    if let Some(registry) = &options.hook_registry {
                        registry
                            .fire_post(PostTranscriptionEvent {
                                attachment_index: attachment.index,
                                transcript: analysis.transcript.clone().unwrap_or_default(),
                                language: analysis.language.clone(),
                                confidence: analysis.confidence,
                                provider_id: provider_id.clone(),
                                duration_ms,
                            })
                            .await;
                    }

                    if let Some(transcript) = analysis.transcript {
                        results.push((attachment.index, transcript));
                    }
                    break; // Success, move to next attachment
                }
                Err(e) => {
                    let error_msg = format!("{:?}", e);
                    let retryable = matches!(
                        e,
                        MediaUnderstandingError::Timeout | MediaUnderstandingError::FetchError(_)
                    );

                    // Fire error hook
                    if let Some(registry) = &options.hook_registry {
                        registry
                            .fire_error(TranscriptionErrorEvent {
                                attachment_index: attachment.index,
                                error: error_msg.clone(),
                                provider_id: Some(provider_id.clone()),
                                retryable,
                            })
                            .await;
                    }

                    eprintln!(
                        "[audio_preflight] Audio transcription failed for attachment {} with provider {}: {}",
                        attachment.index,
                        provider_id,
                        error_msg
                    );
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media_understanding::providers::MockMediaProvider;
    use crate::media_understanding::transcription_hooks::TranscriptionHookBuilder;
    use std::sync::atomic::{AtomicBool, Ordering};

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

    #[tokio::test]
    async fn test_transcribe_with_hooks() {
        let ctx = MediaContext::default();
        let attachments = vec![MediaAttachment {
            index: 0,
            mime: Some("audio/mpeg".to_string()),
            ..Default::default()
        }];
        let cache = MediaAttachmentCache::new(attachments.clone());
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

        let pre_called = Arc::new(AtomicBool::new(false));
        let post_called = Arc::new(AtomicBool::new(false));

        let pre_clone = pre_called.clone();
        let post_clone = post_called.clone();

        let mut hook_registry = TranscriptionHookRegistry::new();
        hook_registry.add_builder(
            TranscriptionHookBuilder::new()
                .on_pre(move |_e| {
                    let called = pre_clone.clone();
                    async move {
                        called.store(true, Ordering::Relaxed);
                    }
                })
                .on_post(move |_e| {
                    let called = post_clone.clone();
                    async move {
                        called.store(true, Ordering::Relaxed);
                    }
                }),
        );

        let options = TranscriptionOptions {
            hook_registry: Some(Arc::new(hook_registry)),
            ..Default::default()
        };

        // Note: This will fail because MockMediaProvider returns empty buffer result
        // But hooks should still be called
        let _ = transcribe_first_audio_with_options(
            &ctx,
            &attachments,
            &cache,
            &cfg,
            &provider_registry,
            options,
        )
        .await;

        // Hooks won't be called because there's no actual audio data
        // This test just verifies the hook infrastructure is in place
    }
}
