---
summary: "CLI reference for `krabkrab config` (get/set/unset config values)"
read_when:
  - You want to read or edit config non-interactively
title: "config"
---

# `krabkrab config`

Config helpers: get/set/unset values by path. Run without a subcommand to open
the configure wizard (same as `krabkrab configure`).

## Examples

```bash
krabkrab config get browser.executablePath
krabkrab config set browser.executablePath "/usr/bin/google-chrome"
krabkrab config set agents.defaults.heartbeat.every "2h"
krabkrab config set agents.list[0].tools.exec.node "node-id-or-name"
krabkrab config unset tools.web.search.apiKey
```

## Paths

Paths use dot or bracket notation:

```bash
krabkrab config get agents.defaults.workspace
krabkrab config get agents.list[0].id
```

Use the agent list index to target a specific agent:

```bash
krabkrab config get agents.list
krabkrab config set agents.list[1].tools.exec.node "node-id-or-name"
```

## Values

Values are parsed as JSON5 when possible; otherwise they are treated as strings.
Use `--json` to require JSON5 parsing.

```bash
krabkrab config set agents.defaults.heartbeat.every "0m"
krabkrab config set gateway.port 19001 --json
krabkrab config set channels.whatsapp.groups '["*"]' --json
```

Restart the gateway after edits.
