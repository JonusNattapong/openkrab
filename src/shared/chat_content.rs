//! Port of `openclaw/src/shared/chat-content.ts`
//!
//! Extract plain text from chat completion content blocks.
//! Content may be a plain string *or* an array of `{ type: "text", text: "..." }` blocks
//! (the OpenAI multi-modal content format).

use regex::Regex;
use serde_json::Value;

/// Options for [`extract_text_from_chat_content`].
pub struct ExtractOptions<F1, F2>
where
    F1: Fn(&str) -> String,
    F2: Fn(&str) -> String,
{
    /// Optional function to sanitize each text fragment before joining.
    pub sanitize_text: Option<F1>,
    /// Separator used when joining multiple text blocks (default: `" "`).
    pub join_with: Option<String>,
    /// Optional post-join normalizer (default: collapse whitespace + trim).
    pub normalize_text: Option<F2>,
}

impl Default for ExtractOptions<fn(&str) -> String, fn(&str) -> String> {
    fn default() -> Self {
        Self {
            sanitize_text: None,
            join_with: None,
            normalize_text: None,
        }
    }
}

/// Default normalization: collapse runs of whitespace to a single space & trim.
fn default_normalize(text: &str) -> String {
    lazy_static::lazy_static! {
        static ref WS: Regex = Regex::new(r"\s+").unwrap();
    }
    let collapsed = WS.replace_all(text, " ");
    collapsed.trim().to_string()
}

/// Extract plain text from a [`serde_json::Value`] that is either:
/// - a JSON string, or
/// - a JSON array of `{ "type": "text", "text": "..." }` blocks.
///
/// Returns `None` when no text could be extracted.
pub fn extract_text_from_chat_content(content: &Value) -> Option<String> {
    extract_text_from_chat_content_with(content, ExtractOptions::<fn(&str) -> String, fn(&str) -> String>::default())
}

/// Same as [`extract_text_from_chat_content`] but with custom options.
pub fn extract_text_from_chat_content_with<F1, F2>(
    content: &Value,
    opts: ExtractOptions<F1, F2>,
) -> Option<String>
where
    F1: Fn(&str) -> String,
    F2: Fn(&str) -> String,
{
    let join_with = opts.join_with.as_deref().unwrap_or(" ");

    let do_normalize = |text: &str| -> String {
        if let Some(ref f) = opts.normalize_text {
            f(text)
        } else {
            default_normalize(text)
        }
    };

    let do_sanitize = |text: &str| -> String {
        if let Some(ref f) = opts.sanitize_text {
            f(text)
        } else {
            text.to_string()
        }
    };

    // Case 1: plain string
    if let Some(s) = content.as_str() {
        let value = do_sanitize(s);
        let normalized = do_normalize(&value);
        return if normalized.is_empty() { None } else { Some(normalized) };
    }

    // Case 2: array of content blocks
    if let Some(arr) = content.as_array() {
        let mut chunks: Vec<String> = Vec::new();
        for block in arr {
            let obj = match block.as_object() {
                Some(o) => o,
                None => continue,
            };
            match obj.get("type").and_then(|v| v.as_str()) {
                Some("text") => {}
                _ => continue,
            }
            let text = match obj.get("text").and_then(|v| v.as_str()) {
                Some(t) => t,
                None => continue,
            };
            let value = do_sanitize(text);
            if !value.trim().is_empty() {
                chunks.push(value);
            }
        }

        let joined = do_normalize(&chunks.join(join_with));
        return if joined.is_empty() { None } else { Some(joined) };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_plain_string() {
        let content = json!("hello world");
        assert_eq!(extract_text_from_chat_content(&content), Some("hello world".into()));
    }

    #[test]
    fn normalizes_whitespace() {
        let content = json!("  hello   world  ");
        assert_eq!(extract_text_from_chat_content(&content), Some("hello world".into()));
    }

    #[test]
    fn returns_none_for_empty_string() {
        let content = json!("   ");
        assert_eq!(extract_text_from_chat_content(&content), None);
    }

    #[test]
    fn extracts_text_from_content_blocks() {
        let content = json!([
            { "type": "text", "text": "hello" },
            { "type": "image_url", "image_url": "..." },
            { "type": "text", "text": "world" }
        ]);
        assert_eq!(extract_text_from_chat_content(&content), Some("hello world".into()));
    }

    #[test]
    fn skips_non_text_blocks() {
        let content = json!([
            { "type": "image_url", "image_url": "..." },
            { "type": "audio", "data": "..." }
        ]);
        assert_eq!(extract_text_from_chat_content(&content), None);
    }

    #[test]
    fn returns_none_for_empty_array() {
        let content = json!([]);
        assert_eq!(extract_text_from_chat_content(&content), None);
    }

    #[test]
    fn returns_none_for_null() {
        let content = json!(null);
        assert_eq!(extract_text_from_chat_content(&content), None);
    }

    #[test]
    fn returns_none_for_number() {
        let content = json!(42);
        assert_eq!(extract_text_from_chat_content(&content), None);
    }
}
