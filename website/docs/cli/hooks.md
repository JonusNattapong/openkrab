---
summary: "CLI reference for `openkrab hooks` (agent hooks)"
read_when:
  - You want to manage agent hooks
  - You want to install or update hooks
title: "hooks"
---

# `openkrab hooks`

Manage agent hooks (event-driven automations for commands like `/new`, `/reset`, and gateway startup).

Related:

- Hooks: [Hooks](/automation/hooks)
- Plugin hooks: [Plugins](/tools/plugin#plugin-hooks)

## List All Hooks

```bash\nOpenKrab hooks list
```

List all discovered hooks from workspace, managed, and bundled directories.

**Options:**

- `--eligible`: Show only eligible hooks (requirements met)
- `--json`: Output as JSON
- `-v, --verbose`: Show detailed information including missing requirements

**Example output:**

```
Hooks (4/4 ready)

Ready:
  ðŸš€ boot-md âœ“ - Run BOOT.md on gateway startup
  ðŸ“Ž bootstrap-extra-files âœ“ - Inject extra workspace bootstrap files during agent bootstrap
  ðŸ“ command-logger âœ“ - Log all command events to a centralized audit file
  ðŸ’¾ session-memory âœ“ - Save session context to memory when /new command is issued
```

**Example (verbose):**

```bash\nOpenKrab hooks list --verbose
```

Shows missing requirements for ineligible hooks.

**Example (JSON):**

```bash\nOpenKrab hooks list --json
```

Returns structured JSON for programmatic use.

## Get Hook Information

```bash\nOpenKrab hooks info <name>
```

Show detailed information about a specific hook.

**Arguments:**

- `<name>`: Hook name (e.g., `session-memory`)

**Options:**

- `--json`: Output as JSON

**Example:**

```bash\nOpenKrab hooks info session-memory
```

**Output:**

```
ðŸ’¾ session-memory âœ“ Ready

Save session context to memory when /new command is issued

Details:
  Source: openkrab-bundled
  Path: /path/to/openkrab/hooks/bundled/session-memory/HOOK.md
  Handler: /path/to/openkrab/hooks/bundled/session-memory/handler.ts
  Homepage: https://docs.openkrab.ai/automation/hooks#session-memory
  Events: command:new

Requirements:
  Config: âœ“ workspace.dir
```

## Check Hooks Eligibility

```bash\nOpenKrab hooks check
```

Show summary of hook eligibility status (how many are ready vs. not ready).

**Options:**

- `--json`: Output as JSON

**Example output:**

```
Hooks Status

Total hooks: 4
Ready: 4
Not ready: 0
```

## Enable a Hook

```bash\nOpenKrab hooks enable <name>
```

Enable a specific hook by adding it to your config (`~/.openkrab/config.json`).

**Note:** Hooks managed by plugins show `plugin:<id>` in `openkrab hooks list` and
canâ€™t be enabled/disabled here. Enable/disable the plugin instead.

**Arguments:**

- `<name>`: Hook name (e.g., `session-memory`)

**Example:**

```bash\nOpenKrab hooks enable session-memory
```

**Output:**

```
âœ“ Enabled hook: ðŸ’¾ session-memory
```

**What it does:**

- Checks if hook exists and is eligible
- Updates `hooks.internal.entries.<name>.enabled = true` in your config
- Saves config to disk

**After enabling:**

- Restart the gateway so hooks reload (menu bar app restart on macOS, or restart your gateway process in dev).

## Disable a Hook

```bash\nOpenKrab hooks disable <name>
```

Disable a specific hook by updating your config.

**Arguments:**

- `<name>`: Hook name (e.g., `command-logger`)

**Example:**

```bash\nOpenKrab hooks disable command-logger
```

**Output:**

```
â¸ Disabled hook: ðŸ“ command-logger
```

**After disabling:**

- Restart the gateway so hooks reload

## Install Hooks

```bash\nOpenKrab hooks install <path-or-spec>\nOpenKrab hooks install <npm-spec> --pin
```

Install a hook pack from a local folder/archive or npm.

Npm specs are **registry-only** (package name + optional version/tag). Git/URL/file
specs are rejected. Dependency installs run with `--ignore-scripts` for safety.

**What it does:**

- Copies the hook pack into `~/.openkrab/hooks/<id>`
- Enables the installed hooks in `hooks.internal.entries.*`
- Records the install under `hooks.internal.installs`

**Options:**

- `-l, --link`: Link a local directory instead of copying (adds it to `hooks.internal.load.extraDirs`)
- `--pin`: Record npm installs as exact resolved `name@version` in `hooks.internal.installs`

**Supported archives:** `.zip`, `.tgz`, `.tar.gz`, `.tar`

**Examples:**

```bash
# Local directory\nOpenKrab hooks install ./my-hook-pack

# Local archive\nOpenKrab hooks install ./my-hook-pack.zip

# NPM package\nOpenKrab hooks install @openkrab/my-hook-pack

# Link a local directory without copying\nOpenKrab hooks install -l ./my-hook-pack
```

## Update Hooks

```bash\nOpenKrab hooks update <id>\nOpenKrab hooks update --all
```

Update installed hook packs (npm installs only).

**Options:**

- `--all`: Update all tracked hook packs
- `--dry-run`: Show what would change without writing

When a stored integrity hash exists and the fetched artifact hash changes,\nOpenKrab prints a warning and asks for confirmation before proceeding. Use
global `--yes` to bypass prompts in CI/non-interactive runs.

## Bundled Hooks

### session-memory

Saves session context to memory when you issue `/new`.

**Enable:**

```bash\nOpenKrab hooks enable session-memory
```

**Output:** `~/.openkrab/workspace/memory/YYYY-MM-DD-slug.md`

**See:** [session-memory documentation](/automation/hooks#session-memory)

### bootstrap-extra-files

Injects additional bootstrap files (for example monorepo-local `AGENTS.md` / `TOOLS.md`) during `agent:bootstrap`.

**Enable:**

```bash\nOpenKrab hooks enable bootstrap-extra-files
```

**See:** [bootstrap-extra-files documentation](/automation/hooks#bootstrap-extra-files)

### command-logger

Logs all command events to a centralized audit file.

**Enable:**

```bash\nOpenKrab hooks enable command-logger
```

**Output:** `~/.openkrab/logs/commands.log`

**View logs:**

```bash
# Recent commands
tail -n 20 ~/.openkrab/logs/commands.log

# Pretty-print
cat ~/.openkrab/logs/commands.log | jq .

# Filter by action
grep '"action":"new"' ~/.openkrab/logs/commands.log | jq .
```

**See:** [command-logger documentation](/automation/hooks#command-logger)

### boot-md

Runs `BOOT.md` when the gateway starts (after channels start).

**Events**: `gateway:startup`

**Enable**:

```bash\nOpenKrab hooks enable boot-md
```

**See:** [boot-md documentation](/automation/hooks#boot-md)

