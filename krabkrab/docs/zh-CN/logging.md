---
read_when:
  - ä½ éœ€è¦ä¸€ä¸ªé€‚åˆåˆå­¦è€…çš„æ—¥å¿—æ¦‚è¿°
  - ä½ æƒ³é…ç½®æ—¥å¿—çº§åˆ«æˆ–æ ¼å¼
  - ä½ æ­£åœ¨æ•…éšœæŽ’é™¤å¹¶éœ€è¦å¿«é€Ÿæ‰¾åˆ°æ—¥å¿—
summary: æ—¥å¿—æ¦‚è¿°ï¼šæ–‡ä»¶æ—¥å¿—ã€æŽ§åˆ¶å°è¾“å‡ºã€CLI è·Ÿè¸ªå’ŒæŽ§åˆ¶ UI
title: æ—¥å¿—
x-i18n:
  generated_at: "2026-02-03T07:50:52Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 884fcf4a906adff34d546908e22abd283cb89fe0845076cf925c72384ec3556b
  source_path: logging.md
  workflow: 15
---

# æ—¥å¿—

KrabKrab åœ¨ä¸¤ä¸ªåœ°æ–¹è®°å½•æ—¥å¿—ï¼š

- **æ–‡ä»¶æ—¥å¿—**ï¼ˆJSON è¡Œï¼‰ç”± Gateway ç½‘å…³å†™å…¥ã€‚
- **æŽ§åˆ¶å°è¾“å‡º**æ˜¾ç¤ºåœ¨ç»ˆç«¯å’ŒæŽ§åˆ¶ UI ä¸­ã€‚

æœ¬é¡µè¯´æ˜Žæ—¥å¿—å­˜æ”¾ä½ç½®ã€å¦‚ä½•è¯»å–æ—¥å¿—ä»¥åŠå¦‚ä½•é…ç½®æ—¥å¿—çº§åˆ«å’Œæ ¼å¼ã€‚

## æ—¥å¿—å­˜æ”¾ä½ç½®

é»˜è®¤æƒ…å†µä¸‹ï¼ŒGateway ç½‘å…³åœ¨ä»¥ä¸‹ä½ç½®å†™å…¥æ»šåŠ¨æ—¥å¿—æ–‡ä»¶ï¼š

`/tmp/krabkrab/krabkrab-YYYY-MM-DD.log`

æ—¥æœŸä½¿ç”¨ Gateway ç½‘å…³ä¸»æœºçš„æœ¬åœ°æ—¶åŒºã€‚

ä½ å¯ä»¥åœ¨ `~/.krabkrab/krabkrab.json` ä¸­è¦†ç›–æ­¤è®¾ç½®ï¼š

```json
{
  "logging": {
    "file": "/path/to/krabkrab.log"
  }
}
```

## å¦‚ä½•è¯»å–æ—¥å¿—

### CLIï¼šå®žæ—¶è·Ÿè¸ªï¼ˆæŽ¨èï¼‰

ä½¿ç”¨ CLI é€šè¿‡ RPC è·Ÿè¸ª Gateway ç½‘å…³æ—¥å¿—æ–‡ä»¶ï¼š

```bash
krabkrab logs --follow
```

è¾“å‡ºæ¨¡å¼ï¼š

- **TTY ä¼šè¯**ï¼šç¾Žè§‚ã€å½©è‰²ã€ç»“æž„åŒ–çš„æ—¥å¿—è¡Œã€‚
- **éž TTY ä¼šè¯**ï¼šçº¯æ–‡æœ¬ã€‚
- `--json`ï¼šè¡Œåˆ†éš”çš„ JSONï¼ˆæ¯è¡Œä¸€ä¸ªæ—¥å¿—äº‹ä»¶ï¼‰ã€‚
- `--plain`ï¼šåœ¨ TTY ä¼šè¯ä¸­å¼ºåˆ¶çº¯æ–‡æœ¬ã€‚
- `--no-color`ï¼šç¦ç”¨ ANSI é¢œè‰²ã€‚

åœ¨ JSON æ¨¡å¼ä¸‹ï¼ŒCLI è¾“å‡ºå¸¦ `type` æ ‡ç­¾çš„å¯¹è±¡ï¼š

- `meta`ï¼šæµå…ƒæ•°æ®ï¼ˆæ–‡ä»¶ã€æ¸¸æ ‡ã€å¤§å°ï¼‰
- `log`ï¼šè§£æžçš„æ—¥å¿—æ¡ç›®
- `notice`ï¼šæˆªæ–­/è½®è½¬æç¤º
- `raw`ï¼šæœªè§£æžçš„æ—¥å¿—è¡Œ

å¦‚æžœ Gateway ç½‘å…³æ— æ³•è®¿é—®ï¼ŒCLI ä¼šæ‰“å°ä¸€ä¸ªç®€çŸ­æç¤ºè¿è¡Œï¼š

```bash
krabkrab doctor
```

### æŽ§åˆ¶ UIï¼ˆWebï¼‰

æŽ§åˆ¶ UI çš„**æ—¥å¿—**æ ‡ç­¾é¡µä½¿ç”¨ `logs.tail` è·Ÿè¸ªç›¸åŒçš„æ–‡ä»¶ã€‚
å‚è§ [/web/control-ui](/web/control-ui) äº†è§£å¦‚ä½•æ‰“å¼€å®ƒã€‚

### ä»…æ¸ é“æ—¥å¿—

è¦è¿‡æ»¤æ¸ é“æ´»åŠ¨ï¼ˆWhatsApp/Telegram ç­‰ï¼‰ï¼Œä½¿ç”¨ï¼š

```bash
krabkrab channels logs --channel whatsapp
```

## æ—¥å¿—æ ¼å¼

### æ–‡ä»¶æ—¥å¿—ï¼ˆJSONLï¼‰

æ—¥å¿—æ–‡ä»¶ä¸­çš„æ¯ä¸€è¡Œéƒ½æ˜¯ä¸€ä¸ª JSON å¯¹è±¡ã€‚CLI å’ŒæŽ§åˆ¶ UI è§£æžè¿™äº›æ¡ç›®ä»¥æ¸²æŸ“ç»“æž„åŒ–è¾“å‡ºï¼ˆæ—¶é—´ã€çº§åˆ«ã€å­ç³»ç»Ÿã€æ¶ˆæ¯ï¼‰ã€‚

### æŽ§åˆ¶å°è¾“å‡º

æŽ§åˆ¶å°æ—¥å¿—**æ„ŸçŸ¥ TTY**å¹¶æ ¼å¼åŒ–ä»¥æé«˜å¯è¯»æ€§ï¼š

- å­ç³»ç»Ÿå‰ç¼€ï¼ˆä¾‹å¦‚ `gateway/channels/whatsapp`ï¼‰
- çº§åˆ«ç€è‰²ï¼ˆinfo/warn/errorï¼‰
- å¯é€‰çš„ç´§å‡‘æˆ– JSON æ¨¡å¼

æŽ§åˆ¶å°æ ¼å¼ç”± `logging.consoleStyle` æŽ§åˆ¶ã€‚

## é…ç½®æ—¥å¿—

æ‰€æœ‰æ—¥å¿—é…ç½®éƒ½åœ¨ `~/.krabkrab/krabkrab.json` çš„ `logging` ä¸‹ã€‚

```json
{
  "logging": {
    "level": "info",
    "file": "/tmp/krabkrab/krabkrab-YYYY-MM-DD.log",
    "consoleLevel": "info",
    "consoleStyle": "pretty",
    "redactSensitive": "tools",
    "redactPatterns": ["sk-.*"]
  }
}
```

### æ—¥å¿—çº§åˆ«

- `logging.level`ï¼š**æ–‡ä»¶æ—¥å¿—**ï¼ˆJSONLï¼‰çº§åˆ«ã€‚
- `logging.consoleLevel`ï¼š**æŽ§åˆ¶å°**è¯¦ç»†ç¨‹åº¦çº§åˆ«ã€‚

`--verbose` ä»…å½±å“æŽ§åˆ¶å°è¾“å‡ºï¼›å®ƒä¸æ”¹å˜æ–‡ä»¶æ—¥å¿—çº§åˆ«ã€‚

### æŽ§åˆ¶å°æ ·å¼

`logging.consoleStyle`ï¼š

- `pretty`ï¼šäººç±»å‹å¥½ã€å½©è‰²ã€å¸¦æ—¶é—´æˆ³ã€‚
- `compact`ï¼šæ›´ç´§å‡‘çš„è¾“å‡ºï¼ˆæœ€é€‚åˆé•¿ä¼šè¯ï¼‰ã€‚
- `json`ï¼šæ¯è¡Œ JSONï¼ˆç”¨äºŽæ—¥å¿—å¤„ç†å™¨ï¼‰ã€‚

### è„±æ•

å·¥å…·æ‘˜è¦å¯ä»¥åœ¨æ•æ„Ÿä»¤ç‰Œè¾“å‡ºåˆ°æŽ§åˆ¶å°ä¹‹å‰å¯¹å…¶è¿›è¡Œè„±æ•ï¼š

- `logging.redactSensitive`ï¼š`off` | `tools`ï¼ˆé»˜è®¤ï¼š`tools`ï¼‰
- `logging.redactPatterns`ï¼šç”¨äºŽè¦†ç›–é»˜è®¤é›†çš„æ­£åˆ™è¡¨è¾¾å¼å­—ç¬¦ä¸²åˆ—è¡¨

è„±æ•ä»…å½±å“**æŽ§åˆ¶å°è¾“å‡º**ï¼Œä¸ä¼šæ”¹å˜æ–‡ä»¶æ—¥å¿—ã€‚

## è¯Šæ–­ + OpenTelemetry

è¯Šæ–­æ˜¯ç”¨äºŽæ¨¡åž‹è¿è¡Œ**å’Œ**æ¶ˆæ¯æµé¥æµ‹ï¼ˆwebhooksã€é˜Ÿåˆ—ã€ä¼šè¯çŠ¶æ€ï¼‰çš„ç»“æž„åŒ–ã€æœºå™¨å¯è¯»äº‹ä»¶ã€‚å®ƒä»¬**ä¸**æ›¿ä»£æ—¥å¿—ï¼›å®ƒä»¬å­˜åœ¨æ˜¯ä¸ºäº†å‘æŒ‡æ ‡ã€è¿½è¸ªå’Œå…¶ä»–å¯¼å‡ºå™¨æä¾›æ•°æ®ã€‚

è¯Šæ–­äº‹ä»¶åœ¨è¿›ç¨‹å†…å‘å‡ºï¼Œä½†å¯¼å‡ºå™¨ä»…åœ¨å¯ç”¨è¯Šæ–­ + å¯¼å‡ºå™¨æ’ä»¶æ—¶æ‰é™„åŠ ã€‚

### OpenTelemetry ä¸Ž OTLP

- **OpenTelemetryï¼ˆOTelï¼‰**ï¼šè¿½è¸ªã€æŒ‡æ ‡å’Œæ—¥å¿—çš„æ•°æ®æ¨¡åž‹ + SDKã€‚
- **OTLP**ï¼šç”¨äºŽå°† OTel æ•°æ®å¯¼å‡ºåˆ°æ”¶é›†å™¨/åŽç«¯çš„çº¿è·¯åè®®ã€‚
- KrabKrab ç›®å‰é€šè¿‡ **OTLP/HTTPï¼ˆprotobufï¼‰** å¯¼å‡ºã€‚

### å¯¼å‡ºçš„ä¿¡å·

- **æŒ‡æ ‡**ï¼šè®¡æ•°å™¨ + ç›´æ–¹å›¾ï¼ˆä»¤ç‰Œä½¿ç”¨ã€æ¶ˆæ¯æµã€é˜Ÿåˆ—ï¼‰ã€‚
- **è¿½è¸ª**ï¼šæ¨¡åž‹ä½¿ç”¨ + webhook/æ¶ˆæ¯å¤„ç†çš„ spanã€‚
- **æ—¥å¿—**ï¼šå¯ç”¨ `diagnostics.otel.logs` æ—¶é€šè¿‡ OTLP å¯¼å‡ºã€‚æ—¥å¿—é‡å¯èƒ½å¾ˆå¤§ï¼›è¯·æ³¨æ„ `logging.level` å’Œå¯¼å‡ºå™¨è¿‡æ»¤å™¨ã€‚

### è¯Šæ–­äº‹ä»¶ç›®å½•

æ¨¡åž‹ä½¿ç”¨ï¼š

- `model.usage`ï¼šä»¤ç‰Œã€æˆæœ¬ã€æŒç»­æ—¶é—´ã€ä¸Šä¸‹æ–‡ã€æä¾›å•†/æ¨¡åž‹/æ¸ é“ã€ä¼šè¯ IDã€‚

æ¶ˆæ¯æµï¼š

- `webhook.received`ï¼šæ¯æ¸ é“çš„ webhook å…¥å£ã€‚
- `webhook.processed`ï¼šwebhook å·²å¤„ç† + æŒç»­æ—¶é—´ã€‚
- `webhook.error`ï¼šwebhook å¤„ç†ç¨‹åºé”™è¯¯ã€‚
- `message.queued`ï¼šæ¶ˆæ¯å…¥é˜Ÿç­‰å¾…å¤„ç†ã€‚
- `message.processed`ï¼šç»“æžœ + æŒç»­æ—¶é—´ + å¯é€‰é”™è¯¯ã€‚

é˜Ÿåˆ— + ä¼šè¯ï¼š

- `queue.lane.enqueue`ï¼šå‘½ä»¤é˜Ÿåˆ—é€šé“å…¥é˜Ÿ + æ·±åº¦ã€‚
- `queue.lane.dequeue`ï¼šå‘½ä»¤é˜Ÿåˆ—é€šé“å‡ºé˜Ÿ + ç­‰å¾…æ—¶é—´ã€‚
- `session.state`ï¼šä¼šè¯çŠ¶æ€è½¬æ¢ + åŽŸå› ã€‚
- `session.stuck`ï¼šä¼šè¯å¡ä½è­¦å‘Š + æŒç»­æ—¶é—´ã€‚
- `run.attempt`ï¼šè¿è¡Œé‡è¯•/å°è¯•å…ƒæ•°æ®ã€‚
- `diagnostic.heartbeat`ï¼šèšåˆè®¡æ•°å™¨ï¼ˆwebhooks/é˜Ÿåˆ—/ä¼šè¯ï¼‰ã€‚

### å¯ç”¨è¯Šæ–­ï¼ˆæ— å¯¼å‡ºå™¨ï¼‰

å¦‚æžœä½ å¸Œæœ›è¯Šæ–­äº‹ä»¶å¯ç”¨äºŽæ’ä»¶æˆ–è‡ªå®šä¹‰æŽ¥æ”¶å™¨ï¼Œä½¿ç”¨æ­¤é…ç½®ï¼š

```json
{
  "diagnostics": {
    "enabled": true
  }
}
```

### è¯Šæ–­æ ‡å¿—ï¼ˆå®šå‘æ—¥å¿—ï¼‰

ä½¿ç”¨æ ‡å¿—åœ¨ä¸æé«˜ `logging.level` çš„æƒ…å†µä¸‹å¼€å¯é¢å¤–çš„å®šå‘è°ƒè¯•æ—¥å¿—ã€‚
æ ‡å¿—ä¸åŒºåˆ†å¤§å°å†™ï¼Œæ”¯æŒé€šé…ç¬¦ï¼ˆä¾‹å¦‚ `telegram.*` æˆ– `*`ï¼‰ã€‚

```json
{
  "diagnostics": {
    "flags": ["telegram.http"]
  }
}
```

çŽ¯å¢ƒå˜é‡è¦†ç›–ï¼ˆä¸€æ¬¡æ€§ï¼‰ï¼š

```
krabkrab_DIAGNOSTICS=telegram.http,telegram.payload
```

æ³¨æ„ï¼š

- æ ‡å¿—æ—¥å¿—è¿›å…¥æ ‡å‡†æ—¥å¿—æ–‡ä»¶ï¼ˆä¸Ž `logging.file` ç›¸åŒï¼‰ã€‚
- è¾“å‡ºä»æ ¹æ® `logging.redactSensitive` è¿›è¡Œè„±æ•ã€‚
- å®Œæ•´æŒ‡å—ï¼š[/diagnostics/flags](/diagnostics/flags)ã€‚

### å¯¼å‡ºåˆ° OpenTelemetry

è¯Šæ–­å¯ä»¥é€šè¿‡ `diagnostics-otel` æ’ä»¶ï¼ˆOTLP/HTTPï¼‰å¯¼å‡ºã€‚è¿™é€‚ç”¨äºŽä»»ä½•æŽ¥å— OTLP/HTTP çš„ OpenTelemetry æ”¶é›†å™¨/åŽç«¯ã€‚

```json
{
  "plugins": {
    "allow": ["diagnostics-otel"],
    "entries": {
      "diagnostics-otel": {
        "enabled": true
      }
    }
  },
  "diagnostics": {
    "enabled": true,
    "otel": {
      "enabled": true,
      "endpoint": "http://otel-collector:4318",
      "protocol": "http/protobuf",
      "serviceName": "krabkrab-gateway",
      "traces": true,
      "metrics": true,
      "logs": true,
      "sampleRate": 0.2,
      "flushIntervalMs": 60000
    }
  }
}
```

æ³¨æ„ï¼š

- ä½ ä¹Ÿå¯ä»¥ä½¿ç”¨ `krabkrab plugins enable diagnostics-otel` å¯ç”¨æ’ä»¶ã€‚
- `protocol` ç›®å‰ä»…æ”¯æŒ `http/protobuf`ã€‚`grpc` è¢«å¿½ç•¥ã€‚
- æŒ‡æ ‡åŒ…æ‹¬ä»¤ç‰Œä½¿ç”¨ã€æˆæœ¬ã€ä¸Šä¸‹æ–‡å¤§å°ã€è¿è¡ŒæŒç»­æ—¶é—´å’Œæ¶ˆæ¯æµè®¡æ•°å™¨/ç›´æ–¹å›¾ï¼ˆwebhooksã€é˜Ÿåˆ—ã€ä¼šè¯çŠ¶æ€ã€é˜Ÿåˆ—æ·±åº¦/ç­‰å¾…ï¼‰ã€‚
- è¿½è¸ª/æŒ‡æ ‡å¯ä»¥é€šè¿‡ `traces` / `metrics` åˆ‡æ¢ï¼ˆé»˜è®¤ï¼šå¼€å¯ï¼‰ã€‚å¯ç”¨æ—¶ï¼Œè¿½è¸ªåŒ…æ‹¬æ¨¡åž‹ä½¿ç”¨ span åŠ ä¸Š webhook/æ¶ˆæ¯å¤„ç† spanã€‚
- å½“ä½ çš„æ”¶é›†å™¨éœ€è¦è®¤è¯æ—¶è®¾ç½® `headers`ã€‚
- æ”¯æŒçš„çŽ¯å¢ƒå˜é‡ï¼š`OTEL_EXPORTER_OTLP_ENDPOINT`ã€`OTEL_SERVICE_NAME`ã€`OTEL_EXPORTER_OTLP_PROTOCOL`ã€‚

### å¯¼å‡ºçš„æŒ‡æ ‡ï¼ˆåç§° + ç±»åž‹ï¼‰

æ¨¡åž‹ä½¿ç”¨ï¼š

- `krabkrab.tokens`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.token`ã€`krabkrab.channel`ã€`krabkrab.provider`ã€`krabkrab.model`ï¼‰
- `krabkrab.cost.usd`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.provider`ã€`krabkrab.model`ï¼‰
- `krabkrab.run.duration_ms`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.provider`ã€`krabkrab.model`ï¼‰
- `krabkrab.context.tokens`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.context`ã€`krabkrab.channel`ã€`krabkrab.provider`ã€`krabkrab.model`ï¼‰

æ¶ˆæ¯æµï¼š

- `krabkrab.webhook.received`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.webhook`ï¼‰
- `krabkrab.webhook.error`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.webhook`ï¼‰
- `krabkrab.webhook.duration_ms`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.webhook`ï¼‰
- `krabkrab.message.queued`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.source`ï¼‰
- `krabkrab.message.processed`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.outcome`ï¼‰
- `krabkrab.message.duration_ms`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.channel`ã€`krabkrab.outcome`ï¼‰

é˜Ÿåˆ— + ä¼šè¯ï¼š

- `krabkrab.queue.lane.enqueue`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.lane`ï¼‰
- `krabkrab.queue.lane.dequeue`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.lane`ï¼‰
- `krabkrab.queue.depth`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.lane` æˆ– `krabkrab.channel=heartbeat`ï¼‰
- `krabkrab.queue.wait_ms`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.lane`ï¼‰
- `krabkrab.session.state`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.state`ã€`krabkrab.reason`ï¼‰
- `krabkrab.session.stuck`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.state`ï¼‰
- `krabkrab.session.stuck_age_ms`ï¼ˆç›´æ–¹å›¾ï¼Œå±žæ€§ï¼š`krabkrab.state`ï¼‰
- `krabkrab.run.attempt`ï¼ˆè®¡æ•°å™¨ï¼Œå±žæ€§ï¼š`krabkrab.attempt`ï¼‰

### å¯¼å‡ºçš„ spanï¼ˆåç§° + å…³é”®å±žæ€§ï¼‰

- `krabkrab.model.usage`
  - `krabkrab.channel`ã€`krabkrab.provider`ã€`krabkrab.model`
  - `krabkrab.sessionKey`ã€`krabkrab.sessionId`
  - `krabkrab.tokens.*`ï¼ˆinput/output/cache_read/cache_write/totalï¼‰
- `krabkrab.webhook.processed`
  - `krabkrab.channel`ã€`krabkrab.webhook`ã€`krabkrab.chatId`
- `krabkrab.webhook.error`
  - `krabkrab.channel`ã€`krabkrab.webhook`ã€`krabkrab.chatId`ã€`krabkrab.error`
- `krabkrab.message.processed`
  - `krabkrab.channel`ã€`krabkrab.outcome`ã€`krabkrab.chatId`ã€`krabkrab.messageId`ã€`krabkrab.sessionKey`ã€`krabkrab.sessionId`ã€`krabkrab.reason`
- `krabkrab.session.stuck`
  - `krabkrab.state`ã€`krabkrab.ageMs`ã€`krabkrab.queueDepth`ã€`krabkrab.sessionKey`ã€`krabkrab.sessionId`

### é‡‡æ · + åˆ·æ–°

- è¿½è¸ªé‡‡æ ·ï¼š`diagnostics.otel.sampleRate`ï¼ˆ0.0â€“1.0ï¼Œä»…æ ¹ spanï¼‰ã€‚
- æŒ‡æ ‡å¯¼å‡ºé—´éš”ï¼š`diagnostics.otel.flushIntervalMs`ï¼ˆæœ€å° 1000msï¼‰ã€‚

### åè®®è¯´æ˜Ž

- OTLP/HTTP ç«¯ç‚¹å¯ä»¥é€šè¿‡ `diagnostics.otel.endpoint` æˆ– `OTEL_EXPORTER_OTLP_ENDPOINT` è®¾ç½®ã€‚
- å¦‚æžœç«¯ç‚¹å·²åŒ…å« `/v1/traces` æˆ– `/v1/metrics`ï¼Œåˆ™æŒ‰åŽŸæ ·ä½¿ç”¨ã€‚
- å¦‚æžœç«¯ç‚¹å·²åŒ…å« `/v1/logs`ï¼Œåˆ™æŒ‰åŽŸæ ·ç”¨äºŽæ—¥å¿—ã€‚
- `diagnostics.otel.logs` ä¸ºä¸»æ—¥å¿—å™¨è¾“å‡ºå¯ç”¨ OTLP æ—¥å¿—å¯¼å‡ºã€‚

### æ—¥å¿—å¯¼å‡ºè¡Œä¸º

- OTLP æ—¥å¿—ä½¿ç”¨ä¸Žå†™å…¥ `logging.file` ç›¸åŒçš„ç»“æž„åŒ–è®°å½•ã€‚
- éµå®ˆ `logging.level`ï¼ˆæ–‡ä»¶æ—¥å¿—çº§åˆ«ï¼‰ã€‚æŽ§åˆ¶å°è„±æ•**ä¸**é€‚ç”¨äºŽ OTLP æ—¥å¿—ã€‚
- é«˜æµé‡å®‰è£…åº”ä¼˜å…ˆä½¿ç”¨ OTLP æ”¶é›†å™¨é‡‡æ ·/è¿‡æ»¤ã€‚

## æ•…éšœæŽ’é™¤æç¤º

- **Gateway ç½‘å…³æ— æ³•è®¿é—®ï¼Ÿ** å…ˆè¿è¡Œ `krabkrab doctor`ã€‚
- **æ—¥å¿—ä¸ºç©ºï¼Ÿ** æ£€æŸ¥ Gateway ç½‘å…³æ˜¯å¦æ­£åœ¨è¿è¡Œå¹¶å†™å…¥ `logging.file` ä¸­çš„æ–‡ä»¶è·¯å¾„ã€‚
- **éœ€è¦æ›´å¤šç»†èŠ‚ï¼Ÿ** å°† `logging.level` è®¾ç½®ä¸º `debug` æˆ– `trace` å¹¶é‡è¯•ã€‚

