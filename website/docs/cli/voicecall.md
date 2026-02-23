---
summary: "CLI reference for `openkrab voicecall` (voice-call plugin command surface)"
read_when:
  - You use the voice-call plugin and want the CLI entry points
  - You want quick examples for `voicecall call|continue|status|tail|expose`
title: "voicecall"
---

# `openkrab voicecall`

`voicecall` is a plugin-provided command. It only appears if the voice-call plugin is installed and enabled.

Primary doc:

- Voice-call plugin: [Voice Call](/plugins/voice-call)

## Common commands

```bash\nOpenKrab voicecall status --call-id <id>\nOpenKrab voicecall call --to "+15555550123" --message "Hello" --mode notify\nOpenKrab voicecall continue --call-id <id> --message "Any questions?"\nOpenKrab voicecall end --call-id <id>
```

## Exposing webhooks (Tailscale)

```bash\nOpenKrab voicecall expose --mode serve\nOpenKrab voicecall expose --mode funnel\nOpenKrab voicecall unexpose
```

Security note: only expose the webhook endpoint to networks you trust. Prefer Tailscale Serve over Funnel when possible.

