---
summary: "通过 Llama.cpp（本地 OpenAI 兼容器）运行 OpenClaw"
read_when:
  - 你想通过 Llama.cpp 使用本地模型运行 OpenClaw
  - 你需要 llama.cpp 的安装和配置指导
title: "Llama.cpp"
---

# Llama.cpp

OpenClaw 支持通过 OpenAI 兼容 API (`/v1`) 集成 `llama.cpp`。这让你可以使用 CPU 或者 GPU 高效地运行量化的本地模型，并将其无缝接入 OpenClaw 的生态系统。

## 快速开始

1. 启动你的 `llama-server`:

```bash
# 例如，在 8080 端口启动 llama-server
./llama-server -m your-model.gguf --port 8080 --ctx-size 8192
```

1. 通过环境变量启用 Llama.cpp:

```bash
export LLAMA_CPP_HOST="http://localhost:8080/v1"
```

1. (可选) 配置你的模型名称:

```bash
export LLAMA_CPP_CHAT_MODEL="local-model"
export LLAMA_CPP_EMBED_MODEL="local-model"
```

## 配置

如果你更喜欢在 `openclaw.json` (或通过 CLI) 中显式地定义配置，你可以在 `providers` 下面这样配置它：

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

配置完成后，就可以在代理设置中使用这一模型：

```json5
{
  agents: {
    defaults: {
      model: { primary: "llama-cpp/local-model" }
    }
  }
}
```

## 故障排除

### 拒绝连接 (Connection refused)

确保 `llama-server` 正在运行并可以被访问。手动测试端点:

```bash
curl http://localhost:8080/v1/models
```

如果它运行在不同的端口，请更新 `LLAMA_CPP_HOST` 变量或更新你的配置文件。
