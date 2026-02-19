use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const ALLOWED_LOG_LEVELS: &[&str] =
    &["silent", "fatal", "error", "warn", "info", "debug", "trace"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Silent,
    Fatal,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Silent => write!(f, "silent"),
            LogLevel::Fatal => write!(f, "fatal"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Trace => write!(f, "trace"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "silent" => Ok(LogLevel::Silent),
            "fatal" => Ok(LogLevel::Fatal),
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

pub fn normalize_log_level(level: Option<&str>, fallback: LogLevel) -> LogLevel {
    level
        .and_then(|s| s.trim().parse::<LogLevel>().ok())
        .unwrap_or(fallback)
}

pub fn level_to_min_level(level: LogLevel) -> u8 {
    match level {
        LogLevel::Fatal => 0,
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
        LogLevel::Trace => 5,
        LogLevel::Silent => u8::MAX,
    }
}

static LOG_LEVEL_STRS: OnceLock<Vec<&'static str>> = OnceLock::new();

pub fn allowed_log_levels() -> &'static Vec<&'static str> {
    LOG_LEVEL_STRS.get_or_init(|| ALLOWED_LOG_LEVELS.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_log_level() {
        assert_eq!(
            normalize_log_level(Some("debug"), LogLevel::Info),
            LogLevel::Debug
        );
        assert_eq!(
            normalize_log_level(Some("INVALID"), LogLevel::Info),
            LogLevel::Info
        );
        assert_eq!(normalize_log_level(None, LogLevel::Warn), LogLevel::Warn);
    }

    #[test]
    fn test_level_to_min_level() {
        assert_eq!(level_to_min_level(LogLevel::Fatal), 0);
        assert_eq!(level_to_min_level(LogLevel::Error), 1);
        assert_eq!(level_to_min_level(LogLevel::Info), 3);
        assert_eq!(level_to_min_level(LogLevel::Silent), u8::MAX);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Debug), "debug");
        assert_eq!(format!("{}", LogLevel::Info), "info");
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!("warn".parse::<LogLevel>(), Ok(LogLevel::Warn));
        assert!("invalid".parse::<LogLevel>().is_err());
    }
}
