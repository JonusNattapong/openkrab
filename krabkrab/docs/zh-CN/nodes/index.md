---
read_when:
  - å°† iOS/Android èŠ‚ç‚¹é…å¯¹åˆ° Gateway ç½‘å…³æ—¶
  - ä½¿ç”¨èŠ‚ç‚¹ canvas/camera ä¸ºæ™ºèƒ½ä½“æä¾›ä¸Šä¸‹æ–‡æ—¶
  - æ·»åŠ æ–°çš„èŠ‚ç‚¹å‘½ä»¤æˆ– CLI è¾…åŠ©å·¥å…·æ—¶
summary: èŠ‚ç‚¹ï¼šé…å¯¹ã€èƒ½åŠ›ã€æƒé™ä»¥åŠ canvas/camera/screen/system çš„ CLI è¾…åŠ©å·¥å…·
title: èŠ‚ç‚¹
x-i18n:
  generated_at: "2026-02-03T07:51:55Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 74e9420f61c653e4ceeb00f5a27e4266bd1c7715c1000edd969c3ee185e74de9
  source_path: nodes/index.md
  workflow: 15
---

# èŠ‚ç‚¹

**èŠ‚ç‚¹**æ˜¯ä¸€ä¸ªé…å¥—è®¾å¤‡ï¼ˆmacOS/iOS/Android/æ— å¤´ï¼‰ï¼Œå®ƒä»¥ `role: "node"` è¿žæŽ¥åˆ° Gateway ç½‘å…³ **WebSocket**ï¼ˆä¸Žæ“ä½œå‘˜ç›¸åŒçš„ç«¯å£ï¼‰ï¼Œå¹¶é€šè¿‡ `node.invoke` æš´éœ²å‘½ä»¤æŽ¥å£ï¼ˆä¾‹å¦‚ `canvas.*`ã€`camera.*`ã€`system.*`ï¼‰ã€‚åè®®è¯¦æƒ…ï¼š[Gateway ç½‘å…³åè®®](/gateway/protocol)ã€‚

æ—§ç‰ˆä¼ è¾“ï¼š[Bridge åè®®](/gateway/bridge-protocol)ï¼ˆTCP JSONLï¼›å½“å‰èŠ‚ç‚¹å·²å¼ƒç”¨/ç§»é™¤ï¼‰ã€‚

macOS ä¹Ÿå¯ä»¥åœ¨**èŠ‚ç‚¹æ¨¡å¼**ä¸‹è¿è¡Œï¼šèœå•æ åº”ç”¨è¿žæŽ¥åˆ° Gateway ç½‘å…³çš„ WS æœåŠ¡å™¨ï¼Œå¹¶å°†å…¶æœ¬åœ° canvas/camera å‘½ä»¤ä½œä¸ºèŠ‚ç‚¹æš´éœ²ï¼ˆå› æ­¤ `krabkrab nodes â€¦` å¯ä»¥é’ˆå¯¹è¿™å° Mac å·¥ä½œï¼‰ã€‚

æ³¨æ„äº‹é¡¹ï¼š

- èŠ‚ç‚¹æ˜¯**å¤–å›´è®¾å¤‡**ï¼Œä¸æ˜¯ Gateway ç½‘å…³ã€‚å®ƒä»¬ä¸è¿è¡Œ Gateway ç½‘å…³æœåŠ¡ã€‚
- Telegram/WhatsApp ç­‰æ¶ˆæ¯è½åœ¨ **Gateway ç½‘å…³**ä¸Šï¼Œè€Œä¸æ˜¯èŠ‚ç‚¹ä¸Šã€‚

## é…å¯¹ + çŠ¶æ€

**WS èŠ‚ç‚¹ä½¿ç”¨è®¾å¤‡é…å¯¹ã€‚** èŠ‚ç‚¹åœ¨ `connect` æœŸé—´å‘ˆçŽ°è®¾å¤‡èº«ä»½ï¼›Gateway ç½‘å…³
ä¸º `role: node` åˆ›å»ºè®¾å¤‡é…å¯¹è¯·æ±‚ã€‚é€šè¿‡è®¾å¤‡ CLIï¼ˆæˆ– UIï¼‰æ‰¹å‡†ã€‚

å¿«é€Ÿ CLIï¼š

```bash
krabkrab devices list
krabkrab devices approve <requestId>
krabkrab devices reject <requestId>
krabkrab nodes status
krabkrab nodes describe --node <idOrNameOrIp>
```

æ³¨æ„äº‹é¡¹ï¼š

- å½“èŠ‚ç‚¹çš„è®¾å¤‡é…å¯¹è§’è‰²åŒ…å« `node` æ—¶ï¼Œ`nodes status` å°†èŠ‚ç‚¹æ ‡è®°ä¸º**å·²é…å¯¹**ã€‚
- `node.pair.*`ï¼ˆCLIï¼š`krabkrab nodes pending/approve/reject`ï¼‰æ˜¯ä¸€ä¸ªå•ç‹¬çš„ Gateway ç½‘å…³æ‹¥æœ‰çš„
  èŠ‚ç‚¹é…å¯¹å­˜å‚¨ï¼›å®ƒ**ä¸ä¼š**é™åˆ¶ WS `connect` æ¡æ‰‹ã€‚

## è¿œç¨‹èŠ‚ç‚¹ä¸»æœºï¼ˆsystem.runï¼‰

å½“ä½ çš„ Gateway ç½‘å…³åœ¨ä¸€å°æœºå™¨ä¸Šè¿è¡Œè€Œä½ å¸Œæœ›å‘½ä»¤
åœ¨å¦ä¸€å°æœºå™¨ä¸Šæ‰§è¡Œæ—¶ï¼Œä½¿ç”¨**èŠ‚ç‚¹ä¸»æœº**ã€‚æ¨¡åž‹ä»ç„¶ä¸Ž **Gateway ç½‘å…³**é€šä¿¡ï¼›å½“é€‰æ‹© `host=node` æ—¶ï¼ŒGateway ç½‘å…³
å°† `exec` è°ƒç”¨è½¬å‘åˆ°**èŠ‚ç‚¹ä¸»æœº**ã€‚

### ä»€ä¹ˆåœ¨å“ªé‡Œè¿è¡Œ

- **Gateway ç½‘å…³ä¸»æœº**ï¼šæŽ¥æ”¶æ¶ˆæ¯ï¼Œè¿è¡Œæ¨¡åž‹ï¼Œè·¯ç”±å·¥å…·è°ƒç”¨ã€‚
- **èŠ‚ç‚¹ä¸»æœº**ï¼šåœ¨èŠ‚ç‚¹æœºå™¨ä¸Šæ‰§è¡Œ `system.run`/`system.which`ã€‚
- **æ‰¹å‡†**ï¼šé€šè¿‡ `~/.krabkrab/exec-approvals.json` åœ¨èŠ‚ç‚¹ä¸»æœºä¸Šæ‰§è¡Œã€‚

### å¯åŠ¨èŠ‚ç‚¹ä¸»æœºï¼ˆå‰å°ï¼‰

åœ¨èŠ‚ç‚¹æœºå™¨ä¸Šï¼š

```bash
krabkrab node run --host <gateway-host> --port 18789 --display-name "Build Node"
```

### é€šè¿‡ SSH éš§é“è®¿é—®è¿œç¨‹ Gateway ç½‘å…³ï¼ˆloopback ç»‘å®šï¼‰

å¦‚æžœ Gateway ç½‘å…³ç»‘å®šåˆ° loopbackï¼ˆ`gateway.bind=loopback`ï¼Œæœ¬åœ°æ¨¡å¼ä¸‹çš„é»˜è®¤å€¼ï¼‰ï¼Œ
è¿œç¨‹èŠ‚ç‚¹ä¸»æœºæ— æ³•ç›´æŽ¥è¿žæŽ¥ã€‚åˆ›å»º SSH éš§é“å¹¶å°†
èŠ‚ç‚¹ä¸»æœºæŒ‡å‘éš§é“çš„æœ¬åœ°ç«¯ã€‚

ç¤ºä¾‹ï¼ˆèŠ‚ç‚¹ä¸»æœº -> Gateway ç½‘å…³ä¸»æœºï¼‰ï¼š

```bash
# ç»ˆç«¯ Aï¼ˆä¿æŒè¿è¡Œï¼‰ï¼šè½¬å‘æœ¬åœ° 18790 -> Gateway ç½‘å…³ 127.0.0.1:18789
ssh -N -L 18790:127.0.0.1:18789 user@gateway-host

# ç»ˆç«¯ Bï¼šå¯¼å‡º Gateway ç½‘å…³ä»¤ç‰Œå¹¶é€šè¿‡éš§é“è¿žæŽ¥
export krabkrab_GATEWAY_TOKEN="<gateway-token>"
krabkrab node run --host 127.0.0.1 --port 18790 --display-name "Build Node"
```

æ³¨æ„äº‹é¡¹ï¼š

- ä»¤ç‰Œæ˜¯ Gateway ç½‘å…³é…ç½®ä¸­çš„ `gateway.auth.token`ï¼ˆGateway ç½‘å…³ä¸»æœºä¸Šçš„ `~/.krabkrab/krabkrab.json`ï¼‰ã€‚
- `krabkrab node run` è¯»å– `krabkrab_GATEWAY_TOKEN` è¿›è¡Œè®¤è¯ã€‚

### å¯åŠ¨èŠ‚ç‚¹ä¸»æœºï¼ˆæœåŠ¡ï¼‰

```bash
krabkrab node install --host <gateway-host> --port 18789 --display-name "Build Node"
krabkrab node restart
```

### é…å¯¹ + å‘½å

åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šï¼š

```bash
krabkrab nodes pending
krabkrab nodes approve <requestId>
krabkrab nodes list
```

å‘½åé€‰é¡¹ï¼š

- åœ¨ `krabkrab node run` / `krabkrab node install` ä¸Šä½¿ç”¨ `--display-name`ï¼ˆæŒä¹…åŒ–åœ¨èŠ‚ç‚¹ä¸Šçš„ `~/.krabkrab/node.json` ä¸­ï¼‰ã€‚
- `krabkrab nodes rename --node <id|name|ip> --name "Build Node"`ï¼ˆGateway ç½‘å…³è¦†ç›–ï¼‰ã€‚

### å°†å‘½ä»¤åŠ å…¥å…è®¸åˆ—è¡¨

Exec æ‰¹å‡†æ˜¯**æ¯ä¸ªèŠ‚ç‚¹ä¸»æœº**çš„ã€‚ä»Ž Gateway ç½‘å…³æ·»åŠ å…è®¸åˆ—è¡¨æ¡ç›®ï¼š

```bash
krabkrab approvals allowlist add --node <id|name|ip> "/usr/bin/uname"
krabkrab approvals allowlist add --node <id|name|ip> "/usr/bin/sw_vers"
```

æ‰¹å‡†å­˜å‚¨åœ¨èŠ‚ç‚¹ä¸»æœºçš„ `~/.krabkrab/exec-approvals.json` ä¸­ã€‚

### å°† exec æŒ‡å‘èŠ‚ç‚¹

é…ç½®é»˜è®¤å€¼ï¼ˆGateway ç½‘å…³é…ç½®ï¼‰ï¼š

```bash
krabkrab config set tools.exec.host node
krabkrab config set tools.exec.security allowlist
krabkrab config set tools.exec.node "<id-or-name>"
```

æˆ–æŒ‰ä¼šè¯ï¼š

```
/exec host=node security=allowlist node=<id-or-name>
```

è®¾ç½®åŽï¼Œä»»ä½•å¸¦æœ‰ `host=node` çš„ `exec` è°ƒç”¨éƒ½ä¼šåœ¨èŠ‚ç‚¹ä¸»æœºä¸Šè¿è¡Œï¼ˆå—
èŠ‚ç‚¹å…è®¸åˆ—è¡¨/æ‰¹å‡†çº¦æŸï¼‰ã€‚

ç›¸å…³ï¼š

- [èŠ‚ç‚¹ä¸»æœº CLI](/cli/node)
- [Exec å·¥å…·](/tools/exec)
- [Exec æ‰¹å‡†](/tools/exec-approvals)

## è°ƒç”¨å‘½ä»¤

ä½Žçº§ï¼ˆåŽŸå§‹ RPCï¼‰ï¼š

```bash
krabkrab nodes invoke --node <idOrNameOrIp> --command canvas.eval --params '{"javaScript":"location.href"}'
```

å¯¹äºŽå¸¸è§çš„"ç»™æ™ºèƒ½ä½“ä¸€ä¸ª MEDIA é™„ä»¶"å·¥ä½œæµï¼Œå­˜åœ¨æ›´é«˜çº§çš„è¾…åŠ©å·¥å…·ã€‚

## æˆªå›¾ï¼ˆcanvas å¿«ç…§ï¼‰

å¦‚æžœèŠ‚ç‚¹æ­£åœ¨æ˜¾ç¤º Canvasï¼ˆWebViewï¼‰ï¼Œ`canvas.snapshot` è¿”å›ž `{ format, base64 }`ã€‚

CLI è¾…åŠ©å·¥å…·ï¼ˆå†™å…¥ä¸´æ—¶æ–‡ä»¶å¹¶æ‰“å° `MEDIA:<path>`ï¼‰ï¼š

```bash
krabkrab nodes canvas snapshot --node <idOrNameOrIp> --format png
krabkrab nodes canvas snapshot --node <idOrNameOrIp> --format jpg --max-width 1200 --quality 0.9
```

### Canvas æŽ§åˆ¶

```bash
krabkrab nodes canvas present --node <idOrNameOrIp> --target https://example.com
krabkrab nodes canvas hide --node <idOrNameOrIp>
krabkrab nodes canvas navigate https://example.com --node <idOrNameOrIp>
krabkrab nodes canvas eval --node <idOrNameOrIp> --js "document.title"
```

æ³¨æ„äº‹é¡¹ï¼š

- `canvas present` æŽ¥å— URL æˆ–æœ¬åœ°æ–‡ä»¶è·¯å¾„ï¼ˆ`--target`ï¼‰ï¼Œä»¥åŠå¯é€‰çš„ `--x/--y/--width/--height` ç”¨äºŽå®šä½ã€‚
- `canvas eval` æŽ¥å—å†…è” JSï¼ˆ`--js`ï¼‰æˆ–ä½ç½®å‚æ•°ã€‚

### A2UIï¼ˆCanvasï¼‰

```bash
krabkrab nodes canvas a2ui push --node <idOrNameOrIp> --text "Hello"
krabkrab nodes canvas a2ui push --node <idOrNameOrIp> --jsonl ./payload.jsonl
krabkrab nodes canvas a2ui reset --node <idOrNameOrIp>
```

æ³¨æ„äº‹é¡¹ï¼š

- ä»…æ”¯æŒ A2UI v0.8 JSONLï¼ˆv0.9/createSurface è¢«æ‹’ç»ï¼‰ã€‚

## ç…§ç‰‡ + è§†é¢‘ï¼ˆèŠ‚ç‚¹ç›¸æœºï¼‰

ç…§ç‰‡ï¼ˆ`jpg`ï¼‰ï¼š

```bash
krabkrab nodes camera list --node <idOrNameOrIp>
krabkrab nodes camera snap --node <idOrNameOrIp>            # é»˜è®¤ï¼šä¸¤ä¸ªæœå‘ï¼ˆ2 ä¸ª MEDIA è¡Œï¼‰
krabkrab nodes camera snap --node <idOrNameOrIp> --facing front
```

è§†é¢‘ç‰‡æ®µï¼ˆ`mp4`ï¼‰ï¼š

```bash
krabkrab nodes camera clip --node <idOrNameOrIp> --duration 10s
krabkrab nodes camera clip --node <idOrNameOrIp> --duration 3000 --no-audio
```

æ³¨æ„äº‹é¡¹ï¼š

- èŠ‚ç‚¹å¿…é¡»å¤„äºŽ**å‰å°**æ‰èƒ½ä½¿ç”¨ `canvas.*` å’Œ `camera.*`ï¼ˆåŽå°è°ƒç”¨è¿”å›ž `NODE_BACKGROUND_UNAVAILABLE`ï¼‰ã€‚
- ç‰‡æ®µæ—¶é•¿è¢«é™åˆ¶ï¼ˆå½“å‰ `<= 60s`ï¼‰ä»¥é¿å…è¿‡å¤§çš„ base64 è´Ÿè½½ã€‚
- Android ä¼šåœ¨å¯èƒ½æ—¶æç¤º `CAMERA`/`RECORD_AUDIO` æƒé™ï¼›æƒé™è¢«æ‹’ç»ä¼šä»¥ `*_PERMISSION_REQUIRED` å¤±è´¥ã€‚

## å±å¹•å½•åˆ¶ï¼ˆèŠ‚ç‚¹ï¼‰

èŠ‚ç‚¹æš´éœ² `screen.record`ï¼ˆmp4ï¼‰ã€‚ç¤ºä¾‹ï¼š

```bash
krabkrab nodes screen record --node <idOrNameOrIp> --duration 10s --fps 10
krabkrab nodes screen record --node <idOrNameOrIp> --duration 10s --fps 10 --no-audio
```

æ³¨æ„äº‹é¡¹ï¼š

- `screen.record` éœ€è¦èŠ‚ç‚¹åº”ç”¨å¤„äºŽå‰å°ã€‚
- Android ä¼šåœ¨å½•åˆ¶å‰æ˜¾ç¤ºç³»ç»Ÿå±å¹•æ•èŽ·æç¤ºã€‚
- å±å¹•å½•åˆ¶è¢«é™åˆ¶ä¸º `<= 60s`ã€‚
- `--no-audio` ç¦ç”¨éº¦å…‹é£Žæ•èŽ·ï¼ˆiOS/Android æ”¯æŒï¼›macOS ä½¿ç”¨ç³»ç»Ÿæ•èŽ·éŸ³é¢‘ï¼‰ã€‚
- å½“æœ‰å¤šä¸ªå±å¹•å¯ç”¨æ—¶ï¼Œä½¿ç”¨ `--screen <index>` é€‰æ‹©æ˜¾ç¤ºå™¨ã€‚

## ä½ç½®ï¼ˆèŠ‚ç‚¹ï¼‰

å½“åœ¨è®¾ç½®ä¸­å¯ç”¨ä½ç½®æ—¶ï¼ŒèŠ‚ç‚¹æš´éœ² `location.get`ã€‚

CLI è¾…åŠ©å·¥å…·ï¼š

```bash
krabkrab nodes location get --node <idOrNameOrIp>
krabkrab nodes location get --node <idOrNameOrIp> --accuracy precise --max-age 15000 --location-timeout 10000
```

æ³¨æ„äº‹é¡¹ï¼š

- ä½ç½®**é»˜è®¤å…³é—­**ã€‚
- "å§‹ç»ˆ"éœ€è¦ç³»ç»Ÿæƒé™ï¼›åŽå°èŽ·å–æ˜¯å°½åŠ›è€Œä¸ºçš„ã€‚
- å“åº”åŒ…æ‹¬çº¬åº¦/ç»åº¦ã€ç²¾åº¦ï¼ˆç±³ï¼‰å’Œæ—¶é—´æˆ³ã€‚

## çŸ­ä¿¡ï¼ˆAndroid èŠ‚ç‚¹ï¼‰

å½“ç”¨æˆ·æŽˆäºˆ **SMS** æƒé™ä¸”è®¾å¤‡æ”¯æŒç”µè¯åŠŸèƒ½æ—¶ï¼ŒAndroid èŠ‚ç‚¹å¯ä»¥æš´éœ² `sms.send`ã€‚

ä½Žçº§è°ƒç”¨ï¼š

```bash
krabkrab nodes invoke --node <idOrNameOrIp> --command sms.send --params '{"to":"+15555550123","message":"Hello from KrabKrab"}'
```

æ³¨æ„äº‹é¡¹ï¼š

- åœ¨èƒ½åŠ›è¢«å¹¿æ’­ä¹‹å‰ï¼Œå¿…é¡»åœ¨ Android è®¾å¤‡ä¸ŠæŽ¥å—æƒé™æç¤ºã€‚
- æ²¡æœ‰ç”µè¯åŠŸèƒ½çš„çº¯ Wi-Fi è®¾å¤‡ä¸ä¼šå¹¿æ’­ `sms.send`ã€‚

## ç³»ç»Ÿå‘½ä»¤ï¼ˆèŠ‚ç‚¹ä¸»æœº / mac èŠ‚ç‚¹ï¼‰

macOS èŠ‚ç‚¹æš´éœ² `system.run`ã€`system.notify` å’Œ `system.execApprovals.get/set`ã€‚
æ— å¤´èŠ‚ç‚¹ä¸»æœºæš´éœ² `system.run`ã€`system.which` å’Œ `system.execApprovals.get/set`ã€‚

ç¤ºä¾‹ï¼š

```bash
krabkrab nodes run --node <idOrNameOrIp> -- echo "Hello from mac node"
krabkrab nodes notify --node <idOrNameOrIp> --title "Ping" --body "Gateway ready"
```

æ³¨æ„äº‹é¡¹ï¼š

- `system.run` åœ¨è´Ÿè½½ä¸­è¿”å›ž stdout/stderr/é€€å‡ºç ã€‚
- `system.notify` éµå®ˆ macOS åº”ç”¨ä¸Šçš„é€šçŸ¥æƒé™çŠ¶æ€ã€‚
- `system.run` æ”¯æŒ `--cwd`ã€`--env KEY=VAL`ã€`--command-timeout` å’Œ `--needs-screen-recording`ã€‚
- `system.notify` æ”¯æŒ `--priority <passive|active|timeSensitive>` å’Œ `--delivery <system|overlay|auto>`ã€‚
- macOS èŠ‚ç‚¹ä¼šä¸¢å¼ƒ `PATH` è¦†ç›–ï¼›æ— å¤´èŠ‚ç‚¹ä¸»æœºä»…åœ¨ `PATH` å‰ç½®åˆ°èŠ‚ç‚¹ä¸»æœº PATH æ—¶æ‰æŽ¥å—å®ƒã€‚
- åœ¨ macOS èŠ‚ç‚¹æ¨¡å¼ä¸‹ï¼Œ`system.run` å— macOS åº”ç”¨ä¸­çš„ exec æ‰¹å‡†é™åˆ¶ï¼ˆè®¾ç½® â†’ Exec æ‰¹å‡†ï¼‰ã€‚
  Ask/allowlist/full çš„è¡Œä¸ºä¸Žæ— å¤´èŠ‚ç‚¹ä¸»æœºç›¸åŒï¼›è¢«æ‹’ç»çš„æç¤ºè¿”å›ž `SYSTEM_RUN_DENIED`ã€‚
- åœ¨æ— å¤´èŠ‚ç‚¹ä¸»æœºä¸Šï¼Œ`system.run` å— exec æ‰¹å‡†é™åˆ¶ï¼ˆ`~/.krabkrab/exec-approvals.json`ï¼‰ã€‚

## Exec èŠ‚ç‚¹ç»‘å®š

å½“æœ‰å¤šä¸ªèŠ‚ç‚¹å¯ç”¨æ—¶ï¼Œä½ å¯ä»¥å°† exec ç»‘å®šåˆ°ç‰¹å®šèŠ‚ç‚¹ã€‚
è¿™è®¾ç½®äº† `exec host=node` çš„é»˜è®¤èŠ‚ç‚¹ï¼ˆå¯ä»¥æŒ‰æ™ºèƒ½ä½“è¦†ç›–ï¼‰ã€‚

å…¨å±€é»˜è®¤ï¼š

```bash
krabkrab config set tools.exec.node "node-id-or-name"
```

æŒ‰æ™ºèƒ½ä½“è¦†ç›–ï¼š

```bash
krabkrab config get agents.list
krabkrab config set agents.list[0].tools.exec.node "node-id-or-name"
```

å–æ¶ˆè®¾ç½®ä»¥å…è®¸ä»»ä½•èŠ‚ç‚¹ï¼š

```bash
krabkrab config unset tools.exec.node
krabkrab config unset agents.list[0].tools.exec.node
```

## æƒé™æ˜ å°„

èŠ‚ç‚¹å¯èƒ½åœ¨ `node.list` / `node.describe` ä¸­åŒ…å« `permissions` æ˜ å°„ï¼ŒæŒ‰æƒé™åç§°ï¼ˆä¾‹å¦‚ `screenRecording`ã€`accessibility`ï¼‰é”®å…¥ï¼Œå€¼ä¸ºå¸ƒå°”å€¼ï¼ˆ`true` = å·²æŽˆäºˆï¼‰ã€‚

## æ— å¤´èŠ‚ç‚¹ä¸»æœºï¼ˆè·¨å¹³å°ï¼‰

KrabKrab å¯ä»¥è¿è¡Œ**æ— å¤´èŠ‚ç‚¹ä¸»æœº**ï¼ˆæ—  UIï¼‰ï¼Œå®ƒè¿žæŽ¥åˆ° Gateway ç½‘å…³
WebSocket å¹¶æš´éœ² `system.run` / `system.which`ã€‚è¿™åœ¨ Linux/Windows
ä¸Šæˆ–åœ¨æœåŠ¡å™¨æ—è¿è¡Œæœ€å°èŠ‚ç‚¹æ—¶å¾ˆæœ‰ç”¨ã€‚

å¯åŠ¨å®ƒï¼š

```bash
krabkrab node run --host <gateway-host> --port 18789
```

æ³¨æ„äº‹é¡¹ï¼š

- ä»ç„¶éœ€è¦é…å¯¹ï¼ˆGateway ç½‘å…³ä¼šæ˜¾ç¤ºèŠ‚ç‚¹æ‰¹å‡†æç¤ºï¼‰ã€‚
- èŠ‚ç‚¹ä¸»æœºå°†å…¶èŠ‚ç‚¹ idã€ä»¤ç‰Œã€æ˜¾ç¤ºåç§°å’Œ Gateway ç½‘å…³è¿žæŽ¥ä¿¡æ¯å­˜å‚¨åœ¨ `~/.krabkrab/node.json` ä¸­ã€‚
- Exec æ‰¹å‡†é€šè¿‡ `~/.krabkrab/exec-approvals.json` åœ¨æœ¬åœ°æ‰§è¡Œ
  ï¼ˆå‚è§ [Exec æ‰¹å‡†](/tools/exec-approvals)ï¼‰ã€‚
- åœ¨ macOS ä¸Šï¼Œå½“é…å¥—åº”ç”¨ exec ä¸»æœºå¯è¾¾æ—¶ï¼Œæ— å¤´èŠ‚ç‚¹ä¸»æœºä¼˜å…ˆä½¿ç”¨å®ƒï¼Œ
  å¦‚æžœåº”ç”¨ä¸å¯ç”¨åˆ™å›žé€€åˆ°æœ¬åœ°æ‰§è¡Œã€‚è®¾ç½® `krabkrab_NODE_EXEC_HOST=app` è¦æ±‚
  ä½¿ç”¨åº”ç”¨ï¼Œæˆ–è®¾ç½® `krabkrab_NODE_EXEC_FALLBACK=0` ç¦ç”¨å›žé€€ã€‚
- å½“ Gateway ç½‘å…³ WS ä½¿ç”¨ TLS æ—¶ï¼Œæ·»åŠ  `--tls` / `--tls-fingerprint`ã€‚

## Mac èŠ‚ç‚¹æ¨¡å¼

- macOS èœå•æ åº”ç”¨ä½œä¸ºèŠ‚ç‚¹è¿žæŽ¥åˆ° Gateway ç½‘å…³ WS æœåŠ¡å™¨ï¼ˆå› æ­¤ `krabkrab nodes â€¦` å¯ä»¥é’ˆå¯¹è¿™å° Mac å·¥ä½œï¼‰ã€‚
- åœ¨è¿œç¨‹æ¨¡å¼ä¸‹ï¼Œåº”ç”¨ä¸º Gateway ç½‘å…³ç«¯å£æ‰“å¼€ SSH éš§é“å¹¶è¿žæŽ¥åˆ° `localhost`ã€‚

