use super::console::ConsoleSettings;
use super::levels::LogLevel;
use super::logger::{LoggerResolvedSettings, LoggerSettings};
use std::sync::{Mutex, RwLock};

pub struct LoggingState {
    pub cached_logger: RwLock<Option<()>>,
    pub cached_settings: RwLock<Option<LoggerResolvedSettings>>,
    pub cached_console_settings: RwLock<Option<ConsoleSettings>>,
    pub override_settings: RwLock<Option<LoggerSettings>>,
    pub console_patched: Mutex<bool>,
    pub force_console_to_stderr: Mutex<bool>,
    pub console_timestamp_prefix: Mutex<bool>,
    pub console_subsystem_filter: RwLock<Option<Vec<String>>>,
}

impl LoggingState {
    pub fn new() -> Self {
        Self {
            cached_logger: RwLock::new(None),
            cached_settings: RwLock::new(None),
            cached_console_settings: RwLock::new(None),
            override_settings: RwLock::new(None),
            console_patched: Mutex::new(false),
            force_console_to_stderr: Mutex::new(false),
            console_timestamp_prefix: Mutex::new(false),
            console_subsystem_filter: RwLock::new(None),
        }
    }

    pub fn reset(&self) {
        *self.cached_logger.write().unwrap() = None;
        *self.cached_settings.write().unwrap() = None;
        *self.cached_console_settings.write().unwrap() = None;
        *self.override_settings.write().unwrap() = None;
        *self.console_patched.lock().unwrap() = false;
        *self.force_console_to_stderr.lock().unwrap() = false;
        *self.console_timestamp_prefix.lock().unwrap() = false;
        *self.console_subsystem_filter.write().unwrap() = None;
    }
}

impl Default for LoggingState {
    fn default() -> Self {
        Self::new()
    }
}

pub static LOGGING_STATE: once_cell::sync::Lazy<LoggingState> =
    once_cell::sync::Lazy::new(LoggingState::new);

pub fn get_logging_state() -> &'static LoggingState {
    &LOGGING_STATE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_state_reset() {
        let state = LoggingState::new();
        *state.force_console_to_stderr.lock().unwrap() = true;
        state.reset();
        assert!(!*state.force_console_to_stderr.lock().unwrap());
    }
}
