---
summary: "OpenKrab CLI reference for `krabkrab` commands, subcommands, and options"
read_when:
  - Adding or modifying CLI commands or options
  - Documenting new command surfaces
title: "CLI Reference"
---

# CLI reference

This page describes the current CLI behavior for OpenKrab (Rust implementation).

## Command pages

- [`setup`](/cli/setup)
- [`doctor`](/cli/doctor)
- [`dashboard`](/cli/dashboard)
- [`config`](/cli/config)
- [`reset`](/cli/reset)
- [`uninstall`](/cli/uninstall)
- [`update`](/cli/update)
- [`message`](/cli/message)
- [`agent`](/cli/agent)
- [`agents`](/cli/agents)
- [`status`](/cli/status)
- [`health`](/cli/health)
- [`sessions`](/cli/sessions)
- [`gateway`](/cli/gateway)
- [`logs`](/cli/logs)
- [`system`](/cli/system)
- [`models`](/cli/models)
- [`memory`](/cli/memory)
- [`nodes`](/cli/nodes)
- [`pairing`](/cli/pairing)
- [`channels`](/cli/channels)
- [`security`](/cli/security)
- [`skills`](/cli/skills)
- [`voice`](/cli/voice)
- [`plugin`](/cli/plugin)

## Global flags

- `--config <path>`: specify custom config file path
- `--data-dir <path>`: specify custom data directory
- `--no-color`: disable ANSI colors
- `-V`, `--version`: print version and exit
- `-h`, `--help`: show help

## Output styling

- ANSI colors and progress indicators only render in TTY sessions
- `--json` (and `--plain` where supported) disables styling for clean output
- `--no-color` disables ANSI styling; `NO_COLOR=1` is also respected

## Color palette

OpenKrab uses a lobster palette for CLI output.

- `accent` (#FF5A2D): headings, labels, primary highlights
- `accentBright` (#FF7A3D): command names, emphasis
- `success` (#2FBF71): success states
- `warn` (#FFB020): warnings, fallbacks, attention
- `error` (#E23D2D): errors, failures
- `muted` (#8B7F77): de-emphasis, metadata

## Command tree

```
krabkrab [--config <path>] [--data-dir <path>] <command>
  setup                    # Initialize config and workspace
  config
    get <key>              # Get config value
    set <key> <value>      # Set config value
    unset <key>            # Remove config value
  doctor                   # Health checks and diagnostics
  security
    audit                  # Security audit
  reset                    # Reset local config/state
  uninstall                # Uninstall service and data
  update                   # Update to latest version
  channels
    list                   # List configured channels
    status                 # Check channel health
    logs                   # Show channel logs
    add                    # Add a channel
    remove                 # Remove a channel
  skills
    list                   # List available skills
    info <name>            # Show skill details
    check                  # Check skill requirements
  memory
    status                 # Show memory index stats
    sync                   # Sync memory from path
    search <query>         # Search memory
    index                  # Reindex memory files
  message
    send                   # Send a message
  agent
    ask                    # Ask the AI agent
  agents
    list                   # List configured agents
    add <name>             # Add a new agent
    delete <id>            # Delete an agent
  status                   # Show system status
  health                   # Gateway health check
  sessions
    list                   # List sessions
    history                # Show session history
  gateway
    run                    # Run the gateway server
    status                 # Check gateway status
    start                  # Start gateway service
    stop                   # Stop gateway service
    restart                # Restart gateway service
  logs                     # View gateway logs
  system
    event                  # Send system event
    heartbeat              # Trigger heartbeat
  models
    list                   # List available models
    status                 # Show model status
    set <model>            # Set default model
  nodes
    list                   # List connected nodes
    status                 # Check node status
    approve                # Approve node pairing
  pairing
    list                   # List pairing requests
    approve <code>         # Approve pairing code
  plugins
    list                   # List loaded plugins
    load <path>            # Load a plugin
    unload <name>          # Unload a plugin
  voice
    wake                   # Force wake voice system
    sleep                  # Force sleep voice system
    status                 # Show voice status
    speak <text>           # Text-to-speech
    mic_list               # List microphones
    mic_start              # Start microphone capture
    mic_stop               # Stop microphone capture
```

## Security

- `krabkrab security audit` — audit config + local state for common security issues
- `krabkrab security audit --fix` — tighten safe defaults

## Memory

Vector search over memory files:

- `krabkrab memory status` — show index stats
- `krabkrab memory sync --path <dir>` — sync memory from directory
- `krabkrab memory search "<query>"` — semantic search over memory
- `krabkrab memory index` — reindex memory files

## Setup + configuration

### `setup`

Initialize config + workspace.

Options:
- `--workspace <dir>`: agent workspace path
- `--non-interactive`: run without prompts

### `config`

Non-interactive config helpers (get/set/unset).

Subcommands:
- `config get <key>`: print a config value (dot notation)
- `config set <key> <value>`: set a value
- `config unset <key>`: remove a value

Examples:
```bash
krabkrab config get providers.openai.api_key
krabkrab config set providers.openai.model "gpt-4o"
krabkrab config unset providers.openai.api_key
```

### `doctor`

Health checks + quick fixes (config + gateway).

Options:
- `--yes`: accept defaults without prompting
- `--non-interactive`: skip prompts

## Channel helpers

### `channels`

Manage chat channel accounts (Telegram/Discord/Slack/WhatsApp/Signal/iMessage).

Subcommands:
- `channels list`: show configured channels
- `channels status`: check gateway reachability and channel health
- `channels logs`: show recent channel logs
- `channels add`: add a channel
- `channels remove`: remove a channel

Examples:
```bash
krabkrab channels add telegram --token $TELEGRAM_BOT_TOKEN
krabkrab channels add discord --token $DISCORD_BOT_TOKEN
krabkrab channels status
```

### `skills`

List and inspect available skills.

Subcommands:
- `skills list`: list skills
- `skills info <name>`: show details for one skill
- `skills check`: summary of ready vs missing requirements

### `pairing`

Approve DM pairing requests across channels.

Subcommands:
- `pairing list`
- `pairing approve <code>`

## Messaging + agent

### `message`

Unified outbound messaging.

Examples:
```bash
krabkrab message send --to @username --text "Hello"
krabkrab telegram --to @username --text "Hello from OpenKrab!"
krabkrab discord --to 123456789 --text "Hello!"
```

### `agent` / `ask`

Run one agent turn.

Required:
- `--message <text>` or positional argument

Options:
- `--session-id <id>`
- `--channel <telegram|discord|slack>`
- `--timeout <seconds>`

Examples:
```bash
krabkrab ask "What's on my calendar today?"
krabkrab agent --message "Hello" --channel telegram
```

### `agents`

Manage isolated agents.

#### `agents list`

List configured agents.

#### `agents add [name]`

Add a new isolated agent.

Options:
- `--workspace <dir>`
- `--model <id>`

#### `agents delete <id>`

Delete an agent.

Options:
- `--force`

## System status

### `status`

Show linked session health and recent recipients.

Options:
- `--json`
- `--verbose`

### `health`

Fetch health from the running Gateway.

Options:
- `--json`
- `--timeout <ms>`

### `sessions`

List stored conversation sessions.

Options:
- `--json`
- `--verbose`

## Reset / Uninstall

### `reset`

Reset local config/state (keeps the binary).

Options:
- `--scope <config|full>`
- `--yes`
- `--dry-run`

### `uninstall`

Uninstall the gateway service + local data.

Options:
- `--service`
- `--data`
- `--all`
- `--yes`

## Gateway

### `gateway run`

Run the WebSocket Gateway.

Options:
- `--port <port>` (default: 18789)
- `--bind <host>`
- `--token <token>`

### `gateway service`

Manage the Gateway service.

Subcommands:
- `gateway status`: check gateway status
- `gateway start`: start service
- `gateway stop`: stop service
- `gateway restart`: restart service

### `logs`

Tail Gateway logs.

Examples:
```bash
krabkrab logs --follow
krabkrab logs --limit 200
krabkrab logs --json
```

## Models

### `models list`

List available models.

Options:
- `--provider <name>`
- `--json`

### `models status`

Show model provider status.

Options:
- `--json`
- `--check`

### `models set <model>`

Set default model.

## Voice

### `voice`

Voice system controls.

Subcommands:
- `voice wake`: force wake
- `voice sleep`: force sleep
- `voice status`: show status
- `voice speak <text>`: text-to-speech
- `voice mic_list`: list microphones
- `voice mic_start`: start capture
- `voice mic_stop`: stop capture

Examples:
```bash
krabkrab voice wake
krabkrab voice speak "Hello World"
krabkrab voice mic_start
```

## Plugins

Manage WASM plugins:

- `krabkrab plugin list` — list loaded plugins
- `krabkrab plugin load <path>` — load a plugin
- `krabkrab plugin unload <name>` — unload a plugin

## Environment variables

- `KRABKRAB_CONFIG_DIR`: config directory (default: `~/.config/krabkrab/`)
- `KRABKRAB_DATA_DIR`: data directory
- `RUST_LOG`: logging level (e.g., `info`, `debug`)
