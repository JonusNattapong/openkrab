---
read_when:
  - æ‰“åŒ… KrabKrab.app
  - è°ƒè¯• macOS Gateway ç½‘å…³ launchd æœåŠ¡
  - ä¸º macOS å®‰è£… Gateway ç½‘å…³ CLI
summary: macOS ä¸Šçš„ Gateway ç½‘å…³è¿è¡Œæ—¶ï¼ˆå¤–éƒ¨ launchd æœåŠ¡ï¼‰
title: macOS ä¸Šçš„ Gateway ç½‘å…³
x-i18n:
  generated_at: "2026-02-03T07:52:30Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 4a3e963d13060b123538005439213e786e76127b370a6c834d85a369e4626fe5
  source_path: platforms/mac/bundled-gateway.md
  workflow: 15
---

# macOS ä¸Šçš„ Gateway ç½‘å…³ï¼ˆå¤–éƒ¨ launchdï¼‰

KrabKrab.app ä¸å†æ†ç»‘ Node/Bun æˆ– Gateway ç½‘å…³è¿è¡Œæ—¶ã€‚macOS åº”ç”¨æœŸæœ›æœ‰ä¸€ä¸ª**å¤–éƒ¨**çš„ `krabkrab` CLI å®‰è£…ï¼Œä¸ä¼šå°† Gateway ç½‘å…³ä½œä¸ºå­è¿›ç¨‹å¯åŠ¨ï¼Œè€Œæ˜¯ç®¡ç†ä¸€ä¸ªæ¯ç”¨æˆ·çš„ launchd æœåŠ¡æ¥ä¿æŒ Gateway ç½‘å…³è¿è¡Œï¼ˆæˆ–è€…å¦‚æžœå·²æœ‰æœ¬åœ° Gateway ç½‘å…³æ­£åœ¨è¿è¡Œï¼Œåˆ™è¿žæŽ¥åˆ°çŽ°æœ‰çš„ï¼‰ã€‚

## å®‰è£… CLIï¼ˆæœ¬åœ°æ¨¡å¼å¿…éœ€ï¼‰

ä½ éœ€è¦åœ¨ Mac ä¸Šå®‰è£… Node 22+ï¼Œç„¶åŽå…¨å±€å®‰è£… `krabkrab`ï¼š

```bash
npm install -g krabkrab@<version>
```

macOS åº”ç”¨çš„**å®‰è£… CLI**æŒ‰é’®é€šè¿‡ npm/pnpm è¿è¡Œç›¸åŒçš„æµç¨‹ï¼ˆä¸æŽ¨èä½¿ç”¨ bun ä½œä¸º Gateway ç½‘å…³è¿è¡Œæ—¶ï¼‰ã€‚

## Launchdï¼ˆGateway ç½‘å…³ä½œä¸º LaunchAgentï¼‰

æ ‡ç­¾ï¼š

- `bot.molt.gateway`ï¼ˆæˆ– `bot.molt.<profile>`ï¼›æ—§ç‰ˆ `com.krabkrab.*` å¯èƒ½ä»ç„¶å­˜åœ¨ï¼‰

Plist ä½ç½®ï¼ˆæ¯ç”¨æˆ·ï¼‰ï¼š

- `~/Library/LaunchAgents/bot.molt.gateway.plist`
  ï¼ˆæˆ– `~/Library/LaunchAgents/bot.molt.<profile>.plist`ï¼‰

ç®¡ç†è€…ï¼š

- macOS åº”ç”¨åœ¨æœ¬åœ°æ¨¡å¼ä¸‹æ‹¥æœ‰ LaunchAgent çš„å®‰è£…/æ›´æ–°æƒé™ã€‚
- CLI ä¹Ÿå¯ä»¥å®‰è£…å®ƒï¼š`krabkrab gateway install`ã€‚

è¡Œä¸ºï¼š

- "KrabKrab Active"å¯ç”¨/ç¦ç”¨ LaunchAgentã€‚
- åº”ç”¨é€€å‡º**ä¸ä¼š**åœæ­¢ Gateway ç½‘å…³ï¼ˆlaunchd ä¿æŒå…¶å­˜æ´»ï¼‰ã€‚
- å¦‚æžœ Gateway ç½‘å…³å·²ç»åœ¨é…ç½®çš„ç«¯å£ä¸Šè¿è¡Œï¼Œåº”ç”¨ä¼šè¿žæŽ¥åˆ°å®ƒè€Œä¸æ˜¯å¯åŠ¨æ–°çš„ã€‚

æ—¥å¿—ï¼š

- launchd stdout/errï¼š`/tmp/krabkrab/krabkrab-gateway.log`

## ç‰ˆæœ¬å…¼å®¹æ€§

macOS åº”ç”¨ä¼šæ£€æŸ¥ Gateway ç½‘å…³ç‰ˆæœ¬ä¸Žå…¶è‡ªèº«ç‰ˆæœ¬æ˜¯å¦åŒ¹é…ã€‚å¦‚æžœä¸å…¼å®¹ï¼Œè¯·æ›´æ–°å…¨å±€ CLI ä»¥åŒ¹é…åº”ç”¨ç‰ˆæœ¬ã€‚

## å†’çƒŸæµ‹è¯•

```bash
krabkrab --version

krabkrab_SKIP_CHANNELS=1 \
krabkrab_SKIP_CANVAS_HOST=1 \
krabkrab gateway --port 18999 --bind loopback
```

ç„¶åŽï¼š

```bash
krabkrab gateway call health --url ws://127.0.0.1:18999 --timeout 3000
```

