---
read_when:
  - ä»Ž CLI è¿è¡Œ Gateway ç½‘å…³ï¼ˆå¼€å‘æˆ–æœåŠ¡å™¨ï¼‰
  - è°ƒè¯• Gateway ç½‘å…³è®¤è¯ã€ç»‘å®šæ¨¡å¼å’Œè¿žæŽ¥æ€§
  - é€šè¿‡ Bonjour å‘çŽ° Gateway ç½‘å…³ï¼ˆå±€åŸŸç½‘ + tailnetï¼‰
summary: KrabKrab Gateway ç½‘å…³ CLIï¼ˆ`krabkrab gateway`ï¼‰â€” è¿è¡Œã€æŸ¥è¯¢å’Œå‘çŽ° Gateway ç½‘å…³
title: gateway
x-i18n:
  generated_at: "2026-02-03T07:45:15Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 054dd48056e4784f153c6511c8eb35b56f239db8d4e629661841a00259e9abbf
  source_path: cli/gateway.md
  workflow: 15
---

# Gateway ç½‘å…³ CLI

Gateway ç½‘å…³æ˜¯ KrabKrab çš„ WebSocket æœåŠ¡å™¨ï¼ˆæ¸ é“ã€èŠ‚ç‚¹ã€ä¼šè¯ã€hooksï¼‰ã€‚

æœ¬é¡µä¸­çš„å­å‘½ä»¤ä½äºŽ `krabkrab gateway â€¦` ä¸‹ã€‚

ç›¸å…³æ–‡æ¡£ï¼š

- [/gateway/bonjour](/gateway/bonjour)
- [/gateway/discovery](/gateway/discovery)
- [/gateway/configuration](/gateway/configuration)

## è¿è¡Œ Gateway ç½‘å…³

è¿è¡Œæœ¬åœ° Gateway ç½‘å…³è¿›ç¨‹ï¼š

```bash
krabkrab gateway
```

å‰å°è¿è¡Œåˆ«åï¼š

```bash
krabkrab gateway run
```

æ³¨æ„äº‹é¡¹ï¼š

- é»˜è®¤æƒ…å†µä¸‹ï¼Œé™¤éžåœ¨ `~/.krabkrab/krabkrab.json` ä¸­è®¾ç½®äº† `gateway.mode=local`ï¼Œå¦åˆ™ Gateway ç½‘å…³å°†æ‹’ç»å¯åŠ¨ã€‚ä½¿ç”¨ `--allow-unconfigured` è¿›è¡Œä¸´æ—¶/å¼€å‘è¿è¡Œã€‚
- åœ¨æ²¡æœ‰è®¤è¯çš„æƒ…å†µä¸‹ç»‘å®šåˆ° loopback ä¹‹å¤–çš„åœ°å€ä¼šè¢«é˜»æ­¢ï¼ˆå®‰å…¨æŠ¤æ ï¼‰ã€‚
- `SIGUSR1` åœ¨æŽˆæƒæ—¶è§¦å‘è¿›ç¨‹å†…é‡å¯ï¼ˆå¯ç”¨ `commands.restart` æˆ–ä½¿ç”¨ gateway å·¥å…·/config apply/updateï¼‰ã€‚
- `SIGINT`/`SIGTERM` å¤„ç†ç¨‹åºä¼šåœæ­¢ Gateway ç½‘å…³è¿›ç¨‹ï¼Œä½†ä¸ä¼šæ¢å¤ä»»ä½•è‡ªå®šä¹‰ç»ˆç«¯çŠ¶æ€ã€‚å¦‚æžœä½ ç”¨ TUI æˆ– raw-mode è¾“å…¥åŒ…è£… CLIï¼Œè¯·åœ¨é€€å‡ºå‰æ¢å¤ç»ˆç«¯ã€‚

### é€‰é¡¹

- `--port <port>`ï¼šWebSocket ç«¯å£ï¼ˆé»˜è®¤æ¥è‡ªé…ç½®/çŽ¯å¢ƒå˜é‡ï¼›é€šå¸¸ä¸º `18789`ï¼‰ã€‚
- `--bind <loopback|lan|tailnet|auto|custom>`ï¼šç›‘å¬å™¨ç»‘å®šæ¨¡å¼ã€‚
- `--auth <token|password>`ï¼šè®¤è¯æ¨¡å¼è¦†ç›–ã€‚
- `--token <token>`ï¼šä»¤ç‰Œè¦†ç›–ï¼ˆåŒæ—¶ä¸ºè¿›ç¨‹è®¾ç½® `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- `--password <password>`ï¼šå¯†ç è¦†ç›–ï¼ˆåŒæ—¶ä¸ºè¿›ç¨‹è®¾ç½® `krabkrab_GATEWAY_PASSWORD`ï¼‰ã€‚
- `--tailscale <off|serve|funnel>`ï¼šé€šè¿‡ Tailscale æš´éœ² Gateway ç½‘å…³ã€‚
- `--tailscale-reset-on-exit`ï¼šå…³é—­æ—¶é‡ç½® Tailscale serve/funnel é…ç½®ã€‚
- `--allow-unconfigured`ï¼šå…è®¸åœ¨é…ç½®ä¸­æ²¡æœ‰ `gateway.mode=local` çš„æƒ…å†µä¸‹å¯åŠ¨ Gateway ç½‘å…³ã€‚
- `--dev`ï¼šå¦‚æžœç¼ºå¤±åˆ™åˆ›å»ºå¼€å‘é…ç½® + å·¥ä½œåŒºï¼ˆè·³è¿‡ BOOTSTRAP.mdï¼‰ã€‚
- `--reset`ï¼šé‡ç½®å¼€å‘é…ç½® + å‡­è¯ + ä¼šè¯ + å·¥ä½œåŒºï¼ˆéœ€è¦ `--dev`ï¼‰ã€‚
- `--force`ï¼šå¯åŠ¨å‰æ€æ­»æ‰€é€‰ç«¯å£ä¸Šçš„ä»»ä½•çŽ°æœ‰ç›‘å¬å™¨ã€‚
- `--verbose`ï¼šè¯¦ç»†æ—¥å¿—ã€‚
- `--claude-cli-logs`ï¼šä»…åœ¨æŽ§åˆ¶å°æ˜¾ç¤º claude-cli æ—¥å¿—ï¼ˆå¹¶å¯ç”¨å…¶ stdout/stderrï¼‰ã€‚
- `--ws-log <auto|full|compact>`ï¼šWebSocket æ—¥å¿—æ ·å¼ï¼ˆé»˜è®¤ `auto`ï¼‰ã€‚
- `--compact`ï¼š`--ws-log compact` çš„åˆ«åã€‚
- `--raw-stream`ï¼šå°†åŽŸå§‹æ¨¡åž‹æµäº‹ä»¶è®°å½•åˆ° jsonlã€‚
- `--raw-stream-path <path>`ï¼šåŽŸå§‹æµ jsonl è·¯å¾„ã€‚

## æŸ¥è¯¢è¿è¡Œä¸­çš„ Gateway ç½‘å…³

æ‰€æœ‰æŸ¥è¯¢å‘½ä»¤ä½¿ç”¨ WebSocket RPCã€‚

è¾“å‡ºæ¨¡å¼ï¼š

- é»˜è®¤ï¼šäººç±»å¯è¯»ï¼ˆTTY ä¸­å¸¦é¢œè‰²ï¼‰ã€‚
- `--json`ï¼šæœºå™¨å¯è¯» JSONï¼ˆæ— æ ·å¼/è¿›åº¦æŒ‡ç¤ºå™¨ï¼‰ã€‚
- `--no-color`ï¼ˆæˆ– `NO_COLOR=1`ï¼‰ï¼šç¦ç”¨ ANSI ä½†ä¿æŒäººç±»å¯è¯»å¸ƒå±€ã€‚

å…±äº«é€‰é¡¹ï¼ˆåœ¨æ”¯æŒçš„åœ°æ–¹ï¼‰ï¼š

- `--url <url>`ï¼šGateway ç½‘å…³ WebSocket URLã€‚
- `--token <token>`ï¼šGateway ç½‘å…³ä»¤ç‰Œã€‚
- `--password <password>`ï¼šGateway ç½‘å…³å¯†ç ã€‚
- `--timeout <ms>`ï¼šè¶…æ—¶/é¢„ç®—ï¼ˆå› å‘½ä»¤è€Œå¼‚ï¼‰ã€‚
- `--expect-final`ï¼šç­‰å¾…"æœ€ç»ˆ"å“åº”ï¼ˆæ™ºèƒ½ä½“è°ƒç”¨ï¼‰ã€‚

### `gateway health`

```bash
krabkrab gateway health --url ws://127.0.0.1:18789
```

### `gateway status`

`gateway status` æ˜¾ç¤º Gateway ç½‘å…³æœåŠ¡ï¼ˆlaunchd/systemd/schtasksï¼‰ä»¥åŠå¯é€‰çš„ RPC æŽ¢æµ‹ã€‚

```bash
krabkrab gateway status
krabkrab gateway status --json
```

é€‰é¡¹ï¼š

- `--url <url>`ï¼šè¦†ç›–æŽ¢æµ‹ URLã€‚
- `--token <token>`ï¼šæŽ¢æµ‹çš„ä»¤ç‰Œè®¤è¯ã€‚
- `--password <password>`ï¼šæŽ¢æµ‹çš„å¯†ç è®¤è¯ã€‚
- `--timeout <ms>`ï¼šæŽ¢æµ‹è¶…æ—¶ï¼ˆé»˜è®¤ `10000`ï¼‰ã€‚
- `--no-probe`ï¼šè·³è¿‡ RPC æŽ¢æµ‹ï¼ˆä»…æœåŠ¡è§†å›¾ï¼‰ã€‚
- `--deep`ï¼šä¹Ÿæ‰«æç³»ç»Ÿçº§æœåŠ¡ã€‚

### `gateway probe`

`gateway probe` æ˜¯"è°ƒè¯•ä¸€åˆ‡"å‘½ä»¤ã€‚å®ƒå§‹ç»ˆæŽ¢æµ‹ï¼š

- ä½ é…ç½®çš„è¿œç¨‹ Gateway ç½‘å…³ï¼ˆå¦‚æžœè®¾ç½®äº†ï¼‰ï¼Œä»¥åŠ
- localhostï¼ˆloopbackï¼‰**å³ä½¿é…ç½®äº†è¿œç¨‹ä¹Ÿä¼šæŽ¢æµ‹**ã€‚

å¦‚æžœå¤šä¸ª Gateway ç½‘å…³å¯è¾¾ï¼Œå®ƒä¼šæ‰“å°æ‰€æœ‰ã€‚å½“ä½ ä½¿ç”¨éš”ç¦»çš„é…ç½®æ–‡ä»¶/ç«¯å£ï¼ˆä¾‹å¦‚æ•‘æ´æœºå™¨äººï¼‰æ—¶æ”¯æŒå¤šä¸ª Gateway ç½‘å…³ï¼Œä½†å¤§å¤šæ•°å®‰è£…ä»ç„¶è¿è¡Œå•ä¸ª Gateway ç½‘å…³ã€‚

```bash
krabkrab gateway probe
krabkrab gateway probe --json
```

#### é€šè¿‡ SSH è¿œç¨‹ï¼ˆMac åº”ç”¨å¯¹ç­‰ï¼‰

macOS åº”ç”¨çš„"é€šè¿‡ SSH è¿œç¨‹"æ¨¡å¼ä½¿ç”¨æœ¬åœ°ç«¯å£è½¬å‘ï¼Œå› æ­¤è¿œç¨‹ Gateway ç½‘å…³ï¼ˆå¯èƒ½ä»…ç»‘å®šåˆ° loopbackï¼‰å˜å¾—å¯ä»¥é€šè¿‡ `ws://127.0.0.1:<port>` è®¿é—®ã€‚

CLI ç­‰æ•ˆå‘½ä»¤ï¼š

```bash
krabkrab gateway probe --ssh user@gateway-host
```

é€‰é¡¹ï¼š

- `--ssh <target>`ï¼š`user@host` æˆ– `user@host:port`ï¼ˆç«¯å£é»˜è®¤ä¸º `22`ï¼‰ã€‚
- `--ssh-identity <path>`ï¼šèº«ä»½æ–‡ä»¶ã€‚
- `--ssh-auto`ï¼šé€‰æ‹©ç¬¬ä¸€ä¸ªå‘çŽ°çš„ Gateway ç½‘å…³ä¸»æœºä½œä¸º SSH ç›®æ ‡ï¼ˆä»…é™å±€åŸŸç½‘/WABï¼‰ã€‚

é…ç½®ï¼ˆå¯é€‰ï¼Œç”¨ä½œé»˜è®¤å€¼ï¼‰ï¼š

- `gateway.remote.sshTarget`
- `gateway.remote.sshIdentity`

### `gateway call <method>`

ä½Žçº§ RPC è¾…åŠ©å·¥å…·ã€‚

```bash
krabkrab gateway call status
krabkrab gateway call logs.tail --params '{"sinceMs": 60000}'
```

## ç®¡ç† Gateway ç½‘å…³æœåŠ¡

```bash
krabkrab gateway install
krabkrab gateway start
krabkrab gateway stop
krabkrab gateway restart
krabkrab gateway uninstall
```

æ³¨æ„äº‹é¡¹ï¼š

- `gateway install` æ”¯æŒ `--port`ã€`--runtime`ã€`--token`ã€`--force`ã€`--json`ã€‚
- ç”Ÿå‘½å‘¨æœŸå‘½ä»¤æŽ¥å— `--json` ç”¨äºŽè„šæœ¬ã€‚

## å‘çŽ° Gateway ç½‘å…³ï¼ˆBonjourï¼‰

`gateway discover` æ‰«æ Gateway ç½‘å…³ä¿¡æ ‡ï¼ˆ`_krabkrab-gw._tcp`ï¼‰ã€‚

- ç»„æ’­ DNS-SDï¼š`local.`
- å•æ’­ DNS-SDï¼ˆå¹¿åŸŸ Bonjourï¼‰ï¼šé€‰æ‹©ä¸€ä¸ªåŸŸï¼ˆç¤ºä¾‹ï¼š`krabkrab.internal.`ï¼‰å¹¶è®¾ç½®åˆ†å‰² DNS + DNS æœåŠ¡å™¨ï¼›å‚è§ [/gateway/bonjour](/gateway/bonjour)

åªæœ‰å¯ç”¨äº† Bonjour å‘çŽ°ï¼ˆé»˜è®¤ï¼‰çš„ Gateway ç½‘å…³æ‰ä¼šå¹¿æ’­ä¿¡æ ‡ã€‚

å¹¿åŸŸå‘çŽ°è®°å½•åŒ…æ‹¬ï¼ˆTXTï¼‰ï¼š

- `role`ï¼ˆGateway ç½‘å…³è§’è‰²æç¤ºï¼‰
- `transport`ï¼ˆä¼ è¾“æç¤ºï¼Œä¾‹å¦‚ `gateway`ï¼‰
- `gatewayPort`ï¼ˆWebSocket ç«¯å£ï¼Œé€šå¸¸ä¸º `18789`ï¼‰
- `sshPort`ï¼ˆSSH ç«¯å£ï¼›å¦‚æžœä¸å­˜åœ¨åˆ™é»˜è®¤ä¸º `22`ï¼‰
- `tailnetDns`ï¼ˆMagicDNS ä¸»æœºåï¼Œå¦‚æžœå¯ç”¨ï¼‰
- `gatewayTls` / `gatewayTlsSha256`ï¼ˆTLS å¯ç”¨ + è¯ä¹¦æŒ‡çº¹ï¼‰
- `cliPath`ï¼ˆè¿œç¨‹å®‰è£…çš„å¯é€‰æç¤ºï¼‰

### `gateway discover`

```bash
krabkrab gateway discover
```

é€‰é¡¹ï¼š

- `--timeout <ms>`ï¼šæ¯ä¸ªå‘½ä»¤çš„è¶…æ—¶ï¼ˆæµè§ˆ/è§£æžï¼‰ï¼›é»˜è®¤ `2000`ã€‚
- `--json`ï¼šæœºå™¨å¯è¯»è¾“å‡ºï¼ˆåŒæ—¶ç¦ç”¨æ ·å¼/è¿›åº¦æŒ‡ç¤ºå™¨ï¼‰ã€‚

ç¤ºä¾‹ï¼š

```bash
krabkrab gateway discover --timeout 4000
krabkrab gateway discover --json | jq '.beacons[].wsUrl'
```

