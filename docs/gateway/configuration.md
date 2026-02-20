---
summary: "Configuration overview: common tasks, quick setup, and links to the full reference"
read_when:
  - Setting up OpenKrab for the first time
  - Looking for common configuration patterns
  - Navigating to specific config sections
title: "Configuration"
---

# Configuration

OpenKrab reads an optional **TOML** config from `~/.config/krabkrab/config.toml`.

If the file is missing, OpenKrab uses safe defaults. Common reasons to add a config:

- Connect channels and control who can message the bot
- Set models, tools, sandboxing, or automation
- Tune sessions, media, networking, or UI

See the [full reference](/gateway/configuration-reference) for every available field.

<Tip>
**New to configuration?** Check out the [Configuration Examples](/gateway/configuration-examples) guide for complete copy-paste configs.
</Tip>

## Minimal config

```toml
# ~/.config/krabkrab/config.toml
[channels.telegram]
enabled = true
bot_token = "123:abc"
allow_from = ["@username"]

[providers.openai]
api_key = "sk-..."
model = "gpt-4o"
```

## Editing config

<Tabs>
  <Tab title="CLI (one-liners)">
    ```bash
    krabkrab config get providers.openai.api_key
    krabkrab config set providers.openai.model "gpt-4o"
    krabkrab config unset providers.openai.api_key
    ```
  </Tab>
  <Tab title="Direct edit">
    Edit `~/.config/krabkrab/config.toml` directly. The Gateway watches the file and applies changes automatically (see [hot reload](#config-hot-reload)).
  </Tab>
</Tabs>

## Strict validation

<Warning>
OpenKrab only accepts configurations that fully match the schema. Unknown keys, malformed types, or invalid values cause the Gateway to **refuse to start**.
</Warning>

When validation fails:

- The Gateway does not boot
- Only diagnostic commands work (`krabkrab doctor`, `krabkrab status`)
- Run `krabkrab doctor` to see exact issues

## Common tasks

<AccordionGroup>
  <Accordion title="Set up a channel (Telegram, Discord, Slack, etc.)">
    Each channel has its own config section under `channels.<provider>`. See the dedicated channel page for setup steps:

    - [Telegram](/channels/telegram) — `channels.telegram`
    - [Discord](/channels/discord) — `channels.discord`
    - [Slack](/channels/slack) — `channels.slack`
    - [WhatsApp](/channels/whatsapp) — `channels.whatsapp`
    - [Signal](/channels/signal) — `channels.signal`
    - [iMessage](/channels/imessage) — `channels.imessage`

    All channels share the same DM policy pattern:

    ```toml
    [channels.telegram]
    enabled = true
    bot_token = "123:abc"
    dm_policy = "pairing"   # pairing | allowlist | open | disabled
    allow_from = ["tg:123"] # only for allowlist/open
    ```

  </Accordion>

  <Accordion title="Set up AI providers">
    ```toml
    [providers.openai]
    api_key = "sk-..."
    model = "gpt-4o"

    [providers.anthropic]
    api_key = "sk-ant-..."
    model = "claude-3-5-sonnet-20241022"

    [providers.gemini]
    api_key = "..."
    model = "gemini-1.5-pro"
    ```

    See [AI Providers](/providers) for all supported providers.
  </Accordion>

  <Accordion title="Enable memory system">
    ```toml
    [memory]
    enabled = true
    embedding_provider = "openai"
    vector_backend = "qdrant"  # or "memory" for in-memory
    ```

    See [Memory](/concepts/memory) for details.
  </Accordion>
</AccordionGroup>

## Config hot reload

The Gateway watches `config.toml` and reloads most settings without a restart. Changes take effect on the next session start.

## Environment variables

Override config values with environment variables:

```bash
KRABKRAB_PROVIDERS__OPENAI__API_KEY="sk-..." krabkrab gateway
```

See [Environment vars](/help/environment) for full details.
