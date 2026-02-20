use super::console::ConsoleStyle;
use super::levels::{level_to_min_level, normalize_log_level, LogLevel};
use super::state::get_logging_state;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tracing::{debug, error, info, trace, warn};

const LOG_PREFIX: &str = "krabkrab";
const LOG_SUFFIX: &str = ".log";
const MAX_LOG_AGE_MS: i64 = 24 * 60 * 60 * 1000;

pub fn default_log_dir() -> PathBuf {
    if let Ok(tmp) = std::env::var("TMPDIR") {
        PathBuf::from(tmp)
    } else if let Ok(tmp) = std::env::var("TEMP") {
        PathBuf::from(tmp)
    } else {
        std::env::temp_dir()
    }
}

pub fn default_log_file() -> PathBuf {
    default_log_dir().join(format!("{}{}", LOG_PREFIX, LOG_SUFFIX))
}

pub static DEFAULT_LOG_DIR: once_cell::sync::Lazy<PathBuf> =
    once_cell::sync::Lazy::new(default_log_dir);

pub static DEFAULT_LOG_FILE: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    let today = Local::now().format("%Y-%m-%d").to_string();
    default_log_dir().join(format!("{}-{}{}", LOG_PREFIX, today, LOG_SUFFIX))
});

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggerSettings {
    #[serde(default)]
    pub level: Option<String>,
    pub file: Option<String>,
    #[serde(default)]
    pub console_level: Option<String>,
    #[serde(default)]
    pub console_style: Option<ConsoleStyle>,
}

#[derive(Debug, Clone)]
pub struct LoggerResolvedSettings {
    pub level: LogLevel,
    pub file: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogRecord {
    pub time: String,
    pub level: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsystem: Option<String>,
}

fn format_local_date(date: &DateTime<Local>) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn default_rolling_path_for_today() -> PathBuf {
    let today = format_local_date(&Local::now());
    default_log_dir().join(format!("{}-{}{}", LOG_PREFIX, today, LOG_SUFFIX))
}

fn is_rolling_path(file: &Path) -> bool {
    let base = file.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let expected_len = format!("{}-YYYY-MM-DD{}", LOG_PREFIX, LOG_SUFFIX).len();
    base.starts_with(&format!("{}-", LOG_PREFIX))
        && base.ends_with(LOG_SUFFIX)
        && base.len() == expected_len
}

fn prune_old_rolling_logs(dir: &Path) {
    let cutoff = Local::now().timestamp_millis() - MAX_LOG_AGE_MS;

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !is_rolling_path(&path) {
                continue;
            }

            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let modified_ms: i64 = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis() as i64)
                        .unwrap_or(0);

                    if modified_ms < cutoff {
                        let _ = std::fs::remove_file(&path);
                    }
                }
            }
        }
    }
}

fn resolve_settings() -> LoggerResolvedSettings {
    let state = get_logging_state();
    let override_settings = state.override_settings.read().unwrap();

    let cfg = override_settings.as_ref();

    let default_level = if std::env::var("VITEST").ok().as_deref() == Some("true")
        && std::env::var("OPENKRAB_TEST_FILE_LOG").ok().as_deref() != Some("1")
    {
        LogLevel::Silent
    } else {
        LogLevel::Info
    };

    let level = cfg
        .and_then(|c| c.level.as_deref())
        .map(|l| normalize_log_level(Some(l), default_level))
        .unwrap_or(default_level);

    let file = cfg
        .and_then(|c| c.file.as_ref())
        .map(PathBuf::from)
        .unwrap_or_else(default_rolling_path_for_today);

    LoggerResolvedSettings { level, file }
}

fn settings_changed(a: Option<&LoggerResolvedSettings>, b: &LoggerResolvedSettings) -> bool {
    match a {
        None => true,
        Some(a) => a.level != b.level || a.file != b.file,
    }
}

pub fn is_file_log_level_enabled(level: LogLevel) -> bool {
    let state = get_logging_state();
    let cached = state.cached_settings.read().unwrap();
    let settings = cached.clone().unwrap_or_else(resolve_settings);

    if settings.level == LogLevel::Silent {
        return false;
    }

    level_to_min_level(level) <= level_to_min_level(settings.level)
}

pub fn get_resolved_logger_settings() -> LoggerResolvedSettings {
    resolve_settings()
}

pub fn get_logger() -> &'static tracing_subscriber::FmtSubscriber<
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format,
> {
    static LOGGER: once_cell::sync::Lazy<
        tracing_subscriber::FmtSubscriber<
            tracing_subscriber::fmt::format::DefaultFields,
            tracing_subscriber::fmt::format::Format,
        >,
    > = once_cell::sync::Lazy::new(|| tracing_subscriber::fmt::Subscriber::new());
    &LOGGER
}

pub fn init_logger() {
    let settings = resolve_settings();
    let state = get_logging_state();

    if let Ok(mut cached_settings) = state.cached_settings.write() {
        if settings_changed(cached_settings.as_ref(), &settings) {
            *cached_settings = Some(settings.clone());
        }
    }

    if settings.level != LogLevel::Silent {
        if let Some(parent) = settings.file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if is_rolling_path(&settings.file) {
            if let Some(parent) = settings.file.parent() {
                prune_old_rolling_logs(parent);
            }
        }
    }
}

pub fn get_child_logger(bindings: Option<&serde_json::Value>, level: Option<LogLevel>) {
    let _ = (bindings, level);
}

pub fn set_logger_override(settings: Option<LoggerSettings>) {
    let state = get_logging_state();
    *state.override_settings.write().unwrap() = settings;
    *state.cached_logger.write().unwrap() = None;
    *state.cached_settings.write().unwrap() = None;
    *state.cached_console_settings.write().unwrap() = None;
}

pub fn reset_logger() {
    let state = get_logging_state();
    *state.cached_logger.write().unwrap() = None;
    *state.cached_settings.write().unwrap() = None;
    *state.cached_console_settings.write().unwrap() = None;
    *state.override_settings.write().unwrap() = None;
}

#[derive(Debug, Clone)]
pub struct PinoLikeLogger {
    pub level: String,
}

impl PinoLikeLogger {
    pub fn child(&self, _bindings: Option<&serde_json::Value>) -> Self {
        Self {
            level: self.level.clone(),
        }
    }

    pub fn trace(&self, args: &str) {
        trace!("{}", args);
    }

    pub fn debug(&self, args: &str) {
        debug!("{}", args);
    }

    pub fn info(&self, args: &str) {
        info!("{}", args);
    }

    pub fn warn(&self, args: &str) {
        warn!("{}", args);
    }

    pub fn error(&self, args: &str) {
        error!("{}", args);
    }

    pub fn fatal(&self, args: &str) {
        error!("[FATAL] {}", args);
    }
}

pub fn to_pino_like_logger(level: LogLevel) -> PinoLikeLogger {
    PinoLikeLogger {
        level: level.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_paths() {
        let dir = default_log_dir();
        assert!(dir.to_string_lossy().len() > 0);
    }

    #[test]
    fn test_is_file_log_level_enabled() {
        assert!(is_file_log_level_enabled(LogLevel::Fatal));
        assert!(is_file_log_level_enabled(LogLevel::Error));
    }

    #[test]
    fn test_reset_logger() {
        set_logger_override(Some(LoggerSettings {
            level: Some("debug".to_string()),
            file: Some("/tmp/test.log".to_string()),
            console_level: None,
            console_style: None,
        }));

        reset_logger();

        let state = get_logging_state();
        assert!(state.override_settings.read().unwrap().is_none());
    }
}
