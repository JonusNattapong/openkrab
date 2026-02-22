//! Ported from `openclaw/src/sessions/level-overrides.ts`

use super::{Session, VerbosityLevel};

pub fn parse_verbose_override(raw: Option<&str>) -> Result<Option<VerbosityLevel>, String> {
    match raw {
        None => Ok(None),
        Some(s) => {
            let s_lower = s.trim().to_lowercase();
            if s_lower.is_empty() || s_lower == "null" {
                return Ok(None);
            }
            if let Some(level) = parse_verbosity_level(&s_lower) {
                Ok(Some(level))
            } else {
                Err("invalid verboseLevel (use \"on\"|\"off\"|\"quiet\"|\"verbose\"|\"debug\"|\"normal\")".to_string())
            }
        }
    }
}

pub fn apply_verbose_override(entry: &mut Session, level: Option<Option<VerbosityLevel>>) {
    if let Some(lvl_opt) = level {
        if let Some(lvl) = lvl_opt {
            entry.verbosity = lvl;
        } else {
            entry.verbosity = VerbosityLevel::Normal;
        }
    }
}

pub fn parse_verbosity_level(s: &str) -> Option<VerbosityLevel> {
    match s {
        "on" | "verbose" | "v" => Some(VerbosityLevel::Verbose),
        "off" | "quiet" | "q" => Some(VerbosityLevel::Quiet),
        "debug" | "d" => Some(VerbosityLevel::Debug),
        "normal" | "n" => Some(VerbosityLevel::Normal),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_verbose_override() {
        assert_eq!(
            parse_verbose_override(Some("on")),
            Ok(Some(VerbosityLevel::Verbose))
        );
        assert_eq!(
            parse_verbose_override(Some("off")),
            Ok(Some(VerbosityLevel::Quiet))
        );
        assert_eq!(parse_verbose_override(None), Ok(None));
        assert_eq!(parse_verbose_override(Some("null")), Ok(None));
        assert!(parse_verbose_override(Some("invalid")).is_err());
    }

    #[test]
    fn test_apply_verbose_override() {
        let mut session = Session::new("test");
        session.verbosity = VerbosityLevel::Normal;

        apply_verbose_override(&mut session, Some(Some(VerbosityLevel::Verbose)));
        assert_eq!(session.verbosity, VerbosityLevel::Verbose);

        apply_verbose_override(&mut session, Some(None));
        assert_eq!(session.verbosity, VerbosityLevel::Normal);

        apply_verbose_override(&mut session, None);
        // Remains untouched.
        assert_eq!(session.verbosity, VerbosityLevel::Normal);
    }
}
