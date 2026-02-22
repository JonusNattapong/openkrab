//! Ported from `openclaw/src/sessions/session-label.ts`

pub const SESSION_LABEL_MAX_LENGTH: usize = 64;

pub fn parse_session_label(raw: &str) -> std::result::Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("invalid label: empty".to_string());
    }
    if trimmed.len() > SESSION_LABEL_MAX_LENGTH {
        return Err(format!(
            "invalid label: too long (max {})",
            SESSION_LABEL_MAX_LENGTH
        ));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_label() {
        assert_eq!(parse_session_label("  hello  "), Ok("hello".to_string()));
        assert_eq!(
            parse_session_label("a max length string---------------------------------------------"),
            Ok("a max length string---------------------------------------------".to_string())
        );
    }

    #[test]
    fn empty_label() {
        assert!(parse_session_label("   ").is_err());
        assert!(parse_session_label("").is_err());
    }

    #[test]
    fn too_long_label() {
        let long_label = "a".repeat(65);
        assert!(parse_session_label(&long_label).is_err());
    }
}
