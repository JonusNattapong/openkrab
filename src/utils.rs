use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn is_truthy_env(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

pub fn safe_json_parse(input: &str) -> Option<Value> {
    serde_json::from_str::<Value>(input).ok()
}

pub fn safe_parse_json<T: DeserializeOwned>(raw: &str) -> Option<T> {
    serde_json::from_str::<T>(raw).ok()
}

pub fn truncate_text(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    input.chars().take(max_chars).collect()
}

pub fn ensure_dir(path: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

pub fn path_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}

pub fn clamp_number(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

pub fn clamp_int(value: i64, min: i64, max: i64) -> i64 {
    clamp_number(value as f64, min as f64, max as f64) as i64
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    clamp_number(value, min, max)
}

pub fn escape_regexp(value: &str) -> String {
    const SPECIALS: &[char] = &[
        '.', '*', '+', '?', '^', '$', '{', '}', '(', ')', '|', '[', ']', '\\', '/', ' ', '\t', '\n', '\r',
    ];
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if SPECIALS.contains(&ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

pub fn is_plain_object(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

pub fn is_record(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

pub fn normalize_path(p: &str) -> String {
    if p.starts_with('/') {
        p.to_string()
    } else {
        format!("/{}", p)
    }
}

pub fn with_whatsapp_prefix(number: &str) -> String {
    if number.starts_with("whatsapp:") {
        number.to_string()
    } else {
        format!("whatsapp:{}", number)
    }
}

pub fn normalize_e164(number: &str) -> String {
    let without_prefix = number.trim().trim_start_matches("whatsapp:");
    let digits: String = without_prefix.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
    if digits.starts_with('+') {
        format!("+{}", digits.trim_start_matches('+'))
    } else {
        format!("+{}", digits)
    }
}

pub fn is_self_chat_mode(self_e164: Option<&str>, allow_from: Option<&[String]>) -> bool {
    let self_e164 = match self_e164 {
        Some(s) => s,
        None => return false,
    };
    let allow = match allow_from {
        Some(a) if !a.is_empty() => a,
        _ => return false,
    };
    let normalized_self = normalize_e164(self_e164);
    for n in allow.iter() {
        if n == "*" {
            continue;
        }
        if normalize_e164(n) == normalized_self {
            return true;
        }
    }
    false
}

pub fn to_whatsapp_jid(number: &str) -> String {
    let without_prefix = number.trim().trim_start_matches("whatsapp:");
    if without_prefix.contains('@') {
        return without_prefix.to_string();
    }
    let e164 = normalize_e164(without_prefix);
    let digits: String = e164.chars().filter(|c| c.is_ascii_digit()).collect();
    format!("{}@s.whatsapp.net", digits)
}

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

pub fn slice_utf16_safe(input: &str, start: isize, end: Option<isize>) -> String {
    // Work with char indices which are Unicode scalar values in Rust
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len() as isize;
    let mut from = if start < 0 { (len + start).max(0) } else { start.min(len) };
    let mut to = match end {
        Some(e) => if e < 0 { (len + e).max(0) } else { e.min(len) },
        None => len,
    };
    if to < from {
        std::mem::swap(&mut from, &mut to);
    }
    chars[from as usize..to as usize].iter().collect()
}

pub fn truncate_utf16_safe(input: &str, max_len: usize) -> String {
    let limit = max_len;
    if input.chars().count() <= limit {
        return input.to_string();
    }
    input.chars().take(limit).collect()
}

pub fn resolve_user_path(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return trimmed.to_string();
    }
    if trimmed.starts_with('~') {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        let expanded = trimmed.replacen("~", &home.to_string_lossy(), 1);
        return fs::canonicalize(&expanded).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| expanded);
    }
    fs::canonicalize(trimmed).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| trimmed.to_string())
}

pub fn resolve_config_dir() -> String {
    let override_dir = env::var("krabkrab_STATE_DIR").ok().or_else(|| env::var("CLAWDBOT_STATE_DIR").ok());
    if let Some(o) = override_dir.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
        return resolve_user_path(&o);
    }
    let mut new_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    new_dir.push(".krabkrab");
    new_dir.to_string_lossy().to_string()
}

fn resolve_home_display_prefix() -> Option<(String, String)> {
    let home = dirs::home_dir()?.to_string_lossy().to_string();
    let explicit_home = env::var("krabkrab_HOME").ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    if explicit_home.is_some() {
        return Some((home, "$krabkrab_HOME".to_string()));
    }
    Some((home, "~".to_string()))
}

pub fn shorten_home_path(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    if let Some((home, prefix)) = resolve_home_display_prefix() {
        if input == home {
            return prefix;
        }
        if input.starts_with(&format!("{}/", home)) || input.starts_with(&format!("{}\\", home)) {
            return format!("{}{}", prefix, &input[home.len()..]);
        }
    }
    input.to_string()
}

pub fn shorten_home_in_string(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    if let Some((home, prefix)) = resolve_home_display_prefix() {
        return input.replace(&home, &prefix);
    }
    input.to_string()
}

pub fn display_path(input: &str) -> String {
    shorten_home_path(input)
}

pub fn display_string(input: &str) -> String {
    shorten_home_in_string(input)
}

