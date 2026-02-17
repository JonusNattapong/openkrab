---
read_when:
  - ä½ éœ€è¦æ£€æŸ¥åŽŸå§‹æ¨¡åž‹è¾“å‡ºä»¥æŸ¥æ‰¾æŽ¨ç†æ³„æ¼
  - ä½ æƒ³åœ¨è¿­ä»£æ—¶ä»¥ç›‘è§†æ¨¡å¼è¿è¡Œ Gateway ç½‘å…³
  - ä½ éœ€è¦å¯é‡å¤çš„è°ƒè¯•å·¥ä½œæµ
summary: è°ƒè¯•å·¥å…·ï¼šç›‘è§†æ¨¡å¼ã€åŽŸå§‹æ¨¡åž‹æµå’Œè¿½è¸ªæŽ¨ç†æ³„æ¼
title: è°ƒè¯•
x-i18n:
  generated_at: "2026-02-03T07:47:23Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 504c824bff4790006c8b73600daca66b919e049178e9711e6e65b6254731911a
  source_path: help/debugging.md
  workflow: 15
---

# è°ƒè¯•

æœ¬é¡µä»‹ç»ç”¨äºŽæµå¼è¾“å‡ºçš„è°ƒè¯•è¾…åŠ©å·¥å…·ï¼Œç‰¹åˆ«æ˜¯å½“æä¾›å•†å°†æŽ¨ç†æ··å…¥æ­£å¸¸æ–‡æœ¬æ—¶ã€‚

## è¿è¡Œæ—¶è°ƒè¯•è¦†ç›–

åœ¨èŠå¤©ä¸­ä½¿ç”¨ `/debug` è®¾ç½®**ä»…è¿è¡Œæ—¶**é…ç½®è¦†ç›–ï¼ˆå†…å­˜ä¸­ï¼Œä¸å†™å…¥ç£ç›˜ï¼‰ã€‚
`/debug` é»˜è®¤ç¦ç”¨ï¼›é€šè¿‡ `commands.debug: true` å¯ç”¨ã€‚
å½“ä½ éœ€è¦åˆ‡æ¢ä¸å¸¸ç”¨çš„è®¾ç½®è€Œä¸ç¼–è¾‘ `krabkrab.json` æ—¶ï¼Œè¿™éžå¸¸æ–¹ä¾¿ã€‚

ç¤ºä¾‹ï¼š

```
/debug show
/debug set messages.responsePrefix="[krabkrab]"
/debug unset messages.responsePrefix
/debug reset
```

`/debug reset` æ¸…é™¤æ‰€æœ‰è¦†ç›–å¹¶è¿”å›žåˆ°ç£ç›˜ä¸Šçš„é…ç½®ã€‚

## Gateway ç½‘å…³ç›‘è§†æ¨¡å¼

ä¸ºäº†å¿«é€Ÿè¿­ä»£ï¼Œåœ¨æ–‡ä»¶ç›‘è§†å™¨ä¸‹è¿è¡Œ Gateway ç½‘å…³ï¼š

```bash
pnpm gateway:watch --force
```

è¿™æ˜ å°„åˆ°ï¼š

```bash
tsx watch src/entry.ts gateway --force
```

åœ¨ `gateway:watch` åŽæ·»åŠ ä»»ä½• Gateway ç½‘å…³ CLI æ ‡å¿—ï¼Œå®ƒä»¬å°†åœ¨æ¯æ¬¡é‡å¯æ—¶ä¼ é€’ã€‚

## Dev é…ç½®æ–‡ä»¶ + dev Gateway ç½‘å…³ï¼ˆ--devï¼‰

ä½¿ç”¨ dev é…ç½®æ–‡ä»¶æ¥éš”ç¦»çŠ¶æ€ï¼Œå¹¶å¯åŠ¨ä¸€ä¸ªå®‰å…¨ã€å¯ä¸¢å¼ƒçš„è°ƒè¯•è®¾ç½®ã€‚æœ‰**ä¸¤ä¸ª** `--dev` æ ‡å¿—ï¼š

- **å…¨å±€ `--dev`ï¼ˆé…ç½®æ–‡ä»¶ï¼‰ï¼š** å°†çŠ¶æ€éš”ç¦»åˆ° `~/.krabkrab-dev` ä¸‹ï¼Œå¹¶å°† Gateway ç½‘å…³ç«¯å£é»˜è®¤ä¸º `19001`ï¼ˆæ´¾ç”Ÿç«¯å£éšä¹‹ç§»åŠ¨ï¼‰ã€‚
- **`gateway --dev`ï¼šå‘Šè¯‰ Gateway ç½‘å…³åœ¨ç¼ºå¤±æ—¶è‡ªåŠ¨åˆ›å»ºé»˜è®¤é…ç½® + å·¥ä½œåŒº**ï¼ˆå¹¶è·³è¿‡ BOOTSTRAP.mdï¼‰ã€‚

æŽ¨èæµç¨‹ï¼ˆdev é…ç½®æ–‡ä»¶ + dev å¼•å¯¼ï¼‰ï¼š

```bash
pnpm gateway:dev
krabkrab_PROFILE=dev krabkrab tui
```

å¦‚æžœä½ è¿˜æ²¡æœ‰å…¨å±€å®‰è£…ï¼Œè¯·é€šè¿‡ `pnpm krabkrab ...` è¿è¡Œ CLIã€‚

è¿™ä¼šæ‰§è¡Œï¼š

1. **é…ç½®æ–‡ä»¶éš”ç¦»**ï¼ˆå…¨å±€ `--dev`ï¼‰
   - `krabkrab_PROFILE=dev`
   - `krabkrab_STATE_DIR=~/.krabkrab-dev`
   - `krabkrab_CONFIG_PATH=~/.krabkrab-dev/krabkrab.json`
   - `krabkrab_GATEWAY_PORT=19001`ï¼ˆæµè§ˆå™¨/ç”»å¸ƒç›¸åº”ç§»åŠ¨ï¼‰

2. **Dev å¼•å¯¼**ï¼ˆ`gateway --dev`ï¼‰
   - å¦‚æžœç¼ºå¤±åˆ™å†™å…¥æœ€å°é…ç½®ï¼ˆ`gateway.mode=local`ï¼Œç»‘å®š loopbackï¼‰ã€‚
   - å°† `agent.workspace` è®¾ç½®ä¸º dev å·¥ä½œåŒºã€‚
   - è®¾ç½® `agent.skipBootstrap=true`ï¼ˆæ—  BOOTSTRAP.mdï¼‰ã€‚
   - å¦‚æžœç¼ºå¤±åˆ™å¡«å……å·¥ä½œåŒºæ–‡ä»¶ï¼š
     `AGENTS.md`ã€`SOUL.md`ã€`TOOLS.md`ã€`IDENTITY.md`ã€`USER.md`ã€`HEARTBEAT.md`ã€‚
   - é»˜è®¤èº«ä»½ï¼š**C3â€‘PO**ï¼ˆç¤¼ä»ªæœºå™¨äººï¼‰ã€‚
   - åœ¨ dev æ¨¡å¼ä¸‹è·³è¿‡æ¸ é“æä¾›å•†ï¼ˆ`krabkrab_SKIP_CHANNELS=1`ï¼‰ã€‚

é‡ç½®æµç¨‹ï¼ˆå…¨æ–°å¼€å§‹ï¼‰ï¼š

```bash
pnpm gateway:dev:reset
```

æ³¨æ„ï¼š`--dev` æ˜¯**å…¨å±€**é…ç½®æ–‡ä»¶æ ‡å¿—ï¼Œä¼šè¢«æŸäº›è¿è¡Œå™¨åžæŽ‰ã€‚
å¦‚æžœä½ éœ€è¦æ˜Žç¡®æ‹¼å†™ï¼Œè¯·ä½¿ç”¨çŽ¯å¢ƒå˜é‡å½¢å¼ï¼š

```bash
krabkrab_PROFILE=dev krabkrab gateway --dev --reset
```

`--reset` æ¸…é™¤é…ç½®ã€å‡­è¯ã€ä¼šè¯å’Œ dev å·¥ä½œåŒºï¼ˆä½¿ç”¨ `trash`ï¼Œè€Œéž `rm`ï¼‰ï¼Œç„¶åŽé‡æ–°åˆ›å»ºé»˜è®¤çš„ dev è®¾ç½®ã€‚

æç¤ºï¼šå¦‚æžœéž dev Gateway ç½‘å…³å·²åœ¨è¿è¡Œï¼ˆlaunchd/systemdï¼‰ï¼Œè¯·å…ˆåœæ­¢å®ƒï¼š

```bash
krabkrab gateway stop
```

## åŽŸå§‹æµæ—¥å¿—ï¼ˆKrabKrabï¼‰

KrabKrab å¯ä»¥åœ¨ä»»ä½•è¿‡æ»¤/æ ¼å¼åŒ–ä¹‹å‰è®°å½•**åŽŸå§‹åŠ©æ‰‹æµ**ã€‚
è¿™æ˜¯æŸ¥çœ‹æŽ¨ç†æ˜¯å¦ä½œä¸ºçº¯æ–‡æœ¬å¢žé‡åˆ°è¾¾ï¼ˆæˆ–ä½œä¸ºå•ç‹¬çš„æ€è€ƒå—ï¼‰çš„æœ€ä½³æ–¹å¼ã€‚

é€šè¿‡ CLI å¯ç”¨ï¼š

```bash
pnpm gateway:watch --force --raw-stream
```

å¯é€‰è·¯å¾„è¦†ç›–ï¼š

```bash
pnpm gateway:watch --force --raw-stream --raw-stream-path ~/.krabkrab/logs/raw-stream.jsonl
```

ç­‰æ•ˆçŽ¯å¢ƒå˜é‡ï¼š

```bash
krabkrab_RAW_STREAM=1
krabkrab_RAW_STREAM_PATH=~/.krabkrab/logs/raw-stream.jsonl
```

é»˜è®¤æ–‡ä»¶ï¼š

`~/.krabkrab/logs/raw-stream.jsonl`

## åŽŸå§‹å—æ—¥å¿—ï¼ˆpi-monoï¼‰

è¦åœ¨è§£æžä¸ºå—ä¹‹å‰æ•èŽ·**åŽŸå§‹ OpenAI å…¼å®¹å—**ï¼Œpi-mono æš´éœ²äº†ä¸€ä¸ªå•ç‹¬çš„æ—¥å¿—è®°å½•å™¨ï¼š

```bash
PI_RAW_STREAM=1
```

å¯é€‰è·¯å¾„ï¼š

```bash
PI_RAW_STREAM_PATH=~/.pi-mono/logs/raw-openai-completions.jsonl
```

é»˜è®¤æ–‡ä»¶ï¼š

`~/.pi-mono/logs/raw-openai-completions.jsonl`

> æ³¨æ„ï¼šè¿™ä»…ç”±ä½¿ç”¨ pi-mono çš„ `openai-completions` æä¾›å•†çš„è¿›ç¨‹å‘å‡ºã€‚

## å®‰å…¨æ³¨æ„äº‹é¡¹

- åŽŸå§‹æµæ—¥å¿—å¯èƒ½åŒ…å«å®Œæ•´æç¤ºã€å·¥å…·è¾“å‡ºå’Œç”¨æˆ·æ•°æ®ã€‚
- ä¿æŒæ—¥å¿—åœ¨æœ¬åœ°å¹¶åœ¨è°ƒè¯•åŽåˆ é™¤å®ƒä»¬ã€‚
- å¦‚æžœä½ åˆ†äº«æ—¥å¿—ï¼Œè¯·å…ˆæ¸…é™¤å¯†é’¥å’Œä¸ªäººèº«ä»½ä¿¡æ¯ã€‚

