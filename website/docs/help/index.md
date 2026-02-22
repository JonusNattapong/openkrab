---
summary: "Help hub: common fixes, install sanity, and where to look when something breaks"
read_when:
  - You're new and want the "what do I click/run" guide
  - Something broke and you want the fastest path to a fix
title: "Help"
---

# Help

If you want a quick "get unstuck" flow, start here:

- **Troubleshooting:** [Start here](/help/troubleshooting)
- **Install sanity (Rust/cargo):** [Install](/install)
- **Gateway issues:** [Gateway troubleshooting](/gateway/troubleshooting)
- **Logs:** [Logging](/logging) and [Gateway logging](/gateway/logging)
- **Repairs:** [Doctor](/gateway/doctor)

If you're looking for conceptual questions (not "something broke"):

- [FAQ (concepts)](/help/faq)

## Quick Fixes

### Gateway won't start

```bash
krabkrab doctor          # Check configuration
krabkrab status          # Check gateway status
krabkrab logs --follow   # View logs
```

### Can't connect to channel

1. Check channel configuration: `krabkrab channels status`
2. Verify API tokens are set correctly
3. Check gateway logs for errors

### AI not responding

1. Verify provider API key: `krabkrab config get providers.openai.api_key`
2. Check model status: `krabkrab models status`
3. Test with simple query: `krabkrab ask "hello"`

## Getting Help

- GitHub Issues: [github.com/openkrab/openkrab/issues](https://github.com/openkrab/openkrab/issues)
- Check existing documentation in [Docs](/start/hubs)
- Review [Troubleshooting](/help/troubleshooting) guide
