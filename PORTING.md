# Porting Status — OpenClaw → OpenKrab

> **Source**: `../openclaw` (TypeScript/Node.js)
> **Target**: `openkrab/` (Rust)
> **Last Updated**: 2026-02-20

---

## Summary

| Phase | Status | Description |
|-------|--------|-------------|
| 1-20 | ✅ Complete | Core modules, connectors, providers |
| 21 | ✅ Complete | Optional extensions (copilot-proxy, open-prose, zalouser) |
| 22 | ✅ Complete | Shared utilities |

**Tests**: 410+ unit + integration tests passing

---

## Ported Modules

### Core (Fully Ported)

| Module | OpenKrab Location | Notes |
|--------|-------------------|-------|
| `agents/` | `src/agents/` | Agent runner loop |
| `auto-reply/` | `src/auto_reply/` | Keyword auto-reply engine |
| `broadcast/` | `src/broadcast/` | 🆕 Fan-out message broadcast |
| `channels/` | `src/channels/` | Channel registry & config |
| `commands/` | `src/commands/` | CLI sub-commands |
| `compat/` | `src/compat/` | Legacy API compatibility |
| `cron/` | `src/cron/` | Scheduled task engine |
| `daemon/` | `src/daemon.rs` | Background service manager |
| `dashboard/` | `src/dashboard.rs` | 🆕 Runtime stats dashboard |
| `diagnostics/` | `src/diagnostics/` | 🆕 OTel-compatible diagnostics |
| `gateway/` | `src/gateway/` | WebSocket control plane |
| `infra/` | `src/infra/` | Retry, rate-limit, circuit-breaker |
| `link-understanding/` | `src/link_understanding/` | URL metadata extraction |
| `llm-task/` | `src/llm_task/` | LLM task runner |
| `logging/` | `src/logging_impl/` | Renamed from `logging/` |
| `markdown/` | `src/markdown/` | Markdown renderer |
| `matrix/` | `src/matrix/` | Matrix connector |
| `media/` | `src/media/` | Media upload/download |
| `media-understanding/` | `src/media_understanding/` | Vision/audio analysis |
| `memory/` | `src/memory/` | Conversation memory |
| `node-host/` | `src/node_host/` | Device node integration |
| `oauth/` | `src/oauth/` | 🆕 OAuth 2.0 PKCE helper |
| `pairing/` | `src/pairing/` | Device pairing protocol |
| `plugin-sdk/` | `src/plugin_sdk/` | Plugin API types |
| `plugins/` | `src/plugins/` | Plugin loader & registry |
| `polls/` | `src/polls/` | In-chat polls |
| `process/` | `src/process/` | Subprocess management |
| `providers/` | `src/providers/` | 10 providers (see below) |
| `routing/` | `src/routing/` | Message routing rules |
| `sessions/` | `src/sessions/` | Conversation sessions |
| `signal/` | `src/signal/` | Signal connector |
| `slack/` | `src/slack/` | Slack connector |
| `terminal/` | `src/terminal/` | TUI terminal interface |
| `thread-ownership/` | `src/thread_ownership/` | Thread ownership tracking |
| `tts/` | `src/tts/` | Text-to-speech |
| `tui/` | `src/tui/` | Terminal UI |
| `voice/` | `src/voice/` | 🆕 Voice system (wake/speak) |
| `web/` | `src/web_connector/` | Renamed from `web/` |
| `whatsapp/` | `src/whatsapp/` | WhatsApp connector |

### Partial / Stubs

| Module | OpenKrab Location | Status |
|--------|-------------------|--------|
| `acp/` | `src/acp/` | ⚡ Core types only |
| `browser/` | `src/browser/` | ⚡ Simplified CDP |
| `canvas-host/` | `src/canvas_host/` | ⚡ Simplified A2UI |
| `config/` | `src/config.rs` + `openkrab_config.rs` | ⚡ Some fields missing |
| `discord/` | `src/connectors/discord.rs` | ⚡ Merged, simplified vs 70+ TS files |
| `hooks/` | `src/hooks/` | ⚡ Core types only |
| `security/` | `src/security.rs` + `secure.rs` | ⚡ Partial features |
| `utils/` | `src/utils.rs` | ⚡ Core utilities only |

### Not Ported (Intentional)

| Module | Reason |
|--------|--------|
| `cli/` | TypeScript CLI - not needed in Rust |
| `imessage/` | macOS only - private Apple API |
| `macos/` | macOS specific |
| `scripts/` | Build scripts |
| `test-helpers/` | Test utilities - not needed |
| `test-utils/` | Test utilities - not needed |
| `types/` | TypeScript definitions - not needed |
| `wizard/` | Onboarding - not ported |

---

## Ported Extensions (30/37)

### ✅ Ported

| Extension | OpenKrab Location |
|-----------|-------------------|
| `bluebubbles/` | `src/connectors/bluebubbles/` |
| `copilot-proxy/` | `src/providers/copilot_proxy.rs` |
| `diagnostics-otel/` | `src/diagnostics/` |
| `discord/` | `src/connectors/discord.rs` |
| `feishu/` | `src/connectors/feishu.rs` |
| `google-gemini-cli-auth/` | `src/providers/gemini_cli_auth.rs` |
| `googlechat/` | `src/connectors/googlechat.rs` |
| `irc/` | `src/connectors/irc.rs` |
| `line/` | `src/connectors/line.rs` |
| `llm-task/` | `src/llm_task/` |
| `lobster/` | `src/tools/lobster.rs` |
| `matrix/` | `src/matrix/` |
| `mattermost/` | `src/connectors/mattermost.rs` |
| `minimax-portal-auth/` | `src/providers/minimax_oauth.rs` |
| `msteams/` | `src/connectors/msteams.rs` |
| `nextcloud-talk/` | `src/connectors/nextcloud_talk.rs` |
| `nostr/` | `src/connectors/nostr.rs` |
| `open-prose/` | `src/tools/open_prose.rs` |
| `qwen-portal-auth/` | `src/providers/qwen_oauth.rs` |
| `shared/` | `src/shared/` (17 files) |
| `signal/` | `src/signal/` |
| `slack/` | `src/slack/` |
| `telegram/` | `src/connectors/telegram.rs` |
| `thread-ownership/` | `src/thread_ownership/` |
| `tlon/` | `src/connectors/tlon.rs` |
| `twitch/` | `src/connectors/twitch.rs` |
| `whatsapp/` | `src/whatsapp/` |
| `zalo/` | `src/connectors/zalo.rs` |
| `zalouser/` | `src/connectors/zalouser.rs` |

### ❌ Not Ported (7/37)

| Extension | Reason |
|-----------|--------|
| `device-pair/` | Bluetooth - hardware specific |
| `google-antigravity-auth/` | Complex OAuth - low priority |
| `imessage/` | macOS only - private Apple API |
| `memory-core/` | Using sqlite-vec instead |
| `memory-lancedb/` | Alternative vector DB - low priority |
| `phone-control/` | Hardware specific |
| `talk-voice/` | Voice system - low priority |
| `voice-call/` | Voice calls - low priority |

---

## Providers (10 Total)

| Provider | File | Source |
|----------|------|--------|
| OpenAI | `src/providers/openai.rs` | 🆕 New |
| Gemini | `src/providers/gemini.rs` | 🆕 New |
| Ollama | `src/providers/ollama.rs` | 🆕 New |
| GitHub Copilot Token | `src/providers/copilot_token.rs` | `github-copilot-token.ts` |
| GitHub Copilot Models | `src/providers/copilot_models.rs` | `github-copilot-models.ts` |
| GitHub Copilot Proxy | `src/providers/copilot_proxy.rs` | `extensions/copilot-proxy/` |
| MiniMax OAuth | `src/providers/minimax_oauth.rs` | `extensions/minimax-portal-auth/` |
| Gemini CLI Auth | `src/providers/gemini_cli_auth.rs` | `extensions/google-gemini-cli-auth/` |
| Qwen OAuth | `src/providers/qwen_oauth.rs` | `extensions/qwen-portal-auth/` |

---

## Shared Code (17/17 Files Ported)

| TypeScript | Rust |
|------------|------|
| `text-chunking.ts` | `text_chunking.rs` |
| `string-normalization.ts` | `string_normalization.rs` |
| `requirements.ts` | `requirements.rs` |
| `pid-alive.ts` | `pid_alive.rs` |
| `entry-metadata.ts` | `entry_metadata.rs` |
| `usage-aggregates.ts` | `usage_aggregates.rs` |
| `chat-envelope.ts` | `chat_envelope.rs` |
| `chat-content.ts` | `chat_content.rs` |
| `frontmatter.ts` | `frontmatter.rs` |
| `subagents-format.ts` | `subagents_format.rs` |
| `diff-engine.ts` | `diff_engine.rs` |
| `escape-regex.ts` | `escape_regex.rs` |
| `file-icons.ts` | `file_icons.rs` |
| `jsonc.ts` | `jsonc.rs` |
| `markdown-split.ts` | `markdown_split.rs` |
| `merge-pdfs.ts` | `merge_pdfs.rs` |
| `uri-template.ts` | `uri_template.rs` |

---

## Statistics

| Metric | OpenClaw | OpenKrab |
|--------|----------|----------|
| src/ modules | 68 | 48 ported + 8 partial |
| Extensions | 37 | 30 ported / 7 not ported |
| Providers | 4 | 10 |
| Connectors | 18 | 18 |
| Shared files | 10 | 17 (all ported) |
| Tests | - | 410+ passing |
| Compilation | - | ✅ All fixed |

---

## Running Tests

```bash
cd openkrab
cargo test                  # all tests
cargo test --lib            # unit tests only
cargo test -p krabkrab-cli  # CLI tests
cargo build                 # check compilation
```

---

## Phase History

| Phase | Modules | Status |
|-------|---------|--------|
| 1 | `common`, `version`, `utils` | ✅ |
| 2 | `logging` | ✅ |
| 3 | `config` | ✅ |
| 4 | `channels` | ✅ |
| 5 | Connectors (Telegram, Slack, IRC, etc.) | ✅ |
| 6 | `commands` | ✅ |
| 7 | Providers (OpenAI, Gemini, Ollama) | ✅ |
| 8 | `gateway`, `daemon`, `security` | ✅ |
| 9 | `routing`, `sessions`, `memory`, `hooks`, `auto_reply`, `cron` | ✅ |
| 10 | `media`, `media_understanding`, `tts`, `markdown`, `link_understanding` | ✅ |
| 11 | `agents`, `llm_task`, `thread_ownership`, `plugins`, `plugin_sdk`, `acp` | ✅ |
| 12 | `infra`, `process`, `terminal`, `compat`, `broadcast`, `pairing`, `polls` | ✅ |
| 13 | `signal`, `matrix`, `web_connector`, `diagnostics` | ✅ |
| 14 | `tools::lobster`, `oauth` | ✅ |
| 15 | `providers::minimax_oauth`, `providers::gemini_cli_auth`, `providers::copilot_token` | ✅ |
| 16 | `providers::qwen_oauth`, `providers::copilot_models` | ✅ |
| 17 | `connectors::discord` | ✅ |
| 18 | Security hardening (PKCE, signatures) | ✅ |
| 19 | `connectors::bluebubbles` | ✅ |
| 20 | Release readiness | ✅ |
| 21 | `providers::copilot_proxy`, `tools::open_prose`, `connectors::zalouser` | ✅ |
| 22 | `shared` utilities (all 17 files) | ✅ |
| 23 | `.github` workflows and templates adaptation | ✅ |
| 24 | `wizard` onboarding (prompts, session, gateway config, completion, finalize) | ✅ |
| 25 | De-mocking CLI commands & Wizard parity | ⏳ Planned |

---

## De-mocking Plan (CLI & Wizard Parity)

During the rapid port to Rust, several CLI commands were provided with stubbed implementations (`format!("... (not yet implemented)")`) or simplified logic compared to the original Node.js OpenClaw codebase.

### 1. Onboarding Wizard (`src/commands/onboard.rs`)

* **OS Detection**: Implement checks for Windows/WSL2 and print the WSL2 recommendation banner.
* **Security Warnings**: Add the detailed security baseline warning (sandbox, least-privilege tools, tailscale) and prompt for acknowledgment.
* **Config Discovery**: Discover existing `gateway.port`, `gateway.bind`, and `models` from existing `openclaw.json` instead of starting from scratch every time.
* **Channel Probing**: Fetch and display the status of all available channels (e.g., "Telegram: configured", "Feishu: install plugin to enable").
* **Health Checks**: Attempt to connect to the Gateway WS endpoint (`ws://127.0.0.1...`) and report health status accurately during the "Restarting Gateway" phase.
* **Browser Launch**: Automatically open the Web UI URL with the generated authentication token after onboarding.

### 2. Administrative Commands (`src/commands/admin.rs`)

* **Actual Implementations Needed**:
  * `skills` (listing, fetching updates, enabling/disabling)
  * `sandbox` (Docker container control)
  * `nodes` (Device node management)
  * `browser` (Chrome CDP profile management)
  * `hooks` & `webhooks` (Listing and modifying hook configurations)
  * `exec-approvals`
  * `dns` & `directory`
  * `system` & `devices`

### 3. Task & State Management

* **Cron** (`src/commands/cron.rs`): Implement `remove`, `enable`, and `disable`.
* **Pairing** (`src/commands/pairing.rs`): Implement `revoke`.
* **Channels** (`src/commands/channels.rs`): Implement actual `add` and `remove` logic against the active gateway.
* **Sessions** (`src/commands/sessions.rs`): Implement `lock`, `unlock`, `archive`, and `delete`.
* **Logs** (`src/commands/logs.rs`): Implement `--follow` tailing.
* **Memory** (`src/commands/memory.rs`): Parity with SQLite-vec / LanceDB integrations.

### 4. Detailed Module Porting Checklists

#### 4.1. Sessions Module (`src/sessions/`)

While the core struct and some tests were ported, there's significant logic missing from the TypeScript equivalent (`openclaw/src/sessions/`). We need to implement:

* [x] **`input-provenance.ts` parity**: Add `InputProvenance` struct (kind: `external_user`, `inter_session`, `internal_system`), normalization logic, and injection into `AgentMessage`s so the agent knows where input came from.
* [x] **`level-overrides.ts` parity**: Enhance `VerbosityLevel` logic to include `parseVerboseOverride` returning nullable variants, and `applyVerboseOverride` to properly clear or set the override on the session config.
* [x] **`model-overrides.ts` parity**: Add `applyModelOverrideToSessionEntry` logic that manages `providerOverride`, `modelOverride`, `authProfileOverride`, checks `isDefault` flags, and properly clears `fallbackNoticeSelectedModel` when a user switches models.
* [x] **`send-policy.ts` parity**: Fully implement `resolveSendPolicy`. It must parse config rules (`matchChannel`, `matchChatType`, `rawKeyPrefix`), strip `agent:<id>:` prefixes, and deduce channel vs group chat types based on session keys before falling back to default logic.
* [x] **`session-key-utils.ts` parity**: The whole file is missing. Add string matching utilities: `parseAgentSessionKey`, `isCronRunSessionKey`, `isCronSessionKey`, `isSubagentSessionKey`, `getSubagentDepth`, `isAcpSessionKey`, and `resolveThreadParentSessionKey`.
* [x] **`session-label.ts` parity**: Add label parsing logic enforcing the 64-character limit (`SESSION_LABEL_MAX_LENGTH`).

#### 4.2. Memory Module (`src/memory/`)

* [x] **Automatic background syncing** of Markdown files via `notify`.
* [x] **Fallback FTS-only search** mechanism when vector search fails.
* [x] **MMR and Temporal Decay** for relevance ranking to avoid redundancy and prioritize recent info.
* [x] **Session integration** (`warm_session`) for proactive syncing before processing.
