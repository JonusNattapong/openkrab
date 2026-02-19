use once_cell::sync::Lazy;
use regex::Regex;

static MEDIA_TOKEN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\bMEDIA:\s*`?([^\n]+)`?").unwrap());

pub fn normalize_media_source(src: &str) -> String {
    if src.starts_with("file://") {
        src[7..].to_string()
    } else {
        src.to_string()
    }
}

fn clean_candidate(raw: &str) -> String {
    raw.trim_start_matches(|c| {
        c == '`' || c == '"' || c == '\'' || c == '[' || c == '{' || c == '('
    })
    .trim_end_matches(|c| {
        c == '`'
            || c == '"'
            || c == '\''
            || c == ']'
            || c == '}'
            || c == ')'
            || c == ','
            || c == '\\'
    })
    .to_string()
}

static WINDOWS_DRIVE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z]:[\\/]").unwrap());
static SCHEME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*:").unwrap());
static HAS_FILE_EXT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.\w{1,10}$").unwrap());

fn is_likely_local_path(candidate: &str) -> bool {
    candidate.starts_with('/')
        || candidate.starts_with("./")
        || candidate.starts_with("../")
        || candidate.starts_with('~')
        || WINDOWS_DRIVE_RE.is_match(candidate)
        || candidate.starts_with("\\\\")
        || (!SCHEME_RE.is_match(candidate) && (candidate.contains('/') || candidate.contains('\\')))
}

fn is_valid_media(candidate: &str, allow_spaces: bool, allow_bare_filename: bool) -> bool {
    if candidate.is_empty() || candidate.len() > 4096 {
        return false;
    }

    if !allow_spaces && candidate.contains(char::is_whitespace) {
        return false;
    }

    if candidate.starts_with("http://") || candidate.starts_with("https://") {
        return true;
    }

    if is_likely_local_path(candidate) {
        return true;
    }

    if allow_bare_filename && !SCHEME_RE.is_match(candidate) && HAS_FILE_EXT.is_match(candidate) {
        return true;
    }

    false
}

fn unwrap_quoted(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.len() < 2 {
        return None;
    }

    let first = trimmed.chars().next()?;
    let last = trimmed.chars().last()?;

    if first != last {
        return None;
    }

    if first != '"' && first != '\'' && first != '`' {
        return None;
    }

    Some(trimmed[1..trimmed.len() - 1].trim().to_string())
}

#[derive(Debug, Clone, Default)]
pub struct SplitMediaResult {
    pub text: String,
    pub media_urls: Option<Vec<String>>,
    pub media_url: Option<String>,
    pub audio_as_voice: Option<bool>,
}

pub fn split_media_from_output(raw: &str) -> SplitMediaResult {
    let trimmed_raw = raw.trim_end();
    if trimmed_raw.trim().is_empty() {
        return SplitMediaResult {
            text: String::new(),
            ..Default::default()
        };
    }

    let mut media: Vec<String> = Vec::new();
    let mut found_media_token = false;

    let lines: Vec<&str> = trimmed_raw.lines().collect();
    let mut kept_lines: Vec<String> = Vec::new();

    for line in lines {
        let trimmed_start = line.trim_start();

        if !trimmed_start.starts_with("MEDIA:") {
            kept_lines.push(line.to_string());
            continue;
        }

        let matches: Vec<_> = MEDIA_TOKEN_RE.captures_iter(line).collect();
        if matches.is_empty() {
            kept_lines.push(line.to_string());
            continue;
        }

        for caps in matches {
            let payload = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let unwrapped = unwrap_quoted(payload);
            let payload_value = unwrapped.as_deref().unwrap_or(payload);

            let parts: Vec<&str> = if unwrapped.is_some() {
                vec![unwrapped.as_deref().unwrap()]
            } else {
                payload.split_whitespace().collect()
            };

            let mut has_valid_media = false;
            for part in parts {
                let candidate = normalize_media_source(&clean_candidate(part));
                if is_valid_media(&candidate, unwrapped.is_some(), false) {
                    media.push(candidate);
                    has_valid_media = true;
                    found_media_token = true;
                }
            }

            if !has_valid_media {
                let fallback = normalize_media_source(&clean_candidate(payload_value));
                if is_valid_media(&fallback, true, true) {
                    media.push(fallback);
                    found_media_token = true;
                }
            }
        }

        let cleaned_line = line
            .replace(
                MEDIA_TOKEN_RE.find(line).map(|m| m.as_str()).unwrap_or(""),
                "",
            )
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        if !cleaned_line.is_empty() {
            kept_lines.push(cleaned_line);
        }
    }

    let cleaned_text = kept_lines
        .join("\n")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    let has_audio_as_voice = cleaned_text.contains("[[audio_as_voice]]");
    let final_text = if has_audio_as_voice {
        cleaned_text
            .replace("[[audio_as_voice]]", "")
            .trim()
            .to_string()
    } else {
        cleaned_text
    };

    if media.is_empty() {
        return SplitMediaResult {
            text: if found_media_token || has_audio_as_voice {
                final_text
            } else {
                trimmed_raw.to_string()
            },
            audio_as_voice: if has_audio_as_voice { Some(true) } else { None },
            ..Default::default()
        };
    }

    SplitMediaResult {
        text: final_text,
        media_urls: Some(media.clone()),
        media_url: media.into_iter().next(),
        audio_as_voice: if has_audio_as_voice { Some(true) } else { None },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_media_source() {
        assert_eq!(
            normalize_media_source("file:///path/to/file"),
            "/path/to/file"
        );
        assert_eq!(normalize_media_source("/path/to/file"), "/path/to/file");
    }

    #[test]
    fn test_is_likely_local_path() {
        assert!(is_likely_local_path("/path/to/file"));
        assert!(is_likely_local_path("./file"));
        assert!(is_likely_local_path("~/file"));
        assert!(is_likely_local_path("C:\\Windows\\file"));
        assert!(!is_likely_local_path("http://example.com/file"));
    }

    #[test]
    fn test_split_media_from_output() {
        let result =
            split_media_from_output("Here's the image:\nMEDIA: https://example.com/image.png");
        assert!(result.media_urls.is_some());
        assert_eq!(
            result.media_url,
            Some("https://example.com/image.png".to_string())
        );
        assert!(!result.text.contains("MEDIA:"));
    }

    #[test]
    fn test_split_media_with_audio_tag() {
        let result = split_media_from_output("Text [[audio_as_voice]]\nMEDIA: /path/to/audio.mp3");
        assert_eq!(result.audio_as_voice, Some(true));
        assert!(!result.text.contains("[[audio_as_voice]]"));
    }
}
