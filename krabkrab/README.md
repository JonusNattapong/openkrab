# 🦀 krabkrab (Rust) — Personal AI Assistant

<p align="center">
    <strong>Rust port of the krabkrab personal AI assistant</strong>
</p>

<p align="center">
  <a href="https://github.com/openkrab/krabkrab/actions"><img src="https://img.shields.io/github/actions/workflow/status/openkrab/krabkrab/ci.yml?branch=main&style=for-the-badge" alt="CI status"></a>
  <a href="https://crates.io/crates/krabkrab"><img src="https://img.shields.io/crates/v/krabkrab?style=for-the-badge" alt="Crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="MIT License"></a>
</p>

**krabkrab** is a _personal AI assistant_ written in Rust. It answers you on the channels you already use (Telegram, Slack, Discord, Signal, Matrix, BlueBubbles, and more), with improved performance and lower memory footprint compared to the TypeScript version.

This is a Rust port of the original [krabkrab](https://github.com/krabkrab/krabkrab) TypeScript/Node.js project.

## Why Rust?

- **Performance**: Faster startup, lower memory usage, and better resource efficiency
- **Safety**: Memory safety guarantees without garbage collection
- **Single binary**: No Node.js runtime required, easier deployment
- **Cross-platform**: Compile for Linux, macOS, Windows, ARM, and more

## Installation

### From crates.io

```bash
cargo install krabkrab
```

### From source

```bash
git clone https://github.com/openkrab/krabkrab.git
cd krabkrab
cargo build --release

# Binary at: target/release/krabkrab
```

### Requirements

- Rust 1.75+ (for building from source)
- No runtime dependencies

## Quick Start

```bash
# Start the gateway
krabkrab gateway start

# Check status
krabkrab status

# Configure interactively
krabkrab configure

# Send a message (Telegram)
krabkrab telegram --text "Hello from krabkrab"

# Send a message (Slack)
krabkrab slack --text "Hello from krabkrab"

# Send a message (Discord)
krabkrab discord --to 123456789 --text "Hello from krabkrab"

# Memory operations
krabkrab memory sync --path ./docs
krabkrab memory search "query text"

# Ask LLM directly
krabkrab ask "What is Rust?"
```

## Supported Channels

| Channel | Status | Notes |
|---------|--------|-------|
| Telegram | ✅ Full | Bot API polling |
| Slack | ✅ Full | Webhook + Socket Mode |
| Discord | ✅ Full | Gateway WebSocket + HTTP API |
| Signal | ✅ Full | signal-cli REST API |
| Matrix | ✅ Full | Client API |
| BlueBubbles | ✅ Full | iMessage bridge |
| IRC | ✅ Full | Basic IRC protocol |
| MSTeams | ✅ Full | Bot Framework |
| Mattermost | ✅ Full | Webhook |
| Twitch | ✅ Full | IRC-based |
| Zalo | ✅ Full | Webhook |
| GoogleChat | ✅ Full | Webhook |
| Feishu | ✅ Full | Webhook |
| Nextcloud Talk | ✅ Full | API |
| Nostr | ✅ Full | Relay-based |
| Tlon | ✅ Full | Urbit |
| WhatsApp | ⚠️ Basic | Use JS layer for full support |
| LINE | ⚠️ Basic | Use JS layer for full support |
| iMessage | ✅ Via BlueBubbles | Recommended path |

## Supported Providers

| Provider | Status | Auth Methods |
|----------|--------|--------------|
| OpenAI | ✅ Full | API Key |
| Anthropic (Claude) | ✅ Full | API Key |
| Google Gemini | ✅ Full | API Key, CLI credentials |
| Ollama | ✅ Full | Local server |
| MiniMax | ✅ Full | OAuth (device code) |
| Qwen | ✅ Full | OAuth |
| GitHub Copilot | ✅ Full | OAuth token chain |

## Configuration

Configuration is stored in TOML format at `~/.config/krabkrab/krabkrab.toml`:

```toml
[agent]
model = "anthropic/claude-opus-4"

[providers.openai]
api_key = "sk-..."
model = "gpt-4"

[providers.gemini]
api_key = "..."
model = "gemini-pro"

[providers.ollama]
base_url = "http://localhost:11434"
model = "llama2"

[channels.telegram]
enabled = true
bot_token = "..."

[channels.slack]
enabled = true
bot_token = "..."
app_token = "..."

[channels.discord]
enabled = true
bot_token = "..."

[channels.bluebubbles]
enabled = true
server_url = "http://..."
password = "..."
```

## Gateway Architecture

```
┌─────────────────────────────────────────────────────┐
│               Messaging Channels                      │
│  Telegram │ Slack │ Discord │ Signal │ Matrix │ ... │
└───────────────────────┬─────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────┐
│                   Gateway                            │
│              (WebSocket Control Plane)               │
│            ws://127.0.0.1:3000                       │
└───────────────────────┬─────────────────────────────┘
                        │
          ┌─────────────┼─────────────┐
          │             │             │
          ▼             ▼             ▼
     ┌─────────┐  ┌──────────┐  ┌──────────┐
     │  Agent  │  │   CLI    │  │  Web UI  │
     │  (LLM)  │  │ (krabkrab)│  │(dashboard)│
     └─────────┘  └──────────┘  └──────────┘
```

## Key Features

### Core Platform
- **Gateway WebSocket control plane** — single control point for sessions, channels, and events
- **Multi-channel inbox** — unified messaging across all platforms
- **Multi-agent routing** — route channels/accounts to isolated agents
- **Session management** — context, memory, and transcript handling

### Memory & Context
- **Hybrid search** — vector + recency-based memory retrieval
- **Document sync** — sync and search local documents
- **Session transcripts** — persistent conversation history

### Discord-Specific (Phase 17)
- Gateway lifecycle with reconnect/backoff
- Inbound/outbound message handling
- Polls, reactions, threads, embeds
- Guild actions (channels, members, roles)
- Moderation (timeout, kick, ban)

### Security
- **DM pairing** — unknown senders require approval
- **PKCE OAuth** — secure authentication flows
- **Webhook signature verification** — for LINE, Slack, etc.

## Development

### Build & Test

```bash
# Build
cargo build

# Run all tests
cargo test --workspace

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test discord:: --lib
```

### Test Coverage

- **410+ tests** — unit + integration
- **0 failures** — all tests passing
- Comprehensive coverage of all modules

## What's NOT Ported (Intentional)

| Area | Reason | Alternative |
|------|--------|-------------|
| `apps/ios`, `apps/macos`, `apps/android` | Platform-native (Swift/Kotlin) | Use original JS project |
| `browser/`, `canvas-host/` | Browser automation | Use JS layer |
| `macos/` | macOS-specific APIs | Use JS layer |
| `tui/` | Replaced | Use `terminal` module |
| Full WhatsApp/LINE | Vendor SDKs | Use JS layer |

## Migration from TypeScript

If migrating from the original `openclaw` TypeScript version:

1. **Config format**: JSON → TOML
2. **Config location**: `~/.clawdbot/` → `~/.config/krabkrab/`
3. **CLI commands**: Minor changes (see `MIGRATION_NOTES.md`)
4. **Most connectors**: Compatible with same tokens/webhooks

See [MIGRATION_NOTES.md](MIGRATION_NOTES.md) for detailed migration guide.

## Documentation

- [PORTING.md](PORTING.md) — Porting status and module map
- [RELEASE_READINESS.md](RELEASE_READINESS.md) — Release checklist
- [MIGRATION_NOTES.md](MIGRATION_NOTES.md) — Migration from TypeScript
- [AGENT.md](AGENT.md) — Agent development guide

## Related Projects

- [krabkrab (TypeScript)](https://github.com/krabkrab/krabkrab) — Original implementation
- [openclaw](https://github.com/openkrab/openclaw) — Source for this port

## License

MIT License — see [LICENSE](LICENSE)

## Contributing

Contributions welcome! This is an active port project.

1. Check [PORTING.md](PORTING.md) for current status
2. Look for modules marked as incomplete
3. Follow Rust best practices
4. Add tests for new functionality
5. Submit a PR

---

**krabkrab** — EXFOLIATE! EXFOLIATE! 🦀

Built with ❤️ in Rust, ported from the original TypeScript project.
