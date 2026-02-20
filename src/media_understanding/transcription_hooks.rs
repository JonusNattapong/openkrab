//! transcription_hooks — Event system for audio transcription lifecycle.
//!
//! Provides hooks for:
//! - Pre-transcription: Before audio is sent to provider
//! - Post-transcription: After transcript is received
//! - On-error: When transcription fails
//! - On-cleanup: When temp files are cleaned up

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Event fired before transcription starts
#[derive(Debug, Clone)]
pub struct PreTranscriptionEvent {
    pub attachment_index: usize,
    pub mime_type: Option<String>,
    pub file_name: Option<String>,
    pub size_bytes: usize,
    pub provider_id: Option<String>,
}

/// Event fired after successful transcription
#[derive(Debug, Clone)]
pub struct PostTranscriptionEvent {
    pub attachment_index: usize,
    pub transcript: String,
    pub language: Option<String>,
    pub confidence: f32,
    pub provider_id: String,
    pub duration_ms: u64,
}

/// Event fired when transcription fails
#[derive(Debug, Clone)]
pub struct TranscriptionErrorEvent {
    pub attachment_index: usize,
    pub error: String,
    pub provider_id: Option<String>,
    pub retryable: bool,
}

/// Event fired when temp files are cleaned up
#[derive(Debug, Clone)]
pub struct CleanupEvent {
    pub file_paths: Vec<std::path::PathBuf>,
    pub reason: CleanupReason,
}

#[derive(Debug, Clone)]
pub enum CleanupReason {
    TtlExpired,
    ExplicitDelete,
    SessionEnded,
    Error,
}

/// Hook trait for transcription lifecycle events
#[async_trait::async_trait]
pub trait TranscriptionHook: Send + Sync {
    /// Called before transcription starts
    async fn on_pre_transcription(&self, _event: PreTranscriptionEvent) {}

    /// Called after successful transcription
    async fn on_post_transcription(&self, _event: PostTranscriptionEvent) {}

    /// Called when transcription fails
    async fn on_transcription_error(&self, _event: TranscriptionErrorEvent) {}

    /// Called when temp files are cleaned up
    async fn on_cleanup(&self, _event: CleanupEvent) {}
}

/// Type alias for hook callbacks
type HookCallback<T> = Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Builder for transcription hooks using callbacks
pub struct TranscriptionHookBuilder {
    pre_hook: Option<HookCallback<PreTranscriptionEvent>>,
    post_hook: Option<HookCallback<PostTranscriptionEvent>>,
    error_hook: Option<HookCallback<TranscriptionErrorEvent>>,
    cleanup_hook: Option<HookCallback<CleanupEvent>>,
}

impl Default for TranscriptionHookBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionHookBuilder {
    pub fn new() -> Self {
        Self {
            pre_hook: None,
            post_hook: None,
            error_hook: None,
            cleanup_hook: None,
        }
    }

    /// Set pre-transcription callback
    pub fn on_pre<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(PreTranscriptionEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.pre_hook = Some(Arc::new(move |event| {
            Box::pin(callback(event))
        }));
        self
    }

    /// Set post-transcription callback
    pub fn on_post<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(PostTranscriptionEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.post_hook = Some(Arc::new(move |event| {
            Box::pin(callback(event))
        }));
        self
    }

    /// Set error callback
    pub fn on_error<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(TranscriptionErrorEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.error_hook = Some(Arc::new(move |event| {
            Box::pin(callback(event))
        }));
        self
    }

    /// Set cleanup callback
    pub fn on_cleanup<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn(CleanupEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.cleanup_hook = Some(Arc::new(move |event| {
            Box::pin(callback(event))
        }));
        self
    }

    /// Build into a hook implementation
    pub fn build(self) -> Box<dyn TranscriptionHook> {
        Box::new(CallbackHook {
            pre_hook: self.pre_hook,
            post_hook: self.post_hook,
            error_hook: self.error_hook,
            cleanup_hook: self.cleanup_hook,
        })
    }
}

struct CallbackHook {
    pre_hook: Option<HookCallback<PreTranscriptionEvent>>,
    post_hook: Option<HookCallback<PostTranscriptionEvent>>,
    error_hook: Option<HookCallback<TranscriptionErrorEvent>>,
    cleanup_hook: Option<HookCallback<CleanupEvent>>,
}

#[async_trait::async_trait]
impl TranscriptionHook for CallbackHook {
    async fn on_pre_transcription(&self, event: PreTranscriptionEvent) {
        if let Some(hook) = &self.pre_hook {
            hook(event).await;
        }
    }

    async fn on_post_transcription(&self, event: PostTranscriptionEvent) {
        if let Some(hook) = &self.post_hook {
            hook(event).await;
        }
    }

    async fn on_transcription_error(&self, event: TranscriptionErrorEvent) {
        if let Some(hook) = &self.error_hook {
            hook(event).await;
        }
    }

    async fn on_cleanup(&self, event: CleanupEvent) {
        if let Some(hook) = &self.cleanup_hook {
            hook(event).await;
        }
    }
}

/// Registry for multiple transcription hooks
pub struct TranscriptionHookRegistry {
    hooks: Vec<Arc<dyn TranscriptionHook>>,
}

impl std::fmt::Debug for TranscriptionHookRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TranscriptionHookRegistry")
            .field("hooks_count", &self.hooks.len())
            .finish()
    }
}

impl Default for TranscriptionHookRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionHookRegistry {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    /// Add a hook to the registry
    pub fn add_hook(&mut self, hook: Arc<dyn TranscriptionHook>) {
        self.hooks.push(hook);
    }

    /// Add a hook from builder
    pub fn add_builder(&mut self, builder: TranscriptionHookBuilder) {
        self.hooks.push(Arc::from(builder.build()));
    }

    /// Fire pre-transcription event to all hooks
    pub async fn fire_pre(&self, event: PreTranscriptionEvent) {
        for hook in &self.hooks {
            hook.on_pre_transcription(event.clone()).await;
        }
    }

    /// Fire post-transcription event to all hooks
    pub async fn fire_post(&self, event: PostTranscriptionEvent) {
        for hook in &self.hooks {
            hook.on_post_transcription(event.clone()).await;
        }
    }

    /// Fire error event to all hooks
    pub async fn fire_error(&self, event: TranscriptionErrorEvent) {
        for hook in &self.hooks {
            hook.on_transcription_error(event.clone()).await;
        }
    }

    /// Fire cleanup event to all hooks
    pub async fn fire_cleanup(&self, event: CleanupEvent) {
        for hook in &self.hooks {
            hook.on_cleanup(event.clone()).await;
        }
    }

    /// Check if registry has any hooks
    pub fn is_empty(&self) -> bool {
        self.hooks.is_empty()
    }

    /// Get number of registered hooks
    pub fn len(&self) -> usize {
        self.hooks.len()
    }
}

/// Logging hook implementation for debugging
pub struct LoggingTranscriptionHook;

#[async_trait::async_trait]
impl TranscriptionHook for LoggingTranscriptionHook {
    async fn on_pre_transcription(&self, event: PreTranscriptionEvent) {
        eprintln!(
            "[transcription] Starting transcription for attachment {} (provider: {:?}, size: {} bytes)",
            event.attachment_index,
            event.provider_id,
            event.size_bytes
        );
    }

    async fn on_post_transcription(&self, event: PostTranscriptionEvent) {
        eprintln!(
            "[transcription] Completed transcription for attachment {} (provider: {}, duration: {}ms, confidence: {})",
            event.attachment_index,
            event.provider_id,
            event.duration_ms,
            event.confidence
        );
    }

    async fn on_transcription_error(&self, event: TranscriptionErrorEvent) {
        eprintln!(
            "[transcription] Failed for attachment {} (provider: {:?}, retryable: {}): {}",
            event.attachment_index,
            event.provider_id,
            event.retryable,
            event.error
        );
    }

    async fn on_cleanup(&self, event: CleanupEvent) {
        eprintln!(
            "[transcription] Cleaned up {} files (reason: {:?})",
            event.file_paths.len(),
            event.reason
        );
    }
}

/// Metrics hook for tracking transcription statistics
pub struct MetricsTranscriptionHook {
    total_attempts: std::sync::atomic::AtomicU64,
    total_success: std::sync::atomic::AtomicU64,
    total_errors: std::sync::atomic::AtomicU64,
    total_duration_ms: std::sync::atomic::AtomicU64,
}

impl Default for MetricsTranscriptionHook {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsTranscriptionHook {
    pub fn new() -> Self {
        Self {
            total_attempts: std::sync::atomic::AtomicU64::new(0),
            total_success: std::sync::atomic::AtomicU64::new(0),
            total_errors: std::sync::atomic::AtomicU64::new(0),
            total_duration_ms: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn get_stats(&self) -> TranscriptionStats {
        TranscriptionStats {
            total_attempts: self.total_attempts.load(std::sync::atomic::Ordering::Relaxed),
            total_success: self.total_success.load(std::sync::atomic::Ordering::Relaxed),
            total_errors: self.total_errors.load(std::sync::atomic::Ordering::Relaxed),
            total_duration_ms: self.total_duration_ms.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TranscriptionStats {
    pub total_attempts: u64,
    pub total_success: u64,
    pub total_errors: u64,
    pub total_duration_ms: u64,
}

impl TranscriptionStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.total_success as f64 / self.total_attempts as f64
        }
    }

    pub fn average_duration_ms(&self) -> f64 {
        if self.total_success == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.total_success as f64
        }
    }
}

#[async_trait::async_trait]
impl TranscriptionHook for MetricsTranscriptionHook {
    async fn on_pre_transcription(&self, _event: PreTranscriptionEvent) {
        self.total_attempts.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    async fn on_post_transcription(&self, event: PostTranscriptionEvent) {
        self.total_success.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.total_duration_ms.fetch_add(event.duration_ms, std::sync::atomic::Ordering::Relaxed);
    }

    async fn on_transcription_error(&self, _event: TranscriptionErrorEvent) {
        self.total_errors.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_hook_builder() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let hook = TranscriptionHookBuilder::new()
            .on_pre(move |_event| {
                let called = called_clone.clone();
                async move {
                    called.store(true, Ordering::Relaxed);
                }
            })
            .build();

        hook.on_pre_transcription(PreTranscriptionEvent {
            attachment_index: 0,
            mime_type: Some("audio/mp3".to_string()),
            file_name: Some("test.mp3".to_string()),
            size_bytes: 1024,
            provider_id: Some("openai".to_string()),
        }).await;

        assert!(called.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_hook_registry() {
        let mut registry = TranscriptionHookRegistry::new();
        
        let pre_called = Arc::new(AtomicBool::new(false));
        let post_called = Arc::new(AtomicBool::new(false));

        let pre_clone = pre_called.clone();
        let post_clone = post_called.clone();

        registry.add_builder(
            TranscriptionHookBuilder::new()
                .on_pre(move |_e| {
                    let called = pre_clone.clone();
                    async move { called.store(true, Ordering::Relaxed); }
                })
                .on_post(move |_e| {
                    let called = post_clone.clone();
                    async move { called.store(true, Ordering::Relaxed); }
                })
        );

        registry.fire_pre(PreTranscriptionEvent {
            attachment_index: 0,
            mime_type: None,
            file_name: None,
            size_bytes: 0,
            provider_id: None,
        }).await;

        registry.fire_post(PostTranscriptionEvent {
            attachment_index: 0,
            transcript: "test".to_string(),
            language: None,
            confidence: 1.0,
            provider_id: "test".to_string(),
            duration_ms: 100,
        }).await;

        assert!(pre_called.load(Ordering::Relaxed));
        assert!(post_called.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_metrics_hook() {
        let hook = Arc::new(MetricsTranscriptionHook::new());
        
        hook.on_pre_transcription(PreTranscriptionEvent {
            attachment_index: 0,
            mime_type: None,
            file_name: None,
            size_bytes: 0,
            provider_id: None,
        }).await;

        hook.on_post_transcription(PostTranscriptionEvent {
            attachment_index: 0,
            transcript: "test".to_string(),
            language: None,
            confidence: 1.0,
            provider_id: "test".to_string(),
            duration_ms: 150,
        }).await;

        let stats = hook.get_stats();
        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.total_success, 1);
        assert_eq!(stats.total_duration_ms, 150);
        assert_eq!(stats.success_rate(), 1.0);
    }
}
