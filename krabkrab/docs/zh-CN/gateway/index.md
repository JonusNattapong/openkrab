---
read_when:
  - è¿è¡Œæˆ–è°ƒè¯• Gateway ç½‘å…³è¿›ç¨‹æ—¶
summary: Gateway ç½‘å…³æœåŠ¡ã€ç”Ÿå‘½å‘¨æœŸå’Œè¿ç»´çš„è¿è¡Œæ‰‹å†Œ
title: Gateway ç½‘å…³è¿è¡Œæ‰‹å†Œ
x-i18n:
  generated_at: "2026-02-03T07:50:03Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 497d58090faaa6bdae62780ce887b40a1ad81e2e99ff186ea2a5c2249c35d9ba
  source_path: gateway/index.md
  workflow: 15
---

# Gateway ç½‘å…³æœåŠ¡è¿è¡Œæ‰‹å†Œ

æœ€åŽæ›´æ–°ï¼š2025-12-09

## æ˜¯ä»€ä¹ˆ

- æ‹¥æœ‰å•ä¸€ Baileys/Telegram è¿žæŽ¥å’ŒæŽ§åˆ¶/äº‹ä»¶å¹³é¢çš„å¸¸é©»è¿›ç¨‹ã€‚
- æ›¿ä»£æ—§ç‰ˆ `gateway` å‘½ä»¤ã€‚CLI å…¥å£ç‚¹ï¼š`krabkrab gateway`ã€‚
- è¿è¡Œç›´åˆ°åœæ­¢ï¼›å‡ºçŽ°è‡´å‘½é”™è¯¯æ—¶ä»¥éžé›¶é€€å‡ºç é€€å‡ºï¼Œä»¥ä¾¿ supervisor é‡å¯å®ƒã€‚

## å¦‚ä½•è¿è¡Œï¼ˆæœ¬åœ°ï¼‰

```bash
krabkrab gateway --port 18789
# åœ¨ stdio ä¸­èŽ·å–å®Œæ•´çš„è°ƒè¯•/è¿½è¸ªæ—¥å¿—ï¼š
krabkrab gateway --port 18789 --verbose
# å¦‚æžœç«¯å£è¢«å ç”¨ï¼Œç»ˆæ­¢ç›‘å¬å™¨ç„¶åŽå¯åŠ¨ï¼š
krabkrab gateway --force
# å¼€å‘å¾ªçŽ¯ï¼ˆTS æ›´æ”¹æ—¶è‡ªåŠ¨é‡è½½ï¼‰ï¼š
pnpm gateway:watch
```

- é…ç½®çƒ­é‡è½½ç›‘è§† `~/.krabkrab/krabkrab.json`ï¼ˆæˆ– `krabkrab_CONFIG_PATH`ï¼‰ã€‚
  - é»˜è®¤æ¨¡å¼ï¼š`gateway.reload.mode="hybrid"`ï¼ˆçƒ­åº”ç”¨å®‰å…¨æ›´æ”¹ï¼Œå…³é”®æ›´æ”¹æ—¶é‡å¯ï¼‰ã€‚
  - çƒ­é‡è½½åœ¨éœ€è¦æ—¶é€šè¿‡ **SIGUSR1** ä½¿ç”¨è¿›ç¨‹å†…é‡å¯ã€‚
  - ä½¿ç”¨ `gateway.reload.mode="off"` ç¦ç”¨ã€‚
- å°† WebSocket æŽ§åˆ¶å¹³é¢ç»‘å®šåˆ° `127.0.0.1:<port>`ï¼ˆé»˜è®¤ 18789ï¼‰ã€‚
- åŒä¸€ç«¯å£ä¹Ÿæä¾› HTTP æœåŠ¡ï¼ˆæŽ§åˆ¶ç•Œé¢ã€hooksã€A2UIï¼‰ã€‚å•ç«¯å£å¤šè·¯å¤ç”¨ã€‚
  - OpenAI Chat Completionsï¼ˆHTTPï¼‰ï¼š[`/v1/chat/completions`](/gateway/openai-http-api)ã€‚
  - OpenResponsesï¼ˆHTTPï¼‰ï¼š[`/v1/responses`](/gateway/openresponses-http-api)ã€‚
  - Tools Invokeï¼ˆHTTPï¼‰ï¼š[`/tools/invoke`](/gateway/tools-invoke-http-api)ã€‚
- é»˜è®¤åœ¨ `canvasHost.port`ï¼ˆé»˜è®¤ `18793`ï¼‰ä¸Šå¯åŠ¨ Canvas æ–‡ä»¶æœåŠ¡å™¨ï¼Œä»Ž `~/.krabkrab/workspace/canvas` æä¾› `http://<gateway-host>:18793/__krabkrab__/canvas/`ã€‚ä½¿ç”¨ `canvasHost.enabled=false` æˆ– `krabkrab_SKIP_CANVAS_HOST=1` ç¦ç”¨ã€‚
- è¾“å‡ºæ—¥å¿—åˆ° stdoutï¼›ä½¿ç”¨ launchd/systemd ä¿æŒè¿è¡Œå¹¶è½®è½¬æ—¥å¿—ã€‚
- æ•…éšœæŽ’é™¤æ—¶ä¼ é€’ `--verbose` ä»¥å°†è°ƒè¯•æ—¥å¿—ï¼ˆæ¡æ‰‹ã€è¯·æ±‚/å“åº”ã€äº‹ä»¶ï¼‰ä»Žæ—¥å¿—æ–‡ä»¶é•œåƒåˆ° stdioã€‚
- `--force` ä½¿ç”¨ `lsof` æŸ¥æ‰¾æ‰€é€‰ç«¯å£ä¸Šçš„ç›‘å¬å™¨ï¼Œå‘é€ SIGTERMï¼Œè®°å½•å®ƒç»ˆæ­¢äº†ä»€ä¹ˆï¼Œç„¶åŽå¯åŠ¨ Gateway ç½‘å…³ï¼ˆå¦‚æžœç¼ºå°‘ `lsof` åˆ™å¿«é€Ÿå¤±è´¥ï¼‰ã€‚
- å¦‚æžœä½ åœ¨ supervisorï¼ˆlaunchd/systemd/mac åº”ç”¨å­è¿›ç¨‹æ¨¡å¼ï¼‰ä¸‹è¿è¡Œï¼Œstop/restart é€šå¸¸å‘é€ **SIGTERM**ï¼›æ—§ç‰ˆæœ¬å¯èƒ½å°†å…¶æ˜¾ç¤ºä¸º `pnpm` `ELIFECYCLE` é€€å‡ºç  **143**ï¼ˆSIGTERMï¼‰ï¼Œè¿™æ˜¯æ­£å¸¸å…³é—­ï¼Œä¸æ˜¯å´©æºƒã€‚
- **SIGUSR1** åœ¨æŽˆæƒæ—¶è§¦å‘è¿›ç¨‹å†…é‡å¯ï¼ˆGateway ç½‘å…³å·¥å…·/é…ç½®åº”ç”¨/æ›´æ–°ï¼Œæˆ–å¯ç”¨ `commands.restart` ä»¥è¿›è¡Œæ‰‹åŠ¨é‡å¯ï¼‰ã€‚
- é»˜è®¤éœ€è¦ Gateway ç½‘å…³è®¤è¯ï¼šè®¾ç½® `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰æˆ– `gateway.auth.password`ã€‚å®¢æˆ·ç«¯å¿…é¡»å‘é€ `connect.params.auth.token/password`ï¼Œé™¤éžä½¿ç”¨ Tailscale Serve èº«ä»½ã€‚
- å‘å¯¼çŽ°åœ¨é»˜è®¤ç”Ÿæˆä»¤ç‰Œï¼Œå³ä½¿åœ¨ loopback ä¸Šä¹Ÿæ˜¯å¦‚æ­¤ã€‚
- ç«¯å£ä¼˜å…ˆçº§ï¼š`--port` > `krabkrab_GATEWAY_PORT` > `gateway.port` > é»˜è®¤ `18789`ã€‚

## è¿œç¨‹è®¿é—®

- é¦–é€‰ Tailscale/VPNï¼›å¦åˆ™ä½¿ç”¨ SSH éš§é“ï¼š
  ```bash
  ssh -N -L 18789:127.0.0.1:18789 user@host
  ```
- ç„¶åŽå®¢æˆ·ç«¯é€šè¿‡éš§é“è¿žæŽ¥åˆ° `ws://127.0.0.1:18789`ã€‚
- å¦‚æžœé…ç½®äº†ä»¤ç‰Œï¼Œå³ä½¿é€šè¿‡éš§é“ï¼Œå®¢æˆ·ç«¯ä¹Ÿå¿…é¡»åœ¨ `connect.params.auth.token` ä¸­åŒ…å«å®ƒã€‚

## å¤šä¸ª Gateway ç½‘å…³ï¼ˆåŒä¸€ä¸»æœºï¼‰

é€šå¸¸ä¸éœ€è¦ï¼šä¸€ä¸ª Gateway ç½‘å…³å¯ä»¥æœåŠ¡å¤šä¸ªæ¶ˆæ¯æ¸ é“å’Œæ™ºèƒ½ä½“ã€‚ä»…åœ¨éœ€è¦å†—ä½™æˆ–ä¸¥æ ¼éš”ç¦»ï¼ˆä¾‹å¦‚ï¼šæ•‘æ´æœºå™¨äººï¼‰æ—¶ä½¿ç”¨å¤šä¸ª Gateway ç½‘å…³ã€‚

å¦‚æžœä½ éš”ç¦»çŠ¶æ€ + é…ç½®å¹¶ä½¿ç”¨å”¯ä¸€ç«¯å£ï¼Œåˆ™æ”¯æŒã€‚å®Œæ•´æŒ‡å—ï¼š[å¤šä¸ª Gateway ç½‘å…³](/gateway/multiple-gateways)ã€‚

æœåŠ¡åç§°æ˜¯é…ç½®æ–‡ä»¶æ„ŸçŸ¥çš„ï¼š

- macOSï¼š`bot.molt.<profile>`ï¼ˆæ—§ç‰ˆ `com.krabkrab.*` å¯èƒ½ä»ç„¶å­˜åœ¨ï¼‰
- Linuxï¼š`krabkrab-gateway-<profile>.service`
- Windowsï¼š`KrabKrab Gateway (<profile>)`

å®‰è£…å…ƒæ•°æ®åµŒå…¥åœ¨æœåŠ¡é…ç½®ä¸­ï¼š

- `krabkrab_SERVICE_MARKER=krabkrab`
- `krabkrab_SERVICE_KIND=gateway`
- `krabkrab_SERVICE_VERSION=<version>`

æ•‘æ´æœºå™¨äººæ¨¡å¼ï¼šä¿æŒç¬¬äºŒä¸ª Gateway ç½‘å…³éš”ç¦»ï¼Œä½¿ç”¨è‡ªå·±çš„é…ç½®æ–‡ä»¶ã€çŠ¶æ€ç›®å½•ã€å·¥ä½œåŒºå’ŒåŸºç¡€ç«¯å£é—´éš”ã€‚å®Œæ•´æŒ‡å—ï¼š[æ•‘æ´æœºå™¨äººæŒ‡å—](/gateway/multiple-gateways#rescue-bot-guide)ã€‚

### Dev é…ç½®æ–‡ä»¶ï¼ˆ`--dev`ï¼‰

å¿«é€Ÿè·¯å¾„ï¼šè¿è¡Œå®Œå…¨éš”ç¦»çš„ dev å®žä¾‹ï¼ˆé…ç½®/çŠ¶æ€/å·¥ä½œåŒºï¼‰è€Œä¸è§¦åŠä½ çš„ä¸»è®¾ç½®ã€‚

```bash
krabkrab --dev setup
krabkrab --dev gateway --allow-unconfigured
# ç„¶åŽå®šä½åˆ° dev å®žä¾‹ï¼š
krabkrab --dev status
krabkrab --dev health
```

é»˜è®¤å€¼ï¼ˆå¯é€šè¿‡ env/flags/config è¦†ç›–ï¼‰ï¼š

- `krabkrab_STATE_DIR=~/.krabkrab-dev`
- `krabkrab_CONFIG_PATH=~/.krabkrab-dev/krabkrab.json`
- `krabkrab_GATEWAY_PORT=19001`ï¼ˆGateway ç½‘å…³ WS + HTTPï¼‰
- æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡ç«¯å£ = `19003`ï¼ˆæ´¾ç”Ÿï¼š`gateway.port+2`ï¼Œä»… loopbackï¼‰
- `canvasHost.port=19005`ï¼ˆæ´¾ç”Ÿï¼š`gateway.port+4`ï¼‰
- å½“ä½ åœ¨ `--dev` ä¸‹è¿è¡Œ `setup`/`onboard` æ—¶ï¼Œ`agents.defaults.workspace` é»˜è®¤å˜ä¸º `~/.krabkrab/workspace-dev`ã€‚

æ´¾ç”Ÿç«¯å£ï¼ˆç»éªŒæ³•åˆ™ï¼‰ï¼š

- åŸºç¡€ç«¯å£ = `gateway.port`ï¼ˆæˆ– `krabkrab_GATEWAY_PORT` / `--port`ï¼‰
- æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡ç«¯å£ = åŸºç¡€ + 2ï¼ˆä»… loopbackï¼‰
- `canvasHost.port = åŸºç¡€ + 4`ï¼ˆæˆ– `krabkrab_CANVAS_HOST_PORT` / é…ç½®è¦†ç›–ï¼‰
- æµè§ˆå™¨é…ç½®æ–‡ä»¶ CDP ç«¯å£ä»Ž `browser.controlPort + 9 .. + 108` è‡ªåŠ¨åˆ†é…ï¼ˆæŒ‰é…ç½®æ–‡ä»¶æŒä¹…åŒ–ï¼‰ã€‚

æ¯ä¸ªå®žä¾‹çš„æ£€æŸ¥æ¸…å•ï¼š

- å”¯ä¸€çš„ `gateway.port`
- å”¯ä¸€çš„ `krabkrab_CONFIG_PATH`
- å”¯ä¸€çš„ `krabkrab_STATE_DIR`
- å”¯ä¸€çš„ `agents.defaults.workspace`
- å•ç‹¬çš„ WhatsApp å·ç ï¼ˆå¦‚æžœä½¿ç”¨ WAï¼‰

æŒ‰é…ç½®æ–‡ä»¶å®‰è£…æœåŠ¡ï¼š

```bash
krabkrab --profile main gateway install
krabkrab --profile rescue gateway install
```

ç¤ºä¾‹ï¼š

```bash
krabkrab_CONFIG_PATH=~/.krabkrab/a.json krabkrab_STATE_DIR=~/.krabkrab-a krabkrab gateway --port 19001
krabkrab_CONFIG_PATH=~/.krabkrab/b.json krabkrab_STATE_DIR=~/.krabkrab-b krabkrab gateway --port 19002
```

## åè®®ï¼ˆè¿ç»´è§†è§’ï¼‰

- å®Œæ•´æ–‡æ¡£ï¼š[Gateway ç½‘å…³åè®®](/gateway/protocol) å’Œ [Bridge åè®®ï¼ˆæ—§ç‰ˆï¼‰](/gateway/bridge-protocol)ã€‚
- å®¢æˆ·ç«¯å¿…é¡»å‘é€çš„ç¬¬ä¸€å¸§ï¼š`req {type:"req", id, method:"connect", params:{minProtocol,maxProtocol,client:{id,displayName?,version,platform,deviceFamily?,modelIdentifier?,mode,instanceId?}, caps, auth?, locale?, userAgent? } }`ã€‚
- Gateway ç½‘å…³å›žå¤ `res {type:"res", id, ok:true, payload:hello-ok }`ï¼ˆæˆ– `ok:false` å¸¦é”™è¯¯ï¼Œç„¶åŽå…³é—­ï¼‰ã€‚
- æ¡æ‰‹åŽï¼š
  - è¯·æ±‚ï¼š`{type:"req", id, method, params}` â†’ `{type:"res", id, ok, payload|error}`
  - äº‹ä»¶ï¼š`{type:"event", event, payload, seq?, stateVersion?}`
- ç»“æž„åŒ– presence æ¡ç›®ï¼š`{host, ip, version, platform?, deviceFamily?, modelIdentifier?, mode, lastInputSeconds?, ts, reason?, tags?[], instanceId? }`ï¼ˆå¯¹äºŽ WS å®¢æˆ·ç«¯ï¼Œ`instanceId` æ¥è‡ª `connect.client.instanceId`ï¼‰ã€‚
- `agent` å“åº”æ˜¯ä¸¤é˜¶æ®µçš„ï¼šé¦–å…ˆ `res` ç¡®è®¤ `{runId,status:"accepted"}`ï¼Œç„¶åŽåœ¨è¿è¡Œå®ŒæˆåŽå‘é€æœ€ç»ˆ `res` `{runId,status:"ok"|"error",summary}`ï¼›æµå¼è¾“å‡ºä½œä¸º `event:"agent"` åˆ°è¾¾ã€‚

## æ–¹æ³•ï¼ˆåˆå§‹é›†ï¼‰

- `health` â€” å®Œæ•´å¥åº·å¿«ç…§ï¼ˆä¸Ž `krabkrab health --json` å½¢çŠ¶ç›¸åŒï¼‰ã€‚
- `status` â€” ç®€çŸ­æ‘˜è¦ã€‚
- `system-presence` â€” å½“å‰ presence åˆ—è¡¨ã€‚
- `system-event` â€” å‘å¸ƒ presence/ç³»ç»Ÿæ³¨é‡Šï¼ˆç»“æž„åŒ–ï¼‰ã€‚
- `send` â€” é€šè¿‡æ´»è·ƒæ¸ é“å‘é€æ¶ˆæ¯ã€‚
- `agent` â€” è¿è¡Œæ™ºèƒ½ä½“è½®æ¬¡ï¼ˆåœ¨åŒä¸€è¿žæŽ¥ä¸Šæµå›žäº‹ä»¶ï¼‰ã€‚
- `node.list` â€” åˆ—å‡ºå·²é…å¯¹ + å½“å‰è¿žæŽ¥çš„èŠ‚ç‚¹ï¼ˆåŒ…æ‹¬ `caps`ã€`deviceFamily`ã€`modelIdentifier`ã€`paired`ã€`connected` å’Œå¹¿æ’­çš„ `commands`ï¼‰ã€‚
- `node.describe` â€” æè¿°èŠ‚ç‚¹ï¼ˆèƒ½åŠ› + æ”¯æŒçš„ `node.invoke` å‘½ä»¤ï¼›é€‚ç”¨äºŽå·²é…å¯¹èŠ‚ç‚¹å’Œå½“å‰è¿žæŽ¥çš„æœªé…å¯¹èŠ‚ç‚¹ï¼‰ã€‚
- `node.invoke` â€” åœ¨èŠ‚ç‚¹ä¸Šè°ƒç”¨å‘½ä»¤ï¼ˆä¾‹å¦‚ `canvas.*`ã€`camera.*`ï¼‰ã€‚
- `node.pair.*` â€” é…å¯¹ç”Ÿå‘½å‘¨æœŸï¼ˆ`request`ã€`list`ã€`approve`ã€`reject`ã€`verify`ï¼‰ã€‚

å¦è§ï¼š[Presence](/concepts/presence) äº†è§£ presence å¦‚ä½•äº§ç”Ÿ/åŽ»é‡ä»¥åŠä¸ºä»€ä¹ˆç¨³å®šçš„ `client.instanceId` å¾ˆé‡è¦ã€‚

## äº‹ä»¶

- `agent` â€” æ¥è‡ªæ™ºèƒ½ä½“è¿è¡Œçš„æµå¼å·¥å…·/è¾“å‡ºäº‹ä»¶ï¼ˆå¸¦ seq æ ‡è®°ï¼‰ã€‚
- `presence` â€” presence æ›´æ–°ï¼ˆå¸¦ stateVersion çš„å¢žé‡ï¼‰æŽ¨é€åˆ°æ‰€æœ‰è¿žæŽ¥çš„å®¢æˆ·ç«¯ã€‚
- `tick` â€” å®šæœŸä¿æ´»/æ— æ“ä½œä»¥ç¡®è®¤æ´»è·ƒã€‚
- `shutdown` â€” Gateway ç½‘å…³æ­£åœ¨é€€å‡ºï¼›payload åŒ…æ‹¬ `reason` å’Œå¯é€‰çš„ `restartExpectedMs`ã€‚å®¢æˆ·ç«¯åº”é‡æ–°è¿žæŽ¥ã€‚

## WebChat é›†æˆ

- WebChat æ˜¯åŽŸç”Ÿ SwiftUI UIï¼Œç›´æŽ¥ä¸Ž Gateway ç½‘å…³ WebSocket é€šä¿¡ä»¥èŽ·å–åŽ†å²è®°å½•ã€å‘é€ã€ä¸­æ­¢å’Œäº‹ä»¶ã€‚
- è¿œç¨‹ä½¿ç”¨é€šè¿‡ç›¸åŒçš„ SSH/Tailscale éš§é“ï¼›å¦‚æžœé…ç½®äº† Gateway ç½‘å…³ä»¤ç‰Œï¼Œå®¢æˆ·ç«¯åœ¨ `connect` æœŸé—´åŒ…å«å®ƒã€‚
- macOS åº”ç”¨é€šè¿‡å•ä¸ª WS è¿žæŽ¥ï¼ˆå…±äº«è¿žæŽ¥ï¼‰ï¼›å®ƒä»Žåˆå§‹å¿«ç…§å¡«å…… presence å¹¶ç›‘å¬ `presence` äº‹ä»¶ä»¥æ›´æ–° UIã€‚

## ç±»åž‹å’ŒéªŒè¯

- æœåŠ¡å™¨ä½¿ç”¨ AJV æ ¹æ®ä»Žåè®®å®šä¹‰å‘å‡ºçš„ JSON Schema éªŒè¯æ¯ä¸ªå…¥ç«™å¸§ã€‚
- å®¢æˆ·ç«¯ï¼ˆTS/Swiftï¼‰æ¶ˆè´¹ç”Ÿæˆçš„ç±»åž‹ï¼ˆTS ç›´æŽ¥ä½¿ç”¨ï¼›Swift é€šè¿‡ä»“åº“çš„ç”Ÿæˆå™¨ï¼‰ã€‚
- åè®®å®šä¹‰æ˜¯çœŸå®žæ¥æºï¼›ä½¿ç”¨ä»¥ä¸‹å‘½ä»¤é‡æ–°ç”Ÿæˆ schema/æ¨¡åž‹ï¼š
  - `pnpm protocol:gen`
  - `pnpm protocol:gen:swift`

## è¿žæŽ¥å¿«ç…§

- `hello-ok` åŒ…å«å¸¦æœ‰ `presence`ã€`health`ã€`stateVersion` å’Œ `uptimeMs` çš„ `snapshot`ï¼Œä»¥åŠ `policy {maxPayload,maxBufferedBytes,tickIntervalMs}`ï¼Œè¿™æ ·å®¢æˆ·ç«¯æ— éœ€é¢å¤–è¯·æ±‚å³å¯ç«‹å³æ¸²æŸ“ã€‚
- `health`/`system-presence` ä»å¯ç”¨äºŽæ‰‹åŠ¨åˆ·æ–°ï¼Œä½†åœ¨è¿žæŽ¥æ—¶ä¸æ˜¯å¿…éœ€çš„ã€‚

## é”™è¯¯ç ï¼ˆres.error å½¢çŠ¶ï¼‰

- é”™è¯¯ä½¿ç”¨ `{ code, message, details?, retryable?, retryAfterMs? }`ã€‚
- æ ‡å‡†ç ï¼š
  - `NOT_LINKED` â€” WhatsApp æœªè®¤è¯ã€‚
  - `AGENT_TIMEOUT` â€” æ™ºèƒ½ä½“æœªåœ¨é…ç½®çš„æˆªæ­¢æ—¶é—´å†…å“åº”ã€‚
  - `INVALID_REQUEST` â€” schema/å‚æ•°éªŒè¯å¤±è´¥ã€‚
  - `UNAVAILABLE` â€” Gateway ç½‘å…³æ­£åœ¨å…³é—­æˆ–ä¾èµ–é¡¹ä¸å¯ç”¨ã€‚

## ä¿æ´»è¡Œä¸º

- `tick` äº‹ä»¶ï¼ˆæˆ– WS ping/pongï¼‰å®šæœŸå‘å‡ºï¼Œä»¥ä¾¿å®¢æˆ·ç«¯çŸ¥é“å³ä½¿æ²¡æœ‰æµé‡æ—¶ Gateway ç½‘å…³ä¹Ÿæ˜¯æ´»è·ƒçš„ã€‚
- å‘é€/æ™ºèƒ½ä½“ç¡®è®¤ä¿æŒä¸ºå•ç‹¬çš„å“åº”ï¼›ä¸è¦ä¸ºå‘é€é‡è½½ tickã€‚

## é‡æ”¾ / é—´éš™

- äº‹ä»¶ä¸ä¼šé‡æ”¾ã€‚å®¢æˆ·ç«¯æ£€æµ‹ seq é—´éš™ï¼Œåº”åœ¨ç»§ç»­ä¹‹å‰åˆ·æ–°ï¼ˆ`health` + `system-presence`ï¼‰ã€‚WebChat å’Œ macOS å®¢æˆ·ç«¯çŽ°åœ¨ä¼šåœ¨é—´éš™æ—¶è‡ªåŠ¨åˆ·æ–°ã€‚

## ç›‘ç®¡ï¼ˆmacOS ç¤ºä¾‹ï¼‰

- ä½¿ç”¨ launchd ä¿æŒæœåŠ¡å­˜æ´»ï¼š
  - Programï¼š`krabkrab` çš„è·¯å¾„
  - Argumentsï¼š`gateway`
  - KeepAliveï¼štrue
  - StandardOut/Errï¼šæ–‡ä»¶è·¯å¾„æˆ– `syslog`
- å¤±è´¥æ—¶ï¼Œlaunchd é‡å¯ï¼›è‡´å‘½çš„é…ç½®é”™è¯¯åº”ä¿æŒé€€å‡ºï¼Œä»¥ä¾¿è¿ç»´äººå‘˜æ³¨æ„åˆ°ã€‚
- LaunchAgents æ˜¯æŒ‰ç”¨æˆ·çš„ï¼Œéœ€è¦å·²ç™»å½•çš„ä¼šè¯ï¼›å¯¹äºŽæ— å¤´è®¾ç½®ï¼Œä½¿ç”¨è‡ªå®šä¹‰ LaunchDaemonï¼ˆæœªéšé™„ï¼‰ã€‚
  - `krabkrab gateway install` å†™å…¥ `~/Library/LaunchAgents/bot.molt.gateway.plist`
    ï¼ˆæˆ– `bot.molt.<profile>.plist`ï¼›æ—§ç‰ˆ `com.krabkrab.*` ä¼šè¢«æ¸…ç†ï¼‰ã€‚
  - `krabkrab doctor` å®¡è®¡ LaunchAgent é…ç½®ï¼Œå¯ä»¥å°†å…¶æ›´æ–°ä¸ºå½“å‰é»˜è®¤å€¼ã€‚

## Gateway ç½‘å…³æœåŠ¡ç®¡ç†ï¼ˆCLIï¼‰

ä½¿ç”¨ Gateway ç½‘å…³ CLI è¿›è¡Œ install/start/stop/restart/statusï¼š

```bash
krabkrab gateway status
krabkrab gateway install
krabkrab gateway stop
krabkrab gateway restart
krabkrab logs --follow
```

æ³¨æ„äº‹é¡¹ï¼š

- `gateway status` é»˜è®¤ä½¿ç”¨æœåŠ¡è§£æžçš„ç«¯å£/é…ç½®æŽ¢æµ‹ Gateway ç½‘å…³ RPCï¼ˆä½¿ç”¨ `--url` è¦†ç›–ï¼‰ã€‚
- `gateway status --deep` æ·»åŠ ç³»ç»Ÿçº§æ‰«æï¼ˆLaunchDaemons/ç³»ç»Ÿå•å…ƒï¼‰ã€‚
- `gateway status --no-probe` è·³è¿‡ RPC æŽ¢æµ‹ï¼ˆåœ¨ç½‘ç»œæ•…éšœæ—¶æœ‰ç”¨ï¼‰ã€‚
- `gateway status --json` å¯¹è„šæœ¬æ˜¯ç¨³å®šçš„ã€‚
- `gateway status` å°† **supervisor è¿è¡Œæ—¶**ï¼ˆlaunchd/systemd è¿è¡Œä¸­ï¼‰ä¸Ž **RPC å¯è¾¾æ€§**ï¼ˆWS è¿žæŽ¥ + status RPCï¼‰åˆ†å¼€æŠ¥å‘Šã€‚
- `gateway status` æ‰“å°é…ç½®è·¯å¾„ + æŽ¢æµ‹ç›®æ ‡ä»¥é¿å…"localhost vs LAN ç»‘å®š"æ··æ·†å’Œé…ç½®æ–‡ä»¶ä¸åŒ¹é…ã€‚
- `gateway status` åœ¨æœåŠ¡çœ‹èµ·æ¥æ­£åœ¨è¿è¡Œä½†ç«¯å£å·²å…³é—­æ—¶åŒ…å«æœ€åŽä¸€è¡Œ Gateway ç½‘å…³é”™è¯¯ã€‚
- `logs` é€šè¿‡ RPC å°¾éš Gateway ç½‘å…³æ–‡ä»¶æ—¥å¿—ï¼ˆæ— éœ€æ‰‹åŠ¨ `tail`/`grep`ï¼‰ã€‚
- å¦‚æžœæ£€æµ‹åˆ°å…¶ä»–ç±»ä¼¼ Gateway ç½‘å…³çš„æœåŠ¡ï¼ŒCLI ä¼šå‘å‡ºè­¦å‘Šï¼Œé™¤éžå®ƒä»¬æ˜¯ KrabKrab é…ç½®æ–‡ä»¶æœåŠ¡ã€‚
  æˆ‘ä»¬ä»ç„¶å»ºè®®å¤§å¤šæ•°è®¾ç½®**æ¯å°æœºå™¨ä¸€ä¸ª Gateway ç½‘å…³**ï¼›ä½¿ç”¨éš”ç¦»çš„é…ç½®æ–‡ä»¶/ç«¯å£è¿›è¡Œå†—ä½™æˆ–æ•‘æ´æœºå™¨äººã€‚å‚è§[å¤šä¸ª Gateway ç½‘å…³](/gateway/multiple-gateways)ã€‚
  - æ¸…ç†ï¼š`krabkrab gateway uninstall`ï¼ˆå½“å‰æœåŠ¡ï¼‰å’Œ `krabkrab doctor`ï¼ˆæ—§ç‰ˆè¿ç§»ï¼‰ã€‚
- `gateway install` åœ¨å·²å®‰è£…æ—¶æ˜¯æ— æ“ä½œçš„ï¼›ä½¿ç”¨ `krabkrab gateway install --force` é‡æ–°å®‰è£…ï¼ˆé…ç½®æ–‡ä»¶/env/è·¯å¾„æ›´æ”¹ï¼‰ã€‚

æ†ç»‘çš„ mac åº”ç”¨ï¼š

- KrabKrab.app å¯ä»¥æ†ç»‘åŸºäºŽ Node çš„ Gateway ç½‘å…³ä¸­ç»§å¹¶å®‰è£…æ ‡è®°ä¸º
  `bot.molt.gateway`ï¼ˆæˆ– `bot.molt.<profile>`ï¼›æ—§ç‰ˆ `com.krabkrab.*` æ ‡ç­¾ä»èƒ½å¹²å‡€å¸è½½ï¼‰çš„æŒ‰ç”¨æˆ· LaunchAgentã€‚
- è¦å¹²å‡€åœ°åœæ­¢å®ƒï¼Œä½¿ç”¨ `krabkrab gateway stop`ï¼ˆæˆ– `launchctl bootout gui/$UID/bot.molt.gateway`ï¼‰ã€‚
- è¦é‡å¯ï¼Œä½¿ç”¨ `krabkrab gateway restart`ï¼ˆæˆ– `launchctl kickstart -k gui/$UID/bot.molt.gateway`ï¼‰ã€‚
  - `launchctl` ä»…åœ¨ LaunchAgent å·²å®‰è£…æ—¶æœ‰æ•ˆï¼›å¦åˆ™å…ˆä½¿ç”¨ `krabkrab gateway install`ã€‚
  - è¿è¡Œå‘½åé…ç½®æ–‡ä»¶æ—¶ï¼Œå°†æ ‡ç­¾æ›¿æ¢ä¸º `bot.molt.<profile>`ã€‚

## ç›‘ç®¡ï¼ˆsystemd ç”¨æˆ·å•å…ƒï¼‰

KrabKrab åœ¨ Linux/WSL2 ä¸Šé»˜è®¤å®‰è£… **systemd ç”¨æˆ·æœåŠ¡**ã€‚æˆ‘ä»¬
å»ºè®®å•ç”¨æˆ·æœºå™¨ä½¿ç”¨ç”¨æˆ·æœåŠ¡ï¼ˆæ›´ç®€å•çš„ envï¼ŒæŒ‰ç”¨æˆ·é…ç½®ï¼‰ã€‚
å¯¹äºŽå¤šç”¨æˆ·æˆ–å¸¸é©»æœåŠ¡å™¨ä½¿ç”¨**ç³»ç»ŸæœåŠ¡**ï¼ˆæ— éœ€ lingeringï¼Œ
å…±äº«ç›‘ç®¡ï¼‰ã€‚

`krabkrab gateway install` å†™å…¥ç”¨æˆ·å•å…ƒã€‚`krabkrab doctor` å®¡è®¡
å•å…ƒå¹¶å¯ä»¥å°†å…¶æ›´æ–°ä»¥åŒ¹é…å½“å‰æŽ¨èçš„é»˜è®¤å€¼ã€‚

åˆ›å»º `~/.config/systemd/user/krabkrab-gateway[-<profile>].service`ï¼š

```
[Unit]
Description=KrabKrab Gateway (profile: <profile>, v<version>)
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=/usr/local/bin/krabkrab gateway --port 18789
Restart=always
RestartSec=5
Environment=krabkrab_GATEWAY_TOKEN=
WorkingDirectory=/home/youruser

[Install]
WantedBy=default.target
```

å¯ç”¨ lingeringï¼ˆå¿…éœ€ï¼Œä»¥ä¾¿ç”¨æˆ·æœåŠ¡åœ¨ç™»å‡º/ç©ºé—²åŽç»§ç»­å­˜æ´»ï¼‰ï¼š

```
sudo loginctl enable-linger youruser
```

æ–°æ‰‹å¼•å¯¼åœ¨ Linux/WSL2 ä¸Šè¿è¡Œæ­¤å‘½ä»¤ï¼ˆå¯èƒ½æç¤ºè¾“å…¥ sudoï¼›å†™å…¥ `/var/lib/systemd/linger`ï¼‰ã€‚
ç„¶åŽå¯ç”¨æœåŠ¡ï¼š

```
systemctl --user enable --now krabkrab-gateway[-<profile>].service
```

**æ›¿ä»£æ–¹æ¡ˆï¼ˆç³»ç»ŸæœåŠ¡ï¼‰** - å¯¹äºŽå¸¸é©»æˆ–å¤šç”¨æˆ·æœåŠ¡å™¨ï¼Œä½ å¯ä»¥
å®‰è£… systemd **ç³»ç»Ÿ**å•å…ƒè€Œä¸æ˜¯ç”¨æˆ·å•å…ƒï¼ˆæ— éœ€ lingeringï¼‰ã€‚
åˆ›å»º `/etc/systemd/system/krabkrab-gateway[-<profile>].service`ï¼ˆå¤åˆ¶ä¸Šé¢çš„å•å…ƒï¼Œ
åˆ‡æ¢ `WantedBy=multi-user.target`ï¼Œè®¾ç½® `User=` + `WorkingDirectory=`ï¼‰ï¼Œç„¶åŽï¼š

```
sudo systemctl daemon-reload
sudo systemctl enable --now krabkrab-gateway[-<profile>].service
```

## Windowsï¼ˆWSL2ï¼‰

Windows å®‰è£…åº”ä½¿ç”¨ **WSL2** å¹¶éµå¾ªä¸Šé¢çš„ Linux systemd éƒ¨åˆ†ã€‚

## è¿ç»´æ£€æŸ¥

- å­˜æ´»æ£€æŸ¥ï¼šæ‰“å¼€ WS å¹¶å‘é€ `req:connect` â†’ æœŸæœ›æ”¶åˆ°å¸¦æœ‰ `payload.type="hello-ok"`ï¼ˆå¸¦å¿«ç…§ï¼‰çš„ `res`ã€‚
- å°±ç»ªæ£€æŸ¥ï¼šè°ƒç”¨ `health` â†’ æœŸæœ› `ok: true` å¹¶åœ¨ `linkChannel` ä¸­æœ‰å·²å…³è”çš„æ¸ é“ï¼ˆé€‚ç”¨æ—¶ï¼‰ã€‚
- è°ƒè¯•ï¼šè®¢é˜… `tick` å’Œ `presence` äº‹ä»¶ï¼›ç¡®ä¿ `status` æ˜¾ç¤ºå·²å…³è”/è®¤è¯æ—¶é—´ï¼›presence æ¡ç›®æ˜¾ç¤º Gateway ç½‘å…³ä¸»æœºå’Œå·²è¿žæŽ¥çš„å®¢æˆ·ç«¯ã€‚

## å®‰å…¨ä¿è¯

- é»˜è®¤å‡è®¾æ¯å°ä¸»æœºä¸€ä¸ª Gateway ç½‘å…³ï¼›å¦‚æžœä½ è¿è¡Œå¤šä¸ªé…ç½®æ–‡ä»¶ï¼Œéš”ç¦»ç«¯å£/çŠ¶æ€å¹¶å®šä½åˆ°æ­£ç¡®çš„å®žä¾‹ã€‚
- ä¸ä¼šå›žé€€åˆ°ç›´æŽ¥ Baileys è¿žæŽ¥ï¼›å¦‚æžœ Gateway ç½‘å…³å…³é—­ï¼Œå‘é€ä¼šå¿«é€Ÿå¤±è´¥ã€‚
- éž connect çš„ç¬¬ä¸€å¸§æˆ–æ ¼å¼é”™è¯¯çš„ JSON ä¼šè¢«æ‹’ç»å¹¶å…³é—­ socketã€‚
- ä¼˜é›…å…³é—­ï¼šå…³é—­å‰å‘å‡º `shutdown` äº‹ä»¶ï¼›å®¢æˆ·ç«¯å¿…é¡»å¤„ç†å…³é—­ + é‡æ–°è¿žæŽ¥ã€‚

## CLI è¾…åŠ©å·¥å…·

- `krabkrab gateway health|status` â€” é€šè¿‡ Gateway ç½‘å…³ WS è¯·æ±‚ health/statusã€‚
- `krabkrab message send --target <num> --message "hi" [--media ...]` â€” é€šè¿‡ Gateway ç½‘å…³å‘é€ï¼ˆå¯¹ WhatsApp æ˜¯å¹‚ç­‰çš„ï¼‰ã€‚
- `krabkrab agent --message "hi" --to <num>` â€” è¿è¡Œæ™ºèƒ½ä½“è½®æ¬¡ï¼ˆé»˜è®¤ç­‰å¾…æœ€ç»ˆç»“æžœï¼‰ã€‚
- `krabkrab gateway call <method> --params '{"k":"v"}'` â€” ç”¨äºŽè°ƒè¯•çš„åŽŸå§‹æ–¹æ³•è°ƒç”¨å™¨ã€‚
- `krabkrab gateway stop|restart` â€” åœæ­¢/é‡å¯å—ç›‘ç®¡çš„ Gateway ç½‘å…³æœåŠ¡ï¼ˆlaunchd/systemdï¼‰ã€‚
- Gateway ç½‘å…³è¾…åŠ©å­å‘½ä»¤å‡è®¾ `--url` ä¸Šæœ‰è¿è¡Œä¸­çš„ Gateway ç½‘å…³ï¼›å®ƒä»¬ä¸å†è‡ªåŠ¨ç”Ÿæˆä¸€ä¸ªã€‚

## è¿ç§»æŒ‡å—

- æ·˜æ±° `krabkrab gateway` å’Œæ—§ç‰ˆ TCP æŽ§åˆ¶ç«¯å£çš„ä½¿ç”¨ã€‚
- æ›´æ–°å®¢æˆ·ç«¯ä»¥ä½¿ç”¨å¸¦æœ‰å¼ºåˆ¶ connect å’Œç»“æž„åŒ– presence çš„ WS åè®®ã€‚

