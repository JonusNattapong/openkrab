//! Port of `openkrab/src/shared/chat-envelope.ts`
//!
//! Utilities for stripping channel-envelope prefixes and `[message_id: …]` hints
//! from chat messages. These prefixes are added by multi-channel routing so the
//! LLM knows which platform a message came from, but they should be stripped
//! before echoing text back to the user.

use regex::Regex;

/// Known channel labels that may appear inside an envelope header.
const ENVELOPE_CHANNELS: &[&str] = &[
    "WebChat",
    "WhatsApp",
    "Telegram",
    "Signal",
    "Slack",
    "Discord",
    "Google Chat",
    "iMessage",
    "Teams",
    "Matrix",
    "Zalo",
    "Zalo Personal",
    "BlueBubbles",
];

/// Check whether a bracketed header looks like an envelope prefix.
///
/// Returns `true` if the header contains an ISO-8601-ish timestamp
/// (`2024-01-15T10:30Z` or `2024-01-15 10:30`) **or** starts with one of the
/// known channel names.
fn looks_like_envelope_header(header: &str) -> bool {
    lazy_static::lazy_static! {
        static ref ISO_FULL: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}Z\b").unwrap();
        static ref ISO_SHORT: Regex = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}\b").unwrap();
    }

    if ISO_FULL.is_match(header) || ISO_SHORT.is_match(header) {
        return true;
    }

    ENVELOPE_CHANNELS
        .iter()
        .any(|label| header.starts_with(&format!("{} ", label)))
}

/// Strip the leading `[ChannelName …]` envelope prefix from a message.
///
/// If the text starts with `[…]` and the bracketed content looks like a known
/// envelope header, that prefix is removed and the remaining text is returned.
/// Otherwise the original text is returned unchanged.
pub fn strip_envelope(text: &str) -> &str {
    lazy_static::lazy_static! {
        static ref ENVELOPE_PREFIX: Regex = Regex::new(r"^\[([^\]]+)\]\s*").unwrap();
    }

    let caps = match ENVELOPE_PREFIX.captures(text) {
        Some(c) => c,
        None => return text,
    };

    let header = caps.get(1).map(|m| m.as_str()).unwrap_or("");
    if !looks_like_envelope_header(header) {
        return text;
    }

    let full_match = caps.get(0).unwrap();
    &text[full_match.end()..]
}

/// Strip all lines that look like `[message_id: …]` hints.
///
/// These hints are injected by some routing layers and should not be shown to
/// end users.
pub fn strip_message_id_hints(text: &str) -> String {
    if !text.contains("[message_id:") {
        return text.to_string();
    }

    lazy_static::lazy_static! {
        static ref MSG_ID_LINE: Regex = Regex::new(r"(?i)^\s*\[message_id:\s*[^\]]+\]\s*$").unwrap();
    }

    let lines: Vec<&str> = text.lines().collect();
    let filtered: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|line| !MSG_ID_LINE.is_match(line))
        .collect();

    if filtered.len() == lines.len() {
        text.to_string()
    } else {
        filtered.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_known_channel_envelope() {
        assert_eq!(
            strip_envelope("[Telegram 2024-01-15T10:30Z user] Hello"),
            "Hello"
        );
    }

    #[test]
    fn strips_channel_prefix() {
        assert_eq!(strip_envelope("[Discord bot] hi there"), "hi there");
    }

    #[test]
    fn preserves_non_envelope_brackets() {
        assert_eq!(
            strip_envelope("[something random] hi"),
            "[something random] hi"
        );
    }

    #[test]
    fn preserves_text_without_brackets() {
        assert_eq!(strip_envelope("hello world"), "hello world");
    }

    #[test]
    fn strips_iso_short_timestamp() {
        assert_eq!(
            strip_envelope("[Signal 2024-01-15 10:30 user] test"),
            "test"
        );
    }

    #[test]
    fn strip_message_id_lines() {
        let input = "Hello\n[message_id: abc123]\nWorld";
        assert_eq!(strip_message_id_hints(input), "Hello\nWorld");
    }

    #[test]
    fn preserves_text_without_message_id() {
        let input = "Hello World";
        assert_eq!(strip_message_id_hints(input), "Hello World");
    }

    #[test]
    fn strips_multiple_message_id_lines() {
        let input = "[message_id: a]\nHello\n  [message_id: b]  \nWorld";
        assert_eq!(strip_message_id_hints(input), "Hello\nWorld");
    }
}
