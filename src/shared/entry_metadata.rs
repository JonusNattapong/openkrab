//! Entry metadata utilities for tracking entry points and execution context

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entry point type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    Cli,
    Gateway,
    Daemon,
    Web,
    Api,
    Hook,
    Cron,
    Test,
    Unknown,
}

impl Default for EntryType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Entry metadata for tracking execution context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntryMetadata {
    /// Entry point type
    #[serde(default)]
    pub entry_type: EntryType,

    /// Entry point name/command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,

    /// Version of the application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Timestamp when entry was made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// Process ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,

    /// Session ID if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// User ID if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Additional context
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub context: HashMap<String, String>,
}

impl EntryMetadata {
    /// Create new entry metadata
    pub fn new(entry_type: EntryType) -> Self {
        Self {
            entry_type,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            pid: Some(std::process::id()),
            ..Default::default()
        }
    }

    /// Create CLI entry metadata
    pub fn cli(command: impl Into<String>) -> Self {
        Self {
            entry_type: EntryType::Cli,
            entry_point: Some(command.into()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            pid: Some(std::process::id()),
            ..Default::default()
        }
    }

    /// Create gateway entry metadata
    pub fn gateway() -> Self {
        Self::new(EntryType::Gateway)
    }

    /// Create daemon entry metadata
    pub fn daemon() -> Self {
        Self::new(EntryType::Daemon)
    }

    /// Create web entry metadata
    pub fn web(endpoint: impl Into<String>) -> Self {
        let mut meta = Self::new(EntryType::Web);
        meta.entry_point = Some(endpoint.into());
        meta
    }

    /// Create API entry metadata
    pub fn api(endpoint: impl Into<String>) -> Self {
        let mut meta = Self::new(EntryType::Api);
        meta.entry_point = Some(endpoint.into());
        meta
    }

    /// Create hook entry metadata
    pub fn hook(hook_name: impl Into<String>) -> Self {
        let mut meta = Self::new(EntryType::Hook);
        meta.entry_point = Some(hook_name.into());
        meta
    }

    /// Create cron entry metadata
    pub fn cron(job_name: impl Into<String>) -> Self {
        let mut meta = Self::new(EntryType::Cron);
        meta.entry_point = Some(job_name.into());
        meta
    }

    /// Create test entry metadata
    pub fn test(test_name: impl Into<String>) -> Self {
        let mut meta = Self::new(EntryType::Test);
        meta.entry_point = Some(test_name.into());
        meta
    }

    /// Set version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set session ID
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Set user ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Add context
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Check if entry is from CLI
    pub fn is_cli(&self) -> bool {
        self.entry_type == EntryType::Cli
    }

    /// Check if entry is from gateway
    pub fn is_gateway(&self) -> bool {
        self.entry_type == EntryType::Gateway
    }

    /// Check if entry is from daemon
    pub fn is_daemon(&self) -> bool {
        self.entry_type == EntryType::Daemon
    }

    /// Check if entry is from web
    pub fn is_web(&self) -> bool {
        self.entry_type == EntryType::Web
    }

    /// Get entry type as string
    pub fn entry_type_str(&self) -> &'static str {
        match self.entry_type {
            EntryType::Cli => "cli",
            EntryType::Gateway => "gateway",
            EntryType::Daemon => "daemon",
            EntryType::Web => "web",
            EntryType::Api => "api",
            EntryType::Hook => "hook",
            EntryType::Cron => "cron",
            EntryType::Test => "test",
            EntryType::Unknown => "unknown",
        }
    }
}

/// Global entry metadata (thread-local storage)
use std::cell::RefCell;

thread_local! {
    static CURRENT_ENTRY: RefCell<Option<EntryMetadata>> = const { RefCell::new(None) };
}

/// Set current entry metadata for this thread
pub fn set_current_entry(metadata: EntryMetadata) {
    CURRENT_ENTRY.with(|e| {
        *e.borrow_mut() = Some(metadata);
    });
}

/// Get current entry metadata for this thread
pub fn get_current_entry() -> Option<EntryMetadata> {
    CURRENT_ENTRY.with(|e| e.borrow().clone())
}

/// Clear current entry metadata
pub fn clear_current_entry() {
    CURRENT_ENTRY.with(|e| {
        *e.borrow_mut() = None;
    });
}

/// Execute a function with entry metadata context
pub fn with_entry_context<T>(metadata: EntryMetadata, f: impl FnOnce() -> T) -> T {
    set_current_entry(metadata);
    let result = f();
    clear_current_entry();
    result
}

/// Entry metadata collector for aggregating multiple entries
#[derive(Debug, Default)]
pub struct EntryCollector {
    entries: Vec<EntryMetadata>,
}

impl EntryCollector {
    /// Create new collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Add entry
    pub fn add(&mut self, entry: EntryMetadata) {
        self.entries.push(entry);
    }

    /// Get all entries
    pub fn entries(&self) -> &[EntryMetadata] {
        &self.entries
    }

    /// Get entry count
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Get entries by type
    pub fn entries_by_type(&self, entry_type: EntryType) -> Vec<&EntryMetadata> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_metadata_cli() {
        let meta = EntryMetadata::cli("test command");
        assert!(meta.is_cli());
        assert!(!meta.is_gateway());
        assert_eq!(meta.entry_point, Some("test command".to_string()));
        assert!(meta.timestamp.is_some());
        assert!(meta.pid.is_some());
    }

    #[test]
    fn test_entry_metadata_gateway() {
        let meta = EntryMetadata::gateway();
        assert!(meta.is_gateway());
        assert_eq!(meta.entry_type_str(), "gateway");
    }

    #[test]
    fn test_entry_metadata_builder() {
        let meta = EntryMetadata::cli("test")
            .with_version("1.0.0")
            .with_session_id("session-123")
            .with_user_id("user-456")
            .with_context("key", "value");

        assert_eq!(meta.version, Some("1.0.0".to_string()));
        assert_eq!(meta.session_id, Some("session-123".to_string()));
        assert_eq!(meta.user_id, Some("user-456".to_string()));
        assert_eq!(meta.context.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_entry_type_str() {
        assert_eq!(EntryMetadata::cli("").entry_type_str(), "cli");
        assert_eq!(EntryMetadata::gateway().entry_type_str(), "gateway");
        assert_eq!(EntryMetadata::daemon().entry_type_str(), "daemon");
        assert_eq!(EntryMetadata::web("/").entry_type_str(), "web");
        assert_eq!(EntryMetadata::api("/v1").entry_type_str(), "api");
        assert_eq!(EntryMetadata::hook("test").entry_type_str(), "hook");
        assert_eq!(EntryMetadata::cron("daily").entry_type_str(), "cron");
        assert_eq!(EntryMetadata::test("unit").entry_type_str(), "test");
    }

    #[test]
    fn test_entry_collector() {
        let mut collector = EntryCollector::new();
        collector.add(EntryMetadata::cli("cmd1"));
        collector.add(EntryMetadata::cli("cmd2"));
        collector.add(EntryMetadata::gateway());

        assert_eq!(collector.count(), 3);
        assert_eq!(collector.entries_by_type(EntryType::Cli).len(), 2);
        assert_eq!(collector.entries_by_type(EntryType::Gateway).len(), 1);
    }

    #[test]
    fn test_serialization() {
        let meta = EntryMetadata::cli("test").with_version("1.0.0");
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("cli"));
        assert!(json.contains("1.0.0"));

        let deserialized: EntryMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.entry_type, EntryType::Cli);
        assert_eq!(deserialized.version, Some("1.0.0".to_string()));
    }
}
