---
summary: "CLI reference for `openkrab config` (get/set/unset config values)"
read_when:
  - You want to read or edit config non-interactively
title: "config"
---

# `openkrab config`

Config helpers: get/set/unset values by path. Run without a subcommand to open
the configure wizard (same as `openkrab configure`).

## Examples

```bash\nOpenKrab config get browser.executablePath\nOpenKrab config set browser.executablePath "/usr/bin/google-chrome"\nOpenKrab config set agents.defaults.heartbeat.every "2h"\nOpenKrab config set agents.list[0].tools.exec.node "node-id-or-name"\nOpenKrab config unset tools.web.search.apiKey
```

## Paths

Paths use dot or bracket notation:

```bash\nOpenKrab config get agents.defaults.workspace\nOpenKrab config get agents.list[0].id
```

Use the agent list index to target a specific agent:

```bash\nOpenKrab config get agents.list\nOpenKrab config set agents.list[1].tools.exec.node "node-id-or-name"
```

## Values

Values are parsed as JSON5 when possible; otherwise they are treated as strings.
Use `--strict-json` to require JSON5 parsing. `--json` remains supported as a legacy alias.

```bash\nOpenKrab config set agents.defaults.heartbeat.every "0m"\nOpenKrab config set gateway.port 19001 --strict-json\nOpenKrab config set channels.whatsapp.groups '["*"]' --strict-json
```

Restart the gateway after edits.

