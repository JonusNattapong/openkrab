---
read_when:
  - åœ¨æœ¬åœ°æˆ– CI ä¸­è¿è¡Œæµ‹è¯•
  - ä¸ºæ¨¡åž‹/æä¾›å•†é—®é¢˜æ·»åŠ å›žå½’æµ‹è¯•
  - è°ƒè¯• Gateway ç½‘å…³ + æ™ºèƒ½ä½“è¡Œä¸º
summary: æµ‹è¯•å¥—ä»¶ï¼šå•å…ƒ/ç«¯åˆ°ç«¯/å®žæ—¶æµ‹è¯•å¥—ä»¶ã€Docker è¿è¡Œå™¨ï¼Œä»¥åŠæ¯ä¸ªæµ‹è¯•çš„è¦†ç›–èŒƒå›´
title: æµ‹è¯•
x-i18n:
  generated_at: "2026-02-03T09:23:12Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 8c236673838731c49464622ac54bf0336acf787b857677c8c2d2aa52949c8ad5
  source_path: help/testing.md
  workflow: 15
---

# æµ‹è¯•

KrabKrab åŒ…å«ä¸‰ä¸ª Vitest æµ‹è¯•å¥—ä»¶ï¼ˆå•å…ƒ/é›†æˆã€ç«¯åˆ°ç«¯ã€å®žæ—¶ï¼‰ä»¥åŠä¸€å°ç»„ Docker è¿è¡Œå™¨ã€‚

æœ¬æ–‡æ¡£æ˜¯ä¸€ä»½"æˆ‘ä»¬å¦‚ä½•æµ‹è¯•"çš„æŒ‡å—ï¼š

- æ¯ä¸ªå¥—ä»¶è¦†ç›–ä»€ä¹ˆï¼ˆä»¥åŠå®ƒåˆ»æ„*ä¸*è¦†ç›–ä»€ä¹ˆï¼‰
- å¸¸è§å·¥ä½œæµç¨‹åº”è¿è¡Œå“ªäº›å‘½ä»¤ï¼ˆæœ¬åœ°ã€æŽ¨é€å‰ã€è°ƒè¯•ï¼‰
- å®žæ—¶æµ‹è¯•å¦‚ä½•å‘çŽ°å‡­è¯å¹¶é€‰æ‹©æ¨¡åž‹/æä¾›å•†
- å¦‚ä½•ä¸ºçŽ°å®žä¸­çš„æ¨¡åž‹/æä¾›å•†é—®é¢˜æ·»åŠ å›žå½’æµ‹è¯•

## å¿«é€Ÿå¼€å§‹

æ—¥å¸¸ä½¿ç”¨ï¼š

- å®Œæ•´æ£€æŸ¥ï¼ˆæŽ¨é€å‰çš„é¢„æœŸæµç¨‹ï¼‰ï¼š`pnpm build && pnpm check && pnpm test`

å½“ä½ ä¿®æ”¹æµ‹è¯•æˆ–éœ€è¦é¢å¤–çš„ä¿¡å¿ƒæ—¶ï¼š

- è¦†ç›–çŽ‡æ£€æŸ¥ï¼š`pnpm test:coverage`
- ç«¯åˆ°ç«¯å¥—ä»¶ï¼š`pnpm test:e2e`

è°ƒè¯•çœŸå®žæä¾›å•†/æ¨¡åž‹æ—¶ï¼ˆéœ€è¦çœŸå®žå‡­è¯ï¼‰ï¼š

- å®žæ—¶å¥—ä»¶ï¼ˆæ¨¡åž‹ + Gateway ç½‘å…³å·¥å…·/å›¾åƒæŽ¢æµ‹ï¼‰ï¼š`pnpm test:live`

æç¤ºï¼šå½“ä½ åªéœ€è¦ä¸€ä¸ªå¤±è´¥ç”¨ä¾‹æ—¶ï¼Œå»ºè®®ä½¿ç”¨ä¸‹æ–‡æè¿°çš„å…è®¸åˆ—è¡¨çŽ¯å¢ƒå˜é‡æ¥ç¼©å°å®žæ—¶æµ‹è¯•èŒƒå›´ã€‚

## æµ‹è¯•å¥—ä»¶ï¼ˆåœ¨å“ªé‡Œè¿è¡Œä»€ä¹ˆï¼‰

å¯ä»¥å°†è¿™äº›å¥—ä»¶ç†è§£ä¸º"é€æ¸å¢žå¼ºçš„çœŸå®žæ€§"ï¼ˆä»¥åŠé€æ¸å¢žåŠ çš„ä¸ç¨³å®šæ€§/æˆæœ¬ï¼‰ï¼š

### å•å…ƒ/é›†æˆæµ‹è¯•ï¼ˆé»˜è®¤ï¼‰

- å‘½ä»¤ï¼š`pnpm test`
- é…ç½®ï¼š`vitest.config.ts`
- æ–‡ä»¶ï¼š`src/**/*.test.ts`
- èŒƒå›´ï¼š
  - çº¯å•å…ƒæµ‹è¯•
  - è¿›ç¨‹å†…é›†æˆæµ‹è¯•ï¼ˆGateway ç½‘å…³è®¤è¯ã€è·¯ç”±ã€å·¥å…·ã€è§£æžã€é…ç½®ï¼‰
  - å·²çŸ¥é—®é¢˜çš„ç¡®å®šæ€§å›žå½’æµ‹è¯•
- é¢„æœŸï¼š
  - åœ¨ CI ä¸­è¿è¡Œ
  - ä¸éœ€è¦çœŸå®žå¯†é’¥
  - åº”è¯¥å¿«é€Ÿä¸”ç¨³å®š

### ç«¯åˆ°ç«¯æµ‹è¯•ï¼ˆGateway ç½‘å…³å†’çƒŸæµ‹è¯•ï¼‰

- å‘½ä»¤ï¼š`pnpm test:e2e`
- é…ç½®ï¼š`vitest.e2e.config.ts`
- æ–‡ä»¶ï¼š`src/**/*.e2e.test.ts`
- èŒƒå›´ï¼š
  - å¤šå®žä¾‹ Gateway ç½‘å…³ç«¯åˆ°ç«¯è¡Œä¸º
  - WebSocket/HTTP æŽ¥å£ã€èŠ‚ç‚¹é…å¯¹å’Œè¾ƒé‡çš„ç½‘ç»œæ“ä½œ
- é¢„æœŸï¼š
  - åœ¨ CI ä¸­è¿è¡Œï¼ˆå½“åœ¨æµæ°´çº¿ä¸­å¯ç”¨æ—¶ï¼‰
  - ä¸éœ€è¦çœŸå®žå¯†é’¥
  - æ¯”å•å…ƒæµ‹è¯•æœ‰æ›´å¤šæ´»åŠ¨éƒ¨ä»¶ï¼ˆå¯èƒ½è¾ƒæ…¢ï¼‰

### å®žæ—¶æµ‹è¯•ï¼ˆçœŸå®žæä¾›å•† + çœŸå®žæ¨¡åž‹ï¼‰

- å‘½ä»¤ï¼š`pnpm test:live`
- é…ç½®ï¼š`vitest.live.config.ts`
- æ–‡ä»¶ï¼š`src/**/*.live.test.ts`
- é»˜è®¤ï¼šé€šè¿‡ `pnpm test:live` **å¯ç”¨**ï¼ˆè®¾ç½® `krabkrab_LIVE_TEST=1`ï¼‰
- èŒƒå›´ï¼š
  - "è¿™ä¸ªæä¾›å•†/æ¨¡åž‹ç”¨çœŸå®žå‡­è¯*ä»Šå¤©*å®žé™…èƒ½å·¥ä½œå—ï¼Ÿ"
  - æ•èŽ·æä¾›å•†æ ¼å¼å˜æ›´ã€å·¥å…·è°ƒç”¨æ€ªç™–ã€è®¤è¯é—®é¢˜å’Œé€ŸçŽ‡é™åˆ¶è¡Œä¸º
- é¢„æœŸï¼š
  - è®¾è®¡ä¸Šä¸é€‚åˆ CI ç¨³å®šè¿è¡Œï¼ˆçœŸå®žç½‘ç»œã€çœŸå®žæä¾›å•†ç­–ç•¥ã€é…é¢ã€æ•…éšœï¼‰
  - èŠ±è´¹é‡‘é’±/ä½¿ç”¨é€ŸçŽ‡é™åˆ¶
  - å»ºè®®è¿è¡Œç¼©å°èŒƒå›´çš„å­é›†è€Œéž"å…¨éƒ¨"
  - å®žæ—¶è¿è¡Œä¼šåŠ è½½ `~/.profile` ä»¥èŽ·å–ç¼ºå¤±çš„ API å¯†é’¥
  - Anthropic å¯†é’¥è½®æ¢ï¼šè®¾ç½® `krabkrab_LIVE_ANTHROPIC_KEYS="sk-...,sk-..."`ï¼ˆæˆ– `krabkrab_LIVE_ANTHROPIC_KEY=sk-...`ï¼‰æˆ–å¤šä¸ª `ANTHROPIC_API_KEY*` å˜é‡ï¼›æµ‹è¯•ä¼šåœ¨é‡åˆ°é€ŸçŽ‡é™åˆ¶æ—¶é‡è¯•

## æˆ‘åº”è¯¥è¿è¡Œå“ªä¸ªå¥—ä»¶ï¼Ÿ

ä½¿ç”¨è¿™ä¸ªå†³ç­–è¡¨ï¼š

- ç¼–è¾‘é€»è¾‘/æµ‹è¯•ï¼šè¿è¡Œ `pnpm test`ï¼ˆå¦‚æžœæ”¹åŠ¨è¾ƒå¤§ï¼ŒåŠ ä¸Š `pnpm test:coverage`ï¼‰
- æ¶‰åŠ Gateway ç½‘å…³ç½‘ç»œ/WS åè®®/é…å¯¹ï¼šåŠ ä¸Š `pnpm test:e2e`
- è°ƒè¯•"æˆ‘çš„æœºå™¨äººæŒ‚äº†"/æä¾›å•†ç‰¹å®šæ•…éšœ/å·¥å…·è°ƒç”¨ï¼šè¿è¡Œç¼©å°èŒƒå›´çš„ `pnpm test:live`

## å®žæ—¶æµ‹è¯•ï¼šæ¨¡åž‹å†’çƒŸæµ‹è¯•ï¼ˆé…ç½®æ–‡ä»¶å¯†é’¥ï¼‰

å®žæ—¶æµ‹è¯•åˆ†ä¸ºä¸¤å±‚ï¼Œä»¥ä¾¿éš”ç¦»æ•…éšœï¼š

- "ç›´æŽ¥æ¨¡åž‹"å‘Šè¯‰æˆ‘ä»¬æä¾›å•†/æ¨¡åž‹æ˜¯å¦èƒ½ç”¨ç»™å®šçš„å¯†é’¥æ­£å¸¸å“åº”ã€‚
- "Gateway ç½‘å…³å†’çƒŸæµ‹è¯•"å‘Šè¯‰æˆ‘ä»¬å®Œæ•´çš„ Gateway ç½‘å…³ + æ™ºèƒ½ä½“ç®¡é“æ˜¯å¦å¯¹è¯¥æ¨¡åž‹æ­£å¸¸å·¥ä½œï¼ˆä¼šè¯ã€åŽ†å²è®°å½•ã€å·¥å…·ã€æ²™ç®±ç­–ç•¥ç­‰ï¼‰ã€‚

### ç¬¬ä¸€å±‚ï¼šç›´æŽ¥æ¨¡åž‹è¡¥å…¨ï¼ˆæ—  Gateway ç½‘å…³ï¼‰

- æµ‹è¯•ï¼š`src/agents/models.profiles.live.test.ts`
- ç›®æ ‡ï¼š
  - æžšä¸¾å‘çŽ°çš„æ¨¡åž‹
  - ä½¿ç”¨ `getApiKeyForModel` é€‰æ‹©ä½ æœ‰å‡­è¯çš„æ¨¡åž‹
  - æ¯ä¸ªæ¨¡åž‹è¿è¡Œä¸€ä¸ªå°åž‹è¡¥å…¨ï¼ˆä»¥åŠéœ€è¦æ—¶çš„é’ˆå¯¹æ€§å›žå½’æµ‹è¯•ï¼‰
- å¦‚ä½•å¯ç”¨ï¼š
  - `pnpm test:live`ï¼ˆæˆ–ç›´æŽ¥è°ƒç”¨ Vitest æ—¶ä½¿ç”¨ `krabkrab_LIVE_TEST=1`ï¼‰
- è®¾ç½® `krabkrab_LIVE_MODELS=modern`ï¼ˆæˆ– `all`ï¼Œmodern çš„åˆ«åï¼‰ä»¥å®žé™…è¿è¡Œæ­¤å¥—ä»¶ï¼›å¦åˆ™ä¼šè·³è¿‡ä»¥ä¿æŒ `pnpm test:live` ä¸“æ³¨äºŽ Gateway ç½‘å…³å†’çƒŸæµ‹è¯•
- å¦‚ä½•é€‰æ‹©æ¨¡åž‹ï¼š
  - `krabkrab_LIVE_MODELS=modern` è¿è¡ŒçŽ°ä»£å…è®¸åˆ—è¡¨ï¼ˆOpus/Sonnet/Haiku 4.5ã€GPT-5.x + Codexã€Gemini 3ã€GLM 4.7ã€MiniMax M2.1ã€Grok 4ï¼‰
  - `krabkrab_LIVE_MODELS=all` æ˜¯çŽ°ä»£å…è®¸åˆ—è¡¨çš„åˆ«å
  - æˆ– `krabkrab_LIVE_MODELS="openai/gpt-5.2,anthropic/claude-opus-4-5,..."`ï¼ˆé€—å·åˆ†éš”çš„å…è®¸åˆ—è¡¨ï¼‰
- å¦‚ä½•é€‰æ‹©æä¾›å•†ï¼š
  - `krabkrab_LIVE_PROVIDERS="google,google-antigravity,google-gemini-cli"`ï¼ˆé€—å·åˆ†éš”çš„å…è®¸åˆ—è¡¨ï¼‰
- å¯†é’¥æ¥æºï¼š
  - é»˜è®¤ï¼šé…ç½®æ–‡ä»¶å­˜å‚¨å’ŒçŽ¯å¢ƒå˜é‡å›žé€€
  - è®¾ç½® `krabkrab_LIVE_REQUIRE_PROFILE_KEYS=1` ä»¥å¼ºåˆ¶**ä»…ä½¿ç”¨é…ç½®æ–‡ä»¶å­˜å‚¨**
- ä¸ºä»€ä¹ˆå­˜åœ¨è¿™ä¸ªæµ‹è¯•ï¼š
  - å°†"æä¾›å•† API æŸå/å¯†é’¥æ— æ•ˆ"ä¸Ž"Gateway ç½‘å…³æ™ºèƒ½ä½“ç®¡é“æŸå"åˆ†ç¦»
  - åŒ…å«å°åž‹ã€éš”ç¦»çš„å›žå½’æµ‹è¯•ï¼ˆä¾‹å¦‚ï¼šOpenAI Responses/Codex Responses æŽ¨ç†é‡æ”¾ + å·¥å…·è°ƒç”¨æµç¨‹ï¼‰

### ç¬¬äºŒå±‚ï¼šGateway ç½‘å…³ + å¼€å‘æ™ºèƒ½ä½“å†’çƒŸæµ‹è¯•ï¼ˆ"@krabkrab"å®žé™…åšçš„äº‹ï¼‰

- æµ‹è¯•ï¼š`src/gateway/gateway-models.profiles.live.test.ts`
- ç›®æ ‡ï¼š
  - å¯åŠ¨ä¸€ä¸ªè¿›ç¨‹å†… Gateway ç½‘å…³
  - åˆ›å»º/ä¿®è¡¥ä¸€ä¸ª `agent:dev:*` ä¼šè¯ï¼ˆæ¯æ¬¡è¿è¡Œè¦†ç›–æ¨¡åž‹ï¼‰
  - éåŽ†æœ‰å¯†é’¥çš„æ¨¡åž‹å¹¶æ–­è¨€ï¼š
    - "æœ‰æ„ä¹‰çš„"å“åº”ï¼ˆæ— å·¥å…·ï¼‰
    - çœŸå®žçš„å·¥å…·è°ƒç”¨å·¥ä½œæ­£å¸¸ï¼ˆè¯»å–æŽ¢æµ‹ï¼‰
    - å¯é€‰çš„é¢å¤–å·¥å…·æŽ¢æµ‹ï¼ˆæ‰§è¡Œ+è¯»å–æŽ¢æµ‹ï¼‰
    - OpenAI å›žå½’è·¯å¾„ï¼ˆä»…å·¥å…·è°ƒç”¨ â†’ åŽç»­ï¼‰ä¿æŒå·¥ä½œ
- æŽ¢æµ‹è¯¦æƒ…ï¼ˆä»¥ä¾¿ä½ èƒ½å¿«é€Ÿè§£é‡Šæ•…éšœï¼‰ï¼š
  - `read` æŽ¢æµ‹ï¼šæµ‹è¯•åœ¨å·¥ä½œåŒºå†™å…¥ä¸€ä¸ªéšæœºæ•°æ–‡ä»¶ï¼Œè¦æ±‚æ™ºèƒ½ä½“ `read` å®ƒå¹¶å›žæ˜¾éšæœºæ•°ã€‚
  - `exec+read` æŽ¢æµ‹ï¼šæµ‹è¯•è¦æ±‚æ™ºèƒ½ä½“ `exec` å°†éšæœºæ•°å†™å…¥ä¸´æ—¶æ–‡ä»¶ï¼Œç„¶åŽ `read` å›žæ¥ã€‚
  - å›¾åƒæŽ¢æµ‹ï¼šæµ‹è¯•é™„åŠ ä¸€ä¸ªç”Ÿæˆçš„ PNGï¼ˆçŒ« + éšæœºä»£ç ï¼‰ï¼ŒæœŸæœ›æ¨¡åž‹è¿”å›ž `cat <CODE>`ã€‚
  - å®žçŽ°å‚è€ƒï¼š`src/gateway/gateway-models.profiles.live.test.ts` å’Œ `src/gateway/live-image-probe.ts`ã€‚
- å¦‚ä½•å¯ç”¨ï¼š
  - `pnpm test:live`ï¼ˆæˆ–ç›´æŽ¥è°ƒç”¨ Vitest æ—¶ä½¿ç”¨ `krabkrab_LIVE_TEST=1`ï¼‰
- å¦‚ä½•é€‰æ‹©æ¨¡åž‹ï¼š
  - é»˜è®¤ï¼šçŽ°ä»£å…è®¸åˆ—è¡¨ï¼ˆOpus/Sonnet/Haiku 4.5ã€GPT-5.x + Codexã€Gemini 3ã€GLM 4.7ã€MiniMax M2.1ã€Grok 4ï¼‰
  - `krabkrab_LIVE_GATEWAY_MODELS=all` æ˜¯çŽ°ä»£å…è®¸åˆ—è¡¨çš„åˆ«å
  - æˆ–è®¾ç½® `krabkrab_LIVE_GATEWAY_MODELS="provider/model"`ï¼ˆæˆ–é€—å·åˆ†éš”åˆ—è¡¨ï¼‰æ¥ç¼©å°èŒƒå›´
- å¦‚ä½•é€‰æ‹©æä¾›å•†ï¼ˆé¿å…"OpenRouter å…¨éƒ¨"ï¼‰ï¼š
  - `krabkrab_LIVE_GATEWAY_PROVIDERS="google,google-antigravity,google-gemini-cli,openai,anthropic,zai,minimax"`ï¼ˆé€—å·åˆ†éš”çš„å…è®¸åˆ—è¡¨ï¼‰
- å·¥å…· + å›¾åƒæŽ¢æµ‹åœ¨æ­¤å®žæ—¶æµ‹è¯•ä¸­å§‹ç»ˆå¼€å¯ï¼š
  - `read` æŽ¢æµ‹ + `exec+read` æŽ¢æµ‹ï¼ˆå·¥å…·åŽ‹åŠ›æµ‹è¯•ï¼‰
  - å½“æ¨¡åž‹å£°æ˜Žæ”¯æŒå›¾åƒè¾“å…¥æ—¶è¿è¡Œå›¾åƒæŽ¢æµ‹
  - æµç¨‹ï¼ˆé«˜å±‚æ¬¡ï¼‰ï¼š
    - æµ‹è¯•ç”Ÿæˆä¸€ä¸ªå¸¦æœ‰"CAT"+ éšæœºä»£ç çš„å°åž‹ PNGï¼ˆ`src/gateway/live-image-probe.ts`ï¼‰
    - é€šè¿‡ `agent` `attachments: [{ mimeType: "image/png", content: "<base64>" }]` å‘é€
    - Gateway ç½‘å…³å°†é™„ä»¶è§£æžä¸º `images[]`ï¼ˆ`src/gateway/server-methods/agent.ts` + `src/gateway/chat-attachments.ts`ï¼‰
    - åµŒå…¥å¼æ™ºèƒ½ä½“å°†å¤šæ¨¡æ€ç”¨æˆ·æ¶ˆæ¯è½¬å‘ç»™æ¨¡åž‹
    - æ–­è¨€ï¼šå›žå¤åŒ…å« `cat` + ä»£ç ï¼ˆOCR å®¹å·®ï¼šå…è®¸è½»å¾®é”™è¯¯ï¼‰

æç¤ºï¼šè¦æŸ¥çœ‹ä½ çš„æœºå™¨ä¸Šå¯ä»¥æµ‹è¯•ä»€ä¹ˆï¼ˆä»¥åŠç¡®åˆ‡çš„ `provider/model` IDï¼‰ï¼Œè¿è¡Œï¼š

```bash
krabkrab models list
krabkrab models list --json
```

## å®žæ—¶æµ‹è¯•ï¼šAnthropic è®¾ç½®ä»¤ç‰Œå†’çƒŸæµ‹è¯•

- æµ‹è¯•ï¼š`src/agents/anthropic.setup-token.live.test.ts`
- ç›®æ ‡ï¼šéªŒè¯ Claude Code CLI è®¾ç½®ä»¤ç‰Œï¼ˆæˆ–ç²˜è´´çš„è®¾ç½®ä»¤ç‰Œé…ç½®æ–‡ä»¶ï¼‰èƒ½å®Œæˆ Anthropic æç¤ºã€‚
- å¯ç”¨ï¼š
  - `pnpm test:live`ï¼ˆæˆ–ç›´æŽ¥è°ƒç”¨ Vitest æ—¶ä½¿ç”¨ `krabkrab_LIVE_TEST=1`ï¼‰
  - `krabkrab_LIVE_SETUP_TOKEN=1`
- ä»¤ç‰Œæ¥æºï¼ˆé€‰æ‹©ä¸€ä¸ªï¼‰ï¼š
  - é…ç½®æ–‡ä»¶ï¼š`krabkrab_LIVE_SETUP_TOKEN_PROFILE=anthropic:setup-token-test`
  - åŽŸå§‹ä»¤ç‰Œï¼š`krabkrab_LIVE_SETUP_TOKEN_VALUE=sk-ant-oat01-...`
- æ¨¡åž‹è¦†ç›–ï¼ˆå¯é€‰ï¼‰ï¼š
  - `krabkrab_LIVE_SETUP_TOKEN_MODEL=anthropic/claude-opus-4-5`

è®¾ç½®ç¤ºä¾‹ï¼š

```bash
krabkrab models auth paste-token --provider anthropic --profile-id anthropic:setup-token-test
krabkrab_LIVE_SETUP_TOKEN=1 krabkrab_LIVE_SETUP_TOKEN_PROFILE=anthropic:setup-token-test pnpm test:live src/agents/anthropic.setup-token.live.test.ts
```

## å®žæ—¶æµ‹è¯•ï¼šCLI åŽç«¯å†’çƒŸæµ‹è¯•ï¼ˆClaude Code CLI æˆ–å…¶ä»–æœ¬åœ° CLIï¼‰

- æµ‹è¯•ï¼š`src/gateway/gateway-cli-backend.live.test.ts`
- ç›®æ ‡ï¼šä½¿ç”¨æœ¬åœ° CLI åŽç«¯éªŒè¯ Gateway ç½‘å…³ + æ™ºèƒ½ä½“ç®¡é“ï¼Œè€Œä¸å½±å“ä½ çš„é»˜è®¤é…ç½®ã€‚
- å¯ç”¨ï¼š
  - `pnpm test:live`ï¼ˆæˆ–ç›´æŽ¥è°ƒç”¨ Vitest æ—¶ä½¿ç”¨ `krabkrab_LIVE_TEST=1`ï¼‰
  - `krabkrab_LIVE_CLI_BACKEND=1`
- é»˜è®¤å€¼ï¼š
  - æ¨¡åž‹ï¼š`claude-cli/claude-sonnet-4-5`
  - å‘½ä»¤ï¼š`claude`
  - å‚æ•°ï¼š`["-p","--output-format","json","--dangerously-skip-permissions"]`
- è¦†ç›–ï¼ˆå¯é€‰ï¼‰ï¼š
  - `krabkrab_LIVE_CLI_BACKEND_MODEL="claude-cli/claude-opus-4-5"`
  - `krabkrab_LIVE_CLI_BACKEND_MODEL="codex-cli/gpt-5.2-codex"`
  - `krabkrab_LIVE_CLI_BACKEND_COMMAND="/full/path/to/claude"`
  - `krabkrab_LIVE_CLI_BACKEND_ARGS='["-p","--output-format","json","--permission-mode","bypassPermissions"]'`
  - `krabkrab_LIVE_CLI_BACKEND_CLEAR_ENV='["ANTHROPIC_API_KEY","ANTHROPIC_API_KEY_OLD"]'`
  - `krabkrab_LIVE_CLI_BACKEND_IMAGE_PROBE=1` å‘é€çœŸå®žå›¾åƒé™„ä»¶ï¼ˆè·¯å¾„æ³¨å…¥åˆ°æç¤ºä¸­ï¼‰ã€‚
  - `krabkrab_LIVE_CLI_BACKEND_IMAGE_ARG="--image"` å°†å›¾åƒæ–‡ä»¶è·¯å¾„ä½œä¸º CLI å‚æ•°ä¼ é€’è€Œéžæç¤ºæ³¨å…¥ã€‚
  - `krabkrab_LIVE_CLI_BACKEND_IMAGE_MODE="repeat"`ï¼ˆæˆ– `"list"`ï¼‰æŽ§åˆ¶è®¾ç½® `IMAGE_ARG` æ—¶å¦‚ä½•ä¼ é€’å›¾åƒå‚æ•°ã€‚
  - `krabkrab_LIVE_CLI_BACKEND_RESUME_PROBE=1` å‘é€ç¬¬äºŒè½®å¹¶éªŒè¯æ¢å¤æµç¨‹ã€‚
- `krabkrab_LIVE_CLI_BACKEND_DISABLE_MCP_CONFIG=0` ä¿æŒ Claude Code CLI MCP é…ç½®å¯ç”¨ï¼ˆé»˜è®¤ä½¿ç”¨ä¸´æ—¶ç©ºæ–‡ä»¶ç¦ç”¨ MCP é…ç½®ï¼‰ã€‚

ç¤ºä¾‹ï¼š

```bash
krabkrab_LIVE_CLI_BACKEND=1 \
  krabkrab_LIVE_CLI_BACKEND_MODEL="claude-cli/claude-sonnet-4-5" \
  pnpm test:live src/gateway/gateway-cli-backend.live.test.ts
```

### æŽ¨èçš„å®žæ—¶æµ‹è¯•é…æ–¹

ç¼©å°èŒƒå›´çš„æ˜¾å¼å…è®¸åˆ—è¡¨æœ€å¿«ä¸”æœ€ä¸æ˜“å‡ºé”™ï¼š

- å•ä¸ªæ¨¡åž‹ï¼Œç›´æŽ¥æµ‹è¯•ï¼ˆæ—  Gateway ç½‘å…³ï¼‰ï¼š
  - `krabkrab_LIVE_MODELS="openai/gpt-5.2" pnpm test:live src/agents/models.profiles.live.test.ts`

- å•ä¸ªæ¨¡åž‹ï¼ŒGateway ç½‘å…³å†’çƒŸæµ‹è¯•ï¼š
  - `krabkrab_LIVE_GATEWAY_MODELS="openai/gpt-5.2" pnpm test:live src/gateway/gateway-models.profiles.live.test.ts`

- è·¨å¤šä¸ªæä¾›å•†çš„å·¥å…·è°ƒç”¨ï¼š
  - `krabkrab_LIVE_GATEWAY_MODELS="openai/gpt-5.2,anthropic/claude-opus-4-5,google/gemini-3-flash-preview,zai/glm-4.7,minimax/minimax-m2.1" pnpm test:live src/gateway/gateway-models.profiles.live.test.ts`

- Google ä¸“é¡¹ï¼ˆGemini API å¯†é’¥ + Antigravityï¼‰ï¼š
  - Geminiï¼ˆAPI å¯†é’¥ï¼‰ï¼š`krabkrab_LIVE_GATEWAY_MODELS="google/gemini-3-flash-preview" pnpm test:live src/gateway/gateway-models.profiles.live.test.ts`
  - Antigravityï¼ˆOAuthï¼‰ï¼š`krabkrab_LIVE_GATEWAY_MODELS="google-antigravity/claude-opus-4-6-thinking,google-antigravity/gemini-3-pro-high" pnpm test:live src/gateway/gateway-models.profiles.live.test.ts`

æ³¨æ„ï¼š

- `google/...` ä½¿ç”¨ Gemini APIï¼ˆAPI å¯†é’¥ï¼‰ã€‚
- `google-antigravity/...` ä½¿ç”¨ Antigravity OAuth æ¡¥æŽ¥ï¼ˆCloud Code Assist é£Žæ ¼çš„æ™ºèƒ½ä½“ç«¯ç‚¹ï¼‰ã€‚
- `google-gemini-cli/...` ä½¿ç”¨ä½ æœºå™¨ä¸Šçš„æœ¬åœ° Gemini CLIï¼ˆç‹¬ç«‹çš„è®¤è¯ + å·¥å…·æ€ªç™–ï¼‰ã€‚
- Gemini API ä¸Ž Gemini CLIï¼š
  - APIï¼šKrabKrab é€šè¿‡ HTTP è°ƒç”¨ Google æ‰˜ç®¡çš„ Gemini APIï¼ˆAPI å¯†é’¥/é…ç½®æ–‡ä»¶è®¤è¯ï¼‰ï¼›è¿™æ˜¯å¤§å¤šæ•°ç”¨æˆ·è¯´çš„"Gemini"ã€‚
  - CLIï¼šKrabKrab è°ƒç”¨æœ¬åœ° `gemini` äºŒè¿›åˆ¶æ–‡ä»¶ï¼›å®ƒæœ‰è‡ªå·±çš„è®¤è¯ï¼Œè¡Œä¸ºå¯èƒ½ä¸åŒï¼ˆæµå¼ä¼ è¾“/å·¥å…·æ”¯æŒ/ç‰ˆæœ¬å·®å¼‚ï¼‰ã€‚

## å®žæ—¶æµ‹è¯•ï¼šæ¨¡åž‹çŸ©é˜µï¼ˆæˆ‘ä»¬è¦†ç›–ä»€ä¹ˆï¼‰

æ²¡æœ‰å›ºå®šçš„"CI æ¨¡åž‹åˆ—è¡¨"ï¼ˆå®žæ—¶æµ‹è¯•æ˜¯å¯é€‰çš„ï¼‰ï¼Œä½†è¿™äº›æ˜¯å»ºè®®åœ¨æœ‰å¯†é’¥çš„å¼€å‘æœºå™¨ä¸Šå®šæœŸè¦†ç›–çš„**æŽ¨è**æ¨¡åž‹ã€‚

### çŽ°ä»£å†’çƒŸæµ‹è¯•é›†ï¼ˆå·¥å…·è°ƒç”¨ + å›¾åƒï¼‰

è¿™æ˜¯æˆ‘ä»¬æœŸæœ›ä¿æŒå·¥ä½œçš„"å¸¸ç”¨æ¨¡åž‹"è¿è¡Œï¼š

- OpenAIï¼ˆéž Codexï¼‰ï¼š`openai/gpt-5.2`ï¼ˆå¯é€‰ï¼š`openai/gpt-5.1`ï¼‰
- OpenAI Codexï¼š`openai-codex/gpt-5.2`ï¼ˆå¯é€‰ï¼š`openai-codex/gpt-5.2-codex`ï¼‰
- Anthropicï¼š`anthropic/claude-opus-4-5`ï¼ˆæˆ– `anthropic/claude-sonnet-4-5`ï¼‰
- Googleï¼ˆGemini APIï¼‰ï¼š`google/gemini-3-pro-preview` å’Œ `google/gemini-3-flash-preview`ï¼ˆé¿å…è¾ƒæ—§çš„ Gemini 2.x æ¨¡åž‹ï¼‰
- Googleï¼ˆAntigravityï¼‰ï¼š`google-antigravity/claude-opus-4-6-thinking` å’Œ `google-antigravity/gemini-3-flash`
- Z.AIï¼ˆGLMï¼‰ï¼š`zai/glm-4.7`
- MiniMaxï¼š`minimax/minimax-m2.1`

è¿è¡Œå¸¦å·¥å…· + å›¾åƒçš„ Gateway ç½‘å…³å†’çƒŸæµ‹è¯•ï¼š
`krabkrab_LIVE_GATEWAY_MODELS="openai/gpt-5.2,openai-codex/gpt-5.2,anthropic/claude-opus-4-5,google/gemini-3-pro-preview,google/gemini-3-flash-preview,google-antigravity/claude-opus-4-6-thinking,google-antigravity/gemini-3-flash,zai/glm-4.7,minimax/minimax-m2.1" pnpm test:live src/gateway/gateway-models.profiles.live.test.ts`

### åŸºçº¿ï¼šå·¥å…·è°ƒç”¨ï¼ˆRead + å¯é€‰ Execï¼‰

æ¯ä¸ªæä¾›å•†ç³»åˆ—è‡³å°‘é€‰æ‹©ä¸€ä¸ªï¼š

- OpenAIï¼š`openai/gpt-5.2`ï¼ˆæˆ– `openai/gpt-5-mini`ï¼‰
- Anthropicï¼š`anthropic/claude-opus-4-5`ï¼ˆæˆ– `anthropic/claude-sonnet-4-5`ï¼‰
- Googleï¼š`google/gemini-3-flash-preview`ï¼ˆæˆ– `google/gemini-3-pro-preview`ï¼‰
- Z.AIï¼ˆGLMï¼‰ï¼š`zai/glm-4.7`
- MiniMaxï¼š`minimax/minimax-m2.1`

å¯é€‰çš„é¢å¤–è¦†ç›–ï¼ˆé”¦ä¸Šæ·»èŠ±ï¼‰ï¼š

- xAIï¼š`xai/grok-4`ï¼ˆæˆ–æœ€æ–°å¯ç”¨ç‰ˆæœ¬ï¼‰
- Mistralï¼š`mistral/`â€¦ï¼ˆé€‰æ‹©ä¸€ä¸ªä½ å·²å¯ç”¨çš„"å·¥å…·"èƒ½åŠ›æ¨¡åž‹ï¼‰
- Cerebrasï¼š`cerebras/`â€¦ï¼ˆå¦‚æžœä½ æœ‰è®¿é—®æƒé™ï¼‰
- LM Studioï¼š`lmstudio/`â€¦ï¼ˆæœ¬åœ°ï¼›å·¥å…·è°ƒç”¨å–å†³äºŽ API æ¨¡å¼ï¼‰

### è§†è§‰ï¼šå›¾åƒå‘é€ï¼ˆé™„ä»¶ â†’ å¤šæ¨¡æ€æ¶ˆæ¯ï¼‰

åœ¨ `krabkrab_LIVE_GATEWAY_MODELS` ä¸­è‡³å°‘åŒ…å«ä¸€ä¸ªæ”¯æŒå›¾åƒçš„æ¨¡åž‹ï¼ˆClaude/Gemini/OpenAI è§†è§‰èƒ½åŠ›å˜ä½“ç­‰ï¼‰ä»¥æµ‹è¯•å›¾åƒæŽ¢æµ‹ã€‚

### èšåˆå™¨/æ›¿ä»£ Gateway ç½‘å…³

å¦‚æžœä½ å¯ç”¨äº†å¯†é’¥ï¼Œæˆ‘ä»¬ä¹Ÿæ”¯æŒé€šè¿‡ä»¥ä¸‹æ–¹å¼æµ‹è¯•ï¼š

- OpenRouterï¼š`openrouter/...`ï¼ˆæ•°ç™¾ä¸ªæ¨¡åž‹ï¼›ä½¿ç”¨ `krabkrab models scan` æŸ¥æ‰¾æ”¯æŒå·¥å…·+å›¾åƒçš„å€™é€‰æ¨¡åž‹ï¼‰
- OpenCode Zenï¼š`opencode/...`ï¼ˆé€šè¿‡ `OPENCODE_API_KEY` / `OPENCODE_ZEN_API_KEY` è®¤è¯ï¼‰

å¦‚æžœä½ æœ‰å‡­è¯/é…ç½®ï¼Œå¯ä»¥åœ¨å®žæ—¶çŸ©é˜µä¸­åŒ…å«æ›´å¤šæä¾›å•†ï¼š

- å†…ç½®ï¼š`openai`ã€`openai-codex`ã€`anthropic`ã€`google`ã€`google-vertex`ã€`google-antigravity`ã€`google-gemini-cli`ã€`zai`ã€`openrouter`ã€`opencode`ã€`xai`ã€`groq`ã€`cerebras`ã€`mistral`ã€`github-copilot`
- é€šè¿‡ `models.providers`ï¼ˆè‡ªå®šä¹‰ç«¯ç‚¹ï¼‰ï¼š`minimax`ï¼ˆäº‘/APIï¼‰ï¼Œä»¥åŠä»»ä½• OpenAI/Anthropic å…¼å®¹ä»£ç†ï¼ˆLM Studioã€vLLMã€LiteLLM ç­‰ï¼‰

æç¤ºï¼šä¸è¦è¯•å›¾åœ¨æ–‡æ¡£ä¸­ç¡¬ç¼–ç "æ‰€æœ‰æ¨¡åž‹"ã€‚æƒå¨åˆ—è¡¨æ˜¯ä½ æœºå™¨ä¸Š `discoverModels(...)` è¿”å›žçš„å†…å®¹ + å¯ç”¨çš„å¯†é’¥ã€‚

## å‡­è¯ï¼ˆç»ä¸æäº¤ï¼‰

å®žæ—¶æµ‹è¯•ä»¥ä¸Ž CLI ç›¸åŒçš„æ–¹å¼å‘çŽ°å‡­è¯ã€‚å®žé™…å«ä¹‰ï¼š

- å¦‚æžœ CLI èƒ½å·¥ä½œï¼Œå®žæ—¶æµ‹è¯•åº”è¯¥èƒ½æ‰¾åˆ°ç›¸åŒçš„å¯†é’¥ã€‚
- å¦‚æžœå®žæ—¶æµ‹è¯•è¯´"æ— å‡­è¯"ï¼Œç”¨è°ƒè¯• `krabkrab models list`/æ¨¡åž‹é€‰æ‹©ç›¸åŒçš„æ–¹å¼è°ƒè¯•ã€‚

- é…ç½®æ–‡ä»¶å­˜å‚¨ï¼š`~/.krabkrab/credentials/`ï¼ˆé¦–é€‰ï¼›æµ‹è¯•ä¸­"é…ç½®æ–‡ä»¶å¯†é’¥"çš„å«ä¹‰ï¼‰
- é…ç½®ï¼š`~/.krabkrab/krabkrab.json`ï¼ˆæˆ– `krabkrab_CONFIG_PATH`ï¼‰

å¦‚æžœä½ æƒ³ä¾èµ–çŽ¯å¢ƒå˜é‡å¯†é’¥ï¼ˆä¾‹å¦‚åœ¨ `~/.profile` ä¸­å¯¼å‡ºçš„ï¼‰ï¼Œåœ¨ `source ~/.profile` åŽè¿è¡Œæœ¬åœ°æµ‹è¯•ï¼Œæˆ–ä½¿ç”¨ä¸‹é¢çš„ Docker è¿è¡Œå™¨ï¼ˆå®ƒä»¬å¯ä»¥å°† `~/.profile` æŒ‚è½½åˆ°å®¹å™¨ä¸­ï¼‰ã€‚

## Deepgram å®žæ—¶æµ‹è¯•ï¼ˆéŸ³é¢‘è½¬å½•ï¼‰

- æµ‹è¯•ï¼š`src/media-understanding/providers/deepgram/audio.live.test.ts`
- å¯ç”¨ï¼š`DEEPGRAM_API_KEY=... DEEPGRAM_LIVE_TEST=1 pnpm test:live src/media-understanding/providers/deepgram/audio.live.test.ts`

## Docker è¿è¡Œå™¨ï¼ˆå¯é€‰çš„"åœ¨ Linux ä¸­å·¥ä½œ"æ£€æŸ¥ï¼‰

è¿™äº›åœ¨ä»“åº“ Docker é•œåƒå†…è¿è¡Œ `pnpm test:live`ï¼ŒæŒ‚è½½ä½ çš„æœ¬åœ°é…ç½®ç›®å½•å’Œå·¥ä½œåŒºï¼ˆå¦‚æžœæŒ‚è½½äº† `~/.profile` åˆ™ä¼šåŠ è½½å®ƒï¼‰ï¼š

- ç›´æŽ¥æ¨¡åž‹ï¼š`pnpm test:docker:live-models`ï¼ˆè„šæœ¬ï¼š`scripts/test-live-models-docker.sh`ï¼‰
- Gateway ç½‘å…³ + å¼€å‘æ™ºèƒ½ä½“ï¼š`pnpm test:docker:live-gateway`ï¼ˆè„šæœ¬ï¼š`scripts/test-live-gateway-models-docker.sh`ï¼‰
- æ–°æ‰‹å¼•å¯¼å‘å¯¼ï¼ˆTTYï¼Œå®Œæ•´è„šæ‰‹æž¶ï¼‰ï¼š`pnpm test:docker:onboard`ï¼ˆè„šæœ¬ï¼š`scripts/e2e/onboard-docker.sh`ï¼‰
- Gateway ç½‘å…³ç½‘ç»œï¼ˆä¸¤ä¸ªå®¹å™¨ï¼ŒWS è®¤è¯ + å¥åº·æ£€æŸ¥ï¼‰ï¼š`pnpm test:docker:gateway-network`ï¼ˆè„šæœ¬ï¼š`scripts/e2e/gateway-network-docker.sh`ï¼‰
- æ’ä»¶ï¼ˆè‡ªå®šä¹‰æ‰©å±•åŠ è½½ + æ³¨å†Œè¡¨å†’çƒŸæµ‹è¯•ï¼‰ï¼š`pnpm test:docker:plugins`ï¼ˆè„šæœ¬ï¼š`scripts/e2e/plugins-docker.sh`ï¼‰

æœ‰ç”¨çš„çŽ¯å¢ƒå˜é‡ï¼š

- `krabkrab_CONFIG_DIR=...`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab`ï¼‰æŒ‚è½½åˆ° `/home/node/.krabkrab`
- `krabkrab_WORKSPACE_DIR=...`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/workspace`ï¼‰æŒ‚è½½åˆ° `/home/node/.krabkrab/workspace`
- `krabkrab_PROFILE_FILE=...`ï¼ˆé»˜è®¤ï¼š`~/.profile`ï¼‰æŒ‚è½½åˆ° `/home/node/.profile` å¹¶åœ¨è¿è¡Œæµ‹è¯•å‰åŠ è½½
- `krabkrab_LIVE_GATEWAY_MODELS=...` / `krabkrab_LIVE_MODELS=...` ç”¨äºŽç¼©å°è¿è¡ŒèŒƒå›´
- `krabkrab_LIVE_REQUIRE_PROFILE_KEYS=1` ç¡®ä¿å‡­è¯æ¥è‡ªé…ç½®æ–‡ä»¶å­˜å‚¨ï¼ˆè€ŒéžçŽ¯å¢ƒå˜é‡ï¼‰

## æ–‡æ¡£å®Œæ•´æ€§æ£€æŸ¥

æ–‡æ¡£ç¼–è¾‘åŽè¿è¡Œæ–‡æ¡£æ£€æŸ¥ï¼š`pnpm docs:list`ã€‚

## ç¦»çº¿å›žå½’æµ‹è¯•ï¼ˆCI å®‰å…¨ï¼‰

è¿™äº›æ˜¯æ²¡æœ‰çœŸå®žæä¾›å•†çš„"çœŸå®žç®¡é“"å›žå½’æµ‹è¯•ï¼š

- Gateway ç½‘å…³å·¥å…·è°ƒç”¨ï¼ˆæ¨¡æ‹Ÿ OpenAIï¼ŒçœŸå®ž Gateway ç½‘å…³ + æ™ºèƒ½ä½“å¾ªçŽ¯ï¼‰ï¼š`src/gateway/gateway.tool-calling.mock-openai.test.ts`
- Gateway ç½‘å…³å‘å¯¼ï¼ˆWS `wizard.start`/`wizard.next`ï¼Œå†™å…¥é…ç½® + å¼ºåˆ¶è®¤è¯ï¼‰ï¼š`src/gateway/gateway.wizard.e2e.test.ts`

## æ™ºèƒ½ä½“å¯é æ€§è¯„ä¼°ï¼ˆSkillsï¼‰

æˆ‘ä»¬å·²ç»æœ‰ä¸€äº› CI å®‰å…¨çš„æµ‹è¯•ï¼Œå®ƒä»¬çš„è¡Œä¸ºç±»ä¼¼äºŽ"æ™ºèƒ½ä½“å¯é æ€§è¯„ä¼°"ï¼š

- é€šè¿‡çœŸå®ž Gateway ç½‘å…³ + æ™ºèƒ½ä½“å¾ªçŽ¯çš„æ¨¡æ‹Ÿå·¥å…·è°ƒç”¨ï¼ˆ`src/gateway/gateway.tool-calling.mock-openai.test.ts`ï¼‰ã€‚
- éªŒè¯ä¼šè¯è¿žæŽ¥å’Œé…ç½®æ•ˆæžœçš„ç«¯åˆ°ç«¯å‘å¯¼æµç¨‹ï¼ˆ`src/gateway/gateway.wizard.e2e.test.ts`ï¼‰ã€‚

å¯¹äºŽ Skills ä»ç„¶ç¼ºå°‘çš„å†…å®¹ï¼ˆå‚è§ [Skills](/tools/skills)ï¼‰ï¼š

- **å†³ç­–ï¼š** å½“ Skills åœ¨æç¤ºä¸­åˆ—å‡ºæ—¶ï¼Œæ™ºèƒ½ä½“æ˜¯å¦é€‰æ‹©æ­£ç¡®çš„ skillï¼ˆæˆ–é¿å…ä¸ç›¸å…³çš„ï¼‰ï¼Ÿ
- **åˆè§„æ€§ï¼š** æ™ºèƒ½ä½“æ˜¯å¦åœ¨ä½¿ç”¨å‰è¯»å– `SKILL.md` å¹¶éµå¾ªæ‰€éœ€çš„æ­¥éª¤/å‚æ•°ï¼Ÿ
- **å·¥ä½œæµå¥‘çº¦ï¼š** æ–­è¨€å·¥å…·é¡ºåºã€ä¼šè¯åŽ†å²å»¶ç»­å’Œæ²™ç®±è¾¹ç•Œçš„å¤šè½®åœºæ™¯ã€‚

æœªæ¥çš„è¯„ä¼°åº”è¯¥é¦–å…ˆä¿æŒç¡®å®šæ€§ï¼š

- ä½¿ç”¨æ¨¡æ‹Ÿæä¾›å•†æ¥æ–­è¨€å·¥å…·è°ƒç”¨ + é¡ºåºã€skill æ–‡ä»¶è¯»å–å’Œä¼šè¯è¿žæŽ¥çš„åœºæ™¯è¿è¡Œå™¨ã€‚
- ä¸€å°å¥—ä¸“æ³¨äºŽ skill çš„åœºæ™¯ï¼ˆä½¿ç”¨ vs é¿å…ã€é—¨æŽ§ã€æç¤ºæ³¨å…¥ï¼‰ã€‚
- å¯é€‰çš„å®žæ—¶è¯„ä¼°ï¼ˆå¯é€‰çš„ï¼ŒçŽ¯å¢ƒå˜é‡é—¨æŽ§ï¼‰ï¼Œä»…åœ¨ CI å®‰å…¨å¥—ä»¶å°±ä½åŽã€‚

## æ·»åŠ å›žå½’æµ‹è¯•ï¼ˆæŒ‡å¯¼ï¼‰

å½“ä½ ä¿®å¤åœ¨å®žæ—¶æµ‹è¯•ä¸­å‘çŽ°çš„æä¾›å•†/æ¨¡åž‹é—®é¢˜æ—¶ï¼š

- å¦‚æžœå¯èƒ½ï¼Œæ·»åŠ  CI å®‰å…¨çš„å›žå½’æµ‹è¯•ï¼ˆæ¨¡æ‹Ÿ/å­˜æ ¹æä¾›å•†ï¼Œæˆ–æ•èŽ·ç¡®åˆ‡çš„è¯·æ±‚å½¢çŠ¶è½¬æ¢ï¼‰
- å¦‚æžœå®ƒæœ¬è´¨ä¸Šæ˜¯ä»…é™å®žæ—¶çš„ï¼ˆé€ŸçŽ‡é™åˆ¶ã€è®¤è¯ç­–ç•¥ï¼‰ï¼Œä¿æŒå®žæ—¶æµ‹è¯•èŒƒå›´å°ä¸”é€šè¿‡çŽ¯å¢ƒå˜é‡å¯é€‰
- ä¼˜å…ˆé’ˆå¯¹èƒ½æ•èŽ·é—®é¢˜çš„æœ€å°å±‚ï¼š
  - æä¾›å•†è¯·æ±‚è½¬æ¢/é‡æ”¾é—®é¢˜ â†’ ç›´æŽ¥æ¨¡åž‹æµ‹è¯•
  - Gateway ç½‘å…³ä¼šè¯/åŽ†å²/å·¥å…·ç®¡é“é—®é¢˜ â†’ Gateway ç½‘å…³å®žæ—¶å†’çƒŸæµ‹è¯•æˆ– CI å®‰å…¨çš„ Gateway ç½‘å…³æ¨¡æ‹Ÿæµ‹è¯•

