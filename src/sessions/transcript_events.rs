//! transcript_events — Session transcript update events.
//! Ported from `openclaw/src/sessions/transcript-events.ts`.
//!
//! Provides a pub/sub mechanism for notifying listeners when session
//! transcripts are updated.

use std::sync::{Arc, Mutex};

// ─── Types ────────────────────────────────────────────────────────────────────

/// An update event for a session transcript.
#[derive(Debug, Clone)]
pub struct SessionTranscriptUpdate {
    /// The session ID that was updated.
    pub session_id: String,
    /// Optional file path if transcript is persisted.
    pub session_file: Option<String>,
}

/// Listener callback type.
pub type TranscriptListener = Arc<dyn Fn(&SessionTranscriptUpdate) + Send + Sync>;

// ─── Event bus ────────────────────────────────────────────────────────────────

/// A thread-safe event bus for session transcript updates.
#[derive(Clone)]
pub struct TranscriptEventBus {
    listeners: Arc<Mutex<Vec<(usize, TranscriptListener)>>>,
    next_id: Arc<Mutex<usize>>,
}

impl Default for TranscriptEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptEventBus {
    /// Create a new event bus.
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    /// Register a listener. Returns a handle ID that can be used to unsubscribe.
    pub fn on_update(&self, listener: TranscriptListener) -> usize {
        let mut id = self.next_id.lock().unwrap();
        let handle = *id;
        *id += 1;
        self.listeners.lock().unwrap().push((handle, listener));
        handle
    }

    /// Convenience: register a closure as listener.
    pub fn subscribe<F>(&self, f: F) -> usize
    where
        F: Fn(&SessionTranscriptUpdate) + Send + Sync + 'static,
    {
        self.on_update(Arc::new(f))
    }

    /// Remove a listener by handle ID.
    pub fn unsubscribe(&self, handle: usize) {
        self.listeners
            .lock()
            .unwrap()
            .retain(|(id, _)| *id != handle);
    }

    /// Emit a transcript update event.
    pub fn emit(&self, session_id: &str, session_file: Option<&str>) {
        let trimmed = session_id.trim();
        if trimmed.is_empty() {
            return;
        }
        let update = SessionTranscriptUpdate {
            session_id: trimmed.to_string(),
            session_file: session_file.map(|s| s.to_string()),
        };
        let listeners = self.listeners.lock().unwrap().clone();
        for (_, listener) in &listeners {
            listener(&update);
        }
    }

    /// Get the number of active listeners.
    pub fn listener_count(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }
}

// ─── Global instance (optional convenience) ───────────────────────────────────

lazy_static::lazy_static! {
    static ref GLOBAL_TRANSCRIPT_BUS: TranscriptEventBus = TranscriptEventBus::new();
}

/// Get the global transcript event bus.
pub fn global_transcript_bus() -> &'static TranscriptEventBus {
    &GLOBAL_TRANSCRIPT_BUS
}

/// Subscribe to the global transcript event bus.
pub fn on_session_transcript_update<F>(listener: F) -> usize
where
    F: Fn(&SessionTranscriptUpdate) + Send + Sync + 'static,
{
    GLOBAL_TRANSCRIPT_BUS.subscribe(listener)
}

/// Emit a transcript update on the global bus.
pub fn emit_session_transcript_update(session_id: &str) {
    GLOBAL_TRANSCRIPT_BUS.emit(session_id, None);
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn subscribe_and_emit() {
        let bus = TranscriptEventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(move |update| {
            assert_eq!(update.session_id, "test-session");
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.emit("test-session", None);
        bus.emit("test-session", None);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn unsubscribe() {
        let bus = TranscriptEventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let handle = bus.subscribe(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.emit("test", None);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        bus.unsubscribe(handle);
        bus.emit("test", None);
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Should not increment
    }

    #[test]
    fn empty_session_id_ignored() {
        let bus = TranscriptEventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.emit("", None);
        bus.emit("   ", None);
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn multiple_listeners() {
        let bus = TranscriptEventBus::new();
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));

        let c1 = counter1.clone();
        bus.subscribe(move |_| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let c2 = counter2.clone();
        bus.subscribe(move |_| {
            c2.fetch_add(1, Ordering::SeqCst);
        });

        bus.emit("session-1", None);
        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn listener_count() {
        let bus = TranscriptEventBus::new();
        assert_eq!(bus.listener_count(), 0);

        let h1 = bus.subscribe(|_| {});
        assert_eq!(bus.listener_count(), 1);

        let h2 = bus.subscribe(|_| {});
        assert_eq!(bus.listener_count(), 2);

        bus.unsubscribe(h1);
        assert_eq!(bus.listener_count(), 1);

        bus.unsubscribe(h2);
        assert_eq!(bus.listener_count(), 0);
    }

    #[test]
    fn with_session_file() {
        let bus = TranscriptEventBus::new();
        let captured_file = Arc::new(Mutex::new(None::<String>));
        let captured = captured_file.clone();

        bus.subscribe(move |update| {
            *captured.lock().unwrap() = update.session_file.clone();
        });

        bus.emit("test", Some("/path/to/transcript.json"));
        assert_eq!(
            *captured_file.lock().unwrap(),
            Some("/path/to/transcript.json".to_string())
        );
    }
}
