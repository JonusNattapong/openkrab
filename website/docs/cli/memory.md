---
summary: "CLI reference for `openkrab memory` (status/index/search)"
read_when:
  - You want to index or search semantic memory
  - Youâ€™re debugging memory availability or indexing
title: "memory"
---

# `openkrab memory`

Manage semantic memory indexing and search.
Provided by the active memory plugin (default: `memory-core`; set `plugins.slots.memory = "none"` to disable).

Related:

- Memory concept: [Memory](/concepts/memory)
- Plugins: [Plugins](/tools/plugin)

## Examples

```bash\nOpenKrab memory status\nOpenKrab memory status --deep\nOpenKrab memory status --deep --index\nOpenKrab memory status --deep --index --verbose\nOpenKrab memory index\nOpenKrab memory index --verbose\nOpenKrab memory search "release checklist"\nOpenKrab memory status --agent main\nOpenKrab memory index --agent main --verbose
```

## Options

Common:

- `--agent <id>`: scope to a single agent (default: all configured agents).
- `--verbose`: emit detailed logs during probes and indexing.

Notes:

- `memory status --deep` probes vector + embedding availability.
- `memory status --deep --index` runs a reindex if the store is dirty.
- `memory index --verbose` prints per-phase details (provider, model, sources, batch activity).
- `memory status` includes any extra paths configured via `memorySearch.extraPaths`.

