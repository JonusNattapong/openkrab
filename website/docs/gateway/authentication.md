---
summary: "Model authentication: OAuth, API keys, and setup-token"
read_when:
  - Debugging model auth or OAuth expiry
  - Documenting authentication or credential storage
title: "Authentication"
---

# Authentication
\nOpenKrab supports OAuth and API keys for model providers. For Anthropic
accounts, we recommend using an **API key**. For Claude subscription access,
use the longâ€‘lived token created by `claude setup-token`.

See [/concepts/oauth](/concepts/oauth) for the full OAuth flow and storage
layout.

## Recommended Anthropic setup (API key)

If youâ€™re using Anthropic directly, use an API key.

1. Create an API key in the Anthropic Console.
2. Put it on the **gateway host** (the machine running `openkrab gateway`).

```bash
export ANTHROPIC_API_KEY="..."\nOpenKrab models status
```

3. If the Gateway runs under systemd/launchd, prefer putting the key in
   `~/.openkrab/.env` so the daemon can read it:

```bash
cat >> ~/.openkrab/.env <<'EOF'
ANTHROPIC_API_KEY=...
EOF
```

Then restart the daemon (or restart your Gateway process) and re-check:

```bash\nOpenKrab models status\nOpenKrab doctor
```

If youâ€™d rather not manage env vars yourself, the onboarding wizard can store
API keys for daemon use: `openkrab onboard`.

See [Help](/help) for details on env inheritance (`env.shellEnv`,
`~/.openkrab/.env`, systemd/launchd).

## Anthropic: setup-token (subscription auth)

For Anthropic, the recommended path is an **API key**. If youâ€™re using a Claude
subscription, the setup-token flow is also supported. Run it on the **gateway host**:

```bash
claude setup-token
```

Then paste it into openkrab:

```bash\nOpenKrab models auth setup-token --provider anthropic
```

If the token was created on another machine, paste it manually:

```bash\nOpenKrab models auth paste-token --provider anthropic
```

If you see an Anthropic error like:

```
This credential is only authorized for use with Claude Code and cannot be used for other API requests.
```

â€¦use an Anthropic API key instead.

Manual token entry (any provider; writes `auth-profiles.json` + updates config):

```bash\nOpenKrab models auth paste-token --provider anthropic\nOpenKrab models auth paste-token --provider openrouter
```

Automation-friendly check (exit `1` when expired/missing, `2` when expiring):

```bash\nOpenKrab models status --check
```

Optional ops scripts (systemd/Termux) are documented here:
[/automation/auth-monitoring](/automation/auth-monitoring)

> `claude setup-token` requires an interactive TTY.

## Checking model auth status

```bash\nOpenKrab models status\nOpenKrab doctor
```

## API key rotation behavior (gateway)

Some providers support retrying a request with alternative keys when an API call
hits a provider rate limit.

- Priority order:
  - `OPENKRAB_LIVE_<PROVIDER>_KEY` (single override)
  - `<PROVIDER>_API_KEYS`
  - `<PROVIDER>_API_KEY`
  - `<PROVIDER>_API_KEY_*`
- Google providers also include `GOOGLE_API_KEY` as an additional fallback.
- The same key list is deduplicated before use.
- OpenKrab retries with the next key only for rate-limit errors (for example
  `429`, `rate_limit`, `quota`, `resource exhausted`).
- Non-rate-limit errors are not retried with alternate keys.
- If all keys fail, the final error from the last attempt is returned.

## Controlling which credential is used

### Per-session (chat command)

Use `/model <alias-or-id>@<profileId>` to pin a specific provider credential for the current session (example profile ids: `anthropic:default`, `anthropic:work`).

Use `/model` (or `/model list`) for a compact picker; use `/model status` for the full view (candidates + next auth profile, plus provider endpoint details when configured).

### Per-agent (CLI override)

Set an explicit auth profile order override for an agent (stored in that agentâ€™s `auth-profiles.json`):

```bash\nOpenKrab models auth order get --provider anthropic\nOpenKrab models auth order set --provider anthropic anthropic:default\nOpenKrab models auth order clear --provider anthropic
```

Use `--agent <id>` to target a specific agent; omit it to use the configured default agent.

## Troubleshooting

### â€œNo credentials foundâ€

If the Anthropic token profile is missing, run `claude setup-token` on the
**gateway host**, then re-check:

```bash\nOpenKrab models status
```

### Token expiring/expired

Run `openkrab models status` to confirm which profile is expiring. If the profile
is missing, rerun `claude setup-token` and paste the token again.

## Requirements

- Claude Max or Pro subscription (for `claude setup-token`)
- Claude Code CLI installed (`claude` command available)


