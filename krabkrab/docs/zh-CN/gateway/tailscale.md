---
read_when:
  - åœ¨ localhost ä¹‹å¤–æš´éœ² Gateway ç½‘å…³æŽ§åˆ¶ UI
  - è‡ªåŠ¨åŒ– tailnet æˆ–å…¬å…±ä»ªè¡¨ç›˜è®¿é—®
summary: ä¸º Gateway ç½‘å…³ä»ªè¡¨ç›˜é›†æˆ Tailscale Serve/Funnel
title: Tailscale
x-i18n:
  generated_at: "2026-02-03T07:49:04Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: c900c70a9301f2909a3a29a6fb0e6edfc8c18dba443f2e71b9cfadbc58167911
  source_path: gateway/tailscale.md
  workflow: 15
---

# Tailscaleï¼ˆGateway ç½‘å…³ä»ªè¡¨ç›˜ï¼‰

KrabKrab å¯ä»¥ä¸º Gateway ç½‘å…³ä»ªè¡¨ç›˜å’Œ WebSocket ç«¯å£è‡ªåŠ¨é…ç½® Tailscale **Serve**ï¼ˆtailnetï¼‰æˆ– **Funnel**ï¼ˆå…¬å…±ï¼‰ã€‚è¿™ä½¿ Gateway ç½‘å…³ä¿æŒç»‘å®šåˆ° loopbackï¼ŒåŒæ—¶ Tailscale æä¾› HTTPSã€è·¯ç”±å’Œï¼ˆå¯¹äºŽ Serveï¼‰èº«ä»½å¤´ã€‚

## æ¨¡å¼

- `serve`ï¼šä»…é™ Tailnet çš„ Serveï¼Œé€šè¿‡ `tailscale serve`ã€‚Gateway ç½‘å…³ä¿æŒåœ¨ `127.0.0.1` ä¸Šã€‚
- `funnel`ï¼šé€šè¿‡ `tailscale funnel` çš„å…¬å…± HTTPSã€‚KrabKrab éœ€è¦å…±äº«å¯†ç ã€‚
- `off`ï¼šé»˜è®¤ï¼ˆæ—  Tailscale è‡ªåŠ¨åŒ–ï¼‰ã€‚

## è®¤è¯

è®¾ç½® `gateway.auth.mode` æ¥æŽ§åˆ¶æ¡æ‰‹ï¼š

- `token`ï¼ˆè®¾ç½® `krabkrab_GATEWAY_TOKEN` æ—¶çš„é»˜è®¤å€¼ï¼‰
- `password`ï¼ˆé€šè¿‡ `krabkrab_GATEWAY_PASSWORD` æˆ–é…ç½®çš„å…±äº«å¯†é’¥ï¼‰

å½“ `tailscale.mode = "serve"` ä¸” `gateway.auth.allowTailscale` ä¸º `true` æ—¶ï¼Œ
æœ‰æ•ˆçš„ Serve ä»£ç†è¯·æ±‚å¯ä»¥é€šè¿‡ Tailscale èº«ä»½å¤´ï¼ˆ`tailscale-user-login`ï¼‰è¿›è¡Œè®¤è¯ï¼Œæ— éœ€æä¾›ä»¤ç‰Œ/å¯†ç ã€‚KrabKrab é€šè¿‡æœ¬åœ° Tailscale å®ˆæŠ¤è¿›ç¨‹ï¼ˆ`tailscale whois`ï¼‰è§£æž `x-forwarded-for` åœ°å€å¹¶å°†å…¶ä¸Žå¤´åŒ¹é…æ¥éªŒè¯èº«ä»½ï¼Œç„¶åŽæ‰æŽ¥å—å®ƒã€‚
KrabKrab ä»…åœ¨è¯·æ±‚ä»Ž loopback åˆ°è¾¾å¹¶å¸¦æœ‰ Tailscale çš„ `x-forwarded-for`ã€`x-forwarded-proto` å’Œ `x-forwarded-host` å¤´æ—¶æ‰å°†å…¶è§†ä¸º Serve è¯·æ±‚ã€‚
è¦è¦æ±‚æ˜¾å¼å‡­è¯ï¼Œè®¾ç½® `gateway.auth.allowTailscale: false` æˆ–å¼ºåˆ¶ `gateway.auth.mode: "password"`ã€‚

## é…ç½®ç¤ºä¾‹

### ä»…é™ Tailnetï¼ˆServeï¼‰

```json5
{
  gateway: {
    bind: "loopback",
    tailscale: { mode: "serve" },
  },
}
```

æ‰“å¼€ï¼š`https://<magicdns>/`ï¼ˆæˆ–ä½ é…ç½®çš„ `gateway.controlUi.basePath`ï¼‰

### ä»…é™ Tailnetï¼ˆç»‘å®šåˆ° Tailnet IPï¼‰

å½“ä½ å¸Œæœ› Gateway ç½‘å…³ç›´æŽ¥ç›‘å¬ Tailnet IP æ—¶ä½¿ç”¨æ­¤æ–¹å¼ï¼ˆæ—  Serve/Funnelï¼‰ã€‚

```json5
{
  gateway: {
    bind: "tailnet",
    auth: { mode: "token", token: "your-token" },
  },
}
```

ä»Žå¦ä¸€ä¸ª Tailnet è®¾å¤‡è¿žæŽ¥ï¼š

- æŽ§åˆ¶ UIï¼š`http://<tailscale-ip>:18789/`
- WebSocketï¼š`ws://<tailscale-ip>:18789`

æ³¨æ„ï¼šåœ¨æ­¤æ¨¡å¼ä¸‹ loopbackï¼ˆ`http://127.0.0.1:18789`ï¼‰å°†**ä¸**å·¥ä½œã€‚

### å…¬å…±äº’è”ç½‘ï¼ˆFunnel + å…±äº«å¯†ç ï¼‰

```json5
{
  gateway: {
    bind: "loopback",
    tailscale: { mode: "funnel" },
    auth: { mode: "password", password: "replace-me" },
  },
}
```

ä¼˜å…ˆä½¿ç”¨ `krabkrab_GATEWAY_PASSWORD` è€Œä¸æ˜¯å°†å¯†ç æäº¤åˆ°ç£ç›˜ã€‚

## CLI ç¤ºä¾‹

```bash
krabkrab gateway --tailscale serve
krabkrab gateway --tailscale funnel --auth password
```

## æ³¨æ„äº‹é¡¹

- Tailscale Serve/Funnel éœ€è¦å®‰è£…å¹¶ç™»å½• `tailscale` CLIã€‚
- `tailscale.mode: "funnel"` é™¤éžè®¤è¯æ¨¡å¼ä¸º `password`ï¼Œå¦åˆ™æ‹’ç»å¯åŠ¨ï¼Œä»¥é¿å…å…¬å…±æš´éœ²ã€‚
- å¦‚æžœä½ å¸Œæœ› KrabKrab åœ¨å…³é—­æ—¶æ’¤é”€ `tailscale serve` æˆ– `tailscale funnel` é…ç½®ï¼Œè®¾ç½® `gateway.tailscale.resetOnExit`ã€‚
- `gateway.bind: "tailnet"` æ˜¯ç›´æŽ¥ Tailnet ç»‘å®šï¼ˆæ—  HTTPSï¼Œæ—  Serve/Funnelï¼‰ã€‚
- `gateway.bind: "auto"` ä¼˜å…ˆ loopbackï¼›å¦‚æžœä½ æƒ³è¦ä»… Tailnetï¼Œä½¿ç”¨ `tailnet`ã€‚
- Serve/Funnel ä»…æš´éœ² **Gateway ç½‘å…³æŽ§åˆ¶ UI + WS**ã€‚èŠ‚ç‚¹é€šè¿‡ç›¸åŒçš„ Gateway ç½‘å…³ WS ç«¯ç‚¹è¿žæŽ¥ï¼Œå› æ­¤ Serve å¯ä»¥ç”¨äºŽèŠ‚ç‚¹è®¿é—®ã€‚

## æµè§ˆå™¨æŽ§åˆ¶ï¼ˆè¿œç¨‹ Gateway ç½‘å…³ + æœ¬åœ°æµè§ˆå™¨ï¼‰

å¦‚æžœä½ åœ¨ä¸€å°æœºå™¨ä¸Šè¿è¡Œ Gateway ç½‘å…³ä½†æƒ³åœ¨å¦ä¸€å°æœºå™¨ä¸Šé©±åŠ¨æµè§ˆå™¨ï¼Œ
åœ¨æµè§ˆå™¨æœºå™¨ä¸Šè¿è¡Œä¸€ä¸ª**èŠ‚ç‚¹ä¸»æœº**å¹¶è®©ä¸¤è€…ä¿æŒåœ¨åŒä¸€ä¸ª tailnet ä¸Šã€‚
Gateway ç½‘å…³ä¼šå°†æµè§ˆå™¨æ“ä½œä»£ç†åˆ°èŠ‚ç‚¹ï¼›ä¸éœ€è¦å•ç‹¬çš„æŽ§åˆ¶æœåŠ¡å™¨æˆ– Serve URLã€‚

é¿å…å°† Funnel ç”¨äºŽæµè§ˆå™¨æŽ§åˆ¶ï¼›å°†èŠ‚ç‚¹é…å¯¹è§†ä¸ºæ“ä½œè€…è®¿é—®ã€‚

## Tailscale å‰ææ¡ä»¶ + é™åˆ¶

- Serve éœ€è¦ä¸ºä½ çš„ tailnet å¯ç”¨ HTTPSï¼›å¦‚æžœç¼ºå°‘ï¼ŒCLI ä¼šæç¤ºã€‚
- Serve æ³¨å…¥ Tailscale èº«ä»½å¤´ï¼›Funnel ä¸ä¼šã€‚
- Funnel éœ€è¦ Tailscale v1.38.3+ã€MagicDNSã€å¯ç”¨ HTTPS å’Œ funnel èŠ‚ç‚¹å±žæ€§ã€‚
- Funnel ä»…æ”¯æŒé€šè¿‡ TLS çš„ç«¯å£ `443`ã€`8443` å’Œ `10000`ã€‚
- macOS ä¸Šçš„ Funnel éœ€è¦å¼€æº Tailscale åº”ç”¨å˜ä½“ã€‚

## äº†è§£æ›´å¤š

- Tailscale Serve æ¦‚è¿°ï¼šhttps://tailscale.com/kb/1312/serve
- `tailscale serve` å‘½ä»¤ï¼šhttps://tailscale.com/kb/1242/tailscale-serve
- Tailscale Funnel æ¦‚è¿°ï¼šhttps://tailscale.com/kb/1223/tailscale-funnel
- `tailscale funnel` å‘½ä»¤ï¼šhttps://tailscale.com/kb/1311/tailscale-funnel

