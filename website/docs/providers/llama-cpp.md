---
summary: "Run OpenClaw with Llama.cpp (OpenAI-compatible local LLM)"
read_when:
  - You want to run OpenClaw with local models via llama.cpp
  - You need llama.cpp setup and configuration guidance
title: "Llama.cpp"
---

# Llama.cpp

OpenClaw integrates with `llama.cpp` using its OpenAI-compatible API endpoint `/v1`. This allows you to run quantized local models efficiently on CPU or GPU and connect them seamlessly to OpenClaw.

## Quick start

1. Start your `llama-server`:

```bash
# Example starting llama-server on port 8080
./llama-server -m your-model.gguf --port 8080 --ctx-size 8192
```

1. Enable Llama.cpp for OpenClaw via environment variables:

```bash
export LLAMA_CPP_HOST="http://localhost:8080/v1"
```

1. Optionally configure the model names:

```bash
export LLAMA_CPP_CHAT_MODEL="local-model"
export LLAMA_CPP_EMBED_MODEL="local-model"
```

## Configuration

If you prefer to define the configuration explicitly in your `openclaw.json` (or via the CLI), you can configure it under the `providers` section:

```json5
{
  models: {
    providers: {
      "llama-cpp": {
        baseUrl: "http://localhost:8080/v1",
        models: [
          {
            id: "local-model",
            name: "Llama.cpp Local Model",
            input: ["text"],
            cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 },
            contextWindow: 8192,
            maxTokens: 81920
          }
        ]
      }
    }
  }
}
```

Once configured, you can use the model in your agent settings:

```json5
{
  agents: {
    defaults: {
      model: { primary: "llama-cpp/local-model" }
    }
  }
}
```

## Model Costs

Llama.cpp is free and runs locally, so all model costs are set to $0.

## Troubleshooting

### Connection refused

Ensure that your `llama-server` is running and accessible. Test the endpoint manually:

```bash
curl http://localhost:8080/v1/models
```

If it is running on a different port, update the `LLAMA_CPP_HOST` variable or your configuration file.
