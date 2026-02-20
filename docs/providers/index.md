---
summary: "Model providers (LLMs) supported by OpenKrab"
read_when:
  - You want to choose a model provider
  - You need a quick overview of supported LLM backends
title: "Model Providers"
---

# Model Providers

OpenKrab can use many LLM providers. Pick a provider, authenticate, then set the
default model.

Looking for chat channel docs (Telegram/Discord/Slack/WhatsApp/etc.)? See [Channels](/channels).

## Quick start

1. Authenticate with the provider via `krabkrab config`.
2. Set the default model:

```toml
[providers.openai]
model = "gpt-4o"
api_key = "sk-..."
```

## Supported Providers

| Provider | Status | Auth | Models |
|----------|--------|------|--------|
| **OpenAI** | ✅ | API Key | GPT-4, GPT-4o, GPT-3.5 |
| **Anthropic** | ✅ | API Key | Claude 3.5/3 Opus/Sonnet/Haiku |
| **Gemini** | ✅ | API Key / OAuth | Gemini 1.5 Pro/Flash |
| **Ollama** | ✅ | Local | Llama, Mistral, CodeLlama, etc. |
| **OpenRouter** | ✅ | API Key | Multiple models |
| **Moonshot** | ✅ | API Key | Kimi models |
| **MiniMax** | ✅ | OAuth | MiniMax models |
| **Qwen** | ✅ | API Key | Qwen models |

## Provider Configuration

### OpenAI

```toml
[providers.openai]
api_key = "sk-..."
model = "gpt-4o"
base_url = "https://api.openai.com/v1"  # optional
```

### Anthropic

```toml
[providers.anthropic]
api_key = "sk-ant-..."
model = "claude-3-5-sonnet-20241022"
```

### Gemini

```toml
[providers.gemini]
api_key = "..."
model = "gemini-1.5-pro"
```

### Ollama (Local)

```toml
[providers.ollama]
base_url = "http://localhost:11434"
model = "llama3.2"
```

## Environment Variables

You can also set provider credentials via environment variables:

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

## Model Selection

Set the default provider/model:

```toml
[agents.defaults]
provider = "openai"
model = "gpt-4o"
```

Or per-agent:

```toml
[[agents.list]]
id = "coding"
provider = "anthropic"
model = "claude-3-5-sonnet-20241022"
```

## Fallback Models

Configure fallback models if the primary fails:

```toml
[agents.defaults.fallbacks]
models = ["gpt-4o-mini", "claude-3-haiku"]
```

## Local Models

OpenKrab supports local models via:

- **Ollama** — Recommended for beginners
- **vLLM** — For production deployments
- **llama.cpp** — For edge devices

See [Local Models](/gateway/local-models) for setup details.
