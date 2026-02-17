---
read_when:
  - è°ƒæŸ¥è¿è¡Œæ—¶é—®é¢˜æˆ–æ•…éšœ
summary: KrabKrab å¸¸è§æ•…éšœçš„å¿«é€Ÿæ•…éšœæŽ’é™¤æŒ‡å—
title: æ•…éšœæŽ’é™¤
x-i18n:
  generated_at: "2026-02-03T10:09:42Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: a07bb06f0b5ef56872578aaff6ac83adb740e2f1d23e3eed86934b51f62a877e
  source_path: gateway/troubleshooting.md
  workflow: 15
---

# æ•…éšœæŽ’é™¤ ðŸ”§

å½“ KrabKrab å‡ºçŽ°å¼‚å¸¸æ—¶ï¼Œä»¥ä¸‹æ˜¯è§£å†³æ–¹æ³•ã€‚

å¦‚æžœä½ åªæƒ³å¿«é€Ÿåˆ†ç±»é—®é¢˜ï¼Œè¯·å…ˆæŸ¥çœ‹å¸¸è§é—®é¢˜çš„[æœ€åˆçš„å…­åç§’](/help/faq#first-60-seconds-if-somethings-broken)ã€‚æœ¬é¡µæ·±å…¥ä»‹ç»è¿è¡Œæ—¶æ•…éšœå’Œè¯Šæ–­ã€‚

ç‰¹å®šæä¾›å•†çš„å¿«æ·æ–¹å¼ï¼š[/channels/troubleshooting](/channels/troubleshooting)

## çŠ¶æ€ä¸Žè¯Šæ–­

å¿«é€Ÿåˆ†ç±»å‘½ä»¤ï¼ˆæŒ‰é¡ºåºï¼‰ï¼š

| å‘½ä»¤                               | å®ƒå‘Šè¯‰ä½ ä»€ä¹ˆ                                                                          | ä½•æ—¶ä½¿ç”¨                              |
| ---------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------- |
| `krabkrab status`                  | æœ¬åœ°æ‘˜è¦ï¼šæ“ä½œç³»ç»Ÿ + æ›´æ–°ã€Gateway ç½‘å…³å¯è¾¾æ€§/æ¨¡å¼ã€æœåŠ¡ã€æ™ºèƒ½ä½“/ä¼šè¯ã€æä¾›å•†é…ç½®çŠ¶æ€ | é¦–æ¬¡æ£€æŸ¥ï¼Œå¿«é€Ÿæ¦‚è§ˆ                    |
| `krabkrab status --all`            | å®Œæ•´æœ¬åœ°è¯Šæ–­ï¼ˆåªè¯»ã€å¯ç²˜è´´ã€ç›¸å¯¹å®‰å…¨ï¼‰åŒ…æ‹¬æ—¥å¿—å°¾éƒ¨                                    | å½“ä½ éœ€è¦åˆ†äº«è°ƒè¯•æŠ¥å‘Šæ—¶                |
| `krabkrab status --deep`           | è¿è¡Œ Gateway ç½‘å…³å¥åº·æ£€æŸ¥ï¼ˆåŒ…æ‹¬æä¾›å•†æŽ¢æµ‹ï¼›éœ€è¦å¯è¾¾çš„ Gateway ç½‘å…³ï¼‰                  | å½“"å·²é…ç½®"ä¸æ„å‘³ç€"æ­£å¸¸å·¥ä½œ"æ—¶        |
| `krabkrab gateway probe`           | Gateway ç½‘å…³å‘çŽ° + å¯è¾¾æ€§ï¼ˆæœ¬åœ° + è¿œç¨‹ç›®æ ‡ï¼‰                                          | å½“ä½ æ€€ç–‘æ­£åœ¨æŽ¢æµ‹é”™è¯¯çš„ Gateway ç½‘å…³æ—¶ |
| `krabkrab channels status --probe` | å‘è¿è¡Œä¸­çš„ Gateway ç½‘å…³æŸ¥è¯¢æ¸ é“çŠ¶æ€ï¼ˆå¹¶å¯é€‰æŽ¢æµ‹ï¼‰                                     | å½“ Gateway ç½‘å…³å¯è¾¾ä½†æ¸ é“å¼‚å¸¸æ—¶       |
| `krabkrab gateway status`          | ç›‘ç®¡ç¨‹åºçŠ¶æ€ï¼ˆlaunchd/systemd/schtasksï¼‰ã€è¿è¡Œæ—¶ PID/é€€å‡ºã€æœ€åŽçš„ Gateway ç½‘å…³é”™è¯¯    | å½“æœåŠ¡"çœ‹èµ·æ¥å·²åŠ è½½"ä½†æ²¡æœ‰è¿è¡Œæ—¶      |
| `krabkrab logs --follow`           | å®žæ—¶æ—¥å¿—ï¼ˆè¿è¡Œæ—¶é—®é¢˜çš„æœ€ä½³ä¿¡å·ï¼‰                                                      | å½“ä½ éœ€è¦å®žé™…çš„æ•…éšœåŽŸå› æ—¶              |

**åˆ†äº«è¾“å‡ºï¼š** ä¼˜å…ˆä½¿ç”¨ `krabkrab status --all`ï¼ˆå®ƒä¼šéšè—ä»¤ç‰Œï¼‰ã€‚å¦‚æžœä½ ç²˜è´´ `krabkrab status`ï¼Œè€ƒè™‘å…ˆè®¾ç½® `krabkrab_SHOW_SECRETS=0`ï¼ˆä»¤ç‰Œé¢„è§ˆï¼‰ã€‚

å¦è¯·å‚é˜…ï¼š[å¥åº·æ£€æŸ¥](/gateway/health) å’Œ [æ—¥å¿—](/logging)ã€‚

## å¸¸è§é—®é¢˜

### No API key found for provider "anthropic"

è¿™æ„å‘³ç€**æ™ºèƒ½ä½“çš„è®¤è¯å­˜å‚¨ä¸ºç©º**æˆ–ç¼ºå°‘ Anthropic å‡­è¯ã€‚
è®¤è¯æ˜¯**æ¯ä¸ªæ™ºèƒ½ä½“ç‹¬ç«‹çš„**ï¼Œæ‰€ä»¥æ–°æ™ºèƒ½ä½“ä¸ä¼šç»§æ‰¿ä¸»æ™ºèƒ½ä½“çš„å¯†é’¥ã€‚

ä¿®å¤é€‰é¡¹ï¼š

- é‡æ–°è¿è¡Œæ–°æ‰‹å¼•å¯¼å¹¶ä¸ºè¯¥æ™ºèƒ½ä½“é€‰æ‹© **Anthropic**ã€‚
- æˆ–åœ¨ **Gateway ç½‘å…³ä¸»æœº**ä¸Šç²˜è´´ setup-tokenï¼š
  ```bash
  krabkrab models auth setup-token --provider anthropic
  ```
- æˆ–å°† `auth-profiles.json` ä»Žä¸»æ™ºèƒ½ä½“ç›®å½•å¤åˆ¶åˆ°æ–°æ™ºèƒ½ä½“ç›®å½•ã€‚

éªŒè¯ï¼š

```bash
krabkrab models status
```

### OAuth token refresh failedï¼ˆAnthropic Claude è®¢é˜…ï¼‰

è¿™æ„å‘³ç€å­˜å‚¨çš„ Anthropic OAuth ä»¤ç‰Œå·²è¿‡æœŸä¸”åˆ·æ–°å¤±è´¥ã€‚
å¦‚æžœä½ ä½¿ç”¨ Claude è®¢é˜…ï¼ˆæ—  API å¯†é’¥ï¼‰ï¼Œæœ€å¯é çš„ä¿®å¤æ–¹æ³•æ˜¯
åˆ‡æ¢åˆ° **Claude Code setup-token** å¹¶åœ¨ **Gateway ç½‘å…³ä¸»æœº**ä¸Šç²˜è´´ã€‚

**æŽ¨èï¼ˆsetup-tokenï¼‰ï¼š**

```bash
# åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè¿è¡Œï¼ˆç²˜è´´ setup-tokenï¼‰
krabkrab models auth setup-token --provider anthropic
krabkrab models status
```

å¦‚æžœä½ åœ¨å…¶ä»–åœ°æ–¹ç”Ÿæˆäº†ä»¤ç‰Œï¼š

```bash
krabkrab models auth paste-token --provider anthropic
krabkrab models status
```

æ›´å¤šè¯¦æƒ…ï¼š[Anthropic](/providers/anthropic) å’Œ [OAuth](/concepts/oauth)ã€‚

### Control UI åœ¨ HTTP ä¸Šå¤±è´¥ï¼ˆ"device identity required" / "connect failed"ï¼‰

å¦‚æžœä½ é€šè¿‡çº¯ HTTP æ‰“å¼€ä»ªè¡¨æ¿ï¼ˆä¾‹å¦‚ `http://<lan-ip>:18789/` æˆ–
`http://<tailscale-ip>:18789/`ï¼‰ï¼Œæµè§ˆå™¨è¿è¡Œåœ¨**éžå®‰å…¨ä¸Šä¸‹æ–‡**ä¸­ï¼Œ
ä¼šé˜»æ­¢ WebCryptoï¼Œå› æ­¤æ— æ³•ç”Ÿæˆè®¾å¤‡èº«ä»½ã€‚

**ä¿®å¤ï¼š**

- ä¼˜å…ˆé€šè¿‡ [Tailscale Serve](/gateway/tailscale) ä½¿ç”¨ HTTPSã€‚
- æˆ–åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šæœ¬åœ°æ‰“å¼€ï¼š`http://127.0.0.1:18789/`ã€‚
- å¦‚æžœå¿…é¡»ä½¿ç”¨ HTTPï¼Œå¯ç”¨ `gateway.controlUi.allowInsecureAuth: true` å¹¶
  ä½¿ç”¨ Gateway ç½‘å…³ä»¤ç‰Œï¼ˆä»…ä»¤ç‰Œï¼›æ— è®¾å¤‡èº«ä»½/é…å¯¹ï¼‰ã€‚å‚è§
  [Control UI](/web/control-ui#insecure-http)ã€‚

### CI Secrets Scan Failed

è¿™æ„å‘³ç€ `detect-secrets` å‘çŽ°äº†å°šæœªåœ¨åŸºçº¿ä¸­çš„æ–°å€™é€‰é¡¹ã€‚
æŒ‰ç…§ [å¯†é’¥æ‰«æ](/gateway/security#secret-scanning-detect-secrets) æ“ä½œã€‚

### æœåŠ¡å·²å®‰è£…ä½†æ²¡æœ‰è¿è¡Œ

å¦‚æžœ Gateway ç½‘å…³æœåŠ¡å·²å®‰è£…ä½†è¿›ç¨‹ç«‹å³é€€å‡ºï¼ŒæœåŠ¡
å¯èƒ½æ˜¾ç¤º"å·²åŠ è½½"ä½†å®žé™…æ²¡æœ‰è¿è¡Œã€‚

**æ£€æŸ¥ï¼š**

```bash
krabkrab gateway status
krabkrab doctor
```

Doctor/service å°†æ˜¾ç¤ºè¿è¡Œæ—¶çŠ¶æ€ï¼ˆPID/æœ€åŽé€€å‡ºï¼‰å’Œæ—¥å¿—æç¤ºã€‚

**æ—¥å¿—ï¼š**

- ä¼˜å…ˆï¼š`krabkrab logs --follow`
- æ–‡ä»¶æ—¥å¿—ï¼ˆå§‹ç»ˆï¼‰ï¼š`/tmp/krabkrab/krabkrab-YYYY-MM-DD.log`ï¼ˆæˆ–ä½ é…ç½®çš„ `logging.file`ï¼‰
- macOS LaunchAgentï¼ˆå¦‚æžœå·²å®‰è£…ï¼‰ï¼š`$krabkrab_STATE_DIR/logs/gateway.log` å’Œ `gateway.err.log`
- Linux systemdï¼ˆå¦‚æžœå·²å®‰è£…ï¼‰ï¼š`journalctl --user -u krabkrab-gateway[-<profile>].service -n 200 --no-pager`
- Windowsï¼š`schtasks /Query /TN "KrabKrab Gateway (<profile>)" /V /FO LIST`

**å¯ç”¨æ›´å¤šæ—¥å¿—ï¼š**

- æé«˜æ–‡ä»¶æ—¥å¿—è¯¦ç»†ç¨‹åº¦ï¼ˆæŒä¹…åŒ– JSONLï¼‰ï¼š
  ```json
  { "logging": { "level": "debug" } }
  ```
- æé«˜æŽ§åˆ¶å°è¯¦ç»†ç¨‹åº¦ï¼ˆä»… TTY è¾“å‡ºï¼‰ï¼š
  ```json
  { "logging": { "consoleLevel": "debug", "consoleStyle": "pretty" } }
  ```
- å¿«é€Ÿæç¤ºï¼š`--verbose` ä»…å½±å“**æŽ§åˆ¶å°**è¾“å‡ºã€‚æ–‡ä»¶æ—¥å¿—ä»ç”± `logging.level` æŽ§åˆ¶ã€‚

å‚è§ [/logging](/logging) äº†è§£æ ¼å¼ã€é…ç½®å’Œè®¿é—®çš„å®Œæ•´æ¦‚è¿°ã€‚

### "Gateway start blocked: set gateway.mode=local"

è¿™æ„å‘³ç€é…ç½®å­˜åœ¨ä½† `gateway.mode` æœªè®¾ç½®ï¼ˆæˆ–ä¸æ˜¯ `local`ï¼‰ï¼Œæ‰€ä»¥
Gateway ç½‘å…³æ‹’ç»å¯åŠ¨ã€‚

**ä¿®å¤ï¼ˆæŽ¨èï¼‰ï¼š**

- è¿è¡Œå‘å¯¼å¹¶å°† Gateway ç½‘å…³è¿è¡Œæ¨¡å¼è®¾ç½®ä¸º **Local**ï¼š
  ```bash
  krabkrab configure
  ```
- æˆ–ç›´æŽ¥è®¾ç½®ï¼š
  ```bash
  krabkrab config set gateway.mode local
  ```

**å¦‚æžœä½ æ‰“ç®—è¿è¡Œè¿œç¨‹ Gateway ç½‘å…³ï¼š**

- è®¾ç½®è¿œç¨‹ URL å¹¶ä¿æŒ `gateway.mode=remote`ï¼š
  ```bash
  krabkrab config set gateway.mode remote
  krabkrab config set gateway.remote.url "wss://gateway.example.com"
  ```

**ä»…ä¸´æ—¶/å¼€å‘ä½¿ç”¨ï¼š** ä¼ é€’ `--allow-unconfigured` ä»¥åœ¨æ²¡æœ‰
`gateway.mode=local` çš„æƒ…å†µä¸‹å¯åŠ¨ Gateway ç½‘å…³ã€‚

**è¿˜æ²¡æœ‰é…ç½®æ–‡ä»¶ï¼Ÿ** è¿è¡Œ `krabkrab setup` åˆ›å»ºåˆå§‹é…ç½®ï¼Œç„¶åŽé‡æ–°è¿è¡Œ
Gateway ç½‘å…³ã€‚

### æœåŠ¡çŽ¯å¢ƒï¼ˆPATH + è¿è¡Œæ—¶ï¼‰

Gateway ç½‘å…³æœåŠ¡ä½¿ç”¨**æœ€å° PATH** è¿è¡Œä»¥é¿å… shell/ç®¡ç†å™¨çš„å¹²æ‰°ï¼š

- macOSï¼š`/opt/homebrew/bin`ã€`/usr/local/bin`ã€`/usr/bin`ã€`/bin`
- Linuxï¼š`/usr/local/bin`ã€`/usr/bin`ã€`/bin`

è¿™æœ‰æ„æŽ’é™¤ç‰ˆæœ¬ç®¡ç†å™¨ï¼ˆnvm/fnm/volta/asdfï¼‰å’ŒåŒ…
ç®¡ç†å™¨ï¼ˆpnpm/npmï¼‰ï¼Œå› ä¸ºæœåŠ¡ä¸åŠ è½½ä½ çš„ shell åˆå§‹åŒ–ã€‚è¿è¡Œæ—¶
å˜é‡å¦‚ `DISPLAY` åº”è¯¥æ”¾åœ¨ `~/.krabkrab/.env` ä¸­ï¼ˆç”± Gateway ç½‘å…³æ—©æœŸåŠ è½½ï¼‰ã€‚
åœ¨ `host=gateway` ä¸Šçš„ Exec è¿è¡Œä¼šå°†ä½ çš„ç™»å½• shell `PATH` åˆå¹¶åˆ° exec çŽ¯å¢ƒä¸­ï¼Œ
æ‰€ä»¥ç¼ºå°‘çš„å·¥å…·é€šå¸¸æ„å‘³ç€ä½ çš„ shell åˆå§‹åŒ–æ²¡æœ‰å¯¼å‡ºå®ƒä»¬ï¼ˆæˆ–è®¾ç½®
`tools.exec.pathPrepend`ï¼‰ã€‚å‚è§ [/tools/exec](/tools/exec)ã€‚

WhatsApp + Telegram æ¸ é“éœ€è¦ **Node**ï¼›ä¸æ”¯æŒ Bunã€‚å¦‚æžœä½ çš„
æœåŠ¡æ˜¯ç”¨ Bun æˆ–ç‰ˆæœ¬ç®¡ç†çš„ Node è·¯å¾„å®‰è£…çš„ï¼Œè¿è¡Œ `krabkrab doctor`
è¿ç§»åˆ°ç³»ç»Ÿ Node å®‰è£…ã€‚

### æ²™ç®±ä¸­ Skill ç¼ºå°‘ API å¯†é’¥

**ç—‡çŠ¶ï¼š** Skill åœ¨ä¸»æœºä¸Šå·¥ä½œä½†åœ¨æ²™ç®±ä¸­å› ç¼ºå°‘ API å¯†é’¥è€Œå¤±è´¥ã€‚

**åŽŸå› ï¼š** æ²™ç®± exec åœ¨ Docker å†…è¿è¡Œï¼Œ**ä¸**ç»§æ‰¿ä¸»æœº `process.env`ã€‚

**ä¿®å¤ï¼š**

- è®¾ç½® `agents.defaults.sandbox.docker.env`ï¼ˆæˆ–æ¯ä¸ªæ™ºèƒ½ä½“çš„ `agents.list[].sandbox.docker.env`ï¼‰
- æˆ–å°†å¯†é’¥çƒ˜ç„™åˆ°ä½ çš„è‡ªå®šä¹‰æ²™ç®±é•œåƒä¸­
- ç„¶åŽè¿è¡Œ `krabkrab sandbox recreate --agent <id>`ï¼ˆæˆ– `--all`ï¼‰

### æœåŠ¡è¿è¡Œä½†ç«¯å£æœªç›‘å¬

å¦‚æžœæœåŠ¡æŠ¥å‘Š**æ­£åœ¨è¿è¡Œ**ä½† Gateway ç½‘å…³ç«¯å£ä¸Šæ²¡æœ‰ç›‘å¬ï¼Œ
Gateway ç½‘å…³å¯èƒ½æ‹’ç»ç»‘å®šã€‚

**è¿™é‡Œ"æ­£åœ¨è¿è¡Œ"çš„å«ä¹‰**

- `Runtime: running` æ„å‘³ç€ä½ çš„ç›‘ç®¡ç¨‹åºï¼ˆlaunchd/systemd/schtasksï¼‰è®¤ä¸ºè¿›ç¨‹å­˜æ´»ã€‚
- `RPC probe` æ„å‘³ç€ CLI å®žé™…ä¸Šèƒ½å¤Ÿè¿žæŽ¥åˆ° Gateway ç½‘å…³ WebSocket å¹¶è°ƒç”¨ `status`ã€‚
- å§‹ç»ˆä¿¡ä»» `Probe target:` + `Config (service):` ä½œä¸º"æˆ‘ä»¬å®žé™…å°è¯•äº†ä»€ä¹ˆï¼Ÿ"çš„ä¿¡æ¯è¡Œã€‚

**æ£€æŸ¥ï¼š**

- `gateway.mode` å¿…é¡»ä¸º `local` æ‰èƒ½è¿è¡Œ `krabkrab gateway` å’ŒæœåŠ¡ã€‚
- å¦‚æžœä½ è®¾ç½®äº† `gateway.mode=remote`ï¼Œ**CLI é»˜è®¤**ä½¿ç”¨è¿œç¨‹ URLã€‚æœåŠ¡å¯èƒ½ä»åœ¨æœ¬åœ°è¿è¡Œï¼Œä½†ä½ çš„ CLI å¯èƒ½åœ¨æŽ¢æµ‹é”™è¯¯çš„ä½ç½®ã€‚ä½¿ç”¨ `krabkrab gateway status` æŸ¥çœ‹æœåŠ¡è§£æžçš„ç«¯å£ + æŽ¢æµ‹ç›®æ ‡ï¼ˆæˆ–ä¼ é€’ `--url`ï¼‰ã€‚
- `krabkrab gateway status` å’Œ `krabkrab doctor` åœ¨æœåŠ¡çœ‹èµ·æ¥æ­£åœ¨è¿è¡Œä½†ç«¯å£å…³é—­æ—¶ä¼šæ˜¾ç¤ºæ—¥å¿—ä¸­çš„**æœ€åŽ Gateway ç½‘å…³é”™è¯¯**ã€‚
- éžæœ¬åœ°å›žçŽ¯ç»‘å®šï¼ˆ`lan`/`tailnet`/`custom`ï¼Œæˆ–æœ¬åœ°å›žçŽ¯ä¸å¯ç”¨æ—¶çš„ `auto`ï¼‰éœ€è¦è®¤è¯ï¼š
  `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- `gateway.remote.token` ä»…ç”¨äºŽè¿œç¨‹ CLI è°ƒç”¨ï¼›å®ƒ**ä¸**å¯ç”¨æœ¬åœ°è®¤è¯ã€‚
- `gateway.token` è¢«å¿½ç•¥ï¼›ä½¿ç”¨ `gateway.auth.token`ã€‚

**å¦‚æžœ `krabkrab gateway status` æ˜¾ç¤ºé…ç½®ä¸åŒ¹é…**

- `Config (cli): ...` å’Œ `Config (service): ...` é€šå¸¸åº”è¯¥åŒ¹é…ã€‚
- å¦‚æžœä¸åŒ¹é…ï¼Œä½ å‡ ä¹Žè‚¯å®šæ˜¯åœ¨ç¼–è¾‘ä¸€ä¸ªé…ç½®è€ŒæœåŠ¡è¿è¡Œçš„æ˜¯å¦ä¸€ä¸ªã€‚
- ä¿®å¤ï¼šä»Žä½ å¸Œæœ›æœåŠ¡ä½¿ç”¨çš„ç›¸åŒ `--profile` / `krabkrab_STATE_DIR` é‡æ–°è¿è¡Œ `krabkrab gateway install --force`ã€‚

**å¦‚æžœ `krabkrab gateway status` æŠ¥å‘ŠæœåŠ¡é…ç½®é—®é¢˜**

- ç›‘ç®¡ç¨‹åºé…ç½®ï¼ˆlaunchd/systemd/schtasksï¼‰ç¼ºå°‘å½“å‰é»˜è®¤å€¼ã€‚
- ä¿®å¤ï¼šè¿è¡Œ `krabkrab doctor` æ›´æ–°å®ƒï¼ˆæˆ– `krabkrab gateway install --force` å®Œå…¨é‡å†™ï¼‰ã€‚

**å¦‚æžœ `Last gateway error:` æåˆ°"refusing to bind â€¦ without auth"**

- ä½ å°† `gateway.bind` è®¾ç½®ä¸ºéžæœ¬åœ°å›žçŽ¯æ¨¡å¼ï¼ˆ`lan`/`tailnet`/`custom`ï¼Œæˆ–æœ¬åœ°å›žçŽ¯ä¸å¯ç”¨æ—¶çš„ `auto`ï¼‰ä½†æ²¡æœ‰é…ç½®è®¤è¯ã€‚
- ä¿®å¤ï¼šè®¾ç½® `gateway.auth.mode` + `gateway.auth.token`ï¼ˆæˆ–å¯¼å‡º `krabkrab_GATEWAY_TOKEN`ï¼‰å¹¶é‡å¯æœåŠ¡ã€‚

**å¦‚æžœ `krabkrab gateway status` æ˜¾ç¤º `bind=tailnet` ä½†æœªæ‰¾åˆ° tailnet æŽ¥å£**

- Gateway ç½‘å…³å°è¯•ç»‘å®šåˆ° Tailscale IPï¼ˆ100.64.0.0/10ï¼‰ä½†åœ¨ä¸»æœºä¸Šæœªæ£€æµ‹åˆ°ã€‚
- ä¿®å¤ï¼šåœ¨è¯¥æœºå™¨ä¸Šå¯åŠ¨ Tailscaleï¼ˆæˆ–å°† `gateway.bind` æ”¹ä¸º `loopback`/`lan`ï¼‰ã€‚

**å¦‚æžœ `Probe note:` è¯´æŽ¢æµ‹ä½¿ç”¨æœ¬åœ°å›žçŽ¯**

- å¯¹äºŽ `bind=lan` è¿™æ˜¯é¢„æœŸçš„ï¼šGateway ç½‘å…³ç›‘å¬ `0.0.0.0`ï¼ˆæ‰€æœ‰æŽ¥å£ï¼‰ï¼Œæœ¬åœ°å›žçŽ¯ä»åº”æœ¬åœ°è¿žæŽ¥ã€‚
- å¯¹äºŽè¿œç¨‹å®¢æˆ·ç«¯ï¼Œä½¿ç”¨çœŸå®žçš„ LAN IPï¼ˆä¸æ˜¯ `0.0.0.0`ï¼‰åŠ ç«¯å£ï¼Œå¹¶ç¡®ä¿é…ç½®äº†è®¤è¯ã€‚

### åœ°å€å·²è¢«ä½¿ç”¨ï¼ˆç«¯å£ 18789ï¼‰

è¿™æ„å‘³ç€æŸäº›ä¸œè¥¿å·²ç»åœ¨ Gateway ç½‘å…³ç«¯å£ä¸Šç›‘å¬ã€‚

**æ£€æŸ¥ï¼š**

```bash
krabkrab gateway status
```

å®ƒå°†æ˜¾ç¤ºç›‘å¬å™¨å’Œå¯èƒ½çš„åŽŸå› ï¼ˆGateway ç½‘å…³å·²åœ¨è¿è¡Œã€SSH éš§é“ï¼‰ã€‚
å¦‚æžœéœ€è¦ï¼Œåœæ­¢æœåŠ¡æˆ–é€‰æ‹©ä¸åŒçš„ç«¯å£ã€‚

### æ£€æµ‹åˆ°é¢å¤–çš„å·¥ä½œåŒºæ–‡ä»¶å¤¹

å¦‚æžœä½ ä»Žæ—§ç‰ˆæœ¬å‡çº§ï¼Œä½ çš„ç£ç›˜ä¸Šå¯èƒ½ä»æœ‰ `~/krabkrab`ã€‚
å¤šä¸ªå·¥ä½œåŒºç›®å½•å¯èƒ½å¯¼è‡´ä»¤äººå›°æƒ‘çš„è®¤è¯æˆ–çŠ¶æ€æ¼‚ç§»ï¼Œå› ä¸º
åªæœ‰ä¸€ä¸ªå·¥ä½œåŒºæ˜¯æ´»åŠ¨çš„ã€‚

**ä¿®å¤ï¼š** ä¿ç•™å•ä¸ªæ´»åŠ¨å·¥ä½œåŒºå¹¶å½’æ¡£/åˆ é™¤å…¶ä½™çš„ã€‚å‚è§
[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace#extra-workspace-folders)ã€‚

### ä¸»èŠå¤©åœ¨æ²™ç®±å·¥ä½œåŒºä¸­è¿è¡Œ

ç—‡çŠ¶ï¼š`pwd` æˆ–æ–‡ä»¶å·¥å…·æ˜¾ç¤º `~/.krabkrab/sandboxes/...` å³ä½¿ä½ 
æœŸæœ›çš„æ˜¯ä¸»æœºå·¥ä½œåŒºã€‚

**åŽŸå› ï¼š** `agents.defaults.sandbox.mode: "non-main"` åŸºäºŽ `session.mainKey`ï¼ˆé»˜è®¤ `"main"`ï¼‰åˆ¤æ–­ã€‚
ç¾¤ç»„/æ¸ é“ä¼šè¯ä½¿ç”¨å®ƒä»¬è‡ªå·±çš„é”®ï¼Œæ‰€ä»¥å®ƒä»¬è¢«è§†ä¸ºéžä¸»ä¼šè¯å¹¶
èŽ·å¾—æ²™ç®±å·¥ä½œåŒºã€‚

**ä¿®å¤é€‰é¡¹ï¼š**

- å¦‚æžœä½ æƒ³ä¸ºæ™ºèƒ½ä½“ä½¿ç”¨ä¸»æœºå·¥ä½œåŒºï¼šè®¾ç½® `agents.list[].sandbox.mode: "off"`ã€‚
- å¦‚æžœä½ æƒ³åœ¨æ²™ç®±å†…è®¿é—®ä¸»æœºå·¥ä½œåŒºï¼šä¸ºè¯¥æ™ºèƒ½ä½“è®¾ç½® `workspaceAccess: "rw"`ã€‚

### "Agent was aborted"

æ™ºèƒ½ä½“åœ¨å“åº”ä¸­é€”è¢«ä¸­æ–­ã€‚

**åŽŸå› ï¼š**

- ç”¨æˆ·å‘é€äº† `stop`ã€`abort`ã€`esc`ã€`wait` æˆ– `exit`
- è¶…æ—¶è¶…é™
- è¿›ç¨‹å´©æºƒ

**ä¿®å¤ï¼š** åªéœ€å‘é€å¦ä¸€æ¡æ¶ˆæ¯ã€‚ä¼šè¯å°†ç»§ç»­ã€‚

### "Agent failed before reply: Unknown model: anthropic/claude-haiku-3-5"

KrabKrab æœ‰æ„æ‹’ç»**è¾ƒæ—§/ä¸å®‰å…¨çš„æ¨¡åž‹**ï¼ˆå°¤å…¶æ˜¯é‚£äº›æ›´å®¹æ˜“å—åˆ°æç¤ºè¯æ³¨å…¥æ”»å‡»çš„æ¨¡åž‹ï¼‰ã€‚å¦‚æžœä½ çœ‹åˆ°æ­¤é”™è¯¯ï¼Œè¯¥æ¨¡åž‹åç§°å·²ä¸å†æ”¯æŒã€‚

**ä¿®å¤ï¼š**

- ä¸ºæä¾›å•†é€‰æ‹©**æœ€æ–°**æ¨¡åž‹å¹¶æ›´æ–°ä½ çš„é…ç½®æˆ–æ¨¡åž‹åˆ«åã€‚
- å¦‚æžœä½ ä¸ç¡®å®šå“ªäº›æ¨¡åž‹å¯ç”¨ï¼Œè¿è¡Œ `krabkrab models list` æˆ–
  `krabkrab models scan` å¹¶é€‰æ‹©ä¸€ä¸ªæ”¯æŒçš„æ¨¡åž‹ã€‚
- æ£€æŸ¥ Gateway ç½‘å…³æ—¥å¿—ä»¥èŽ·å–è¯¦ç»†çš„å¤±è´¥åŽŸå› ã€‚

å¦è¯·å‚é˜…ï¼š[æ¨¡åž‹ CLI](/cli/models) å’Œ [æ¨¡åž‹æä¾›å•†](/concepts/model-providers)ã€‚

### æ¶ˆæ¯æœªè§¦å‘

**æ£€æŸ¥ 1ï¼š** å‘é€è€…æ˜¯å¦åœ¨ç™½åå•ä¸­ï¼Ÿ

```bash
krabkrab status
```

åœ¨è¾“å‡ºä¸­æŸ¥æ‰¾ `AllowFrom: ...`ã€‚

**æ£€æŸ¥ 2ï¼š** å¯¹äºŽç¾¤èŠï¼Œæ˜¯å¦éœ€è¦æåŠï¼Ÿ

```bash
# æ¶ˆæ¯å¿…é¡»åŒ¹é… mentionPatterns æˆ–æ˜¾å¼æåŠï¼›é»˜è®¤å€¼åœ¨æ¸ é“ groups/guilds ä¸­ã€‚
# å¤šæ™ºèƒ½ä½“ï¼š`agents.list[].groupChat.mentionPatterns` è¦†ç›–å…¨å±€æ¨¡å¼ã€‚
grep -n "agents\\|groupChat\\|mentionPatterns\\|channels\\.whatsapp\\.groups\\|channels\\.telegram\\.groups\\|channels\\.imessage\\.groups\\|channels\\.discord\\.guilds" \
  "${krabkrab_CONFIG_PATH:-$HOME/.krabkrab/krabkrab.json}"
```

**æ£€æŸ¥ 3ï¼š** æ£€æŸ¥æ—¥å¿—

```bash
krabkrab logs --follow
# æˆ–è€…å¦‚æžœä½ æƒ³å¿«é€Ÿè¿‡æ»¤ï¼š
tail -f "$(ls -t /tmp/krabkrab/krabkrab-*.log | head -1)" | grep "blocked\\|skip\\|unauthorized"
```

### é…å¯¹ç æœªåˆ°è¾¾

å¦‚æžœ `dmPolicy` æ˜¯ `pairing`ï¼ŒæœªçŸ¥å‘é€è€…åº”è¯¥æ”¶åˆ°ä¸€ä¸ªä»£ç ï¼Œä»–ä»¬çš„æ¶ˆæ¯åœ¨æ‰¹å‡†å‰ä¼šè¢«å¿½ç•¥ã€‚

**æ£€æŸ¥ 1ï¼š** æ˜¯å¦å·²æœ‰å¾…å¤„ç†çš„è¯·æ±‚åœ¨ç­‰å¾…ï¼Ÿ

```bash
krabkrab pairing list <channel>
```

å¾…å¤„ç†çš„ç§ä¿¡é…å¯¹è¯·æ±‚é»˜è®¤æ¯ä¸ªæ¸ é“ä¸Šé™ä¸º **3 ä¸ª**ã€‚å¦‚æžœåˆ—è¡¨å·²æ»¡ï¼Œæ–°è¯·æ±‚å°†ä¸ä¼šç”Ÿæˆä»£ç ï¼Œç›´åˆ°ä¸€ä¸ªè¢«æ‰¹å‡†æˆ–è¿‡æœŸã€‚

**æ£€æŸ¥ 2ï¼š** è¯·æ±‚æ˜¯å¦å·²åˆ›å»ºä½†æœªå‘é€å›žå¤ï¼Ÿ

```bash
krabkrab logs --follow | grep "pairing request"
```

**æ£€æŸ¥ 3ï¼š** ç¡®è®¤è¯¥æ¸ é“çš„ `dmPolicy` ä¸æ˜¯ `open`/`allowlist`ã€‚

### å›¾ç‰‡ + æåŠä¸å·¥ä½œ

å·²çŸ¥é—®é¢˜ï¼šå½“ä½ å‘é€åªæœ‰æåŠçš„å›¾ç‰‡ï¼ˆæ²¡æœ‰å…¶ä»–æ–‡å­—ï¼‰æ—¶ï¼ŒWhatsApp æœ‰æ—¶ä¸åŒ…å«æåŠå…ƒæ•°æ®ã€‚

**å˜é€šæ–¹æ³•ï¼š** åœ¨æåŠæ—¶æ·»åŠ ä¸€äº›æ–‡å­—ï¼š

- âŒ `@krabkrab` + å›¾ç‰‡
- âœ… `@krabkrab check this` + å›¾ç‰‡

### ä¼šè¯æœªæ¢å¤

**æ£€æŸ¥ 1ï¼š** ä¼šè¯æ–‡ä»¶æ˜¯å¦å­˜åœ¨ï¼Ÿ

```bash
ls -la ~/.krabkrab/agents/<agentId>/sessions/
```

**æ£€æŸ¥ 2ï¼š** é‡ç½®çª—å£æ˜¯å¦å¤ªçŸ­ï¼Ÿ

```json
{
  "session": {
    "reset": {
      "mode": "daily",
      "atHour": 4,
      "idleMinutes": 10080 // 7 å¤©
    }
  }
}
```

**æ£€æŸ¥ 3ï¼š** æ˜¯å¦æœ‰äººå‘é€äº† `/new`ã€`/reset` æˆ–é‡ç½®è§¦å‘å™¨ï¼Ÿ

### æ™ºèƒ½ä½“è¶…æ—¶

é»˜è®¤è¶…æ—¶æ˜¯ 30 åˆ†é’Ÿã€‚å¯¹äºŽé•¿ä»»åŠ¡ï¼š

```json
{
  "reply": {
    "timeoutSeconds": 3600 // 1 å°æ—¶
  }
}
```

æˆ–ä½¿ç”¨ `process` å·¥å…·åœ¨åŽå°è¿è¡Œé•¿å‘½ä»¤ã€‚

### WhatsApp æ–­å¼€è¿žæŽ¥

```bash
# æ£€æŸ¥æœ¬åœ°çŠ¶æ€ï¼ˆå‡­è¯ã€ä¼šè¯ã€æŽ’é˜Ÿäº‹ä»¶ï¼‰
krabkrab status
# æŽ¢æµ‹è¿è¡Œä¸­çš„ Gateway ç½‘å…³ + æ¸ é“ï¼ˆWA è¿žæŽ¥ + Telegram + Discord APIï¼‰
krabkrab status --deep

# æŸ¥çœ‹æœ€è¿‘çš„è¿žæŽ¥äº‹ä»¶
krabkrab logs --limit 200 | grep "connection\\|disconnect\\|logout"
```

**ä¿®å¤ï¼š** é€šå¸¸åœ¨ Gateway ç½‘å…³è¿è¡ŒåŽä¼šè‡ªåŠ¨é‡è¿žã€‚å¦‚æžœå¡ä½ï¼Œé‡å¯ Gateway ç½‘å…³è¿›ç¨‹ï¼ˆæ— è®ºä½ å¦‚ä½•ç›‘ç®¡å®ƒï¼‰ï¼Œæˆ–ä½¿ç”¨è¯¦ç»†è¾“å‡ºæ‰‹åŠ¨è¿è¡Œï¼š

```bash
krabkrab gateway --verbose
```

å¦‚æžœä½ å·²ç™»å‡º/å–æ¶ˆå…³è”ï¼š

```bash
krabkrab channels logout
trash "${krabkrab_STATE_DIR:-$HOME/.krabkrab}/credentials" # å¦‚æžœ logout æ— æ³•å®Œå…¨æ¸…é™¤æ‰€æœ‰å†…å®¹
krabkrab channels login --verbose       # é‡æ–°æ‰«æäºŒç»´ç 
```

### åª’ä½“å‘é€å¤±è´¥

**æ£€æŸ¥ 1ï¼š** æ–‡ä»¶è·¯å¾„æ˜¯å¦æœ‰æ•ˆï¼Ÿ

```bash
ls -la /path/to/your/image.jpg
```

**æ£€æŸ¥ 2ï¼š** æ˜¯å¦å¤ªå¤§ï¼Ÿ

- å›¾ç‰‡ï¼šæœ€å¤§ 6MB
- éŸ³é¢‘/è§†é¢‘ï¼šæœ€å¤§ 16MB
- æ–‡æ¡£ï¼šæœ€å¤§ 100MB

**æ£€æŸ¥ 3ï¼š** æ£€æŸ¥åª’ä½“æ—¥å¿—

```bash
grep "media\\|fetch\\|download" "$(ls -t /tmp/krabkrab/krabkrab-*.log | head -1)" | tail -20
```

### é«˜å†…å­˜ä½¿ç”¨

KrabKrab åœ¨å†…å­˜ä¸­ä¿ç•™å¯¹è¯åŽ†å²ã€‚

**ä¿®å¤ï¼š** å®šæœŸé‡å¯æˆ–è®¾ç½®ä¼šè¯é™åˆ¶ï¼š

```json
{
  "session": {
    "historyLimit": 100 // ä¿ç•™çš„æœ€å¤§æ¶ˆæ¯æ•°
  }
}
```

## å¸¸è§æ•…éšœæŽ’é™¤

### "Gateway won't start â€” configuration invalid"

å½“é…ç½®åŒ…å«æœªçŸ¥é”®ã€æ ¼å¼é”™è¯¯çš„å€¼æˆ–æ— æ•ˆç±»åž‹æ—¶ï¼ŒKrabKrab çŽ°åœ¨æ‹’ç»å¯åŠ¨ã€‚
è¿™æ˜¯ä¸ºäº†å®‰å…¨è€Œæ•…æ„è®¾è®¡çš„ã€‚

ç”¨ Doctor ä¿®å¤ï¼š

```bash
krabkrab doctor
krabkrab doctor --fix
```

æ³¨æ„äº‹é¡¹ï¼š

- `krabkrab doctor` æŠ¥å‘Šæ¯ä¸ªæ— æ•ˆæ¡ç›®ã€‚
- `krabkrab doctor --fix` åº”ç”¨è¿ç§»/ä¿®å¤å¹¶é‡å†™é…ç½®ã€‚
- è¯Šæ–­å‘½ä»¤å¦‚ `krabkrab logs`ã€`krabkrab health`ã€`krabkrab status`ã€`krabkrab gateway status` å’Œ `krabkrab gateway probe` å³ä½¿é…ç½®æ— æ•ˆä¹Ÿèƒ½è¿è¡Œã€‚

### "All models failed" â€” æˆ‘åº”è¯¥é¦–å…ˆæ£€æŸ¥ä»€ä¹ˆï¼Ÿ

- **å‡­è¯**å­˜åœ¨äºŽæ­£åœ¨å°è¯•çš„æä¾›å•†ï¼ˆè®¤è¯é…ç½®æ–‡ä»¶ + çŽ¯å¢ƒå˜é‡ï¼‰ã€‚
- **æ¨¡åž‹è·¯ç”±**ï¼šç¡®è®¤ `agents.defaults.model.primary` å’Œå›žé€€æ˜¯ä½ å¯ä»¥è®¿é—®çš„æ¨¡åž‹ã€‚
- `/tmp/krabkrab/â€¦` ä¸­çš„ **Gateway ç½‘å…³æ—¥å¿—**ä»¥èŽ·å–ç¡®åˆ‡çš„æä¾›å•†é”™è¯¯ã€‚
- **æ¨¡åž‹çŠ¶æ€**ï¼šä½¿ç”¨ `/model status`ï¼ˆèŠå¤©ï¼‰æˆ– `krabkrab models status`ï¼ˆCLIï¼‰ã€‚

### æˆ‘åœ¨æˆ‘çš„ä¸ªäºº WhatsApp å·ç ä¸Šè¿è¡Œ â€” ä¸ºä»€ä¹ˆè‡ªèŠå¤©å¾ˆå¥‡æ€ªï¼Ÿ

å¯ç”¨è‡ªèŠå¤©æ¨¡å¼å¹¶å°†ä½ è‡ªå·±çš„å·ç åŠ å…¥ç™½åå•ï¼š

```json5
{
  channels: {
    whatsapp: {
      selfChatMode: true,
      dmPolicy: "allowlist",
      allowFrom: ["+15555550123"],
    },
  },
}
```

å‚è§ [WhatsApp è®¾ç½®](/channels/whatsapp)ã€‚

### WhatsApp å°†æˆ‘ç™»å‡ºã€‚å¦‚ä½•é‡æ–°è®¤è¯ï¼Ÿ

å†æ¬¡è¿è¡Œç™»å½•å‘½ä»¤å¹¶æ‰«æäºŒç»´ç ï¼š

```bash
krabkrab channels login
```

### `main` ä¸Šçš„æž„å»ºé”™è¯¯ â€” æ ‡å‡†ä¿®å¤è·¯å¾„æ˜¯ä»€ä¹ˆï¼Ÿ

1. `git pull origin main && pnpm install`
2. `krabkrab doctor`
3. æ£€æŸ¥ GitHub issues æˆ– Discord
4. ä¸´æ—¶å˜é€šæ–¹æ³•ï¼šæ£€å‡ºè¾ƒæ—§çš„æäº¤

### npm install å¤±è´¥ï¼ˆallow-build-scripts / ç¼ºå°‘ tar æˆ– yargsï¼‰ã€‚çŽ°åœ¨æ€Žä¹ˆåŠžï¼Ÿ

å¦‚æžœä½ ä»Žæºä»£ç è¿è¡Œï¼Œä½¿ç”¨ä»“åº“çš„åŒ…ç®¡ç†å™¨ï¼š**pnpm**ï¼ˆé¦–é€‰ï¼‰ã€‚
ä»“åº“å£°æ˜Žäº† `packageManager: "pnpm@â€¦"`ã€‚

å…¸åž‹æ¢å¤ï¼š

```bash
git status   # ç¡®ä¿ä½ åœ¨ä»“åº“æ ¹ç›®å½•
pnpm install
pnpm build
krabkrab doctor
krabkrab gateway restart
```

åŽŸå› ï¼špnpm æ˜¯æ­¤ä»“åº“é…ç½®çš„åŒ…ç®¡ç†å™¨ã€‚

### å¦‚ä½•åœ¨ git å®‰è£…å’Œ npm å®‰è£…ä¹‹é—´åˆ‡æ¢ï¼Ÿ

ä½¿ç”¨**ç½‘ç«™å®‰è£…ç¨‹åº**å¹¶é€šè¿‡æ ‡å¿—é€‰æ‹©å®‰è£…æ–¹æ³•ã€‚å®ƒ
åŽŸåœ°å‡çº§å¹¶é‡å†™ Gateway ç½‘å…³æœåŠ¡ä»¥æŒ‡å‘æ–°å®‰è£…ã€‚

åˆ‡æ¢**åˆ° git å®‰è£…**ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git --no-onboard
```

åˆ‡æ¢**åˆ° npm å…¨å±€**ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash
```

æ³¨æ„äº‹é¡¹ï¼š

- git æµç¨‹ä»…åœ¨ä»“åº“å¹²å‡€æ—¶æ‰ rebaseã€‚å…ˆæäº¤æˆ– stash æ›´æ”¹ã€‚
- åˆ‡æ¢åŽï¼Œè¿è¡Œï¼š
  ```bash
  krabkrab doctor
  krabkrab gateway restart
  ```

### Telegram åˆ†å—æµå¼ä¼ è¾“æ²¡æœ‰åœ¨å·¥å…·è°ƒç”¨ä¹‹é—´åˆ†å‰²æ–‡æœ¬ã€‚ä¸ºä»€ä¹ˆï¼Ÿ

åˆ†å—æµå¼ä¼ è¾“åªå‘é€**å·²å®Œæˆçš„æ–‡æœ¬å—**ã€‚ä½ çœ‹åˆ°å•æ¡æ¶ˆæ¯çš„å¸¸è§åŽŸå› ï¼š

- `agents.defaults.blockStreamingDefault` ä»ç„¶æ˜¯ `"off"`ã€‚
- `channels.telegram.blockStreaming` è®¾ç½®ä¸º `false`ã€‚
- `channels.telegram.streamMode` æ˜¯ `partial` æˆ– `block` **ä¸”è‰ç¨¿æµå¼ä¼ è¾“å¤„äºŽæ´»åŠ¨çŠ¶æ€**
  ï¼ˆç§èŠ + è¯é¢˜ï¼‰ã€‚åœ¨è¿™ç§æƒ…å†µä¸‹ï¼Œè‰ç¨¿æµå¼ä¼ è¾“ä¼šç¦ç”¨åˆ†å—æµå¼ä¼ è¾“ã€‚
- ä½ çš„ `minChars` / coalesce è®¾ç½®å¤ªé«˜ï¼Œæ‰€ä»¥å—è¢«åˆå¹¶äº†ã€‚
- æ¨¡åž‹å‘å‡ºä¸€ä¸ªå¤§çš„æ–‡æœ¬å—ï¼ˆæ²¡æœ‰ä¸­é—´å›žå¤åˆ·æ–°ç‚¹ï¼‰ã€‚

ä¿®å¤æ¸…å•ï¼š

1. å°†åˆ†å—æµå¼ä¼ è¾“è®¾ç½®æ”¾åœ¨ `agents.defaults` ä¸‹ï¼Œè€Œä¸æ˜¯æ ¹ç›®å½•ã€‚
2. å¦‚æžœä½ æƒ³è¦çœŸæ­£çš„å¤šæ¶ˆæ¯åˆ†å—å›žå¤ï¼Œè®¾ç½® `channels.telegram.streamMode: "off"`ã€‚
3. è°ƒè¯•æ—¶ä½¿ç”¨è¾ƒå°çš„ chunk/coalesce é˜ˆå€¼ã€‚

å‚è§ [æµå¼ä¼ è¾“](/concepts/streaming)ã€‚

### å³ä½¿è®¾ç½®äº† `requireMention: false`ï¼ŒDiscord ä¹Ÿä¸åœ¨æˆ‘çš„æœåŠ¡å™¨ä¸­å›žå¤ã€‚ä¸ºä»€ä¹ˆï¼Ÿ

`requireMention` åªæŽ§åˆ¶æ¸ é“é€šè¿‡ç™½åå•**ä¹‹åŽ**çš„æåŠé—¨æŽ§ã€‚
é»˜è®¤æƒ…å†µä¸‹ `channels.discord.groupPolicy` æ˜¯ **allowlist**ï¼Œæ‰€ä»¥å¿…é¡»æ˜¾å¼å¯ç”¨ guildã€‚
å¦‚æžœä½ è®¾ç½®äº† `channels.discord.guilds.<guildId>.channels`ï¼Œåªå…è®¸åˆ—å‡ºçš„é¢‘é“ï¼›çœç•¥å®ƒä»¥å…è®¸ guild ä¸­çš„æ‰€æœ‰é¢‘é“ã€‚

ä¿®å¤æ¸…å•ï¼š

1. è®¾ç½® `channels.discord.groupPolicy: "open"` **æˆ–**æ·»åŠ  guild ç™½åå•æ¡ç›®ï¼ˆå¹¶å¯é€‰æ·»åŠ é¢‘é“ç™½åå•ï¼‰ã€‚
2. åœ¨ `channels.discord.guilds.<guildId>.channels` ä¸­ä½¿ç”¨**æ•°å­—é¢‘é“ ID**ã€‚
3. å°† `requireMention: false` æ”¾åœ¨ `channels.discord.guilds` **ä¸‹é¢**ï¼ˆå…¨å±€æˆ–æ¯ä¸ªé¢‘é“ï¼‰ã€‚
   é¡¶çº§ `channels.discord.requireMention` ä¸æ˜¯æ”¯æŒçš„é”®ã€‚
4. ç¡®ä¿æœºå™¨äººæœ‰ **Message Content Intent** å’Œé¢‘é“æƒé™ã€‚
5. è¿è¡Œ `krabkrab channels status --probe` èŽ·å–å®¡è®¡æç¤ºã€‚

æ–‡æ¡£ï¼š[Discord](/channels/discord)ã€[æ¸ é“æ•…éšœæŽ’é™¤](/channels/troubleshooting)ã€‚

### Cloud Code Assist API é”™è¯¯ï¼šinvalid tool schemaï¼ˆ400ï¼‰ã€‚çŽ°åœ¨æ€Žä¹ˆåŠžï¼Ÿ

è¿™å‡ ä¹Žæ€»æ˜¯**å·¥å…·æ¨¡å¼å…¼å®¹æ€§**é—®é¢˜ã€‚Cloud Code Assist
ç«¯ç‚¹æŽ¥å— JSON Schema çš„ä¸¥æ ¼å­é›†ã€‚KrabKrab åœ¨å½“å‰ `main` ä¸­æ¸…ç†/è§„èŒƒåŒ–å·¥å…·
æ¨¡å¼ï¼Œä½†ä¿®å¤å°šæœªåŒ…å«åœ¨æœ€åŽä¸€ä¸ªç‰ˆæœ¬ä¸­ï¼ˆæˆªè‡³
2026 å¹´ 1 æœˆ 13 æ—¥ï¼‰ã€‚

ä¿®å¤æ¸…å•ï¼š

1. **æ›´æ–° KrabKrab**ï¼š
   - å¦‚æžœä½ å¯ä»¥ä»Žæºä»£ç è¿è¡Œï¼Œæ‹‰å– `main` å¹¶é‡å¯ Gateway ç½‘å…³ã€‚
   - å¦åˆ™ï¼Œç­‰å¾…åŒ…å«æ¨¡å¼æ¸…ç†å™¨çš„ä¸‹ä¸€ä¸ªç‰ˆæœ¬ã€‚
2. é¿å…ä¸æ”¯æŒçš„å…³é”®å­—å¦‚ `anyOf/oneOf/allOf`ã€`patternProperties`ã€
   `additionalProperties`ã€`minLength`ã€`maxLength`ã€`format` ç­‰ã€‚
3. å¦‚æžœä½ å®šä¹‰è‡ªå®šä¹‰å·¥å…·ï¼Œä¿æŒé¡¶çº§æ¨¡å¼ä¸º `type: "object"` å¹¶ä½¿ç”¨
   `properties` å’Œç®€å•æžšä¸¾ã€‚

å‚è§ [å·¥å…·](/tools) å’Œ [TypeBox æ¨¡å¼](/concepts/typebox)ã€‚

## macOS ç‰¹å®šé—®é¢˜

### æŽˆäºˆæƒé™ï¼ˆè¯­éŸ³/éº¦å…‹é£Žï¼‰æ—¶åº”ç”¨å´©æºƒ

å¦‚æžœåœ¨ä½ ç‚¹å‡»éšç§æç¤ºçš„"å…è®¸"æ—¶åº”ç”¨æ¶ˆå¤±æˆ–æ˜¾ç¤º"Abort trap 6"ï¼š

**ä¿®å¤ 1ï¼šé‡ç½® TCC ç¼“å­˜**

```bash
tccutil reset All bot.molt.mac.debug
```

**ä¿®å¤ 2ï¼šå¼ºåˆ¶ä½¿ç”¨æ–°çš„ Bundle ID**
å¦‚æžœé‡ç½®ä¸èµ·ä½œç”¨ï¼Œåœ¨ [`scripts/package-mac-app.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/package-mac-app.sh) ä¸­æ›´æ”¹ `BUNDLE_ID`ï¼ˆä¾‹å¦‚ï¼Œæ·»åŠ  `.test` åŽç¼€ï¼‰å¹¶é‡æ–°æž„å»ºã€‚è¿™ä¼šå¼ºåˆ¶ macOS å°†å…¶è§†ä¸ºæ–°åº”ç”¨ã€‚

### Gateway ç½‘å…³å¡åœ¨"Starting..."

åº”ç”¨è¿žæŽ¥åˆ°ç«¯å£ `18789` ä¸Šçš„æœ¬åœ° Gateway ç½‘å…³ã€‚å¦‚æžœä¸€ç›´å¡ä½ï¼š

**ä¿®å¤ 1ï¼šåœæ­¢ç›‘ç®¡ç¨‹åºï¼ˆé¦–é€‰ï¼‰**
å¦‚æžœ Gateway ç½‘å…³ç”± launchd ç›‘ç®¡ï¼Œæ€æ­» PID åªä¼šé‡æ–°ç”Ÿæˆå®ƒã€‚å…ˆåœæ­¢ç›‘ç®¡ç¨‹åºï¼š

```bash
krabkrab gateway status
krabkrab gateway stop
# æˆ–ï¼šlaunchctl bootout gui/$UID/bot.molt.gatewayï¼ˆç”¨ bot.molt.<profile> æ›¿æ¢ï¼›æ—§ç‰ˆ com.krabkrab.* ä»ç„¶æœ‰æ•ˆï¼‰
```

**ä¿®å¤ 2ï¼šç«¯å£è¢«å ç”¨ï¼ˆæŸ¥æ‰¾ç›‘å¬å™¨ï¼‰**

```bash
lsof -nP -iTCP:18789 -sTCP:LISTEN
```

å¦‚æžœæ˜¯æœªè¢«ç›‘ç®¡çš„è¿›ç¨‹ï¼Œå…ˆå°è¯•ä¼˜é›…åœæ­¢ï¼Œç„¶åŽå‡çº§ï¼š

```bash
kill -TERM <PID>
sleep 1
kill -9 <PID> # æœ€åŽæ‰‹æ®µ
```

**ä¿®å¤ 3ï¼šæ£€æŸ¥ CLI å®‰è£…**
ç¡®ä¿å…¨å±€ `krabkrab` CLI å·²å®‰è£…ä¸”ä¸Žåº”ç”¨ç‰ˆæœ¬åŒ¹é…ï¼š

```bash
krabkrab --version
npm install -g krabkrab@<version>
```

## è°ƒè¯•æ¨¡å¼

èŽ·å–è¯¦ç»†æ—¥å¿—ï¼š

```bash
# åœ¨é…ç½®ä¸­æ‰“å¼€è·Ÿè¸ªæ—¥å¿—ï¼š
#   ${krabkrab_CONFIG_PATH:-$HOME/.krabkrab/krabkrab.json} -> { logging: { level: "trace" } }
#
# ç„¶åŽè¿è¡Œè¯¦ç»†å‘½ä»¤å°†è°ƒè¯•è¾“å‡ºé•œåƒåˆ°æ ‡å‡†è¾“å‡ºï¼š
krabkrab gateway --verbose
krabkrab channels login --verbose
```

## æ—¥å¿—ä½ç½®

| æ—¥å¿—                             | ä½ç½®                                                                                                                                                                                                                                                                                                                      |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Gateway ç½‘å…³æ–‡ä»¶æ—¥å¿—ï¼ˆç»“æž„åŒ–ï¼‰   | `/tmp/krabkrab/krabkrab-YYYY-MM-DD.log`ï¼ˆæˆ– `logging.file`ï¼‰                                                                                                                                                                                                                                                              |
| Gateway ç½‘å…³æœåŠ¡æ—¥å¿—ï¼ˆç›‘ç®¡ç¨‹åºï¼‰ | macOSï¼š`$krabkrab_STATE_DIR/logs/gateway.log` + `gateway.err.log`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/logs/...`ï¼›é…ç½®æ–‡ä»¶ä½¿ç”¨ `~/.krabkrab-<profile>/logs/...`ï¼‰<br />Linuxï¼š`journalctl --user -u krabkrab-gateway[-<profile>].service -n 200 --no-pager`<br />Windowsï¼š`schtasks /Query /TN "KrabKrab Gateway (<profile>)" /V /FO LIST` |
| ä¼šè¯æ–‡ä»¶                         | `$krabkrab_STATE_DIR/agents/<agentId>/sessions/`                                                                                                                                                                                                                                                                          |
| åª’ä½“ç¼“å­˜                         | `$krabkrab_STATE_DIR/media/`                                                                                                                                                                                                                                                                                              |
| å‡­è¯                             | `$krabkrab_STATE_DIR/credentials/`                                                                                                                                                                                                                                                                                        |

## å¥åº·æ£€æŸ¥

```bash
# ç›‘ç®¡ç¨‹åº + æŽ¢æµ‹ç›®æ ‡ + é…ç½®è·¯å¾„
krabkrab gateway status
# åŒ…æ‹¬ç³»ç»Ÿçº§æ‰«æï¼ˆæ—§ç‰ˆ/é¢å¤–æœåŠ¡ã€ç«¯å£ç›‘å¬å™¨ï¼‰
krabkrab gateway status --deep

# Gateway ç½‘å…³æ˜¯å¦å¯è¾¾ï¼Ÿ
krabkrab health --json
# å¦‚æžœå¤±è´¥ï¼Œä½¿ç”¨è¿žæŽ¥è¯¦æƒ…é‡æ–°è¿è¡Œï¼š
krabkrab health --verbose

# é»˜è®¤ç«¯å£ä¸Šæ˜¯å¦æœ‰ä¸œè¥¿åœ¨ç›‘å¬ï¼Ÿ
lsof -nP -iTCP:18789 -sTCP:LISTEN

# æœ€è¿‘æ´»åŠ¨ï¼ˆRPC æ—¥å¿—å°¾éƒ¨ï¼‰
krabkrab logs --follow
# å¦‚æžœ RPC å®•æœºçš„å¤‡ç”¨æ–¹æ¡ˆ
tail -20 /tmp/krabkrab/krabkrab-*.log
```

## é‡ç½®æ‰€æœ‰å†…å®¹

æ ¸é€‰é¡¹ï¼š

```bash
krabkrab gateway stop
# å¦‚æžœä½ å®‰è£…äº†æœåŠ¡å¹¶æƒ³è¦å¹²å‡€å®‰è£…ï¼š
# krabkrab gateway uninstall

trash "${krabkrab_STATE_DIR:-$HOME/.krabkrab}"
krabkrab channels login         # é‡æ–°é…å¯¹ WhatsApp
krabkrab gateway restart           # æˆ–ï¼škrabkrab gateway
```

âš ï¸ è¿™ä¼šä¸¢å¤±æ‰€æœ‰ä¼šè¯å¹¶éœ€è¦é‡æ–°é…å¯¹ WhatsAppã€‚

## èŽ·å–å¸®åŠ©

1. é¦–å…ˆæ£€æŸ¥æ—¥å¿—ï¼š`/tmp/krabkrab/`ï¼ˆé»˜è®¤ï¼š`krabkrab-YYYY-MM-DD.log`ï¼Œæˆ–ä½ é…ç½®çš„ `logging.file`ï¼‰
2. åœ¨ GitHub ä¸Šæœç´¢çŽ°æœ‰é—®é¢˜
3. æäº¤æ–°é—®é¢˜æ—¶åŒ…å«ï¼š
   - KrabKrab ç‰ˆæœ¬
   - ç›¸å…³æ—¥å¿—ç‰‡æ®µ
   - é‡çŽ°æ­¥éª¤
   - ä½ çš„é…ç½®ï¼ˆéšè—å¯†é’¥ï¼ï¼‰

---

_"ä½ è¯•è¿‡å…³æŽ‰å†å¼€å—ï¼Ÿ"_ â€” æ¯ä¸ª IT äººå‘˜éƒ½è¿™ä¹ˆè¯´

ðŸ¦žðŸ”§

### æµè§ˆå™¨æ— æ³•å¯åŠ¨ï¼ˆLinuxï¼‰

å¦‚æžœä½ çœ‹åˆ° `"Failed to start Chrome CDP on port 18800"`ï¼š

**æœ€å¯èƒ½çš„åŽŸå› ï¼š** Ubuntu ä¸Šçš„ Snap æ‰“åŒ…çš„ Chromiumã€‚

**å¿«é€Ÿä¿®å¤ï¼š** æ”¹ä¸ºå®‰è£… Google Chromeï¼š

```bash
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
sudo dpkg -i google-chrome-stable_current_amd64.deb
```

ç„¶åŽåœ¨é…ç½®ä¸­è®¾ç½®ï¼š

```json
{
  "browser": {
    "executablePath": "/usr/bin/google-chrome-stable"
  }
}
```

**å®Œæ•´æŒ‡å—ï¼š** å‚è§ [browser-linux-troubleshooting](/tools/browser-linux-troubleshooting)

