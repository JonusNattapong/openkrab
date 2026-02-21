use crate::media_understanding::attachments::{MediaAttachment, MediaAttachmentCache};
use crate::media_understanding::resolve::{
    resolve_max_bytes, resolve_prompt, resolve_scope_decision,
    resolve_timeout_ms, MediaContext as ResolveMediaContext,
};
use crate::media_understanding::types::MediaUnderstandingError;
use crate::media_understanding::{
    MediaCapability, MediaUnderstandingDecision, MediaUnderstandingOutput,
    MediaUnderstandingProvider,
};
use futures::future::join_all;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct ApplyMediaUnderstandingResult {
    pub outputs: Vec<MediaUnderstandingOutput>,
    pub decisions: Vec<MediaUnderstandingDecision>,
    pub applied_image: bool,
    pub applied_audio: bool,
    pub applied_video: bool,
    pub applied_file: bool,
}

pub async fn apply_media_understanding(
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
    config: &HashMap<
        MediaCapability,
        crate::media_understanding::resolve::MediaUnderstandingConfig,
    >,
) -> Result<ApplyMediaUnderstandingResult, MediaUnderstandingError> {
    let capabilities = vec![
        MediaCapability::Image,
        MediaCapability::Audio,
        MediaCapability::Video,
    ];

    let mut all_outputs = Vec::new();
    let mut all_decisions = Vec::new();
    let mut applied_image = false;
    let mut applied_audio = false;
    let mut applied_video = false;
    let mut applied_file = false;

    let tasks: Vec<_> = capabilities
        .into_iter()
        .map(|capability| async move {
            run_capability(
                capability,
                ctx,
                attachments,
                cache,
                provider_registry,
                config.get(&capability),
            )
            .await
        })
        .collect();

    let results: Vec<Result<(_, Vec<_>), _>> = join_all(tasks).await;

    for result in results {
        let (decision, outputs) = result?;
        
        match decision.capability.as_str() {
            "image" => applied_image = true,
            "audio" => applied_audio = true,
            "video" => applied_video = true,
            _ => {}
        }
        
        all_decisions.push(decision);
        all_outputs.extend(outputs);
    }

    // Basic file extraction for non image/audio/video attachments.
    for att in attachments {
        let kind = crate::media_understanding::attachments::resolve_attachment_kind(att);
        if kind == "unknown" {
            let locator = att
                .path
                .as_ref()
                .cloned()
                .or_else(|| att.url.as_ref().cloned())
                .unwrap_or_else(|| "(unknown source)".to_string());
            all_outputs.push(MediaUnderstandingOutput {
                kind: "file.extract".to_string(),
                text: format!("File attachment detected: {}", locator),
                attachment_index: att.index,
                provider: None,
                model: None,
                confidence: None,
            });
            applied_file = true;
        }
    }

    Ok(ApplyMediaUnderstandingResult {
        outputs: all_outputs,
        decisions: all_decisions,
        applied_image,
        applied_audio,
        applied_video,
        applied_file,
    })
}

async fn run_capability(
    capability: MediaCapability,
    ctx: &MediaContext,
    attachments: &[MediaAttachment],
    cache: &MediaAttachmentCache,
    provider_registry: &HashMap<String, Box<dyn MediaUnderstandingProvider>>,
    config: Option<&crate::media_understanding::resolve::MediaUnderstandingConfig>,
) -> Result<(MediaUnderstandingDecision, Vec<MediaUnderstandingOutput>), MediaUnderstandingError> {
    let default_config = crate::media_understanding::resolve::MediaUnderstandingConfig::default();
    let config = config.unwrap_or(&default_config);

    if !config.enabled.unwrap_or(false) {
        return Ok((
            MediaUnderstandingDecision {
                capability: capability.to_string(),
                outcome: crate::media_understanding::types::MediaUnderstandingOutcome::Disabled,
                attachments: Vec::new(),
            },
            Vec::new(),
        ));
    }

    // Check scope
    let resolve_ctx = to_resolve_context(ctx);
    let scope_decision = resolve_scope_decision(&config.scope.clone().unwrap_or_default(), &resolve_ctx);
    if scope_decision == crate::media_understanding::resolve::ScopePolicy::Deny {
        return Ok((
            MediaUnderstandingDecision {
                capability: capability.to_string(),
                outcome: crate::media_understanding::types::MediaUnderstandingOutcome::ScopeDeny,
                attachments: Vec::new(),
            },
            Vec::new(),
        ));
    }

    // Find matching attachments
    let matching_attachments: Vec<_> = attachments
        .iter()
        .filter(|att| match capability {
            MediaCapability::Image => att.mime.as_ref().map_or(false, |m| m.starts_with("image/")),
            MediaCapability::Audio => att.mime.as_ref().map_or(false, |m| m.starts_with("audio/")),
            MediaCapability::Video => att.mime.as_ref().map_or(false, |m| m.starts_with("video/")),
        })
        .collect();

    if matching_attachments.is_empty() {
        return Ok((
            MediaUnderstandingDecision {
                capability: capability.to_string(),
                outcome: crate::media_understanding::types::MediaUnderstandingOutcome::NoAttachment,
                attachments: Vec::new(),
            },
            Vec::new(),
        ));
    }

    let mut outputs = Vec::new();
    let mut attachment_decisions = Vec::new();

    // Process each attachment
    for attachment in matching_attachments {
        let mut attempts = Vec::new();

        // Try each provider
        for provider in provider_registry.values() {
            if !provider.capabilities().contains(&capability) {
                continue;
            }

            let buffer_result = cache
                .get_buffer(
                    attachment.index,
                    resolve_max_bytes(config.max_bytes, 5 * 1024 * 1024),
                    resolve_timeout_ms(config.timeout_seconds, 30.0) as u64,
                )
                .await
                .map_err(|e| MediaUnderstandingError::FetchError(e.to_string()))?;

            let prompt = resolve_prompt(capability, config.prompt.as_deref(), Some(500));

            let analysis_result = match capability {
                MediaCapability::Image => {
                    provider
                        .analyse_image(&buffer_result.buffer, Some(&prompt))
                        .await
                }
                MediaCapability::Audio => provider.transcribe_audio(&buffer_result.buffer).await,
                MediaCapability::Video => {
                    provider
                        .describe_video(&buffer_result.buffer, Some(&prompt))
                        .await
                }
            };

            match analysis_result {
                Ok(analysis) => {
                    let output = match capability {
                        MediaCapability::Image => MediaUnderstandingOutput::image_description(
                            analysis.description,
                            attachment.index,
                        ),
                        MediaCapability::Audio => MediaUnderstandingOutput::audio_transcription(
                            analysis.transcript.unwrap_or_default(),
                            attachment.index,
                        ),
                        MediaCapability::Video => MediaUnderstandingOutput::video_description(
                            analysis.description,
                            attachment.index,
                        ),
                    };

                    attempts.push(
                        crate::media_understanding::types::ModelDecision::success_provider(
                            provider.id(),
                            None,
                        ),
                    );
                    outputs.push(output);
                    break;
                }
                Err(e) => {
                    attempts.push(crate::media_understanding::types::ModelDecision::failed(
                        &format!("{:?}", e),
                    ));
                }
            }
        }

        attachment_decisions.push(crate::media_understanding::types::AttachmentDecision {
            attachment_index: attachment.index,
            chosen: attempts.iter().find(|a| a.outcome == "success").cloned(),
            attempts,
        });
    }

    let outcome = if outputs.is_empty() {
        crate::media_understanding::types::MediaUnderstandingOutcome::Skipped
    } else {
        crate::media_understanding::types::MediaUnderstandingOutcome::Success
    };

    Ok((
        MediaUnderstandingDecision {
            capability: capability.to_string(),
            outcome,
            attachments: attachment_decisions,
        },
        outputs,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media_understanding::providers::MockMediaProvider;

    #[tokio::test]
    async fn test_apply_media_understanding_no_attachments() {
        let ctx = MediaContext::default();
        let attachments = Vec::new();
        let cache = MediaAttachmentCache::new(attachments);
        let config = HashMap::new();
        let mut provider_registry = HashMap::new();
        let provider = MockMediaProvider;
        provider_registry.insert(
            "mock".to_string(),
            Box::new(provider) as Box<dyn MediaUnderstandingProvider>,
        );

        let result =
            apply_media_understanding(&ctx, &[], &cache, &provider_registry, &config).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.outputs.len(), 0);
        assert_eq!(result.decisions.len(), 3); // image, audio, video
    }
}
