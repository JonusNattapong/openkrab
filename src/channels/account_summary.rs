use serde_json::{Map, Value};
use std::sync::Arc;

pub struct FormatAllowFromParams<'a> {
    pub cfg: &'a Value,
    pub account_id: Option<&'a str>,
    pub allow_from: &'a [Value],
}

pub type FormatAllowFromFn = Arc<dyn Fn(&FormatAllowFromParams) -> Vec<String> + Send + Sync>;

pub fn build_channel_account_snapshot(
    described: Option<Value>,
    account_id: &str,
    enabled: bool,
    configured: bool,
) -> Value {
    let mut map = Map::new();
    map.insert("enabled".to_string(), Value::Bool(enabled));
    map.insert("configured".to_string(), Value::Bool(configured));
    if let Some(desc) = described {
        if let Value::Object(obj) = desc {
            for (k, v) in obj.into_iter() {
                map.insert(k, v);
            }
        }
    }
    map.insert("accountId".to_string(), Value::String(account_id.to_string()));
    Value::Object(map)
}

pub fn format_channel_allow_from(
    format_fn: Option<FormatAllowFromFn>,
    cfg: &Value,
    account_id: Option<&str>,
    allow_from: &[Value],
) -> Vec<String> {
    if let Some(f) = format_fn {
        return f(&FormatAllowFromParams { cfg, account_id, allow_from });
    }
    allow_from
        .iter()
        .map(|entry| match entry {
            Value::String(s) => s.trim().to_string(),
            other => other.to_string().trim().to_string(),
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_snapshot_merges_described() {
        let desc = json!({"foo": "bar"});
        let snap = build_channel_account_snapshot(Some(desc), "a1", true, false);
        assert_eq!(snap["enabled"], Value::Bool(true));
        assert_eq!(snap["configured"], Value::Bool(false));
        assert_eq!(snap["foo"], Value::String("bar".to_string()));
        assert_eq!(snap["accountId"], Value::String("a1".to_string()));
    }

    #[test]
    fn format_allow_from_default() {
        let cfg = json!({});
        let allow = vec![json!(" alice "), json!(123), json!("" )];
        let out = format_channel_allow_from(None, &cfg, None, &allow);
        assert_eq!(out, vec!["alice".to_string(), "123".to_string()]);
    }
}
