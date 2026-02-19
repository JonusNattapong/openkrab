# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2026.2.19] - 2026-02-19

### Added

#### Core Platform
- Gateway WebSocket control plane for sessions, channels, and events
- Multi-channel inbox with unified message handling
- Multi-agent routing for channel/account isolation
- Session management with context, memory, and transcripts

#### Connectors (15+ platforms)
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
- **WhatsApp** - Basic stub (use JS layer for full support)
- **LINE** - Basic support with signature verification

#### Providers (7 LLM providers)
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

#### Security
- DM pairing for unknown sender approval
- PKCE OAuth 2.0 with SHA-256 challenge
- Webhook signature verification (LINE, Slack)
- Allowlist/denylist management

#### CLI Commands
- `gateway start` - Start the gateway server
- `status` - Check system status
- `configure` - Interactive configuration
- `telegram` - Send Telegram messages
- `slack` - Send Slack messages
- `discord` - Send Discord messages
- `memory sync` - Sync documents to memory
- `memory search` - Search memory
- `ask` - Direct LLM query
- `models` - List available models

#### Infrastructure
- Retry policy with exponential backoff
- Rate limiting
- Circuit breaker pattern
- Process/subprocess management
- Cron job scheduling

### Technical Details

- **Language**: Rust 1.75+
- **Architecture**: Gateway WebSocket control plane + multi-channel routing
- **Config Format**: TOML (changed from JSON)
- **Config Location**: `~/.config/krabkrab/` (changed from `~/.clawdbot/`)
- **Test Coverage**: 410+ tests, 0 failures

### Porting Status

All 20 phases from the TypeScript version have been ported:

| Phase | Module(s) |
|-------|-----------|
| 1-5 | Core, config, channels, connectors |
| 6-10 | Commands, providers, gateway, memory, media |
| 11-15 | Agents, plugins, security, OAuth, providers |
| 16-20 | Provider auth, Discord, security, BlueBubbles, release |

### Intentional Non-Ports

The following were intentionally not ported:

| Area | Reason |
|------|--------|
| iOS/macOS/Android apps | Platform-native (Swift/Kotlin) |
| Browser automation | JS-driven control plane in original TS layer |
| Native app SDK parity (WhatsApp/LINE proprietary SDK features) | Vendor SDKs required |

### Migration Notes

Users migrating from the TypeScript version should note:

1. **Config format**: JSON → TOML
2. **Config location**: `~/.clawdbot/` → `~/.config/krabkrab/`
3. **CLI syntax**: Minor changes (see MIGRATION_NOTES.md)
4. **Most connectors**: Compatible with same tokens/webhooks

---

## Future Releases

### Planned

- [ ] Native WhatsApp SDK parity extras
- [ ] Native LINE SDK parity extras
- [ ] Browser automation (via JS layer integration)
- [ ] Native macOS notifications (via `cocoa` crate)
- [ ] Docker container images
- [ ] Cross-compiled binaries for multiple platforms

---

For migration details, see [MIGRATION_NOTES.md](MIGRATION_NOTES.md).
For porting status, see [PORTING.md](PORTING.md).
