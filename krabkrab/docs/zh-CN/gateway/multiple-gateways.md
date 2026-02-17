---
read_when:
  - åœ¨åŒä¸€å°æœºå™¨ä¸Šè¿è¡Œå¤šä¸ª Gateway ç½‘å…³
  - ä½ éœ€è¦æ¯ä¸ª Gateway ç½‘å…³æœ‰éš”ç¦»çš„é…ç½®/çŠ¶æ€/ç«¯å£
summary: åœ¨åŒä¸€ä¸»æœºä¸Šè¿è¡Œå¤šä¸ª KrabKrab Gateway ç½‘å…³ï¼ˆéš”ç¦»ã€ç«¯å£å’Œé…ç½®æ–‡ä»¶ï¼‰
title: å¤š Gateway ç½‘å…³
x-i18n:
  generated_at: "2026-02-03T07:48:13Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 09b5035d4e5fb97c8d4596f7e23dea67224dad3b6d9e2c37ecb99840f28bd77d
  source_path: gateway/multiple-gateways.md
  workflow: 15
---

# å¤š Gateway ç½‘å…³ï¼ˆåŒä¸€ä¸»æœºï¼‰

å¤§å¤šæ•°è®¾ç½®åº”è¯¥ä½¿ç”¨å•ä¸ª Gateway ç½‘å…³ï¼Œå› ä¸ºä¸€ä¸ª Gateway ç½‘å…³å¯ä»¥å¤„ç†å¤šä¸ªæ¶ˆæ¯è¿žæŽ¥å’Œæ™ºèƒ½ä½“ã€‚å¦‚æžœä½ éœ€è¦æ›´å¼ºçš„éš”ç¦»æˆ–å†—ä½™ï¼ˆä¾‹å¦‚ï¼Œæ•‘æ´æœºå™¨äººï¼‰ï¼Œè¯·ä½¿ç”¨éš”ç¦»çš„é…ç½®æ–‡ä»¶/ç«¯å£è¿è¡Œå¤šä¸ª Gateway ç½‘å…³ã€‚

## éš”ç¦»æ£€æŸ¥æ¸…å•ï¼ˆå¿…éœ€ï¼‰

- `krabkrab_CONFIG_PATH` â€” æ¯ä¸ªå®žä¾‹çš„é…ç½®æ–‡ä»¶
- `krabkrab_STATE_DIR` â€” æ¯ä¸ªå®žä¾‹çš„ä¼šè¯ã€å‡­è¯ã€ç¼“å­˜
- `agents.defaults.workspace` â€” æ¯ä¸ªå®žä¾‹çš„å·¥ä½œåŒºæ ¹ç›®å½•
- `gateway.port`ï¼ˆæˆ– `--port`ï¼‰â€” æ¯ä¸ªå®žä¾‹å”¯ä¸€
- æ´¾ç”Ÿç«¯å£ï¼ˆæµè§ˆå™¨/ç”»å¸ƒï¼‰ä¸å¾—é‡å 

å¦‚æžœè¿™äº›æ˜¯å…±äº«çš„ï¼Œä½ å°†é‡åˆ°é…ç½®ç«žäº‰å’Œç«¯å£å†²çªã€‚

## æŽ¨èï¼šé…ç½®æ–‡ä»¶ï¼ˆ`--profile`ï¼‰

é…ç½®æ–‡ä»¶è‡ªåŠ¨é™å®š `krabkrab_STATE_DIR` + `krabkrab_CONFIG_PATH` èŒƒå›´å¹¶ä¸ºæœåŠ¡åç§°æ·»åŠ åŽç¼€ã€‚

```bash
# main
krabkrab --profile main setup
krabkrab --profile main gateway --port 18789

# rescue
krabkrab --profile rescue setup
krabkrab --profile rescue gateway --port 19001
```

æŒ‰é…ç½®æ–‡ä»¶çš„æœåŠ¡ï¼š

```bash
krabkrab --profile main gateway install
krabkrab --profile rescue gateway install
```

## æ•‘æ´æœºå™¨äººæŒ‡å—

åœ¨åŒä¸€ä¸»æœºä¸Šè¿è¡Œç¬¬äºŒä¸ª Gateway ç½‘å…³ï¼Œä½¿ç”¨ç‹¬ç«‹çš„ï¼š

- é…ç½®æ–‡ä»¶/é…ç½®
- çŠ¶æ€ç›®å½•
- å·¥ä½œåŒº
- åŸºç¡€ç«¯å£ï¼ˆåŠ ä¸Šæ´¾ç”Ÿç«¯å£ï¼‰

è¿™ä½¿æ•‘æ´æœºå™¨äººä¸Žä¸»æœºå™¨äººéš”ç¦»ï¼Œä»¥ä¾¿åœ¨ä¸»æœºå™¨äººå®•æœºæ—¶å¯ä»¥è°ƒè¯•æˆ–åº”ç”¨é…ç½®æ›´æ”¹ã€‚

ç«¯å£é—´è·ï¼šåœ¨åŸºç¡€ç«¯å£ä¹‹é—´è‡³å°‘ç•™å‡º 20 ä¸ªç«¯å£ï¼Œè¿™æ ·æ´¾ç”Ÿçš„æµè§ˆå™¨/ç”»å¸ƒ/CDP ç«¯å£æ°¸è¿œä¸ä¼šå†²çªã€‚

### å¦‚ä½•å®‰è£…ï¼ˆæ•‘æ´æœºå™¨äººï¼‰

```bash
# ä¸»æœºå™¨äººï¼ˆçŽ°æœ‰æˆ–æ–°å»ºï¼Œä¸å¸¦ --profile å‚æ•°ï¼‰
# è¿è¡Œåœ¨ç«¯å£ 18789 + Chrome CDC/Canvas/... ç«¯å£
krabkrab onboard
krabkrab gateway install

# æ•‘æ´æœºå™¨äººï¼ˆéš”ç¦»çš„é…ç½®æ–‡ä»¶ + ç«¯å£ï¼‰
krabkrab --profile rescue onboard
# æ³¨æ„ï¼š
# - å·¥ä½œåŒºåç§°é»˜è®¤ä¼šæ·»åŠ  -rescue åŽç¼€
# - ç«¯å£åº”è‡³å°‘ä¸º 18789 + 20 ä¸ªç«¯å£ï¼Œ
#   æœ€å¥½é€‰æ‹©å®Œå…¨ä¸åŒçš„åŸºç¡€ç«¯å£ï¼Œå¦‚ 19789ï¼Œ
# - å…¶ä½™çš„æ–°æ‰‹å¼•å¯¼ä¸Žæ­£å¸¸ç›¸åŒ

# å®‰è£…æœåŠ¡ï¼ˆå¦‚æžœåœ¨æ–°æ‰‹å¼•å¯¼æœŸé—´æ²¡æœ‰è‡ªåŠ¨å®Œæˆï¼‰
krabkrab --profile rescue gateway install
```

## ç«¯å£æ˜ å°„ï¼ˆæ´¾ç”Ÿï¼‰

åŸºç¡€ç«¯å£ = `gateway.port`ï¼ˆæˆ– `krabkrab_GATEWAY_PORT` / `--port`ï¼‰ã€‚

- æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡ç«¯å£ = åŸºç¡€ + 2ï¼ˆä»… loopbackï¼‰
- `canvasHost.port = åŸºç¡€ + 4`
- æµè§ˆå™¨é…ç½®æ–‡ä»¶ CDP ç«¯å£ä»Ž `browser.controlPort + 9 .. + 108` è‡ªåŠ¨åˆ†é…

å¦‚æžœä½ åœ¨é…ç½®æˆ–çŽ¯å¢ƒå˜é‡ä¸­è¦†ç›–äº†è¿™äº›ï¼Œå¿…é¡»ç¡®ä¿æ¯ä¸ªå®žä¾‹éƒ½å”¯ä¸€ã€‚

## æµè§ˆå™¨/CDP æ³¨æ„äº‹é¡¹ï¼ˆå¸¸è§é™·é˜±ï¼‰

- **ä¸è¦**åœ¨å¤šä¸ªå®žä¾‹ä¸Šå°† `browser.cdpUrl` å›ºå®šä¸ºç›¸åŒçš„å€¼ã€‚
- æ¯ä¸ªå®žä¾‹éœ€è¦è‡ªå·±çš„æµè§ˆå™¨æŽ§åˆ¶ç«¯å£å’Œ CDP èŒƒå›´ï¼ˆä»Žå…¶ Gateway ç½‘å…³ç«¯å£æ´¾ç”Ÿï¼‰ã€‚
- å¦‚æžœä½ éœ€è¦æ˜¾å¼çš„ CDP ç«¯å£ï¼Œè¯·ä¸ºæ¯ä¸ªå®žä¾‹è®¾ç½® `browser.profiles.<name>.cdpPort`ã€‚
- è¿œç¨‹ Chromeï¼šä½¿ç”¨ `browser.profiles.<name>.cdpUrl`ï¼ˆæ¯ä¸ªé…ç½®æ–‡ä»¶ï¼Œæ¯ä¸ªå®žä¾‹ï¼‰ã€‚

## æ‰‹åŠ¨çŽ¯å¢ƒå˜é‡ç¤ºä¾‹

```bash
krabkrab_CONFIG_PATH=~/.krabkrab/main.json \
krabkrab_STATE_DIR=~/.krabkrab-main \
krabkrab gateway --port 18789

krabkrab_CONFIG_PATH=~/.krabkrab/rescue.json \
krabkrab_STATE_DIR=~/.krabkrab-rescue \
krabkrab gateway --port 19001
```

## å¿«é€Ÿæ£€æŸ¥

```bash
krabkrab --profile main status
krabkrab --profile rescue status
krabkrab --profile rescue browser status
```

