---
title: Pi å¼€å‘å·¥ä½œæµç¨‹
x-i18n:
  generated_at: "2026-02-03T10:07:59Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 65bd0580dd03df05321ced35a036ce6fb815ce3ddac1d35c9976279adcbf87c0
  source_path: pi-dev.md
  workflow: 15
---

# Pi å¼€å‘å·¥ä½œæµç¨‹

æœ¬æŒ‡å—æ€»ç»“äº†åœ¨ KrabKrab ä¸­å¼€å‘ Pi é›†æˆçš„åˆç†å·¥ä½œæµç¨‹ã€‚

## ç±»åž‹æ£€æŸ¥å’Œä»£ç æ£€æŸ¥

- ç±»åž‹æ£€æŸ¥å’Œæž„å»ºï¼š`pnpm build`
- ä»£ç æ£€æŸ¥ï¼š`pnpm lint`
- æ ¼å¼æ£€æŸ¥ï¼š`pnpm format`
- æŽ¨é€å‰å®Œæ•´æ£€æŸ¥ï¼š`pnpm lint && pnpm build && pnpm test`

## è¿è¡Œ Pi æµ‹è¯•

ä½¿ç”¨ä¸“ç”¨è„šæœ¬è¿è¡Œ Pi é›†æˆæµ‹è¯•é›†ï¼š

```bash
scripts/pi/run-tests.sh
```

è¦åŒ…å«æ‰§è¡ŒçœŸå®žæä¾›å•†è¡Œä¸ºçš„å®žæ—¶æµ‹è¯•ï¼š

```bash
scripts/pi/run-tests.sh --live
```

è¯¥è„šæœ¬é€šè¿‡ä»¥ä¸‹ glob æ¨¡å¼è¿è¡Œæ‰€æœ‰ Pi ç›¸å…³çš„å•å…ƒæµ‹è¯•ï¼š

- `src/agents/pi-*.test.ts`
- `src/agents/pi-embedded-*.test.ts`
- `src/agents/pi-tools*.test.ts`
- `src/agents/pi-settings.test.ts`
- `src/agents/pi-tool-definition-adapter.test.ts`
- `src/agents/pi-extensions/*.test.ts`

## æ‰‹åŠ¨æµ‹è¯•

æŽ¨èæµç¨‹ï¼š

- ä»¥å¼€å‘æ¨¡å¼è¿è¡Œ Gateway ç½‘å…³ï¼š
  - `pnpm gateway:dev`
- ç›´æŽ¥è§¦å‘æ™ºèƒ½ä½“ï¼š
  - `pnpm krabkrab agent --message "Hello" --thinking low`
- ä½¿ç”¨ TUI è¿›è¡Œäº¤äº’å¼è°ƒè¯•ï¼š
  - `pnpm tui`

å¯¹äºŽå·¥å…·è°ƒç”¨è¡Œä¸ºï¼Œæç¤ºæ‰§è¡Œ `read` æˆ– `exec` æ“ä½œï¼Œä»¥ä¾¿æŸ¥çœ‹å·¥å…·æµå¼ä¼ è¾“å’Œè´Ÿè½½å¤„ç†ã€‚

## å®Œå…¨é‡ç½®

çŠ¶æ€å­˜å‚¨åœ¨ KrabKrab çŠ¶æ€ç›®å½•ä¸‹ã€‚é»˜è®¤ä¸º `~/.krabkrab`ã€‚å¦‚æžœè®¾ç½®äº† `krabkrab_STATE_DIR`ï¼Œåˆ™ä½¿ç”¨è¯¥ç›®å½•ã€‚

è¦é‡ç½®æ‰€æœ‰å†…å®¹ï¼š

- `krabkrab.json` ç”¨äºŽé…ç½®
- `credentials/` ç”¨äºŽè®¤è¯é…ç½®æ–‡ä»¶å’Œ token
- `agents/<agentId>/sessions/` ç”¨äºŽæ™ºèƒ½ä½“ä¼šè¯åŽ†å²
- `agents/<agentId>/sessions.json` ç”¨äºŽä¼šè¯ç´¢å¼•
- `sessions/` å¦‚æžœå­˜åœ¨æ—§ç‰ˆè·¯å¾„
- `workspace/` å¦‚æžœä½ æƒ³è¦ä¸€ä¸ªç©ºç™½å·¥ä½œåŒº

å¦‚æžœåªæƒ³é‡ç½®ä¼šè¯ï¼Œåˆ é™¤è¯¥æ™ºèƒ½ä½“çš„ `agents/<agentId>/sessions/` å’Œ `agents/<agentId>/sessions.json`ã€‚å¦‚æžœä¸æƒ³é‡æ–°è®¤è¯ï¼Œä¿ç•™ `credentials/`ã€‚

## å‚è€ƒèµ„æ–™

- https://docs.krabkrab.ai/testing
- https://docs.krabkrab.ai/start/getting-started

