# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2026.2.20] - 2026-02-20

### Added

#### Browser Automation (Complete)
- **CDP (Chrome DevTools Protocol)** — Full browser automation support
- **Connection pooling** — Efficient session management with `pool.rs` (857 lines)
- **Network interception** — Monitor and modify HTTP requests/responses
- **Multi-tab support** — Handle multiple browser tabs simultaneously
- **Screenshots & snapshots** — Visual testing and debugging capabilities
- **DOM manipulation** — Click, type, navigate, evaluate JavaScript
- **Profile management** — Multiple browser profiles with JSON persistence

#### Canvas/A2UI (Complete)
- **A2UI Protocol** — Agent-to-UI rendering system (452 lines)
- **Surface management** — Multiple canvas surfaces with state isolation
- **Component system** — Reusable UI components with props
- **Theme support** — Customizable colors and typography
- **Data model** — Reactive data binding for UI updates

#### Hooks System (Complete)
- **Event system** — Comprehensive hook registry (177 lines)
- **Lifecycle events** — `message:inbound`, `agent:start`, `session:created`, etc.
- **Plugin integration** — WASM plugins can register hooks
- **Payload system** — Type-safe event payloads with JSON serialization

### Changed

#### Documentation
- Updated all documentation from OpenClaw to OpenKrab
- Added comprehensive [Migration Guide](docs/install/migrating.md)
- Updated feature status from "Partial" to "Complete" for Browser, Canvas, Hooks
- Added detailed module statistics and line counts

### Fixed

- Corrected feature status in documentation (Browser, Canvas, Hooks are complete, not partial)
- Updated directory references from `~/.clawdbot/` to `~/.config/krabkrab/`

---

## [2026.2.19] - 2026-02-19

### Added

#### Core Platform
- Gateway WebSocket control plane for sessions, channels, and events
- Multi-channel inbox with unified message handling
- Multi-agent routing for channel/account isolation
- Session management with context, memory, and transcripts

#### Connectors (18 platforms)
- **Telegram** - Bot API polling with full inbound/outbound support
- **Slack** - Webhook + Socket Mode with block kit support
- **Discord** - Gateway WebSocket + HTTP API with comprehensive features
  - Polls, reactions, threads, embeds
  - Guild actions (channels, members, roles)
  - Moderation (timeout, kick, ban)
  - Action gates for permission control
- **Signal** - signal-cli REST API integration
- **Matrix** - Client API with room handling
- **BlueBubbles** - iMessage bridge (recommended for iMessage)
- **IRC** - Basic IRC protocol support
- **MSTeams** - Bot Framework integration
- **Mattermost** - Webhook support
- **Twitch** - IRC-based chat
- **Zalo** - Webhook integration
- **GoogleChat** - Webhook support
- **Feishu** - Webhook integration
- **Nextcloud Talk** - API integration
- **Nostr** - Relay-based messaging
- **Tlon** - Urbit integration
- **WhatsApp** - Cloud API and Business API
- **LINE** - LINE API with signature verification

#### Providers (7+ LLM providers)
- **OpenAI** - GPT models with API key auth
- **Anthropic** - Claude models with API key auth
- **Google Gemini** - API key + CLI credentials extraction
- **Ollama** - Local server integration
- **MiniMax** - OAuth device code flow
- **Qwen** - OAuth integration
- **GitHub Copilot** - OAuth token chain with caching

#### Memory System
- Hybrid search (vector + recency-based)
- Document sync and indexing
- Session transcripts with pruning
- SQLite-based persistence
- MMR reranking for diverse results
- Temporal decay for older memories

#### Voice System
- Voice wake mode — "Hey KrabKrab" activation
- Talk mode — Continuous conversation
- VAD (Voice Activity Detection)
- Spectral analysis (FFT)
- Beep generation for audio feedback
- Microphone capture (cross-platform)
- Text-to-speech (ElevenLabs, Windows SAPI, macOS say, Linux espeak)

#### Plugin System
- WASM runtime via Wasmtime
- Hot reload for development
- Sandboxing with 4 security levels
- Native library loading
- Hook system for event-driven architecture

#### Security
- DM pairing for unknown sender approval
- PKCE OAuth 2.0 with SHA-256 challenge
- Webhook signature verification (LINE, Slack)
- Allowlist/denylist management
- Rate limiting (per-user and global)
- Audit logging

#### CLI Commands
- `gateway start` - Start the gateway server
- `status` - Check system status
- `doctor` - Health checks and diagnostics
- `configure` - Interactive configuration
- `config get/set/unset` - Configuration management
- `telegram` - Send Telegram messages
- `slack` - Send Slack messages
- `discord` - Send Discord messages
- `memory sync` - Sync documents to memory
- `memory search` - Search memory
- `ask` - Direct LLM query
- `models` - List available models
- `voice` - Voice system controls
- `plugin` - Plugin management
- `browser` - Browser automation

#### Infrastructure
- Retry policy with exponential backoff
- Rate limiting
- Circuit breaker pattern
- Process/subprocess management
- Cron job scheduling
- Web dashboard (self-contained HTML/JS)

### Technical Details

- **Language**: Rust 1.75+
- **Architecture**: Gateway WebSocket control plane + multi-channel routing
- **Config Format**: TOML (changed from JSON)
- **Config Location**: `~/.config/krabkrab/` (changed from `~/.clawdbot/`)
- **Test Coverage**: 410+ tests, 0 failures
- **Total Lines**: ~56,276 lines of Rust

### Porting Status

All 24 phases from the TypeScript version have been ported:

| Phase | Module(s) | Lines | Status |
|-------|-----------|-------|--------|
| 1-4 | Core, config, channels, connectors | ~10,000 | ✅ Complete |
| 5-6 | Agents + Tools | ~8,000 | ✅ Complete |
| 7-8 | Gateway + Providers | ~12,000 | ✅ Complete |
| 9-10 | Memory + Media | ~10,000 | ✅ Complete |
| 11-12 | Infrastructure + Commands | ~6,000 | ✅ Complete |
| 13-14 | Signal/Matrix + OAuth | ~5,000 | ✅ Complete |
| 15-16 | Provider auth wiring | ~3,000 | ✅ Complete |
| 17-18 | Discord + Security hardening | ~5,000 | ✅ Complete |
| 19-20 | BlueBubbles + Release | ~3,000 | ✅ Complete |
| 21-24 | Browser, Canvas, Hooks, Polish | ~5,000 | ✅ Complete |

### Intentional Non-Ports

The following were intentionally not ported:

| Area | Reason | Alternative |
|------|--------|-------------|
| iOS/macOS/Android apps | Platform-native (Swift/Kotlin) | Use web dashboard |
| Voice calls | Low priority, complex | Use other voice apps |
| Native app SDK parity | Vendor SDKs required | Use web APIs |

### Migration Notes

Users migrating from the TypeScript version should note:

1. **Config format**: JSON → TOML
2. **Config location**: `~/.clawdbot/` → `~/.config/krabkrab/`
3. **CLI syntax**: Minor changes (see [Migration Guide](docs/install/migrating.md))
4. **Most connectors**: Compatible with same tokens/webhooks

---

## Future Plans

See [PLAN.md](PLAN.md) for detailed roadmap and future features.

Highlights:
- Docker container images
- WebRTC voice calls
- Local LLM inference
- Enterprise SSO

---

For migration details, see [Migration Guide](docs/install/migrating.md).
For porting status, see [PORTING.md](PORTING.md).
