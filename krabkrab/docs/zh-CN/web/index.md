---
read_when:
  - ä½ æƒ³é€šè¿‡ Tailscale è®¿é—® Gateway ç½‘å…³
  - ä½ æƒ³ä½¿ç”¨æµè§ˆå™¨ Control UI å’Œé…ç½®ç¼–è¾‘
summary: Gateway ç½‘å…³ Web ç•Œé¢ï¼šControl UIã€ç»‘å®šæ¨¡å¼å’Œå®‰å…¨
title: Web
x-i18n:
  generated_at: "2026-02-03T10:13:29Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 4da8bc9831018c482ac918a759b9739f75ca130f70993f81911818bc60a685d1
  source_path: web/index.md
  workflow: 15
---

# Webï¼ˆGateway ç½‘å…³ï¼‰

Gateway ç½‘å…³ä»Žä¸Ž Gateway ç½‘å…³ WebSocket ç›¸åŒçš„ç«¯å£æä¾›ä¸€ä¸ªå°åž‹**æµè§ˆå™¨ Control UI**ï¼ˆVite + Litï¼‰ï¼š

- é»˜è®¤ï¼š`http://<host>:18789/`
- å¯é€‰å‰ç¼€ï¼šè®¾ç½® `gateway.controlUi.basePath`ï¼ˆä¾‹å¦‚ `/krabkrab`ï¼‰

åŠŸèƒ½è¯¦è§ [Control UI](/web/control-ui)ã€‚
æœ¬é¡µé‡ç‚¹ä»‹ç»ç»‘å®šæ¨¡å¼ã€å®‰å…¨å’Œé¢å‘ Web çš„ç•Œé¢ã€‚

## Webhooks

å½“ `hooks.enabled=true` æ—¶ï¼ŒGateway ç½‘å…³è¿˜åœ¨åŒä¸€ HTTP æœåŠ¡å™¨ä¸Šå…¬å¼€ä¸€ä¸ªå°åž‹ webhook ç«¯ç‚¹ã€‚
å‚è§ [Gateway ç½‘å…³é…ç½®](/gateway/configuration) â†’ `hooks` äº†è§£è®¤è¯ + è½½è·ã€‚

## é…ç½®ï¼ˆé»˜è®¤å¼€å¯ï¼‰

å½“èµ„æºå­˜åœ¨æ—¶ï¼ˆ`dist/control-ui`ï¼‰ï¼ŒControl UI **é»˜è®¤å¯ç”¨**ã€‚
ä½ å¯ä»¥é€šè¿‡é…ç½®æŽ§åˆ¶å®ƒï¼š

```json5
{
  gateway: {
    controlUi: { enabled: true, basePath: "/krabkrab" }, // basePath å¯é€‰
  },
}
```

## Tailscale è®¿é—®

### é›†æˆ Serveï¼ˆæŽ¨èï¼‰

ä¿æŒ Gateway ç½‘å…³åœ¨æœ¬åœ°å›žçŽ¯ä¸Šï¼Œè®© Tailscale Serve ä»£ç†å®ƒï¼š

```json5
{
  gateway: {
    bind: "loopback",
    tailscale: { mode: "serve" },
  },
}
```

ç„¶åŽå¯åŠ¨ Gateway ç½‘å…³ï¼š

```bash
krabkrab gateway
```

æ‰“å¼€ï¼š

- `https://<magicdns>/`ï¼ˆæˆ–ä½ é…ç½®çš„ `gateway.controlUi.basePath`ï¼‰

### Tailnet ç»‘å®š + ä»¤ç‰Œ

```json5
{
  gateway: {
    bind: "tailnet",
    controlUi: { enabled: true },
    auth: { mode: "token", token: "your-token" },
  },
}
```

ç„¶åŽå¯åŠ¨ Gateway ç½‘å…³ï¼ˆéžæœ¬åœ°å›žçŽ¯ç»‘å®šéœ€è¦ä»¤ç‰Œï¼‰ï¼š

```bash
krabkrab gateway
```

æ‰“å¼€ï¼š

- `http://<tailscale-ip>:18789/`ï¼ˆæˆ–ä½ é…ç½®çš„ `gateway.controlUi.basePath`ï¼‰

### å…¬å…±äº’è”ç½‘ï¼ˆFunnelï¼‰

```json5
{
  gateway: {
    bind: "loopback",
    tailscale: { mode: "funnel" },
    auth: { mode: "password" }, // æˆ– krabkrab_GATEWAY_PASSWORD
  },
}
```

## å®‰å…¨æ³¨æ„äº‹é¡¹

- Gateway ç½‘å…³è®¤è¯é»˜è®¤æ˜¯å¿…éœ€çš„ï¼ˆä»¤ç‰Œ/å¯†ç æˆ– Tailscale èº«ä»½å¤´ï¼‰ã€‚
- éžæœ¬åœ°å›žçŽ¯ç»‘å®šä»ç„¶**éœ€è¦**å…±äº«ä»¤ç‰Œ/å¯†ç ï¼ˆ`gateway.auth` æˆ–çŽ¯å¢ƒå˜é‡ï¼‰ã€‚
- å‘å¯¼é»˜è®¤ç”Ÿæˆ Gateway ç½‘å…³ä»¤ç‰Œï¼ˆå³ä½¿åœ¨æœ¬åœ°å›žçŽ¯ä¸Šï¼‰ã€‚
- UI å‘é€ `connect.params.auth.token` æˆ– `connect.params.auth.password`ã€‚
- ä½¿ç”¨ Serve æ—¶ï¼Œå½“ `gateway.auth.allowTailscale` ä¸º `true` æ—¶ï¼ŒTailscale èº«ä»½å¤´å¯ä»¥æ»¡è¶³è®¤è¯ï¼ˆæ— éœ€ä»¤ç‰Œ/å¯†ç ï¼‰ã€‚è®¾ç½® `gateway.auth.allowTailscale: false` ä»¥è¦æ±‚æ˜¾å¼å‡­è¯ã€‚å‚è§ [Tailscale](/gateway/tailscale) å’Œ [å®‰å…¨](/gateway/security)ã€‚
- `gateway.tailscale.mode: "funnel"` éœ€è¦ `gateway.auth.mode: "password"`ï¼ˆå…±äº«å¯†ç ï¼‰ã€‚

## æž„å»º UI

Gateway ç½‘å…³ä»Ž `dist/control-ui` æä¾›é™æ€æ–‡ä»¶ã€‚ä½¿ç”¨ä»¥ä¸‹å‘½ä»¤æž„å»ºï¼š

```bash
pnpm ui:build # é¦–æ¬¡è¿è¡Œæ—¶è‡ªåŠ¨å®‰è£… UI ä¾èµ–
```

