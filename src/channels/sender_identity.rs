use regex::Regex;

#[derive(Debug, Default)]
pub struct MsgContext {
    pub chat_type: Option<String>,
    pub sender_id: Option<String>,
    pub sender_name: Option<String>,
    pub sender_username: Option<String>,
    pub sender_e164: Option<String>,
}

fn normalize_chat_type(t: Option<&str>) -> String {
    match t.unwrap_or("").trim() {
        "direct" => "direct".to_string(),
        other => other.to_string(),
    }
}

pub fn validate_sender_identity(ctx: &MsgContext) -> Vec<String> {
    let mut issues: Vec<String> = Vec::new();

    let chat_type = normalize_chat_type(ctx.chat_type.as_deref());
    let is_direct = chat_type == "direct";

    let sender_id = ctx.sender_id.as_deref().unwrap_or("").trim();
    let sender_name = ctx.sender_name.as_deref().unwrap_or("").trim();
    let sender_username = ctx.sender_username.as_deref().unwrap_or("").trim();
    let sender_e164 = ctx.sender_e164.as_deref().unwrap_or("").trim();

    if !is_direct {
        if sender_id.is_empty()
            && sender_name.is_empty()
            && sender_username.is_empty()
            && sender_e164.is_empty()
        {
            issues.push(
                "missing sender identity (sender_id/sender_name/sender_username/sender_e164)"
                    .to_string(),
            );
        }
    }

    if !sender_e164.is_empty() {
        let re = Regex::new(r"^\+\d{3,}$").unwrap();
        if !re.is_match(sender_e164) {
            issues.push(format!("invalid SenderE164: {}", sender_e164));
        }
    }

    if !sender_username.is_empty() {
        if sender_username.contains('@') {
            issues.push(format!(
                "SenderUsername should not include \"@\": {}",
                sender_username
            ));
        }
        if sender_username.chars().any(|c| c.is_whitespace()) {
            issues.push(format!(
                "SenderUsername should not include whitespace: {}",
                sender_username
            ));
        }
    }

    if ctx.sender_id.is_some() && sender_id.is_empty() {
        issues.push("sender_id is set but empty".to_string());
    }

    issues
}
