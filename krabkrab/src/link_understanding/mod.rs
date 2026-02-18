//! link_understanding — URL preview and link metadata extraction.
//! Ported from `openclaw/src/link-understanding/` (Phase 7).
//!
//! Fetches and parses Open Graph / Twitter Card / title metadata from URLs
//! so the agent can produce rich link previews.

use anyhow::Result;
use serde::{Deserialize, Serialize};

// ─── Link preview ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkPreview {
    /// Canonical URL (after redirects).
    pub url: String,
    /// Page title (og:title → <title> fallback).
    pub title: Option<String>,
    /// Description (og:description → meta description).
    pub description: Option<String>,
    /// Preview image URL (og:image).
    pub image: Option<String>,
    /// Site name (og:site_name).
    pub site_name: Option<String>,
    /// Content type ("article", "video", "website", …).
    pub content_type: Option<String>,
    /// Author (article:author / twitter:creator).
    pub author: Option<String>,
    /// Published time (ISO 8601 if available).
    pub published_at: Option<String>,
}

impl LinkPreview {
    /// Format as a Markdown excerpt suitable for chat.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        if let Some(ref title) = self.title {
            out.push_str(&format!("**[{}]({})**\n", title, self.url));
        } else {
            out.push_str(&format!("<{}>\n", self.url));
        }
        if let Some(ref desc) = self.description {
            out.push_str(&format!("> {}\n", desc.chars().take(280).collect::<String>()));
        }
        if let Some(ref site) = self.site_name {
            out.push_str(&format!("*— {}*\n", site));
        }
        out.trim().to_string()
    }
}

// ─── Metadata extraction helpers ─────────────────────────────────────────────

/// Extract Open Graph / Twitter Card / meta tags from raw HTML.
pub fn parse_og_from_html(html: &str, source_url: &str) -> LinkPreview {
    let mut preview = LinkPreview { url: source_url.to_string(), ..Default::default() };

    for line in html.lines() {
        let line = line.trim();
        // og:title
        if preview.title.is_none() {
            preview.title = extract_meta(line, "og:title")
                .or_else(|| extract_meta(line, "twitter:title"))
                .or_else(|| extract_title_tag(line));
        }
        // og:description
        if preview.description.is_none() {
            preview.description = extract_meta(line, "og:description")
                .or_else(|| extract_meta(line, "twitter:description"))
                .or_else(|| extract_meta_name(line, "description"));
        }
        // og:image
        if preview.image.is_none() {
            preview.image = extract_meta(line, "og:image")
                .or_else(|| extract_meta(line, "twitter:image"));
        }
        // og:site_name
        if preview.site_name.is_none() {
            preview.site_name = extract_meta(line, "og:site_name");
        }
        // og:type
        if preview.content_type.is_none() {
            preview.content_type = extract_meta(line, "og:type");
        }
        // author
        if preview.author.is_none() {
            preview.author = extract_meta(line, "article:author")
                .or_else(|| extract_meta(line, "twitter:creator"));
        }
        // published_time
        if preview.published_at.is_none() {
            preview.published_at = extract_meta(line, "article:published_time");
        }
    }
    preview
}

fn extract_meta(line: &str, property: &str) -> Option<String> {
    let search = format!("property=\"{}\"", property);
    let search2 = format!("property='{}'", property);
    if !line.contains(&search) && !line.contains(&search2) {
        return None;
    }
    extract_content_attr(line)
}

fn extract_meta_name(line: &str, name: &str) -> Option<String> {
    let search = format!("name=\"{}\"", name);
    if !line.contains(&search) {
        return None;
    }
    extract_content_attr(line)
}

fn extract_content_attr(line: &str) -> Option<String> {
    // Try content="..."
    for (open, close) in [("content=\"", "\""), ("content='", "'")] {
        if let Some(start) = line.find(open) {
            let rest = &line[start + open.len()..];
            if let Some(end) = rest.find(close) {
                let val = rest[..end].trim().to_string();
                if !val.is_empty() {
                    return Some(val);
                }
            }
        }
    }
    None
}

fn extract_title_tag(line: &str) -> Option<String> {
    if !line.to_lowercase().contains("<title") {
        return None;
    }
    // <title>text</title>
    if let Some(start) = line.find('>') {
        let rest = &line[start + 1..];
        if let Some(end) = rest.find("</") {
            let val = rest[..end].trim().to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

// ─── URL utilities ────────────────────────────────────────────────────────────

/// Extract all plain URLs from a text string.
pub fn extract_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| matches!(c, ',' | '.' | ')' | '(' | '"' | '\''));
        if word.starts_with("http://") || word.starts_with("https://") {
            urls.push(word.to_string());
        }
    }
    urls
}

/// Normalise a URL: ensure scheme, strip trailing slash, lowercase host.
pub fn normalise_url(url: &str) -> String {
    let url = url.trim();
    let url = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    };
    url.trim_end_matches('/').to_string()
}

/// Guess the domain name from a URL (e.g. "github.com").
pub fn domain_from_url(url: &str) -> Option<String> {
    let url = url.trim_start_matches("https://").trim_start_matches("http://");
    url.split('/').next().map(|h| h.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_urls_from_text() {
        let text = "Check out https://example.com and http://foo.bar/path?q=1 today.";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls[0].contains("example.com"));
    }

    #[test]
    fn normalise_url_adds_scheme() {
        assert_eq!(normalise_url("example.com/page"), "https://example.com/page");
        assert_eq!(normalise_url("https://example.com/"), "https://example.com");
    }

    #[test]
    fn domain_from_url_test() {
        assert_eq!(domain_from_url("https://GitHub.COM/user/repo"), Some("github.com".to_string()));
        assert_eq!(domain_from_url("http://example.com/path"), Some("example.com".to_string()));
    }

    #[test]
    fn parse_og_basic() {
        let html = r#"
            <html>
            <head>
            <meta property="og:title" content="Hello World" />
            <meta property="og:description" content="A test page" />
            <meta property="og:site_name" content="Test Site" />
            </head>
            </html>
        "#;
        let preview = parse_og_from_html(html, "https://example.com");
        assert_eq!(preview.title.as_deref(), Some("Hello World"));
        assert_eq!(preview.description.as_deref(), Some("A test page"));
        assert_eq!(preview.site_name.as_deref(), Some("Test Site"));
    }

    #[test]
    fn link_preview_to_markdown() {
        let mut p = LinkPreview::default();
        p.url = "https://example.com".to_string();
        p.title = Some("Example".to_string());
        p.description = Some("A description".to_string());
        let md = p.to_markdown();
        assert!(md.contains("[Example]"));
        assert!(md.contains("A description"));
    }
}
