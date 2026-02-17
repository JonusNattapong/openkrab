use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::{debug, error, info, instrument, warn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Silent,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Silent => "silent",
            LogLevel::Fatal => "fatal",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
    }
}

pub fn normalize_log_level(level: Option<&str>, fallback: LogLevel) -> LogLevel {
    match level.map(|s| s.trim()) {
        Some("silent") => LogLevel::Silent,
        Some("fatal") => LogLevel::Fatal,
        Some("error") => LogLevel::Error,
        Some("warn") => LogLevel::Warn,
        Some("info") => LogLevel::Info,
        Some("debug") => LogLevel::Debug,
        Some("trace") => LogLevel::Trace,
        _ => fallback,
    }
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

pub static DEFAULT_LOG_DIR: Lazy<PathBuf> = Lazy::new(|| std::env::temp_dir().join("krabkrab"));
pub static DEFAULT_LOG_FILE: Lazy<PathBuf> = Lazy::new(|| DEFAULT_LOG_DIR.join("krabkrab.log"));

#[derive(Clone, Debug)]
pub struct LoggerResolvedSettings {
    pub level: LogLevel,
    pub file: PathBuf,
}

struct LoggingState {
    cached_settings: Option<LoggerResolvedSettings>,
    override_settings: Option<LoggerResolvedSettings>,
}

static LOGGING_STATE: Lazy<Mutex<LoggingState>> = Lazy::new(|| {
    Mutex::new(LoggingState { cached_settings: None, override_settings: None })
});

pub fn get_resolved_logger_settings() -> LoggerResolvedSettings {
    let mut state = LOGGING_STATE.lock().unwrap();
    if let Some(ref o) = state.override_settings {
        return o.clone();
    }
    if let Some(ref c) = state.cached_settings {
        return c.clone();
    }
    let default = LoggerResolvedSettings { level: LogLevel::Info, file: DEFAULT_LOG_FILE.clone() };
    state.cached_settings = Some(default.clone());
    default
}

pub fn is_file_log_level_enabled(level: LogLevel) -> bool {
    let settings = get_resolved_logger_settings();
    if settings.level == LogLevel::Silent {
        return false;
    }
    level_to_min_level(level) <= level_to_min_level(settings.level)
}

pub fn set_logger_override(settings: Option<LoggerResolvedSettings>) {
    let mut state = LOGGING_STATE.lock().unwrap();
    state.override_settings = settings;
    state.cached_settings = None;
}

pub fn reset_logger() {
    let mut state = LOGGING_STATE.lock().unwrap();
    state.override_settings = None;
    state.cached_settings = None;
}

pub fn register_log_transport<F>(_transport: F) -> impl FnOnce()
where
    F: Fn(&str) + Send + Sync + 'static,
{
    // no-op stub; returning a no-op unregister function
    || {}
}

pub fn init() {
    let _ = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_target(false).try_init();
}

#[instrument]
pub fn log_example() {
    info!("logging initialized");
    debug!("debug entry");
    warn!("warning sample");
    error!("error sample");
}

// Minimal pino-like adapter
pub struct PinoLikeLogger {
    pub level: String,
}

impl PinoLikeLogger {
    pub fn child(&self, _bindings: Option<&str>) -> PinoLikeLogger {
        PinoLikeLogger { level: self.level.clone() }
    }

    pub fn trace(&self, args: &str) {
        debug!("{}", args);
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
        error!("FATAL: {}", args);
    }
}

pub fn to_pino_like_logger(level: LogLevel) -> PinoLikeLogger {
    PinoLikeLogger { level: level.as_str().to_string() }
}
