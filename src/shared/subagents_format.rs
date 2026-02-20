//! Port of `openclaw/src/shared/subagents-format.ts`
//!
//! Formatting utilities for sub-agent status display: compact durations,
//! short token counts, truncated lines, and token usage summaries.

// ─── Duration formatting ────────────────────────────────────────────────────

/// Format a duration in milliseconds into a compact human-readable string.
///
/// Examples: `"3m"`, `"1h30m"`, `"2d5h"`, `"n/a"` (for zero/invalid).
pub fn format_duration_compact(value_ms: Option<f64>) -> String {
    let value_ms = match value_ms {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => return "n/a".to_string(),
    };

    let minutes = (value_ms / 60_000.0).round().max(1.0) as u64;

    if minutes < 60 {
        return format!("{}m", minutes);
    }

    let hours = minutes / 60;
    let minutes_remainder = minutes % 60;

    if hours < 24 {
        return if minutes_remainder > 0 {
            format!("{}h{}m", hours, minutes_remainder)
        } else {
            format!("{}h", hours)
        };
    }

    let days = hours / 24;
    let hours_remainder = hours % 24;
    if hours_remainder > 0 {
        format!("{}d{}h", days, hours_remainder)
    } else {
        format!("{}d", days)
    }
}

// ─── Token formatting ───────────────────────────────────────────────────────

/// Format a token count into a compact short form (`"1.5k"`, `"42"`, `"1.2m"`).
///
/// Returns `None` for zero/negative/invalid values.
pub fn format_token_short(value: Option<f64>) -> Option<String> {
    let value = match value {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => return None,
    };

    let n = value.floor() as u64;
    Some(if n < 1_000 {
        format!("{}", n)
    } else if n < 10_000 {
        let k = n as f64 / 1_000.0;
        format_trim_trailing_zero(k, "k")
    } else if n < 1_000_000 {
        format!("{}k", (n as f64 / 1_000.0).round() as u64)
    } else {
        let m = n as f64 / 1_000_000.0;
        format_trim_trailing_zero(m, "m")
    })
}

/// Format `value` with one decimal and suffix, trimming `.0`.
fn format_trim_trailing_zero(value: f64, suffix: &str) -> String {
    let formatted = format!("{:.1}", value);
    let trimmed = formatted.trim_end_matches(".0");
    // If trimmed == formatted (i.e. it didn't end with .0), use formatted
    if trimmed.len() < formatted.len() {
        format!("{}{}", trimmed, suffix)
    } else {
        format!("{}{}", formatted, suffix)
    }
}

// ─── Line truncation ────────────────────────────────────────────────────────

/// Truncate a string to `max_length`, appending `"..."` if truncated.
pub fn truncate_line(value: &str, max_length: usize) -> String {
    if value.len() <= max_length {
        return value.to_string();
    }
    format!("{}...", value[..max_length].trim_end())
}

// ─── Token usage display ────────────────────────────────────────────────────

/// Loose token-usage struct (fields may or may not be present).
#[derive(Debug, Clone, Default)]
pub struct TokenUsageLike {
    pub total_tokens: Option<f64>,
    pub input_tokens: Option<f64>,
    pub output_tokens: Option<f64>,
}

/// Resolved I/O token breakdown.
#[derive(Debug, Clone)]
pub struct IoTokens {
    pub input: f64,
    pub output: f64,
    pub total: f64,
}

/// Resolve total tokens from a usage entry.
pub fn resolve_total_tokens(entry: Option<&TokenUsageLike>) -> Option<f64> {
    let entry = entry?;
    if let Some(t) = entry.total_tokens {
        if t.is_finite() {
            return Some(t);
        }
    }
    let input = entry.input_tokens.unwrap_or(0.0);
    let output = entry.output_tokens.unwrap_or(0.0);
    let total = input + output;
    if total > 0.0 { Some(total) } else { None }
}

/// Resolve I/O token breakdown from a usage entry.
pub fn resolve_io_tokens(entry: Option<&TokenUsageLike>) -> Option<IoTokens> {
    let entry = entry?;
    let input = entry
        .input_tokens
        .filter(|v| v.is_finite())
        .unwrap_or(0.0);
    let output = entry
        .output_tokens
        .filter(|v| v.is_finite())
        .unwrap_or(0.0);
    let total = input + output;
    if total <= 0.0 {
        return None;
    }
    Some(IoTokens { input, output, total })
}

/// Format a token usage display string like
/// `"tokens 1.5k (in 500 / out 1k)"`.
pub fn format_token_usage_display(entry: Option<&TokenUsageLike>) -> String {
    let io = resolve_io_tokens(entry);
    let prompt_cache = resolve_total_tokens(entry);
    let mut parts: Vec<String> = Vec::new();

    if let Some(ref io) = io {
        let input_s = format_token_short(Some(io.input)).unwrap_or_else(|| "0".to_string());
        let output_s = format_token_short(Some(io.output)).unwrap_or_else(|| "0".to_string());
        let total_s = format_token_short(Some(io.total)).unwrap_or_else(|| "0".to_string());
        parts.push(format!("tokens {} (in {} / out {})", total_s, input_s, output_s));
    } else if let Some(pc) = prompt_cache {
        if pc > 0.0 {
            let pc_s = format_token_short(Some(pc)).unwrap_or_else(|| "0".to_string());
            parts.push(format!("tokens {} prompt/cache", pc_s));
        }
    }

    if let Some(pc) = prompt_cache {
        if let Some(ref io) = io {
            if pc > io.total {
                let pc_s = format_token_short(Some(pc)).unwrap_or_else(|| "0".to_string());
                parts.push(format!("prompt/cache {}", pc_s));
            }
        }
    }

    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── format_duration_compact ──────────────────────────────────────────

    #[test]
    fn duration_none() {
        assert_eq!(format_duration_compact(None), "n/a");
    }

    #[test]
    fn duration_zero() {
        assert_eq!(format_duration_compact(Some(0.0)), "n/a");
    }

    #[test]
    fn duration_negative() {
        assert_eq!(format_duration_compact(Some(-1000.0)), "n/a");
    }

    #[test]
    fn duration_minutes() {
        assert_eq!(format_duration_compact(Some(180_000.0)), "3m");
    }

    #[test]
    fn duration_hours() {
        assert_eq!(format_duration_compact(Some(5_400_000.0)), "1h30m");
    }

    #[test]
    fn duration_exact_hours() {
        assert_eq!(format_duration_compact(Some(7_200_000.0)), "2h");
    }

    #[test]
    fn duration_days() {
        assert_eq!(format_duration_compact(Some(90_000_000.0)), "1d1h");
    }

    // ── format_token_short ──────────────────────────────────────────────

    #[test]
    fn token_none() {
        assert_eq!(format_token_short(None), None);
    }

    #[test]
    fn token_small() {
        assert_eq!(format_token_short(Some(42.0)), Some("42".to_string()));
    }

    #[test]
    fn token_thousands() {
        assert_eq!(format_token_short(Some(1500.0)), Some("1.5k".to_string()));
    }

    #[test]
    fn token_tens_of_thousands() {
        assert_eq!(format_token_short(Some(45_000.0)), Some("45k".to_string()));
    }

    #[test]
    fn token_millions() {
        assert_eq!(format_token_short(Some(1_500_000.0)), Some("1.5m".to_string()));
    }

    // ── truncate_line ───────────────────────────────────────────────────

    #[test]
    fn no_truncation() {
        assert_eq!(truncate_line("hello", 10), "hello");
    }

    #[test]
    fn truncation() {
        assert_eq!(truncate_line("hello world foo", 11), "hello world...");
    }

    // ── resolve_total_tokens ────────────────────────────────────────────

    #[test]
    fn total_from_total_field() {
        let usage = TokenUsageLike {
            total_tokens: Some(100.0),
            input_tokens: Some(50.0),
            output_tokens: Some(30.0),
        };
        assert_eq!(resolve_total_tokens(Some(&usage)), Some(100.0));
    }

    #[test]
    fn total_from_io_fields() {
        let usage = TokenUsageLike {
            total_tokens: None,
            input_tokens: Some(50.0),
            output_tokens: Some(30.0),
        };
        assert_eq!(resolve_total_tokens(Some(&usage)), Some(80.0));
    }

    #[test]
    fn total_none() {
        assert_eq!(resolve_total_tokens(None), None);
    }

    // ── format_token_usage_display ──────────────────────────────────────

    #[test]
    fn format_io_usage() {
        let usage = TokenUsageLike {
            total_tokens: None,
            input_tokens: Some(500.0),
            output_tokens: Some(1000.0),
        };
        let display = format_token_usage_display(Some(&usage));
        assert!(display.contains("tokens"));
        assert!(display.contains("in"));
        assert!(display.contains("out"));
    }

    #[test]
    fn format_empty_usage() {
        assert_eq!(format_token_usage_display(None), "");
    }
}
