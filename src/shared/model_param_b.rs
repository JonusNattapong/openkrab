//! Port of `openkrab/src/shared/model-param-b.ts`
//!
//! Infer the parameter count (in billions, "B") from a model ID or display name.
//! For example, `"llama-3.1-70b-instruct"` → `70.0`.

use regex::Regex;

/// Try to extract the largest `<N>B` parameter count from a model ID or name.
///
/// Matches patterns like `70b`, `3.1b` (case-insensitive), preceded by a
/// non-alphanumeric character (or start-of-string with one optional letter).
/// Returns the largest match, or `None` if no match is found.
///
/// # Examples
///
/// ```
/// use krabkrab::shared::model_param_b::infer_param_b_from_id_or_name;
///
/// assert_eq!(infer_param_b_from_id_or_name("llama-3.1-70b-instruct"), Some(70.0));
/// assert_eq!(infer_param_b_from_id_or_name("phi-3-mini"), None);
/// ```
pub fn infer_param_b_from_id_or_name(text: &str) -> Option<f64> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?i)(?:^|[^a-z0-9])[a-z]?(\d+(?:\.\d+)?)b(?:[^a-z0-9]|$)"
        ).unwrap();
    }

    let lower = text.to_lowercase();
    let mut best: Option<f64> = None;

    for caps in RE.captures_iter(&lower) {
        if let Some(m) = caps.get(1) {
            if let Ok(value) = m.as_str().parse::<f64>() {
                if value.is_finite() && value > 0.0 {
                    best = Some(match best {
                        Some(prev) if value > prev => value,
                        Some(prev) => prev,
                        None => value,
                    });
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_b_param() {
        assert_eq!(infer_param_b_from_id_or_name("llama-70b"), Some(70.0));
    }

    #[test]
    fn extracts_decimal() {
        assert_eq!(infer_param_b_from_id_or_name("phi-3.5b-mini"), Some(3.5));
    }

    #[test]
    fn picks_largest() {
        assert_eq!(
            infer_param_b_from_id_or_name("model-7b-vs-13b-comparison"),
            Some(13.0)
        );
    }

    #[test]
    fn returns_none_for_no_match() {
        assert_eq!(infer_param_b_from_id_or_name("gpt-4o-mini"), None);
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(infer_param_b_from_id_or_name("Llama-70B"), Some(70.0));
    }

    #[test]
    fn at_start_of_string() {
        assert_eq!(infer_param_b_from_id_or_name("8b-instruct"), Some(8.0));
    }

    #[test]
    fn ignores_zero() {
        assert_eq!(infer_param_b_from_id_or_name("model-0b"), None);
    }
}
