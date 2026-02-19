//! Example plugin: hello-plugin
//!
//! This is an example of a statically-linked plugin that demonstrates
//! the plugin system capabilities.

use krabkrab::plugin_sdk::{PluginContext, PluginDeclaration, PluginTool};
use krabkrab::plugins::{PluginKind, PluginManifest};

/// Get the plugin manifest.
pub fn manifest() -> PluginManifest {
    PluginManifest {
        name: "hello-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A simple example plugin that provides a greeting tool".to_string(),
        author: Some("krabkrab Team".to_string()),
        enabled: true,
        kind: PluginKind::Extension,
        requires: vec!["tools".to_string()],
        entry: None,
    }
}

/// Get the plugin declaration with tools and routes.
pub fn declaration() -> PluginDeclaration {
    PluginDeclaration::new("hello-plugin", "1.0.0")
        .with_tool(
            PluginTool::new("greet", "Generate a greeting message")
                .with_param("name", "string", "Name of the person to greet", true)
                .with_param(
                    "language",
                    "string",
                    "Language for the greeting (en, es, fr)",
                    false,
                ),
        )
        .with_tool(
            PluginTool::new("farewell", "Generate a farewell message").with_param(
                "name",
                "string",
                "Name of the person to bid farewell",
                true,
            ),
        )
}

/// Handle the greet tool call.
pub fn handle_greet(args: &serde_json::Value, _ctx: &PluginContext) -> serde_json::Value {
    let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("World");
    let language = args
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("en");

    let greeting = match language {
        "es" => format!("¡Hola, {}!", name),
        "fr" => format!("Bonjour, {}!", name),
        "de" => format!("Hallo, {}!", name),
        _ => format!("Hello, {}!", name),
    };

    serde_json::json!({
        "greeting": greeting,
        "language": language,
        "name": name
    })
}

/// Handle the farewell tool call.
pub fn handle_farewell(args: &serde_json::Value, _ctx: &PluginContext) -> serde_json::Value {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Friend");

    serde_json::json!({
        "farewell": format!("Goodbye, {}! Have a great day!", name),
        "name": name
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest() {
        let m = manifest();
        assert_eq!(m.name, "hello-plugin");
        assert_eq!(m.version, "1.0.0");
        assert!(m.enabled);
    }

    #[test]
    fn test_handle_greet_english() {
        let ctx = PluginContext::new("test-session", "test");
        let args = serde_json::json!({"name": "Alice", "language": "en"});
        let result = handle_greet(&args, &ctx);

        assert_eq!(result["greeting"], "Hello, Alice!");
        assert_eq!(result["language"], "en");
    }

    #[test]
    fn test_handle_greet_spanish() {
        let ctx = PluginContext::new("test-session", "test");
        let args = serde_json::json!({"name": "Carlos", "language": "es"});
        let result = handle_greet(&args, &ctx);

        assert_eq!(result["greeting"], "¡Hola, Carlos!");
    }

    #[test]
    fn test_handle_greet_german() {
        let ctx = PluginContext::new("test-session", "test");
        let args = serde_json::json!({"name": "Hans", "language": "de"});
        let result = handle_greet(&args, &ctx);

        assert_eq!(result["greeting"], "Hallo, Hans!");
    }

    #[test]
    fn test_handle_farewell() {
        let ctx = PluginContext::new("test-session", "test");
        let args = serde_json::json!({"name": "Bob"});
        let result = handle_farewell(&args, &ctx);

        assert_eq!(result["farewell"], "Goodbye, Bob! Have a great day!");
    }
}
