# 🦞 OpenKrab — Personal AI Assistant

<p align="center">
  <strong>EXFOLIATE! EXFOLIATE!</strong>
</p>

<p align="center">
  <a href="https://github.com/JonusNattapong/openkrab/actions/workflows/rust.yml?branch=main"><img src="https://img.shields.io/github/actions/workflow/status/JonusNattapong/openkrab/rust.yml?branch=main&style=for-the-badge" alt="CI status"></a>
  <a href="https://github.com/JonusNattapong/openkrab/releases"><img src="https://img.shields.io/github/v/release/JonusNattapong/openkrab?include_prereleases&style=for-the-badge" alt="GitHub release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="MIT License"></a>
  <img src="https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge" alt="Rust">
</p>

**OpenKrab** is a _personal AI assistant_ you run on your own devices — rewritten in Rust.
It answers you on the channels you already use (Telegram, Slack, Discord, Signal, Matrix, BlueBubbles, Google Chat, IRC, Microsoft Teams, WebChat), with native Rust speed, lower memory footprint, and single-binary deployment.

This is a Rust port of [OpenClaw](https://github.com/openclaw/openclaw) (TypeScript/Node.js).

[Porting Status](PORTING.md) · [Quick Start](#quick-start-tldr) · [Architecture](#how-it-works-short) · [Channels](#channels) · [Providers](#providers)

## Why Rust?

- **Single binary** — compile once, run anywhere (no Node.js runtime needed)
- **Low memory footprint** — no GC pauses, efficient async with Tokio
- **Fast startup** — native binary, instant cold start
- **Type safety** — Rust's ownership model prevents entire classes of bugs
- **Zero-cost abstractions** — performance without sacrifice

## Install (recommended)

```bash
# From source
git clone https://github.com/JonusNattapong/openkrab.git
cd openkrab

cargo build --release

# Binary at: target/release/krabkrab
./target/release/krabkrab --help
```

Pre-built binaries: [Releases](https://github.com/JonusNattapong/openkrab/releases)

## Quick start (TL;DR)

```bash
# Start the gateway
krabkrab gateway --port 18789

# Send a message (Telegram)
krabkrab telegram --to @username --text "Hello from OpenKrab"

# Send a message (Discord)
krabkrab discord --to 123456789 --text "Hello from OpenKrab"

# Talk to the assistant
krabkrab ask "What's on my calendar today?"

# Check status
krabkrab status

# Configure interactively
krabkrab configure

# Memory operations
krabkrab memory sync --path ./docs
krabkrab memory search "query text"
```

## Development channels

- **stable**: tagged releases (`vYYYY.M.D`), GitHub Releases
- **beta**: prerelease tags (`vYYYY.M.D-beta.N`)
- **dev**: moving head of `main`

## From source (development)

```bash
git clone https://github.com/JonusNattapong/openkrab.git
cd openkrab

cargo build
cargo test

# Run CLI
cargo run -- --help
```

## How it works (short)

```
Telegram / Slack / Discord / Signal / Matrix / BlueBubbles / Google Chat / IRC / MSTeams / WebChat
               │
               ▼
┌───────────────────────────────┐
│            Gateway            │
│     (Tokio async runtime)     │
│     127.0.0.1:18789           │
└──────────────┬────────────────┘
               │
               ├─ LLM Providers (OpenAI, Gemini, Ollama, Copilot, MiniMax, Qwen)
               ├─ CLI (krabkrab …)
               ├─ Memory (SQLite + vector embeddings)
               └─ Tools (shell, media, web)
```

## Channels

| Channel | Status | Notes |
|---------|--------|-------|
| [Telegram](src/connectors/telegram.rs) | ✅ | Bot API HTTP polling |
| [Slack](src/connectors/slack.rs) | ✅ | Bolt-style events |
| [Discord](src/connectors/discord.rs) | ✅ | Serenity gateway + HTTP API |
| [Google Chat](src/connectors/googlechat.rs) | ✅ | Chat API |
| [IRC](src/connectors/irc.rs) | ✅ | Basic IRC protocol |
| [Matrix](src/matrix/mod.rs) | ✅ | Matrix.org |
| [Signal](src/signal/mod.rs) | ✅ | signal-cli integration |
| [Microsoft Teams](src/connectors/msteams.rs) | ✅ | Bot Framework |
| [BlueBubbles](src/connectors/bluebubbles.rs) | ✅ | iMessage via BlueBubbles |
| [Mattermost](src/connectors/mattermost.rs) | ✅ | Webhook-based |
| [Twitch](src/connectors/twitch.rs) | ✅ | IRC + API |
| [Zalo](src/connectors/zalo.rs) | ✅ | Zalo API |
| [Feishu](src/connectors/feishu.rs) | ✅ | Lark/Feishu |
| [Nextcloud Talk](src/connectors/nextcloud_talk.rs) | ✅ | API |
| [Nostr](src/connectors/nostr.rs) | ✅ | Nostr protocol |
| [Tlon](src/connectors/tlon.rs) | ✅ | Urbit |
| WhatsApp | ⚠️ | Requires vendor SDK — see ts-layer/ |
| LINE | ⚠️ | Requires vendor SDK — see ts-layer/ |

## Providers

| Provider | Status | Auth Method |
|----------|--------|-------------|
| OpenAI | ✅ | API Key |
| Anthropic (Claude) | ✅ | API Key |
| Gemini | ✅ | API Key / OAuth (CLI credentials) |
| Ollama | ✅ | Local server |
| GitHub Copilot | ✅ | OAuth token chain |
| MiniMax | ✅ | Device-code OAuth |
| Qwen | ✅ | Portal OAuth |

## Highlights

- **[Gateway control plane](src/gateway.rs)** — Tokio-based async runtime with sessions, channels, and events
- **[Multi-channel inbox](src/channels/)** — unified messaging across all platforms
- **[Multi-agent routing](src/routing/)** — route channels/accounts to isolated agents
- **[Memory + vector search](src/memory/)** — SQLite-backed storage with embeddings
- **[TUI](src/tui/mod.rs)** — Terminal UI with ratatui
- **[Security](src/security.rs)** — DM pairing, allowlists, PKCE OAuth

## Configuration

OpenKrab uses TOML configuration at `~/.config/krabkrab/krabkrab.toml`:

```toml
[agent]
model = "anthropic/claude-opus-4-6"

[providers.openai]
api_key = "sk-..."
model = "gpt-4"

[channels.telegram]
enabled = true
bot_token = "..."

[channels.discord]
enabled = true
token = "..."

[channels.bluebubbles]
enabled = true
server_url = "http://..."
password = "..."
```

## Security model (important)

- **Default:** tools run on the host for the **main** session
- **DM pairing** — unknown senders receive a pairing code
- **Allowlists** — control who can interact via `allowFrom`
- **Sandbox mode** — run non-main sessions in Docker (opt-in)

Details: [Security](src/security.rs)

## Development

### Build & Test

```bash
cargo build              # Debug build
cargo build --release    # Optimized build
cargo test               # Run all 410+ tests
cargo test --lib         # Lib tests only
cargo clippy             # Lint
cargo fmt                # Format
```

### Project Structure

```
src/
├── acp/              ← ACP protocol types & routing
├── agents/           ← Agent runner loop
├── auto_reply/       ← Keyword auto-reply engine
├── broadcast/        ← Fan-out message broadcast
├── channels/         ← Channel registry & config
├── commands/         ← CLI sub-commands
├── compat/           ← Legacy API compatibility shims
├── connectors/       ← Platform connectors
├── cron/             ← Cron/scheduled task engine
├── daemon.rs         ← Background service manager
├── gateway.rs        ← Gateway routing logic
├── memory/           ← Conversation memory
├── oauth/            ← OAuth 2.0 PKCE helper
├── providers/        ← LLM providers
├── routing/          ← Message routing rules
├── sessions/         ← Conversation sessions
├── signal/           ← Signal connector
├── slack/            ← Slack blocks & threading
├── tools/            ← Tool integrations
├── tui/              ← Terminal UI
├── voice/            ← Voice wake/talk mode
└── web_connector/    ← Web/HTTP gateway
```

## Porting Status

OpenKrab is a port of [OpenClaw](https://github.com/openclaw/openclaw) from TypeScript to Rust.

**Status: Phase 20 complete ✅**

| Phase | Module(s) | Status |
|-------|-----------|--------|
| 1-4 | Core (common, config, channels) | ✅ |
| 5-6 | Connectors + Commands | ✅ |
| 7-8 | Providers + Gateway | ✅ |
| 9-10 | Memory + Media | ✅ |
| 11-12 | Agents + Infrastructure | ✅ |
| 13-14 | Signal/Matrix + OAuth | ✅ |
| 15-16 | Provider auth wiring | ✅ |
| 17-18 | Discord + Security hardening | ✅ |
| 19-20 | BlueBubbles + Release | ✅ |

**Total tests: 410 unit + integration, 0 failures**

See [PORTING.md](PORTING.md) for detailed progress and module map.

## What's NOT Ported (Intentional)

| Area | Reason |
|------|--------|
| `apps/ios`, `apps/macos`, `apps/android` | Swift/Kotlin — platform-native, out of scope |
| `assets/chrome-extension/` | Browser extension JS |
| `src/browser/`, `src/canvas-host/` | Browser automation — kept in ts-layer/ |
| Docker / fly.toml / render.yaml | Infrastructure config |
| WhatsApp, LINE connectors | Require vendor SDKs — see ts-layer/ |

## TypeScript Interop (ts-layer/)

For features requiring JavaScript/TypeScript runtime:

```bash
cd ts-layer
npm install
npm run bridge
```

## Migration from OpenClaw

1. **Config format**: JSON → TOML
2. **Config location**: `~/.clawdbot/` → `~/.config/krabkrab/`
3. **Most connectors**: Compatible with same tokens/webhooks

See [PORTING.md](PORTING.md) for detailed migration guide.

## Docs

- [PORTING.md](PORTING.md) — Porting status and module map
- [AGENT.md](AGENT.md) — Agent development guide
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=JonusNattapong/openkrab&type=date)](https://www.star-history.com/#JonusNattapong/openkrab&type=date)

## Community

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Molty

OpenKrab was built for **Molty**, a space lobster AI assistant. 🦞

This is a Rust port of [OpenClaw](https://github.com/openclaw/openclaw), originally by Peter Steinberger and the community.

- [openclaw.ai](https://openclaw.ai)
- [@openclaw](https://x.com/openclaw)

## License

MIT License — see [LICENSE](LICENSE)
