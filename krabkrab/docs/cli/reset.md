---
summary: "CLI reference for `krabkrab reset` (reset local state/config)"
read_when:
  - You want to wipe local state while keeping the CLI installed
  - You want a dry-run of what would be removed
title: "reset"
---

# `krabkrab reset`

Reset local config/state (keeps the CLI installed).

```bash
krabkrab reset
krabkrab reset --dry-run
krabkrab reset --scope config+creds+sessions --yes --non-interactive
```
