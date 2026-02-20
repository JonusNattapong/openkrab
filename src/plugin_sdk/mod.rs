//! plugin_sdk — Developer-facing Plugin SDK types and helpers.
//! Ported from `openkrab/src/plugin-sdk/` (Phase 10).
//!
//! This module exposes the stable API surface that external krabkrab plugins
//! use to integrate with the core runtime: hooks, tools, providers, routes.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Native plugin ABI symbol names.
pub const ABI_MANIFEST_SYMBOL: &[u8] = b"krabkrab_plugin_manifest_json\0";
pub const ABI_DECLARATION_SYMBOL: &[u8] = b"krabkrab_plugin_declaration_json\0";

// ─── Plugin context ───────────────────────────────────────────────────────────

/// Context passed to every plugin callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    /// The active agent session ID.
    pub session_id: String,
    /// Connector that originated the event (e.g. "telegram", "slack").
    pub connector: String,
    /// Arbitrary metadata the plugin can read and write.
    pub metadata: HashMap<String, serde_json::Value>,
}

impl PluginContext {
    pub fn new(session_id: impl Into<String>, connector: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            connector: connector.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn set_meta(&mut self, key: impl Into<String>, val: impl Serialize) {
        if let Ok(v) = serde_json::to_value(val) {
            self.metadata.insert(key.into(), v);
        }
    }

    pub fn get_meta<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.metadata
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

// ─── Plugin tool ──────────────────────────────────────────────────────────────

/// A tool that a plugin registers so the agent can call it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginTool {
    /// Unique tool name (snake_case).
    pub name: String,
    /// Human-readable description shown to the LLM.
    pub description: String,
    /// JSON Schema for the tool's input parameters.
    pub parameters: serde_json::Value,
}

impl PluginTool {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }

    pub fn with_param(
        mut self,
        name: &str,
        type_: &str,
        description: &str,
        required: bool,
    ) -> Self {
        let props = self.parameters["properties"].as_object_mut().unwrap();
        props.insert(
            name.to_string(),
            serde_json::json!({
                "type": type_,
                "description": description
            }),
        );
        if required {
            self.parameters["required"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!(name));
        }
        self
    }
}

/// Trait that plugin tools must implement.
#[async_trait]
pub trait PluginToolHandler: Send + Sync {
    fn tool(&self) -> &PluginTool;

    async fn call(&self, args: serde_json::Value, ctx: &PluginContext)
        -> Result<serde_json::Value>;
}

// ─── Plugin route ─────────────────────────────────────────────────────────────

/// An HTTP route registered by a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRoute {
    /// HTTP method ("GET", "POST", etc.)
    pub method: String,
    /// Path pattern, e.g. "/plugins/my-plugin/webhook".
    pub path: String,
    /// Optional description.
    pub description: Option<String>,
}

impl PluginRoute {
    pub fn post(path: impl Into<String>) -> Self {
        Self {
            method: "POST".into(),
            path: path.into(),
            description: None,
        }
    }
    pub fn get(path: impl Into<String>) -> Self {
        Self {
            method: "GET".into(),
            path: path.into(),
            description: None,
        }
    }
}

// ─── Plugin service ───────────────────────────────────────────────────────────

/// A long-running background service provided by a plugin.
#[async_trait]
pub trait PluginService: Send + Sync {
    fn name(&self) -> &str;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}

// ─── Plugin declaration ───────────────────────────────────────────────────────

/// Full declaration of a plugin's capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginDeclaration {
    pub name: String,
    pub version: String,
    pub tools: Vec<PluginTool>,
    pub routes: Vec<PluginRoute>,
    pub provides_providers: Vec<String>,
    pub hook_phases: Vec<String>,
}

impl PluginDeclaration {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    pub fn with_tool(mut self, tool: PluginTool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn with_route(mut self, route: PluginRoute) -> Self {
        self.routes.push(route);
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provides_providers.push(provider.into());
        self
    }
}

// ─── SDK helpers ──────────────────────────────────────────────────────────────

/// Build a standard tool-call result JSON.
pub fn tool_result(output: impl Into<String>) -> serde_json::Value {
    serde_json::json!({ "output": output.into() })
}

/// Build a standard error result JSON.
pub fn tool_error(err: impl Into<String>) -> serde_json::Value {
    serde_json::json!({ "error": err.into() })
}

/// Read a required string argument from tool call args.
pub fn require_string_arg<'a>(args: &'a serde_json::Value, key: &str) -> Result<&'a str> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing required argument `{}`", key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_context_meta() {
        let mut ctx = PluginContext::new("sess-1", "telegram");
        ctx.set_meta("count", 42u32);
        let v: Option<u32> = ctx.get_meta("count");
        assert_eq!(v, Some(42));
    }

    #[test]
    fn plugin_tool_builder() {
        let t = PluginTool::new("search", "Search the web").with_param(
            "query",
            "string",
            "The search query",
            true,
        );
        assert_eq!(t.name, "search");
        assert!(t.parameters["properties"]["query"]["type"].as_str() == Some("string"));
        assert!(t.parameters["required"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("query")));
    }

    #[test]
    fn plugin_route_constructors() {
        let r = PluginRoute::post("/plugins/test/webhook");
        assert_eq!(r.method, "POST");
        let g = PluginRoute::get("/plugins/test/status");
        assert_eq!(g.method, "GET");
    }

    #[test]
    fn plugin_declaration_builder() {
        let decl = PluginDeclaration::new("my-plugin", "1.0.0")
            .with_tool(PluginTool::new("search", "Search"))
            .with_route(PluginRoute::post("/webhook"))
            .with_provider("openai");
        assert_eq!(decl.tools.len(), 1);
        assert_eq!(decl.routes.len(), 1);
        assert_eq!(decl.provides_providers.len(), 1);
    }

    #[test]
    fn tool_result_json() {
        let r = tool_result("done");
        assert_eq!(r["output"].as_str(), Some("done"));
    }

    #[test]
    fn require_string_arg_ok_and_err() {
        let args = serde_json::json!({ "q": "hello" });
        assert_eq!(require_string_arg(&args, "q").unwrap(), "hello");
        assert!(require_string_arg(&args, "missing").is_err());
    }
}
