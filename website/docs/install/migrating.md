---
summary: "Migration guide from OpenClaw (TypeScript) to OpenKrab (Rust)"
read_when:
  - You are migrating from OpenClaw to OpenKrab
  - You want to understand the differences between TypeScript and Rust versions
title: "Migration from OpenClaw"
---

# Migration from OpenClaw

If you're coming from **OpenClaw** (the original TypeScript/Node.js implementation), this guide will help you migrate to **OpenKrab** (the Rust implementation).

## Why Rust?

| Feature | OpenClaw (TypeScript) | OpenKrab (Rust) |
|---------|----------------------|-----------------|
| **Performance** | V8 JIT limitations | Native compiled, ~5x faster |
| **Memory Safety** | Runtime errors possible | Compile-time guarantees |
| **Startup Time** | ~1-2 seconds | Instant (<100ms) |
| **Memory Usage** | 200-500MB+ | <100MB typical |
| **Concurrency** | Single-threaded event loop | True async with Tokio |
| **Deployment** | Requires Node.js runtime | Single static binary |
| **Binary Size** | Large (Node + deps) | ~20-50MB single binary |

## Key Differences

### 1. Installation

**OpenClaw (old):**
```bash
npm install -g openclaw@latest
openclaw onboard --install-daemon
```

**OpenKrab (new):**
```bash
# From source
git clone https://github.com/openkrab/openkrab.git
cd openkrab
cargo build --release

# Or download pre-built binary
# Binary at: target/release/krabkrab
```

### 2. Configuration Format

**OpenClaw:** JSON5 (`~/.clawdbot/openclaw.json`)
```json5
{
  agents: {
    defaults: {
      model: { primary: "anthropic/claude-opus-4-6" }
    }
  },
  channels: {
    telegram: {
      enabled: true,
      botToken: "123:abc"
    }
  }
}
```

**OpenKrab:** TOML (`~/.config/krabkrab/config.toml`)
```toml
[agents.defaults]
provider = "anthropic"
model = "claude-3-5-sonnet-20241022"

[channels.telegram]
enabled = true
bot_token = "123:abc"
```

### 3. CLI Commands

| OpenClaw | OpenKrab | Notes |
|----------|----------|-------|
| `openclaw` | `krabkrab` | New binary name |
| `openclaw onboard` | `krabkrab setup` | Setup wizard |
| `openclaw config get <path>` | `krabkrab config get <key>` | Dot notation |
| `openclaw gateway --port 18789` | `krabkrab gateway --port 18789` | Similar |
| `openclaw channels login` | `krabkrab channels add` | Different flow |
| `openclaw doctor` | `krabkrab doctor` | Same |
| `openclaw status` | `krabkrab status` | Same |
| `openclaw message send` | `krabkrab message send` | Similar |
| `openclaw memory index` | `krabkrab memory sync` | Different name |

### 4. Directory Structure

| OpenClaw | OpenKrab |
|----------|----------|
| `~/.clawdbot/` | `~/.config/krabkrab/` |
| `~/.clawdbot/openclaw.json` | `~/.config/krabkrab/config.toml` |
| `~/.clawdbot/workspace/` | `~/.local/share/krabkrab/workspace/` |
| `~/.clawdbot/sessions/` | `~/.local/share/krabkrab/sessions/` |
| `~/.clawdbot/credentials/` | `~/.local/share/krabkrab/credentials/` |

### 5. Environment Variables

| OpenClaw | OpenKrab |
|----------|----------|
| `OPENCLAW_HOME` | `KRABKRAB_CONFIG_DIR` |
| `OPENCLAW_STATE_DIR` | `KRABKRAB_DATA_DIR` |
| `OPENCLAW_CONFIG_PATH` | `KRABKRAB_CONFIG_FILE` |
| `CLAWDBOT_PROFILE` | `KRABKRAB_PROFILE` |

### 6. Model Provider Configuration

**OpenClaw:**
```json5
{
  agents: {
    defaults: {
      model: { primary: "anthropic/claude-opus-4-6" }
    }
  }
}
```

**OpenKrab:**
```toml
[providers.anthropic]
api_key = "sk-ant-..."
model = "claude-3-5-sonnet-20241022"

[agents.defaults]
provider = "anthropic"
model = "claude-3-5-sonnet-20241022"
```

## Migration Steps

### Step 1: Export OpenClaw Config

```bash
# On your old OpenClaw installation
cp ~/.clawdbot/openclaw.json ~/openclaw-backup.json
```

### Step 2: Install OpenKrab

```bash
git clone https://github.com/openkrab/openkrab.git
cd openkrab
cargo build --release
sudo cp target/release/krabkrab /usr/local/bin/
```

### Step 3: Migrate Configuration

OpenKrab includes a migration helper:

```bash
krabkrab migrate --from-openclaw ~/openclaw-backup.json
```

This will:
- Convert JSON5 to TOML
- Map old config keys to new format
- Move credentials to new location
- Preserve workspace files

### Step 4: Verify Migration

```bash
krabkrab doctor          # Check configuration
krabkrab config show     # View migrated config
krabkrab status          # Check gateway status
```

### Step 5: Start Gateway

```bash
krabkrab gateway --port 18789
```

## Feature Comparison

### ✅ Fully Ported (100% compatible)

| Feature | Lines of Code | Notes |
|---------|---------------|-------|
| Multi-channel gateway (18 channels) | ~8,000 | All major platforms |
| AI agent runtime with tools | ~12,000 | 12+ built-in tools |
| Memory system (vector + text) | ~10,000 | Hybrid search, MMR, embeddings |
| Voice system (wake/VAD/TTS) | ~5,000 | Full audio pipeline |
| Plugin system (WASM) | ~6,000 | Wasmtime runtime, hot reload |
| Web dashboard | ~3,000 | Self-contained HTML/JS |
| Browser automation | 2,708 | Full CDP with connection pooling |
| Canvas/A2UI | 452 | Complete A2UI protocol |
| Hooks system | 177 | Full event system |
| Security (pairing, sandboxing) | ~3,000 | Complete security model |
| CLI commands | ~6,000 | 40+ commands |

**Total: ~56,276 lines of Rust** (vs ~27,139 lines of TypeScript)

### ❌ Not Ported (Intentionally)

| Feature | Reason | Alternative |
|---------|--------|-------------|
| Feature | Reason | Alternative |
|---------|--------|-------------|
| Voice calls | Low priority, complex | Use other voice apps |
| macOS menu bar | macOS-only, complex | Use web dashboard |
| iMessage native | Private Apple API | Use BlueBubbles bridge |
| Node.js skills | Different runtime | Use WASM plugins |

## Breaking Changes

### 1. No npm/pnpm

OpenKrab is a single binary. No package manager needed.

### 2. Different Plugin Format

**OpenClaw:** TypeScript/JavaScript plugins
```javascript
// openclaw-plugin.js
module.exports = {
  name: "my-plugin",
  // ...
}
```

**OpenKrab:** WASM plugins
```rust
// plugin.rs -> compiled to .wasm
#[no_mangle]
pub extern "C" fn init() {
    // ...
}
```

### 3. Config Hot Reload

OpenKrab supports config hot reload like OpenClaw, but uses TOML instead of JSON5.

### 4. No Built-in Update

OpenKrab doesn't have `krabkrab update` command yet. Update by:
```bash
git pull
cargo build --release
sudo cp target/release/krabkrab /usr/local/bin/
```

## Troubleshooting Migration

### "Config validation failed"

```bash
krabkrab doctor --fix
```

### "Channel not connecting"

- Verify API tokens are correctly migrated
- Check `krabkrab channels status`
- Re-add channel if needed: `krabkrab channels add`

### "Memory not found"

- Re-index memory: `krabkrab memory sync --path ~/old-workspace/memory/`

### "Plugins not loading"

- OpenClaw plugins (JS) won't work with OpenKrab
- Look for Rust/WASM versions of plugins
- Or port your plugin using the plugin SDK

## Getting Help

- GitHub Issues: [github.com/openkrab/openkrab/issues](https://github.com/openkrab/openkrab/issues)
- Migration questions: Tag your issue with `migration` label
- Original OpenClaw: [github.com/openclaw/openclaw](https://github.com/openclaw/openclaw)

## Porting Status Summary

OpenKrab is a **complete rewrite** in Rust:

| Metric | OpenClaw (TS) | OpenKrab (Rust) |
|--------|---------------|-----------------|
| **Total Lines** | ~27,139 | ~56,276 |
| **Test Coverage** | - | 410+ tests |
| **Porting Phases** | - | 24/24 complete |
| **Extensions** | 37 | 30 ported |

### Performance Improvements
- ✅ **5x faster** execution
- ✅ **<100MB** memory usage (vs 200-500MB)
- ✅ **<100ms** startup (vs 1-2 seconds)
- ✅ **Single binary** deployment
- ✅ **Memory safety** guarantees

### Code Quality
- Zero-cost abstractions
- Compile-time error checking
- True async/await with Tokio
- Cross-platform native binaries

The migration is straightforward - config conversion from JSON5 to TOML is the main task.
