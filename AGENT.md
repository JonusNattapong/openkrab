# 🤖 Agent Development Guide

Complete guide for developing and extending AI agents in OpenKrab.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Agent Architecture](#agent-architecture)
- [Creating Agents](#creating-agents)
- [Built-in Tools](#built-in-tools)
- [Custom Tools](#custom-tools)
- [Agent Hooks](#agent-hooks)
- [Memory System](#memory-system)
- [Configuration](#configuration)
- [Best Practices](#best-practices)
- [Examples](#examples)

---

## Overview

OpenKrab uses a **multi-agent architecture** where different channels route to different AI personalities. Each agent has:

| Component | Description |
|-----------|-------------|
| **Identity** | Name, emoji, personality, system prompt |
| **Provider** | LLM backend (OpenAI, Anthropic, Gemini, Ollama) |
| **Tools** | Capabilities the agent can invoke |
| **Memory** | Long-term context storage (vector + text search) |
| **Hooks** | Event-driven customization points |

---

## Quick Start

### Basic Agent Usage

```rust
use openkrab::agents::{Agent, AgentConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Create agent with default config
    let agent = Agent::from_config(AgentConfig::default()).await?;
    
    // Chat with the agent
    let response = agent.chat("Hello!").await?;
    println!("{}", response);
    
    Ok(())
}
```

### Configuration File

```toml
# ~/.config/krabkrab/agents/research.toml
[agent]
name = "ResearchBot"
emoji = "🔬"
personality = "A meticulous research assistant"
system_prompt = "Always cite sources and verify facts."

[agent.provider]
type = "anthropic"
model = "claude-3-opus-20240229"

[agent.tools]
enabled = ["web_search", "web_fetch", "memory_store"]
```

Load the agent:
```rust
let agent = Agent::load("research").await?;
```

---

## Agent Architecture

```
┌─────────────────────────────────────────┐
│           Agent Runtime                 │
├─────────────────────────────────────────┤
│  Identity  │  Provider  │  Memory      │
│  ├ name    │  ├ type    │  ├ vector    │
│  ├ emoji   │  ├ model   │  ├ text FTS  │
│  ├ personality         │  ├ hybrid     │
│  └ system_prompt       │  └ temporal   │
├─────────────────────────────────────────┤
│           Tool Registry                 │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │  bash   │ │  read   │ │  write  │  │
│  └─────────┘ └─────────┘ └─────────┘  │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │web_search│ │web_fetch│ │memory_* │  │
│  └─────────┘ └─────────┘ └─────────┘  │
├─────────────────────────────────────────┤
│           Hook System                   │
│  BeforeAgent → BeforeTool → AfterTool   │
│       ↓           ↓           ↓         │
│  BeforeLLM → AfterLLM → BeforeReply     │
└─────────────────────────────────────────┘
```

---

## Creating Agents

### Programmatic Creation

```rust
use openkrab::agents::{Agent, AgentConfig, AgentIdentity};

let config = AgentConfig {
    identity: AgentIdentity {
        name: "CodeAssistant".to_string(),
        emoji: "💻".to_string(),
        personality: "An expert programmer focused on clean code.".to_string(),
        system_prompt: Some(
            "Use bash_pty for interactive debugging. Explain your reasoning.".to_string()
        ),
    },
    provider: ProviderConfig::Anthropic {
        model: "claude-3-opus-20240229".to_string(),
        api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
    },
    tools: ToolConfig {
        enabled: vec!["bash_pty", "read", "write", "web_search"],
        ..Default::default()
    },
    memory: MemoryConfig {
        enabled: true,
        provider: "openai".to_string(),
        model: "text-embedding-3-small".to_string(),
    },
};

let agent = Agent::from_config(config).await?;
```

### Agent Registry

```rust
use openkrab::agents::AgentRegistry;

let mut registry = AgentRegistry::new();

// Register multiple agents
registry.register("research", research_config).await?;
registry.register("code", code_config).await?;
registry.register("general", general_config).await?;

// Route by channel
match channel {
    "#research" => registry.get("research"),
    "#coding" => registry.get("code"),
    _ => registry.get("general"),
}
```

---

## Built-in Tools

### Core Tools

| Tool | Description | Safety |
|------|-------------|--------|
| `bash` | Execute shell commands | Sandboxed, allowlist required |
| `bash_pty` | Interactive PTY shell | Sandboxed, full terminal support |
| `read` | Read file contents | Path validation |
| `write` | Write file contents | Path validation |
| `fetch` | HTTP GET request | URL validation |
| `web_search` | Search the web | Safe |

### Memory Tools

| Tool | Description |
|------|-------------|
| `memory_search` | Search vector + text memory |
| `memory_store` | Store key-value in memory |
| `memory_recall` | Recall by key |
| `memory_forget` | Remove from memory |

### Usage Examples

```rust
// Execute command
let result = agent.use_tool("bash", json!({
    "command": "ls -la",
    "cwd": "/home/user"
})).await?;

// Read file
let content = agent.use_tool("read", json!({
    "path": "/home/user/document.txt"
})).await?;

// Search web
let results = agent.use_tool("web_search", json!({
    "query": "rust async programming"
})).await?;

// Store memory
agent.use_tool("memory_store", json!({
    "key": "user_preference",
    "value": "likes_detailed_explanations"
})).await?;
```

---

## Custom Tools

### Basic Tool Implementation

```rust
use openkrab::agents::tools::{Tool, ToolContext, ToolResult};
use async_trait::async_trait;

pub struct WeatherTool {
    api_key: String,
}

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "weather"
    }
    
    fn description(&self) -> &str {
        "Get current weather for a location"
    }
    
    fn schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name or coordinates"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "default": "celsius"
                }
            },
            "required": ["location"]
        })
    }
    
    async fn execute(&self, _ctx: &ToolContext, args: &str) -> ToolResult {
        let args: serde_json::Value = serde_json::from_str(args)?;
        let location = args["location"].as_str()
            .ok_or("Location required")?;
        
        // Call weather API
        let weather = self.fetch_weather(location).await?;
        
        Ok(format!("Weather in {}: {}", location, weather))
    }
}

// Register
agent.register_tool(Box::new(WeatherTool {
    api_key: std::env::var("WEATHER_API_KEY")?,
})).await?;
```

### Tool with State

```rust
use std::sync::{Arc, Mutex};

pub struct CounterTool {
    count: Arc<Mutex<u32>>,
}

#[async_trait]
impl Tool for CounterTool {
    fn name(&self) -> &str {
        "counter"
    }
    
    async fn execute(&self, _ctx: &ToolContext, args: &str) -> ToolResult {
        let mut count = self.count.lock().unwrap();
        
        if args == "increment" {
            *count += 1;
        } else if args == "decrement" {
            *count -= 1;
        }
        
        Ok(format!("Count: {}", *count))
    }
}
```

---

## Agent Hooks

### Hook Phases

```rust
pub enum HookPhase {
    BeforeAgentStart,    // Agent initialization
    AfterAgentStart,     // Agent ready
    BeforeToolCall,      // Before tool execution
    AfterToolCall,       // After tool execution
    BeforeLlmRequest,    // Before LLM API call
    AfterLlmResponse,    // After LLM response
    BeforeReply,         // Before sending to user
    AfterReply,          // After sending to user
    OnCompaction,        // Memory compaction
    OnSessionEnd,        // Session cleanup
}
```

### Creating Hooks

```rust
use openkrab::agents::hooks::{AgentHook, HookContext, HookPhase};

pub struct MetricsHook {
    metrics: Arc<Mutex<Metrics>>,
}

impl AgentHook for MetricsHook {
    fn phase(&self) -> HookPhase {
        HookPhase::AfterToolCall
    }
    
    fn execute(&self, ctx: &HookContext) -> Result<(), HookError> {
        let mut metrics = self.metrics.lock().unwrap();
        
        metrics.tool_calls += 1;
        metrics.total_duration_ms += ctx.duration_ms;
        
        if let Some(error) = &ctx.error {
            metrics.errors += 1;
            tracing::error!("Tool {} failed: {}", ctx.tool_name, error);
        }
        
        Ok(())
    }
}

// Register
agent.register_hook(Box::new(MetricsHook {
    metrics: Arc::new(Mutex::new(Metrics::default())),
})).await?;
```

### Async Hooks

```rust
use openkrab::agents::hooks::AsyncAgentHook;

#[async_trait]
impl AsyncAgentHook for DataEnrichmentHook {
    async fn execute(&self, ctx: &HookContext) -> Result<(), HookError> {
        // Fetch additional data
        let enrichment = self.fetch_data(&ctx.user_message).await?;
        
        // Add to context
        ctx.metadata.insert("enrichment", enrichment);
        
        Ok(())
    }
}
```

---

## Memory System

### Hybrid Memory Architecture

```
┌─────────────────────────────────────┐
│         Memory Manager              │
├─────────────────────────────────────┤
│  Vector Store    │  Text Store      │
│  ├ Embeddings    │  ├ Full-text     │
│  ├ Similarity    │  ├ BM25          │
│  └ ANN Search    │  └ Hybrid        │
├─────────────────────────────────────┤
│  MMR Reranking  │ Temporal Decay   │
│  Diverse results │ Older = lower    │
└─────────────────────────────────────┘
```

### Using Memory

```rust
// Store with metadata
agent.memory.store(
    "user_prefers_python",
    "User prefers Python over Rust for scripting",
    Some(json!({
        "category": "preference",
        "confidence": 0.95
    }))
).await?;

// Semantic search
let results = agent.memory.search(
    "What programming language does the user like?",
    SearchOptions {
        limit: 5,
        min_score: 0.7,
        use_mmr: true,
    }
).await?;

// Recall by key
let pref = agent.memory.recall("user_prefers_python").await?;

// Forget
agent.memory.forget("outdated_info").await?;
```

### Memory Configuration

```toml
[agent.memory]
enabled = true
provider = "openai"           # Embedding provider
model = "text-embedding-3-small"
dimensions = 1536

[agent.memory.hybrid]
vector_weight = 0.7           # Vector search weight
text_weight = 0.3             # Text search weight

[agent.memory.reranking]
enabled = true
mmr_lambda = 0.5              # Diversity vs relevance

[agent.memory.decay]
enabled = true
half_life_days = 30           # Memory fade rate
```

---

## Configuration

### Environment Variables

```bash
# Provider API Keys
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GEMINI_API_KEY="..."

# Agent Defaults
export KRABKRAB_AGENT_NAME="MyBot"
export KRABKRAB_AGENT_EMOJI="🤖"
export KRABKRAB_LOG_LEVEL="info"

# Memory
export KRABKRAB_MEMORY_PROVIDER="openai"
export KRABKRAB_MEMORY_MODEL="text-embedding-3-small"

# Safety
export KRABKRAB_SANDBOX_MODE="strict"
export KRABKRAB_BASH_ALLOWLIST="/usr/bin,/bin"
```

### Complete Agent Config

```toml
[agent]
name = "ResearchAssistant"
emoji = "🔬"
personality = "A thorough research assistant focused on accuracy"
system_prompt = """
You are a research assistant. Always:
1. Cite sources when providing information
2. Use web_search and web_fetch tools
3. Store findings in memory
4. Ask clarifying questions when needed
"""

[agent.provider]
type = "anthropic"
model = "claude-3-opus-20240229"
api_key = "${ANTHROPIC_API_KEY}"
temperature = 0.7
max_tokens = 4096

[agent.tools]
enabled = ["web_search", "web_fetch", "read", "memory_store", "memory_search"]

[agent.tools.bash]
enabled = false  # Disable for safety

[agent.tools.web_search]
max_results = 10
timeout_seconds = 30

[agent.memory]
enabled = true
provider = "openai"
model = "text-embedding-3-small"
max_results = 10

[agent.hooks]
enabled = ["logging", "metrics"]

[agent.security]
sandbox_mode = "strict"
allow_file_write = false
max_tool_calls = 50
```

---

## Best Practices

### 1. Safety First

```rust
// Always validate paths
fn validate_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path);
    
    // Prevent path traversal
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        bail!("Path traversal not allowed");
    }
    
    // Check allowlist
    let allowed = ["/home/user/data", "/tmp"];
    if !allowed.iter().any(|a| path.starts_with(a)) {
        bail!("Path not in allowlist");
    }
    
    Ok(path)
}
```

### 2. Error Handling

```rust
impl Tool for MyTool {
    async fn execute(&self, ctx: &ToolContext, args: &str) -> ToolResult {
        match self.process(args).await {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!(tool = self.name(), error = %e);
                
                Err(ToolError::ExecutionFailed {
                    tool: self.name().to_string(),
                    error: e.to_string(),
                    retryable: e.is_retryable(),
                })
            }
        }
    }
}
```

### 3. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_chat() {
        let agent = create_test_agent().await;
        let response = agent.chat("Hello").await.unwrap();
        assert!(!response.is_empty());
    }
    
    #[tokio::test]
    async fn test_tool_execution() {
        let tool = CalculatorTool;
        let ctx = ToolContext::default();
        
        let result = tool.execute(&ctx, "2 + 2").await.unwrap();
        assert_eq!(result, "4");
    }
    
    #[tokio::test]
    async fn test_memory_operations() {
        let agent = create_test_agent().await;
        
        agent.memory.store("key", "value", None).await.unwrap();
        let value = agent.memory.recall("key").await.unwrap();
        
        assert_eq!(value, "value");
    }
}
```

### 4. Performance

```rust
// Use connection pooling
lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();
}

// Cache expensive operations
use cached::proc_macro::cached;

#[cached(time = 300)]  // 5 minutes
async fn expensive_lookup(query: String) -> Result<String> {
    // API call or heavy computation
}
```

---

## Examples

### Research Agent

```rust
let research_agent = Agent::from_config(AgentConfig {
    identity: AgentIdentity {
        name: "ResearchBot".to_string(),
        emoji: "🔬".to_string(),
        personality: "A meticulous research assistant.".to_string(),
        system_prompt: Some(
            "Always cite sources. Use web_search and web_fetch.".to_string()
        ),
    },
    provider: ProviderConfig::Anthropic {
        model: "claude-3-opus-20240229".to_string(),
        ..Default::default()
    },
    tools: ToolConfig {
        enabled: vec!["web_search", "web_fetch", "memory_store"],
        ..Default::default()
    },
    ..Default::default()
}).await?;
```

### Code Assistant

```rust
let code_agent = Agent::from_config(AgentConfig {
    identity: AgentIdentity {
        name: "CodeBot".to_string(),
        emoji: "💻".to_string(),
        personality: "An expert programmer.".to_string(),
        system_prompt: Some(
            "Use bash_pty for debugging. Explain complex code.".to_string()
        ),
    },
    provider: ProviderConfig::OpenAI {
        model: "gpt-4".to_string(),
        ..Default::default()
    },
    tools: ToolConfig {
        enabled: vec!["bash_pty", "read", "write", "web_search"],
        bash: BashConfig {
            allowlist: vec!["/usr/bin", "/bin", "/usr/local/bin"],
            ..Default::default()
        },
        ..Default::default()
    },
    ..Default::default()
}).await?;
```

### Personal Assistant

```rust
let personal_agent = Agent::from_config(AgentConfig {
    identity: AgentIdentity {
        name: "Assistant".to_string(),
        emoji: "🤖".to_string(),
        personality: "A helpful and friendly assistant.".to_string(),
        system_prompt: Some(
            "Remember user preferences. Be concise but helpful.".to_string()
        ),
    },
    memory: MemoryConfig {
        enabled: true,
        ..Default::default()
    },
    ..Default::default()
}).await?;
```

---

## API Reference

For complete API documentation, see:
- [docs.rs/openkrab](https://docs.rs/openkrab)
- [Rust docs](https://docs.rs/openkrab/latest/openkrab/agents/index.html)

---

## Contributing

Contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md).

---

**🦀 Happy Agent Building!**
