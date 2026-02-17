---
summary: "CLI reference for `krabkrab agents` (list/add/delete/set identity)"
read_when:
  - You want multiple isolated agents (workspaces + routing + auth)
title: "agents"
---

# `krabkrab agents`

Manage isolated agents (workspaces + auth + routing).

Related:

- Multi-agent routing: [Multi-Agent Routing](/concepts/multi-agent)
- Agent workspace: [Agent workspace](/concepts/agent-workspace)

## Examples

```bash
krabkrab agents list
krabkrab agents add work --workspace ~/.krabkrab/workspace-work
krabkrab agents set-identity --workspace ~/.krabkrab/workspace --from-identity
krabkrab agents set-identity --agent main --avatar avatars/krabkrab.png
krabkrab agents delete work
```

## Identity files

Each agent workspace can include an `IDENTITY.md` at the workspace root:

- Example path: `~/.krabkrab/workspace/IDENTITY.md`
- `set-identity --from-identity` reads from the workspace root (or an explicit `--identity-file`)

Avatar paths resolve relative to the workspace root.

## Set identity

`set-identity` writes fields into `agents.list[].identity`:

- `name`
- `theme`
- `emoji`
- `avatar` (workspace-relative path, http(s) URL, or data URI)

Load from `IDENTITY.md`:

```bash
krabkrab agents set-identity --workspace ~/.krabkrab/workspace --from-identity
```

Override fields explicitly:

```bash
krabkrab agents set-identity --agent main --name "KrabKrab" --emoji "🦞" --avatar avatars/krabkrab.png
```

Config sample:

```json5
{
  agents: {
    list: [
      {
        id: "main",
        identity: {
          name: "KrabKrab",
          theme: "space lobster",
          emoji: "🦞",
          avatar: "avatars/krabkrab.png",
        },
      },
    ],
  },
}
```
