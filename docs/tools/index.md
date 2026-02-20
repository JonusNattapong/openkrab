---
summary: "Agent tool surface for OpenKrab (browser, memory, voice, message, cron)"
read_when:
  - Adding or modifying agent tools
  - Understanding available tools for AI agents
title: "Tools"
---

# Tools (OpenKrab)

OpenKrab exposes **first-class agent tools** for browser, memory, voice, and messaging.
These tools are typed and the agent should rely on them directly.

## Disabling tools

You can globally allow/deny tools via `tools.allow` / `tools.deny` in `config.toml`
(deny wins). This prevents disallowed tools from being sent to model providers.

```toml
[tools]
deny = ["browser"]
```

Notes:

- Matching is case-insensitive.
- `*` wildcards are supported (`"*"` means all tools).

## Tool profiles (base allowlist)

`tools.profile` sets a **base tool allowlist** before `tools.allow`/`tools.deny`.

Profiles:

- `minimal`: `session_status` only
- `coding`: `group:fs`, `group:runtime`, `group:sessions`, `group:memory`
- `messaging`: `group:messaging`, `sessions_list`, `sessions_history`, `sessions_send`
- `full`: no restriction (same as unset)

Example (messaging-only by default):

```toml
[tools]
profile = "messaging"
allow = ["slack", "discord"]
```

## Tool groups (shorthands)

Tool policies support `group:*` entries that expand to multiple tools.

Available groups:

- `group:runtime`: `exec`, `bash`, `process`
- `group:fs`: `read`, `write`, `edit`, `apply_patch`
- `group:sessions`: `sessions_list`, `sessions_history`, `sessions_send`, `session_status`
- `group:memory`: `memory_search`, `memory_get`
- `group:web`: `web_search`, `web_fetch`
- `group:ui`: `browser`, `canvas`
- `group:messaging`: `message`
- `group:voice`: `voice_wake`, `voice_speak`, `voice_listen`

## Plugins + tools

Plugins can register **additional tools** beyond the core set.
See [Plugins](/tools/plugin) for install + config.

## Tool inventory

### `exec`

Run shell commands in the workspace.

Core parameters:

- `command` (required)
- `timeout` (seconds; kills the process if exceeded, default 1800)
- `background` (immediate background)

Notes:

- Returns `status: "running"` with a `session_id` when backgrounded.
- Use `process` to poll/log/kill background sessions.

### `process`

Manage background exec sessions.

Core actions:

- `list`, `poll`, `log`, `kill`, `clear`

### `web_search`

Search the web using Brave Search API or other providers.

Core parameters:

- `query` (required)
- `count` (1–10)

Notes:

- Requires API key configuration.
- Responses are cached.

### `web_fetch`

Fetch and extract readable content from a URL.

Core parameters:

- `url` (required)
- `extract_mode` (`markdown` | `text`)
- `max_chars` (truncate long pages)

### `browser`

Control the dedicated browser instance.

Core actions:

- `status`, `start`, `stop`, `tabs`, `open`, `close`
- `snapshot` (accessibility tree)
- `screenshot`
- `act` (UI actions: click/type/press/hover)
- `navigate`, `evaluate`

### `memory_search`

Search the memory system (vector + text).

Core parameters:

- `query` (required)
- `limit` (default 10)
- `threshold` (similarity threshold)

### `memory_get`

Retrieve a specific memory entry.

Core parameters:

- `id` (required)

### `message`

Send messages across channels.

Core actions:

- `send` (text + optional media)
- `react`, `read`, `edit`, `delete`

### `voice_speak`

Text-to-speech output.

Core parameters:

- `text` (required)
- `voice` (optional voice ID)

### `voice_wake`

Wake the voice system.

### `session_status`

Get current session status.

### `sessions_list`

List available sessions.

### `sessions_send`

Send a message to another session.

Core parameters:

- `session_key` (required)
- `message` (required)

## Parameters (common)

Gateway-backed tools:

- `gateway_url` (default `ws://127.0.0.1:18789`)
- `gateway_token` (if auth enabled)
- `timeout_ms`

## Recommended agent flows

Browser automation:

1. `browser` → `status` / `start`
2. `snapshot`
3. `act` (click/type/press)
4. `screenshot` if you need visual confirmation

Memory search:

1. `memory_search` with query
2. `memory_get` for specific entries

Voice interaction:

1. `voice_wake` to activate
2. `voice_speak` for responses

## Safety

- Avoid direct `exec` with untrusted input
- Respect user consent for voice/camera
- Use `status` to ensure permissions before invoking commands

## How tools are presented to the agent

Tools are exposed in two parallel channels:

1. **System prompt text**: a human-readable list + guidance.
2. **Tool schema**: the structured function definitions sent to the model API.

That means the agent sees both "what tools exist" and "how to call them." If a tool
doesn't appear in the system prompt or the schema, the model cannot call it.
