use anyhow::anyhow;

pub fn missing_target_error(provider: &str, hint: Option<&str>) -> anyhow::Error {
    anyhow!(missing_target_message(provider, hint))
}

fn missing_target_message(provider: &str, hint: Option<&str>) -> String {
    format!(
        "Delivering to {} requires target{}",
        provider,
        format_target_hint(hint, false)
    )
}

fn format_target_hint(hint: Option<&str>, with_label: bool) -> String {
    match hint {
        Some(h) if !h.is_empty() => {
            if with_label {
                format!(" Hint: {}", h)
            } else {
                format!(" {}", h)
            }
        }
        _ => String::new(),
    }
}
