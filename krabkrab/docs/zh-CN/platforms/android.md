---
read_when:
  - é…å¯¹æˆ–é‡æ–°è¿žæŽ¥ Android èŠ‚ç‚¹
  - è°ƒè¯• Android Gateway ç½‘å…³å‘çŽ°æˆ–è®¤è¯
  - éªŒè¯è·¨å®¢æˆ·ç«¯çš„èŠå¤©åŽ†å²ä¸€è‡´æ€§
summary: Android åº”ç”¨ï¼ˆèŠ‚ç‚¹ï¼‰ï¼šè¿žæŽ¥æ“ä½œæ‰‹å†Œ + Canvas/Chat/Camera
title: Android åº”ç”¨
x-i18n:
  generated_at: "2026-02-03T07:51:34Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 9cd02f12065ce2bc483379c9afd7537489d9076094f4a412cf9f21ccc47f0e38
  source_path: platforms/android.md
  workflow: 15
---

# Android åº”ç”¨ï¼ˆèŠ‚ç‚¹ï¼‰

## æ”¯æŒæ¦‚è§ˆ

- è§’è‰²ï¼šé…å¥—èŠ‚ç‚¹åº”ç”¨ï¼ˆAndroid ä¸æ‰˜ç®¡ Gateway ç½‘å…³ï¼‰ã€‚
- éœ€è¦ Gateway ç½‘å…³ï¼šæ˜¯ï¼ˆåœ¨ macOSã€Linux æˆ–é€šè¿‡ WSL2 çš„ Windows ä¸Šè¿è¡Œï¼‰ã€‚
- å®‰è£…ï¼š[å…¥é—¨æŒ‡å—](/start/getting-started) + [é…å¯¹](/gateway/pairing)ã€‚
- Gateway ç½‘å…³ï¼š[æ“ä½œæ‰‹å†Œ](/gateway) + [é…ç½®](/gateway/configuration)ã€‚
  - åè®®ï¼š[Gateway ç½‘å…³åè®®](/gateway/protocol)ï¼ˆèŠ‚ç‚¹ + æŽ§åˆ¶å¹³é¢ï¼‰ã€‚

## ç³»ç»ŸæŽ§åˆ¶

ç³»ç»ŸæŽ§åˆ¶ï¼ˆlaunchd/systemdï¼‰ä½äºŽ Gateway ç½‘å…³ä¸»æœºä¸Šã€‚å‚è§ [Gateway ç½‘å…³](/gateway)ã€‚

## è¿žæŽ¥æ“ä½œæ‰‹å†Œ

Android èŠ‚ç‚¹åº”ç”¨ â‡„ï¼ˆmDNS/NSD + WebSocketï¼‰â‡„ **Gateway ç½‘å…³**

Android ç›´æŽ¥è¿žæŽ¥åˆ° Gateway ç½‘å…³ WebSocketï¼ˆé»˜è®¤ `ws://<host>:18789`ï¼‰å¹¶ä½¿ç”¨ Gateway ç½‘å…³æ‹¥æœ‰çš„é…å¯¹ã€‚

### å‰ç½®æ¡ä»¶

- ä½ å¯ä»¥åœ¨"ä¸»"æœºå™¨ä¸Šè¿è¡Œ Gateway ç½‘å…³ã€‚
- Android è®¾å¤‡/æ¨¡æ‹Ÿå™¨å¯ä»¥è®¿é—® Gateway ç½‘å…³ WebSocketï¼š
  - ä½¿ç”¨ mDNS/NSD çš„åŒä¸€å±€åŸŸç½‘ï¼Œ**æˆ–**
  - ä½¿ç”¨ Wide-Area Bonjour / unicast DNS-SD çš„åŒä¸€ Tailscale tailnetï¼ˆè§ä¸‹æ–‡ï¼‰ï¼Œ**æˆ–**
  - æ‰‹åŠ¨ Gateway ç½‘å…³ä¸»æœº/ç«¯å£ï¼ˆå›žé€€æ–¹æ¡ˆï¼‰
- ä½ å¯ä»¥åœ¨ Gateway ç½‘å…³æœºå™¨ä¸Šè¿è¡Œ CLIï¼ˆ`krabkrab`ï¼‰ï¼ˆæˆ–é€šè¿‡ SSHï¼‰ã€‚

### 1ï¼‰å¯åŠ¨ Gateway ç½‘å…³

```bash
krabkrab gateway --port 18789 --verbose
```

åœ¨æ—¥å¿—ä¸­ç¡®è®¤ä½ çœ‹åˆ°ç±»ä¼¼å†…å®¹ï¼š

- `listening on ws://0.0.0.0:18789`

å¯¹äºŽä»… tailnet è®¾ç½®ï¼ˆæŽ¨èç”¨äºŽç»´ä¹Ÿçº³ â‡„ ä¼¦æ•¦ï¼‰ï¼Œå°† Gateway ç½‘å…³ç»‘å®šåˆ° tailnet IPï¼š

- åœ¨ Gateway ç½‘å…³ä¸»æœºçš„ `~/.krabkrab/krabkrab.json` ä¸­è®¾ç½® `gateway.bind: "tailnet"`ã€‚
- é‡å¯ Gateway ç½‘å…³ / macOS èœå•æ åº”ç”¨ã€‚

### 2ï¼‰éªŒè¯å‘çŽ°ï¼ˆå¯é€‰ï¼‰

ä»Ž Gateway ç½‘å…³æœºå™¨ï¼š

```bash
dns-sd -B _krabkrab-gw._tcp local.
```

æ›´å¤šè°ƒè¯•è¯´æ˜Žï¼š[Bonjour](/gateway/bonjour)ã€‚

#### é€šè¿‡ unicast DNS-SD çš„ Tailnetï¼ˆç»´ä¹Ÿçº³ â‡„ ä¼¦æ•¦ï¼‰å‘çŽ°

Android NSD/mDNS å‘çŽ°æ— æ³•è·¨ç½‘ç»œã€‚å¦‚æžœä½ çš„ Android èŠ‚ç‚¹å’Œ Gateway ç½‘å…³åœ¨ä¸åŒç½‘ç»œä½†é€šè¿‡ Tailscale è¿žæŽ¥ï¼Œè¯·æ”¹ç”¨ Wide-Area Bonjour / unicast DNS-SDï¼š

1. åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè®¾ç½® DNS-SD åŒºåŸŸï¼ˆç¤ºä¾‹ `krabkrab.internal.`ï¼‰å¹¶å‘å¸ƒ `_krabkrab-gw._tcp` è®°å½•ã€‚
2. é…ç½® Tailscale split DNSï¼Œå°†ä½ é€‰æ‹©çš„åŸŸæŒ‡å‘è¯¥ DNS æœåŠ¡å™¨ã€‚

è¯¦æƒ…å’Œç¤ºä¾‹ CoreDNS é…ç½®ï¼š[Bonjour](/gateway/bonjour)ã€‚

### 3ï¼‰ä»Ž Android è¿žæŽ¥

åœ¨ Android åº”ç”¨ä¸­ï¼š

- åº”ç”¨é€šè¿‡**å‰å°æœåŠ¡**ï¼ˆæŒä¹…é€šçŸ¥ï¼‰ä¿æŒ Gateway ç½‘å…³è¿žæŽ¥æ´»åŠ¨ã€‚
- æ‰“å¼€**è®¾ç½®**ã€‚
- åœ¨**å‘çŽ°çš„ Gateway ç½‘å…³**ä¸‹ï¼Œé€‰æ‹©ä½ çš„ Gateway ç½‘å…³å¹¶ç‚¹å‡»**è¿žæŽ¥**ã€‚
- å¦‚æžœ mDNS è¢«é˜»æ­¢ï¼Œä½¿ç”¨**é«˜çº§ â†’ æ‰‹åŠ¨ Gateway ç½‘å…³**ï¼ˆä¸»æœº + ç«¯å£ï¼‰å¹¶**è¿žæŽ¥ï¼ˆæ‰‹åŠ¨ï¼‰**ã€‚

é¦–æ¬¡æˆåŠŸé…å¯¹åŽï¼ŒAndroid åœ¨å¯åŠ¨æ—¶è‡ªåŠ¨é‡è¿žï¼š

- æ‰‹åŠ¨ç«¯ç‚¹ï¼ˆå¦‚æžœå¯ç”¨ï¼‰ï¼Œå¦åˆ™
- ä¸Šæ¬¡å‘çŽ°çš„ Gateway ç½‘å…³ï¼ˆå°½åŠ›è€Œä¸ºï¼‰ã€‚

### 4ï¼‰æ‰¹å‡†é…å¯¹ï¼ˆCLIï¼‰

åœ¨ Gateway ç½‘å…³æœºå™¨ä¸Šï¼š

```bash
krabkrab nodes pending
krabkrab nodes approve <requestId>
```

é…å¯¹è¯¦æƒ…ï¼š[Gateway ç½‘å…³é…å¯¹](/gateway/pairing)ã€‚

### 5ï¼‰éªŒè¯èŠ‚ç‚¹å·²è¿žæŽ¥

- é€šè¿‡èŠ‚ç‚¹çŠ¶æ€ï¼š
  ```bash
  krabkrab nodes status
  ```
- é€šè¿‡ Gateway ç½‘å…³ï¼š
  ```bash
  krabkrab gateway call node.list --params "{}"
  ```

### 6ï¼‰èŠå¤© + åŽ†å²

Android èŠ‚ç‚¹çš„ Chat é¢æ¿ä½¿ç”¨ Gateway ç½‘å…³çš„**ä¸»ä¼šè¯é”®**ï¼ˆ`main`ï¼‰ï¼Œå› æ­¤åŽ†å²å’Œå›žå¤ä¸Ž WebChat å’Œå…¶ä»–å®¢æˆ·ç«¯å…±äº«ï¼š

- åŽ†å²ï¼š`chat.history`
- å‘é€ï¼š`chat.send`
- æŽ¨é€æ›´æ–°ï¼ˆå°½åŠ›è€Œä¸ºï¼‰ï¼š`chat.subscribe` â†’ `event:"chat"`

### 7ï¼‰Canvas + æ‘„åƒå¤´

#### Gateway ç½‘å…³ Canvas ä¸»æœºï¼ˆæŽ¨èç”¨äºŽ web å†…å®¹ï¼‰

å¦‚æžœä½ æƒ³è®©èŠ‚ç‚¹æ˜¾ç¤ºæ™ºèƒ½ä½“å¯ä»¥åœ¨ç£ç›˜ä¸Šç¼–è¾‘çš„çœŸå®ž HTML/CSS/JSï¼Œè¯·å°†èŠ‚ç‚¹æŒ‡å‘ Gateway ç½‘å…³ canvas ä¸»æœºã€‚

æ³¨æ„ï¼šèŠ‚ç‚¹ä½¿ç”¨ `canvasHost.port`ï¼ˆé»˜è®¤ `18793`ï¼‰ä¸Šçš„ç‹¬ç«‹ canvas ä¸»æœºã€‚

1. åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šåˆ›å»º `~/.krabkrab/workspace/canvas/index.html`ã€‚

2. å°†èŠ‚ç‚¹å¯¼èˆªåˆ°å®ƒï¼ˆå±€åŸŸç½‘ï¼‰ï¼š

```bash
krabkrab nodes invoke --node "<Android Node>" --command canvas.navigate --params '{"url":"http://<gateway-hostname>.local:18793/__krabkrab__/canvas/"}'
```

Tailnetï¼ˆå¯é€‰ï¼‰ï¼šå¦‚æžœä¸¤ä¸ªè®¾å¤‡éƒ½åœ¨ Tailscale ä¸Šï¼Œä½¿ç”¨ MagicDNS åç§°æˆ– tailnet IP è€Œä¸æ˜¯ `.local`ï¼Œä¾‹å¦‚ `http://<gateway-magicdns>:18793/__krabkrab__/canvas/`ã€‚

æ­¤æœåŠ¡å™¨å°†å®žæ—¶é‡è½½å®¢æˆ·ç«¯æ³¨å…¥ HTML å¹¶åœ¨æ–‡ä»¶æ›´æ”¹æ—¶é‡æ–°åŠ è½½ã€‚
A2UI ä¸»æœºä½äºŽ `http://<gateway-host>:18793/__krabkrab__/a2ui/`ã€‚

Canvas å‘½ä»¤ï¼ˆä»…å‰å°ï¼‰ï¼š

- `canvas.eval`ã€`canvas.snapshot`ã€`canvas.navigate`ï¼ˆä½¿ç”¨ `{"url":""}` æˆ– `{"url":"/"}` è¿”å›žé»˜è®¤è„šæ‰‹æž¶ï¼‰ã€‚`canvas.snapshot` è¿”å›ž `{ format, base64 }`ï¼ˆé»˜è®¤ `format="jpeg"`ï¼‰ã€‚
- A2UIï¼š`canvas.a2ui.push`ã€`canvas.a2ui.reset`ï¼ˆ`canvas.a2ui.pushJSONL` é—ç•™åˆ«åï¼‰

æ‘„åƒå¤´å‘½ä»¤ï¼ˆä»…å‰å°ï¼›æƒé™é™åˆ¶ï¼‰ï¼š

- `camera.snap`ï¼ˆjpgï¼‰
- `camera.clip`ï¼ˆmp4ï¼‰

å‚è§ [Camera èŠ‚ç‚¹](/nodes/camera) äº†è§£å‚æ•°å’Œ CLI åŠ©æ‰‹ã€‚

