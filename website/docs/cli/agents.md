---
summary: "CLI reference for `openkrab agents` (list/add/delete/set identity)"
read_when:
  - You want multiple isolated agents (workspaces + routing + auth)
title: "agents"
---

# `openkrab agents`

Manage isolated agents (workspaces + auth + routing).

Related:

- Multi-agent routing: [Multi-Agent Routing](/concepts/multi-agent)
- Agent workspace: [Agent workspace](/concepts/agent-workspace)

## Examples

```bash\nOpenKrab agents list\nOpenKrab agents add work --workspace ~/.openkrab/workspace-work\nOpenKrab agents set-identity --workspace ~/.openkrab/workspace --from-identity\nOpenKrab agents set-identity --agent main --avatar avatars/openkrab.png\nOpenKrab agents delete work
```

## Identity files

Each agent workspace can include an `IDENTITY.md` at the workspace root:

- Example path: `~/.openkrab/workspace/IDENTITY.md`
- `set-identity --from-identity` reads from the workspace root (or an explicit `--identity-file`)

Avatar paths resolve relative to the workspace root.

## Set identity

`set-identity` writes fields into `agents.list[].identity`:

- `name`
- `theme`
- `emoji`
- `avatar` (workspace-relative path, http(s) URL, or data URI)

Load from `IDENTITY.md`:

```bash\nOpenKrab agents set-identity --workspace ~/.openkrab/workspace --from-identity
```

Override fields explicitly:

```bash\nOpenKrab agents set-identity --agent main --name "openkrab" --emoji "ðŸ¦ž" --avatar avatars/openkrab.png
```

Config sample:

```json5
{
  agents: {
    list: [
      {
        id: "main",
        identity: {
          name: "openkrab",
          theme: "space lobster",
          emoji: "ðŸ¦ž",
          avatar: "avatars/openkrab.png",
        },
      },
    ],
  },
}
```

