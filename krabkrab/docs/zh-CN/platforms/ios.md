---
read_when:
  - é…å¯¹æˆ–é‡æ–°è¿žæŽ¥ iOS èŠ‚ç‚¹
  - ä»Žæºç è¿è¡Œ iOS åº”ç”¨
  - è°ƒè¯• Gateway ç½‘å…³å‘çŽ°æˆ– canvas å‘½ä»¤
summary: iOS èŠ‚ç‚¹åº”ç”¨ï¼šè¿žæŽ¥åˆ° Gateway ç½‘å…³ã€é…å¯¹ã€canvas å’Œæ•…éšœæŽ’é™¤
title: iOS åº”ç”¨
x-i18n:
  generated_at: "2026-02-03T07:52:17Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 692eebdc82e4bb8dc221bcbabf6a344a861a839fc377f1aeeb6eecaa4917a232
  source_path: platforms/ios.md
  workflow: 15
---

# iOS åº”ç”¨ï¼ˆèŠ‚ç‚¹ï¼‰

å¯ç”¨æ€§ï¼šå†…éƒ¨é¢„è§ˆã€‚iOS åº”ç”¨å°šæœªå…¬å¼€åˆ†å‘ã€‚

## åŠŸèƒ½

- é€šè¿‡ WebSocketï¼ˆLAN æˆ– tailnetï¼‰è¿žæŽ¥åˆ° Gateway ç½‘å…³ã€‚
- æš´éœ²èŠ‚ç‚¹èƒ½åŠ›ï¼šCanvasã€å±å¹•å¿«ç…§ã€ç›¸æœºæ•èŽ·ã€ä½ç½®ã€å¯¹è¯æ¨¡å¼ã€è¯­éŸ³å”¤é†’ã€‚
- æŽ¥æ”¶ `node.invoke` å‘½ä»¤å¹¶æŠ¥å‘ŠèŠ‚ç‚¹çŠ¶æ€äº‹ä»¶ã€‚

## è¦æ±‚

- Gateway ç½‘å…³è¿è¡Œåœ¨å¦ä¸€å°è®¾å¤‡ä¸Šï¼ˆmacOSã€Linux æˆ–é€šè¿‡ WSL2 çš„ Windowsï¼‰ã€‚
- ç½‘ç»œè·¯å¾„ï¼š
  - é€šè¿‡ Bonjour çš„åŒä¸€ LANï¼Œ**æˆ–**
  - é€šè¿‡å•æ’­ DNS-SD çš„ Tailnetï¼ˆç¤ºä¾‹åŸŸï¼š`krabkrab.internal.`ï¼‰ï¼Œ**æˆ–**
  - æ‰‹åŠ¨ä¸»æœº/ç«¯å£ï¼ˆå¤‡é€‰ï¼‰ã€‚

## å¿«é€Ÿå¼€å§‹ï¼ˆé…å¯¹ + è¿žæŽ¥ï¼‰

1. å¯åŠ¨ Gateway ç½‘å…³ï¼š

```bash
krabkrab gateway --port 18789
```

2. åœ¨ iOS åº”ç”¨ä¸­ï¼Œæ‰“å¼€è®¾ç½®å¹¶é€‰æ‹©ä¸€ä¸ªå·²å‘çŽ°çš„ Gateway ç½‘å…³ï¼ˆæˆ–å¯ç”¨æ‰‹åŠ¨ä¸»æœºå¹¶è¾“å…¥ä¸»æœº/ç«¯å£ï¼‰ã€‚

3. åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šæ‰¹å‡†é…å¯¹è¯·æ±‚ï¼š

```bash
krabkrab nodes pending
krabkrab nodes approve <requestId>
```

4. éªŒè¯è¿žæŽ¥ï¼š

```bash
krabkrab nodes status
krabkrab gateway call node.list --params "{}"
```

## å‘çŽ°è·¯å¾„

### Bonjourï¼ˆLANï¼‰

Gateway ç½‘å…³åœ¨ `local.` ä¸Šå¹¿æ’­ `_krabkrab-gw._tcp`ã€‚iOS åº”ç”¨ä¼šè‡ªåŠ¨åˆ—å‡ºè¿™äº›ã€‚

### Tailnetï¼ˆè·¨ç½‘ç»œï¼‰

å¦‚æžœ mDNS è¢«é˜»æ­¢ï¼Œä½¿ç”¨å•æ’­ DNS-SD åŒºåŸŸï¼ˆé€‰æ‹©ä¸€ä¸ªåŸŸï¼›ç¤ºä¾‹ï¼š`krabkrab.internal.`ï¼‰å’Œ Tailscale åˆ†å‰² DNSã€‚
å‚è§ [Bonjour](/gateway/bonjour) äº†è§£ CoreDNS ç¤ºä¾‹ã€‚

### æ‰‹åŠ¨ä¸»æœº/ç«¯å£

åœ¨è®¾ç½®ä¸­ï¼Œå¯ç”¨**æ‰‹åŠ¨ä¸»æœº**å¹¶è¾“å…¥ Gateway ç½‘å…³ä¸»æœº + ç«¯å£ï¼ˆé»˜è®¤ `18789`ï¼‰ã€‚

## Canvas + A2UI

iOS èŠ‚ç‚¹æ¸²æŸ“ä¸€ä¸ª WKWebView canvasã€‚ä½¿ç”¨ `node.invoke` æ¥é©±åŠ¨å®ƒï¼š

```bash
krabkrab nodes invoke --node "iOS Node" --command canvas.navigate --params '{"url":"http://<gateway-host>:18793/__krabkrab__/canvas/"}'
```

æ³¨æ„äº‹é¡¹ï¼š

- Gateway ç½‘å…³ canvas ä¸»æœºæœåŠ¡äºŽ `/__krabkrab__/canvas/` å’Œ `/__krabkrab__/a2ui/`ã€‚
- å½“å¹¿æ’­äº† canvas ä¸»æœº URL æ—¶ï¼ŒiOS èŠ‚ç‚¹åœ¨è¿žæŽ¥æ—¶è‡ªåŠ¨å¯¼èˆªåˆ° A2UIã€‚
- ä½¿ç”¨ `canvas.navigate` å’Œ `{"url":""}` è¿”å›žå†…ç½®è„šæ‰‹æž¶ã€‚

### Canvas eval / snapshot

```bash
krabkrab nodes invoke --node "iOS Node" --command canvas.eval --params '{"javaScript":"(() => { const {ctx} = window.__krabkrab; ctx.clearRect(0,0,innerWidth,innerHeight); ctx.lineWidth=6; ctx.strokeStyle=\"#ff2d55\"; ctx.beginPath(); ctx.moveTo(40,40); ctx.lineTo(innerWidth-40, innerHeight-40); ctx.stroke(); return \"ok\"; })()"}'
```

```bash
krabkrab nodes invoke --node "iOS Node" --command canvas.snapshot --params '{"maxWidth":900,"format":"jpeg"}'
```

## è¯­éŸ³å”¤é†’ + å¯¹è¯æ¨¡å¼

- è¯­éŸ³å”¤é†’å’Œå¯¹è¯æ¨¡å¼åœ¨è®¾ç½®ä¸­å¯ç”¨ã€‚
- iOS å¯èƒ½ä¼šæš‚åœåŽå°éŸ³é¢‘ï¼›å½“åº”ç”¨ä¸æ´»è·ƒæ—¶ï¼Œå°†è¯­éŸ³åŠŸèƒ½è§†ä¸ºå°½åŠ›è€Œä¸ºã€‚

## å¸¸è§é”™è¯¯

- `NODE_BACKGROUND_UNAVAILABLE`ï¼šå°† iOS åº”ç”¨å¸¦åˆ°å‰å°ï¼ˆcanvas/ç›¸æœº/å±å¹•å‘½ä»¤éœ€è¦å®ƒï¼‰ã€‚
- `A2UI_HOST_NOT_CONFIGURED`ï¼šGateway ç½‘å…³æœªå¹¿æ’­ canvas ä¸»æœº URLï¼›æ£€æŸ¥ [Gateway ç½‘å…³é…ç½®](/gateway/configuration) ä¸­çš„ `canvasHost`ã€‚
- é…å¯¹æç¤ºä»Žæœªå‡ºçŽ°ï¼šè¿è¡Œ `krabkrab nodes pending` å¹¶æ‰‹åŠ¨æ‰¹å‡†ã€‚
- é‡æ–°å®‰è£…åŽé‡è¿žå¤±è´¥ï¼šé’¥åŒ™ä¸²é…å¯¹ä»¤ç‰Œå·²è¢«æ¸…é™¤ï¼›é‡æ–°é…å¯¹èŠ‚ç‚¹ã€‚

## ç›¸å…³æ–‡æ¡£

- [é…å¯¹](/gateway/pairing)
- [è®¾å¤‡å‘çŽ°](/gateway/discovery)
- [Bonjour](/gateway/bonjour)

