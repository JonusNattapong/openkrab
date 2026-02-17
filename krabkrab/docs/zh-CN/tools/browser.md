---
read_when:
  - æ·»åŠ æ™ºèƒ½ä½“æŽ§åˆ¶çš„æµè§ˆå™¨è‡ªåŠ¨åŒ–
  - è°ƒè¯• krabkrab å¹²æ‰°ä½ è‡ªå·± Chrome çš„é—®é¢˜
  - åœ¨ macOS åº”ç”¨ä¸­å®žçŽ°æµè§ˆå™¨è®¾ç½®å’Œç”Ÿå‘½å‘¨æœŸç®¡ç†
summary: é›†æˆæµè§ˆå™¨æŽ§åˆ¶æœåŠ¡ + æ“ä½œå‘½ä»¤
title: æµè§ˆå™¨ï¼ˆKrabKrab æ‰˜ç®¡ï¼‰
x-i18n:
  generated_at: "2026-02-03T09:26:06Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: a868d040183436a1fb355130995e79782cb817b5ea298beaf1e1d2cb82e21c4c
  source_path: tools/browser.md
  workflow: 15
---

# æµè§ˆå™¨ï¼ˆkrabkrab æ‰˜ç®¡ï¼‰

KrabKrab å¯ä»¥è¿è¡Œä¸€ä¸ªç”±æ™ºèƒ½ä½“æŽ§åˆ¶çš„**ä¸“ç”¨ Chrome/Brave/Edge/Chromium é…ç½®æ–‡ä»¶**ã€‚
å®ƒä¸Žä½ çš„ä¸ªäººæµè§ˆå™¨éš”ç¦»ï¼Œé€šè¿‡ Gateway ç½‘å…³å†…éƒ¨çš„å°åž‹æœ¬åœ°æŽ§åˆ¶æœåŠ¡è¿›è¡Œç®¡ç†ï¼ˆä»…é™ loopbackï¼‰ã€‚

æ–°æ‰‹è§†è§’ï¼š

- æŠŠå®ƒæƒ³è±¡æˆä¸€ä¸ª**ç‹¬ç«‹çš„ã€ä»…ä¾›æ™ºèƒ½ä½“ä½¿ç”¨çš„æµè§ˆå™¨**ã€‚
- `krabkrab` é…ç½®æ–‡ä»¶**ä¸ä¼š**è§¦åŠä½ çš„ä¸ªäººæµè§ˆå™¨é…ç½®æ–‡ä»¶ã€‚
- æ™ºèƒ½ä½“å¯ä»¥åœ¨å®‰å…¨çš„é€šé“ä¸­**æ‰“å¼€æ ‡ç­¾é¡µã€è¯»å–é¡µé¢ã€ç‚¹å‡»å’Œè¾“å…¥**ã€‚
- é»˜è®¤çš„ `chrome` é…ç½®æ–‡ä»¶é€šè¿‡æ‰©å±•ä¸­ç»§ä½¿ç”¨**ç³»ç»Ÿé»˜è®¤çš„ Chromium æµè§ˆå™¨**ï¼›åˆ‡æ¢åˆ° `krabkrab` å¯ä½¿ç”¨éš”ç¦»çš„æ‰˜ç®¡æµè§ˆå™¨ã€‚

## åŠŸèƒ½æ¦‚è§ˆ

- ä¸€ä¸ªåä¸º **krabkrab** çš„ç‹¬ç«‹æµè§ˆå™¨é…ç½®æ–‡ä»¶ï¼ˆé»˜è®¤æ©™è‰²ä¸»é¢˜ï¼‰ã€‚
- ç¡®å®šæ€§æ ‡ç­¾é¡µæŽ§åˆ¶ï¼ˆåˆ—å‡º/æ‰“å¼€/èšç„¦/å…³é—­ï¼‰ã€‚
- æ™ºèƒ½ä½“æ“ä½œï¼ˆç‚¹å‡»/è¾“å…¥/æ‹–åŠ¨/é€‰æ‹©ï¼‰ã€å¿«ç…§ã€æˆªå›¾ã€PDFã€‚
- å¯é€‰çš„å¤šé…ç½®æ–‡ä»¶æ”¯æŒï¼ˆ`krabkrab`ã€`work`ã€`remote` ç­‰ï¼‰ã€‚

æ­¤æµè§ˆå™¨**ä¸æ˜¯**ä½ çš„æ—¥å¸¸æµè§ˆå™¨ã€‚å®ƒæ˜¯ä¸€ä¸ªå®‰å…¨ã€éš”ç¦»çš„ç•Œé¢ï¼Œç”¨äºŽæ™ºèƒ½ä½“è‡ªåŠ¨åŒ–å’ŒéªŒè¯ã€‚

## å¿«é€Ÿå¼€å§‹

```bash
krabkrab browser --browser-profile krabkrab status
krabkrab browser --browser-profile krabkrab start
krabkrab browser --browser-profile krabkrab open https://example.com
krabkrab browser --browser-profile krabkrab snapshot
```

å¦‚æžœå‡ºçŽ°"Browser disabled"ï¼Œè¯·åœ¨é…ç½®ä¸­å¯ç”¨å®ƒï¼ˆè§ä¸‹æ–‡ï¼‰å¹¶é‡å¯ Gateway ç½‘å…³ã€‚

## é…ç½®æ–‡ä»¶ï¼š`krabkrab` ä¸Ž `chrome`

- `krabkrab`ï¼šæ‰˜ç®¡çš„éš”ç¦»æµè§ˆå™¨ï¼ˆæ— éœ€æ‰©å±•ï¼‰ã€‚
- `chrome`ï¼šåˆ°ä½ **ç³»ç»Ÿæµè§ˆå™¨**çš„æ‰©å±•ä¸­ç»§ï¼ˆéœ€è¦å°† KrabKrab æ‰©å±•é™„åŠ åˆ°æ ‡ç­¾é¡µï¼‰ã€‚

å¦‚æžœä½ å¸Œæœ›é»˜è®¤ä½¿ç”¨æ‰˜ç®¡æ¨¡å¼ï¼Œè¯·è®¾ç½® `browser.defaultProfile: "krabkrab"`ã€‚

## é…ç½®

æµè§ˆå™¨è®¾ç½®ä½äºŽ `~/.krabkrab/krabkrab.json`ã€‚

```json5
{
  browser: {
    enabled: true, // default: true
    // cdpUrl: "http://127.0.0.1:18792", // legacy single-profile override
    remoteCdpTimeoutMs: 1500, // remote CDP HTTP timeout (ms)
    remoteCdpHandshakeTimeoutMs: 3000, // remote CDP WebSocket handshake timeout (ms)
    defaultProfile: "chrome",
    color: "#FF4500",
    headless: false,
    noSandbox: false,
    attachOnly: false,
    executablePath: "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
    profiles: {
      krabkrab: { cdpPort: 18800, color: "#FF4500" },
      work: { cdpPort: 18801, color: "#0066CC" },
      remote: { cdpUrl: "http://10.0.0.42:9222", color: "#00AA00" },
    },
  },
}
```

æ³¨æ„äº‹é¡¹ï¼š

- æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡ç»‘å®šåˆ° loopback ä¸Šçš„ç«¯å£ï¼Œè¯¥ç«¯å£ä»Ž `gateway.port` æ´¾ç”Ÿï¼ˆé»˜è®¤ï¼š`18791`ï¼Œå³ gateway + 2ï¼‰ã€‚ä¸­ç»§ä½¿ç”¨ä¸‹ä¸€ä¸ªç«¯å£ï¼ˆ`18792`ï¼‰ã€‚
- å¦‚æžœä½ è¦†ç›–äº† Gateway ç½‘å…³ç«¯å£ï¼ˆ`gateway.port` æˆ– `krabkrab_GATEWAY_PORT`ï¼‰ï¼Œæ´¾ç”Ÿçš„æµè§ˆå™¨ç«¯å£ä¼šç›¸åº”è°ƒæ•´ä»¥ä¿æŒåœ¨åŒä¸€"ç³»åˆ—"ä¸­ã€‚
- æœªè®¾ç½®æ—¶ï¼Œ`cdpUrl` é»˜è®¤ä¸ºä¸­ç»§ç«¯å£ã€‚
- `remoteCdpTimeoutMs` é€‚ç”¨äºŽè¿œç¨‹ï¼ˆéž loopbackï¼‰CDP å¯è¾¾æ€§æ£€æŸ¥ã€‚
- `remoteCdpHandshakeTimeoutMs` é€‚ç”¨äºŽè¿œç¨‹ CDP WebSocket å¯è¾¾æ€§æ£€æŸ¥ã€‚
- `attachOnly: true` è¡¨ç¤º"æ°¸ä¸å¯åŠ¨æœ¬åœ°æµè§ˆå™¨ï¼›ä»…åœ¨æµè§ˆå™¨å·²è¿è¡Œæ—¶é™„åŠ "ã€‚
- `color` + æ¯ä¸ªé…ç½®æ–‡ä»¶çš„ `color` ä¸ºæµè§ˆå™¨ UI ç€è‰²ï¼Œä»¥ä¾¿ä½ èƒ½çœ‹åˆ°å“ªä¸ªé…ç½®æ–‡ä»¶å¤„äºŽæ´»åŠ¨çŠ¶æ€ã€‚
- é»˜è®¤é…ç½®æ–‡ä»¶æ˜¯ `chrome`ï¼ˆæ‰©å±•ä¸­ç»§ï¼‰ã€‚ä½¿ç”¨ `defaultProfile: "krabkrab"` æ¥ä½¿ç”¨æ‰˜ç®¡æµè§ˆå™¨ã€‚
- è‡ªåŠ¨æ£€æµ‹é¡ºåºï¼šå¦‚æžœç³»ç»Ÿé»˜è®¤æµè§ˆå™¨æ˜¯åŸºäºŽ Chromium çš„åˆ™ä½¿ç”¨å®ƒï¼›å¦åˆ™ Chrome â†’ Brave â†’ Edge â†’ Chromium â†’ Chrome Canaryã€‚
- æœ¬åœ° `krabkrab` é…ç½®æ–‡ä»¶ä¼šè‡ªåŠ¨åˆ†é… `cdpPort`/`cdpUrl` â€” ä»…ä¸ºè¿œç¨‹ CDP è®¾ç½®è¿™äº›ã€‚

## ä½¿ç”¨ Braveï¼ˆæˆ–å…¶ä»–åŸºäºŽ Chromium çš„æµè§ˆå™¨ï¼‰

å¦‚æžœä½ çš„**ç³»ç»Ÿé»˜è®¤**æµè§ˆå™¨æ˜¯åŸºäºŽ Chromium çš„ï¼ˆChrome/Brave/Edge ç­‰ï¼‰ï¼ŒKrabKrab ä¼šè‡ªåŠ¨ä½¿ç”¨å®ƒã€‚è®¾ç½® `browser.executablePath` å¯è¦†ç›–è‡ªåŠ¨æ£€æµ‹ï¼š

CLI ç¤ºä¾‹ï¼š

```bash
krabkrab config set browser.executablePath "/usr/bin/google-chrome"
```

```json5
// macOS
{
  browser: {
    executablePath: "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser"
  }
}

// Windows
{
  browser: {
    executablePath: "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe"
  }
}

// Linux
{
  browser: {
    executablePath: "/usr/bin/brave-browser"
  }
}
```

## æœ¬åœ°æŽ§åˆ¶ä¸Žè¿œç¨‹æŽ§åˆ¶

- **æœ¬åœ°æŽ§åˆ¶ï¼ˆé»˜è®¤ï¼‰ï¼š** Gateway ç½‘å…³å¯åŠ¨ loopback æŽ§åˆ¶æœåŠ¡ï¼Œå¯ä»¥å¯åŠ¨æœ¬åœ°æµè§ˆå™¨ã€‚
- **è¿œç¨‹æŽ§åˆ¶ï¼ˆèŠ‚ç‚¹ä¸»æœºï¼‰ï¼š** åœ¨æœ‰æµè§ˆå™¨çš„æœºå™¨ä¸Šè¿è¡ŒèŠ‚ç‚¹ä¸»æœºï¼›Gateway ç½‘å…³å°†æµè§ˆå™¨æ“ä½œä»£ç†åˆ°è¯¥èŠ‚ç‚¹ã€‚
- **è¿œç¨‹ CDPï¼š** è®¾ç½® `browser.profiles.<name>.cdpUrl`ï¼ˆæˆ– `browser.cdpUrl`ï¼‰ä»¥é™„åŠ åˆ°è¿œç¨‹çš„åŸºäºŽ Chromium çš„æµè§ˆå™¨ã€‚åœ¨è¿™ç§æƒ…å†µä¸‹ï¼ŒKrabKrab ä¸ä¼šå¯åŠ¨æœ¬åœ°æµè§ˆå™¨ã€‚

è¿œç¨‹ CDP URL å¯ä»¥åŒ…å«è®¤è¯ä¿¡æ¯ï¼š

- æŸ¥è¯¢ä»¤ç‰Œï¼ˆä¾‹å¦‚ `https://provider.example?token=<token>`ï¼‰
- HTTP Basic è®¤è¯ï¼ˆä¾‹å¦‚ `https://user:pass@provider.example`ï¼‰

KrabKrab åœ¨è°ƒç”¨ `/json/*` ç«¯ç‚¹å’Œè¿žæŽ¥ CDP WebSocket æ—¶ä¼šä¿ç•™è®¤è¯ä¿¡æ¯ã€‚å»ºè®®ä½¿ç”¨çŽ¯å¢ƒå˜é‡æˆ–å¯†é’¥ç®¡ç†å™¨å­˜å‚¨ä»¤ç‰Œï¼Œè€Œä¸æ˜¯å°†å…¶æäº¤åˆ°é…ç½®æ–‡ä»¶ä¸­ã€‚

## èŠ‚ç‚¹æµè§ˆå™¨ä»£ç†ï¼ˆé›¶é…ç½®é»˜è®¤ï¼‰

å¦‚æžœä½ åœ¨æœ‰æµè§ˆå™¨çš„æœºå™¨ä¸Šè¿è¡Œ**èŠ‚ç‚¹ä¸»æœº**ï¼ŒKrabKrab å¯ä»¥è‡ªåŠ¨å°†æµè§ˆå™¨å·¥å…·è°ƒç”¨è·¯ç”±åˆ°è¯¥èŠ‚ç‚¹ï¼Œæ— éœ€ä»»ä½•é¢å¤–çš„æµè§ˆå™¨é…ç½®ã€‚è¿™æ˜¯è¿œç¨‹ Gateway ç½‘å…³çš„é»˜è®¤è·¯å¾„ã€‚

æ³¨æ„äº‹é¡¹ï¼š

- èŠ‚ç‚¹ä¸»æœºé€šè¿‡**ä»£ç†å‘½ä»¤**æš´éœ²å…¶æœ¬åœ°æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡å™¨ã€‚
- é…ç½®æ–‡ä»¶æ¥è‡ªèŠ‚ç‚¹è‡ªå·±çš„ `browser.profiles` é…ç½®ï¼ˆä¸Žæœ¬åœ°ç›¸åŒï¼‰ã€‚
- å¦‚æžœä¸éœ€è¦å¯ä»¥ç¦ç”¨ï¼š
  - åœ¨èŠ‚ç‚¹ä¸Šï¼š`nodeHost.browserProxy.enabled=false`
  - åœ¨ Gateway ç½‘å…³ä¸Šï¼š`gateway.nodes.browser.mode="off"`

## Browserlessï¼ˆæ‰˜ç®¡è¿œç¨‹ CDPï¼‰

[Browserless](https://browserless.io) æ˜¯ä¸€ä¸ªæ‰˜ç®¡çš„ Chromium æœåŠ¡ï¼Œé€šè¿‡ HTTPS æš´éœ² CDP ç«¯ç‚¹ã€‚ä½ å¯ä»¥å°† KrabKrab æµè§ˆå™¨é…ç½®æ–‡ä»¶æŒ‡å‘ Browserless åŒºåŸŸç«¯ç‚¹ï¼Œå¹¶ä½¿ç”¨ä½ çš„ API å¯†é’¥è¿›è¡Œè®¤è¯ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  browser: {
    enabled: true,
    defaultProfile: "browserless",
    remoteCdpTimeoutMs: 2000,
    remoteCdpHandshakeTimeoutMs: 4000,
    profiles: {
      browserless: {
        cdpUrl: "https://production-sfo.browserless.io?token=<BROWSERLESS_API_KEY>",
        color: "#00AA00",
      },
    },
  },
}
```

æ³¨æ„äº‹é¡¹ï¼š

- å°† `<BROWSERLESS_API_KEY>` æ›¿æ¢ä¸ºä½ çœŸå®žçš„ Browserless ä»¤ç‰Œã€‚
- é€‰æ‹©ä¸Žä½ çš„ Browserless è´¦æˆ·åŒ¹é…çš„åŒºåŸŸç«¯ç‚¹ï¼ˆè¯·å‚é˜…å…¶æ–‡æ¡£ï¼‰ã€‚

## å®‰å…¨æ€§

æ ¸å¿ƒç†å¿µï¼š

- æµè§ˆå™¨æŽ§åˆ¶ä»…é™ loopbackï¼›è®¿é—®é€šè¿‡ Gateway ç½‘å…³çš„è®¤è¯æˆ–èŠ‚ç‚¹é…å¯¹è¿›è¡Œã€‚
- å°† Gateway ç½‘å…³å’Œä»»ä½•èŠ‚ç‚¹ä¸»æœºä¿æŒåœ¨ç§æœ‰ç½‘ç»œä¸Šï¼ˆTailscaleï¼‰ï¼›é¿å…å…¬å¼€æš´éœ²ã€‚
- å°†è¿œç¨‹ CDP URL/ä»¤ç‰Œè§†ä¸ºæœºå¯†ï¼›ä¼˜å…ˆä½¿ç”¨çŽ¯å¢ƒå˜é‡æˆ–å¯†é’¥ç®¡ç†å™¨ã€‚

è¿œç¨‹ CDP æç¤ºï¼š

- å°½å¯èƒ½ä½¿ç”¨ HTTPS ç«¯ç‚¹å’ŒçŸ­æœŸä»¤ç‰Œã€‚
- é¿å…åœ¨é…ç½®æ–‡ä»¶ä¸­ç›´æŽ¥åµŒå…¥é•¿æœŸä»¤ç‰Œã€‚

## é…ç½®æ–‡ä»¶ï¼ˆå¤šæµè§ˆå™¨ï¼‰

KrabKrab æ”¯æŒå¤šä¸ªå‘½åé…ç½®æ–‡ä»¶ï¼ˆè·¯ç”±é…ç½®ï¼‰ã€‚é…ç½®æ–‡ä»¶å¯ä»¥æ˜¯ï¼š

- **krabkrab æ‰˜ç®¡**ï¼šå…·æœ‰ç‹¬ç«‹ç”¨æˆ·æ•°æ®ç›®å½•å’Œ CDP ç«¯å£çš„ä¸“ç”¨åŸºäºŽ Chromium çš„æµè§ˆå™¨å®žä¾‹
- **è¿œç¨‹**ï¼šæ˜¾å¼ CDP URLï¼ˆåœ¨å…¶ä»–åœ°æ–¹è¿è¡Œçš„åŸºäºŽ Chromium çš„æµè§ˆå™¨ï¼‰
- **æ‰©å±•ä¸­ç»§**ï¼šé€šè¿‡æœ¬åœ°ä¸­ç»§ + Chrome æ‰©å±•è®¿é—®ä½ çŽ°æœ‰çš„ Chrome æ ‡ç­¾é¡µ

é»˜è®¤å€¼ï¼š

- å¦‚æžœç¼ºå°‘ `krabkrab` é…ç½®æ–‡ä»¶ï¼Œä¼šè‡ªåŠ¨åˆ›å»ºã€‚
- `chrome` é…ç½®æ–‡ä»¶æ˜¯å†…ç½®çš„ï¼Œç”¨äºŽ Chrome æ‰©å±•ä¸­ç»§ï¼ˆé»˜è®¤æŒ‡å‘ `http://127.0.0.1:18792`ï¼‰ã€‚
- æœ¬åœ° CDP ç«¯å£é»˜è®¤ä»Ž **18800â€“18899** åˆ†é…ã€‚
- åˆ é™¤é…ç½®æ–‡ä»¶ä¼šå°†å…¶æœ¬åœ°æ•°æ®ç›®å½•ç§»è‡³å›žæ”¶ç«™ã€‚

æ‰€æœ‰æŽ§åˆ¶ç«¯ç‚¹æŽ¥å— `?profile=<name>`ï¼›CLI ä½¿ç”¨ `--browser-profile`ã€‚

## Chrome æ‰©å±•ä¸­ç»§ï¼ˆä½¿ç”¨ä½ çŽ°æœ‰çš„ Chromeï¼‰

KrabKrab è¿˜å¯ä»¥é€šè¿‡æœ¬åœ° CDP ä¸­ç»§ + Chrome æ‰©å±•é©±åŠ¨**ä½ çŽ°æœ‰çš„ Chrome æ ‡ç­¾é¡µ**ï¼ˆæ— éœ€å•ç‹¬çš„"krabkrab"Chrome å®žä¾‹ï¼‰ã€‚

å®Œæ•´æŒ‡å—ï¼š[Chrome æ‰©å±•](/tools/chrome-extension)

æµç¨‹ï¼š

- Gateway ç½‘å…³åœ¨æœ¬åœ°è¿è¡Œï¼ˆåŒä¸€å°æœºå™¨ï¼‰æˆ–èŠ‚ç‚¹ä¸»æœºåœ¨æµè§ˆå™¨æ‰€åœ¨æœºå™¨ä¸Šè¿è¡Œã€‚
- æœ¬åœ°**ä¸­ç»§æœåŠ¡å™¨**åœ¨ loopback çš„ `cdpUrl` ä¸Šç›‘å¬ï¼ˆé»˜è®¤ï¼š`http://127.0.0.1:18792`ï¼‰ã€‚
- ä½ ç‚¹å‡»æ ‡ç­¾é¡µä¸Šçš„ **KrabKrab Browser Relay** æ‰©å±•å›¾æ ‡æ¥é™„åŠ ï¼ˆå®ƒä¸ä¼šè‡ªåŠ¨é™„åŠ ï¼‰ã€‚
- æ™ºèƒ½ä½“é€šè¿‡é€‰æ‹©æ­£ç¡®çš„é…ç½®æ–‡ä»¶ï¼Œä½¿ç”¨æ™®é€šçš„ `browser` å·¥å…·æŽ§åˆ¶è¯¥æ ‡ç­¾é¡µã€‚

å¦‚æžœ Gateway ç½‘å…³åœ¨å…¶ä»–åœ°æ–¹è¿è¡Œï¼Œè¯·åœ¨æµè§ˆå™¨æ‰€åœ¨æœºå™¨ä¸Šè¿è¡ŒèŠ‚ç‚¹ä¸»æœºï¼Œä»¥ä¾¿ Gateway ç½‘å…³å¯ä»¥ä»£ç†æµè§ˆå™¨æ“ä½œã€‚

### æ²™ç®±ä¼šè¯

å¦‚æžœæ™ºèƒ½ä½“ä¼šè¯æ˜¯æ²™ç®±éš”ç¦»çš„ï¼Œ`browser` å·¥å…·å¯èƒ½é»˜è®¤ä¸º `target="sandbox"`ï¼ˆæ²™ç®±æµè§ˆå™¨ï¼‰ã€‚
Chrome æ‰©å±•ä¸­ç»§æŽ¥ç®¡éœ€è¦ä¸»æœºæµè§ˆå™¨æŽ§åˆ¶ï¼Œå› æ­¤è¦ä¹ˆï¼š

- åœ¨éžæ²™ç®±æ¨¡å¼ä¸‹è¿è¡Œä¼šè¯ï¼Œæˆ–è€…
- è®¾ç½® `agents.defaults.sandbox.browser.allowHostControl: true` å¹¶åœ¨è°ƒç”¨å·¥å…·æ—¶ä½¿ç”¨ `target="host"`ã€‚

### è®¾ç½®

1. åŠ è½½æ‰©å±•ï¼ˆå¼€å‘/æœªæ‰“åŒ…ï¼‰ï¼š

```bash
krabkrab browser extension install
```

- Chrome â†’ `chrome://extensions` â†’ å¯ç”¨"å¼€å‘è€…æ¨¡å¼"
- "åŠ è½½å·²è§£åŽ‹çš„æ‰©å±•ç¨‹åº" â†’ é€‰æ‹© `krabkrab browser extension path` æ‰“å°çš„ç›®å½•
- å›ºå®šæ‰©å±•ï¼Œç„¶åŽåœ¨ä½ æƒ³è¦æŽ§åˆ¶çš„æ ‡ç­¾é¡µä¸Šç‚¹å‡»å®ƒï¼ˆå¾½ç« æ˜¾ç¤º `ON`ï¼‰ã€‚

2. ä½¿ç”¨å®ƒï¼š

- CLIï¼š`krabkrab browser --browser-profile chrome tabs`
- æ™ºèƒ½ä½“å·¥å…·ï¼š`browser` é…åˆ `profile="chrome"`

å¯é€‰ï¼šå¦‚æžœä½ æƒ³è¦ä¸åŒçš„åç§°æˆ–ä¸­ç»§ç«¯å£ï¼Œåˆ›å»ºä½ è‡ªå·±çš„é…ç½®æ–‡ä»¶ï¼š

```bash
krabkrab browser create-profile \
  --name my-chrome \
  --driver extension \
  --cdp-url http://127.0.0.1:18792 \
  --color "#00AA00"
```

æ³¨æ„äº‹é¡¹ï¼š

- æ­¤æ¨¡å¼ä¾èµ– Playwright-on-CDP è¿›è¡Œå¤§å¤šæ•°æ“ä½œï¼ˆæˆªå›¾/å¿«ç…§/æ“ä½œï¼‰ã€‚
- å†æ¬¡ç‚¹å‡»æ‰©å±•å›¾æ ‡å¯åˆ†ç¦»ã€‚

## éš”ç¦»ä¿è¯

- **ä¸“ç”¨ç”¨æˆ·æ•°æ®ç›®å½•**ï¼šæ°¸ä¸è§¦åŠä½ çš„ä¸ªäººæµè§ˆå™¨é…ç½®æ–‡ä»¶ã€‚
- **ä¸“ç”¨ç«¯å£**ï¼šé¿å…ä½¿ç”¨ `9222` ä»¥é˜²æ­¢ä¸Žå¼€å‘å·¥ä½œæµå†²çªã€‚
- **ç¡®å®šæ€§æ ‡ç­¾é¡µæŽ§åˆ¶**ï¼šé€šè¿‡ `targetId` å®šä½æ ‡ç­¾é¡µï¼Œè€Œéž"æœ€åŽä¸€ä¸ªæ ‡ç­¾é¡µ"ã€‚

## æµè§ˆå™¨é€‰æ‹©

æœ¬åœ°å¯åŠ¨æ—¶ï¼ŒKrabKrab é€‰æ‹©ç¬¬ä¸€ä¸ªå¯ç”¨çš„ï¼š

1. Chrome
2. Brave
3. Edge
4. Chromium
5. Chrome Canary

ä½ å¯ä»¥ä½¿ç”¨ `browser.executablePath` è¦†ç›–ã€‚

å¹³å°ï¼š

- macOSï¼šæ£€æŸ¥ `/Applications` å’Œ `~/Applications`ã€‚
- Linuxï¼šæŸ¥æ‰¾ `google-chrome`ã€`brave`ã€`microsoft-edge`ã€`chromium` ç­‰ã€‚
- Windowsï¼šæ£€æŸ¥å¸¸è§å®‰è£…ä½ç½®ã€‚

## æŽ§åˆ¶ APIï¼ˆå¯é€‰ï¼‰

ä»…ç”¨äºŽæœ¬åœ°é›†æˆï¼ŒGateway ç½‘å…³æš´éœ²ä¸€ä¸ªå°åž‹çš„ loopback HTTP APIï¼š

- çŠ¶æ€/å¯åŠ¨/åœæ­¢ï¼š`GET /`ã€`POST /start`ã€`POST /stop`
- æ ‡ç­¾é¡µï¼š`GET /tabs`ã€`POST /tabs/open`ã€`POST /tabs/focus`ã€`DELETE /tabs/:targetId`
- å¿«ç…§/æˆªå›¾ï¼š`GET /snapshot`ã€`POST /screenshot`
- æ“ä½œï¼š`POST /navigate`ã€`POST /act`
- é’©å­ï¼š`POST /hooks/file-chooser`ã€`POST /hooks/dialog`
- ä¸‹è½½ï¼š`POST /download`ã€`POST /wait/download`
- è°ƒè¯•ï¼š`GET /console`ã€`POST /pdf`
- è°ƒè¯•ï¼š`GET /errors`ã€`GET /requests`ã€`POST /trace/start`ã€`POST /trace/stop`ã€`POST /highlight`
- ç½‘ç»œï¼š`POST /response/body`
- çŠ¶æ€ï¼š`GET /cookies`ã€`POST /cookies/set`ã€`POST /cookies/clear`
- çŠ¶æ€ï¼š`GET /storage/:kind`ã€`POST /storage/:kind/set`ã€`POST /storage/:kind/clear`
- è®¾ç½®ï¼š`POST /set/offline`ã€`POST /set/headers`ã€`POST /set/credentials`ã€`POST /set/geolocation`ã€`POST /set/media`ã€`POST /set/timezone`ã€`POST /set/locale`ã€`POST /set/device`

æ‰€æœ‰ç«¯ç‚¹æŽ¥å— `?profile=<name>`ã€‚

### Playwright è¦æ±‚

æŸäº›åŠŸèƒ½ï¼ˆnavigate/act/AI å¿«ç…§/è§’è‰²å¿«ç…§ã€å…ƒç´ æˆªå›¾ã€PDFï¼‰éœ€è¦ Playwrightã€‚å¦‚æžœæœªå®‰è£… Playwrightï¼Œè¿™äº›ç«¯ç‚¹ä¼šè¿”å›žæ˜Žç¡®çš„ 501 é”™è¯¯ã€‚ARIA å¿«ç…§å’ŒåŸºæœ¬æˆªå›¾å¯¹äºŽ krabkrab æ‰˜ç®¡çš„ Chrome ä»ç„¶æœ‰æ•ˆã€‚å¯¹äºŽ Chrome æ‰©å±•ä¸­ç»§é©±åŠ¨ç¨‹åºï¼ŒARIA å¿«ç…§å’Œæˆªå›¾éœ€è¦ Playwrightã€‚

å¦‚æžœä½ çœ‹åˆ° `Playwright is not available in this gateway build`ï¼Œè¯·å®‰è£…å®Œæ•´çš„ Playwright åŒ…ï¼ˆä¸æ˜¯ `playwright-core`ï¼‰å¹¶é‡å¯ Gateway ç½‘å…³ï¼Œæˆ–è€…é‡æ–°å®‰è£…å¸¦æµè§ˆå™¨æ”¯æŒçš„ KrabKrabã€‚

#### Docker Playwright å®‰è£…

å¦‚æžœä½ çš„ Gateway ç½‘å…³åœ¨ Docker ä¸­è¿è¡Œï¼Œé¿å…ä½¿ç”¨ `npx playwright`ï¼ˆnpm è¦†ç›–å†²çªï¼‰ã€‚æ”¹ç”¨æ†ç»‘çš„ CLIï¼š

```bash
docker compose run --rm krabkrab-cli \
  node /app/node_modules/playwright-core/cli.js install chromium
```

è¦æŒä¹…åŒ–æµè§ˆå™¨ä¸‹è½½ï¼Œè®¾ç½® `PLAYWRIGHT_BROWSERS_PATH`ï¼ˆä¾‹å¦‚ `/home/node/.cache/ms-playwright`ï¼‰å¹¶ç¡®ä¿ `/home/node` é€šè¿‡ `krabkrab_HOME_VOLUME` æˆ–ç»‘å®šæŒ‚è½½æŒä¹…åŒ–ã€‚å‚è§ [Docker](/install/docker)ã€‚

## å·¥ä½œåŽŸç†ï¼ˆå†…éƒ¨ï¼‰

é«˜å±‚æµç¨‹ï¼š

- ä¸€ä¸ªå°åž‹**æŽ§åˆ¶æœåŠ¡å™¨**æŽ¥å— HTTP è¯·æ±‚ã€‚
- å®ƒé€šè¿‡ **CDP** è¿žæŽ¥åˆ°åŸºäºŽ Chromium çš„æµè§ˆå™¨ï¼ˆChrome/Brave/Edge/Chromiumï¼‰ã€‚
- å¯¹äºŽé«˜çº§æ“ä½œï¼ˆç‚¹å‡»/è¾“å…¥/å¿«ç…§/PDFï¼‰ï¼Œå®ƒåœ¨ CDP ä¹‹ä¸Šä½¿ç”¨ **Playwright**ã€‚
- å½“ç¼ºå°‘ Playwright æ—¶ï¼Œä»…éž Playwright æ“ä½œå¯ç”¨ã€‚

è¿™ç§è®¾è®¡ä½¿æ™ºèƒ½ä½“ä¿æŒåœ¨ç¨³å®šã€ç¡®å®šæ€§çš„æŽ¥å£ä¸Šï¼ŒåŒæ—¶è®©ä½ å¯ä»¥åˆ‡æ¢æœ¬åœ°/è¿œç¨‹æµè§ˆå™¨å’Œé…ç½®æ–‡ä»¶ã€‚

## CLI å¿«é€Ÿå‚è€ƒ

æ‰€æœ‰å‘½ä»¤æŽ¥å— `--browser-profile <name>` ä»¥å®šä½ç‰¹å®šé…ç½®æ–‡ä»¶ã€‚
æ‰€æœ‰å‘½ä»¤ä¹ŸæŽ¥å— `--json` ä»¥èŽ·å¾—æœºå™¨å¯è¯»çš„è¾“å‡ºï¼ˆç¨³å®šçš„è´Ÿè½½ï¼‰ã€‚

åŸºç¡€æ“ä½œï¼š

- `krabkrab browser status`
- `krabkrab browser start`
- `krabkrab browser stop`
- `krabkrab browser tabs`
- `krabkrab browser tab`
- `krabkrab browser tab new`
- `krabkrab browser tab select 2`
- `krabkrab browser tab close 2`
- `krabkrab browser open https://example.com`
- `krabkrab browser focus abcd1234`
- `krabkrab browser close abcd1234`

æ£€æŸ¥ï¼š

- `krabkrab browser screenshot`
- `krabkrab browser screenshot --full-page`
- `krabkrab browser screenshot --ref 12`
- `krabkrab browser screenshot --ref e12`
- `krabkrab browser snapshot`
- `krabkrab browser snapshot --format aria --limit 200`
- `krabkrab browser snapshot --interactive --compact --depth 6`
- `krabkrab browser snapshot --efficient`
- `krabkrab browser snapshot --labels`
- `krabkrab browser snapshot --selector "#main" --interactive`
- `krabkrab browser snapshot --frame "iframe#main" --interactive`
- `krabkrab browser console --level error`
- `krabkrab browser errors --clear`
- `krabkrab browser requests --filter api --clear`
- `krabkrab browser pdf`
- `krabkrab browser responsebody "**/api" --max-chars 5000`

æ“ä½œï¼š

- `krabkrab browser navigate https://example.com`
- `krabkrab browser resize 1280 720`
- `krabkrab browser click 12 --double`
- `krabkrab browser click e12 --double`
- `krabkrab browser type 23 "hello" --submit`
- `krabkrab browser press Enter`
- `krabkrab browser hover 44`
- `krabkrab browser scrollintoview e12`
- `krabkrab browser drag 10 11`
- `krabkrab browser select 9 OptionA OptionB`
- `krabkrab browser download e12 /tmp/report.pdf`
- `krabkrab browser waitfordownload /tmp/report.pdf`
- `krabkrab browser upload /tmp/file.pdf`
- `krabkrab browser fill --fields '[{"ref":"1","type":"text","value":"Ada"}]'`
- `krabkrab browser dialog --accept`
- `krabkrab browser wait --text "Done"`
- `krabkrab browser wait "#main" --url "**/dash" --load networkidle --fn "window.ready===true"`
- `krabkrab browser evaluate --fn '(el) => el.textContent' --ref 7`
- `krabkrab browser highlight e12`
- `krabkrab browser trace start`
- `krabkrab browser trace stop`

çŠ¶æ€ï¼š

- `krabkrab browser cookies`
- `krabkrab browser cookies set session abc123 --url "https://example.com"`
- `krabkrab browser cookies clear`
- `krabkrab browser storage local get`
- `krabkrab browser storage local set theme dark`
- `krabkrab browser storage session clear`
- `krabkrab browser set offline on`
- `krabkrab browser set headers --json '{"X-Debug":"1"}'`
- `krabkrab browser set credentials user pass`
- `krabkrab browser set credentials --clear`
- `krabkrab browser set geo 37.7749 -122.4194 --origin "https://example.com"`
- `krabkrab browser set geo --clear`
- `krabkrab browser set media dark`
- `krabkrab browser set timezone America/New_York`
- `krabkrab browser set locale en-US`
- `krabkrab browser set device "iPhone 14"`

æ³¨æ„äº‹é¡¹ï¼š

- `upload` å’Œ `dialog` æ˜¯**é¢„å¤‡**è°ƒç”¨ï¼›åœ¨è§¦å‘é€‰æ‹©å™¨/å¯¹è¯æ¡†çš„ç‚¹å‡»/æŒ‰é”®ä¹‹å‰è¿è¡Œå®ƒä»¬ã€‚
- `upload` ä¹Ÿå¯ä»¥é€šè¿‡ `--input-ref` æˆ– `--element` ç›´æŽ¥è®¾ç½®æ–‡ä»¶è¾“å…¥ã€‚
- `snapshot`ï¼š
  - `--format ai`ï¼ˆå®‰è£… Playwright æ—¶çš„é»˜è®¤å€¼ï¼‰ï¼šè¿”å›žå¸¦æœ‰æ•°å­— ref çš„ AI å¿«ç…§ï¼ˆ`aria-ref="<n>"`ï¼‰ã€‚
  - `--format aria`ï¼šè¿”å›žæ— éšœç¢æ ‘ï¼ˆæ—  refï¼›ä»…ä¾›æ£€æŸ¥ï¼‰ã€‚
  - `--efficient`ï¼ˆæˆ– `--mode efficient`ï¼‰ï¼šç´§å‡‘è§’è‰²å¿«ç…§é¢„è®¾ï¼ˆinteractive + compact + depth + è¾ƒä½Žçš„ maxCharsï¼‰ã€‚
  - é…ç½®é»˜è®¤å€¼ï¼ˆä»…é™å·¥å…·/CLIï¼‰ï¼šè®¾ç½® `browser.snapshotDefaults.mode: "efficient"` ä»¥åœ¨è°ƒç”¨è€…æœªä¼ é€’æ¨¡å¼æ—¶ä½¿ç”¨é«˜æ•ˆå¿«ç…§ï¼ˆå‚è§ [Gateway ç½‘å…³é…ç½®](/gateway/configuration#browser-krabkrab-managed-browser)ï¼‰ã€‚
  - è§’è‰²å¿«ç…§é€‰é¡¹ï¼ˆ`--interactive`ã€`--compact`ã€`--depth`ã€`--selector`ï¼‰å¼ºåˆ¶ä½¿ç”¨å¸¦æœ‰ `ref=e12` ç­‰ ref çš„åŸºäºŽè§’è‰²çš„å¿«ç…§ã€‚
  - `--frame "<iframe selector>"` å°†è§’è‰²å¿«ç…§èŒƒå›´é™å®šåˆ° iframeï¼ˆä¸Ž `e12` ç­‰è§’è‰² ref é…åˆä½¿ç”¨ï¼‰ã€‚
  - `--interactive` è¾“å‡ºä¸€ä¸ªæ‰å¹³çš„ã€æ˜“äºŽé€‰æ‹©çš„äº¤äº’å…ƒç´ åˆ—è¡¨ï¼ˆæœ€é€‚åˆé©±åŠ¨æ“ä½œï¼‰ã€‚
  - `--labels` æ·»åŠ ä¸€ä¸ªå¸¦æœ‰å åŠ  ref æ ‡ç­¾çš„è§†å£æˆªå›¾ï¼ˆæ‰“å° `MEDIA:<path>`ï¼‰ã€‚
- `click`/`type` ç­‰éœ€è¦æ¥è‡ª `snapshot` çš„ `ref`ï¼ˆæ•°å­— `12` æˆ–è§’è‰² ref `e12`ï¼‰ã€‚
  æ“ä½œæ•…æ„ä¸æ”¯æŒ CSS é€‰æ‹©å™¨ã€‚

## å¿«ç…§å’Œ ref

KrabKrab æ”¯æŒä¸¤ç§"å¿«ç…§"é£Žæ ¼ï¼š

- **AI å¿«ç…§ï¼ˆæ•°å­— refï¼‰**ï¼š`krabkrab browser snapshot`ï¼ˆé»˜è®¤ï¼›`--format ai`ï¼‰
  - è¾“å‡ºï¼šåŒ…å«æ•°å­— ref çš„æ–‡æœ¬å¿«ç…§ã€‚
  - æ“ä½œï¼š`krabkrab browser click 12`ã€`krabkrab browser type 23 "hello"`ã€‚
  - å†…éƒ¨é€šè¿‡ Playwright çš„ `aria-ref` è§£æž refã€‚

- **è§’è‰²å¿«ç…§ï¼ˆè§’è‰² ref å¦‚ `e12`ï¼‰**ï¼š`krabkrab browser snapshot --interactive`ï¼ˆæˆ– `--compact`ã€`--depth`ã€`--selector`ã€`--frame`ï¼‰
  - è¾“å‡ºï¼šå¸¦æœ‰ `[ref=e12]`ï¼ˆå’Œå¯é€‰çš„ `[nth=1]`ï¼‰çš„åŸºäºŽè§’è‰²çš„åˆ—è¡¨/æ ‘ã€‚
  - æ“ä½œï¼š`krabkrab browser click e12`ã€`krabkrab browser highlight e12`ã€‚
  - å†…éƒ¨é€šè¿‡ `getByRole(...)`ï¼ˆåŠ ä¸Šé‡å¤é¡¹çš„ `nth()`ï¼‰è§£æž refã€‚
  - æ·»åŠ  `--labels` å¯åŒ…å«å¸¦æœ‰å åŠ  `e12` æ ‡ç­¾çš„è§†å£æˆªå›¾ã€‚

ref è¡Œä¸ºï¼š

- ref åœ¨**å¯¼èˆªä¹‹é—´ä¸ç¨³å®š**ï¼›å¦‚æžœå‡ºé”™ï¼Œé‡æ–°è¿è¡Œ `snapshot` å¹¶ä½¿ç”¨æ–°çš„ refã€‚
- å¦‚æžœè§’è‰²å¿«ç…§æ˜¯ä½¿ç”¨ `--frame` æ‹æ‘„çš„ï¼Œè§’è‰² ref å°†é™å®šåœ¨è¯¥ iframe å†…ï¼Œç›´åˆ°ä¸‹ä¸€æ¬¡è§’è‰²å¿«ç…§ã€‚

## ç­‰å¾…å¢žå¼ºåŠŸèƒ½

ä½ å¯ä»¥ç­‰å¾…çš„ä¸ä»…ä»…æ˜¯æ—¶é—´/æ–‡æœ¬ï¼š

- ç­‰å¾… URLï¼ˆPlaywright æ”¯æŒé€šé…ç¬¦ï¼‰ï¼š
  - `krabkrab browser wait --url "**/dash"`
- ç­‰å¾…åŠ è½½çŠ¶æ€ï¼š
  - `krabkrab browser wait --load networkidle`
- ç­‰å¾… JS æ–­è¨€ï¼š
  - `krabkrab browser wait --fn "window.ready===true"`
- ç­‰å¾…é€‰æ‹©å™¨å˜å¾—å¯è§ï¼š
  - `krabkrab browser wait "#main"`

è¿™äº›å¯ä»¥ç»„åˆä½¿ç”¨ï¼š

```bash
krabkrab browser wait "#main" \
  --url "**/dash" \
  --load networkidle \
  --fn "window.ready===true" \
  --timeout-ms 15000
```

## è°ƒè¯•å·¥ä½œæµ

å½“æ“ä½œå¤±è´¥æ—¶ï¼ˆä¾‹å¦‚"not visible"ã€"strict mode violation"ã€"covered"ï¼‰ï¼š

1. `krabkrab browser snapshot --interactive`
2. ä½¿ç”¨ `click <ref>` / `type <ref>`ï¼ˆåœ¨äº¤äº’æ¨¡å¼ä¸‹ä¼˜å…ˆä½¿ç”¨è§’è‰² refï¼‰
3. å¦‚æžœä»ç„¶å¤±è´¥ï¼š`krabkrab browser highlight <ref>` æŸ¥çœ‹ Playwright å®šä½çš„ç›®æ ‡
4. å¦‚æžœé¡µé¢è¡Œä¸ºå¼‚å¸¸ï¼š
   - `krabkrab browser errors --clear`
   - `krabkrab browser requests --filter api --clear`
5. æ·±åº¦è°ƒè¯•ï¼šå½•åˆ¶ traceï¼š
   - `krabkrab browser trace start`
   - é‡çŽ°é—®é¢˜
   - `krabkrab browser trace stop`ï¼ˆæ‰“å° `TRACE:<path>`ï¼‰

## JSON è¾“å‡º

`--json` ç”¨äºŽè„šæœ¬å’Œç»“æž„åŒ–å·¥å…·ã€‚

ç¤ºä¾‹ï¼š

```bash
krabkrab browser status --json
krabkrab browser snapshot --interactive --json
krabkrab browser requests --filter api --json
krabkrab browser cookies --json
```

JSON æ ¼å¼çš„è§’è‰²å¿«ç…§åŒ…å« `refs` åŠ ä¸Šä¸€ä¸ªå°çš„ `stats` å—ï¼ˆlines/chars/refs/interactiveï¼‰ï¼Œä»¥ä¾¿å·¥å…·å¯ä»¥æŽ¨æ–­è´Ÿè½½å¤§å°å’Œå¯†åº¦ã€‚

## çŠ¶æ€å’ŒçŽ¯å¢ƒå¼€å…³

è¿™äº›å¯¹äºŽ"è®©ç½‘ç«™è¡¨çŽ°å¾—åƒ X"çš„å·¥ä½œæµå¾ˆæœ‰ç”¨ï¼š

- Cookiesï¼š`cookies`ã€`cookies set`ã€`cookies clear`
- å­˜å‚¨ï¼š`storage local|session get|set|clear`
- ç¦»çº¿ï¼š`set offline on|off`
- è¯·æ±‚å¤´ï¼š`set headers --json '{"X-Debug":"1"}'`ï¼ˆæˆ– `--clear`ï¼‰
- HTTP basic è®¤è¯ï¼š`set credentials user pass`ï¼ˆæˆ– `--clear`ï¼‰
- åœ°ç†ä½ç½®ï¼š`set geo <lat> <lon> --origin "https://example.com"`ï¼ˆæˆ– `--clear`ï¼‰
- åª’ä½“ï¼š`set media dark|light|no-preference|none`
- æ—¶åŒº/è¯­è¨€çŽ¯å¢ƒï¼š`set timezone ...`ã€`set locale ...`
- è®¾å¤‡/è§†å£ï¼š
  - `set device "iPhone 14"`ï¼ˆPlaywright è®¾å¤‡é¢„è®¾ï¼‰
  - `set viewport 1280 720`

## å®‰å…¨ä¸Žéšç§

- krabkrab æµè§ˆå™¨é…ç½®æ–‡ä»¶å¯èƒ½åŒ…å«å·²ç™»å½•çš„ä¼šè¯ï¼›è¯·å°†å…¶è§†ä¸ºæ•æ„Ÿä¿¡æ¯ã€‚
- `browser act kind=evaluate` / `krabkrab browser evaluate` å’Œ `wait --fn` åœ¨é¡µé¢ä¸Šä¸‹æ–‡ä¸­æ‰§è¡Œä»»æ„ JavaScriptã€‚æç¤ºæ³¨å…¥å¯èƒ½ä¼šæ“çºµå®ƒã€‚å¦‚æžœä¸éœ€è¦ï¼Œè¯·ä½¿ç”¨ `browser.evaluateEnabled=false` ç¦ç”¨å®ƒã€‚
- æœ‰å…³ç™»å½•å’Œåæœºå™¨äººæ³¨æ„äº‹é¡¹ï¼ˆX/Twitter ç­‰ï¼‰ï¼Œè¯·å‚é˜… [æµè§ˆå™¨ç™»å½• + X/Twitter å‘å¸–](/tools/browser-login)ã€‚
- ä¿æŒ Gateway ç½‘å…³/èŠ‚ç‚¹ä¸»æœºç§æœ‰ï¼ˆä»…é™ loopback æˆ– tailnetï¼‰ã€‚
- è¿œç¨‹ CDP ç«¯ç‚¹åŠŸèƒ½å¼ºå¤§ï¼›è¯·é€šè¿‡éš§é“ä¿æŠ¤å®ƒä»¬ã€‚

## æ•…éšœæŽ’é™¤

æœ‰å…³ Linux ç‰¹å®šé—®é¢˜ï¼ˆç‰¹åˆ«æ˜¯ snap Chromiumï¼‰ï¼Œè¯·å‚é˜…[æµè§ˆå™¨æ•…éšœæŽ’é™¤](/tools/browser-linux-troubleshooting)ã€‚

## æ™ºèƒ½ä½“å·¥å…· + æŽ§åˆ¶å·¥ä½œåŽŸç†

æ™ºèƒ½ä½“èŽ·å¾—**ä¸€ä¸ªå·¥å…·**ç”¨äºŽæµè§ˆå™¨è‡ªåŠ¨åŒ–ï¼š

- `browser` â€” status/start/stop/tabs/open/focus/close/snapshot/screenshot/navigate/act

æ˜ å°„æ–¹å¼ï¼š

- `browser snapshot` è¿”å›žç¨³å®šçš„ UI æ ‘ï¼ˆAI æˆ– ARIAï¼‰ã€‚
- `browser act` ä½¿ç”¨å¿«ç…§ `ref` ID æ¥ç‚¹å‡»/è¾“å…¥/æ‹–åŠ¨/é€‰æ‹©ã€‚
- `browser screenshot` æ•èŽ·åƒç´ ï¼ˆæ•´é¡µæˆ–å…ƒç´ ï¼‰ã€‚
- `browser` æŽ¥å—ï¼š
  - `profile` æ¥é€‰æ‹©å‘½åçš„æµè§ˆå™¨é…ç½®æ–‡ä»¶ï¼ˆkrabkrabã€chrome æˆ–è¿œç¨‹ CDPï¼‰ã€‚
  - `target`ï¼ˆ`sandbox` | `host` | `node`ï¼‰æ¥é€‰æ‹©æµè§ˆå™¨æ‰€åœ¨ä½ç½®ã€‚
  - åœ¨æ²™ç®±ä¼šè¯ä¸­ï¼Œ`target: "host"` éœ€è¦ `agents.defaults.sandbox.browser.allowHostControl=true`ã€‚
  - å¦‚æžœçœç•¥ `target`ï¼šæ²™ç®±ä¼šè¯é»˜è®¤ä¸º `sandbox`ï¼Œéžæ²™ç®±ä¼šè¯é»˜è®¤ä¸º `host`ã€‚
  - å¦‚æžœè¿žæŽ¥äº†å…·æœ‰æµè§ˆå™¨èƒ½åŠ›çš„èŠ‚ç‚¹ï¼Œå·¥å…·å¯èƒ½ä¼šè‡ªåŠ¨è·¯ç”±åˆ°è¯¥èŠ‚ç‚¹ï¼Œé™¤éžä½ æŒ‡å®š `target="host"` æˆ– `target="node"`ã€‚

è¿™ä½¿æ™ºèƒ½ä½“ä¿æŒç¡®å®šæ€§å¹¶é¿å…è„†å¼±çš„é€‰æ‹©å™¨ã€‚

