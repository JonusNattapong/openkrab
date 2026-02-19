//! markdown — Markdown processing and chunking utilities.
//! Ported from `openclaw/src/markdown/` (Phase 6).
//!
//! Provides heading-aware chunking (used by the memory indexer),
//! front-matter extraction, and basic text utilities for Markdown content.

use std::collections::HashMap;

// ─── Front-matter ─────────────────────────────────────────────────────────────

/// Parsed front-matter and body from a Markdown document.
#[derive(Debug, Clone, Default)]
pub struct ParsedDoc {
    /// YAML-like key-value front-matter (simple string values only).
    pub frontmatter: HashMap<String, String>,
    /// The body of the document (after the front-matter block).
    pub body: String,
}

/// Extract YAML-style front-matter delimited by `---` lines.
/// Returns `(frontmatter_map, body)`.
pub fn extract_frontmatter(input: &str) -> ParsedDoc {
    let mut lines = input.lines();
    let first = lines.next().unwrap_or("");
    if first.trim() != "---" {
        return ParsedDoc {
            frontmatter: HashMap::new(),
            body: input.to_string(),
        };
    }

    let mut fm = HashMap::new();
    let mut rest_lines = Vec::new();
    let mut in_fm = true;

    for line in lines {
        if in_fm {
            if line.trim() == "---" {
                in_fm = false;
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                fm.insert(k.trim().to_string(), v.trim().to_string());
            }
        } else {
            rest_lines.push(line);
        }
    }

    ParsedDoc {
        frontmatter: fm,
        body: rest_lines.join("\n"),
    }
}

// ─── Heading-aware chunker ────────────────────────────────────────────────────

/// A single chunk produced by the heading-aware splitter.
#[derive(Debug, Clone)]
pub struct MarkdownChunk {
    /// Heading text that introduces this chunk (empty string for preamble).
    pub heading: String,
    /// Heading level (1–6) or 0 if preamble.
    pub level: u8,
    /// Anchor path: `["# H1", "## H2"]` to this chunk.
    pub path: Vec<String>,
    /// The text content of this chunk (includes the heading line).
    pub content: String,
    /// Source file path (optional).
    pub source: Option<String>,
    /// Character offset in the original document.
    pub offset: usize,
}

/// Split a Markdown document into heading-scoped chunks.
///
/// Each chunk contains from its heading up to (but not including) the next
/// heading of equal or higher precedence.
pub fn split_by_headings(
    markdown: &str,
    min_chunk_chars: usize,
    source: Option<&str>,
) -> Vec<MarkdownChunk> {
    let mut chunks: Vec<MarkdownChunk> = Vec::new();
    let mut current_heading = String::new();
    let mut current_level: u8 = 0;
    let mut current_path: Vec<String> = Vec::new();
    let mut current_lines: Vec<&str> = Vec::new();
    let mut offset: usize = 0;
    let mut chunk_start = 0;

    for line in markdown.lines() {
        let heading_level = heading_level(line);

        if heading_level > 0 {
            // Flush current chunk
            let content = current_lines.join("\n");
            if content.trim().len() >= min_chunk_chars || chunks.is_empty() {
                chunks.push(MarkdownChunk {
                    heading: current_heading.clone(),
                    level: current_level,
                    path: current_path.clone(),
                    content,
                    source: source.map(|s| s.to_string()),
                    offset: chunk_start,
                });
            }

            // Update path
            let prefix = "#".repeat(heading_level as usize);
            let h_text = line.trim_start_matches('#').trim().to_string();
            // Truncate path to this level
            current_path.truncate(heading_level.saturating_sub(1) as usize);
            current_path.push(format!("{} {}", prefix, h_text));

            current_heading = h_text;
            current_level = heading_level;
            current_lines = vec![line];
            chunk_start = offset;
        } else {
            current_lines.push(line);
        }

        offset += line.len() + 1; // +1 for '\n'
    }

    // Flush last chunk
    let content = current_lines.join("\n");
    if !content.trim().is_empty() {
        chunks.push(MarkdownChunk {
            heading: current_heading,
            level: current_level,
            path: current_path,
            content,
            source: source.map(|s| s.to_string()),
            offset: chunk_start,
        });
    }

    chunks
}

fn heading_level(line: &str) -> u8 {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return 0;
    }
    let count = trimmed.chars().take_while(|&c| c == '#').count();
    if count > 0 && count <= 6 && trimmed.chars().nth(count).map_or(false, |c| c == ' ') {
        count as u8
    } else {
        0
    }
}

// ─── Text utilities ───────────────────────────────────────────────────────────

/// Strip Markdown formatting, leaving plain text.
pub fn strip_markdown(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_code_block = false;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        // Remove heading markers
        let stripped = trimmed.trim_start_matches('#').trim();
        // Remove bold/italic
        let stripped = stripped
            .replace("**", "")
            .replace("__", "")
            .replace('*', "")
            .replace('_', "");
        // Remove inline code
        let stripped = stripped.replace('`', "");
        // Remove link syntax [text](url) → text
        let stripped = remove_links(&stripped);
        out.push_str(&stripped);
        out.push('\n');
    }
    out.trim().to_string()
}

fn remove_links(s: &str) -> String {
    // Very simplified: [text](url) → text
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '[' {
            let mut text = String::new();
            let mut found_close = false;
            for c in chars.by_ref() {
                if c == ']' {
                    found_close = true;
                    break;
                }
                text.push(c);
            }
            if found_close && chars.peek() == Some(&'(') {
                // Consume (url)
                chars.next();
                for c in chars.by_ref() {
                    if c == ')' {
                        break;
                    }
                }
                result.push_str(&text);
            } else {
                result.push('[');
                result.push_str(&text);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Count words in a Markdown string (after stripping).
pub fn word_count(markdown: &str) -> usize {
    strip_markdown(markdown).split_whitespace().count()
}

/// Estimate reading time in minutes at `wpm` words per minute.
pub fn reading_time_mins(markdown: &str, wpm: f32) -> f32 {
    word_count(markdown) as f32 / wpm.max(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_frontmatter_basic() {
        let doc = "---\ntitle: Hello\nauthor: Alice\n---\n# Body\nContent here.";
        let parsed = extract_frontmatter(doc);
        assert_eq!(
            parsed.frontmatter.get("title").map(|s| s.as_str()),
            Some("Hello")
        );
        assert_eq!(
            parsed.frontmatter.get("author").map(|s| s.as_str()),
            Some("Alice")
        );
        assert!(parsed.body.contains("# Body"));
    }

    #[test]
    fn extract_frontmatter_no_fm() {
        let doc = "# Title\nBody text.";
        let parsed = extract_frontmatter(doc);
        assert!(parsed.frontmatter.is_empty());
        assert_eq!(parsed.body, doc);
    }

    #[test]
    fn split_by_headings_basic() {
        let md = "Preamble text.\n\n# Section 1\n\nContent 1.\n\n## Subsection\n\nSub content.\n\n# Section 2\n\nContent 2.";
        let chunks = split_by_headings(md, 0, Some("test.md"));
        assert!(chunks.len() >= 3);
        assert_eq!(chunks[1].heading, "Section 1");
        assert_eq!(chunks[1].level, 1);
    }

    #[test]
    fn heading_level_detection() {
        assert_eq!(heading_level("# H1"), 1);
        assert_eq!(heading_level("## H2"), 2);
        assert_eq!(heading_level("### H3"), 3);
        assert_eq!(heading_level("#no space"), 0);
        assert_eq!(heading_level("not heading"), 0);
    }

    #[test]
    fn strip_markdown_basic() {
        let md = "# Title\n**bold** and *italic* text with `code`.";
        let plain = strip_markdown(md);
        assert!(!plain.contains('#'));
        assert!(!plain.contains("**"));
        assert!(!plain.contains('`'));
        assert!(plain.contains("bold"));
    }

    #[test]
    fn word_count_test() {
        let md = "# Hello\nThis is five words.";
        let count = word_count(md);
        assert!(count >= 5);
    }
}
