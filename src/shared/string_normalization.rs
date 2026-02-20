//! String normalization utilities

/// Normalize a string for comparison (lowercase, trim, collapse whitespace)
pub fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normalize for case-insensitive comparison only
pub fn normalize_case(input: &str) -> String {
    input.to_lowercase()
}

/// Normalize whitespace (collapse multiple spaces, trim)
pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Normalize for slug/identifier (lowercase, alphanumeric only, hyphenated)
pub fn normalize_slug(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Normalize email address (lowercase, trim)
pub fn normalize_email(input: &str) -> Option<String> {
    let trimmed = input.trim().to_lowercase();
    if trimmed.contains('@') && !trimmed.starts_with('@') && !trimmed.ends_with('@') {
        Some(trimmed)
    } else {
        None
    }
}

/// Normalize phone number (digits only, with optional leading +)
pub fn normalize_phone(input: &str) -> Option<String> {
    let mut result = String::new();
    let mut chars = input.trim().chars();

    // Keep leading +
    if let Some(first) = chars.next() {
        if first == '+' {
            result.push(first);
        } else if first.is_ascii_digit() {
            result.push(first);
        }
    }

    // Keep only digits
    for ch in chars {
        if ch.is_ascii_digit() {
            result.push(ch);
        }
    }

    if result.len() >= 7 {
        Some(result)
    } else {
        None
    }
}

/// Normalize URL (lowercase scheme and host, trim)
pub fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();

    // Try to parse as URL
    if let Some((scheme, rest)) = trimmed.split_once("://") {
        let scheme_lower = scheme.to_lowercase();
        let rest_lower = if let Some((host, path)) = rest.split_once('/') {
            format!("{}/{}", host.to_lowercase(), path)
        } else {
            rest.to_lowercase()
        };
        format!("{}://{}", scheme_lower, rest_lower)
    } else {
        trimmed.to_lowercase()
    }
}

/// Remove diacritics from string (basic implementation)
pub fn remove_diacritics(input: &str) -> String {
    // Basic ASCII folding - for full Unicode support, use unicode-normalization crate
    input
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' | 'ã' | 'å' | 'ā' => 'a',
            'é' | 'è' | 'ë' | 'ê' | 'ē' | 'ė' => 'e',
            'í' | 'ì' | 'ï' | 'î' | 'ī' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' | 'ō' => 'o',
            'ú' | 'ù' | 'ü' | 'û' | 'ū' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            'Á' | 'À' | 'Ä' | 'Â' | 'Ã' | 'Å' | 'Ā' => 'A',
            'É' | 'È' | 'Ë' | 'Ê' | 'Ē' | 'Ė' => 'E',
            'Í' | 'Ì' | 'Ï' | 'Î' | 'Ī' => 'I',
            'Ó' | 'Ò' | 'Ö' | 'Ô' | 'Õ' | 'Ō' => 'O',
            'Ú' | 'Ù' | 'Ü' | 'Û' | 'Ū' => 'U',
            'Ñ' => 'N',
            'Ç' => 'C',
            _ => c,
        })
        .collect()
}

/// Compare two strings for equality after normalization
pub fn normalized_eq(a: &str, b: &str) -> bool {
    normalize(a) == normalize(b)
}

/// Check if normalized string contains substring
pub fn normalized_contains(haystack: &str, needle: &str) -> bool {
    normalize(haystack).contains(&normalize(needle))
}

/// Truncate string to max length with ellipsis
pub fn truncate_with_ellipsis(input: &str, max_len: usize) -> String {
    if input.len() <= max_len {
        input.to_string()
    } else {
        format!("{}...", &input[..max_len.saturating_sub(3)])
    }
}

/// Normalize newlines to \n
pub fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("  Hello   World  "), "hello world");
        assert_eq!(normalize("HELLO"), "hello");
    }

    #[test]
    fn test_normalize_slug() {
        assert_eq!(normalize_slug("Hello World"), "hello-world");
        assert_eq!(normalize_slug("Hello--World!!"), "hello-world");
        assert_eq!(normalize_slug("  test  "), "test");
    }

    #[test]
    fn test_normalize_email() {
        assert_eq!(
            normalize_email("  Test@Example.COM  "),
            Some("test@example.com".to_string())
        );
        assert_eq!(normalize_email("not-an-email"), None);
        assert_eq!(normalize_email("@example.com"), None);
    }

    #[test]
    fn test_normalize_phone() {
        assert_eq!(
            normalize_phone("+1 (555) 123-4567"),
            Some("+15551234567".to_string())
        );
        assert_eq!(normalize_phone("555-1234"), Some("5551234".to_string()));
        assert_eq!(normalize_phone("123"), None);
    }

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_diacritics("café"), "cafe");
        assert_eq!(remove_diacritics("naïve"), "naive");
        assert_eq!(remove_diacritics("résumé"), "resume");
    }

    #[test]
    fn test_normalized_eq() {
        assert!(normalized_eq("Hello World", "  hello   WORLD  "));
        assert!(!normalized_eq("Hello", "World"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
    }
}
