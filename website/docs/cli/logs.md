---
summary: "CLI reference for `openkrab logs` (tail gateway logs via RPC)"
read_when:
  - You need to tail Gateway logs remotely (without SSH)
  - You want JSON log lines for tooling
title: "logs"
---

# `openkrab logs`

Tail Gateway file logs over RPC (works in remote mode).

Related:

- Logging overview: [Logging](/logging)

## Examples

```bash\nOpenKrab logs\nOpenKrab logs --follow\nOpenKrab logs --json\nOpenKrab logs --limit 500\nOpenKrab logs --local-time\nOpenKrab logs --follow --local-time
```

Use `--local-time` to render timestamps in your local timezone.

