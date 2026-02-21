# De-mocking Plan for OpenKrab CLI

The OpenKrab CLI was rapidly ported from the TypeScript `openclaw` codebase. While core functionalities (gateway daemon, prompt handling, config parsing) were ported robustly, many secondary CLI sub-commands were left as "mocks" or "stubs" that simply return `format!("...")` strings.

To achieve 100% feature parody with the original `openclaw` application, we need to replace these mock functions with actual active logic.

This document outlines all identified mock targets, what they currently do, and what they *should* do based on the reference TypeScript codebase.

---

## 1. Administration Commands (`src/commands/admin.rs`)

### `skills` command

- **Current (OpenKrab):** Returns `format!("skills: action={}", action)`
- **Target (OpenClaw):** Read/Write to the gateway HTTP/RPC to install/uninstall, list, and toggle skills. Requires communicating with the active daemon or modifying the `skills.toml` config and signaling a reload.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\skills-cli.ts`

### `sandbox` command

- **Current (OpenKrab):** Returns `format!("sandbox: action={} (docker sandbox control)", action)`
- **Target (OpenClaw):** Controls the Docker container executing the agent's code. Needs to ping Docker socket, start/stop containers, and flush containers.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\sandbox-cli.ts`

### `nodes` command

- **Current (OpenKrab):** Returns `format!("nodes: action={} (device node management)", action)`
- **Target (OpenClaw):** Interact with local hardware nodes (camera, mic) or remote nodes.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\nodes-cli\nodes-cli.ts`

### `browser` command

- **Current (OpenKrab):** Returns `format!("browser: action={} (chrome/chromium control)", action)`
- **Target (OpenClaw):** Connect to a local Chrome Debugging Protocol (CDP) session, manage automated browser contexts, and list active tabs.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\browser-cli.ts`

### `hooks` & `webhooks` commands

- **Current (OpenKrab):** Returns `format!("hooks: ...")` and `format!("webhooks: ...")`
- **Target (OpenClaw):** Manage event subscriptions (e.g., `MESSAGE_INBOUND`). Update config to add webhook endpoint URLs and manage authentication tokens for them.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\hooks-cli.ts` & `webhooks-cli.ts`

### `exec-approvals` command

- **Current (OpenKrab):** Returns `format!("exec-approvals: action={}", action)`
- **Target (OpenClaw):** Command to manage user approval policies for command execution (`allow`, `deny`, `always-ask`).
- **Reference:** `D:\Projects\Github\openclaw\src\cli\exec-approvals-cli.ts`

### `dns`, `directory`, `system`, `devices` commands

- **Current (OpenKrab):** All return simple formatted strings.
- **Target (OpenClaw):** Network discovery, device registry parsing, and local system information fetching.
- **Reference:** `dns-cli.ts`, `directory-cli.ts`, `system-cli.ts`, `devices-cli.ts`

---

## 2. State & Task Management

### Cron (`src/commands/cron.rs`)

- **Current (OpenKrab):** `add` and `list` work (modifies TOML). `remove`, `enable`, `disable` return strings containing `"(not yet implemented)"`.
- **Target (OpenClaw):** Parse the `cron.toml` file, drop or toggle the specified job ID, and re-save the file.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\cron-cli\*`

### Channels (`src/commands/channels.rs`)

- **Current (OpenKrab):** `channel_add_command` and `channel_remove_command` return strings instead of altering the configuration file.
- **Target (OpenClaw):** Push/pull configurations directly to `openkrab.toml` / `channels.toml` and emit an RPC restart signal to the Gateway to immediately hot-swap the channel adapters.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\channels-cli.ts`

### Logs (`src/commands/logs.rs`)

- **Current (OpenKrab):** `logs_command` returns `"(not yet implemented)"` when the `--follow` flag is passed.
- **Target (OpenClaw):** Attach a WebSocket or Unix Domain Socket to the Gateway's log stream to tail logs in real-time.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\logs-cli.ts`

### Pairing (`src/commands/pairing.rs`)

- **Current (OpenKrab):** `pairing_revoke_command` returns `format!("Revoking pairing for device: {}", device_id)`.
- **Target (OpenClaw):** Actually delete the device's token pair from the local persistent keyring.
- **Reference:** `D:\Projects\Github\openclaw\src\cli\pairing-cli.ts`

### Sessions (`src/commands/sessions.rs`)

- **Current (OpenKrab):** `session_lock`, `session_unlock`, `session_archive`, `session_delete` return formatted strings.
- **Target (OpenClaw):** Access the SQLite memory backend (or LanceDB), find the session thread, and modify its `status` or permanently delete its rows.

---

## 3. Communication Channel CLI Injections

### Direct Messaging (`src/commands/telegram.rs`, `slack.rs`, `discord.rs`, `whatsapp_send.rs`)

- **Current (OpenKrab):** When triggering the CLI command `krabkrab telegram send <id> <msg>`, it generally just parses arguments and returns `Ok(format!("telegram to={} payload={}", ...))`.
- **Target (OpenClaw):** Should form an HTTP POST request targeting either the Gateway's local API to queue the outbound message, or send it directly using the connector's internal API logic.

---

## Summary of Action Plan

1. **Phase 1: Admin Configuration Modifiers:** Focus on `hooks`, `webhooks`, `cron`, and `channels` since these require simple modifications to the local `.toml`/`.json` config files. ✓ (cron, channels done)
2. **Phase 2: Database/State Commands:** Focus on `sessions` and `pairing` by connecting the CLI directly into the SQLite instances used by the Gateway. ✓ (pairing done via config)
3. **Phase 3: Sandbox & Browser Automation:** Rewrite `sandbox` and `browser` CLI handlers to manage Docker and Chrome processes respectively. ✓ (browser native CLI handled, sandbox Docker integration done)
4. **Phase 4: Daemon Synchronization:** Implement WebSocket/HTTP RPC calls so that the CLI can notify the Gateway to reload configurations or tail logs (`logs --follow`).
