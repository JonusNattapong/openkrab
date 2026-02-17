---
title: "Discord Channel"
summary: "How to configure and test the Discord channel for OpenClaw"
read_when:
  - Setting up Discord channel connections
  - Writing channel-specific documentation for users
---

# Discord Channel

This page explains how to configure, test, and troubleshoot the Discord channel used by OpenClaw.

## Overview

OpenClaw supports a Discord channel plugin (`openclaw-discord`) that connects the Gateway to Discord using a bot token. When a real Discord token is not available, the project provides comprehensive mocks for local development.

## Configuration (TOML)

Add a `discord` entry under `channels` in your OpenClaw config. Example:

```toml
[channels.discord]
enabled = true
id = "discord-main"

[channels.discord.config]
token = "${DISCORD_BOT_TOKEN}"
channel_id = 123456789012345678
name = "My Discord Channel"
```

- `token`: Bot token **recommended** to be set via environment variable (`DISCORD_BOT_TOKEN`) or secret manager.
- `channel_id`: Optional numeric channel id to send messages to by default.

If you do not have a real token during development, the Gateway will operate in mock mode (see Testing section).

## Recipient / Channel ID Formats

The Gateway and `openclaw-discord` support several recipient formats. The channel-resolution helpers accept:

- Direct numeric channel ID: `123456789012345678`
- Mention-style: `<#123456789012345678>` or `<@123456789012345678>`
- Configured OpenClaw channel id: `discord-main` (maps to the configured `channel_id`)

The functions `resolve_channel_id()` and `map_channel_id()` are used internally to convert these into numeric Discord channel IDs.

## Testing

Local development uses mock HTTP clients and fixtures:

- Run unit tests: `cargo test -p openclaw-channels --lib`
- Integration tests use `crates/openclaw-channels/discord/src/mocks.rs` and `MockDiscordHttp` to avoid needing a live Discord token.

If you want to test against a real bot:

1. Create a Discord bot and copy the token.
2. Set `DISCORD_BOT_TOKEN` in your environment.
3. Ensure the bot is invited to the target server with appropriate permissions.
4. Start the Gateway and check the `channels.discord` probe in the Control UI.

## Troubleshooting

- "Discord token missing": Ensure `token` is set in config or `DISCORD_BOT_TOKEN` is exported.
- API drift / compile errors: Align `openclaw-discord` channel types with `openclaw-core` models; run `cargo test` and inspect failing types.
- If you hit rate limits, implement standard Discord rate-limit handling in the HTTP client layer.

## Examples

Send a message to a specific Discord channel using the CLI (example):

```bash
openclaw message send --channel discord --recipient 123456789012345678 --text "Hello from OpenClaw"
```

## Notes

- For CI and local development prefer mocks to avoid flaky network tests.
- Never commit real tokens to the repository; use environment variables or secret providers.
