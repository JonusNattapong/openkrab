use std::collections::HashMap;

pub const DEFAULT_MEDIA_CONCURRENCY: usize = 2;
pub const DEFAULT_MAX_BYTES: usize = 5 * 1024 * 1024; // 5MB
pub const DEFAULT_MAX_CHARS: usize = 100000;

pub fn resolve_timeout_ms(seconds: Option<f64>, fallback_seconds: f64) -> u64 {
    let value = seconds.unwrap_or(fallback_seconds);
    ((value.max(1.0) * 1000.0) as u64)
}

pub fn resolve_max_bytes(bytes: Option<usize>, fallback: usize) -> usize {
    bytes.unwrap_or(fallback).max(1)
}

pub fn resolve_max_chars(chars: Option<usize>, fallback: usize) -> usize {
    chars.unwrap_or(fallback).max(1)
}

pub fn resolve_concurrency(concurrency: Option<usize>) -> usize {
    concurrency.unwrap_or(DEFAULT_MEDIA_CONCURRENCY).max(1)
}

#[derive(Debug, Clone, Default)]
pub struct MediaUnderstandingConfig {
    pub enabled: Option<bool>,
    pub max_bytes: Option<usize>,
    pub max_chars: Option<usize>,
    pub prompt: Option<String>,
    pub timeout_seconds: Option<f64>,
    pub models: Vec<MediaModelConfig>,
    pub scope: Option<MediaUnderstandingScope>,
}

#[derive(Debug, Clone)]
pub struct MediaModelConfig {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub max_bytes: Option<usize>,
    pub max_chars: Option<usize>,
    pub prompt: Option<String>,
    pub capabilities: Option<Vec<String>>,
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

#[derive(Debug, Clone, Default)]
pub struct MediaUnderstandingScope {
    pub default_policy: ScopePolicy,
    pub rules: Vec<ScopeRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopePolicy {
    Allow,
    Deny,
}

impl Default for ScopePolicy {
    fn default() -> Self {
        ScopePolicy::Deny
    }
}

#[derive(Debug, Clone)]
pub struct ScopeRule {
    pub action: ScopePolicy,
    pub chat_type: Option<String>,
    pub channel: Option<String>,
    pub session_key: Option<String>,
}

pub fn resolve_scope_decision(scope: &MediaUnderstandingScope, ctx: &MediaContext) -> ScopePolicy {
    for rule in &scope.rules {
        let matches = true
            && rule
                .chat_type
                .as_ref()
                .map_or(true, |ct| ctx.chat_type.as_ref() == Some(ct))
            && rule
                .channel
                .as_ref()
                .map_or(true, |ch| ctx.channel.as_ref() == Some(ch))
            && rule
                .session_key
                .as_ref()
                .map_or(true, |sk| ctx.session_key.as_ref() == Some(sk));

        if matches {
            return rule.action;
        }
    }
    scope.default_policy
}

#[derive(Debug, Clone, Default)]
pub struct MediaContext {
    pub session_key: Option<String>,
    pub channel: Option<String>,
    pub chat_type: Option<String>,
}

pub const DEFAULT_IMAGE_PROMPT: &str = "Describe this image in detail. Include any text present, objects, people, scenes, and overall context.";
pub const DEFAULT_AUDIO_PROMPT: &str =
    "Transcribe this audio accurately. Include speaker identification if possible.";
pub const DEFAULT_VIDEO_PROMPT: &str =
    "Describe this video in detail. Include actions, people, scenes, and any text or dialogue.";

pub fn get_default_prompt(capability: MediaCapability) -> &'static str {
    match capability {
        MediaCapability::Image => DEFAULT_IMAGE_PROMPT,
        MediaCapability::Audio => DEFAULT_AUDIO_PROMPT,
        MediaCapability::Video => DEFAULT_VIDEO_PROMPT,
    }
}

pub fn resolve_prompt(
    capability: MediaCapability,
    prompt: Option<&str>,
    max_chars: Option<usize>,
) -> String {
    let base = prompt.unwrap_or(get_default_prompt(capability));

    if let Some(chars) = max_chars {
        if capability != MediaCapability::Audio {
            return format!("{} Respond in at most {} characters.", base, chars);
        }
    }

    base.to_string()
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub id: String,
    pub capabilities: Vec<MediaCapability>,
}

pub fn build_provider_registry() -> HashMap<String, ProviderInfo> {
    let mut registry = HashMap::new();

    registry.insert(
        "openai".to_string(),
        ProviderInfo {
            id: "openai".to_string(),
            capabilities: vec![MediaCapability::Image, MediaCapability::Audio],
        },
    );

    registry.insert(
        "anthropic".to_string(),
        ProviderInfo {
            id: "anthropic".to_string(),
            capabilities: vec![MediaCapability::Image],
        },
    );

    registry.insert(
        "gemini".to_string(),
        ProviderInfo {
            id: "gemini".to_string(),
            capabilities: vec![
                MediaCapability::Image,
                MediaCapability::Audio,
                MediaCapability::Video,
            ],
        },
    );

    registry.insert(
        "groq".to_string(),
        ProviderInfo {
            id: "groq".to_string(),
            capabilities: vec![MediaCapability::Audio],
        },
    );

    registry.insert(
        "deepgram".to_string(),
        ProviderInfo {
            id: "deepgram".to_string(),
            capabilities: vec![MediaCapability::Audio],
        },
    );

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_timeout_ms() {
        assert_eq!(resolve_timeout_ms(Some(5.0), 10.0), 5000);
        assert_eq!(resolve_timeout_ms(None, 10.0), 10000);
        assert_eq!(resolve_timeout_ms(Some(0.5), 10.0), 1000);
    }

    #[test]
    fn test_get_default_prompt() {
        assert!(get_default_prompt(MediaCapability::Image).contains("image"));
        assert!(get_default_prompt(MediaCapability::Audio).contains("Transcribe"));
        assert!(get_default_prompt(MediaCapability::Video).contains("video"));
    }

    #[test]
    fn test_build_provider_registry() {
        let registry = build_provider_registry();
        assert!(registry.contains_key("openai"));
        assert!(registry.contains_key("anthropic"));
        assert!(registry.contains_key("gemini"));
    }
}
