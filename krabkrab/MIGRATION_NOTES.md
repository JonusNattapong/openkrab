# Migration Notes — openclaw → krabkrab (Rust)

This guide helps users migrate from the TypeScript version (`openclaw`) to the Rust version (`krabkrab`).

## Quick Reference

| Aspect | openclaw (TypeScript) | krabkrab (Rust) |
|--------|----------------------|-----------------|
| Runtime | Node.js 22+ | Native binary |
| Config format | JSON | TOML |
| Config location | `~/.clawdbot/` | `~/.config/krabkrab/` |
| Credentials | `~/.clawdbot/credentials/` | `~/.config/krabkrab/credentials/` |
| Package manager | npm/pnpm/bun | cargo |

## Before You Start

1. **Backup your config**: Copy `~/.clawdbot/` to a safe location
2. **Note your tokens**: You'll need the same bot tokens and API keys
3. **Check connector parity**: Most connectors are fully supported

## Installation

### openclaw (TypeScript)
```bash
npm install -g krabkrab@latest
```

### krabkrab (Rust)
```bash
cargo install krabkrab
# or build from source
git clone https://github.com/openkrab/krabkrab.git
cd krabkrab
cargo build --release
```

## Configuration Migration

### Step 1: Convert JSON to TOML

**Before (openclaw - JSON):**
```json
{
  "agent": {
    "model": "anthropic/claude-opus-4"
  },
  "channels": {
    "telegram": {
      "botToken": "123456:ABCDEF"
    },
    "discord": {
      "token": "BotTokenHere",
      "dmPolicy": "pairing"
    }
  }
}
```

**After (krabkrab - TOML):**
```toml
[agent]
model = "anthropic/claude-opus-4"

[channels.telegram]
bot_token = "123456:ABCDEF"

[channels.discord]
token = "BotTokenHere"
dm_policy = "pairing"
```

### Step 2: Move Config Files

```bash
# Create new config directory
mkdir -p ~/.config/krabkrab

# Move and convert config (manual conversion required)
# Old: ~/.clawdbot/moltbot.json
# New: ~/.config/krabkrab/krabkrab.toml

# Move credentials
cp -r ~/.clawdbot/credentials ~/.config/krabkrab/
```

### Key Naming Convention

| JSON (camelCase) | TOML (snake_case) |
|------------------|-------------------|
| `botToken` | `bot_token` |
| `dmPolicy` | `dm_policy` |
| `allowFrom` | `allow_from` |
| `maxLines` | `max_lines` |
| `apiBase` | `api_base` |

## CLI Command Mapping

### Gateway Commands

| openclaw | krabkrab |
|----------|----------|
| `moltbot gateway run` | `krabkrab gateway start` |
| `moltbot gateway --port 18789` | `krabkrab gateway start --port 3000` |

### Status & Configuration

| openclaw | krabkrab |
|----------|----------|
| `moltbot status` | `krabkrab status` |
| `moltbot doctor` | `krabkrab doctor` |
| `moltbot configure` | `krabkrab configure` |
| `moltbot onboard` | `krabkrab configure` (interactive) |

### Messaging Commands

| openclaw | krabkrab |
|----------|----------|
| `moltbot telegram send --to 123 --text "hi"` | `krabkrab telegram --text "hi"` |
| `moltbot slack send --text "hi"` | `krabkrab slack --text "hi"` |
| `moltbot discord send --to 123 --text "hi"` | `krabkrab discord --to 123 --text "hi"` |
| (N/A) | `krabkrab discord --to 123 --text "hi" --dry-run` |

### Memory Commands

| openclaw | krabkrab |
|----------|----------|
| `moltbot memory sync` | `krabkrab memory sync --path ./docs` |
| `moltbot memory search "query"` | `krabkrab memory search "query"` |

### New Commands (krabkrab only)

| Command | Description |
|---------|-------------|
| `krabkrab ask "question"` | Direct LLM query |
| `krabkrab models --provider openai` | List available models |

## Connector Migration

### Telegram

**No changes required** - Same bot token works.

```toml
[channels.telegram]
enabled = true
bot_token = "YOUR_BOT_TOKEN"
```

### Slack

**No changes required** - Same tokens work.

```toml
[channels.slack]
enabled = true
bot_token = "xoxb-..."
app_token = "xapp-..."
```

### Discord

**Enhanced** - More features available.

```toml
[channels.discord]
enabled = true
token = "Bot ..."
dm_policy = "pairing"  # was dmPolicy

[channels.discord.actions]
reactions = true
polls = true
moderation = false  # New: control moderation actions
```

### BlueBubbles (iMessage)

**Same configuration** - Works identically.

```toml
[channels.bluebubbles]
enabled = true
server_url = "http://..."
password = "..."
```

### Signal

**Same configuration** - Requires `signal-cli`.

```toml
[channels.signal]
enabled = true
phone_number = "+..."
```

## Provider Migration

### OpenAI

```toml
[providers.openai]
api_key = "sk-..."
model = "gpt-4"
```

### Anthropic

```toml
[providers.anthropic]
api_key = "sk-ant-..."
model = "claude-opus-4"
```

### Gemini

```toml
[providers.gemini]
api_key = "..."
model = "gemini-pro"
# Or use CLI credentials (auto-detected)
```

### Ollama

```toml
[providers.ollama]
base_url = "http://localhost:11434"
model = "llama2"
```

## Environment Variables

Most environment variables work the same:

| Variable | Status |
|----------|--------|
| `TELEGRAM_BOT_TOKEN` | ✅ Same |
| `SLACK_BOT_TOKEN` | ✅ Same |
| `SLACK_APP_TOKEN` | ✅ Same |
| `DISCORD_BOT_TOKEN` | ✅ Same |
| `OPENAI_API_KEY` | ✅ Same |
| `ANTHROPIC_API_KEY` | ✅ Same |

## Breaking Changes

### 1. Config File Format
- **Before**: JSON
- **After**: TOML
- **Migration**: Manual conversion required

### 2. Config Directory
- **Before**: `~/.clawdbot/`
- **After**: `~/.config/krabkrab/`
- **Migration**: Move and convert files

### 3. CLI Syntax
- Discord now requires `--to` parameter explicitly
- Some commands have different flags

### 4. DM Policy
- Same behavior, but config key changed from `dmPolicy` to `dm_policy`

## New Features

### Action Gates (Discord)

Control which actions are allowed per account:

```toml
[channels.discord.actions]
reactions = true
polls = true
threads = true
moderation = false  # Disable kick/ban/timeout
presence = false    # Disable status changes
```

### Direct LLM Query

```bash
krabkrab ask "Explain Rust ownership"
```

### Model Listing

```bash
krabkrab models --provider openai
krabkrab models --provider ollama
```

## Troubleshooting

### Config Not Found

```bash
# Check config location
ls ~/.config/krabkrab/

# Create if missing
mkdir -p ~/.config/krabkrab
```

### Token Not Working

1. Verify the token is correct
2. Check environment variables override config
3. Check file permissions

### Tests Failing

```bash
# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name --lib
```

## Rollback Plan

If you encounter issues:

1. **Keep your backup**: Don't delete `~/.clawdbot/` backup
2. **Use both versions**: They can coexist
3. **Report issues**: Open an issue with logs and config (no secrets)

## Getting Help

- **Documentation**: See README.md, PORTING.md
- **Issues**: https://github.com/openkrab/krabkrab/issues
- **Source**: https://github.com/openkrab/krabkrab

## Summary Checklist

- [ ] Backup `~/.clawdbot/`
- [ ] Install krabkrab (cargo or build from source)
- [ ] Create `~/.config/krabkrab/`
- [ ] Convert config from JSON to TOML
- [ ] Copy credentials
- [ ] Test with `krabkrab status`
- [ ] Test connector with `krabkrab telegram --text "test"`
- [ ] Update any scripts using old CLI syntax
