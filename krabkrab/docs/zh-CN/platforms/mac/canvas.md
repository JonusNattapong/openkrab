---
read_when:
  - å®žçŽ° macOS Canvas é¢æ¿
  - ä¸ºå¯è§†åŒ–å·¥ä½œåŒºæ·»åŠ æ™ºèƒ½ä½“æŽ§åˆ¶
  - è°ƒè¯• WKWebView canvas åŠ è½½
summary: é€šè¿‡ WKWebView + è‡ªå®šä¹‰ URL æ–¹æ¡ˆåµŒå…¥çš„æ™ºèƒ½ä½“æŽ§åˆ¶ Canvas é¢æ¿
title: Canvas
x-i18n:
  generated_at: "2026-02-03T07:52:39Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: e39caa21542e839d9f59ad0bf7ecefb379225ed7e8f00cd59131d188f193bec6
  source_path: platforms/mac/canvas.md
  workflow: 15
---

# Canvasï¼ˆmacOS åº”ç”¨ï¼‰

macOS åº”ç”¨ä½¿ç”¨ `WKWebView` åµŒå…¥ä¸€ä¸ªæ™ºèƒ½ä½“æŽ§åˆ¶çš„ **Canvas é¢æ¿**ã€‚å®ƒæ˜¯ä¸€ä¸ªç”¨äºŽ HTML/CSS/JSã€A2UI å’Œå°åž‹äº¤äº’å¼ç•Œé¢çš„è½»é‡çº§å¯è§†åŒ–å·¥ä½œåŒºã€‚

## Canvas å­˜å‚¨ä½ç½®

Canvas çŠ¶æ€å­˜å‚¨åœ¨ Application Support ä¸‹ï¼š

- `~/Library/Application Support/KrabKrab/canvas/<session>/...`

Canvas é¢æ¿é€šè¿‡**è‡ªå®šä¹‰ URL æ–¹æ¡ˆ**æä¾›è¿™äº›æ–‡ä»¶ï¼š

- `krabkrab-canvas://<session>/<path>`

ç¤ºä¾‹ï¼š

- `krabkrab-canvas://main/` â†’ `<canvasRoot>/main/index.html`
- `krabkrab-canvas://main/assets/app.css` â†’ `<canvasRoot>/main/assets/app.css`
- `krabkrab-canvas://main/widgets/todo/` â†’ `<canvasRoot>/main/widgets/todo/index.html`

å¦‚æžœæ ¹ç›®å½•ä¸‹æ²¡æœ‰ `index.html`ï¼Œåº”ç”¨ä¼šæ˜¾ç¤ºä¸€ä¸ª**å†…ç½®è„šæ‰‹æž¶é¡µé¢**ã€‚

## é¢æ¿è¡Œä¸º

- æ— è¾¹æ¡†ã€å¯è°ƒæ•´å¤§å°çš„é¢æ¿ï¼Œé”šå®šåœ¨èœå•æ ï¼ˆæˆ–é¼ æ ‡å…‰æ ‡ï¼‰é™„è¿‘ã€‚
- è®°ä½æ¯ä¸ªä¼šè¯çš„å¤§å°/ä½ç½®ã€‚
- å½“æœ¬åœ° canvas æ–‡ä»¶æ›´æ”¹æ—¶è‡ªåŠ¨é‡æ–°åŠ è½½ã€‚
- ä¸€æ¬¡åªæ˜¾ç¤ºä¸€ä¸ª Canvas é¢æ¿ï¼ˆæ ¹æ®éœ€è¦åˆ‡æ¢ä¼šè¯ï¼‰ã€‚

å¯ä»¥ä»Žè®¾ç½® â†’ **å…è®¸ Canvas** ç¦ç”¨ Canvasã€‚ç¦ç”¨æ—¶ï¼Œcanvas èŠ‚ç‚¹å‘½ä»¤è¿”å›ž `CANVAS_DISABLED`ã€‚

## æ™ºèƒ½ä½“ API æŽ¥å£

Canvas é€šè¿‡ **Gateway ç½‘å…³ WebSocket** æš´éœ²ï¼Œå› æ­¤æ™ºèƒ½ä½“å¯ä»¥ï¼š

- æ˜¾ç¤º/éšè—é¢æ¿
- å¯¼èˆªåˆ°è·¯å¾„æˆ– URL
- æ‰§è¡Œ JavaScript
- æ•èŽ·å¿«ç…§å›¾åƒ

CLI ç¤ºä¾‹ï¼š

```bash
krabkrab nodes canvas present --node <id>
krabkrab nodes canvas navigate --node <id> --url "/"
krabkrab nodes canvas eval --node <id> --js "document.title"
krabkrab nodes canvas snapshot --node <id>
```

æ³¨æ„äº‹é¡¹ï¼š

- `canvas.navigate` æŽ¥å—**æœ¬åœ° canvas è·¯å¾„**ã€`http(s)` URL å’Œ `file://` URLã€‚
- å¦‚æžœä¼ é€’ `"/"`ï¼ŒCanvas ä¼šæ˜¾ç¤ºæœ¬åœ°è„šæ‰‹æž¶æˆ– `index.html`ã€‚

## Canvas ä¸­çš„ A2UI

A2UI ç”± Gateway ç½‘å…³ canvas ä¸»æœºæ‰˜ç®¡å¹¶åœ¨ Canvas é¢æ¿å†…æ¸²æŸ“ã€‚
å½“ Gateway ç½‘å…³å¹¿æ’­ Canvas ä¸»æœºæ—¶ï¼ŒmacOS åº”ç”¨åœ¨é¦–æ¬¡æ‰“å¼€æ—¶è‡ªåŠ¨å¯¼èˆªåˆ° A2UI ä¸»æœºé¡µé¢ã€‚

é»˜è®¤ A2UI ä¸»æœº URLï¼š

```
http://<gateway-host>:18793/__krabkrab__/a2ui/
```

### A2UI å‘½ä»¤ï¼ˆv0.8ï¼‰

Canvas ç›®å‰æŽ¥å— **A2UI v0.8** æœåŠ¡å™¨â†’å®¢æˆ·ç«¯æ¶ˆæ¯ï¼š

- `beginRendering`
- `surfaceUpdate`
- `dataModelUpdate`
- `deleteSurface`

`createSurface`ï¼ˆv0.9ï¼‰ä¸å—æ”¯æŒã€‚

CLI ç¤ºä¾‹ï¼š

```bash
cat > /tmp/a2ui-v0.8.jsonl <<'EOFA2'
{"surfaceUpdate":{"surfaceId":"main","components":[{"id":"root","component":{"Column":{"children":{"explicitList":["title","content"]}}}},{"id":"title","component":{"Text":{"text":{"literalString":"Canvas (A2UI v0.8)"},"usageHint":"h1"}}},{"id":"content","component":{"Text":{"text":{"literalString":"If you can read this, A2UI push works."},"usageHint":"body"}}}]}}
{"beginRendering":{"surfaceId":"main","root":"root"}}
EOFA2

krabkrab nodes canvas a2ui push --jsonl /tmp/a2ui-v0.8.jsonl --node <id>
```

å¿«é€Ÿæµ‹è¯•ï¼š

```bash
krabkrab nodes canvas a2ui push --node <id> --text "Hello from A2UI"
```

## ä»Ž Canvas è§¦å‘æ™ºèƒ½ä½“è¿è¡Œ

Canvas å¯ä»¥é€šè¿‡æ·±å±‚é“¾æŽ¥è§¦å‘æ–°çš„æ™ºèƒ½ä½“è¿è¡Œï¼š

- `krabkrab://agent?...`

ç¤ºä¾‹ï¼ˆåœ¨ JS ä¸­ï¼‰ï¼š

```js
window.location.href = "krabkrab://agent?message=Review%20this%20design";
```

é™¤éžæä¾›æœ‰æ•ˆå¯†é’¥ï¼Œå¦åˆ™åº”ç”¨ä¼šæç¤ºç¡®è®¤ã€‚

## å®‰å…¨æ³¨æ„äº‹é¡¹

- Canvas æ–¹æ¡ˆé˜»æ­¢ç›®å½•éåŽ†ï¼›æ–‡ä»¶å¿…é¡»ä½äºŽä¼šè¯æ ¹ç›®å½•ä¸‹ã€‚
- æœ¬åœ° Canvas å†…å®¹ä½¿ç”¨è‡ªå®šä¹‰æ–¹æ¡ˆï¼ˆä¸éœ€è¦ loopback æœåŠ¡å™¨ï¼‰ã€‚
- ä»…åœ¨æ˜¾å¼å¯¼èˆªæ—¶å…è®¸å¤–éƒ¨ `http(s)` URLã€‚

