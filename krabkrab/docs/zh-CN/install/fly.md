---
description: Deploy KrabKrab on Fly.io
title: Fly.io
x-i18n:
  generated_at: "2026-02-03T07:52:55Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: a00bae43e416112eb269126445c51492a30abe9e136d89e161fc4193314a876f
  source_path: platforms/fly.md
  workflow: 15
---

# Fly.io éƒ¨ç½²

**ç›®æ ‡ï¼š** KrabKrab Gateway ç½‘å…³è¿è¡Œåœ¨ [Fly.io](https://fly.io) æœºå™¨ä¸Šï¼Œå…·æœ‰æŒä¹…å­˜å‚¨ã€è‡ªåŠ¨ HTTPS å’Œ Discord/æ¸ é“è®¿é—®ã€‚

## ä½ éœ€è¦ä»€ä¹ˆ

- å·²å®‰è£… [flyctl CLI](https://fly.io/docs/hands-on/install-flyctl/)
- Fly.io è´¦æˆ·ï¼ˆå…è´¹å¥—é¤å¯ç”¨ï¼‰
- æ¨¡åž‹è®¤è¯ï¼šAnthropic API å¯†é’¥ï¼ˆæˆ–å…¶ä»–æä¾›å•†å¯†é’¥ï¼‰
- æ¸ é“å‡­è¯ï¼šDiscord bot tokenã€Telegram token ç­‰

## åˆå­¦è€…å¿«é€Ÿè·¯å¾„

1. å…‹éš†ä»“åº“ â†’ è‡ªå®šä¹‰ `fly.toml`
2. åˆ›å»ºåº”ç”¨ + å· â†’ è®¾ç½®å¯†é’¥
3. ä½¿ç”¨ `fly deploy` éƒ¨ç½²
4. SSH è¿›å…¥åˆ›å»ºé…ç½®æˆ–ä½¿ç”¨ Control UI

## 1ï¼‰åˆ›å»º Fly åº”ç”¨

```bash
# Clone the repo
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab

# Create a new Fly app (pick your own name)
fly apps create my-krabkrab

# Create a persistent volume (1GB is usually enough)
fly volumes create krabkrab_data --size 1 --region iad
```

**æç¤ºï¼š** é€‰æ‹©ç¦»ä½ è¿‘çš„åŒºåŸŸã€‚å¸¸è§é€‰é¡¹ï¼š`lhr`ï¼ˆä¼¦æ•¦ï¼‰ã€`iad`ï¼ˆå¼—å‰å°¼äºšï¼‰ã€`sjc`ï¼ˆåœ£ä½•å¡žï¼‰ã€‚

## 2ï¼‰é…ç½® fly.toml

ç¼–è¾‘ `fly.toml` ä»¥åŒ¹é…ä½ çš„åº”ç”¨åç§°å’Œéœ€æ±‚ã€‚

**å®‰å…¨æ³¨æ„äº‹é¡¹ï¼š** é»˜è®¤é…ç½®æš´éœ²å…¬å…± URLã€‚å¯¹äºŽæ²¡æœ‰å…¬å…± IP çš„åŠ å›ºéƒ¨ç½²ï¼Œå‚è§[ç§æœ‰éƒ¨ç½²](#ç§æœ‰éƒ¨ç½²åŠ å›º)æˆ–ä½¿ç”¨ `fly.private.toml`ã€‚

```toml
app = "my-krabkrab"  # Your app name
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

[env]
  NODE_ENV = "production"
  krabkrab_PREFER_PNPM = "1"
  krabkrab_STATE_DIR = "/data"
  NODE_OPTIONS = "--max-old-space-size=1536"

[processes]
  app = "node dist/index.js gateway --allow-unconfigured --port 3000 --bind lan"

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = false
  auto_start_machines = true
  min_machines_running = 1
  processes = ["app"]

[[vm]]
  size = "shared-cpu-2x"
  memory = "2048mb"

[mounts]
  source = "krabkrab_data"
  destination = "/data"
```

**å…³é”®è®¾ç½®ï¼š**

| è®¾ç½®                           | åŽŸå›                                                                       |
| ------------------------------ | ------------------------------------------------------------------------- |
| `--bind lan`                   | ç»‘å®šåˆ° `0.0.0.0` ä»¥ä¾¿ Fly çš„ä»£ç†å¯ä»¥è®¿é—® Gateway ç½‘å…³                     |
| `--allow-unconfigured`         | æ— éœ€é…ç½®æ–‡ä»¶å¯åŠ¨ï¼ˆä½ ç¨åŽä¼šåˆ›å»ºä¸€ä¸ªï¼‰                                      |
| `internal_port = 3000`         | å¿…é¡»ä¸Ž `--port 3000`ï¼ˆæˆ– `krabkrab_GATEWAY_PORT`ï¼‰åŒ¹é…ä»¥è¿›è¡Œ Fly å¥åº·æ£€æŸ¥ |
| `memory = "2048mb"`            | 512MB å¤ªå°ï¼›æŽ¨è 2GB                                                      |
| `krabkrab_STATE_DIR = "/data"` | åœ¨å·ä¸ŠæŒä¹…åŒ–çŠ¶æ€                                                          |

## 3ï¼‰è®¾ç½®å¯†é’¥

```bash
# Required: Gateway token (for non-loopback binding)
fly secrets set krabkrab_GATEWAY_TOKEN=$(openssl rand -hex 32)

# Model provider API keys
fly secrets set ANTHROPIC_API_KEY=sk-ant-...

# Optional: Other providers
fly secrets set OPENAI_API_KEY=sk-...
fly secrets set GOOGLE_API_KEY=...

# Channel tokens
fly secrets set DISCORD_BOT_TOKEN=MTQ...
```

**æ³¨æ„äº‹é¡¹ï¼š**

- éž loopback ç»‘å®šï¼ˆ`--bind lan`ï¼‰å‡ºäºŽå®‰å…¨éœ€è¦ `krabkrab_GATEWAY_TOKEN`ã€‚
- åƒå¯¹å¾…å¯†ç ä¸€æ ·å¯¹å¾…è¿™äº› tokenã€‚
- **ä¼˜å…ˆä½¿ç”¨çŽ¯å¢ƒå˜é‡è€Œä¸æ˜¯é…ç½®æ–‡ä»¶**æ¥å­˜å‚¨æ‰€æœ‰ API å¯†é’¥å’Œ tokenã€‚è¿™å¯ä»¥é¿å…å¯†é’¥å‡ºçŽ°åœ¨ `krabkrab.json` ä¸­ï¼Œé˜²æ­¢æ„å¤–æš´éœ²æˆ–è®°å½•ã€‚

## 4ï¼‰éƒ¨ç½²

```bash
fly deploy
```

é¦–æ¬¡éƒ¨ç½²æž„å»º Docker é•œåƒï¼ˆçº¦ 2-3 åˆ†é’Ÿï¼‰ã€‚åŽç»­éƒ¨ç½²æ›´å¿«ã€‚

éƒ¨ç½²åŽéªŒè¯ï¼š

```bash
fly status
fly logs
```

ä½ åº”è¯¥çœ‹åˆ°ï¼š

```
[gateway] listening on ws://0.0.0.0:3000 (PID xxx)
[discord] logged in to discord as xxx
```

## 5ï¼‰åˆ›å»ºé…ç½®æ–‡ä»¶

SSH è¿›å…¥æœºå™¨åˆ›å»ºæ­£ç¡®çš„é…ç½®ï¼š

```bash
fly ssh console
```

åˆ›å»ºé…ç½®ç›®å½•å’Œæ–‡ä»¶ï¼š

```bash
mkdir -p /data
cat > /data/krabkrab.json << 'EOF'
{
  "agents": {
    "defaults": {
      "model": {
        "primary": "anthropic/claude-opus-4-5",
        "fallbacks": ["anthropic/claude-sonnet-4-5", "openai/gpt-4o"]
      },
      "maxConcurrent": 4
    },
    "list": [
      {
        "id": "main",
        "default": true
      }
    ]
  },
  "auth": {
    "profiles": {
      "anthropic:default": { "mode": "token", "provider": "anthropic" },
      "openai:default": { "mode": "token", "provider": "openai" }
    }
  },
  "bindings": [
    {
      "agentId": "main",
      "match": { "channel": "discord" }
    }
  ],
  "channels": {
    "discord": {
      "enabled": true,
      "groupPolicy": "allowlist",
      "guilds": {
        "YOUR_GUILD_ID": {
          "channels": { "general": { "allow": true } },
          "requireMention": false
        }
      }
    }
  },
  "gateway": {
    "mode": "local",
    "bind": "auto"
  },
  "meta": {
    "lastTouchedVersion": "2026.1.29"
  }
}
EOF
```

**æ³¨æ„ï¼š** ä½¿ç”¨ `krabkrab_STATE_DIR=/data` æ—¶ï¼Œé…ç½®è·¯å¾„æ˜¯ `/data/krabkrab.json`ã€‚

**æ³¨æ„ï¼š** Discord token å¯ä»¥æ¥è‡ªï¼š

- çŽ¯å¢ƒå˜é‡ï¼š`DISCORD_BOT_TOKEN`ï¼ˆæŽ¨èç”¨äºŽå¯†é’¥ï¼‰
- é…ç½®æ–‡ä»¶ï¼š`channels.discord.token`

å¦‚æžœä½¿ç”¨çŽ¯å¢ƒå˜é‡ï¼Œæ— éœ€å°† token æ·»åŠ åˆ°é…ç½®ä¸­ã€‚Gateway ç½‘å…³ä¼šè‡ªåŠ¨è¯»å– `DISCORD_BOT_TOKEN`ã€‚

é‡å¯ä»¥åº”ç”¨ï¼š

```bash
exit
fly machine restart <machine-id>
```

## 6ï¼‰è®¿é—® Gateway ç½‘å…³

### Control UI

åœ¨æµè§ˆå™¨ä¸­æ‰“å¼€ï¼š

```bash
fly open
```

æˆ–è®¿é—® `https://my-krabkrab.fly.dev/`

ç²˜è´´ä½ çš„ Gateway ç½‘å…³ tokenï¼ˆæ¥è‡ª `krabkrab_GATEWAY_TOKEN` çš„é‚£ä¸ªï¼‰è¿›è¡Œè®¤è¯ã€‚

### æ—¥å¿—

```bash
fly logs              # Live logs
fly logs --no-tail    # Recent logs
```

### SSH æŽ§åˆ¶å°

```bash
fly ssh console
```

## æ•…éšœæŽ’é™¤

### "App is not listening on expected address"

Gateway ç½‘å…³ç»‘å®šåˆ° `127.0.0.1` è€Œä¸æ˜¯ `0.0.0.0`ã€‚

**ä¿®å¤ï¼š** åœ¨ `fly.toml` ä¸­çš„è¿›ç¨‹å‘½ä»¤æ·»åŠ  `--bind lan`ã€‚

### å¥åº·æ£€æŸ¥å¤±è´¥ / è¿žæŽ¥è¢«æ‹’ç»

Fly æ— æ³•åœ¨é…ç½®çš„ç«¯å£ä¸Šè®¿é—® Gateway ç½‘å…³ã€‚

**ä¿®å¤ï¼š** ç¡®ä¿ `internal_port` ä¸Ž Gateway ç½‘å…³ç«¯å£åŒ¹é…ï¼ˆè®¾ç½® `--port 3000` æˆ– `krabkrab_GATEWAY_PORT=3000`ï¼‰ã€‚

### OOM / å†…å­˜é—®é¢˜

å®¹å™¨æŒç»­é‡å¯æˆ–è¢«ç»ˆæ­¢ã€‚è¿¹è±¡ï¼š`SIGABRT`ã€`v8::internal::Runtime_AllocateInYoungGeneration` æˆ–é™é»˜é‡å¯ã€‚

**ä¿®å¤ï¼š** åœ¨ `fly.toml` ä¸­å¢žåŠ å†…å­˜ï¼š

```toml
[[vm]]
  memory = "2048mb"
```

æˆ–æ›´æ–°çŽ°æœ‰æœºå™¨ï¼š

```bash
fly machine update <machine-id> --vm-memory 2048 -y
```

**æ³¨æ„ï¼š** 512MB å¤ªå°ã€‚1GB å¯èƒ½å¯ä»¥å·¥ä½œä½†åœ¨è´Ÿè½½æˆ–è¯¦ç»†æ—¥å¿—è®°å½•ä¸‹å¯èƒ½ OOMã€‚**æŽ¨è 2GBã€‚**

### Gateway ç½‘å…³é”é—®é¢˜

Gateway ç½‘å…³æ‹’ç»å¯åŠ¨å¹¶æ˜¾ç¤º"already running"é”™è¯¯ã€‚

è¿™å‘ç”Ÿåœ¨å®¹å™¨é‡å¯ä½† PID é”æ–‡ä»¶åœ¨å·ä¸ŠæŒä¹…å­˜åœ¨æ—¶ã€‚

**ä¿®å¤ï¼š** åˆ é™¤é”æ–‡ä»¶ï¼š

```bash
fly ssh console --command "rm -f /data/gateway.*.lock"
fly machine restart <machine-id>
```

é”æ–‡ä»¶åœ¨ `/data/gateway.*.lock`ï¼ˆä¸åœ¨å­ç›®å½•ä¸­ï¼‰ã€‚

### é…ç½®æœªè¢«è¯»å–

å¦‚æžœä½¿ç”¨ `--allow-unconfigured`ï¼ŒGateway ç½‘å…³ä¼šåˆ›å»ºæœ€å°é…ç½®ã€‚ä½ åœ¨ `/data/krabkrab.json` çš„è‡ªå®šä¹‰é…ç½®åº”è¯¥åœ¨é‡å¯æ—¶è¢«è¯»å–ã€‚

éªŒè¯é…ç½®æ˜¯å¦å­˜åœ¨ï¼š

```bash
fly ssh console --command "cat /data/krabkrab.json"
```

### é€šè¿‡ SSH å†™å…¥é…ç½®

`fly ssh console -C` å‘½ä»¤ä¸æ”¯æŒ shell é‡å®šå‘ã€‚è¦å†™å…¥é…ç½®æ–‡ä»¶ï¼š

```bash
# Use echo + tee (pipe from local to remote)
echo '{"your":"config"}' | fly ssh console -C "tee /data/krabkrab.json"

# Or use sftp
fly sftp shell
> put /local/path/config.json /data/krabkrab.json
```

**æ³¨æ„ï¼š** å¦‚æžœæ–‡ä»¶å·²å­˜åœ¨ï¼Œ`fly sftp` å¯èƒ½ä¼šå¤±è´¥ã€‚å…ˆåˆ é™¤ï¼š

```bash
fly ssh console --command "rm /data/krabkrab.json"
```

### çŠ¶æ€æœªæŒä¹…åŒ–

å¦‚æžœé‡å¯åŽä¸¢å¤±å‡­è¯æˆ–ä¼šè¯ï¼ŒçŠ¶æ€ç›®å½•æ­£åœ¨å†™å…¥å®¹å™¨æ–‡ä»¶ç³»ç»Ÿã€‚

**ä¿®å¤ï¼š** ç¡®ä¿ `fly.toml` ä¸­è®¾ç½®äº† `krabkrab_STATE_DIR=/data` å¹¶é‡æ–°éƒ¨ç½²ã€‚

## æ›´æ–°

```bash
# Pull latest changes
git pull

# Redeploy
fly deploy

# Check health
fly status
fly logs
```

### æ›´æ–°æœºå™¨å‘½ä»¤

å¦‚æžœä½ éœ€è¦æ›´æ”¹å¯åŠ¨å‘½ä»¤è€Œæ— éœ€å®Œå…¨é‡æ–°éƒ¨ç½²ï¼š

```bash
# Get machine ID
fly machines list

# Update command
fly machine update <machine-id> --command "node dist/index.js gateway --port 3000 --bind lan" -y

# Or with memory increase
fly machine update <machine-id> --vm-memory 2048 --command "node dist/index.js gateway --port 3000 --bind lan" -y
```

**æ³¨æ„ï¼š** `fly deploy` åŽï¼Œæœºå™¨å‘½ä»¤å¯èƒ½ä¼šé‡ç½®ä¸º `fly.toml` ä¸­çš„å†…å®¹ã€‚å¦‚æžœä½ è¿›è¡Œäº†æ‰‹åŠ¨æ›´æ”¹ï¼Œè¯·åœ¨éƒ¨ç½²åŽé‡æ–°åº”ç”¨å®ƒä»¬ã€‚

## ç§æœ‰éƒ¨ç½²ï¼ˆåŠ å›ºï¼‰

é»˜è®¤æƒ…å†µä¸‹ï¼ŒFly åˆ†é…å…¬å…± IPï¼Œä½¿ä½ çš„ Gateway ç½‘å…³å¯é€šè¿‡ `https://your-app.fly.dev` è®¿é—®ã€‚è¿™å¾ˆæ–¹ä¾¿ï¼Œä½†æ„å‘³ç€ä½ çš„éƒ¨ç½²å¯è¢«äº’è”ç½‘æ‰«æå™¨ï¼ˆShodanã€Censys ç­‰ï¼‰å‘çŽ°ã€‚

å¯¹äºŽ**æ— å…¬å…±æš´éœ²**çš„åŠ å›ºéƒ¨ç½²ï¼Œä½¿ç”¨ç§æœ‰æ¨¡æ¿ã€‚

### ä½•æ—¶ä½¿ç”¨ç§æœ‰éƒ¨ç½²

- ä½ åªè¿›è¡Œ**å‡ºç«™**è°ƒç”¨/æ¶ˆæ¯ï¼ˆæ— å…¥ç«™ webhooksï¼‰
- ä½ ä½¿ç”¨ **ngrok æˆ– Tailscale** éš§é“å¤„ç†ä»»ä½• webhook å›žè°ƒ
- ä½ é€šè¿‡ **SSHã€ä»£ç†æˆ– WireGuard** è€Œä¸æ˜¯æµè§ˆå™¨è®¿é—® Gateway ç½‘å…³
- ä½ å¸Œæœ›éƒ¨ç½²**å¯¹äº’è”ç½‘æ‰«æå™¨éšè—**

### è®¾ç½®

ä½¿ç”¨ `fly.private.toml` æ›¿ä»£æ ‡å‡†é…ç½®ï¼š

```bash
# Deploy with private config
fly deploy -c fly.private.toml
```

æˆ–è½¬æ¢çŽ°æœ‰éƒ¨ç½²ï¼š

```bash
# List current IPs
fly ips list -a my-krabkrab

# Release public IPs
fly ips release <public-ipv4> -a my-krabkrab
fly ips release <public-ipv6> -a my-krabkrab

# Switch to private config so future deploys don't re-allocate public IPs
# (remove [http_service] or deploy with the private template)
fly deploy -c fly.private.toml

# Allocate private-only IPv6
fly ips allocate-v6 --private -a my-krabkrab
```

æ­¤åŽï¼Œ`fly ips list` åº”è¯¥åªæ˜¾ç¤º `private` ç±»åž‹çš„ IPï¼š

```
VERSION  IP                   TYPE             REGION
v6       fdaa:x:x:x:x::x      private          global
```

### è®¿é—®ç§æœ‰éƒ¨ç½²

ç”±äºŽæ²¡æœ‰å…¬å…± URLï¼Œä½¿ç”¨ä»¥ä¸‹æ–¹æ³•ä¹‹ä¸€ï¼š

**é€‰é¡¹ 1ï¼šæœ¬åœ°ä»£ç†ï¼ˆæœ€ç®€å•ï¼‰**

```bash
# Forward local port 3000 to the app
fly proxy 3000:3000 -a my-krabkrab

# Then open http://localhost:3000 in browser
```

**é€‰é¡¹ 2ï¼šWireGuard VPN**

```bash
# Create WireGuard config (one-time)
fly wireguard create

# Import to WireGuard client, then access via internal IPv6
# Example: http://[fdaa:x:x:x:x::x]:3000
```

**é€‰é¡¹ 3ï¼šä»… SSH**

```bash
fly ssh console -a my-krabkrab
```

### ç§æœ‰éƒ¨ç½²çš„ Webhooks

å¦‚æžœä½ éœ€è¦ webhook å›žè°ƒï¼ˆTwilioã€Telnyx ç­‰ï¼‰è€Œä¸æš´éœ²å…¬å…±ï¼š

1. **ngrok éš§é“** - åœ¨å®¹å™¨å†…æˆ–ä½œä¸º sidecar è¿è¡Œ ngrok
2. **Tailscale Funnel** - é€šè¿‡ Tailscale æš´éœ²ç‰¹å®šè·¯å¾„
3. **ä»…å‡ºç«™** - æŸäº›æä¾›å•†ï¼ˆTwilioï¼‰å¯¹äºŽå‡ºç«™å‘¼å«æ— éœ€ webhooks ä¹Ÿèƒ½æ­£å¸¸å·¥ä½œ

ä½¿ç”¨ ngrok çš„ç¤ºä¾‹è¯­éŸ³é€šè¯é…ç½®ï¼š

```json
{
  "plugins": {
    "entries": {
      "voice-call": {
        "enabled": true,
        "config": {
          "provider": "twilio",
          "tunnel": { "provider": "ngrok" }
        }
      }
    }
  }
}
```

ngrok éš§é“åœ¨å®¹å™¨å†…è¿è¡Œå¹¶æä¾›å…¬å…± webhook URLï¼Œè€Œä¸æš´éœ² Fly åº”ç”¨æœ¬èº«ã€‚

### å®‰å…¨ä¼˜åŠ¿

| æ–¹é¢            | å…¬å…±   | ç§æœ‰     |
| --------------- | ------ | -------- |
| äº’è”ç½‘æ‰«æå™¨    | å¯å‘çŽ° | éšè—     |
| ç›´æŽ¥æ”»å‡»        | å¯èƒ½   | è¢«é˜»æ­¢   |
| Control UI è®¿é—® | æµè§ˆå™¨ | ä»£ç†/VPN |
| Webhook æŠ•é€’    | ç›´æŽ¥   | é€šè¿‡éš§é“ |

## æ³¨æ„äº‹é¡¹

- Fly.io ä½¿ç”¨ **x86 æž¶æž„**ï¼ˆéž ARMï¼‰
- Dockerfile å…¼å®¹ä¸¤ç§æž¶æž„
- å¯¹äºŽ WhatsApp/Telegram æ–°æ‰‹å¼•å¯¼ï¼Œä½¿ç”¨ `fly ssh console`
- æŒä¹…æ•°æ®ä½äºŽ `/data` å·ä¸Š
- Signal éœ€è¦ Java + signal-cliï¼›ä½¿ç”¨è‡ªå®šä¹‰é•œåƒå¹¶ä¿æŒå†…å­˜åœ¨ 2GB+ã€‚

## æˆæœ¬

ä½¿ç”¨æŽ¨èé…ç½®ï¼ˆ`shared-cpu-2x`ï¼Œ2GB RAMï¼‰ï¼š

- æ ¹æ®ä½¿ç”¨æƒ…å†µçº¦ $10-15/æœˆ
- å…è´¹å¥—é¤åŒ…å«ä¸€äº›é…é¢

è¯¦æƒ…å‚è§ [Fly.io å®šä»·](https://fly.io/docs/about/pricing/)ã€‚

