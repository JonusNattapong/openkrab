use super::levels::{normalize_log_level, LogLevel};
use super::logger::LoggerSettings;
use super::state::get_logging_state;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleStyle {
    #[default]
    Pretty,
    Compact,
    Json,
}

impl std::fmt::Display for ConsoleStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsoleStyle::Pretty => write!(f, "pretty"),
            ConsoleStyle::Compact => write!(f, "compact"),
            ConsoleStyle::Json => write!(f, "json"),
        }
    }
}

impl std::str::FromStr for ConsoleStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "pretty" => Ok(ConsoleStyle::Pretty),
            "compact" => Ok(ConsoleStyle::Compact),
            "json" => Ok(ConsoleStyle::Json),
            _ => Err(format!("Invalid console style: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleSettings {
    pub level: LogLevel,
    pub style: ConsoleStyle,
}

pub type ConsoleLoggerSettings = ConsoleSettings;

const SUPPRESSED_CONSOLE_PREFIXES: &[&str] = &[
    "Closing session:",
    "Opening session:",
    "Removing old closed session:",
    "Session already closed",
    "Session already open",
];

fn is_verbose() -> bool {
    std::env::var("OPENCLAW_VERBOSE")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

fn normalize_console_level(level: Option<&str>) -> LogLevel {
    if is_verbose() {
        return LogLevel::Debug;
    }
    if std::env::var("VITEST").ok().as_deref() == Some("true")
        && std::env::var("OPENCLAW_TEST_CONSOLE").ok().as_deref() != Some("1")
    {
        return LogLevel::Silent;
    }
    normalize_log_level(level, LogLevel::Info)
}

fn normalize_console_style(style: Option<&str>) -> ConsoleStyle {
    if let Some(s) = style {
        if let Ok(parsed) = s.parse::<ConsoleStyle>() {
            return parsed;
        }
    }
    if atty::is(atty::Stream::Stdout) {
        ConsoleStyle::Pretty
    } else {
        ConsoleStyle::Compact
    }
}

fn resolve_console_settings() -> ConsoleSettings {
    let state = get_logging_state();
    let override_settings = state.override_settings.read().unwrap();

    let (cfg_level, cfg_style) = if let Some(ref settings) = *override_settings {
        (
            settings.console_level.as_deref(),
            settings.console_style.as_ref().map(|s| s.to_string()),
        )
    } else {
        (None, None)
    };

    let level = normalize_console_level(cfg_level.map(|s| s));
    let style = normalize_console_style(cfg_style.as_deref());

    ConsoleSettings { level, style }
}

fn console_settings_changed(a: Option<&ConsoleSettings>, b: &ConsoleSettings) -> bool {
    match a {
        None => true,
        Some(a) => a.level != b.level || a.style != b.style,
    }
}

pub fn get_console_settings() -> ConsoleLoggerSettings {
    let state = get_logging_state();
    let settings = resolve_console_settings();
    let cached = state.cached_console_settings.read().unwrap();

    if console_settings_changed(cached.as_ref(), &settings) {
        drop(cached);
        *state.cached_console_settings.write().unwrap() = Some(settings.clone());
        return settings;
    }

    cached.clone().unwrap_or(settings)
}

pub fn get_resolved_console_settings() -> ConsoleLoggerSettings {
    get_console_settings()
}

pub fn route_logs_to_stderr() {
    let state = get_logging_state();
    *state.force_console_to_stderr.lock().unwrap() = true;
}

pub fn set_console_subsystem_filter(filters: Option<&[String]>) {
    let state = get_logging_state();
    if let Some(f) = filters {
        let normalized: Vec<String> = f
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        *state.console_subsystem_filter.write().unwrap() = if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        };
    } else {
        *state.console_subsystem_filter.write().unwrap() = None;
    }
}

pub fn set_console_timestamp_prefix(enabled: bool) {
    let state = get_logging_state();
    *state.console_timestamp_prefix.lock().unwrap() = enabled;
}

pub fn should_log_subsystem_to_console(subsystem: &str) -> bool {
    let state = get_logging_state();
    let filter = state.console_subsystem_filter.read().unwrap();

    match filter.as_ref() {
        None => true,
        Some(filters) => filters
            .iter()
            .any(|prefix| subsystem == prefix || subsystem.starts_with(&format!("{}/", prefix))),
    }
}

pub fn should_suppress_console_message(message: &str) -> bool {
    if is_verbose() {
        return false;
    }
    SUPPRESSED_CONSOLE_PREFIXES
        .iter()
        .any(|prefix| message.starts_with(prefix))
}

pub fn format_console_timestamp(style: ConsoleStyle) -> String {
    let now = chrono::Local::now();
    if style == ConsoleStyle::Pretty {
        now.format("%H:%M:%S").to_string()
    } else {
        now.to_rfc3339()
    }
}

pub fn has_timestamp_prefix(value: &str) -> bool {
    let re = regex::Regex::new(
        r"^(?:\d{2}:\d{2}:\d{2}|\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?)"
    ).unwrap();
    re.is_match(value)
}

pub fn is_json_payload(value: &str) -> bool {
    let trimmed = value.trim();
    if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
        return false;
    }
    serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_style_parse() {
        assert_eq!("pretty".parse::<ConsoleStyle>(), Ok(ConsoleStyle::Pretty));
        assert_eq!("compact".parse::<ConsoleStyle>(), Ok(ConsoleStyle::Compact));
        assert_eq!("json".parse::<ConsoleStyle>(), Ok(ConsoleStyle::Json));
    }

    #[test]
    fn test_should_suppress_console_message() {
        assert!(should_suppress_console_message("Closing session: test"));
        assert!(!should_suppress_console_message("Normal message"));
    }

    #[test]
    fn test_has_timestamp_prefix() {
        assert!(has_timestamp_prefix("12:34:56 Some message"));
        assert!(has_timestamp_prefix("2024-01-15T12:34:56Z Some message"));
        assert!(!has_timestamp_prefix("No timestamp here"));
    }

    #[test]
    fn test_is_json_payload() {
        assert!(is_json_payload(r#"{"key": "value"}"#));
        assert!(is_json_payload(r#"[1, 2, 3]"#));
        assert!(!is_json_payload("Not JSON"));
    }

    #[test]
    fn test_should_log_subsystem_to_console() {
        set_console_subsystem_filter(Some(&["gateway".to_string(), "discord".to_string()]));
        assert!(should_log_subsystem_to_console("gateway"));
        assert!(should_log_subsystem_to_console("discord/messages"));
        assert!(!should_log_subsystem_to_console("telegram"));

        set_console_subsystem_filter(None);
        assert!(should_log_subsystem_to_console("anything"));
    }
}
