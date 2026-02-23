---
summary: "CLI reference for `openkrab health` (gateway health endpoint via RPC)"
read_when:
  - You want to quickly check the running Gatewayâ€™s health
title: "health"
---

# `openkrab health`

Fetch health from the running Gateway.

```bash\nOpenKrab health\nOpenKrab health --json\nOpenKrab health --verbose
```

Notes:

- `--verbose` runs live probes and prints per-account timings when multiple accounts are configured.
- Output includes per-agent session stores when multiple agents are configured.

