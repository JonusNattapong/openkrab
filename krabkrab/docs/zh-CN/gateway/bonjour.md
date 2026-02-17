---
read_when:
  - åœ¨ macOS/iOS ä¸Šè°ƒè¯• Bonjour è®¾å¤‡å‘çŽ°é—®é¢˜æ—¶
  - æ›´æ”¹ mDNS æœåŠ¡ç±»åž‹ã€TXT è®°å½•æˆ–è®¾å¤‡å‘çŽ°ç”¨æˆ·ä½“éªŒæ—¶
summary: Bonjour/mDNS è®¾å¤‡å‘çŽ° + è°ƒè¯•ï¼ˆGateway ç½‘å…³ä¿¡æ ‡ã€å®¢æˆ·ç«¯å’Œå¸¸è§æ•…éšœæ¨¡å¼ï¼‰
title: Bonjour è®¾å¤‡å‘çŽ°
x-i18n:
  generated_at: "2026-02-03T07:47:48Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 47569da55f0c0523bd5ff05275dc95ccb52f75638193cfbdb4eaaa162aadf08c
  source_path: gateway/bonjour.md
  workflow: 15
---

# Bonjour / mDNS è®¾å¤‡å‘çŽ°

KrabKrab ä½¿ç”¨ Bonjourï¼ˆmDNS / DNSâ€‘SDï¼‰ä½œä¸º**ä»…é™å±€åŸŸç½‘çš„ä¾¿æ·æ–¹å¼**æ¥å‘çŽ°
æ´»è·ƒçš„ Gateway ç½‘å…³ï¼ˆWebSocket ç«¯ç‚¹ï¼‰ã€‚è¿™æ˜¯å°½åŠ›è€Œä¸ºçš„ï¼Œ**ä¸èƒ½**æ›¿ä»£ SSH æˆ–
åŸºäºŽ Tailnet çš„è¿žæŽ¥ã€‚

## é€šè¿‡ Tailscale çš„å¹¿åŸŸ Bonjourï¼ˆå•æ’­ DNSâ€‘SDï¼‰

å¦‚æžœèŠ‚ç‚¹å’Œ Gateway ç½‘å…³åœ¨ä¸åŒçš„ç½‘ç»œä¸Šï¼Œå¤šæ’­ mDNS æ— æ³•è·¨è¶Š
è¾¹ç•Œã€‚ä½ å¯ä»¥é€šè¿‡åˆ‡æ¢åˆ°åŸºäºŽ Tailscale çš„**å•æ’­ DNSâ€‘SD**
ï¼ˆ"å¹¿åŸŸ Bonjour"ï¼‰æ¥ä¿æŒç›¸åŒçš„è®¾å¤‡å‘çŽ°ç”¨æˆ·ä½“éªŒã€‚

æ¦‚è¦æ­¥éª¤ï¼š

1. åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè¿è¡Œ DNS æœåŠ¡å™¨ï¼ˆå¯é€šè¿‡ Tailnet è®¿é—®ï¼‰ã€‚
2. åœ¨ä¸“ç”¨åŒºåŸŸä¸‹å‘å¸ƒ `_krabkrab-gw._tcp` çš„ DNSâ€‘SD è®°å½•
   ï¼ˆç¤ºä¾‹ï¼š`krabkrab.internal.`ï¼‰ã€‚
3. é…ç½® Tailscale **åˆ†å‰² DNS**ï¼Œä½¿ä½ é€‰æ‹©çš„åŸŸåé€šè¿‡è¯¥
   DNS æœåŠ¡å™¨ä¸ºå®¢æˆ·ç«¯ï¼ˆåŒ…æ‹¬ iOSï¼‰è§£æžã€‚

KrabKrab æ”¯æŒä»»ä½•å‘çŽ°åŸŸåï¼›`krabkrab.internal.` åªæ˜¯ä¸€ä¸ªç¤ºä¾‹ã€‚
iOS/Android èŠ‚ç‚¹åŒæ—¶æµè§ˆ `local.` å’Œä½ é…ç½®çš„å¹¿åŸŸåŸŸåã€‚

### Gateway ç½‘å…³é…ç½®ï¼ˆæŽ¨èï¼‰

```json5
{
  gateway: { bind: "tailnet" }, // ä»… tailnetï¼ˆæŽ¨èï¼‰
  discovery: { wideArea: { enabled: true } }, // å¯ç”¨å¹¿åŸŸ DNS-SD å‘å¸ƒ
}
```

### ä¸€æ¬¡æ€§ DNS æœåŠ¡å™¨è®¾ç½®ï¼ˆGateway ç½‘å…³ä¸»æœºï¼‰

```bash
krabkrab dns setup --apply
```

è¿™ä¼šå®‰è£… CoreDNS å¹¶é…ç½®å®ƒï¼š

- ä»…åœ¨ Gateway ç½‘å…³çš„ Tailscale æŽ¥å£ä¸Šç›‘å¬ 53 ç«¯å£
- ä»Ž `~/.krabkrab/dns/<domain>.db` æä¾›ä½ é€‰æ‹©çš„åŸŸåæœåŠ¡ï¼ˆç¤ºä¾‹ï¼š`krabkrab.internal.`ï¼‰

ä»Ž Tailnet è¿žæŽ¥çš„æœºå™¨ä¸ŠéªŒè¯ï¼š

```bash
dns-sd -B _krabkrab-gw._tcp krabkrab.internal.
dig @<TAILNET_IPV4> -p 53 _krabkrab-gw._tcp.krabkrab.internal PTR +short
```

### Tailscale DNS è®¾ç½®

åœ¨ Tailscale ç®¡ç†æŽ§åˆ¶å°ä¸­ï¼š

- æ·»åŠ æŒ‡å‘ Gateway ç½‘å…³ Tailnet IP çš„åç§°æœåŠ¡å™¨ï¼ˆUDP/TCP 53ï¼‰ã€‚
- æ·»åŠ åˆ†å‰² DNSï¼Œä½¿ä½ çš„å‘çŽ°åŸŸåä½¿ç”¨è¯¥åç§°æœåŠ¡å™¨ã€‚

ä¸€æ—¦å®¢æˆ·ç«¯æŽ¥å— Tailnet DNSï¼ŒiOS èŠ‚ç‚¹å°±å¯ä»¥åœ¨
ä½ çš„å‘çŽ°åŸŸåä¸­æµè§ˆ `_krabkrab-gw._tcp`ï¼Œæ— éœ€å¤šæ’­ã€‚

### Gateway ç½‘å…³ç›‘å¬å™¨å®‰å…¨ï¼ˆæŽ¨èï¼‰

Gateway ç½‘å…³ WS ç«¯å£ï¼ˆé»˜è®¤ `18789`ï¼‰é»˜è®¤ç»‘å®šåˆ° loopbackã€‚å¯¹äºŽå±€åŸŸç½‘/Tailnet
è®¿é—®ï¼Œè¯·æ˜Žç¡®ç»‘å®šå¹¶ä¿æŒè®¤è¯å¯ç”¨ã€‚

å¯¹äºŽä»… Tailnet çš„è®¾ç½®ï¼š

- åœ¨ `~/.krabkrab/krabkrab.json` ä¸­è®¾ç½® `gateway.bind: "tailnet"`ã€‚
- é‡å¯ Gateway ç½‘å…³ï¼ˆæˆ–é‡å¯ macOS èœå•æ åº”ç”¨ï¼‰ã€‚

## ä»€ä¹ˆåœ¨å¹¿æ’­

åªæœ‰ Gateway ç½‘å…³å¹¿æ’­ `_krabkrab-gw._tcp`ã€‚

## æœåŠ¡ç±»åž‹

- `_krabkrab-gw._tcp` â€” Gateway ç½‘å…³ä¼ è¾“ä¿¡æ ‡ï¼ˆè¢« macOS/iOS/Android èŠ‚ç‚¹ä½¿ç”¨ï¼‰ã€‚

## TXT é”®ï¼ˆéžæœºå¯†æç¤ºï¼‰

Gateway ç½‘å…³å¹¿æ’­å°åž‹éžæœºå¯†æç¤ºä»¥æ–¹ä¾¿ UI æµç¨‹ï¼š

- `role=gateway`
- `displayName=<å‹å¥½åç§°>`
- `lanHost=<hostname>.local`
- `gatewayPort=<port>`ï¼ˆGateway ç½‘å…³ WS + HTTPï¼‰
- `gatewayTls=1`ï¼ˆä»…å½“ TLS å¯ç”¨æ—¶ï¼‰
- `gatewayTlsSha256=<sha256>`ï¼ˆä»…å½“ TLS å¯ç”¨ä¸”æŒ‡çº¹å¯ç”¨æ—¶ï¼‰
- `canvasPort=<port>`ï¼ˆä»…å½“ç”»å¸ƒä¸»æœºå¯ç”¨æ—¶ï¼›é»˜è®¤ `18793`ï¼‰
- `sshPort=<port>`ï¼ˆæœªè¦†ç›–æ—¶é»˜è®¤ä¸º 22ï¼‰
- `transport=gateway`
- `cliPath=<path>`ï¼ˆå¯é€‰ï¼›å¯è¿è¡Œçš„ `krabkrab` å…¥å£ç‚¹çš„ç»å¯¹è·¯å¾„ï¼‰
- `tailnetDns=<magicdns>`ï¼ˆå½“ Tailnet å¯ç”¨æ—¶çš„å¯é€‰æç¤ºï¼‰

## åœ¨ macOS ä¸Šè°ƒè¯•

æœ‰ç”¨çš„å†…ç½®å·¥å…·ï¼š

- æµè§ˆå®žä¾‹ï¼š
  ```bash
  dns-sd -B _krabkrab-gw._tcp local.
  ```
- è§£æžå•ä¸ªå®žä¾‹ï¼ˆæ›¿æ¢ `<instance>`ï¼‰ï¼š
  ```bash
  dns-sd -L "<instance>" _krabkrab-gw._tcp local.
  ```

å¦‚æžœæµè§ˆæœ‰æ•ˆä½†è§£æžå¤±è´¥ï¼Œä½ é€šå¸¸é‡åˆ°çš„æ˜¯å±€åŸŸç½‘ç­–ç•¥æˆ–
mDNS è§£æžå™¨é—®é¢˜ã€‚

## åœ¨ Gateway ç½‘å…³æ—¥å¿—ä¸­è°ƒè¯•

Gateway ç½‘å…³ä¼šå†™å…¥æ»šåŠ¨æ—¥å¿—æ–‡ä»¶ï¼ˆå¯åŠ¨æ—¶æ‰“å°ä¸º
`gateway log file: ...`ï¼‰ã€‚æŸ¥æ‰¾ `bonjour:` è¡Œï¼Œç‰¹åˆ«æ˜¯ï¼š

- `bonjour: advertise failed ...`
- `bonjour: ... name conflict resolved` / `hostname conflict resolved`
- `bonjour: watchdog detected non-announced service ...`

## åœ¨ iOS èŠ‚ç‚¹ä¸Šè°ƒè¯•

iOS èŠ‚ç‚¹ä½¿ç”¨ `NWBrowser` æ¥å‘çŽ° `_krabkrab-gw._tcp`ã€‚

è¦æ•èŽ·æ—¥å¿—ï¼š

- è®¾ç½® â†’ Gateway ç½‘å…³ â†’ é«˜çº§ â†’ **Discovery Debug Logs**
- è®¾ç½® â†’ Gateway ç½‘å…³ â†’ é«˜çº§ â†’ **Discovery Logs** â†’ å¤çŽ° â†’ **Copy**

æ—¥å¿—åŒ…æ‹¬æµè§ˆå™¨çŠ¶æ€è½¬æ¢å’Œç»“æžœé›†å˜åŒ–ã€‚

## å¸¸è§æ•…éšœæ¨¡å¼

- **Bonjour ä¸èƒ½è·¨ç½‘ç»œ**ï¼šä½¿ç”¨ Tailnet æˆ– SSHã€‚
- **å¤šæ’­è¢«é˜»æ­¢**ï¼šæŸäº› Wiâ€‘Fi ç½‘ç»œç¦ç”¨ mDNSã€‚
- **ä¼‘çœ  / æŽ¥å£å˜åŠ¨**ï¼šmacOS å¯èƒ½æš‚æ—¶ä¸¢å¼ƒ mDNS ç»“æžœï¼›é‡è¯•ã€‚
- **æµè§ˆæœ‰æ•ˆä½†è§£æžå¤±è´¥**ï¼šä¿æŒæœºå™¨åç§°ç®€å•ï¼ˆé¿å…è¡¨æƒ…ç¬¦å·æˆ–
  æ ‡ç‚¹ç¬¦å·ï¼‰ï¼Œç„¶åŽé‡å¯ Gateway ç½‘å…³ã€‚æœåŠ¡å®žä¾‹åç§°æºè‡ª
  ä¸»æœºåï¼Œå› æ­¤è¿‡äºŽå¤æ‚çš„åç§°å¯èƒ½ä¼šæ··æ·†æŸäº›è§£æžå™¨ã€‚

## è½¬ä¹‰çš„å®žä¾‹åç§°ï¼ˆ`\032`ï¼‰

Bonjour/DNSâ€‘SD ç»å¸¸å°†æœåŠ¡å®žä¾‹åç§°ä¸­çš„å­—èŠ‚è½¬ä¹‰ä¸ºåè¿›åˆ¶ `\DDD`
åºåˆ—ï¼ˆä¾‹å¦‚ç©ºæ ¼å˜æˆ `\032`ï¼‰ã€‚

- è¿™åœ¨åè®®çº§åˆ«æ˜¯æ­£å¸¸çš„ã€‚
- UI åº”è¯¥è§£ç ä»¥è¿›è¡Œæ˜¾ç¤ºï¼ˆiOS ä½¿ç”¨ `BonjourEscapes.decode`ï¼‰ã€‚

## ç¦ç”¨ / é…ç½®

- `krabkrab_DISABLE_BONJOUR=1` ç¦ç”¨å¹¿æ’­ï¼ˆæ—§ç‰ˆï¼š`krabkrab_DISABLE_BONJOUR`ï¼‰ã€‚
- `~/.krabkrab/krabkrab.json` ä¸­çš„ `gateway.bind` æŽ§åˆ¶ Gateway ç½‘å…³ç»‘å®šæ¨¡å¼ã€‚
- `krabkrab_SSH_PORT` è¦†ç›– TXT ä¸­å¹¿æ’­çš„ SSH ç«¯å£ï¼ˆæ—§ç‰ˆï¼š`krabkrab_SSH_PORT`ï¼‰ã€‚
- `krabkrab_TAILNET_DNS` åœ¨ TXT ä¸­å‘å¸ƒ MagicDNS æç¤ºï¼ˆæ—§ç‰ˆï¼š`krabkrab_TAILNET_DNS`ï¼‰ã€‚
- `krabkrab_CLI_PATH` è¦†ç›–å¹¿æ’­çš„ CLI è·¯å¾„ï¼ˆæ—§ç‰ˆï¼š`krabkrab_CLI_PATH`ï¼‰ã€‚

## ç›¸å…³æ–‡æ¡£

- è®¾å¤‡å‘çŽ°ç­–ç•¥å’Œä¼ è¾“é€‰æ‹©ï¼š[è®¾å¤‡å‘çŽ°](/gateway/discovery)
- èŠ‚ç‚¹é…å¯¹ + æ‰¹å‡†ï¼š[Gateway ç½‘å…³é…å¯¹](/gateway/pairing)

