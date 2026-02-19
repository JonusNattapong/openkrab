pub fn redact_sensitive(input: &str) -> String {
    let mut out = input.to_string();
    for key in ["token", "apikey", "api_key", "secret", "password"] {
        out = out.replace(key, "[REDACTED_KEY]");
    }
    out
}

