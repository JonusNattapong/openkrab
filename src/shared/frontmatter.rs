//! Port of `openkrab/src/shared/frontmatter.ts`
//!
//! Frontmatter and manifest parsing utilities for plugins/extensions.
//! Resolves KrabKrab manifest blocks from YAML/JSON5 frontmatter and extracts
//! requirement declarations, install specs, OS filters, etc.

use serde_json::Value;

// ─── String list normalization ──────────────────────────────────────────────

/// Normalize an input value to a list of non-empty trimmed strings.
///
/// Accepts:
/// - A JSON array → each element is stringified and trimmed
/// - A JSON string → split on commas and trimmed
/// - Anything else → empty vec
pub fn normalize_string_list(input: &Value) -> Vec<String> {
    match input {
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| {
                let s = match v {
                    Value::String(s) => s.trim().to_string(),
                    _ => format!("{}", v).trim().to_string(),
                };
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .collect(),
        Value::String(s) => s
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

/// Get a string value from a frontmatter map.
pub fn get_frontmatter_string<'a>(
    frontmatter: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Option<&'a str> {
    frontmatter.get(key).and_then(|v| v.as_str())
}

/// Parse a boolean value from a string, returning `fallback` if unparseable.
pub fn parse_frontmatter_bool(value: Option<&str>, fallback: bool) -> bool {
    match value {
        None => fallback,
        Some(s) => {
            let lower = s.trim().to_lowercase();
            match lower.as_str() {
                "true" | "yes" | "1" | "on" => true,
                "false" | "no" | "0" | "off" => false,
                _ => fallback,
            }
        }
    }
}

// ─── Manifest block resolution ──────────────────────────────────────────────

/// Known manifest key names (current + legacy).
const MANIFEST_KEY: &str = "krabkrab";
const LEGACY_MANIFEST_KEYS: &[&str] = &["openkrab", "krab"];

/// Try to resolve an KrabKrab manifest block from parsed frontmatter
/// metadata.
///
/// Looks for a JSON5 string under `metadata` (or custom key), parses it, then
/// searches for a top-level key matching one of the known manifest keys.
pub fn resolve_manifest_block(
    frontmatter: &serde_json::Map<String, Value>,
    custom_key: Option<&str>,
) -> Option<serde_json::Map<String, Value>> {
    let key = custom_key.unwrap_or("metadata");
    let raw = get_frontmatter_string(frontmatter, key)?;

    // Parse as JSON (JSON5 would require an extra crate; standard JSON covers
    // the majority of real manifests).
    let parsed: Value = serde_json::from_str(raw).ok()?;
    let obj = parsed.as_object()?;

    // Search for manifest keys in priority order
    let all_keys = std::iter::once(MANIFEST_KEY).chain(LEGACY_MANIFEST_KEYS.iter().copied());
    for k in all_keys {
        if let Some(Value::Object(manifest)) = obj.get(k) {
            return Some(manifest.clone());
        }
    }
    None
}

// ─── Manifest requirements ──────────────────────────────────────────────────

/// Parsed requirements from a manifest block.
#[derive(Debug, Clone, Default)]
pub struct ManifestRequires {
    pub bins: Vec<String>,
    pub any_bins: Vec<String>,
    pub env: Vec<String>,
    pub config: Vec<String>,
}

/// Extract `requires` from a manifest metadata object.
pub fn resolve_manifest_requires(
    metadata_obj: &serde_json::Map<String, Value>,
) -> Option<ManifestRequires> {
    let requires_raw = metadata_obj.get("requires")?.as_object()?;
    Some(ManifestRequires {
        bins: normalize_string_list(requires_raw.get("bins").unwrap_or(&Value::Null)),
        any_bins: normalize_string_list(requires_raw.get("anyBins").unwrap_or(&Value::Null)),
        env: normalize_string_list(requires_raw.get("env").unwrap_or(&Value::Null)),
        config: normalize_string_list(requires_raw.get("config").unwrap_or(&Value::Null)),
    })
}

/// Extract the `os` filter list from a manifest metadata object.
pub fn resolve_manifest_os(metadata_obj: &serde_json::Map<String, Value>) -> Vec<String> {
    normalize_string_list(metadata_obj.get("os").unwrap_or(&Value::Null))
}

// ─── Install spec parsing ───────────────────────────────────────────────────

/// Base fields parsed from an install spec entry.
#[derive(Debug, Clone)]
pub struct ParsedManifestInstallBase {
    pub raw: serde_json::Map<String, Value>,
    pub kind: String,
    pub id: Option<String>,
    pub label: Option<String>,
    pub bins: Option<Vec<String>>,
}

/// Parse an install spec entry if its `kind` (or `type`) is in `allowed_kinds`.
pub fn parse_manifest_install_base(
    input: &Value,
    allowed_kinds: &[&str],
) -> Option<ParsedManifestInstallBase> {
    let obj = input.as_object()?;

    let kind_raw = obj
        .get("kind")
        .and_then(|v| v.as_str())
        .or_else(|| obj.get("type").and_then(|v| v.as_str()))
        .unwrap_or("");
    let kind = kind_raw.trim().to_lowercase();

    if !allowed_kinds.contains(&kind.as_str()) {
        return None;
    }

    let id = obj
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let label = obj
        .get("label")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let bins_list = normalize_string_list(obj.get("bins").unwrap_or(&Value::Null));
    let bins = if bins_list.is_empty() {
        None
    } else {
        Some(bins_list)
    };

    Some(ParsedManifestInstallBase {
        raw: obj.clone(),
        kind,
        id,
        label,
        bins,
    })
}

/// Resolve install entries from a manifest, filtering through a parser.
pub fn resolve_manifest_install<F, T>(
    metadata_obj: &serde_json::Map<String, Value>,
    parse_fn: F,
) -> Vec<T>
where
    F: Fn(&Value) -> Option<T>,
{
    let install_raw = match metadata_obj.get("install") {
        Some(Value::Array(arr)) => arr,
        _ => return Vec::new(),
    };
    install_raw.iter().filter_map(|e| parse_fn(e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn normalize_string_list_from_array() {
        let input = json!(["hello", " world ", "", "foo"]);
        assert_eq!(normalize_string_list(&input), vec!["hello", "world", "foo"]);
    }

    #[test]
    fn normalize_string_list_from_csv_string() {
        let input = json!("a, b, , c");
        assert_eq!(normalize_string_list(&input), vec!["a", "b", "c"]);
    }

    #[test]
    fn normalize_string_list_from_null() {
        assert!(normalize_string_list(&json!(null)).is_empty());
    }

    #[test]
    fn parse_frontmatter_bool_true() {
        assert!(parse_frontmatter_bool(Some("true"), false));
        assert!(parse_frontmatter_bool(Some("yes"), false));
        assert!(parse_frontmatter_bool(Some("1"), false));
    }

    #[test]
    fn parse_frontmatter_bool_false() {
        assert!(!parse_frontmatter_bool(Some("false"), true));
        assert!(!parse_frontmatter_bool(Some("no"), true));
        assert!(!parse_frontmatter_bool(Some("0"), true));
    }

    #[test]
    fn parse_frontmatter_bool_fallback() {
        assert!(parse_frontmatter_bool(Some("maybe"), true));
        assert!(parse_frontmatter_bool(None, true));
        assert!(!parse_frontmatter_bool(None, false));
    }

    #[test]
    fn parse_install_base_valid() {
        let input = json!({
            "kind": "npm",
            "id": "my-plugin",
            "label": "My Plugin",
            "bins": ["my-bin"]
        });
        let result = parse_manifest_install_base(&input, &["npm", "cargo"]).unwrap();
        assert_eq!(result.kind, "npm");
        assert_eq!(result.id.as_deref(), Some("my-plugin"));
        assert_eq!(result.bins.as_deref(), Some(&["my-bin".to_string()][..]));
    }

    #[test]
    fn parse_install_base_unknown_kind() {
        let input = json!({"kind": "pip"});
        assert!(parse_manifest_install_base(&input, &["npm", "cargo"]).is_none());
    }

    #[test]
    fn manifest_requires_parses() {
        let mut meta = serde_json::Map::new();
        meta.insert(
            "requires".to_string(),
            json!({
                "bins": ["git", "node"],
                "env": ["API_KEY"],
                "config": ["providers.openai"]
            }),
        );
        let req = resolve_manifest_requires(&meta).unwrap();
        assert_eq!(req.bins, vec!["git", "node"]);
        assert_eq!(req.env, vec!["API_KEY"]);
        assert_eq!(req.config, vec!["providers.openai"]);
    }
}
