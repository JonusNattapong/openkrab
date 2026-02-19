//! signal::format — Signal text formatting and markdown conversion.
//! Ported from `openclaw/src/signal/format.ts` (Phase 13).

/// Signal text style types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalTextStyle {
    Bold,
    Italic,
    Strikethrough,
    Monospace,
    Spoiler,
}

/// Text style range for Signal messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalTextStyleRange {
    pub start: usize,
    pub length: usize,
    pub style: SignalTextStyle,
}

/// Formatted text with styles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalFormattedText {
    pub text: String,
    pub styles: Vec<SignalTextStyleRange>,
}

impl SignalFormattedText {
    pub fn new(text: String) -> Self {
        Self {
            text,
            styles: Vec::new(),
        }
    }

    pub fn with_styles(text: String, styles: Vec<SignalTextStyleRange>) -> Self {
        Self { text, styles }
    }

    /// Add a style range.
    pub fn add_style(&mut self, start: usize, length: usize, style: SignalTextStyle) {
        self.styles.push(SignalTextStyleRange {
            start,
            length,
            style,
        });
    }
}

/// Markdown to Signal text conversion options.
#[derive(Debug, Clone)]
pub struct SignalMarkdownOptions {
    pub table_mode: Option<String>, // "text" or "markdown"
}

impl Default for SignalMarkdownOptions {
    fn default() -> Self {
        Self {
            table_mode: Some("text".to_string()),
        }
    }
}

/// Convert markdown text to Signal formatted text.
/// Basic implementation - converts **bold**, *italic*, ~~strikethrough~~, `code`, ||spoiler||.
pub fn markdown_to_signal_text(
    markdown: &str,
    _options: &SignalMarkdownOptions,
) -> SignalFormattedText {
    let mut text = String::new();
    let mut styles = Vec::new();
    let mut pos = 0;

    // Simple regex-based parsing for basic markdown
    // This is a simplified version - full markdown parsing would be more complex

    let mut chars = markdown.chars().peekable();
    let mut i = 0;

    while i < markdown.len() {
        let remaining = &markdown[i..];

        // Bold: **text**
        if remaining.starts_with("**") {
            if let Some(end) = find_matching_delimiter(&markdown[i + 2..], "**") {
                let content_start = i + 2;
                let content_end = content_start + end;
                let content_len = end;

                text.push_str(&markdown[content_start..content_end]);
                styles.push(SignalTextStyleRange {
                    start: pos,
                    length: content_len,
                    style: SignalTextStyle::Bold,
                });

                pos += content_len;
                i = content_end + 2;
                continue;
            }
        }

        // Italic: *text*
        if remaining.starts_with('*') && !remaining.starts_with("**") {
            if let Some(end) = find_matching_delimiter(&markdown[i + 1..], "*") {
                let content_start = i + 1;
                let content_end = content_start + end;
                let content_len = end;

                text.push_str(&markdown[content_start..content_end]);
                styles.push(SignalTextStyleRange {
                    start: pos,
                    length: content_len,
                    style: SignalTextStyle::Italic,
                });

                pos += content_len;
                i = content_end + 1;
                continue;
            }
        }

        // Strikethrough: ~~text~~
        if remaining.starts_with("~~") {
            if let Some(end) = find_matching_delimiter(&markdown[i + 2..], "~~") {
                let content_start = i + 2;
                let content_end = content_start + end;
                let content_len = end;

                text.push_str(&markdown[content_start..content_end]);
                styles.push(SignalTextStyleRange {
                    start: pos,
                    length: content_len,
                    style: SignalTextStyle::Strikethrough,
                });

                pos += content_len;
                i = content_end + 2;
                continue;
            }
        }

        // Code: `text`
        if remaining.starts_with('`') {
            if let Some(end) = markdown[i + 1..].find('`') {
                let content_start = i + 1;
                let content_end = content_start + end;
                let content_len = end;

                text.push_str(&markdown[content_start..content_end]);
                styles.push(SignalTextStyleRange {
                    start: pos,
                    length: content_len,
                    style: SignalTextStyle::Monospace,
                });

                pos += content_len;
                i = content_end + 1;
                continue;
            }
        }

        // Spoiler: ||text||
        if remaining.starts_with("||") {
            if let Some(end) = find_matching_delimiter(&markdown[i + 2..], "||") {
                let content_start = i + 2;
                let content_end = content_start + end;
                let content_len = end;

                text.push_str(&markdown[content_start..content_end]);
                styles.push(SignalTextStyleRange {
                    start: pos,
                    length: content_len,
                    style: SignalTextStyle::Spoiler,
                });

                pos += content_len;
                i = content_end + 2;
                continue;
            }
        }

        // Regular character
        if let Some(ch) = chars.next() {
            text.push(ch);
            pos += ch.len_utf8();
            i += ch.len_utf8();
        } else {
            break;
        }
    }

    SignalFormattedText::with_styles(text, styles)
}

/// Find matching delimiter, handling escapes.
fn find_matching_delimiter(text: &str, delimiter: &str) -> Option<usize> {
    let mut pos = 0;
    while let Some(idx) = text[pos..].find(delimiter) {
        let actual_pos = pos + idx;
        // Check if escaped (preceded by \)
        if actual_pos > 0 && text.as_bytes()[actual_pos - 1] == b'\\' {
            pos = actual_pos + delimiter.len();
            continue;
        }
        return Some(actual_pos);
    }
    None
}

/// Chunk formatted text for Signal's length limits.
pub fn chunk_signal_text(
    formatted: &SignalFormattedText,
    max_length: usize,
) -> Vec<SignalFormattedText> {
    if formatted.text.len() <= max_length {
        return vec![formatted.clone()];
    }

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < formatted.text.len() {
        let end = (start + max_length).min(formatted.text.len());

        // Try to break at word boundary if possible
        let chunk_end = if end < formatted.text.len() {
            find_word_boundary(&formatted.text, start, end)
        } else {
            end
        };

        let chunk_text = formatted.text[start..chunk_end].to_string();

        // Find styles that apply to this chunk
        let mut chunk_styles = Vec::new();
        for style in &formatted.styles {
            let style_start = style.start;
            let style_end = style.start + style.length;

            // Style overlaps with chunk
            if style_start < chunk_end && style_end > start {
                let chunk_style_start = if style_start < start {
                    0
                } else {
                    style_start - start
                };
                let chunk_style_end = if style_end > chunk_end {
                    chunk_end - start
                } else {
                    style_end - start
                };
                let chunk_style_length = chunk_style_end - chunk_style_start;

                if chunk_style_length > 0 {
                    chunk_styles.push(SignalTextStyleRange {
                        start: chunk_style_start,
                        length: chunk_style_length,
                        style: style.style,
                    });
                }
            }
        }

        chunks.push(SignalFormattedText::with_styles(chunk_text, chunk_styles));
        start = chunk_end;
    }

    chunks
}

/// Find word boundary within range.
fn find_word_boundary(text: &str, start: usize, end: usize) -> usize {
    let slice = &text[start..end];

    // Look for last space
    if let Some(pos) = slice.rfind(' ') {
        return start + pos;
    }

    // Look for last newline
    if let Some(pos) = slice.rfind('\n') {
        return start + pos;
    }

    // No good boundary, just cut at limit
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_to_signal_basic_text() {
        let result = markdown_to_signal_text("Hello world", &SignalMarkdownOptions::default());
        assert_eq!(result.text, "Hello world");
        assert!(result.styles.is_empty());
    }

    #[test]
    fn markdown_to_signal_bold() {
        let result = markdown_to_signal_text("**bold** text", &SignalMarkdownOptions::default());
        assert_eq!(result.text, "bold text");
        assert_eq!(result.styles.len(), 1);
        assert_eq!(result.styles[0].start, 0);
        assert_eq!(result.styles[0].length, 4);
        assert_eq!(result.styles[0].style, SignalTextStyle::Bold);
    }

    #[test]
    fn markdown_to_signal_italic() {
        let result = markdown_to_signal_text("*italic* text", &SignalMarkdownOptions::default());
        assert_eq!(result.text, "italic text");
        assert_eq!(result.styles.len(), 1);
        assert_eq!(result.styles[0].start, 0);
        assert_eq!(result.styles[0].length, 6);
        assert_eq!(result.styles[0].style, SignalTextStyle::Italic);
    }

    #[test]
    fn markdown_to_signal_code() {
        let result = markdown_to_signal_text("`code` text", &SignalMarkdownOptions::default());
        assert_eq!(result.text, "code text");
        assert_eq!(result.styles.len(), 1);
        assert_eq!(result.styles[0].start, 0);
        assert_eq!(result.styles[0].length, 4);
        assert_eq!(result.styles[0].style, SignalTextStyle::Monospace);
    }

    #[test]
    fn chunk_signal_text_no_chunking() {
        let formatted = SignalFormattedText::new("short text".to_string());
        let chunks = chunk_signal_text(&formatted, 100);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "short text");
    }

    #[test]
    fn chunk_signal_text_with_styles() {
        let mut formatted = SignalFormattedText::new(
            "This is a very long message that needs to be chunked".to_string(),
        );
        formatted.add_style(0, 4, SignalTextStyle::Bold); // "This"
        formatted.add_style(10, 6, SignalTextStyle::Italic); // "a very"

        let chunks = chunk_signal_text(&formatted, 20);
        assert!(chunks.len() > 1);

        // Check that styles are preserved in chunks
        let has_bold = chunks
            .iter()
            .any(|c| c.styles.iter().any(|s| s.style == SignalTextStyle::Bold));
        let has_italic = chunks
            .iter()
            .any(|c| c.styles.iter().any(|s| s.style == SignalTextStyle::Italic));
        assert!(has_bold || has_italic); // At least one style should be preserved
    }

    #[test]
    fn find_matching_delimiter_simple() {
        assert_eq!(find_matching_delimiter("text**", "**"), Some(4));
    }

    #[test]
    fn find_matching_delimiter_escaped() {
        assert_eq!(find_matching_delimiter(r#"text\**"#, "**"), None);
    }
}
