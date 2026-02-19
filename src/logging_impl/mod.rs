pub mod console;
pub mod levels;
pub mod logger;
pub mod state;

pub use console::{
    get_console_settings, route_logs_to_stderr, set_console_subsystem_filter,
    ConsoleLoggerSettings, ConsoleStyle,
};
pub use levels::{level_to_min_level, normalize_log_level, LogLevel, ALLOWED_LOG_LEVELS};
pub use logger::{
    get_child_logger, get_logger, reset_logger, LoggerResolvedSettings, LoggerSettings,
    DEFAULT_LOG_DIR, DEFAULT_LOG_FILE,
};
pub use state::LoggingState;
