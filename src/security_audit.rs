//! security_audit — Comprehensive audit logging for security events.
//!
//! Tracks all security-relevant operations for forensic analysis and compliance.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Security event severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecuritySeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Types of security events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    // Plugin events
    PluginLoadAttempt,
    PluginLoadSuccess,
    PluginLoadFailure,
    PluginSignatureInvalid,
    PluginSignatureMissing,
    PluginSandboxViolation,
    PluginResourceLimitExceeded,
    
    // File system events
    FileReadAttempt,
    FileWriteAttempt,
    PathTraversalBlocked,
    
    // Command execution events
    CommandExecutionAttempt,
    CommandBlocked,
    CommandInjectionDetected,
    
    // Network events
    NetworkRequest,
    NetworkBlocked,
    
    // Authentication events
    AuthSuccess,
    AuthFailure,
    TokenValidation,
    
    // Configuration events
    ConfigReload,
    ConfigValidationFailure,
    
    // Runtime events
    Panic,
    UnsafeBlockExecuted,
    MemoryLimitExceeded,
}

impl std::fmt::Display for SecurityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityEventType::PluginLoadAttempt => write!(f, "plugin_load_attempt"),
            SecurityEventType::PluginLoadSuccess => write!(f, "plugin_load_success"),
            SecurityEventType::PluginLoadFailure => write!(f, "plugin_load_failure"),
            SecurityEventType::PluginSignatureInvalid => write!(f, "plugin_signature_invalid"),
            SecurityEventType::PluginSignatureMissing => write!(f, "plugin_signature_missing"),
            SecurityEventType::PluginSandboxViolation => write!(f, "plugin_sandbox_violation"),
            SecurityEventType::PluginResourceLimitExceeded => write!(f, "plugin_resource_limit_exceeded"),
            SecurityEventType::FileReadAttempt => write!(f, "file_read_attempt"),
            SecurityEventType::FileWriteAttempt => write!(f, "file_write_attempt"),
            SecurityEventType::PathTraversalBlocked => write!(f, "path_traversal_blocked"),
            SecurityEventType::CommandExecutionAttempt => write!(f, "command_execution_attempt"),
            SecurityEventType::CommandBlocked => write!(f, "command_blocked"),
            SecurityEventType::CommandInjectionDetected => write!(f, "command_injection_detected"),
            SecurityEventType::NetworkRequest => write!(f, "network_request"),
            SecurityEventType::NetworkBlocked => write!(f, "network_blocked"),
            SecurityEventType::AuthSuccess => write!(f, "auth_success"),
            SecurityEventType::AuthFailure => write!(f, "auth_failure"),
            SecurityEventType::TokenValidation => write!(f, "token_validation"),
            SecurityEventType::ConfigReload => write!(f, "config_reload"),
            SecurityEventType::ConfigValidationFailure => write!(f, "config_validation_failure"),
            SecurityEventType::Panic => write!(f, "panic"),
            SecurityEventType::UnsafeBlockExecuted => write!(f, "unsafe_block_executed"),
            SecurityEventType::MemoryLimitExceeded => write!(f, "memory_limit_exceeded"),
        }
    }
}

/// Security audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: SecurityEventType,
    /// Severity level
    pub severity: SecuritySeverity,
    /// Event description
    pub message: String,
    /// Source component (e.g., "plugin_loader", "sandbox")
    pub source: String,
    /// Subject of the event (e.g., plugin name, user ID)
    pub subject: Option<String>,
    /// Additional context
    pub context: HashMap<String, String>,
    /// Stack trace (for errors)
    pub stack_trace: Option<String>,
}

impl SecurityEvent {
    pub fn new(
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            severity,
            message: message.into(),
            source: source.into(),
            subject: None,
            context: HashMap::new(),
            stack_trace: None,
        }
    }

    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }
}

/// Security audit logger
pub struct SecurityAuditLogger {
    events: Arc<Mutex<Vec<SecurityEvent>>>,
    log_path: Option<PathBuf>,
    max_events: usize,
}

impl SecurityAuditLogger {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            log_path: None,
            max_events: 10000,
        }
    }

    pub fn with_log_path(mut self, path: PathBuf) -> Self {
        self.log_path = Some(path);
        self
    }

    /// Log a security event
    pub async fn log(&self, event: SecurityEvent) {
        // Log to tracing
        match event.severity {
            SecuritySeverity::Info => info!(
                "[SECURITY] {:?}: {} - {}",
                event.event_type, event.source, event.message
            ),
            SecuritySeverity::Warning => warn!(
                "[SECURITY] {:?}: {} - {}",
                event.event_type, event.source, event.message
            ),
            SecuritySeverity::Error => error!(
                "[SECURITY] {:?}: {} - {}",
                event.event_type, event.source, event.message
            ),
            SecuritySeverity::Critical => {
                error!(
                    "[SECURITY-CRITICAL] {:?}: {} - {}",
                    event.event_type, event.source, event.message
                );
                // TODO: Send alert to admin
            }
        }

        // Store in memory
        let mut events = self.events.lock().await;
        events.push(event);
        
        // Trim old events
        if events.len() > self.max_events {
            events.remove(0);
        }
    }

    /// Get recent events
    pub async fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let events = self.events.lock().await;
        events.iter().rev().take(limit).cloned().collect()
    }

    /// Get events by type
    pub async fn get_events_by_type(&self, event_type: SecurityEventType) -> Vec<SecurityEvent> {
        let events = self.events.lock().await;
        events
            .iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect()
    }

    /// Export events to file
    pub async fn export_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let events = self.events.lock().await;
        let json = serde_json::to_string_pretty(&*events)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}

/// Global security audit logger
static AUDIT_LOGGER: once_cell::sync::Lazy<SecurityAuditLogger> =
    once_cell::sync::Lazy::new(SecurityAuditLogger::new);

/// Get the global audit logger
pub fn audit() -> &'static SecurityAuditLogger {
    &AUDIT_LOGGER
}

/// Macro for easy security logging
#[macro_export]
macro_rules! security_log {
    ($event_type:expr, $severity:expr, $source:expr, $message:expr) => {
        $crate::security_audit::audit().log(
            $crate::security_audit::SecurityEvent::new(
                $event_type,
                $severity,
                $source,
                $message,
            )
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_event_creation() {
        let event = SecurityEvent::new(
            SecurityEventType::PluginLoadAttempt,
            SecuritySeverity::Info,
            "test",
            "Loading plugin",
        )
        .with_subject("my-plugin")
        .with_context("version", "1.0.0");

        assert_eq!(event.event_type, SecurityEventType::PluginLoadAttempt);
        assert_eq!(event.severity, SecuritySeverity::Info);
        assert_eq!(event.subject, Some("my-plugin".to_string()));
        assert!(event.context.contains_key("version"));
    }
}
