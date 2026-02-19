pub use crate::logging_impl::console::{
    format_console_timestamp, get_console_settings, get_resolved_console_settings,
    route_logs_to_stderr, set_console_subsystem_filter, set_console_timestamp_prefix,
    should_log_subsystem_to_console, should_suppress_console_message, ConsoleLoggerSettings,
    ConsoleSettings, ConsoleStyle,
};
pub use crate::logging_impl::levels::{
    level_to_min_level, normalize_log_level, LogLevel, ALLOWED_LOG_LEVELS,
};
pub use crate::logging_impl::logger::{
    get_child_logger, get_logger, get_resolved_logger_settings, init_logger,
    is_file_log_level_enabled, reset_logger, set_logger_override, to_pino_like_logger,
    LoggerResolvedSettings, LoggerSettings, PinoLikeLogger, DEFAULT_LOG_DIR, DEFAULT_LOG_FILE,
};
pub use crate::logging_impl::state::LoggingState;

pub fn init() {
    init_logger();
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .try_init();
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
    fn test_is_file_log_level_enabled() {
        assert!(is_file_log_level_enabled(LogLevel::Fatal));
        assert!(is_file_log_level_enabled(LogLevel::Error));
    }
}
