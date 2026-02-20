use encoding_rs;
use regex::Regex;

lazy_static::lazy_static! {
    static ref WORDISH_CHAR: Regex = Regex::new(r"[\p{L}\p{N}]").unwrap();
}

pub fn get_text_stats(text: &str) -> (f64, f64) {
    if text.is_empty() {
        return (0.0, 0.0);
    }

    let mut printable = 0;
    let mut control = 0;
    let mut wordish = 0;

    for ch in text.chars() {
        let code = ch as u32;
        if code == 9 || code == 10 || code == 13 || code == 32 {
            printable += 1;
            wordish += 1;
            continue;
        }
        if code < 32 || (code >= 0x7f && code <= 0x9f) {
            control += 1;
            continue;
        }
        printable += 1;
        if WORDISH_CHAR.is_match(&ch.to_string()) {
            wordish += 1;
        }
    }

    let total = printable + control;
    if total == 0 {
        (0.0, 0.0)
    } else {
        (
            printable as f64 / total as f64,
            wordish as f64 / total as f64,
        )
    }
}

pub fn is_mostly_printable(text: &str) -> bool {
    get_text_stats(text).0 > 0.85
}

pub fn looks_like_legacy_text_bytes(buffer: &[u8]) -> bool {
    if buffer.is_empty() {
        return false;
    }

    let text = decode_legacy_text(buffer);
    let (printable_ratio, wordish_ratio) = get_text_stats(&text);
    printable_ratio > 0.95 && wordish_ratio > 0.3
}

pub fn looks_like_utf8_text(buffer: Option<&[u8]>) -> bool {
    let buffer = match buffer {
        Some(b) if !b.is_empty() => b,
        _ => return false,
    };

    let sample = &buffer[..std::cmp::min(buffer.len(), 4096)];
    match std::str::from_utf8(sample) {
        Ok(text) => is_mostly_printable(text),
        Err(_) => looks_like_legacy_text_bytes(sample),
    }
}

pub fn decode_text_sample(buffer: Option<&[u8]>) -> String {
    let buffer = match buffer {
        Some(b) if !b.is_empty() => b,
        _ => return String::new(),
    };

    let sample = &buffer[..std::cmp::min(buffer.len(), 8192)];
    let utf16_charset = resolve_utf16_charset(sample);

    match utf16_charset {
        Some("utf-16be") => {
            let mut swapped = Vec::with_capacity(sample.len());
            for i in (0..sample.len()).step_by(2) {
                if i + 1 < sample.len() {
                    swapped.push(sample[i + 1]);
                    swapped.push(sample[i]);
                } else {
                    swapped.push(sample[i]);
                }
            }
            let (cow, _, _) = encoding_rs::UTF_16LE.decode(&swapped);
            cow.to_string()
        }
        Some("utf-16le") => {
            let (cow, _, _) = encoding_rs::UTF_16LE.decode(sample);
            cow.to_string()
        }
        _ => String::from_utf8_lossy(sample).to_string(),
    }
}

pub fn guess_delimited_mime(text: &str) -> Option<String> {
    if text.is_empty() {
        return None;
    }

    let line = text.lines().next().unwrap_or("");
    let tabs = line.chars().filter(|&c| c == '\t').count();
    let commas = line.chars().filter(|&c| c == ',').count();

    if commas > 0 {
        Some("text/csv".to_string())
    } else if tabs > 0 {
        Some("text/tab-separated-values".to_string())
    } else {
        None
    }
}

pub fn resolve_text_mime_from_name(name: Option<&str>) -> Option<String> {
    let name = name?;
    let ext = std::path::Path::new(name)
        .extension()?
        .to_str()?
        .to_lowercase();

    match ext.as_str() {
        "csv" => Some("text/csv".to_string()),
        "tsv" => Some("text/tab-separated-values".to_string()),
        "txt" | "log" | "ini" | "cfg" | "conf" | "env" => Some("text/plain".to_string()),
        "md" => Some("text/markdown".to_string()),
        "json" => Some("application/json".to_string()),
        "yaml" | "yml" => Some("text/yaml".to_string()),
        "xml" => Some("application/xml".to_string()),
        _ => None,
    }
}

fn decode_legacy_text(buffer: &[u8]) -> String {
    // Simple CP1252 decoding - in practice, you'd use a proper encoding library
    let mut output = String::new();
    for &byte in buffer {
        if byte >= 0x80 && byte <= 0x9f {
            // Use Unicode replacement for simplicity
            output.push(char::REPLACEMENT_CHARACTER);
        } else {
            output.push(byte as char);
        }
    }
    output
}

fn resolve_utf16_charset(buffer: &[u8]) -> Option<&'static str> {
    if buffer.len() < 2 {
        return None;
    }

    if buffer[0] == 0xFF && buffer[1] == 0xFE {
        return Some("utf-16le");
    }
    if buffer[0] == 0xFE && buffer[1] == 0xFF {
        return Some("utf-16be");
    }

    let sample_len = std::cmp::min(buffer.len(), 2048);
    let mut zero_even = 0;
    let mut zero_odd = 0;

    for i in 0..sample_len {
        if buffer[i] != 0 {
            continue;
        }
        if i % 2 == 0 {
            zero_even += 1;
        } else {
            zero_odd += 1;
        }
    }

    let zero_count = zero_even + zero_odd;
    if zero_count as f64 / sample_len as f64 > 0.2 {
        if zero_odd >= zero_even {
            Some("utf-16le")
        } else {
            Some("utf-16be")
        }
    } else {
        None
    }
}

pub fn sanitize_mime_type(value: Option<&str>) -> Option<String> {
    let value = value?;
    let trimmed = value.trim().to_lowercase();
    if trimmed.is_empty() {
        return None;
    }

    let re = regex::Regex::new(r"^([a-z0-9!#$&^_.+-]+/[a-z0-9!#$&^_.+-]+)").unwrap();
    re.captures(&trimmed)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

pub fn xml_escape_attr(value: &str) -> String {
    value
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

pub fn escape_file_block_content(value: &str) -> String {
    value
        .replace(r"<\s*/\s*file\s*>", "&lt;/file&gt;")
        .replace(r"<\s*file\b", "&lt;file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_text_stats() {
        let (printable, wordish) = get_text_stats("Hello World");
        assert!(printable > 0.9);
        assert!(wordish > 0.0);
    }

    #[test]
    fn test_is_mostly_printable() {
        assert!(is_mostly_printable("Hello World"));
        assert!(!is_mostly_printable(""));
    }

    #[test]
    fn test_guess_delimited_mime() {
        assert_eq!(
            guess_delimited_mime("name,email\nJohn,john@example.com"),
            Some("text/csv".to_string())
        );
        assert_eq!(
            guess_delimited_mime("name\temail\nJohn\tjohn@example.com"),
            Some("text/tab-separated-values".to_string())
        );
        assert_eq!(guess_delimited_mime("plain text"), None);
    }

    #[test]
    fn test_resolve_text_mime_from_name() {
        assert_eq!(
            resolve_text_mime_from_name(Some("file.csv")),
            Some("text/csv".to_string())
        );
        assert_eq!(
            resolve_text_mime_from_name(Some("file.json")),
            Some("application/json".to_string())
        );
        assert_eq!(resolve_text_mime_from_name(Some("file.unknown")), None);
    }

    #[test]
    fn test_xml_escape_attr() {
        assert_eq!(xml_escape_attr("<>&\"'"), "&lt;&gt;&amp;&quot;&apos;");
    }
}
