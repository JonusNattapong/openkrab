---
read_when:
  - æ·»åŠ æˆ–ä¿®æ”¹é…ç½®å­—æ®µæ—¶
summary: ~/.krabkrab/krabkrab.json çš„æ‰€æœ‰é…ç½®é€‰é¡¹åŠç¤ºä¾‹
title: é…ç½®
x-i18n:
  generated_at: "2026-02-01T21:29:41Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: b5e51290bbc755acb259ad455878625aa894c115e5c0ac6a1a3397e10fff8b4b
  source_path: gateway/configuration.md
  workflow: 15
---

# é…ç½® ðŸ”§

KrabKrab ä»Ž `~/.krabkrab/krabkrab.json` è¯»å–å¯é€‰çš„ **JSON5** é…ç½®ï¼ˆæ”¯æŒæ³¨é‡Šå’Œå°¾é€—å·ï¼‰ã€‚

å¦‚æžœæ–‡ä»¶ä¸å­˜åœ¨ï¼ŒKrabKrab ä½¿ç”¨å®‰å…¨çš„é»˜è®¤å€¼ï¼ˆå†…ç½® Pi æ™ºèƒ½ä½“ + æŒ‰å‘é€è€…åˆ†ä¼šè¯ + å·¥ä½œåŒº `~/.krabkrab/workspace`ï¼‰ã€‚é€šå¸¸åªåœ¨ä»¥ä¸‹æƒ…å†µéœ€è¦é…ç½®ï¼š

- é™åˆ¶è°å¯ä»¥è§¦å‘æœºå™¨äººï¼ˆ`channels.whatsapp.allowFrom`ã€`channels.telegram.allowFrom` ç­‰ï¼‰
- æŽ§åˆ¶ç¾¤ç»„ç™½åå• + æåŠè¡Œä¸ºï¼ˆ`channels.whatsapp.groups`ã€`channels.telegram.groups`ã€`channels.discord.guilds`ã€`agents.list[].groupChat`ï¼‰
- è‡ªå®šä¹‰æ¶ˆæ¯å‰ç¼€ï¼ˆ`messages`ï¼‰
- è®¾ç½®æ™ºèƒ½ä½“å·¥ä½œåŒºï¼ˆ`agents.defaults.workspace` æˆ– `agents.list[].workspace`ï¼‰
- è°ƒæ•´å†…ç½®æ™ºèƒ½ä½“é»˜è®¤å€¼ï¼ˆ`agents.defaults`ï¼‰å’Œä¼šè¯è¡Œä¸ºï¼ˆ`session`ï¼‰
- è®¾ç½®æ¯ä¸ªæ™ºèƒ½ä½“çš„èº«ä»½æ ‡è¯†ï¼ˆ`agents.list[].identity`ï¼‰

> **åˆæ¬¡æŽ¥è§¦é…ç½®ï¼Ÿ** è¯·æŸ¥é˜…[é…ç½®ç¤ºä¾‹](/gateway/configuration-examples)æŒ‡å—ï¼ŒèŽ·å–å¸¦æœ‰è¯¦ç»†è¯´æ˜Žçš„å®Œæ•´ç¤ºä¾‹ï¼

## ä¸¥æ ¼é…ç½®éªŒè¯

KrabKrab åªæŽ¥å—å®Œå…¨åŒ¹é… schema çš„é…ç½®ã€‚
æœªçŸ¥é”®ã€ç±»åž‹é”™è¯¯æˆ–æ— æ•ˆå€¼ä¼šå¯¼è‡´ Gateway ç½‘å…³ **æ‹’ç»å¯åŠ¨**ä»¥ç¡®ä¿å®‰å…¨ã€‚

éªŒè¯å¤±è´¥æ—¶ï¼š

- Gateway ç½‘å…³ä¸ä¼šå¯åŠ¨ã€‚
- åªå…è®¸è¯Šæ–­å‘½ä»¤ï¼ˆä¾‹å¦‚ï¼š`krabkrab doctor`ã€`krabkrab logs`ã€`krabkrab health`ã€`krabkrab status`ã€`krabkrab service`ã€`krabkrab help`ï¼‰ã€‚
- è¿è¡Œ `krabkrab doctor` æŸ¥çœ‹å…·ä½“é—®é¢˜ã€‚
- è¿è¡Œ `krabkrab doctor --fix`ï¼ˆæˆ– `--yes`ï¼‰åº”ç”¨è¿ç§»/ä¿®å¤ã€‚

Doctor ä¸ä¼šå†™å…¥ä»»ä½•æ›´æ”¹ï¼Œé™¤éžä½ æ˜Žç¡®é€‰æ‹©äº† `--fix`/`--yes`ã€‚

## Schema + UI æç¤º

Gateway ç½‘å…³é€šè¿‡ `config.schema` æš´éœ²é…ç½®çš„ JSON Schema è¡¨ç¤ºï¼Œä¾› UI ç¼–è¾‘å™¨ä½¿ç”¨ã€‚
æŽ§åˆ¶å° UI æ ¹æ®æ­¤ schema æ¸²æŸ“è¡¨å•ï¼Œå¹¶æä¾› **Raw JSON** ç¼–è¾‘å™¨ä½œä¸ºåº”æ€¥æ‰‹æ®µã€‚

æ¸ é“æ’ä»¶å’Œæ‰©å±•å¯ä»¥ä¸ºå…¶é…ç½®æ³¨å†Œ schema + UI æç¤ºï¼Œå› æ­¤æ¸ é“è®¾ç½®
åœ¨å„åº”ç”¨é—´ä¿æŒ schema é©±åŠ¨ï¼Œæ— éœ€ç¡¬ç¼–ç è¡¨å•ã€‚

æç¤ºä¿¡æ¯ï¼ˆæ ‡ç­¾ã€åˆ†ç»„ã€æ•æ„Ÿå­—æ®µï¼‰éš schema ä¸€èµ·æä¾›ï¼Œå®¢æˆ·ç«¯æ— éœ€ç¡¬ç¼–ç é…ç½®çŸ¥è¯†å³å¯æ¸²æŸ“æ›´å¥½çš„è¡¨å•ã€‚

## åº”ç”¨ + é‡å¯ï¼ˆRPCï¼‰

ä½¿ç”¨ `config.apply` åœ¨ä¸€æ­¥ä¸­éªŒè¯ + å†™å…¥å®Œæ•´é…ç½®å¹¶é‡å¯ Gateway ç½‘å…³ã€‚
å®ƒä¼šå†™å…¥é‡å¯å“¨å…µæ–‡ä»¶ï¼Œå¹¶åœ¨ Gateway ç½‘å…³æ¢å¤åŽ ping æœ€åŽæ´»è·ƒçš„ä¼šè¯ã€‚

è­¦å‘Šï¼š`config.apply` ä¼šæ›¿æ¢**æ•´ä¸ªé…ç½®**ã€‚å¦‚æžœä½ åªæƒ³æ›´æ”¹éƒ¨åˆ†é”®ï¼Œ
è¯·ä½¿ç”¨ `config.patch` æˆ– `krabkrab config set`ã€‚è¯·å¤‡ä»½ `~/.krabkrab/krabkrab.json`ã€‚

å‚æ•°ï¼š

- `raw`ï¼ˆå­—ç¬¦ä¸²ï¼‰â€” æ•´ä¸ªé…ç½®çš„ JSON5 è´Ÿè½½
- `baseHash`ï¼ˆå¯é€‰ï¼‰â€” æ¥è‡ª `config.get` çš„é…ç½®å“ˆå¸Œï¼ˆå½“é…ç½®å·²å­˜åœ¨æ—¶ä¸ºå¿…éœ€ï¼‰
- `sessionKey`ï¼ˆå¯é€‰ï¼‰â€” æœ€åŽæ´»è·ƒä¼šè¯çš„é”®ï¼Œç”¨äºŽå”¤é†’ ping
- `note`ï¼ˆå¯é€‰ï¼‰â€” åŒ…å«åœ¨é‡å¯å“¨å…µä¸­çš„å¤‡æ³¨
- `restartDelayMs`ï¼ˆå¯é€‰ï¼‰â€” é‡å¯å‰çš„å»¶è¿Ÿï¼ˆé»˜è®¤ 2000ï¼‰

ç¤ºä¾‹ï¼ˆé€šè¿‡ `gateway call`ï¼‰ï¼š

```bash
krabkrab gateway call config.get --params '{}' # capture payload.hash
krabkrab gateway call config.apply --params '{
  "raw": "{\\n  agents: { defaults: { workspace: \\"~/.krabkrab/workspace\\" } }\\n}\\n",
  "baseHash": "<hash-from-config.get>",
  "sessionKey": "agent:main:whatsapp:dm:+15555550123",
  "restartDelayMs": 1000
}'
```

## éƒ¨åˆ†æ›´æ–°ï¼ˆRPCï¼‰

ä½¿ç”¨ `config.patch` å°†éƒ¨åˆ†æ›´æ–°åˆå¹¶åˆ°çŽ°æœ‰é…ç½®ä¸­ï¼Œè€Œä¸ä¼šè¦†ç›–
æ— å…³çš„é”®ã€‚å®ƒé‡‡ç”¨ JSON merge patch è¯­ä¹‰ï¼š

- å¯¹è±¡é€’å½’åˆå¹¶
- `null` åˆ é™¤é”®
- æ•°ç»„æ›¿æ¢
  ä¸Ž `config.apply` ç±»ä¼¼ï¼Œå®ƒä¼šéªŒè¯ã€å†™å…¥é…ç½®ã€å­˜å‚¨é‡å¯å“¨å…µï¼Œå¹¶è°ƒåº¦
  Gateway ç½‘å…³é‡å¯ï¼ˆå½“æä¾› `sessionKey` æ—¶å¯é€‰æ‹©å”¤é†’ï¼‰ã€‚

å‚æ•°ï¼š

- `raw`ï¼ˆå­—ç¬¦ä¸²ï¼‰â€” ä»…åŒ…å«è¦æ›´æ”¹çš„é”®çš„ JSON5 è´Ÿè½½
- `baseHash`ï¼ˆå¿…éœ€ï¼‰â€” æ¥è‡ª `config.get` çš„é…ç½®å“ˆå¸Œ
- `sessionKey`ï¼ˆå¯é€‰ï¼‰â€” æœ€åŽæ´»è·ƒä¼šè¯çš„é”®ï¼Œç”¨äºŽå”¤é†’ ping
- `note`ï¼ˆå¯é€‰ï¼‰â€” åŒ…å«åœ¨é‡å¯å“¨å…µä¸­çš„å¤‡æ³¨
- `restartDelayMs`ï¼ˆå¯é€‰ï¼‰â€” é‡å¯å‰çš„å»¶è¿Ÿï¼ˆé»˜è®¤ 2000ï¼‰

ç¤ºä¾‹ï¼š

```bash
krabkrab gateway call config.get --params '{}' # capture payload.hash
krabkrab gateway call config.patch --params '{
  "raw": "{\\n  channels: { telegram: { groups: { \\"*\\": { requireMention: false } } } }\\n}\\n",
  "baseHash": "<hash-from-config.get>",
  "sessionKey": "agent:main:whatsapp:dm:+15555550123",
  "restartDelayMs": 1000
}'
```

## æœ€å°é…ç½®ï¼ˆæŽ¨èèµ·ç‚¹ï¼‰

```json5
{
  agents: { defaults: { workspace: "~/.krabkrab/workspace" } },
  channels: { whatsapp: { allowFrom: ["+15555550123"] } },
}
```

é¦–æ¬¡æž„å»ºé»˜è®¤é•œåƒï¼š

```bash
scripts/sandbox-setup.sh
```

## è‡ªèŠå¤©æ¨¡å¼ï¼ˆæŽ¨èç”¨äºŽç¾¤ç»„æŽ§åˆ¶ï¼‰

é˜²æ­¢æœºå™¨äººåœ¨ç¾¤ç»„ä¸­å“åº” WhatsApp @æåŠï¼ˆä»…å“åº”ç‰¹å®šæ–‡æœ¬è§¦å‘å™¨ï¼‰ï¼š

```json5
{
  agents: {
    defaults: { workspace: "~/.krabkrab/workspace" },
    list: [
      {
        id: "main",
        groupChat: { mentionPatterns: ["@krabkrab", "reisponde"] },
      },
    ],
  },
  channels: {
    whatsapp: {
      // ç™½åå•ä»…é€‚ç”¨äºŽç§èŠï¼›åŒ…å«ä½ è‡ªå·±çš„å·ç å¯å¯ç”¨è‡ªèŠå¤©æ¨¡å¼ã€‚
      allowFrom: ["+15555550123"],
      groups: { "*": { requireMention: true } },
    },
  },
}
```

## é…ç½®åŒ…å«ï¼ˆ`$include`ï¼‰

ä½¿ç”¨ `$include` æŒ‡ä»¤å°†é…ç½®æ‹†åˆ†ä¸ºå¤šä¸ªæ–‡ä»¶ã€‚é€‚ç”¨äºŽï¼š

- ç»„ç»‡å¤§åž‹é…ç½®ï¼ˆä¾‹å¦‚æŒ‰å®¢æˆ·å®šä¹‰æ™ºèƒ½ä½“ï¼‰
- è·¨çŽ¯å¢ƒå…±äº«é€šç”¨è®¾ç½®
- å°†æ•æ„Ÿé…ç½®å•ç‹¬å­˜æ”¾

### åŸºæœ¬ç”¨æ³•

```json5
// ~/.krabkrab/krabkrab.json
{
  gateway: { port: 18789 },

  // åŒ…å«å•ä¸ªæ–‡ä»¶ï¼ˆæ›¿æ¢è¯¥é”®çš„å€¼ï¼‰
  agents: { $include: "./agents.json5" },

  // åŒ…å«å¤šä¸ªæ–‡ä»¶ï¼ˆæŒ‰é¡ºåºæ·±åº¦åˆå¹¶ï¼‰
  broadcast: {
    $include: ["./clients/mueller.json5", "./clients/schmidt.json5"],
  },
}
```

```json5
// ~/.krabkrab/agents.json5
{
  defaults: { sandbox: { mode: "all", scope: "session" } },
  list: [{ id: "main", workspace: "~/.krabkrab/workspace" }],
}
```

### åˆå¹¶è¡Œä¸º

- **å•ä¸ªæ–‡ä»¶**ï¼šæ›¿æ¢åŒ…å« `$include` çš„å¯¹è±¡
- **æ–‡ä»¶æ•°ç»„**ï¼šæŒ‰é¡ºåºæ·±åº¦åˆå¹¶ï¼ˆåŽé¢çš„æ–‡ä»¶è¦†ç›–å‰é¢çš„ï¼‰
- **å¸¦å…„å¼Ÿé”®**ï¼šå…„å¼Ÿé”®åœ¨åŒ…å«ä¹‹åŽåˆå¹¶ï¼ˆè¦†ç›–è¢«åŒ…å«çš„å€¼ï¼‰
- **å…„å¼Ÿé”® + æ•°ç»„/åŽŸå§‹å€¼**ï¼šä¸æ”¯æŒï¼ˆè¢«åŒ…å«çš„å†…å®¹å¿…é¡»æ˜¯å¯¹è±¡ï¼‰

```json5
// å…„å¼Ÿé”®è¦†ç›–è¢«åŒ…å«çš„å€¼
{
  $include: "./base.json5", // { a: 1, b: 2 }
  b: 99, // ç»“æžœï¼š{ a: 1, b: 99 }
}
```

### åµŒå¥—åŒ…å«

è¢«åŒ…å«çš„æ–‡ä»¶æœ¬èº«å¯ä»¥åŒ…å« `$include` æŒ‡ä»¤ï¼ˆæœ€å¤š 10 å±‚æ·±åº¦ï¼‰ï¼š

```json5
// clients/mueller.json5
{
  agents: { $include: "./mueller/agents.json5" },
  broadcast: { $include: "./mueller/broadcast.json5" },
}
```

### è·¯å¾„è§£æž

- **ç›¸å¯¹è·¯å¾„**ï¼šç›¸å¯¹äºŽåŒ…å«æ–‡ä»¶è§£æž
- **ç»å¯¹è·¯å¾„**ï¼šç›´æŽ¥ä½¿ç”¨
- **çˆ¶ç›®å½•**ï¼š`../` å¼•ç”¨æŒ‰é¢„æœŸå·¥ä½œ

```json5
{ "$include": "./sub/config.json5" }      // ç›¸å¯¹è·¯å¾„
{ "$include": "/etc/krabkrab/base.json5" } // ç»å¯¹è·¯å¾„
{ "$include": "../shared/common.json5" }   // çˆ¶ç›®å½•
```

### é”™è¯¯å¤„ç†

- **æ–‡ä»¶ç¼ºå¤±**ï¼šæ˜¾ç¤ºæ¸…æ™°çš„é”™è¯¯åŠè§£æžåŽçš„è·¯å¾„
- **è§£æžé”™è¯¯**ï¼šæ˜¾ç¤ºå“ªä¸ªè¢«åŒ…å«çš„æ–‡ä»¶å‡ºé”™
- **å¾ªçŽ¯åŒ…å«**ï¼šæ£€æµ‹å¹¶æŠ¥å‘ŠåŒ…å«é“¾

### ç¤ºä¾‹ï¼šå¤šå®¢æˆ·æ³•å¾‹äº‹åŠ¡è®¾ç½®

```json5
// ~/.krabkrab/krabkrab.json
{
  gateway: { port: 18789, auth: { token: "secret" } },

  // é€šç”¨æ™ºèƒ½ä½“é»˜è®¤å€¼
  agents: {
    defaults: {
      sandbox: { mode: "all", scope: "session" },
    },
    // åˆå¹¶æ‰€æœ‰å®¢æˆ·çš„æ™ºèƒ½ä½“åˆ—è¡¨
    list: { $include: ["./clients/mueller/agents.json5", "./clients/schmidt/agents.json5"] },
  },

  // åˆå¹¶å¹¿æ’­é…ç½®
  broadcast: {
    $include: ["./clients/mueller/broadcast.json5", "./clients/schmidt/broadcast.json5"],
  },

  channels: { whatsapp: { groupPolicy: "allowlist" } },
}
```

```json5
// ~/.krabkrab/clients/mueller/agents.json5
[
  { id: "mueller-transcribe", workspace: "~/clients/mueller/transcribe" },
  { id: "mueller-docs", workspace: "~/clients/mueller/docs" },
]
```

```json5
// ~/.krabkrab/clients/mueller/broadcast.json5
{
  "120363403215116621@g.us": ["mueller-transcribe", "mueller-docs"],
}
```

## å¸¸ç”¨é€‰é¡¹

### çŽ¯å¢ƒå˜é‡ + `.env`

KrabKrab ä»Žçˆ¶è¿›ç¨‹ï¼ˆshellã€launchd/systemdã€CI ç­‰ï¼‰è¯»å–çŽ¯å¢ƒå˜é‡ã€‚

æ­¤å¤–ï¼Œå®ƒè¿˜ä¼šåŠ è½½ï¼š

- å½“å‰å·¥ä½œç›®å½•ä¸­çš„ `.env`ï¼ˆå¦‚æžœå­˜åœ¨ï¼‰
- `~/.krabkrab/.env`ï¼ˆå³ `$krabkrab_STATE_DIR/.env`ï¼‰ä½œä¸ºå…¨å±€å›žé€€ `.env`

ä¸¤ä¸ª `.env` æ–‡ä»¶éƒ½ä¸ä¼šè¦†ç›–å·²æœ‰çš„çŽ¯å¢ƒå˜é‡ã€‚

ä½ ä¹Ÿå¯ä»¥åœ¨é…ç½®ä¸­æä¾›å†…è”çŽ¯å¢ƒå˜é‡ã€‚è¿™äº›ä»…åœ¨è¿›ç¨‹çŽ¯å¢ƒä¸­ç¼ºå°‘è¯¥é”®æ—¶åº”ç”¨ï¼ˆç›¸åŒçš„ä¸è¦†ç›–è§„åˆ™ï¼‰ï¼š

```json5
{
  env: {
    OPENROUTER_API_KEY: "sk-or-...",
    vars: {
      GROQ_API_KEY: "gsk-...",
    },
  },
}
```

å‚è§ [/environment](/help/environment) äº†è§£ä¼˜å…ˆçº§å’Œæ¥æºè¯¦æƒ…ã€‚

### `env.shellEnv`ï¼ˆå¯é€‰ï¼‰

å¯é€‰ä¾¿åˆ©åŠŸèƒ½ï¼šå¦‚æžœå¯ç”¨ä¸”é¢„æœŸé”®å‡æœªè®¾ç½®ï¼ŒKrabKrab ä¼šè¿è¡Œä½ çš„ç™»å½• shell å¹¶ä»…å¯¼å…¥ç¼ºå¤±çš„é¢„æœŸé”®ï¼ˆä¸ä¼šè¦†ç›–ï¼‰ã€‚
è¿™å®žé™…ä¸Šä¼š source ä½ çš„ shell é…ç½®æ–‡ä»¶ã€‚

```json5
{
  env: {
    shellEnv: {
      enabled: true,
      timeoutMs: 15000,
    },
  },
}
```

ç­‰æ•ˆçŽ¯å¢ƒå˜é‡ï¼š

- `krabkrab_LOAD_SHELL_ENV=1`
- `krabkrab_SHELL_ENV_TIMEOUT_MS=15000`

### é…ç½®ä¸­çš„çŽ¯å¢ƒå˜é‡æ›¿æ¢

ä½ å¯ä»¥åœ¨ä»»ä½•é…ç½®å­—ç¬¦ä¸²å€¼ä¸­ä½¿ç”¨ `${VAR_NAME}` è¯­æ³•ç›´æŽ¥å¼•ç”¨çŽ¯å¢ƒå˜é‡ã€‚å˜é‡åœ¨é…ç½®åŠ è½½æ—¶ã€éªŒè¯ä¹‹å‰è¿›è¡Œæ›¿æ¢ã€‚

```json5
{
  models: {
    providers: {
      "vercel-gateway": {
        apiKey: "${VERCEL_GATEWAY_API_KEY}",
      },
    },
  },
  gateway: {
    auth: {
      token: "${krabkrab_GATEWAY_TOKEN}",
    },
  },
}
```

**è§„åˆ™ï¼š**

- ä»…åŒ¹é…å¤§å†™çŽ¯å¢ƒå˜é‡åï¼š`[A-Z_][A-Z0-9_]*`
- ç¼ºå¤±æˆ–ä¸ºç©ºçš„çŽ¯å¢ƒå˜é‡åœ¨é…ç½®åŠ è½½æ—¶ä¼šæŠ›å‡ºé”™è¯¯
- ä½¿ç”¨ `$${VAR}` è½¬ä¹‰ä»¥è¾“å‡ºå­—é¢é‡ `${VAR}`
- ä¸Ž `$include` é…åˆä½¿ç”¨ï¼ˆè¢«åŒ…å«çš„æ–‡ä»¶ä¹Ÿä¼šè¿›è¡Œæ›¿æ¢ï¼‰

**å†…è”æ›¿æ¢ï¼š**

```json5
{
  models: {
    providers: {
      custom: {
        baseUrl: "${CUSTOM_API_BASE}/v1", // â†’ "https://api.example.com/v1"
      },
    },
  },
}
```

### è®¤è¯å­˜å‚¨ï¼ˆOAuth + API å¯†é’¥ï¼‰

KrabKrab åœ¨ä»¥ä¸‹ä½ç½®å­˜å‚¨**æ¯ä¸ªæ™ºèƒ½ä½“çš„**è®¤è¯é…ç½®æ–‡ä»¶ï¼ˆOAuth + API å¯†é’¥ï¼‰ï¼š

- `<agentDir>/auth-profiles.json`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/agents/<agentId>/agent/auth-profiles.json`ï¼‰

å¦è¯·å‚é˜…ï¼š[/concepts/oauth](/concepts/oauth)

æ—§ç‰ˆ OAuth å¯¼å…¥ï¼š

- `~/.krabkrab/credentials/oauth.json`ï¼ˆæˆ– `$krabkrab_STATE_DIR/credentials/oauth.json`ï¼‰

å†…ç½® Pi æ™ºèƒ½ä½“åœ¨ä»¥ä¸‹ä½ç½®ç»´æŠ¤è¿è¡Œæ—¶ç¼“å­˜ï¼š

- `<agentDir>/auth.json`ï¼ˆè‡ªåŠ¨ç®¡ç†ï¼›è¯·å‹¿æ‰‹åŠ¨ç¼–è¾‘ï¼‰

æ—§ç‰ˆæ™ºèƒ½ä½“ç›®å½•ï¼ˆå¤šæ™ºèƒ½ä½“ä¹‹å‰ï¼‰ï¼š

- `~/.krabkrab/agent/*`ï¼ˆç”± `krabkrab doctor` è¿ç§»åˆ° `~/.krabkrab/agents/<defaultAgentId>/agent/*`ï¼‰

è¦†ç›–ï¼š

- OAuth ç›®å½•ï¼ˆä»…æ—§ç‰ˆå¯¼å…¥ï¼‰ï¼š`krabkrab_OAUTH_DIR`
- æ™ºèƒ½ä½“ç›®å½•ï¼ˆé»˜è®¤æ™ºèƒ½ä½“æ ¹ç›®å½•è¦†ç›–ï¼‰ï¼š`krabkrab_AGENT_DIR`ï¼ˆæŽ¨èï¼‰ã€`PI_CODING_AGENT_DIR`ï¼ˆæ—§ç‰ˆï¼‰

é¦–æ¬¡ä½¿ç”¨æ—¶ï¼ŒKrabKrab ä¼šå°† `oauth.json` æ¡ç›®å¯¼å…¥åˆ° `auth-profiles.json` ä¸­ã€‚

### `auth`

è®¤è¯é…ç½®æ–‡ä»¶çš„å¯é€‰å…ƒæ•°æ®ã€‚è¿™**ä¸**å­˜å‚¨å¯†é’¥ï¼›å®ƒå°†é…ç½®æ–‡ä»¶ ID æ˜ å°„åˆ°æä¾›å•† + æ¨¡å¼ï¼ˆä»¥åŠå¯é€‰çš„é‚®ç®±ï¼‰ï¼Œå¹¶å®šä¹‰ç”¨äºŽæ•…éšœè½¬ç§»çš„æä¾›å•†è½®æ¢é¡ºåºã€‚

```json5
{
  auth: {
    profiles: {
      "anthropic:me@example.com": { provider: "anthropic", mode: "oauth", email: "me@example.com" },
      "anthropic:work": { provider: "anthropic", mode: "api_key" },
    },
    order: {
      anthropic: ["anthropic:me@example.com", "anthropic:work"],
    },
  },
}
```

### `agents.list[].identity`

ç”¨äºŽé»˜è®¤å€¼å’Œç”¨æˆ·ä½“éªŒçš„å¯é€‰æ¯æ™ºèƒ½ä½“èº«ä»½æ ‡è¯†ã€‚ç”± macOS æ–°æ‰‹å¼•å¯¼åŠ©æ‰‹å†™å…¥ã€‚

å¦‚æžœè®¾ç½®äº†ï¼ŒKrabKrab ä¼šæŽ¨å¯¼é»˜è®¤å€¼ï¼ˆä»…åœ¨ä½ æœªæ˜Žç¡®è®¾ç½®æ—¶ï¼‰ï¼š

- `messages.ackReaction` æ¥è‡ª**æ´»è·ƒæ™ºèƒ½ä½“**çš„ `identity.emoji`ï¼ˆå›žé€€åˆ° ðŸ‘€ï¼‰
- `agents.list[].groupChat.mentionPatterns` æ¥è‡ªæ™ºèƒ½ä½“çš„ `identity.name`/`identity.emoji`ï¼ˆå› æ­¤ "@Samantha" åœ¨ Telegram/Slack/Discord/Google Chat/iMessage/WhatsApp çš„ç¾¤ç»„ä¸­å‡å¯ä½¿ç”¨ï¼‰
- `identity.avatar` æŽ¥å—å·¥ä½œåŒºç›¸å¯¹å›¾ç‰‡è·¯å¾„æˆ–è¿œç¨‹ URL/data URLã€‚æœ¬åœ°æ–‡ä»¶å¿…é¡»ä½äºŽæ™ºèƒ½ä½“å·¥ä½œåŒºå†…ã€‚

`identity.avatar` æŽ¥å—ï¼š

- å·¥ä½œåŒºç›¸å¯¹è·¯å¾„ï¼ˆå¿…é¡»åœ¨æ™ºèƒ½ä½“å·¥ä½œåŒºå†…ï¼‰
- `http(s)` URL
- `data:` URI

```json5
{
  agents: {
    list: [
      {
        id: "main",
        identity: {
          name: "Samantha",
          theme: "helpful sloth",
          emoji: "ðŸ¦¥",
          avatar: "avatars/samantha.png",
        },
      },
    ],
  },
}
```

### `wizard`

ç”± CLI å‘å¯¼ï¼ˆ`onboard`ã€`configure`ã€`doctor`ï¼‰å†™å…¥çš„å…ƒæ•°æ®ã€‚

```json5
{
  wizard: {
    lastRunAt: "2026-01-01T00:00:00.000Z",
    lastRunVersion: "2026.1.4",
    lastRunCommit: "abc1234",
    lastRunCommand: "configure",
    lastRunMode: "local",
  },
}
```

### `logging`

- é»˜è®¤æ—¥å¿—æ–‡ä»¶ï¼š`/tmp/krabkrab/krabkrab-YYYY-MM-DD.log`
- å¦‚éœ€ç¨³å®šè·¯å¾„ï¼Œå°† `logging.file` è®¾ä¸º `/tmp/krabkrab/krabkrab.log`ã€‚
- æŽ§åˆ¶å°è¾“å‡ºå¯é€šè¿‡ä»¥ä¸‹æ–¹å¼å•ç‹¬è°ƒæ•´ï¼š
  - `logging.consoleLevel`ï¼ˆé»˜è®¤ `info`ï¼Œä½¿ç”¨ `--verbose` æ—¶æå‡ä¸º `debug`ï¼‰
  - `logging.consoleStyle`ï¼ˆ`pretty` | `compact` | `json`ï¼‰
- å·¥å…·æ‘˜è¦å¯ä»¥è„±æ•ä»¥é¿å…æ³„éœ²å¯†é’¥ï¼š
  - `logging.redactSensitive`ï¼ˆ`off` | `tools`ï¼Œé»˜è®¤ï¼š`tools`ï¼‰
  - `logging.redactPatterns`ï¼ˆæ­£åˆ™è¡¨è¾¾å¼å­—ç¬¦ä¸²æ•°ç»„ï¼›è¦†ç›–é»˜è®¤å€¼ï¼‰

```json5
{
  logging: {
    level: "info",
    file: "/tmp/krabkrab/krabkrab.log",
    consoleLevel: "info",
    consoleStyle: "pretty",
    redactSensitive: "tools",
    redactPatterns: [
      // ç¤ºä¾‹ï¼šç”¨è‡ªå®šä¹‰è§„åˆ™è¦†ç›–é»˜è®¤å€¼ã€‚
      "\\bTOKEN\\b\\s*[=:]\\s*([\"']?)([^\\s\"']+)\\1",
      "/\\bsk-[A-Za-z0-9_-]{8,}\\b/gi",
    ],
  },
}
```

### `channels.whatsapp.dmPolicy`

æŽ§åˆ¶ WhatsApp ç§èŠï¼ˆç§ä¿¡ï¼‰çš„å¤„ç†æ–¹å¼ï¼š

- `"pairing"`ï¼ˆé»˜è®¤ï¼‰ï¼šæœªçŸ¥å‘é€è€…ä¼šæ”¶åˆ°é…å¯¹ç ï¼›æ‰€æœ‰è€…å¿…é¡»æ‰¹å‡†
- `"allowlist"`ï¼šä»…å…è®¸ `channels.whatsapp.allowFrom`ï¼ˆæˆ–å·²é…å¯¹çš„å…è®¸å­˜å‚¨ï¼‰ä¸­çš„å‘é€è€…
- `"open"`ï¼šå…è®¸æ‰€æœ‰å…¥ç«™ç§èŠï¼ˆ**éœ€è¦** `channels.whatsapp.allowFrom` åŒ…å« `"*"`ï¼‰
- `"disabled"`ï¼šå¿½ç•¥æ‰€æœ‰å…¥ç«™ç§èŠ

é…å¯¹ç åœ¨ 1 å°æ—¶åŽè¿‡æœŸï¼›æœºå™¨äººä»…åœ¨åˆ›å»ºæ–°è¯·æ±‚æ—¶å‘é€é…å¯¹ç ã€‚å¾…å¤„ç†çš„ç§èŠé…å¯¹è¯·æ±‚é»˜è®¤æ¯ä¸ªæ¸ é“ä¸Šé™ä¸º **3 ä¸ª**ã€‚

é…å¯¹æ‰¹å‡†ï¼š

- `krabkrab pairing list whatsapp`
- `krabkrab pairing approve whatsapp <code>`

### `channels.whatsapp.allowFrom`

å…è®¸è§¦å‘ WhatsApp è‡ªåŠ¨å›žå¤çš„ E.164 ç”µè¯å·ç ç™½åå•ï¼ˆ**ä»…é™ç§èŠ**ï¼‰ã€‚
å¦‚æžœä¸ºç©ºä¸” `channels.whatsapp.dmPolicy="pairing"`ï¼ŒæœªçŸ¥å‘é€è€…å°†æ”¶åˆ°é…å¯¹ç ã€‚
å¯¹äºŽç¾¤ç»„ï¼Œä½¿ç”¨ `channels.whatsapp.groupPolicy` + `channels.whatsapp.groupAllowFrom`ã€‚

```json5
{
  channels: {
    whatsapp: {
      dmPolicy: "pairing", // pairing | allowlist | open | disabled
      allowFrom: ["+15555550123", "+447700900123"],
      textChunkLimit: 4000, // å¯é€‰çš„å‡ºç«™åˆ†å—å¤§å°ï¼ˆå­—ç¬¦æ•°ï¼‰
      chunkMode: "length", // å¯é€‰çš„åˆ†å—æ¨¡å¼ï¼ˆlength | newlineï¼‰
      mediaMaxMb: 50, // å¯é€‰çš„å…¥ç«™åª’ä½“ä¸Šé™ï¼ˆMBï¼‰
    },
  },
}
```

### `channels.whatsapp.sendReadReceipts`

æŽ§åˆ¶å…¥ç«™ WhatsApp æ¶ˆæ¯æ˜¯å¦æ ‡è®°ä¸ºå·²è¯»ï¼ˆè“è‰²åŒå‹¾ï¼‰ã€‚é»˜è®¤ï¼š`true`ã€‚

è‡ªèŠå¤©æ¨¡å¼å§‹ç»ˆè·³è¿‡å·²è¯»å›žæ‰§ï¼Œå³ä½¿å·²å¯ç”¨ã€‚

æ¯è´¦å·è¦†ç›–ï¼š`channels.whatsapp.accounts.<id>.sendReadReceipts`ã€‚

```json5
{
  channels: {
    whatsapp: { sendReadReceipts: false },
  },
}
```

### `channels.whatsapp.accounts`ï¼ˆå¤šè´¦å·ï¼‰

åœ¨ä¸€ä¸ª Gateway ç½‘å…³ä¸­è¿è¡Œå¤šä¸ª WhatsApp è´¦å·ï¼š

```json5
{
  channels: {
    whatsapp: {
      accounts: {
        default: {}, // å¯é€‰ï¼›ä¿æŒé»˜è®¤ id ç¨³å®š
        personal: {},
        biz: {
          // å¯é€‰è¦†ç›–ã€‚é»˜è®¤ï¼š~/.krabkrab/credentials/whatsapp/biz
          // authDir: "~/.krabkrab/credentials/whatsapp/biz",
        },
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- å‡ºç«™å‘½ä»¤é»˜è®¤ä½¿ç”¨ `default` è´¦å·ï¼ˆå¦‚æžœå­˜åœ¨ï¼‰ï¼›å¦åˆ™ä½¿ç”¨ç¬¬ä¸€ä¸ªé…ç½®çš„è´¦å· idï¼ˆæŽ’åºåŽï¼‰ã€‚
- æ—§ç‰ˆå•è´¦å· Baileys è®¤è¯ç›®å½•ç”± `krabkrab doctor` è¿ç§»åˆ° `whatsapp/default`ã€‚

### `channels.telegram.accounts` / `channels.discord.accounts` / `channels.googlechat.accounts` / `channels.slack.accounts` / `channels.mattermost.accounts` / `channels.signal.accounts` / `channels.imessage.accounts`

æ¯ä¸ªæ¸ é“è¿è¡Œå¤šä¸ªè´¦å·ï¼ˆæ¯ä¸ªè´¦å·æœ‰è‡ªå·±çš„ `accountId` å’Œå¯é€‰çš„ `name`ï¼‰ï¼š

```json5
{
  channels: {
    telegram: {
      accounts: {
        default: {
          name: "Primary bot",
          botToken: "123456:ABC...",
        },
        alerts: {
          name: "Alerts bot",
          botToken: "987654:XYZ...",
        },
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- çœç•¥ `accountId` æ—¶ä½¿ç”¨ `default`ï¼ˆCLI + è·¯ç”±ï¼‰ã€‚
- çŽ¯å¢ƒå˜é‡ token ä»…é€‚ç”¨äºŽ**é»˜è®¤**è´¦å·ã€‚
- åŸºç¡€æ¸ é“è®¾ç½®ï¼ˆç¾¤ç»„ç­–ç•¥ã€æåŠé—¨æŽ§ç­‰ï¼‰é€‚ç”¨äºŽæ‰€æœ‰è´¦å·ï¼Œé™¤éžåœ¨æ¯ä¸ªè´¦å·ä¸­å•ç‹¬è¦†ç›–ã€‚
- ä½¿ç”¨ `bindings[].match.accountId` å°†æ¯ä¸ªè´¦å·è·¯ç”±åˆ°ä¸åŒçš„ agents.defaultsã€‚

### ç¾¤èŠæåŠé—¨æŽ§ï¼ˆ`agents.list[].groupChat` + `messages.groupChat`ï¼‰

ç¾¤æ¶ˆæ¯é»˜è®¤**éœ€è¦æåŠ**ï¼ˆå…ƒæ•°æ®æåŠæˆ–æ­£åˆ™æ¨¡å¼ï¼‰ã€‚é€‚ç”¨äºŽ WhatsAppã€Telegramã€Discordã€Google Chat å’Œ iMessage ç¾¤èŠã€‚

**æåŠç±»åž‹ï¼š**

- **å…ƒæ•°æ®æåŠ**ï¼šåŽŸç”Ÿå¹³å° @æåŠï¼ˆä¾‹å¦‚ WhatsApp ç‚¹æŒ‰æåŠï¼‰ã€‚åœ¨ WhatsApp è‡ªèŠå¤©æ¨¡å¼ä¸­è¢«å¿½ç•¥ï¼ˆå‚è§ `channels.whatsapp.allowFrom`ï¼‰ã€‚
- **æ–‡æœ¬æ¨¡å¼**ï¼šåœ¨ `agents.list[].groupChat.mentionPatterns` ä¸­å®šä¹‰çš„æ­£åˆ™æ¨¡å¼ã€‚æ— è®ºè‡ªèŠå¤©æ¨¡å¼å¦‚ä½•å§‹ç»ˆæ£€æŸ¥ã€‚
- æåŠé—¨æŽ§ä»…åœ¨å¯ä»¥æ£€æµ‹æåŠæ—¶æ‰§è¡Œï¼ˆåŽŸç”ŸæåŠæˆ–è‡³å°‘ä¸€ä¸ª `mentionPattern`ï¼‰ã€‚

```json5
{
  messages: {
    groupChat: { historyLimit: 50 },
  },
  agents: {
    list: [{ id: "main", groupChat: { mentionPatterns: ["@krabkrab", "krabkrab"] } }],
  },
}
```

`messages.groupChat.historyLimit` è®¾ç½®ç¾¤ç»„åŽ†å²ä¸Šä¸‹æ–‡çš„å…¨å±€é»˜è®¤å€¼ã€‚æ¸ é“å¯ä»¥é€šè¿‡ `channels.<channel>.historyLimit`ï¼ˆæˆ–å¤šè´¦å·çš„ `channels.<channel>.accounts.*.historyLimit`ï¼‰è¦†ç›–ã€‚è®¾ä¸º `0` ç¦ç”¨åŽ†å²åŒ…è£…ã€‚

#### ç§èŠåŽ†å²é™åˆ¶

ç§èŠå¯¹è¯ä½¿ç”¨ç”±æ™ºèƒ½ä½“ç®¡ç†çš„åŸºäºŽä¼šè¯çš„åŽ†å²ã€‚ä½ å¯ä»¥é™åˆ¶æ¯ä¸ªç§èŠä¼šè¯ä¿ç•™çš„ç”¨æˆ·è½®æ¬¡æ•°ï¼š

```json5
{
  channels: {
    telegram: {
      dmHistoryLimit: 30, // å°†ç§èŠä¼šè¯é™åˆ¶ä¸º 30 ä¸ªç”¨æˆ·è½®æ¬¡
      dms: {
        "123456789": { historyLimit: 50 }, // æ¯ç”¨æˆ·è¦†ç›–ï¼ˆç”¨æˆ· IDï¼‰
      },
    },
  },
}
```

è§£æžé¡ºåºï¼š

1. æ¯ç§èŠè¦†ç›–ï¼š`channels.<provider>.dms[userId].historyLimit`
2. æä¾›å•†é»˜è®¤å€¼ï¼š`channels.<provider>.dmHistoryLimit`
3. æ— é™åˆ¶ï¼ˆä¿ç•™æ‰€æœ‰åŽ†å²ï¼‰

æ”¯æŒçš„æä¾›å•†ï¼š`telegram`ã€`whatsapp`ã€`discord`ã€`slack`ã€`signal`ã€`imessage`ã€`msteams`ã€‚

æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼ˆè®¾ç½®åŽä¼˜å…ˆï¼Œå³ä½¿ä¸º `[]`ï¼‰ï¼š

```json5
{
  agents: {
    list: [
      { id: "work", groupChat: { mentionPatterns: ["@workbot", "\\+15555550123"] } },
      { id: "personal", groupChat: { mentionPatterns: ["@homebot", "\\+15555550999"] } },
    ],
  },
}
```

æåŠé—¨æŽ§é»˜è®¤å€¼æŒ‰æ¸ é“è®¾ç½®ï¼ˆ`channels.whatsapp.groups`ã€`channels.telegram.groups`ã€`channels.imessage.groups`ã€`channels.discord.guilds`ï¼‰ã€‚å½“è®¾ç½®äº† `*.groups` æ—¶ï¼Œå®ƒä¹Ÿå……å½“ç¾¤ç»„ç™½åå•ï¼›åŒ…å« `"*"` ä»¥å…è®¸æ‰€æœ‰ç¾¤ç»„ã€‚

ä»…å“åº”ç‰¹å®šæ–‡æœ¬è§¦å‘å™¨ï¼ˆå¿½ç•¥åŽŸç”Ÿ @æåŠï¼‰ï¼š

```json5
{
  channels: {
    whatsapp: {
      // åŒ…å«ä½ è‡ªå·±çš„å·ç ä»¥å¯ç”¨è‡ªèŠå¤©æ¨¡å¼ï¼ˆå¿½ç•¥åŽŸç”Ÿ @æåŠï¼‰ã€‚
      allowFrom: ["+15555550123"],
      groups: { "*": { requireMention: true } },
    },
  },
  agents: {
    list: [
      {
        id: "main",
        groupChat: {
          // ä»…è¿™äº›æ–‡æœ¬æ¨¡å¼ä¼šè§¦å‘å“åº”
          mentionPatterns: ["reisponde", "@krabkrab"],
        },
      },
    ],
  },
}
```

### ç¾¤ç»„ç­–ç•¥ï¼ˆæŒ‰æ¸ é“ï¼‰

ä½¿ç”¨ `channels.*.groupPolicy` æŽ§åˆ¶æ˜¯å¦æŽ¥å—ç¾¤ç»„/æˆ¿é—´æ¶ˆæ¯ï¼š

```json5
{
  channels: {
    whatsapp: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["+15551234567"],
    },
    telegram: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["tg:123456789", "@alice"],
    },
    signal: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["+15551234567"],
    },
    imessage: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["chat_id:123"],
    },
    msteams: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["user@org.com"],
    },
    discord: {
      groupPolicy: "allowlist",
      guilds: {
        GUILD_ID: {
          channels: { help: { allow: true } },
        },
      },
    },
    slack: {
      groupPolicy: "allowlist",
      channels: { "#general": { allow: true } },
    },
  },
}
```

è¯´æ˜Žï¼š

- `"open"`ï¼šç¾¤ç»„ç»•è¿‡ç™½åå•ï¼›æåŠé—¨æŽ§ä»ç„¶é€‚ç”¨ã€‚
- `"disabled"`ï¼šé˜»æ­¢æ‰€æœ‰ç¾¤ç»„/æˆ¿é—´æ¶ˆæ¯ã€‚
- `"allowlist"`ï¼šä»…å…è®¸åŒ¹é…é…ç½®ç™½åå•çš„ç¾¤ç»„/æˆ¿é—´ã€‚
- `channels.defaults.groupPolicy` è®¾ç½®æä¾›å•†çš„ `groupPolicy` æœªè®¾ç½®æ—¶çš„é»˜è®¤å€¼ã€‚
- WhatsApp/Telegram/Signal/iMessage/Microsoft Teams ä½¿ç”¨ `groupAllowFrom`ï¼ˆå›žé€€ï¼šæ˜¾å¼ `allowFrom`ï¼‰ã€‚
- Discord/Slack ä½¿ç”¨æ¸ é“ç™½åå•ï¼ˆ`channels.discord.guilds.*.channels`ã€`channels.slack.channels`ï¼‰ã€‚
- ç¾¤ç»„ç§èŠï¼ˆDiscord/Slackï¼‰ä»ç”± `dm.groupEnabled` + `dm.groupChannels` æŽ§åˆ¶ã€‚
- é»˜è®¤ä¸º `groupPolicy: "allowlist"`ï¼ˆé™¤éžè¢« `channels.defaults.groupPolicy` è¦†ç›–ï¼‰ï¼›å¦‚æžœæœªé…ç½®ç™½åå•ï¼Œç¾¤ç»„æ¶ˆæ¯å°†è¢«é˜»æ­¢ã€‚

### å¤šæ™ºèƒ½ä½“è·¯ç”±ï¼ˆ`agents.list` + `bindings`ï¼‰

åœ¨ä¸€ä¸ª Gateway ç½‘å…³ä¸­è¿è¡Œå¤šä¸ªéš”ç¦»çš„æ™ºèƒ½ä½“ï¼ˆç‹¬ç«‹çš„å·¥ä½œåŒºã€`agentDir`ã€ä¼šè¯ï¼‰ã€‚
å…¥ç«™æ¶ˆæ¯é€šè¿‡ç»‘å®šè·¯ç”±åˆ°æ™ºèƒ½ä½“ã€‚

- `agents.list[]`ï¼šæ¯æ™ºèƒ½ä½“è¦†ç›–ã€‚
  - `id`ï¼šç¨³å®šçš„æ™ºèƒ½ä½“ idï¼ˆå¿…éœ€ï¼‰ã€‚
  - `default`ï¼šå¯é€‰ï¼›å½“è®¾ç½®å¤šä¸ªæ—¶ï¼Œç¬¬ä¸€ä¸ªèŽ·èƒœå¹¶è®°å½•è­¦å‘Šã€‚
    å¦‚æžœæœªè®¾ç½®ï¼Œåˆ—è¡¨ä¸­çš„**ç¬¬ä¸€ä¸ªæ¡ç›®**ä¸ºé»˜è®¤æ™ºèƒ½ä½“ã€‚
  - `name`ï¼šæ™ºèƒ½ä½“çš„æ˜¾ç¤ºåç§°ã€‚
  - `workspace`ï¼šé»˜è®¤ `~/.krabkrab/workspace-<agentId>`ï¼ˆå¯¹äºŽ `main`ï¼Œå›žé€€åˆ° `agents.defaults.workspace`ï¼‰ã€‚
  - `agentDir`ï¼šé»˜è®¤ `~/.krabkrab/agents/<agentId>/agent`ã€‚
  - `model`ï¼šæ¯æ™ºèƒ½ä½“é»˜è®¤æ¨¡åž‹ï¼Œè¦†ç›–è¯¥æ™ºèƒ½ä½“çš„ `agents.defaults.model`ã€‚
    - å­—ç¬¦ä¸²å½¢å¼ï¼š`"provider/model"`ï¼Œä»…è¦†ç›– `agents.defaults.model.primary`
    - å¯¹è±¡å½¢å¼ï¼š`{ primary, fallbacks }`ï¼ˆfallbacks è¦†ç›– `agents.defaults.model.fallbacks`ï¼›`[]` ä¸ºè¯¥æ™ºèƒ½ä½“ç¦ç”¨å…¨å±€å›žé€€ï¼‰
  - `identity`ï¼šæ¯æ™ºèƒ½ä½“çš„åç§°/ä¸»é¢˜/è¡¨æƒ…ï¼ˆç”¨äºŽæåŠæ¨¡å¼ + ç¡®è®¤ååº”ï¼‰ã€‚
  - `groupChat`ï¼šæ¯æ™ºèƒ½ä½“çš„æåŠé—¨æŽ§ï¼ˆ`mentionPatterns`ï¼‰ã€‚
  - `sandbox`ï¼šæ¯æ™ºèƒ½ä½“çš„æ²™ç®±é…ç½®ï¼ˆè¦†ç›– `agents.defaults.sandbox`ï¼‰ã€‚
    - `mode`ï¼š`"off"` | `"non-main"` | `"all"`
    - `workspaceAccess`ï¼š`"none"` | `"ro"` | `"rw"`
    - `scope`ï¼š`"session"` | `"agent"` | `"shared"`
    - `workspaceRoot`ï¼šè‡ªå®šä¹‰æ²™ç®±å·¥ä½œåŒºæ ¹ç›®å½•
    - `docker`ï¼šæ¯æ™ºèƒ½ä½“ docker è¦†ç›–ï¼ˆä¾‹å¦‚ `image`ã€`network`ã€`env`ã€`setupCommand`ã€é™åˆ¶ï¼›`scope: "shared"` æ—¶å¿½ç•¥ï¼‰
    - `browser`ï¼šæ¯æ™ºèƒ½ä½“æ²™ç®±æµè§ˆå™¨è¦†ç›–ï¼ˆ`scope: "shared"` æ—¶å¿½ç•¥ï¼‰
    - `prune`ï¼šæ¯æ™ºèƒ½ä½“æ²™ç®±æ¸…ç†è¦†ç›–ï¼ˆ`scope: "shared"` æ—¶å¿½ç•¥ï¼‰
  - `subagents`ï¼šæ¯æ™ºèƒ½ä½“å­æ™ºèƒ½ä½“é»˜è®¤å€¼ã€‚
    - `allowAgents`ï¼šå…è®¸ä»Žæ­¤æ™ºèƒ½ä½“æ‰§è¡Œ `sessions_spawn` çš„æ™ºèƒ½ä½“ id ç™½åå•ï¼ˆ`["*"]` = å…è®¸ä»»ä½•ï¼›é»˜è®¤ï¼šä»…åŒä¸€æ™ºèƒ½ä½“ï¼‰
  - `tools`ï¼šæ¯æ™ºèƒ½ä½“å·¥å…·é™åˆ¶ï¼ˆåœ¨æ²™ç®±å·¥å…·ç­–ç•¥ä¹‹å‰åº”ç”¨ï¼‰ã€‚
    - `profile`ï¼šåŸºç¡€å·¥å…·é…ç½®æ–‡ä»¶ï¼ˆåœ¨ allow/deny ä¹‹å‰åº”ç”¨ï¼‰
    - `allow`ï¼šå…è®¸çš„å·¥å…·åç§°æ•°ç»„
    - `deny`ï¼šæ‹’ç»çš„å·¥å…·åç§°æ•°ç»„ï¼ˆdeny ä¼˜å…ˆï¼‰
- `agents.defaults`ï¼šå…±äº«çš„æ™ºèƒ½ä½“é»˜è®¤å€¼ï¼ˆæ¨¡åž‹ã€å·¥ä½œåŒºã€æ²™ç®±ç­‰ï¼‰ã€‚
- `bindings[]`ï¼šå°†å…¥ç«™æ¶ˆæ¯è·¯ç”±åˆ° `agentId`ã€‚
  - `match.channel`ï¼ˆå¿…éœ€ï¼‰
  - `match.accountId`ï¼ˆå¯é€‰ï¼›`*` = ä»»ä½•è´¦å·ï¼›çœç•¥ = é»˜è®¤è´¦å·ï¼‰
  - `match.peer`ï¼ˆå¯é€‰ï¼›`{ kind: dm|group|channel, id }`ï¼‰
  - `match.guildId` / `match.teamId`ï¼ˆå¯é€‰ï¼›æ¸ é“ç‰¹å®šï¼‰

ç¡®å®šæ€§åŒ¹é…é¡ºåºï¼š

1. `match.peer`
2. `match.guildId`
3. `match.teamId`
4. `match.accountId`ï¼ˆç²¾ç¡®åŒ¹é…ï¼Œæ—  peer/guild/teamï¼‰
5. `match.accountId: "*"`ï¼ˆæ¸ é“èŒƒå›´ï¼Œæ—  peer/guild/teamï¼‰
6. é»˜è®¤æ™ºèƒ½ä½“ï¼ˆ`agents.list[].default`ï¼Œå¦åˆ™ç¬¬ä¸€ä¸ªåˆ—è¡¨æ¡ç›®ï¼Œå¦åˆ™ `"main"`ï¼‰

åœ¨æ¯ä¸ªåŒ¹é…å±‚çº§å†…ï¼Œ`bindings` ä¸­çš„ç¬¬ä¸€ä¸ªåŒ¹é…æ¡ç›®èŽ·èƒœã€‚

#### æ¯æ™ºèƒ½ä½“è®¿é—®é…ç½®ï¼ˆå¤šæ™ºèƒ½ä½“ï¼‰

æ¯ä¸ªæ™ºèƒ½ä½“å¯ä»¥æºå¸¦è‡ªå·±çš„æ²™ç®± + å·¥å…·ç­–ç•¥ã€‚ç”¨äºŽåœ¨ä¸€ä¸ª Gateway ç½‘å…³ä¸­æ··åˆè®¿é—®çº§åˆ«ï¼š

- **å®Œå…¨è®¿é—®**ï¼ˆä¸ªäººæ™ºèƒ½ä½“ï¼‰
- **åªè¯»**å·¥å…· + å·¥ä½œåŒº
- **æ— æ–‡ä»¶ç³»ç»Ÿè®¿é—®**ï¼ˆä»…æ¶ˆæ¯/ä¼šè¯å·¥å…·ï¼‰

å‚è§[å¤šæ™ºèƒ½ä½“æ²™ç®±ä¸Žå·¥å…·](/tools/multi-agent-sandbox-tools)äº†è§£ä¼˜å…ˆçº§å’Œæ›´å¤šç¤ºä¾‹ã€‚

å®Œå…¨è®¿é—®ï¼ˆæ— æ²™ç®±ï¼‰ï¼š

```json5
{
  agents: {
    list: [
      {
        id: "personal",
        workspace: "~/.krabkrab/workspace-personal",
        sandbox: { mode: "off" },
      },
    ],
  },
}
```

åªè¯»å·¥å…· + åªè¯»å·¥ä½œåŒºï¼š

```json5
{
  agents: {
    list: [
      {
        id: "family",
        workspace: "~/.krabkrab/workspace-family",
        sandbox: {
          mode: "all",
          scope: "agent",
          workspaceAccess: "ro",
        },
        tools: {
          allow: [
            "read",
            "sessions_list",
            "sessions_history",
            "sessions_send",
            "sessions_spawn",
            "session_status",
          ],
          deny: ["write", "edit", "apply_patch", "exec", "process", "browser"],
        },
      },
    ],
  },
}
```

æ— æ–‡ä»¶ç³»ç»Ÿè®¿é—®ï¼ˆå¯ç”¨æ¶ˆæ¯/ä¼šè¯å·¥å…·ï¼‰ï¼š

```json5
{
  agents: {
    list: [
      {
        id: "public",
        workspace: "~/.krabkrab/workspace-public",
        sandbox: {
          mode: "all",
          scope: "agent",
          workspaceAccess: "none",
        },
        tools: {
          allow: [
            "sessions_list",
            "sessions_history",
            "sessions_send",
            "sessions_spawn",
            "session_status",
            "whatsapp",
            "telegram",
            "slack",
            "discord",
            "gateway",
          ],
          deny: [
            "read",
            "write",
            "edit",
            "apply_patch",
            "exec",
            "process",
            "browser",
            "canvas",
            "nodes",
            "cron",
            "gateway",
            "image",
          ],
        },
      },
    ],
  },
}
```

ç¤ºä¾‹ï¼šä¸¤ä¸ª WhatsApp è´¦å· â†’ ä¸¤ä¸ªæ™ºèƒ½ä½“ï¼š

```json5
{
  agents: {
    list: [
      { id: "home", default: true, workspace: "~/.krabkrab/workspace-home" },
      { id: "work", workspace: "~/.krabkrab/workspace-work" },
    ],
  },
  bindings: [
    { agentId: "home", match: { channel: "whatsapp", accountId: "personal" } },
    { agentId: "work", match: { channel: "whatsapp", accountId: "biz" } },
  ],
  channels: {
    whatsapp: {
      accounts: {
        personal: {},
        biz: {},
      },
    },
  },
}
```

### `tools.agentToAgent`ï¼ˆå¯é€‰ï¼‰

æ™ºèƒ½ä½“é—´æ¶ˆæ¯ä¼ é€’ä¸ºå¯é€‰åŠŸèƒ½ï¼š

```json5
{
  tools: {
    agentToAgent: {
      enabled: false,
      allow: ["home", "work"],
    },
  },
}
```

### `messages.queue`

æŽ§åˆ¶æ™ºèƒ½ä½“è¿è¡Œå·²åœ¨æ‰§è¡Œæ—¶å…¥ç«™æ¶ˆæ¯çš„è¡Œä¸ºã€‚

```json5
{
  messages: {
    queue: {
      mode: "collect", // steer | followup | collect | steer-backlog (steer+backlog ok) | interrupt (queue=steer legacy)
      debounceMs: 1000,
      cap: 20,
      drop: "summarize", // old | new | summarize
      byChannel: {
        whatsapp: "collect",
        telegram: "collect",
        discord: "collect",
        imessage: "collect",
        webchat: "collect",
      },
    },
  },
}
```

### `messages.inbound`

é˜²æŠ–**åŒä¸€å‘é€è€…**çš„å¿«é€Ÿå…¥ç«™æ¶ˆæ¯ï¼Œä½¿å¤šæ¡è¿žç»­æ¶ˆæ¯åˆå¹¶ä¸ºä¸€ä¸ªæ™ºèƒ½ä½“è½®æ¬¡ã€‚é˜²æŠ–æŒ‰æ¸ é“ + å¯¹è¯è¿›è¡ŒèŒƒå›´é™å®šï¼Œå¹¶ä½¿ç”¨æœ€æ–°æ¶ˆæ¯è¿›è¡Œå›žå¤çº¿ç¨‹/IDã€‚

```json5
{
  messages: {
    inbound: {
      debounceMs: 2000, // 0 ç¦ç”¨
      byChannel: {
        whatsapp: 5000,
        slack: 1500,
        discord: 1500,
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- é˜²æŠ–ä»…æ‰¹é‡å¤„ç†**çº¯æ–‡æœ¬**æ¶ˆæ¯ï¼›åª’ä½“/é™„ä»¶ç«‹å³åˆ·æ–°ã€‚
- æŽ§åˆ¶å‘½ä»¤ï¼ˆä¾‹å¦‚ `/queue`ã€`/new`ï¼‰ç»•è¿‡é˜²æŠ–ï¼Œä¿æŒç‹¬ç«‹ã€‚

### `commands`ï¼ˆèŠå¤©å‘½ä»¤å¤„ç†ï¼‰

æŽ§åˆ¶è·¨è¿žæŽ¥å™¨çš„èŠå¤©å‘½ä»¤å¯ç”¨æ–¹å¼ã€‚

```json5
{
  commands: {
    native: "auto", // åœ¨æ”¯æŒçš„å¹³å°ä¸Šæ³¨å†ŒåŽŸç”Ÿå‘½ä»¤ï¼ˆautoï¼‰
    text: true, // è§£æžèŠå¤©æ¶ˆæ¯ä¸­çš„æ–œæ å‘½ä»¤
    bash: false, // å…è®¸ !ï¼ˆåˆ«åï¼š/bashï¼‰ï¼ˆä»…é™ä¸»æœºï¼›éœ€è¦ tools.elevated ç™½åå•ï¼‰
    bashForegroundMs: 2000, // bash å‰å°çª—å£ï¼ˆ0 ç«‹å³åŽå°è¿è¡Œï¼‰
    config: false, // å…è®¸ /configï¼ˆå†™å…¥ç£ç›˜ï¼‰
    debug: false, // å…è®¸ /debugï¼ˆä»…è¿è¡Œæ—¶è¦†ç›–ï¼‰
    restart: false, // å…è®¸ /restart + gateway é‡å¯å·¥å…·
    useAccessGroups: true, // å¯¹å‘½ä»¤æ‰§è¡Œè®¿é—®ç»„ç™½åå•/ç­–ç•¥
  },
}
```

è¯´æ˜Žï¼š

- æ–‡æœ¬å‘½ä»¤å¿…é¡»ä½œä¸º**ç‹¬ç«‹**æ¶ˆæ¯å‘é€ï¼Œå¹¶ä½¿ç”¨å‰å¯¼ `/`ï¼ˆæ— çº¯æ–‡æœ¬åˆ«åï¼‰ã€‚
- `commands.text: false` ç¦ç”¨è§£æžèŠå¤©æ¶ˆæ¯ä¸­çš„å‘½ä»¤ã€‚
- `commands.native: "auto"`ï¼ˆé»˜è®¤ï¼‰ä¸º Discord/Telegram å¯ç”¨åŽŸç”Ÿå‘½ä»¤ï¼ŒSlack ä¿æŒå…³é—­ï¼›ä¸æ”¯æŒçš„æ¸ é“ä¿æŒçº¯æ–‡æœ¬ã€‚
- è®¾ä¸º `commands.native: true|false` å¼ºåˆ¶å…¨éƒ¨å¼€å¯æˆ–å…³é—­ï¼Œæˆ–æŒ‰æ¸ é“è¦†ç›– `channels.discord.commands.native`ã€`channels.telegram.commands.native`ã€`channels.slack.commands.native`ï¼ˆbool æˆ– `"auto"`ï¼‰ã€‚`false` åœ¨å¯åŠ¨æ—¶æ¸…é™¤ Discord/Telegram ä¸Šå…ˆå‰æ³¨å†Œçš„å‘½ä»¤ï¼›Slack å‘½ä»¤åœ¨ Slack åº”ç”¨ä¸­ç®¡ç†ã€‚
- `channels.telegram.customCommands` æ·»åŠ é¢å¤–çš„ Telegram æœºå™¨äººèœå•é¡¹ã€‚åç§°ä¼šè¢«è§„èŒƒåŒ–ï¼›ä¸ŽåŽŸç”Ÿå‘½ä»¤å†²çªçš„ä¼šè¢«å¿½ç•¥ã€‚
- `commands.bash: true` å¯ç”¨ `! <cmd>` è¿è¡Œä¸»æœº shell å‘½ä»¤ï¼ˆ`/bash <cmd>` ä¹Ÿå¯ä½œä¸ºåˆ«åï¼‰ã€‚éœ€è¦ `tools.elevated.enabled` å¹¶åœ¨ `tools.elevated.allowFrom.<channel>` ä¸­æ·»åŠ å‘é€è€…ç™½åå•ã€‚
- `commands.bashForegroundMs` æŽ§åˆ¶ bash åœ¨åŽå°è¿è¡Œå‰ç­‰å¾…çš„æ—¶é—´ã€‚å½“ bash ä»»åŠ¡æ­£åœ¨è¿è¡Œæ—¶ï¼Œæ–°çš„ `! <cmd>` è¯·æ±‚ä¼šè¢«æ‹’ç»ï¼ˆä¸€æ¬¡ä¸€ä¸ªï¼‰ã€‚
- `commands.config: true` å¯ç”¨ `/config`ï¼ˆè¯»å†™ `krabkrab.json`ï¼‰ã€‚
- `channels.<provider>.configWrites` æŽ§åˆ¶ç”±è¯¥æ¸ é“å‘èµ·çš„é…ç½®å˜æ›´ï¼ˆé»˜è®¤ï¼štrueï¼‰ã€‚é€‚ç”¨äºŽ `/config set|unset` ä»¥åŠæä¾›å•†ç‰¹å®šçš„è‡ªåŠ¨è¿ç§»ï¼ˆTelegram è¶…çº§ç¾¤ç»„ ID å˜æ›´ã€Slack é¢‘é“ ID å˜æ›´ï¼‰ã€‚
- `commands.debug: true` å¯ç”¨ `/debug`ï¼ˆä»…è¿è¡Œæ—¶è¦†ç›–ï¼‰ã€‚
- `commands.restart: true` å¯ç”¨ `/restart` å’Œ gateway å·¥å…·é‡å¯åŠ¨ä½œã€‚
- `commands.useAccessGroups: false` å…è®¸å‘½ä»¤ç»•è¿‡è®¿é—®ç»„ç™½åå•/ç­–ç•¥ã€‚
- æ–œæ å‘½ä»¤å’ŒæŒ‡ä»¤ä»…å¯¹**å·²æŽˆæƒå‘é€è€…**æœ‰æ•ˆã€‚æŽˆæƒæ¥è‡ªæ¸ é“ç™½åå•/é…å¯¹ä»¥åŠ `commands.useAccessGroups`ã€‚

### `web`ï¼ˆWhatsApp Web æ¸ é“è¿è¡Œæ—¶ï¼‰

WhatsApp é€šè¿‡ Gateway ç½‘å…³çš„ Web æ¸ é“ï¼ˆBaileys Webï¼‰è¿è¡Œã€‚å½“å­˜åœ¨å·²é“¾æŽ¥çš„ä¼šè¯æ—¶è‡ªåŠ¨å¯åŠ¨ã€‚
è®¾ç½® `web.enabled: false` ä½¿å…¶é»˜è®¤å…³é—­ã€‚

```json5
{
  web: {
    enabled: true,
    heartbeatSeconds: 60,
    reconnect: {
      initialMs: 2000,
      maxMs: 120000,
      factor: 1.4,
      jitter: 0.2,
      maxAttempts: 0,
    },
  },
}
```

### `channels.telegram`ï¼ˆæœºå™¨äººä¼ è¾“ï¼‰

KrabKrab ä»…åœ¨å­˜åœ¨ `channels.telegram` é…ç½®æ®µæ—¶å¯åŠ¨ Telegramã€‚æœºå™¨äºº token ä»Ž `channels.telegram.botToken`ï¼ˆæˆ– `channels.telegram.tokenFile`ï¼‰è§£æžï¼Œ`TELEGRAM_BOT_TOKEN` ä½œä¸ºé»˜è®¤è´¦å·çš„å›žé€€ã€‚
è®¾ç½® `channels.telegram.enabled: false` ç¦ç”¨è‡ªåŠ¨å¯åŠ¨ã€‚
å¤šè´¦å·æ”¯æŒåœ¨ `channels.telegram.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ token ä»…é€‚ç”¨äºŽé»˜è®¤è´¦å·ã€‚
è®¾ç½® `channels.telegram.configWrites: false` é˜»æ­¢ Telegram å‘èµ·çš„é…ç½®å†™å…¥ï¼ˆåŒ…æ‹¬è¶…çº§ç¾¤ç»„ ID è¿ç§»å’Œ `/config set|unset`ï¼‰ã€‚

```json5
{
  channels: {
    telegram: {
      enabled: true,
      botToken: "your-bot-token",
      dmPolicy: "pairing", // pairing | allowlist | open | disabled
      allowFrom: ["tg:123456789"], // å¯é€‰ï¼›"open" éœ€è¦ ["*"]
      groups: {
        "*": { requireMention: true },
        "-1001234567890": {
          allowFrom: ["@admin"],
          systemPrompt: "Keep answers brief.",
          topics: {
            "99": {
              requireMention: false,
              skills: ["search"],
              systemPrompt: "Stay on topic.",
            },
          },
        },
      },
      customCommands: [
        { command: "backup", description: "Git backup" },
        { command: "generate", description: "Create an image" },
      ],
      historyLimit: 50, // åŒ…å«æœ€è¿‘ N æ¡ç¾¤æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡ï¼ˆ0 ç¦ç”¨ï¼‰
      replyToMode: "first", // off | first | all
      linkPreview: true, // åˆ‡æ¢å‡ºç«™é“¾æŽ¥é¢„è§ˆ
      streamMode: "partial", // off | partial | blockï¼ˆè‰ç¨¿æµå¼ä¼ è¾“ï¼›ä¸Žåˆ†å—æµå¼ä¼ è¾“åˆ†å¼€ï¼‰
      draftChunk: {
        // å¯é€‰ï¼›ä»…ç”¨äºŽ streamMode=block
        minChars: 200,
        maxChars: 800,
        breakPreference: "paragraph", // paragraph | newline | sentence
      },
      actions: { reactions: true, sendMessage: true }, // å·¥å…·åŠ¨ä½œå¼€å…³ï¼ˆfalse ç¦ç”¨ï¼‰
      reactionNotifications: "own", // off | own | all
      mediaMaxMb: 5,
      retry: {
        // å‡ºç«™é‡è¯•ç­–ç•¥
        attempts: 3,
        minDelayMs: 400,
        maxDelayMs: 30000,
        jitter: 0.1,
      },
      network: {
        // ä¼ è¾“è¦†ç›–
        autoSelectFamily: false,
      },
      proxy: "socks5://localhost:9050",
      webhookUrl: "https://example.com/telegram-webhook", // éœ€è¦ webhookSecret
      webhookSecret: "secret",
      webhookPath: "/telegram-webhook",
    },
  },
}
```

è‰ç¨¿æµå¼ä¼ è¾“è¯´æ˜Žï¼š

- ä½¿ç”¨ Telegram `sendMessageDraft`ï¼ˆè‰ç¨¿æ°”æ³¡ï¼Œä¸æ˜¯çœŸæ­£çš„æ¶ˆæ¯ï¼‰ã€‚
- éœ€è¦**ç§èŠè¯é¢˜**ï¼ˆç§ä¿¡ ä¸­çš„ message_thread_idï¼›æœºå™¨äººå·²å¯ç”¨è¯é¢˜ï¼‰ã€‚
- `/reasoning stream` å°†æŽ¨ç†è¿‡ç¨‹æµå¼ä¼ è¾“åˆ°è‰ç¨¿ä¸­ï¼Œç„¶åŽå‘é€æœ€ç»ˆç­”æ¡ˆã€‚
  é‡è¯•ç­–ç•¥é»˜è®¤å€¼å’Œè¡Œä¸ºè®°å½•åœ¨[é‡è¯•ç­–ç•¥](/concepts/retry)ä¸­ã€‚

### `channels.discord`ï¼ˆæœºå™¨äººä¼ è¾“ï¼‰

é€šè¿‡è®¾ç½®æœºå™¨äºº token å’Œå¯é€‰çš„é—¨æŽ§é…ç½® Discord æœºå™¨äººï¼š
å¤šè´¦å·æ”¯æŒåœ¨ `channels.discord.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ token ä»…é€‚ç”¨äºŽé»˜è®¤è´¦å·ã€‚

```json5
{
  channels: {
    discord: {
      enabled: true,
      token: "your-bot-token",
      mediaMaxMb: 8, // é™åˆ¶å…¥ç«™åª’ä½“å¤§å°
      allowBots: false, // å…è®¸æœºå™¨äººå‘é€çš„æ¶ˆæ¯
      actions: {
        // å·¥å…·åŠ¨ä½œå¼€å…³ï¼ˆfalse ç¦ç”¨ï¼‰
        reactions: true,
        stickers: true,
        polls: true,
        permissions: true,
        messages: true,
        threads: true,
        pins: true,
        search: true,
        memberInfo: true,
        roleInfo: true,
        roles: false,
        channelInfo: true,
        voiceStatus: true,
        events: true,
        moderation: false,
      },
      replyToMode: "off", // off | first | all
      dm: {
        enabled: true, // è®¾ä¸º false æ—¶ç¦ç”¨æ‰€æœ‰ç§èŠ
        policy: "pairing", // pairing | allowlist | open | disabled
        allowFrom: ["1234567890", "steipete"], // å¯é€‰ç§èŠç™½åå•ï¼ˆ"open" éœ€è¦ ["*"]ï¼‰
        groupEnabled: false, // å¯ç”¨ç¾¤ç»„ç§èŠ
        groupChannels: ["krabkrab-dm"], // å¯é€‰ç¾¤ç»„ç§èŠç™½åå•
      },
      guilds: {
        "123456789012345678": {
          // æœåŠ¡å™¨ idï¼ˆæŽ¨èï¼‰æˆ– slug
          slug: "friends-of-krabkrab",
          requireMention: false, // æ¯æœåŠ¡å™¨é»˜è®¤å€¼
          reactionNotifications: "own", // off | own | all | allowlist
          users: ["987654321098765432"], // å¯é€‰çš„æ¯æœåŠ¡å™¨ç”¨æˆ·ç™½åå•
          channels: {
            general: { allow: true },
            help: {
              allow: true,
              requireMention: true,
              users: ["987654321098765432"],
              skills: ["docs"],
              systemPrompt: "Short answers only.",
            },
          },
        },
      },
      historyLimit: 20, // åŒ…å«æœ€è¿‘ N æ¡æœåŠ¡å™¨æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡
      textChunkLimit: 2000, // å¯é€‰å‡ºç«™æ–‡æœ¬åˆ†å—å¤§å°ï¼ˆå­—ç¬¦æ•°ï¼‰
      chunkMode: "length", // å¯é€‰åˆ†å—æ¨¡å¼ï¼ˆlength | newlineï¼‰
      maxLinesPerMessage: 17, // æ¯æ¡æ¶ˆæ¯çš„è½¯æœ€å¤§è¡Œæ•°ï¼ˆDiscord UI è£å‰ªï¼‰
      retry: {
        // å‡ºç«™é‡è¯•ç­–ç•¥
        attempts: 3,
        minDelayMs: 500,
        maxDelayMs: 30000,
        jitter: 0.1,
      },
    },
  },
}
```

KrabKrab ä»…åœ¨å­˜åœ¨ `channels.discord` é…ç½®æ®µæ—¶å¯åŠ¨ Discordã€‚token ä»Ž `channels.discord.token` è§£æžï¼Œ`DISCORD_BOT_TOKEN` ä½œä¸ºé»˜è®¤è´¦å·çš„å›žé€€ï¼ˆé™¤éž `channels.discord.enabled` ä¸º `false`ï¼‰ã€‚åœ¨ä¸º cron/CLI å‘½ä»¤æŒ‡å®šæŠ•é€’ç›®æ ‡æ—¶ï¼Œä½¿ç”¨ `user:<id>`ï¼ˆç§èŠï¼‰æˆ– `channel:<id>`ï¼ˆæœåŠ¡å™¨é¢‘é“ï¼‰ï¼›è£¸æ•°å­— ID æœ‰æ­§ä¹‰ä¼šè¢«æ‹’ç»ã€‚
æœåŠ¡å™¨ slug ä¸ºå°å†™ï¼Œç©ºæ ¼æ›¿æ¢ä¸º `-`ï¼›é¢‘é“é”®ä½¿ç”¨ slug åŒ–çš„é¢‘é“åç§°ï¼ˆæ— å‰å¯¼ `#`ï¼‰ã€‚å»ºè®®ä½¿ç”¨æœåŠ¡å™¨ id ä½œä¸ºé”®ä»¥é¿å…é‡å‘½åæ­§ä¹‰ã€‚
æœºå™¨äººå‘é€çš„æ¶ˆæ¯é»˜è®¤è¢«å¿½ç•¥ã€‚é€šè¿‡ `channels.discord.allowBots` å¯ç”¨ï¼ˆè‡ªèº«æ¶ˆæ¯ä»ä¼šè¢«è¿‡æ»¤ä»¥é˜²æ­¢è‡ªå›žå¤å¾ªçŽ¯ï¼‰ã€‚
ååº”é€šçŸ¥æ¨¡å¼ï¼š

- `off`ï¼šæ— ååº”äº‹ä»¶ã€‚
- `own`ï¼šæœºå™¨äººè‡ªèº«æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆé»˜è®¤ï¼‰ã€‚
- `all`ï¼šæ‰€æœ‰æ¶ˆæ¯ä¸Šçš„æ‰€æœ‰ååº”ã€‚
- `allowlist`ï¼š`guilds.<id>.users` ä¸­çš„ç”¨æˆ·åœ¨æ‰€æœ‰æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆç©ºåˆ—è¡¨ç¦ç”¨ï¼‰ã€‚
  å‡ºç«™æ–‡æœ¬æŒ‰ `channels.discord.textChunkLimit`ï¼ˆé»˜è®¤ 2000ï¼‰åˆ†å—ã€‚è®¾ç½® `channels.discord.chunkMode="newline"` åœ¨é•¿åº¦åˆ†å—å‰æŒ‰ç©ºè¡Œï¼ˆæ®µè½è¾¹ç•Œï¼‰åˆ†å‰²ã€‚Discord å®¢æˆ·ç«¯å¯èƒ½è£å‰ªè¿‡é«˜çš„æ¶ˆæ¯ï¼Œå› æ­¤ `channels.discord.maxLinesPerMessage`ï¼ˆé»˜è®¤ 17ï¼‰å³ä½¿åœ¨ 2000 å­—ç¬¦ä»¥å†…ä¹Ÿä¼šåˆ†å‰²é•¿å¤šè¡Œå›žå¤ã€‚
  é‡è¯•ç­–ç•¥é»˜è®¤å€¼å’Œè¡Œä¸ºè®°å½•åœ¨[é‡è¯•ç­–ç•¥](/concepts/retry)ä¸­ã€‚

### `channels.googlechat`ï¼ˆChat API webhookï¼‰

Google Chat é€šè¿‡ HTTP webhook è¿è¡Œï¼Œä½¿ç”¨åº”ç”¨çº§è®¤è¯ï¼ˆæœåŠ¡è´¦å·ï¼‰ã€‚
å¤šè´¦å·æ”¯æŒåœ¨ `channels.googlechat.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ä»…é€‚ç”¨äºŽé»˜è®¤è´¦å·ã€‚

```json5
{
  channels: {
    googlechat: {
      enabled: true,
      serviceAccountFile: "/path/to/service-account.json",
      audienceType: "app-url", // app-url | project-number
      audience: "https://gateway.example.com/googlechat",
      webhookPath: "/googlechat",
      botUser: "users/1234567890", // å¯é€‰ï¼›æ”¹å–„æåŠæ£€æµ‹
      dm: {
        enabled: true,
        policy: "pairing", // pairing | allowlist | open | disabled
        allowFrom: ["users/1234567890"], // å¯é€‰ï¼›"open" éœ€è¦ ["*"]
      },
      groupPolicy: "allowlist",
      groups: {
        "spaces/AAAA": { allow: true, requireMention: true },
      },
      actions: { reactions: true },
      typingIndicator: "message",
      mediaMaxMb: 20,
    },
  },
}
```

è¯´æ˜Žï¼š

- æœåŠ¡è´¦å· JSON å¯ä»¥å†…è”ï¼ˆ`serviceAccount`ï¼‰æˆ–åŸºäºŽæ–‡ä»¶ï¼ˆ`serviceAccountFile`ï¼‰ã€‚
- é»˜è®¤è´¦å·çš„çŽ¯å¢ƒå˜é‡å›žé€€ï¼š`GOOGLE_CHAT_SERVICE_ACCOUNT` æˆ– `GOOGLE_CHAT_SERVICE_ACCOUNT_FILE`ã€‚
- `audienceType` + `audience` å¿…é¡»ä¸Ž Chat åº”ç”¨çš„ webhook è®¤è¯é…ç½®åŒ¹é…ã€‚
- è®¾ç½®æŠ•é€’ç›®æ ‡æ—¶ä½¿ç”¨ `spaces/<spaceId>` æˆ– `users/<userId|email>`ã€‚

### `channels.slack`ï¼ˆsocket æ¨¡å¼ï¼‰

Slack ä»¥ Socket Mode è¿è¡Œï¼Œéœ€è¦æœºå™¨äºº token å’Œåº”ç”¨ tokenï¼š

```json5
{
  channels: {
    slack: {
      enabled: true,
      botToken: "xoxb-...",
      appToken: "xapp-...",
      dm: {
        enabled: true,
        policy: "pairing", // pairing | allowlist | open | disabled
        allowFrom: ["U123", "U456", "*"], // å¯é€‰ï¼›"open" éœ€è¦ ["*"]
        groupEnabled: false,
        groupChannels: ["G123"],
      },
      channels: {
        C123: { allow: true, requireMention: true, allowBots: false },
        "#general": {
          allow: true,
          requireMention: true,
          allowBots: false,
          users: ["U123"],
          skills: ["docs"],
          systemPrompt: "Short answers only.",
        },
      },
      historyLimit: 50, // åŒ…å«æœ€è¿‘ N æ¡é¢‘é“/ç¾¤ç»„æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡ï¼ˆ0 ç¦ç”¨ï¼‰
      allowBots: false,
      reactionNotifications: "own", // off | own | all | allowlist
      reactionAllowlist: ["U123"],
      replyToMode: "off", // off | first | all
      thread: {
        historyScope: "thread", // thread | channel
        inheritParent: false,
      },
      actions: {
        reactions: true,
        messages: true,
        pins: true,
        memberInfo: true,
        emojiList: true,
      },
      slashCommand: {
        enabled: true,
        name: "krabkrab",
        sessionPrefix: "slack:slash",
        ephemeral: true,
      },
      textChunkLimit: 4000,
      chunkMode: "length",
      mediaMaxMb: 20,
    },
  },
}
```

å¤šè´¦å·æ”¯æŒåœ¨ `channels.slack.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ token ä»…é€‚ç”¨äºŽé»˜è®¤è´¦å·ã€‚

KrabKrab åœ¨æä¾›å•†å¯ç”¨ä¸”ä¸¤ä¸ª token éƒ½å·²è®¾ç½®æ—¶å¯åŠ¨ Slackï¼ˆé€šè¿‡é…ç½®æˆ– `SLACK_BOT_TOKEN` + `SLACK_APP_TOKEN`ï¼‰ã€‚åœ¨ä¸º cron/CLI å‘½ä»¤æŒ‡å®šæŠ•é€’ç›®æ ‡æ—¶ä½¿ç”¨ `user:<id>`ï¼ˆç§èŠï¼‰æˆ– `channel:<id>`ã€‚
è®¾ç½® `channels.slack.configWrites: false` é˜»æ­¢ Slack å‘èµ·çš„é…ç½®å†™å…¥ï¼ˆåŒ…æ‹¬é¢‘é“ ID è¿ç§»å’Œ `/config set|unset`ï¼‰ã€‚

æœºå™¨äººå‘é€çš„æ¶ˆæ¯é»˜è®¤è¢«å¿½ç•¥ã€‚é€šè¿‡ `channels.slack.allowBots` æˆ– `channels.slack.channels.<id>.allowBots` å¯ç”¨ã€‚

ååº”é€šçŸ¥æ¨¡å¼ï¼š

- `off`ï¼šæ— ååº”äº‹ä»¶ã€‚
- `own`ï¼šæœºå™¨äººè‡ªèº«æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆé»˜è®¤ï¼‰ã€‚
- `all`ï¼šæ‰€æœ‰æ¶ˆæ¯ä¸Šçš„æ‰€æœ‰ååº”ã€‚
- `allowlist`ï¼š`channels.slack.reactionAllowlist` ä¸­çš„ç”¨æˆ·åœ¨æ‰€æœ‰æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆç©ºåˆ—è¡¨ç¦ç”¨ï¼‰ã€‚

çº¿ç¨‹ä¼šè¯éš”ç¦»ï¼š

- `channels.slack.thread.historyScope` æŽ§åˆ¶çº¿ç¨‹åŽ†å²æ˜¯æŒ‰çº¿ç¨‹ï¼ˆ`thread`ï¼Œé»˜è®¤ï¼‰è¿˜æ˜¯è·¨é¢‘é“å…±äº«ï¼ˆ`channel`ï¼‰ã€‚
- `channels.slack.thread.inheritParent` æŽ§åˆ¶æ–°çº¿ç¨‹ä¼šè¯æ˜¯å¦ç»§æ‰¿çˆ¶é¢‘é“çš„è®°å½•ï¼ˆé»˜è®¤ï¼šfalseï¼‰ã€‚

Slack åŠ¨ä½œç»„ï¼ˆæŽ§åˆ¶ `slack` å·¥å…·åŠ¨ä½œï¼‰ï¼š
| åŠ¨ä½œç»„ | é»˜è®¤ | è¯´æ˜Ž |
| --- | --- | --- |
| reactions | å·²å¯ç”¨ | ååº” + åˆ—å‡ºååº” |
| messages | å·²å¯ç”¨ | è¯»å–/å‘é€/ç¼–è¾‘/åˆ é™¤ |
| pins | å·²å¯ç”¨ | å›ºå®š/å–æ¶ˆå›ºå®š/åˆ—å‡º |
| memberInfo | å·²å¯ç”¨ | æˆå‘˜ä¿¡æ¯ |
| emojiList | å·²å¯ç”¨ | è‡ªå®šä¹‰è¡¨æƒ…åˆ—è¡¨ |

### `channels.mattermost`ï¼ˆæœºå™¨äºº tokenï¼‰

Mattermost ä½œä¸ºæ’ä»¶æä¾›ï¼Œä¸åŒ…å«åœ¨æ ¸å¿ƒå®‰è£…ä¸­ã€‚
è¯·å…ˆå®‰è£…ï¼š`krabkrab plugins install @krabkrab/mattermost`ï¼ˆæˆ–ä»Ž git checkout ä½¿ç”¨ `./extensions/mattermost`ï¼‰ã€‚

Mattermost éœ€è¦æœºå™¨äºº token åŠ ä¸ŠæœåŠ¡å™¨çš„åŸºç¡€ URLï¼š

```json5
{
  channels: {
    mattermost: {
      enabled: true,
      botToken: "mm-token",
      baseUrl: "https://chat.example.com",
      dmPolicy: "pairing",
      chatmode: "oncall", // oncall | onmessage | onchar
      oncharPrefixes: [">", "!"],
      textChunkLimit: 4000,
      chunkMode: "length",
    },
  },
}
```

KrabKrab åœ¨è´¦å·å·²é…ç½®ï¼ˆæœºå™¨äºº token + åŸºç¡€ URLï¼‰ä¸”å·²å¯ç”¨æ—¶å¯åŠ¨ Mattermostã€‚token + åŸºç¡€ URL ä»Ž `channels.mattermost.botToken` + `channels.mattermost.baseUrl` æˆ–é»˜è®¤è´¦å·çš„ `MATTERMOST_BOT_TOKEN` + `MATTERMOST_URL` è§£æžï¼ˆé™¤éž `channels.mattermost.enabled` ä¸º `false`ï¼‰ã€‚

èŠå¤©æ¨¡å¼ï¼š

- `oncall`ï¼ˆé»˜è®¤ï¼‰ï¼šä»…åœ¨è¢« @æåŠæ—¶å“åº”é¢‘é“æ¶ˆæ¯ã€‚
- `onmessage`ï¼šå“åº”æ¯æ¡é¢‘é“æ¶ˆæ¯ã€‚
- `onchar`ï¼šå½“æ¶ˆæ¯ä»¥è§¦å‘å‰ç¼€å¼€å¤´æ—¶å“åº”ï¼ˆ`channels.mattermost.oncharPrefixes`ï¼Œé»˜è®¤ `[">", "!"]`ï¼‰ã€‚

è®¿é—®æŽ§åˆ¶ï¼š

- é»˜è®¤ç§èŠï¼š`channels.mattermost.dmPolicy="pairing"`ï¼ˆæœªçŸ¥å‘é€è€…æ”¶åˆ°é…å¯¹ç ï¼‰ã€‚
- å…¬å¼€ç§èŠï¼š`channels.mattermost.dmPolicy="open"` åŠ ä¸Š `channels.mattermost.allowFrom=["*"]`ã€‚
- ç¾¤ç»„ï¼š`channels.mattermost.groupPolicy="allowlist"` ä¸ºé»˜è®¤å€¼ï¼ˆæåŠé—¨æŽ§ï¼‰ã€‚ä½¿ç”¨ `channels.mattermost.groupAllowFrom` é™åˆ¶å‘é€è€…ã€‚

å¤šè´¦å·æ”¯æŒåœ¨ `channels.mattermost.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ä»…é€‚ç”¨äºŽé»˜è®¤è´¦å·ã€‚
æŒ‡å®šæŠ•é€’ç›®æ ‡æ—¶ä½¿ç”¨ `channel:<id>` æˆ– `user:<id>`ï¼ˆæˆ– `@username`ï¼‰ï¼›è£¸ id è¢«è§†ä¸ºé¢‘é“ idã€‚

### `channels.signal`ï¼ˆsignal-cliï¼‰

Signal ååº”å¯ä»¥å‘å‡ºç³»ç»Ÿäº‹ä»¶ï¼ˆå…±äº«ååº”å·¥å…·ï¼‰ï¼š

```json5
{
  channels: {
    signal: {
      reactionNotifications: "own", // off | own | all | allowlist
      reactionAllowlist: ["+15551234567", "uuid:123e4567-e89b-12d3-a456-426614174000"],
      historyLimit: 50, // åŒ…å«æœ€è¿‘ N æ¡ç¾¤æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡ï¼ˆ0 ç¦ç”¨ï¼‰
    },
  },
}
```

ååº”é€šçŸ¥æ¨¡å¼ï¼š

- `off`ï¼šæ— ååº”äº‹ä»¶ã€‚
- `own`ï¼šæœºå™¨äººè‡ªèº«æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆé»˜è®¤ï¼‰ã€‚
- `all`ï¼šæ‰€æœ‰æ¶ˆæ¯ä¸Šçš„æ‰€æœ‰ååº”ã€‚
- `allowlist`ï¼š`channels.signal.reactionAllowlist` ä¸­çš„ç”¨æˆ·åœ¨æ‰€æœ‰æ¶ˆæ¯ä¸Šçš„ååº”ï¼ˆç©ºåˆ—è¡¨ç¦ç”¨ï¼‰ã€‚

### `channels.imessage`ï¼ˆimsg CLIï¼‰

KrabKrab ä¼šç”Ÿæˆ `imsg rpc`ï¼ˆé€šè¿‡ stdio çš„ JSON-RPCï¼‰ã€‚æ— éœ€å®ˆæŠ¤è¿›ç¨‹æˆ–ç«¯å£ã€‚

```json5
{
  channels: {
    imessage: {
      enabled: true,
      cliPath: "imsg",
      dbPath: "~/Library/Messages/chat.db",
      remoteHost: "user@gateway-host", // ä½¿ç”¨ SSH åŒ…è£…å™¨æ—¶é€šè¿‡ SCP èŽ·å–è¿œç¨‹é™„ä»¶
      dmPolicy: "pairing", // pairing | allowlist | open | disabled
      allowFrom: ["+15555550123", "user@example.com", "chat_id:123"],
      historyLimit: 50, // åŒ…å«æœ€è¿‘ N æ¡ç¾¤æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡ï¼ˆ0 ç¦ç”¨ï¼‰
      includeAttachments: false,
      mediaMaxMb: 16,
      service: "auto",
      region: "US",
    },
  },
}
```

å¤šè´¦å·æ”¯æŒåœ¨ `channels.imessage.accounts` ä¸‹ï¼ˆå‚è§ä¸Šæ–¹å¤šè´¦å·éƒ¨åˆ†ï¼‰ã€‚

è¯´æ˜Žï¼š

- éœ€è¦å¯¹æ¶ˆæ¯æ•°æ®åº“çš„å®Œå…¨ç£ç›˜è®¿é—®æƒé™ã€‚
- é¦–æ¬¡å‘é€æ—¶ä¼šæç¤ºè¯·æ±‚æ¶ˆæ¯è‡ªåŠ¨åŒ–æƒé™ã€‚
- å»ºè®®ä½¿ç”¨ `chat_id:<id>` ç›®æ ‡ã€‚ä½¿ç”¨ `imsg chats --limit 20` åˆ—å‡ºèŠå¤©ã€‚
- `channels.imessage.cliPath` å¯ä»¥æŒ‡å‘åŒ…è£…è„šæœ¬ï¼ˆä¾‹å¦‚ `ssh` åˆ°å¦ä¸€å°è¿è¡Œ `imsg rpc` çš„ Macï¼‰ï¼›ä½¿ç”¨ SSH å¯†é’¥é¿å…å¯†ç æç¤ºã€‚
- å¯¹äºŽè¿œç¨‹ SSH åŒ…è£…å™¨ï¼Œè®¾ç½® `channels.imessage.remoteHost` ä»¥ä¾¿åœ¨å¯ç”¨ `includeAttachments` æ—¶é€šè¿‡ SCP èŽ·å–é™„ä»¶ã€‚

ç¤ºä¾‹åŒ…è£…å™¨ï¼š

```bash
#!/usr/bin/env bash
exec ssh -T gateway-host imsg "$@"
```

### `agents.defaults.workspace`

è®¾ç½®æ™ºèƒ½ä½“ç”¨äºŽæ–‡ä»¶æ“ä½œçš„**å•ä¸€å…¨å±€å·¥ä½œåŒºç›®å½•**ã€‚

é»˜è®¤ï¼š`~/.krabkrab/workspace`ã€‚

```json5
{
  agents: { defaults: { workspace: "~/.krabkrab/workspace" } },
}
```

å¦‚æžœå¯ç”¨äº† `agents.defaults.sandbox`ï¼Œéžä¸»ä¼šè¯å¯ä»¥åœ¨ `agents.defaults.sandbox.workspaceRoot` ä¸‹ä½¿ç”¨å„è‡ªçš„æ¯èŒƒå›´å·¥ä½œåŒºæ¥è¦†ç›–æ­¤è®¾ç½®ã€‚

### `agents.defaults.repoRoot`

åœ¨ç³»ç»Ÿæç¤ºçš„ Runtime è¡Œä¸­æ˜¾ç¤ºçš„å¯é€‰ä»“åº“æ ¹ç›®å½•ã€‚å¦‚æžœæœªè®¾ç½®ï¼ŒKrabKrab ä¼šä»Žå·¥ä½œåŒºï¼ˆå’Œå½“å‰å·¥ä½œç›®å½•ï¼‰å‘ä¸ŠæŸ¥æ‰¾ `.git` ç›®å½•è¿›è¡Œæ£€æµ‹ã€‚è·¯å¾„å¿…é¡»å­˜åœ¨æ‰èƒ½ä½¿ç”¨ã€‚

```json5
{
  agents: { defaults: { repoRoot: "~/Projects/krabkrab" } },
}
```

### `agents.defaults.skipBootstrap`

ç¦ç”¨è‡ªåŠ¨åˆ›å»ºå·¥ä½œåŒºå¼•å¯¼æ–‡ä»¶ï¼ˆ`AGENTS.md`ã€`SOUL.md`ã€`TOOLS.md`ã€`IDENTITY.md`ã€`USER.md` å’Œ `BOOTSTRAP.md`ï¼‰ã€‚

é€‚ç”¨äºŽå·¥ä½œåŒºæ–‡ä»¶æ¥è‡ªä»“åº“çš„é¢„ç½®éƒ¨ç½²ã€‚

```json5
{
  agents: { defaults: { skipBootstrap: true } },
}
```

### `agents.defaults.bootstrapMaxChars`

æ³¨å…¥ç³»ç»Ÿæç¤ºå‰æ¯ä¸ªå·¥ä½œåŒºå¼•å¯¼æ–‡ä»¶æˆªæ–­å‰çš„æœ€å¤§å­—ç¬¦æ•°ã€‚é»˜è®¤ï¼š`20000`ã€‚

å½“æ–‡ä»¶è¶…è¿‡æ­¤é™åˆ¶æ—¶ï¼ŒKrabKrab ä¼šè®°å½•è­¦å‘Šå¹¶æ³¨å…¥å¸¦æ ‡è®°çš„å¤´å°¾æˆªæ–­å†…å®¹ã€‚

```json5
{
  agents: { defaults: { bootstrapMaxChars: 20000 } },
}
```

### `agents.defaults.userTimezone`

è®¾ç½®ç”¨æˆ·æ—¶åŒºç”¨äºŽ**ç³»ç»Ÿæç¤ºä¸Šä¸‹æ–‡**ï¼ˆä¸ç”¨äºŽæ¶ˆæ¯ä¿¡å°ä¸­çš„æ—¶é—´æˆ³ï¼‰ã€‚å¦‚æžœæœªè®¾ç½®ï¼ŒKrabKrab åœ¨è¿è¡Œæ—¶ä½¿ç”¨ä¸»æœºæ—¶åŒºã€‚

```json5
{
  agents: { defaults: { userTimezone: "America/Chicago" } },
}
```

### `agents.defaults.timeFormat`

æŽ§åˆ¶ç³»ç»Ÿæç¤ºä¸­"å½“å‰æ—¥æœŸå’Œæ—¶é—´"éƒ¨åˆ†æ˜¾ç¤ºçš„**æ—¶é—´æ ¼å¼**ã€‚
é»˜è®¤ï¼š`auto`ï¼ˆæ“ä½œç³»ç»Ÿåå¥½ï¼‰ã€‚

```json5
{
  agents: { defaults: { timeFormat: "auto" } }, // auto | 12 | 24
}
```

### `messages`

æŽ§åˆ¶å…¥ç«™/å‡ºç«™å‰ç¼€å’Œå¯é€‰çš„ç¡®è®¤ååº”ã€‚
å‚è§[æ¶ˆæ¯](/concepts/messages)äº†è§£æŽ’é˜Ÿã€ä¼šè¯å’Œæµå¼ä¸Šä¸‹æ–‡ã€‚

```json5
{
  messages: {
    responsePrefix: "ðŸ¦ž", // æˆ– "auto"
    ackReaction: "ðŸ‘€",
    ackReactionScope: "group-mentions",
    removeAckAfterReply: false,
  },
}
```

`responsePrefix` åº”ç”¨äºŽè·¨æ¸ é“çš„**æ‰€æœ‰å‡ºç«™å›žå¤**ï¼ˆå·¥å…·æ‘˜è¦ã€åˆ†å—æµå¼ä¼ è¾“ã€æœ€ç»ˆå›žå¤ï¼‰ï¼Œé™¤éžå·²å­˜åœ¨ã€‚

å¦‚æžœæœªè®¾ç½® `messages.responsePrefix`ï¼Œé»˜è®¤ä¸åº”ç”¨å‰ç¼€ã€‚WhatsApp è‡ªèŠå¤©å›žå¤æ˜¯ä¾‹å¤–ï¼šå®ƒä»¬åœ¨è®¾ç½®æ—¶é»˜è®¤ä¸º `[{identity.name}]`ï¼Œå¦åˆ™ä¸º `[krabkrab]`ï¼Œä»¥ä¿æŒåŒä¸€æ‰‹æœºä¸Šçš„å¯¹è¯å¯è¯»æ€§ã€‚
è®¾ä¸º `"auto"` å¯ä¸ºè·¯ç”±çš„æ™ºèƒ½ä½“æŽ¨å¯¼ `[{identity.name}]`ï¼ˆå½“è®¾ç½®æ—¶ï¼‰ã€‚

#### æ¨¡æ¿å˜é‡

`responsePrefix` å­—ç¬¦ä¸²å¯ä»¥åŒ…å«åŠ¨æ€è§£æžçš„æ¨¡æ¿å˜é‡ï¼š

| å˜é‡              | æè¿°           | ç¤ºä¾‹                        |
| ----------------- | -------------- | --------------------------- |
| `{model}`         | çŸ­æ¨¡åž‹åç§°     | `claude-opus-4-5`ã€`gpt-4o` |
| `{modelFull}`     | å®Œæ•´æ¨¡åž‹æ ‡è¯†ç¬¦ | `anthropic/claude-opus-4-5` |
| `{provider}`      | æä¾›å•†åç§°     | `anthropic`ã€`openai`       |
| `{thinkingLevel}` | å½“å‰æ€è€ƒçº§åˆ«   | `high`ã€`low`ã€`off`        |
| `{identity.name}` | æ™ºèƒ½ä½“èº«ä»½åç§° | ï¼ˆä¸Ž `"auto"` æ¨¡å¼ç›¸åŒï¼‰    |

å˜é‡ä¸åŒºåˆ†å¤§å°å†™ï¼ˆ`{MODEL}` = `{model}`ï¼‰ã€‚`{think}` æ˜¯ `{thinkingLevel}` çš„åˆ«åã€‚
æœªè§£æžçš„å˜é‡ä¿æŒä¸ºå­—é¢æ–‡æœ¬ã€‚

```json5
{
  messages: {
    responsePrefix: "[{model} | think:{thinkingLevel}]",
  },
}
```

è¾“å‡ºç¤ºä¾‹ï¼š`[claude-opus-4-5 | think:high] Here's my response...`

WhatsApp å…¥ç«™å‰ç¼€é€šè¿‡ `channels.whatsapp.messagePrefix` é…ç½®ï¼ˆå·²å¼ƒç”¨ï¼š`messages.messagePrefix`ï¼‰ã€‚é»˜è®¤ä¿æŒ**ä¸å˜**ï¼šå½“ `channels.whatsapp.allowFrom` ä¸ºç©ºæ—¶ä¸º `"[krabkrab]"`ï¼Œå¦åˆ™ä¸º `""`ï¼ˆæ— å‰ç¼€ï¼‰ã€‚ä½¿ç”¨ `"[krabkrab]"` æ—¶ï¼Œå¦‚æžœè·¯ç”±çš„æ™ºèƒ½ä½“è®¾ç½®äº† `identity.name`ï¼ŒKrabKrab ä¼šæ”¹ç”¨ `[{identity.name}]`ã€‚

`ackReaction` åœ¨æ”¯æŒååº”çš„æ¸ é“ï¼ˆSlack/Discord/Telegram/Google Chatï¼‰ä¸Šå‘é€å°½åŠ›è€Œä¸ºçš„è¡¨æƒ…ååº”æ¥ç¡®è®¤å…¥ç«™æ¶ˆæ¯ã€‚è®¾ç½®æ—¶é»˜è®¤ä¸ºæ´»è·ƒæ™ºèƒ½ä½“çš„ `identity.emoji`ï¼Œå¦åˆ™ä¸º `"ðŸ‘€"`ã€‚è®¾ä¸º `""` ç¦ç”¨ã€‚

`ackReactionScope` æŽ§åˆ¶ååº”è§¦å‘æ—¶æœºï¼š

- `group-mentions`ï¼ˆé»˜è®¤ï¼‰ï¼šä»…åœ¨ç¾¤ç»„/æˆ¿é—´è¦æ±‚æåŠ**ä¸”**æœºå™¨äººè¢«æåŠæ—¶
- `group-all`ï¼šæ‰€æœ‰ç¾¤ç»„/æˆ¿é—´æ¶ˆæ¯
- `direct`ï¼šä»…ç§èŠæ¶ˆæ¯
- `all`ï¼šæ‰€æœ‰æ¶ˆæ¯

`removeAckAfterReply` åœ¨å‘é€å›žå¤åŽç§»é™¤æœºå™¨äººçš„ç¡®è®¤ååº”ï¼ˆä»… Slack/Discord/Telegram/Google Chatï¼‰ã€‚é»˜è®¤ï¼š`false`ã€‚

#### `messages.tts`

ä¸ºå‡ºç«™å›žå¤å¯ç”¨æ–‡å­—è½¬è¯­éŸ³ã€‚å¼€å¯åŽï¼ŒKrabKrab ä½¿ç”¨ ElevenLabs æˆ– OpenAI ç”ŸæˆéŸ³é¢‘å¹¶é™„åŠ åˆ°å›žå¤ä¸­ã€‚Telegram ä½¿ç”¨ Opus è¯­éŸ³æ¶ˆæ¯ï¼›å…¶ä»–æ¸ é“å‘é€ MP3 éŸ³é¢‘ã€‚

```json5
{
  messages: {
    tts: {
      auto: "always", // off | always | inbound | tagged
      mode: "final", // final | allï¼ˆåŒ…å«å·¥å…·/å—å›žå¤ï¼‰
      provider: "elevenlabs",
      summaryModel: "openai/gpt-4.1-mini",
      modelOverrides: {
        enabled: true,
      },
      maxTextLength: 4000,
      timeoutMs: 30000,
      prefsPath: "~/.krabkrab/settings/tts.json",
      elevenlabs: {
        apiKey: "elevenlabs_api_key",
        baseUrl: "https://api.elevenlabs.io",
        voiceId: "voice_id",
        modelId: "eleven_multilingual_v2",
        seed: 42,
        applyTextNormalization: "auto",
        languageCode: "en",
        voiceSettings: {
          stability: 0.5,
          similarityBoost: 0.75,
          style: 0.0,
          useSpeakerBoost: true,
          speed: 1.0,
        },
      },
      openai: {
        apiKey: "openai_api_key",
        model: "gpt-4o-mini-tts",
        voice: "alloy",
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- `messages.tts.auto` æŽ§åˆ¶è‡ªåŠ¨ TTSï¼ˆ`off`ã€`always`ã€`inbound`ã€`tagged`ï¼‰ã€‚
- `/tts off|always|inbound|tagged` è®¾ç½®æ¯ä¼šè¯çš„è‡ªåŠ¨æ¨¡å¼ï¼ˆè¦†ç›–é…ç½®ï¼‰ã€‚
- `messages.tts.enabled` ä¸ºæ—§ç‰ˆï¼›doctor ä¼šå°†å…¶è¿ç§»ä¸º `messages.tts.auto`ã€‚
- `prefsPath` å­˜å‚¨æœ¬åœ°è¦†ç›–ï¼ˆæä¾›å•†/é™åˆ¶/æ‘˜è¦ï¼‰ã€‚
- `maxTextLength` æ˜¯ TTS è¾“å…¥çš„ç¡¬ä¸Šé™ï¼›æ‘˜è¦ä¼šè¢«æˆªæ–­ä»¥é€‚åº”ã€‚
- `summaryModel` è¦†ç›–è‡ªåŠ¨æ‘˜è¦çš„ `agents.defaults.model.primary`ã€‚
  - æŽ¥å— `provider/model` æˆ–æ¥è‡ª `agents.defaults.models` çš„åˆ«åã€‚
- `modelOverrides` å¯ç”¨æ¨¡åž‹é©±åŠ¨çš„è¦†ç›–å¦‚ `[[tts:...]]` æ ‡ç­¾ï¼ˆé»˜è®¤å¼€å¯ï¼‰ã€‚
- `/tts limit` å’Œ `/tts summary` æŽ§åˆ¶æ¯ç”¨æˆ·çš„æ‘˜è¦è®¾ç½®ã€‚
- `apiKey` å€¼å›žé€€åˆ° `ELEVENLABS_API_KEY`/`XI_API_KEY` å’Œ `OPENAI_API_KEY`ã€‚
- `elevenlabs.baseUrl` è¦†ç›– ElevenLabs API åŸºç¡€ URLã€‚
- `elevenlabs.voiceSettings` æ”¯æŒ `stability`/`similarityBoost`/`style`ï¼ˆ0..1ï¼‰ã€
  `useSpeakerBoost` å’Œ `speed`ï¼ˆ0.5..2.0ï¼‰ã€‚

### `talk`

Talk æ¨¡å¼ï¼ˆmacOS/iOS/Androidï¼‰çš„é»˜è®¤å€¼ã€‚è¯­éŸ³ ID åœ¨æœªè®¾ç½®æ—¶å›žé€€åˆ° `ELEVENLABS_VOICE_ID` æˆ– `SAG_VOICE_ID`ã€‚
`apiKey` åœ¨æœªè®¾ç½®æ—¶å›žé€€åˆ° `ELEVENLABS_API_KEY`ï¼ˆæˆ– Gateway ç½‘å…³çš„ shell é…ç½®æ–‡ä»¶ï¼‰ã€‚
`voiceAliases` å…è®¸ Talk æŒ‡ä»¤ä½¿ç”¨å‹å¥½åç§°ï¼ˆä¾‹å¦‚ `"voice":"Clawd"`ï¼‰ã€‚

```json5
{
  talk: {
    voiceId: "elevenlabs_voice_id",
    voiceAliases: {
      Clawd: "EXAVITQu4vr4xnSDxMaL",
      Roger: "CwhRBWXzGAHq8TQ4Fs17",
    },
    modelId: "eleven_v3",
    outputFormat: "mp3_44100_128",
    apiKey: "elevenlabs_api_key",
    interruptOnSpeech: true,
  },
}
```

### `agents.defaults`

æŽ§åˆ¶å†…ç½®æ™ºèƒ½ä½“è¿è¡Œæ—¶ï¼ˆæ¨¡åž‹/æ€è€ƒ/è¯¦ç»†/è¶…æ—¶ï¼‰ã€‚
`agents.defaults.models` å®šä¹‰å·²é…ç½®çš„æ¨¡åž‹ç›®å½•ï¼ˆä¹Ÿå……å½“ `/model` çš„ç™½åå•ï¼‰ã€‚
`agents.defaults.model.primary` è®¾ç½®é»˜è®¤æ¨¡åž‹ï¼›`agents.defaults.model.fallbacks` æ˜¯å…¨å±€æ•…éšœè½¬ç§»ã€‚
`agents.defaults.imageModel` æ˜¯å¯é€‰çš„ï¼Œ**ä»…åœ¨ä¸»æ¨¡åž‹ç¼ºå°‘å›¾åƒè¾“å…¥æ—¶ä½¿ç”¨**ã€‚
æ¯ä¸ª `agents.defaults.models` æ¡ç›®å¯ä»¥åŒ…å«ï¼š

- `alias`ï¼ˆå¯é€‰çš„æ¨¡åž‹å¿«æ·æ–¹å¼ï¼Œä¾‹å¦‚ `/opus`ï¼‰ã€‚
- `params`ï¼ˆå¯é€‰çš„æä¾›å•†ç‰¹å®š API å‚æ•°ï¼Œä¼ é€’ç»™æ¨¡åž‹è¯·æ±‚ï¼‰ã€‚

`params` ä¹Ÿåº”ç”¨äºŽæµå¼è¿è¡Œï¼ˆå†…ç½®æ™ºèƒ½ä½“ + åŽ‹ç¼©ï¼‰ã€‚ç›®å‰æ”¯æŒçš„é”®ï¼š`temperature`ã€`maxTokens`ã€‚è¿™äº›ä¸Žè°ƒç”¨æ—¶é€‰é¡¹åˆå¹¶ï¼›è°ƒç”¨æ–¹æä¾›çš„å€¼ä¼˜å…ˆã€‚`temperature` æ˜¯é«˜çº§æ—‹é’®â€”â€”é™¤éžä½ äº†è§£æ¨¡åž‹çš„é»˜è®¤å€¼ä¸”éœ€è¦æ›´æ”¹ï¼Œå¦åˆ™ä¸è¦è®¾ç½®ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  agents: {
    defaults: {
      models: {
        "anthropic/claude-sonnet-4-5-20250929": {
          params: { temperature: 0.6 },
        },
        "openai/gpt-5.2": {
          params: { maxTokens: 8192 },
        },
      },
    },
  },
}
```

Z.AI GLM-4.x æ¨¡åž‹ä¼šè‡ªåŠ¨å¯ç”¨æ€è€ƒæ¨¡å¼ï¼Œé™¤éžä½ ï¼š

- è®¾ç½® `--thinking off`ï¼Œæˆ–
- è‡ªè¡Œå®šä¹‰ `agents.defaults.models["zai/<model>"].params.thinking`ã€‚

KrabKrab è¿˜å†…ç½®äº†ä¸€äº›åˆ«åå¿«æ·æ–¹å¼ã€‚é»˜è®¤å€¼ä»…åœ¨æ¨¡åž‹å·²å­˜åœ¨äºŽ `agents.defaults.models` ä¸­æ—¶æ‰åº”ç”¨ï¼š

- `opus` -> `anthropic/claude-opus-4-5`
- `sonnet` -> `anthropic/claude-sonnet-4-5`
- `gpt` -> `openai/gpt-5.2`
- `gpt-mini` -> `openai/gpt-5-mini`
- `gemini` -> `google/gemini-3-pro-preview`
- `gemini-flash` -> `google/gemini-3-flash-preview`

å¦‚æžœä½ é…ç½®äº†ç›¸åŒçš„åˆ«åï¼ˆä¸åŒºåˆ†å¤§å°å†™ï¼‰ï¼Œä½ çš„å€¼ä¼˜å…ˆï¼ˆé»˜è®¤å€¼ä¸ä¼šè¦†ç›–ï¼‰ã€‚

ç¤ºä¾‹ï¼šOpus 4.5 ä¸»æ¨¡åž‹ï¼ŒMiniMax M2.1 å›žé€€ï¼ˆæ‰˜ç®¡ MiniMaxï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      models: {
        "anthropic/claude-opus-4-5": { alias: "opus" },
        "minimax/MiniMax-M2.1": { alias: "minimax" },
      },
      model: {
        primary: "anthropic/claude-opus-4-5",
        fallbacks: ["minimax/MiniMax-M2.1"],
      },
    },
  },
}
```

MiniMax è®¤è¯ï¼šè®¾ç½® `MINIMAX_API_KEY`ï¼ˆçŽ¯å¢ƒå˜é‡ï¼‰æˆ–é…ç½® `models.providers.minimax`ã€‚

#### `agents.defaults.cliBackends`ï¼ˆCLI å›žé€€ï¼‰

å¯é€‰çš„ CLI åŽç«¯ç”¨äºŽçº¯æ–‡æœ¬å›žé€€è¿è¡Œï¼ˆæ— å·¥å…·è°ƒç”¨ï¼‰ã€‚å½“ API æä¾›å•†å¤±è´¥æ—¶å¯ä½œä¸ºå¤‡ç”¨è·¯å¾„ã€‚å½“ä½ é…ç½®äº†æŽ¥å—æ–‡ä»¶è·¯å¾„çš„ `imageArg` æ—¶æ”¯æŒå›¾åƒé€ä¼ ã€‚

è¯´æ˜Žï¼š

- CLI åŽç«¯**ä»¥æ–‡æœ¬ä¸ºä¸»**ï¼›å·¥å…·å§‹ç»ˆç¦ç”¨ã€‚
- è®¾ç½® `sessionArg` æ—¶æ”¯æŒä¼šè¯ï¼›ä¼šè¯ id æŒ‰åŽç«¯æŒä¹…åŒ–ã€‚
- å¯¹äºŽ `claude-cli`ï¼Œé»˜è®¤å€¼å·²å†…ç½®ã€‚å¦‚æžœ PATH ä¸å®Œæ•´ï¼ˆlaunchd/systemdï¼‰ï¼Œè¯·è¦†ç›–å‘½ä»¤è·¯å¾„ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  agents: {
    defaults: {
      cliBackends: {
        "claude-cli": {
          command: "/opt/homebrew/bin/claude",
        },
        "my-cli": {
          command: "my-cli",
          args: ["--json"],
          output: "json",
          modelArg: "--model",
          sessionArg: "--session",
          sessionMode: "existing",
          systemPromptArg: "--system",
          systemPromptWhen: "first",
          imageArg: "--image",
          imageMode: "repeat",
        },
      },
    },
  },
}
```

```json5
{
  agents: {
    defaults: {
      models: {
        "anthropic/claude-opus-4-5": { alias: "Opus" },
        "anthropic/claude-sonnet-4-1": { alias: "Sonnet" },
        "openrouter/deepseek/deepseek-r1:free": {},
        "zai/glm-4.7": {
          alias: "GLM",
          params: {
            thinking: {
              type: "enabled",
              clear_thinking: false,
            },
          },
        },
      },
      model: {
        primary: "anthropic/claude-opus-4-5",
        fallbacks: [
          "openrouter/deepseek/deepseek-r1:free",
          "openrouter/meta-llama/llama-3.3-70b-instruct:free",
        ],
      },
      imageModel: {
        primary: "openrouter/qwen/qwen-2.5-vl-72b-instruct:free",
        fallbacks: ["openrouter/google/gemini-2.0-flash-vision:free"],
      },
      thinkingDefault: "low",
      verboseDefault: "off",
      elevatedDefault: "on",
      timeoutSeconds: 600,
      mediaMaxMb: 5,
      heartbeat: {
        every: "30m",
        target: "last",
      },
      maxConcurrent: 3,
      subagents: {
        model: "minimax/MiniMax-M2.1",
        maxConcurrent: 1,
        archiveAfterMinutes: 60,
      },
      exec: {
        backgroundMs: 10000,
        timeoutSec: 1800,
        cleanupMs: 1800000,
      },
      contextTokens: 200000,
    },
  },
}
```

#### `agents.defaults.contextPruning`ï¼ˆå·¥å…·ç»“æžœè£å‰ªï¼‰

`agents.defaults.contextPruning` åœ¨è¯·æ±‚å‘é€åˆ° LLM ä¹‹å‰è£å‰ªå†…å­˜ä¸Šä¸‹æ–‡ä¸­çš„**æ—§å·¥å…·ç»“æžœ**ã€‚
å®ƒ**ä¸ä¼š**ä¿®æ”¹ç£ç›˜ä¸Šçš„ä¼šè¯åŽ†å²ï¼ˆ`*.jsonl` ä¿æŒå®Œæ•´ï¼‰ã€‚

è¿™æ—¨åœ¨å‡å°‘éšæ—¶é—´ç§¯ç´¯å¤§é‡å·¥å…·è¾“å‡ºçš„æ™ºèƒ½ä½“çš„ token æ¶ˆè€—ã€‚

æ¦‚è¿°ï¼š

- ä¸è§¦åŠç”¨æˆ·/åŠ©æ‰‹æ¶ˆæ¯ã€‚
- ä¿æŠ¤æœ€åŽ `keepLastAssistants` æ¡åŠ©æ‰‹æ¶ˆæ¯ï¼ˆè¯¥ç‚¹ä¹‹åŽçš„å·¥å…·ç»“æžœä¸ä¼šè¢«è£å‰ªï¼‰ã€‚
- ä¿æŠ¤å¼•å¯¼å‰ç¼€ï¼ˆç¬¬ä¸€æ¡ç”¨æˆ·æ¶ˆæ¯ä¹‹å‰çš„å†…å®¹ä¸ä¼šè¢«è£å‰ªï¼‰ã€‚
- æ¨¡å¼ï¼š
  - `adaptive`ï¼šå½“ä¼°è®¡çš„ä¸Šä¸‹æ–‡æ¯”çŽ‡è¶…è¿‡ `softTrimRatio` æ—¶ï¼Œè½¯è£å‰ªè¿‡å¤§çš„å·¥å…·ç»“æžœï¼ˆä¿ç•™å¤´å°¾ï¼‰ã€‚
    ç„¶åŽå½“ä¼°è®¡çš„ä¸Šä¸‹æ–‡æ¯”çŽ‡è¶…è¿‡ `hardClearRatio` **ä¸”**æœ‰è¶³å¤Ÿçš„å¯è£å‰ªå·¥å…·ç»“æžœé‡ï¼ˆ`minPrunableToolChars`ï¼‰æ—¶ï¼Œç¡¬æ¸…é™¤æœ€æ—§çš„ç¬¦åˆæ¡ä»¶çš„å·¥å…·ç»“æžœã€‚
  - `aggressive`ï¼šå§‹ç»ˆç”¨ `hardClear.placeholder` æ›¿æ¢æˆªæ­¢ç‚¹ä¹‹å‰ç¬¦åˆæ¡ä»¶çš„å·¥å…·ç»“æžœï¼ˆä¸åšæ¯”çŽ‡æ£€æŸ¥ï¼‰ã€‚

è½¯è£å‰ª vs ç¡¬è£å‰ªï¼ˆå‘é€ç»™ LLM çš„ä¸Šä¸‹æ–‡ä¸­çš„å˜åŒ–ï¼‰ï¼š

- **è½¯è£å‰ª**ï¼šä»…é’ˆå¯¹*è¿‡å¤§*çš„å·¥å…·ç»“æžœã€‚ä¿ç•™å¼€å¤´ + ç»“å°¾ï¼Œåœ¨ä¸­é—´æ’å…¥ `...`ã€‚
  - ä¹‹å‰ï¼š`toolResult("â€¦å¾ˆé•¿çš„è¾“å‡ºâ€¦")`
  - ä¹‹åŽï¼š`toolResult("HEADâ€¦\n...\nâ€¦TAIL\n\n[Tool result trimmed: â€¦]")`
- **ç¡¬æ¸…é™¤**ï¼šç”¨å ä½ç¬¦æ›¿æ¢æ•´ä¸ªå·¥å…·ç»“æžœã€‚
  - ä¹‹å‰ï¼š`toolResult("â€¦å¾ˆé•¿çš„è¾“å‡ºâ€¦")`
  - ä¹‹åŽï¼š`toolResult("[Old tool result content cleared]")`

è¯´æ˜Ž / å½“å‰é™åˆ¶ï¼š

- ç›®å‰åŒ…å«**å›¾åƒå—çš„å·¥å…·ç»“æžœä¼šè¢«è·³è¿‡**ï¼ˆä¸ä¼šè¢«è£å‰ª/æ¸…é™¤ï¼‰ã€‚
- ä¼°è®¡çš„"ä¸Šä¸‹æ–‡æ¯”çŽ‡"åŸºäºŽ**å­—ç¬¦**ï¼ˆè¿‘ä¼¼å€¼ï¼‰ï¼Œä¸æ˜¯ç²¾ç¡®çš„ token æ•°ã€‚
- å¦‚æžœä¼šè¯å°šæœªåŒ…å«è‡³å°‘ `keepLastAssistants` æ¡åŠ©æ‰‹æ¶ˆæ¯ï¼Œåˆ™è·³è¿‡è£å‰ªã€‚
- åœ¨ `aggressive` æ¨¡å¼ä¸‹ï¼Œ`hardClear.enabled` è¢«å¿½ç•¥ï¼ˆç¬¦åˆæ¡ä»¶çš„å·¥å…·ç»“æžœå§‹ç»ˆè¢«æ›¿æ¢ä¸º `hardClear.placeholder`ï¼‰ã€‚

é»˜è®¤å€¼ï¼ˆadaptiveï¼‰ï¼š

```json5
{
  agents: { defaults: { contextPruning: { mode: "adaptive" } } },
}
```

ç¦ç”¨ï¼š

```json5
{
  agents: { defaults: { contextPruning: { mode: "off" } } },
}
```

é»˜è®¤å€¼ï¼ˆå½“ `mode` ä¸º `"adaptive"` æˆ– `"aggressive"` æ—¶ï¼‰ï¼š

- `keepLastAssistants`ï¼š`3`
- `softTrimRatio`ï¼š`0.3`ï¼ˆä»… adaptiveï¼‰
- `hardClearRatio`ï¼š`0.5`ï¼ˆä»… adaptiveï¼‰
- `minPrunableToolChars`ï¼š`50000`ï¼ˆä»… adaptiveï¼‰
- `softTrim`ï¼š`{ maxChars: 4000, headChars: 1500, tailChars: 1500 }`ï¼ˆä»… adaptiveï¼‰
- `hardClear`ï¼š`{ enabled: true, placeholder: "[Old tool result content cleared]" }`

ç¤ºä¾‹ï¼ˆaggressiveï¼Œæœ€å°åŒ–ï¼‰ï¼š

```json5
{
  agents: { defaults: { contextPruning: { mode: "aggressive" } } },
}
```

ç¤ºä¾‹ï¼ˆè°ƒä¼˜çš„ adaptiveï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      contextPruning: {
        mode: "adaptive",
        keepLastAssistants: 3,
        softTrimRatio: 0.3,
        hardClearRatio: 0.5,
        minPrunableToolChars: 50000,
        softTrim: { maxChars: 4000, headChars: 1500, tailChars: 1500 },
        hardClear: { enabled: true, placeholder: "[Old tool result content cleared]" },
        // å¯é€‰ï¼šé™åˆ¶è£å‰ªä»…é’ˆå¯¹ç‰¹å®šå·¥å…·ï¼ˆdeny ä¼˜å…ˆï¼›æ”¯æŒ "*" é€šé…ç¬¦ï¼‰
        tools: { deny: ["browser", "canvas"] },
      },
    },
  },
}
```

å‚è§ [/concepts/session-pruning](/concepts/session-pruning) äº†è§£è¡Œä¸ºç»†èŠ‚ã€‚

#### `agents.defaults.compaction`ï¼ˆé¢„ç•™ç©ºé—´ + è®°å¿†åˆ·æ–°ï¼‰

`agents.defaults.compaction.mode` é€‰æ‹©åŽ‹ç¼©æ‘˜è¦ç­–ç•¥ã€‚é»˜è®¤ä¸º `default`ï¼›è®¾ä¸º `safeguard` å¯ä¸ºè¶…é•¿åŽ†å²å¯ç”¨åˆ†å—æ‘˜è¦ã€‚å‚è§ [/concepts/compaction](/concepts/compaction)ã€‚

`agents.defaults.compaction.reserveTokensFloor` ä¸º Pi åŽ‹ç¼©å¼ºåˆ¶ä¸€ä¸ªæœ€å° `reserveTokens` å€¼ï¼ˆé»˜è®¤ï¼š`20000`ï¼‰ã€‚è®¾ä¸º `0` ç¦ç”¨æ­¤åº•çº¿ã€‚

`agents.defaults.compaction.memoryFlush` åœ¨è‡ªåŠ¨åŽ‹ç¼©å‰è¿è¡Œä¸€ä¸ª**é™é»˜**æ™ºèƒ½ä½“è½®æ¬¡ï¼ŒæŒ‡ç¤ºæ¨¡åž‹å°†æŒä¹…è®°å¿†å­˜å‚¨åˆ°ç£ç›˜ï¼ˆä¾‹å¦‚ `memory/YYYY-MM-DD.md`ï¼‰ã€‚å½“ä¼šè¯ token ä¼°è®¡å€¼è¶…è¿‡åŽ‹ç¼©é™åˆ¶ä»¥ä¸‹çš„è½¯é˜ˆå€¼æ—¶è§¦å‘ã€‚

æ—§ç‰ˆé»˜è®¤å€¼ï¼š

- `memoryFlush.enabled`ï¼š`true`
- `memoryFlush.softThresholdTokens`ï¼š`4000`
- `memoryFlush.prompt` / `memoryFlush.systemPrompt`ï¼šå¸¦ `NO_REPLY` çš„å†…ç½®é»˜è®¤å€¼
- æ³¨æ„ï¼šå½“ä¼šè¯å·¥ä½œåŒºä¸ºåªè¯»æ—¶è·³è¿‡è®°å¿†åˆ·æ–°ï¼ˆ`agents.defaults.sandbox.workspaceAccess: "ro"` æˆ– `"none"`ï¼‰ã€‚

ç¤ºä¾‹ï¼ˆè°ƒä¼˜ï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      compaction: {
        mode: "safeguard",
        reserveTokensFloor: 24000,
        memoryFlush: {
          enabled: true,
          softThresholdTokens: 6000,
          systemPrompt: "Session nearing compaction. Store durable memories now.",
          prompt: "Write any lasting notes to memory/YYYY-MM-DD.md; reply with NO_REPLY if nothing to store.",
        },
      },
    },
  },
}
```

åˆ†å—æµå¼ä¼ è¾“ï¼š

- `agents.defaults.blockStreamingDefault`ï¼š`"on"`/`"off"`ï¼ˆé»˜è®¤ offï¼‰ã€‚
- æ¸ é“è¦†ç›–ï¼š`*.blockStreaming`ï¼ˆåŠæ¯è´¦å·å˜ä½“ï¼‰å¼ºåˆ¶åˆ†å—æµå¼ä¼ è¾“å¼€/å…³ã€‚
  éž Telegram æ¸ é“éœ€è¦æ˜¾å¼è®¾ç½® `*.blockStreaming: true` æ¥å¯ç”¨å—å›žå¤ã€‚
- `agents.defaults.blockStreamingBreak`ï¼š`"text_end"` æˆ– `"message_end"`ï¼ˆé»˜è®¤ï¼štext_endï¼‰ã€‚
- `agents.defaults.blockStreamingChunk`ï¼šæµå¼å—çš„è½¯åˆ†å—ã€‚é»˜è®¤ 800â€“1200 å­—ç¬¦ï¼Œä¼˜å…ˆæ®µè½åˆ†éš”ï¼ˆ`\n\n`ï¼‰ï¼Œç„¶åŽæ¢è¡Œï¼Œç„¶åŽå¥å­ã€‚
  ç¤ºä¾‹ï¼š
  ```json5
  {
    agents: { defaults: { blockStreamingChunk: { minChars: 800, maxChars: 1200 } } },
  }
  ```
- `agents.defaults.blockStreamingCoalesce`ï¼šå‘é€å‰åˆå¹¶æµå¼å—ã€‚
  é»˜è®¤ä¸º `{ idleMs: 1000 }`ï¼Œä»Ž `blockStreamingChunk` ç»§æ‰¿ `minChars`ï¼Œ
  `maxChars` ä¸Šé™ä¸ºæ¸ é“æ–‡æœ¬é™åˆ¶ã€‚Signal/Slack/Discord/Google Chat é»˜è®¤
  `minChars: 1500`ï¼Œé™¤éžè¢«è¦†ç›–ã€‚
  æ¸ é“è¦†ç›–ï¼š`channels.whatsapp.blockStreamingCoalesce`ã€`channels.telegram.blockStreamingCoalesce`ã€
  `channels.discord.blockStreamingCoalesce`ã€`channels.slack.blockStreamingCoalesce`ã€`channels.mattermost.blockStreamingCoalesce`ã€
  `channels.signal.blockStreamingCoalesce`ã€`channels.imessage.blockStreamingCoalesce`ã€`channels.msteams.blockStreamingCoalesce`ã€
  `channels.googlechat.blockStreamingCoalesce`
  ï¼ˆåŠæ¯è´¦å·å˜ä½“ï¼‰ã€‚
- `agents.defaults.humanDelay`ï¼šç¬¬ä¸€æ¡ä¹‹åŽ**å—å›žå¤**ä¹‹é—´çš„éšæœºå»¶è¿Ÿã€‚
  æ¨¡å¼ï¼š`off`ï¼ˆé»˜è®¤ï¼‰ã€`natural`ï¼ˆ800â€“2500msï¼‰ã€`custom`ï¼ˆä½¿ç”¨ `minMs`/`maxMs`ï¼‰ã€‚
  æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼š`agents.list[].humanDelay`ã€‚
  ç¤ºä¾‹ï¼š
  ```json5
  {
    agents: { defaults: { humanDelay: { mode: "natural" } } },
  }
  ```
  å‚è§ [/concepts/streaming](/concepts/streaming) äº†è§£è¡Œä¸º + åˆ†å—ç»†èŠ‚ã€‚

è¾“å…¥æŒ‡ç¤ºå™¨ï¼š

- `agents.defaults.typingMode`ï¼š`"never" | "instant" | "thinking" | "message"`ã€‚ç§èŠ/æåŠé»˜è®¤ä¸º
  `instant`ï¼Œæœªè¢«æåŠçš„ç¾¤èŠé»˜è®¤ä¸º `message`ã€‚
- `session.typingMode`ï¼šæ¯ä¼šè¯çš„æ¨¡å¼è¦†ç›–ã€‚
- `agents.defaults.typingIntervalSeconds`ï¼šè¾“å…¥ä¿¡å·åˆ·æ–°é¢‘çŽ‡ï¼ˆé»˜è®¤ï¼š6sï¼‰ã€‚
- `session.typingIntervalSeconds`ï¼šæ¯ä¼šè¯çš„åˆ·æ–°é—´éš”è¦†ç›–ã€‚
  å‚è§ [/concepts/typing-indicators](/concepts/typing-indicators) äº†è§£è¡Œä¸ºç»†èŠ‚ã€‚

`agents.defaults.model.primary` åº”è®¾ä¸º `provider/model`ï¼ˆä¾‹å¦‚ `anthropic/claude-opus-4-5`ï¼‰ã€‚
åˆ«åæ¥è‡ª `agents.defaults.models.*.alias`ï¼ˆä¾‹å¦‚ `Opus`ï¼‰ã€‚
å¦‚æžœçœç•¥æä¾›å•†ï¼ŒKrabKrab ç›®å‰å‡å®š `anthropic` ä½œä¸ºä¸´æ—¶å¼ƒç”¨å›žé€€ã€‚
Z.AI æ¨¡åž‹å¯é€šè¿‡ `zai/<model>` ä½¿ç”¨ï¼ˆä¾‹å¦‚ `zai/glm-4.7`ï¼‰ï¼Œéœ€è¦çŽ¯å¢ƒä¸­è®¾ç½®
`ZAI_API_KEY`ï¼ˆæˆ–æ—§ç‰ˆ `Z_AI_API_KEY`ï¼‰ã€‚

`agents.defaults.heartbeat` é…ç½®å®šæœŸå¿ƒè·³è¿è¡Œï¼š

- `every`ï¼šæŒç»­æ—¶é—´å­—ç¬¦ä¸²ï¼ˆ`ms`ã€`s`ã€`m`ã€`h`ï¼‰ï¼›é»˜è®¤å•ä½åˆ†é’Ÿã€‚é»˜è®¤ï¼š
  `30m`ã€‚è®¾ä¸º `0m` ç¦ç”¨ã€‚
- `model`ï¼šå¯é€‰çš„å¿ƒè·³è¿è¡Œè¦†ç›–æ¨¡åž‹ï¼ˆ`provider/model`ï¼‰ã€‚
- `includeReasoning`ï¼šä¸º `true` æ—¶ï¼Œå¿ƒè·³ä¹Ÿä¼šä¼ é€’å•ç‹¬çš„ `Reasoning:` æ¶ˆæ¯ï¼ˆä¸Ž `/reasoning on` ç›¸åŒå½¢å¼ï¼‰ã€‚é»˜è®¤ï¼š`false`ã€‚
- `session`ï¼šå¯é€‰çš„ä¼šè¯é”®ï¼ŒæŽ§åˆ¶å¿ƒè·³åœ¨å“ªä¸ªä¼šè¯ä¸­è¿è¡Œã€‚é»˜è®¤ï¼š`main`ã€‚
- `to`ï¼šå¯é€‰çš„æ”¶ä»¶äººè¦†ç›–ï¼ˆæ¸ é“ç‰¹å®š idï¼Œä¾‹å¦‚ WhatsApp çš„ E.164ï¼ŒTelegram çš„èŠå¤© idï¼‰ã€‚
- `target`ï¼šå¯é€‰çš„æŠ•é€’æ¸ é“ï¼ˆ`last`ã€`whatsapp`ã€`telegram`ã€`discord`ã€`slack`ã€`msteams`ã€`signal`ã€`imessage`ã€`none`ï¼‰ã€‚é»˜è®¤ï¼š`last`ã€‚
- `prompt`ï¼šå¯é€‰çš„å¿ƒè·³å†…å®¹è¦†ç›–ï¼ˆé»˜è®¤ï¼š`Read HEARTBEAT.md if it exists (workspace context). Follow it strictly. Do not infer or repeat old tasks from prior chats. If nothing needs attention, reply HEARTBEAT_OK.`ï¼‰ã€‚è¦†ç›–å€¼æŒ‰åŽŸæ ·å‘é€ï¼›å¦‚æžœä»éœ€è¯»å–æ–‡ä»¶ï¼Œè¯·åŒ…å« `Read HEARTBEAT.md` è¡Œã€‚
- `ackMaxChars`ï¼š`HEARTBEAT_OK` ä¹‹åŽæŠ•é€’å‰å…è®¸çš„æœ€å¤§å­—ç¬¦æ•°ï¼ˆé»˜è®¤ï¼š300ï¼‰ã€‚

æ¯æ™ºèƒ½ä½“å¿ƒè·³ï¼š

- è®¾ç½® `agents.list[].heartbeat` ä¸ºç‰¹å®šæ™ºèƒ½ä½“å¯ç”¨æˆ–è¦†ç›–å¿ƒè·³è®¾ç½®ã€‚
- å¦‚æžœä»»ä½•æ™ºèƒ½ä½“æ¡ç›®å®šä¹‰äº† `heartbeat`ï¼Œ**ä»…é‚£äº›æ™ºèƒ½ä½“**è¿è¡Œå¿ƒè·³ï¼›é»˜è®¤å€¼
  æˆä¸ºé‚£äº›æ™ºèƒ½ä½“çš„å…±äº«åŸºçº¿ã€‚

å¿ƒè·³è¿è¡Œå®Œæ•´çš„æ™ºèƒ½ä½“è½®æ¬¡ã€‚è¾ƒçŸ­çš„é—´éš”æ¶ˆè€—æ›´å¤š tokenï¼›è¯·æ³¨æ„
`every`ï¼Œä¿æŒ `HEARTBEAT.md` ç²¾ç®€ï¼Œå’Œ/æˆ–é€‰æ‹©æ›´ä¾¿å®œçš„ `model`ã€‚

`tools.exec` é…ç½®åŽå°æ‰§è¡Œé»˜è®¤å€¼ï¼š

- `backgroundMs`ï¼šè‡ªåŠ¨åŽå°åŒ–å‰çš„æ—¶é—´ï¼ˆmsï¼Œé»˜è®¤ 10000ï¼‰
- `timeoutSec`ï¼šè¶…è¿‡æ­¤è¿è¡Œæ—¶é—´åŽè‡ªåŠ¨ç»ˆæ­¢ï¼ˆç§’ï¼Œé»˜è®¤ 1800ï¼‰
- `cleanupMs`ï¼šå®Œæˆçš„ä¼šè¯åœ¨å†…å­˜ä¸­ä¿ç•™å¤šä¹…ï¼ˆmsï¼Œé»˜è®¤ 1800000ï¼‰
- `notifyOnExit`ï¼šåŽå°æ‰§è¡Œé€€å‡ºæ—¶åŠ å…¥ç³»ç»Ÿäº‹ä»¶ + è¯·æ±‚å¿ƒè·³ï¼ˆé»˜è®¤ trueï¼‰
- `applyPatch.enabled`ï¼šå¯ç”¨å®žéªŒæ€§ `apply_patch`ï¼ˆä»… OpenAI/OpenAI Codexï¼›é»˜è®¤ falseï¼‰
- `applyPatch.allowModels`ï¼šå¯é€‰çš„æ¨¡åž‹ id ç™½åå•ï¼ˆä¾‹å¦‚ `gpt-5.2` æˆ– `openai/gpt-5.2`ï¼‰
  æ³¨æ„ï¼š`applyPatch` ä»…åœ¨ `tools.exec` ä¸‹ã€‚

`tools.web` é…ç½® Web æœç´¢ + èŽ·å–å·¥å…·ï¼š

- `tools.web.search.enabled`ï¼ˆé»˜è®¤ï¼šæœ‰å¯†é’¥æ—¶ä¸º trueï¼‰
- `tools.web.search.apiKey`ï¼ˆæŽ¨èï¼šé€šè¿‡ `krabkrab configure --section web` è®¾ç½®ï¼Œæˆ–ä½¿ç”¨ `BRAVE_API_KEY` çŽ¯å¢ƒå˜é‡ï¼‰
- `tools.web.search.maxResults`ï¼ˆ1â€“10ï¼Œé»˜è®¤ 5ï¼‰
- `tools.web.search.timeoutSeconds`ï¼ˆé»˜è®¤ 30ï¼‰
- `tools.web.search.cacheTtlMinutes`ï¼ˆé»˜è®¤ 15ï¼‰
- `tools.web.fetch.enabled`ï¼ˆé»˜è®¤ trueï¼‰
- `tools.web.fetch.maxChars`ï¼ˆé»˜è®¤ 50000ï¼‰
- `tools.web.fetch.timeoutSeconds`ï¼ˆé»˜è®¤ 30ï¼‰
- `tools.web.fetch.cacheTtlMinutes`ï¼ˆé»˜è®¤ 15ï¼‰
- `tools.web.fetch.userAgent`ï¼ˆå¯é€‰è¦†ç›–ï¼‰
- `tools.web.fetch.readability`ï¼ˆé»˜è®¤ trueï¼›ç¦ç”¨åŽä»…ä½¿ç”¨åŸºæœ¬ HTML æ¸…ç†ï¼‰
- `tools.web.fetch.firecrawl.enabled`ï¼ˆé»˜è®¤ï¼šè®¾ç½®äº† API å¯†é’¥æ—¶ä¸º trueï¼‰
- `tools.web.fetch.firecrawl.apiKey`ï¼ˆå¯é€‰ï¼›é»˜è®¤ä¸º `FIRECRAWL_API_KEY`ï¼‰
- `tools.web.fetch.firecrawl.baseUrl`ï¼ˆé»˜è®¤ https://api.firecrawl.devï¼‰
- `tools.web.fetch.firecrawl.onlyMainContent`ï¼ˆé»˜è®¤ trueï¼‰
- `tools.web.fetch.firecrawl.maxAgeMs`ï¼ˆå¯é€‰ï¼‰
- `tools.web.fetch.firecrawl.timeoutSeconds`ï¼ˆå¯é€‰ï¼‰

`tools.media` é…ç½®å…¥ç«™åª’ä½“ç†è§£ï¼ˆå›¾ç‰‡/éŸ³é¢‘/è§†é¢‘ï¼‰ï¼š

- `tools.media.models`ï¼šå…±äº«æ¨¡åž‹åˆ—è¡¨ï¼ˆæŒ‰èƒ½åŠ›æ ‡è®°ï¼›åœ¨æ¯èƒ½åŠ›åˆ—è¡¨ä¹‹åŽä½¿ç”¨ï¼‰ã€‚
- `tools.media.concurrency`ï¼šæœ€å¤§å¹¶å‘èƒ½åŠ›è¿è¡Œæ•°ï¼ˆé»˜è®¤ 2ï¼‰ã€‚
- `tools.media.image` / `tools.media.audio` / `tools.media.video`ï¼š
  - `enabled`ï¼šé€‰æ‹©é€€å‡ºå¼€å…³ï¼ˆé…ç½®äº†æ¨¡åž‹æ—¶é»˜è®¤ä¸º trueï¼‰ã€‚
  - `prompt`ï¼šå¯é€‰çš„æç¤ºè¦†ç›–ï¼ˆå›¾ç‰‡/è§†é¢‘è‡ªåŠ¨é™„åŠ  `maxChars` æç¤ºï¼‰ã€‚
  - `maxChars`ï¼šæœ€å¤§è¾“å‡ºå­—ç¬¦æ•°ï¼ˆå›¾ç‰‡/è§†é¢‘é»˜è®¤ 500ï¼›éŸ³é¢‘æœªè®¾ç½®ï¼‰ã€‚
  - `maxBytes`ï¼šå‘é€çš„æœ€å¤§åª’ä½“å¤§å°ï¼ˆé»˜è®¤ï¼šå›¾ç‰‡ 10MBï¼ŒéŸ³é¢‘ 20MBï¼Œè§†é¢‘ 50MBï¼‰ã€‚
  - `timeoutSeconds`ï¼šè¯·æ±‚è¶…æ—¶ï¼ˆé»˜è®¤ï¼šå›¾ç‰‡ 60sï¼ŒéŸ³é¢‘ 60sï¼Œè§†é¢‘ 120sï¼‰ã€‚
  - `language`ï¼šå¯é€‰çš„éŸ³é¢‘æç¤ºã€‚
  - `attachments`ï¼šé™„ä»¶ç­–ç•¥ï¼ˆ`mode`ã€`maxAttachments`ã€`prefer`ï¼‰ã€‚
  - `scope`ï¼šå¯é€‰çš„é—¨æŽ§ï¼ˆç¬¬ä¸€ä¸ªåŒ¹é…èŽ·èƒœï¼‰ï¼Œå¸¦ `match.channel`ã€`match.chatType` æˆ– `match.keyPrefix`ã€‚
  - `models`ï¼šæœ‰åºçš„æ¨¡åž‹æ¡ç›®åˆ—è¡¨ï¼›å¤±è´¥æˆ–è¶…å¤§åª’ä½“å›žé€€åˆ°ä¸‹ä¸€ä¸ªæ¡ç›®ã€‚
- æ¯ä¸ª `models[]` æ¡ç›®ï¼š
  - æä¾›å•†æ¡ç›®ï¼ˆ`type: "provider"` æˆ–çœç•¥ï¼‰ï¼š
    - `provider`ï¼šAPI æä¾›å•† idï¼ˆ`openai`ã€`anthropic`ã€`google`/`gemini`ã€`groq` ç­‰ï¼‰ã€‚
    - `model`ï¼šæ¨¡åž‹ id è¦†ç›–ï¼ˆå›¾ç‰‡å¿…éœ€ï¼›éŸ³é¢‘æä¾›å•†é»˜è®¤ä¸º `gpt-4o-mini-transcribe`/`whisper-large-v3-turbo`ï¼Œè§†é¢‘é»˜è®¤ä¸º `gemini-3-flash-preview`ï¼‰ã€‚
    - `profile` / `preferredProfile`ï¼šè®¤è¯é…ç½®æ–‡ä»¶é€‰æ‹©ã€‚
  - CLI æ¡ç›®ï¼ˆ`type: "cli"`ï¼‰ï¼š
    - `command`ï¼šè¦è¿è¡Œçš„å¯æ‰§è¡Œæ–‡ä»¶ã€‚
    - `args`ï¼šæ¨¡æ¿åŒ–å‚æ•°ï¼ˆæ”¯æŒ `{{MediaPath}}`ã€`{{Prompt}}`ã€`{{MaxChars}}` ç­‰ï¼‰ã€‚
  - `capabilities`ï¼šå¯é€‰çš„èƒ½åŠ›åˆ—è¡¨ï¼ˆ`image`ã€`audio`ã€`video`ï¼‰ç”¨äºŽé—¨æŽ§å…±äº«æ¡ç›®ã€‚çœç•¥æ—¶çš„é»˜è®¤å€¼ï¼š`openai`/`anthropic`/`minimax` â†’ imageï¼Œ`google` â†’ image+audio+videoï¼Œ`groq` â†’ audioã€‚
  - `prompt`ã€`maxChars`ã€`maxBytes`ã€`timeoutSeconds`ã€`language` å¯åœ¨æ¯ä¸ªæ¡ç›®ä¸­è¦†ç›–ã€‚

å¦‚æžœæœªé…ç½®æ¨¡åž‹ï¼ˆæˆ– `enabled: false`ï¼‰ï¼Œç†è§£å°†è¢«è·³è¿‡ï¼›æ¨¡åž‹ä»ä¼šæŽ¥æ”¶åŽŸå§‹é™„ä»¶ã€‚

æä¾›å•†è®¤è¯éµå¾ªæ ‡å‡†æ¨¡åž‹è®¤è¯é¡ºåºï¼ˆè®¤è¯é…ç½®æ–‡ä»¶ã€çŽ¯å¢ƒå˜é‡å¦‚ `OPENAI_API_KEY`/`GROQ_API_KEY`/`GEMINI_API_KEY`ï¼Œæˆ– `models.providers.*.apiKey`ï¼‰ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  tools: {
    media: {
      audio: {
        enabled: true,
        maxBytes: 20971520,
        scope: {
          default: "deny",
          rules: [{ action: "allow", match: { chatType: "direct" } }],
        },
        models: [
          { provider: "openai", model: "gpt-4o-mini-transcribe" },
          { type: "cli", command: "whisper", args: ["--model", "base", "{{MediaPath}}"] },
        ],
      },
      video: {
        enabled: true,
        maxBytes: 52428800,
        models: [{ provider: "google", model: "gemini-3-flash-preview" }],
      },
    },
  },
}
```

`agents.defaults.subagents` é…ç½®å­æ™ºèƒ½ä½“é»˜è®¤å€¼ï¼š

- `model`ï¼šç”Ÿæˆçš„å­æ™ºèƒ½ä½“çš„é»˜è®¤æ¨¡åž‹ï¼ˆå­—ç¬¦ä¸²æˆ– `{ primary, fallbacks }`ï¼‰ã€‚å¦‚æžœçœç•¥ï¼Œå­æ™ºèƒ½ä½“ç»§æ‰¿è°ƒç”¨è€…çš„æ¨¡åž‹ï¼Œé™¤éžæŒ‰æ™ºèƒ½ä½“æˆ–æŒ‰è°ƒç”¨è¦†ç›–ã€‚
- `maxConcurrent`ï¼šæœ€å¤§å¹¶å‘å­æ™ºèƒ½ä½“è¿è¡Œæ•°ï¼ˆé»˜è®¤ 1ï¼‰
- `archiveAfterMinutes`ï¼šN åˆ†é’ŸåŽè‡ªåŠ¨å½’æ¡£å­æ™ºèƒ½ä½“ä¼šè¯ï¼ˆé»˜è®¤ 60ï¼›è®¾ä¸º `0` ç¦ç”¨ï¼‰
- æ¯å­æ™ºèƒ½ä½“å·¥å…·ç­–ç•¥ï¼š`tools.subagents.tools.allow` / `tools.subagents.tools.deny`ï¼ˆdeny ä¼˜å…ˆï¼‰

`tools.profile` è®¾ç½® `tools.allow`/`tools.deny` ä¹‹å‰çš„**åŸºç¡€å·¥å…·ç™½åå•**ï¼š

- `minimal`ï¼šä»… `session_status`
- `coding`ï¼š`group:fs`ã€`group:runtime`ã€`group:sessions`ã€`group:memory`ã€`image`
- `messaging`ï¼š`group:messaging`ã€`sessions_list`ã€`sessions_history`ã€`sessions_send`ã€`session_status`
- `full`ï¼šæ— é™åˆ¶ï¼ˆä¸Žæœªè®¾ç½®ç›¸åŒï¼‰

æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼š`agents.list[].tools.profile`ã€‚

ç¤ºä¾‹ï¼ˆé»˜è®¤ä»…æ¶ˆæ¯ä¼ é€’ï¼Œå¦å¤–å…è®¸ Slack + Discord å·¥å…·ï¼‰ï¼š

```json5
{
  tools: {
    profile: "messaging",
    allow: ["slack", "discord"],
  },
}
```

ç¤ºä¾‹ï¼ˆç¼–ç é…ç½®æ–‡ä»¶ï¼Œä½†å…¨å±€æ‹’ç» exec/processï¼‰ï¼š

```json5
{
  tools: {
    profile: "coding",
    deny: ["group:runtime"],
  },
}
```

`tools.byProvider` å…è®¸ä½ ä¸ºç‰¹å®šæä¾›å•†ï¼ˆæˆ–å•ä¸ª `provider/model`ï¼‰**è¿›ä¸€æ­¥é™åˆ¶**å·¥å…·ã€‚
æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼š`agents.list[].tools.byProvider`ã€‚

é¡ºåºï¼šåŸºç¡€é…ç½®æ–‡ä»¶ â†’ æä¾›å•†é…ç½®æ–‡ä»¶ â†’ allow/deny ç­–ç•¥ã€‚
æä¾›å•†é”®æŽ¥å— `provider`ï¼ˆä¾‹å¦‚ `google-antigravity`ï¼‰æˆ– `provider/model`
ï¼ˆä¾‹å¦‚ `openai/gpt-5.2`ï¼‰ã€‚

ç¤ºä¾‹ï¼ˆä¿æŒå…¨å±€ç¼–ç é…ç½®æ–‡ä»¶ï¼Œä½†ä¸º Google Antigravity ä½¿ç”¨æœ€å°å·¥å…·ï¼‰ï¼š

```json5
{
  tools: {
    profile: "coding",
    byProvider: {
      "google-antigravity": { profile: "minimal" },
    },
  },
}
```

ç¤ºä¾‹ï¼ˆæä¾›å•†/æ¨¡åž‹ç‰¹å®šç™½åå•ï¼‰ï¼š

```json5
{
  tools: {
    allow: ["group:fs", "group:runtime", "sessions_list"],
    byProvider: {
      "openai/gpt-5.2": { allow: ["group:fs", "sessions_list"] },
    },
  },
}
```

`tools.allow` / `tools.deny` é…ç½®å…¨å±€å·¥å…·å…è®¸/æ‹’ç»ç­–ç•¥ï¼ˆdeny ä¼˜å…ˆï¼‰ã€‚
åŒ¹é…ä¸åŒºåˆ†å¤§å°å†™å¹¶æ”¯æŒ `*` é€šé…ç¬¦ï¼ˆ`"*"` è¡¨ç¤ºæ‰€æœ‰å·¥å…·ï¼‰ã€‚
å³ä½¿ Docker æ²™ç®±**å…³é—­**ï¼Œæ­¤ç­–ç•¥ä¹Ÿä¼šåº”ç”¨ã€‚

ç¤ºä¾‹ï¼ˆå…¨å±€ç¦ç”¨ browser/canvasï¼‰ï¼š

```json5
{
  tools: { deny: ["browser", "canvas"] },
}
```

å·¥å…·ç»„ï¼ˆç®€å†™ï¼‰åœ¨**å…¨å±€**å’Œ**æ¯æ™ºèƒ½ä½“**å·¥å…·ç­–ç•¥ä¸­å¯ç”¨ï¼š

- `group:runtime`ï¼š`exec`ã€`bash`ã€`process`
- `group:fs`ï¼š`read`ã€`write`ã€`edit`ã€`apply_patch`
- `group:sessions`ï¼š`sessions_list`ã€`sessions_history`ã€`sessions_send`ã€`sessions_spawn`ã€`session_status`
- `group:memory`ï¼š`memory_search`ã€`memory_get`
- `group:web`ï¼š`web_search`ã€`web_fetch`
- `group:ui`ï¼š`browser`ã€`canvas`
- `group:automation`ï¼š`cron`ã€`gateway`
- `group:messaging`ï¼š`message`
- `group:nodes`ï¼š`nodes`
- `group:krabkrab`ï¼šæ‰€æœ‰å†…ç½® KrabKrab å·¥å…·ï¼ˆä¸åŒ…å«æä¾›å•†æ’ä»¶ï¼‰

`tools.elevated` æŽ§åˆ¶æå‡ï¼ˆä¸»æœºï¼‰æ‰§è¡Œè®¿é—®ï¼š

- `enabled`ï¼šå…è®¸æå‡æ¨¡å¼ï¼ˆé»˜è®¤ trueï¼‰
- `allowFrom`ï¼šæ¯æ¸ é“ç™½åå•ï¼ˆç©º = ç¦ç”¨ï¼‰
  - `whatsapp`ï¼šE.164 å·ç 
  - `telegram`ï¼šèŠå¤© id æˆ–ç”¨æˆ·å
  - `discord`ï¼šç”¨æˆ· id æˆ–ç”¨æˆ·åï¼ˆçœç•¥æ—¶å›žé€€åˆ° `channels.discord.dm.allowFrom`ï¼‰
  - `signal`ï¼šE.164 å·ç 
  - `imessage`ï¼šå¥æŸ„/èŠå¤© id
  - `webchat`ï¼šä¼šè¯ id æˆ–ç”¨æˆ·å

ç¤ºä¾‹ï¼š

```json5
{
  tools: {
    elevated: {
      enabled: true,
      allowFrom: {
        whatsapp: ["+15555550123"],
        discord: ["steipete", "1234567890123"],
      },
    },
  },
}
```

æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼ˆè¿›ä¸€æ­¥é™åˆ¶ï¼‰ï¼š

```json5
{
  agents: {
    list: [
      {
        id: "family",
        tools: {
          elevated: { enabled: false },
        },
      },
    ],
  },
}
```

è¯´æ˜Žï¼š

- `tools.elevated` æ˜¯å…¨å±€åŸºçº¿ã€‚`agents.list[].tools.elevated` åªèƒ½è¿›ä¸€æ­¥é™åˆ¶ï¼ˆä¸¤è€…éƒ½å¿…é¡»å…è®¸ï¼‰ã€‚
- `/elevated on|off|ask|full` æŒ‰ä¼šè¯é”®å­˜å‚¨çŠ¶æ€ï¼›å†…è”æŒ‡ä»¤ä»…åº”ç”¨äºŽå•æ¡æ¶ˆæ¯ã€‚
- æå‡çš„ `exec` åœ¨ä¸»æœºä¸Šè¿è¡Œå¹¶ç»•è¿‡æ²™ç®±ã€‚
- å·¥å…·ç­–ç•¥ä»ç„¶é€‚ç”¨ï¼›å¦‚æžœ `exec` è¢«æ‹’ç»ï¼Œåˆ™æ— æ³•ä½¿ç”¨æå‡ã€‚

`agents.defaults.maxConcurrent` è®¾ç½®è·¨ä¼šè¯å¯å¹¶è¡Œæ‰§è¡Œçš„å†…ç½®æ™ºèƒ½ä½“è¿è¡Œçš„æœ€å¤§æ•°é‡ã€‚æ¯ä¸ªä¼šè¯ä»ç„¶æ˜¯ä¸²è¡Œçš„ï¼ˆæ¯ä¸ªä¼šè¯é”®åŒæ—¶åªæœ‰ä¸€ä¸ªè¿è¡Œï¼‰ã€‚é»˜è®¤ï¼š1ã€‚

### `agents.defaults.sandbox`

ä¸ºå†…ç½®æ™ºèƒ½ä½“æä¾›å¯é€‰çš„ **Docker æ²™ç®±**ã€‚é€‚ç”¨äºŽéžä¸»ä¼šè¯ï¼Œä½¿å…¶æ— æ³•è®¿é—®ä½ çš„ä¸»æœºç³»ç»Ÿã€‚

è¯¦æƒ…ï¼š[æ²™ç®±](/gateway/sandboxing)

é»˜è®¤å€¼ï¼ˆå¦‚æžœå¯ç”¨ï¼‰ï¼š

- scopeï¼š`"agent"`ï¼ˆæ¯ä¸ªæ™ºèƒ½ä½“ä¸€ä¸ªå®¹å™¨ + å·¥ä½œåŒºï¼‰
- åŸºäºŽ Debian bookworm-slim çš„é•œåƒ
- æ™ºèƒ½ä½“å·¥ä½œåŒºè®¿é—®ï¼š`workspaceAccess: "none"`ï¼ˆé»˜è®¤ï¼‰
  - `"none"`ï¼šåœ¨ `~/.krabkrab/sandboxes` ä¸‹ä½¿ç”¨æ¯èŒƒå›´çš„æ²™ç®±å·¥ä½œåŒº
- `"ro"`ï¼šå°†æ²™ç®±å·¥ä½œåŒºä¿æŒåœ¨ `/workspace`ï¼Œæ™ºèƒ½ä½“å·¥ä½œåŒºä»¥åªè¯»æ–¹å¼æŒ‚è½½åˆ° `/agent`ï¼ˆç¦ç”¨ `write`/`edit`/`apply_patch`ï¼‰
  - `"rw"`ï¼šå°†æ™ºèƒ½ä½“å·¥ä½œåŒºä»¥è¯»å†™æ–¹å¼æŒ‚è½½åˆ° `/workspace`
- è‡ªåŠ¨æ¸…ç†ï¼šç©ºé—²è¶…è¿‡ 24h æˆ–å­˜åœ¨è¶…è¿‡ 7d
- å·¥å…·ç­–ç•¥ï¼šä»…å…è®¸ `exec`ã€`process`ã€`read`ã€`write`ã€`edit`ã€`apply_patch`ã€`sessions_list`ã€`sessions_history`ã€`sessions_send`ã€`sessions_spawn`ã€`session_status`ï¼ˆdeny ä¼˜å…ˆï¼‰
  - é€šè¿‡ `tools.sandbox.tools` é…ç½®ï¼Œé€šè¿‡ `agents.list[].tools.sandbox.tools` è¿›è¡Œæ¯æ™ºèƒ½ä½“è¦†ç›–
  - æ²™ç®±ç­–ç•¥ä¸­æ”¯æŒå·¥å…·ç»„ç®€å†™ï¼š`group:runtime`ã€`group:fs`ã€`group:sessions`ã€`group:memory`ï¼ˆå‚è§[æ²™ç®± vs å·¥å…·ç­–ç•¥ vs æå‡](/gateway/sandbox-vs-tool-policy-vs-elevated#tool-groups-shorthands)ï¼‰
- å¯é€‰çš„æ²™ç®±æµè§ˆå™¨ï¼ˆChromium + CDPï¼ŒnoVNC è§‚å¯Ÿå™¨ï¼‰
- åŠ å›ºæ—‹é’®ï¼š`network`ã€`user`ã€`pidsLimit`ã€`memory`ã€`cpus`ã€`ulimits`ã€`seccompProfile`ã€`apparmorProfile`

è­¦å‘Šï¼š`scope: "shared"` æ„å‘³ç€å…±äº«å®¹å™¨å’Œå…±äº«å·¥ä½œåŒºã€‚æ— è·¨ä¼šè¯éš”ç¦»ã€‚ä½¿ç”¨ `scope: "session"` èŽ·å¾—æ¯ä¼šè¯éš”ç¦»ã€‚

æ—§ç‰ˆï¼š`perSession` ä»ç„¶æ”¯æŒï¼ˆ`true` â†’ `scope: "session"`ï¼Œ`false` â†’ `scope: "shared"`ï¼‰ã€‚

`setupCommand` åœ¨å®¹å™¨åˆ›å»ºåŽ**è¿è¡Œä¸€æ¬¡**ï¼ˆåœ¨å®¹å™¨å†…é€šè¿‡ `sh -lc` æ‰§è¡Œï¼‰ã€‚
å¯¹äºŽåŒ…å®‰è£…ï¼Œç¡®ä¿ç½‘ç»œå‡ºå£ã€å¯å†™æ ¹æ–‡ä»¶ç³»ç»Ÿå’Œ root ç”¨æˆ·ã€‚

```json5
{
  agents: {
    defaults: {
      sandbox: {
        mode: "non-main", // off | non-main | all
        scope: "agent", // session | agent | sharedï¼ˆagent ä¸ºé»˜è®¤ï¼‰
        workspaceAccess: "none", // none | ro | rw
        workspaceRoot: "~/.krabkrab/sandboxes",
        docker: {
          image: "krabkrab-sandbox:bookworm-slim",
          containerPrefix: "krabkrab-sbx-",
          workdir: "/workspace",
          readOnlyRoot: true,
          tmpfs: ["/tmp", "/var/tmp", "/run"],
          network: "none",
          user: "1000:1000",
          capDrop: ["ALL"],
          env: { LANG: "C.UTF-8" },
          setupCommand: "apt-get update && apt-get install -y git curl jq",
          // æ¯æ™ºèƒ½ä½“è¦†ç›–ï¼ˆå¤šæ™ºèƒ½ä½“ï¼‰ï¼šagents.list[].sandbox.docker.*
          pidsLimit: 256,
          memory: "1g",
          memorySwap: "2g",
          cpus: 1,
          ulimits: {
            nofile: { soft: 1024, hard: 2048 },
            nproc: 256,
          },
          seccompProfile: "/path/to/seccomp.json",
          apparmorProfile: "krabkrab-sandbox",
          dns: ["1.1.1.1", "8.8.8.8"],
          extraHosts: ["internal.service:10.0.0.5"],
          binds: ["/var/run/docker.sock:/var/run/docker.sock", "/home/user/source:/source:rw"],
        },
        browser: {
          enabled: false,
          image: "krabkrab-sandbox-browser:bookworm-slim",
          containerPrefix: "krabkrab-sbx-browser-",
          cdpPort: 9222,
          vncPort: 5900,
          noVncPort: 6080,
          headless: false,
          enableNoVnc: true,
          allowHostControl: false,
          allowedControlUrls: ["http://10.0.0.42:18791"],
          allowedControlHosts: ["browser.lab.local", "10.0.0.42"],
          allowedControlPorts: [18791],
          autoStart: true,
          autoStartTimeoutMs: 12000,
        },
        prune: {
          idleHours: 24, // 0 ç¦ç”¨ç©ºé—²æ¸…ç†
          maxAgeDays: 7, // 0 ç¦ç”¨æœ€å¤§å­˜æ´»æ—¶é—´æ¸…ç†
        },
      },
    },
  },
  tools: {
    sandbox: {
      tools: {
        allow: [
          "exec",
          "process",
          "read",
          "write",
          "edit",
          "apply_patch",
          "sessions_list",
          "sessions_history",
          "sessions_send",
          "sessions_spawn",
          "session_status",
        ],
        deny: ["browser", "canvas", "nodes", "cron", "discord", "gateway"],
      },
    },
  },
}
```

é¦–æ¬¡æž„å»ºé»˜è®¤æ²™ç®±é•œåƒï¼š

```bash
scripts/sandbox-setup.sh
```

æ³¨æ„ï¼šæ²™ç®±å®¹å™¨é»˜è®¤ä¸º `network: "none"`ï¼›å¦‚æžœæ™ºèƒ½ä½“éœ€è¦å‡ºç«™è®¿é—®ï¼Œè¯·å°† `agents.defaults.sandbox.docker.network` è®¾ä¸º `"bridge"`ï¼ˆæˆ–ä½ çš„è‡ªå®šä¹‰ç½‘ç»œï¼‰ã€‚

æ³¨æ„ï¼šå…¥ç«™é™„ä»¶ä¼šæš‚å­˜åˆ°æ´»è·ƒå·¥ä½œåŒºçš„ `media/inbound/*` ä¸­ã€‚ä½¿ç”¨ `workspaceAccess: "rw"` æ—¶ï¼Œæ–‡ä»¶ä¼šå†™å…¥æ™ºèƒ½ä½“å·¥ä½œåŒºã€‚

æ³¨æ„ï¼š`docker.binds` æŒ‚è½½é¢å¤–çš„ä¸»æœºç›®å½•ï¼›å…¨å±€å’Œæ¯æ™ºèƒ½ä½“çš„ binds ä¼šåˆå¹¶ã€‚

æž„å»ºå¯é€‰çš„æµè§ˆå™¨é•œåƒï¼š

```bash
scripts/sandbox-browser-setup.sh
```

å½“ `agents.defaults.sandbox.browser.enabled=true` æ—¶ï¼Œæµè§ˆå™¨å·¥å…·ä½¿ç”¨æ²™ç®±åŒ–çš„
Chromium å®žä¾‹ï¼ˆCDPï¼‰ã€‚å¦‚æžœå¯ç”¨äº† noVNCï¼ˆheadless=false æ—¶é»˜è®¤å¯ç”¨ï¼‰ï¼Œ
noVNC URL ä¼šæ³¨å…¥ç³»ç»Ÿæç¤ºä¸­ï¼Œä»¥ä¾¿æ™ºèƒ½ä½“å¯ä»¥å¼•ç”¨å®ƒã€‚
è¿™ä¸éœ€è¦ä¸»é…ç½®ä¸­çš„ `browser.enabled`ï¼›æ²™ç®±æŽ§åˆ¶ URL æŒ‰ä¼šè¯æ³¨å…¥ã€‚

`agents.defaults.sandbox.browser.allowHostControl`ï¼ˆé»˜è®¤ï¼šfalseï¼‰å…è®¸
æ²™ç®±ä¼šè¯é€šè¿‡æµè§ˆå™¨å·¥å…·æ˜¾å¼è®¿é—®**ä¸»æœº**æµè§ˆå™¨æŽ§åˆ¶æœåŠ¡å™¨
ï¼ˆ`target: "host"`ï¼‰ã€‚å¦‚æžœä½ éœ€è¦ä¸¥æ ¼çš„æ²™ç®±éš”ç¦»ï¼Œè¯·ä¿æŒå…³é—­ã€‚

è¿œç¨‹æŽ§åˆ¶ç™½åå•ï¼š

- `allowedControlUrls`ï¼š`target: "custom"` å…è®¸çš„ç²¾ç¡®æŽ§åˆ¶ URLã€‚
- `allowedControlHosts`ï¼šå…è®¸çš„ä¸»æœºåï¼ˆä»…ä¸»æœºåï¼Œæ— ç«¯å£ï¼‰ã€‚
- `allowedControlPorts`ï¼šå…è®¸çš„ç«¯å£ï¼ˆé»˜è®¤ï¼šhttp=80ï¼Œhttps=443ï¼‰ã€‚
  é»˜è®¤ï¼šæ‰€æœ‰ç™½åå•æœªè®¾ç½®ï¼ˆæ— é™åˆ¶ï¼‰ã€‚`allowHostControl` é»˜è®¤ä¸º falseã€‚

### `models`ï¼ˆè‡ªå®šä¹‰æä¾›å•† + åŸºç¡€ URLï¼‰

KrabKrab ä½¿ç”¨ **pi-coding-agent** æ¨¡åž‹ç›®å½•ã€‚ä½ å¯ä»¥é€šè¿‡ç¼–å†™
`~/.krabkrab/agents/<agentId>/agent/models.json` æˆ–åœ¨ KrabKrab é…ç½®ä¸­çš„ `models.providers` ä¸‹å®šä¹‰ç›¸åŒçš„ schema æ¥æ·»åŠ è‡ªå®šä¹‰æä¾›å•†ï¼ˆLiteLLMã€æœ¬åœ° OpenAI å…¼å®¹æœåŠ¡å™¨ã€Anthropic ä»£ç†ç­‰ï¼‰ã€‚
æŒ‰æä¾›å•†çš„æ¦‚è¿° + ç¤ºä¾‹ï¼š[/concepts/model-providers](/concepts/model-providers)ã€‚

å½“å­˜åœ¨ `models.providers` æ—¶ï¼ŒKrabKrab åœ¨å¯åŠ¨æ—¶å°† `models.json` å†™å…¥/åˆå¹¶åˆ°
`~/.krabkrab/agents/<agentId>/agent/`ï¼š

- é»˜è®¤è¡Œä¸ºï¼š**åˆå¹¶**ï¼ˆä¿ç•™çŽ°æœ‰æä¾›å•†ï¼ŒæŒ‰åç§°è¦†ç›–ï¼‰
- è®¾ä¸º `models.mode: "replace"` è¦†ç›–æ–‡ä»¶å†…å®¹

é€šè¿‡ `agents.defaults.model.primary`ï¼ˆprovider/modelï¼‰é€‰æ‹©æ¨¡åž‹ã€‚

```json5
{
  agents: {
    defaults: {
      model: { primary: "custom-proxy/llama-3.1-8b" },
      models: {
        "custom-proxy/llama-3.1-8b": {},
      },
    },
  },
  models: {
    mode: "merge",
    providers: {
      "custom-proxy": {
        baseUrl: "http://localhost:4000/v1",
        apiKey: "LITELLM_KEY",
        api: "openai-completions",
        models: [
          {
            id: "llama-3.1-8b",
            name: "Llama 3.1 8B",
            reasoning: false,
            input: ["text"],
            cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 },
            contextWindow: 128000,
            maxTokens: 32000,
          },
        ],
      },
    },
  },
}
```

### OpenCode Zenï¼ˆå¤šæ¨¡åž‹ä»£ç†ï¼‰

OpenCode Zen æ˜¯ä¸€ä¸ªå…·æœ‰æ¯æ¨¡åž‹ç«¯ç‚¹çš„å¤šæ¨¡åž‹ç½‘å…³ã€‚KrabKrab ä½¿ç”¨
pi-ai å†…ç½®çš„ `opencode` æä¾›å•†ï¼›ä»Ž https://opencode.ai/auth è®¾ç½® `OPENCODE_API_KEY`ï¼ˆæˆ–
`OPENCODE_ZEN_API_KEY`ï¼‰ã€‚

è¯´æ˜Žï¼š

- æ¨¡åž‹å¼•ç”¨ä½¿ç”¨ `opencode/<modelId>`ï¼ˆç¤ºä¾‹ï¼š`opencode/claude-opus-4-5`ï¼‰ã€‚
- å¦‚æžœä½ é€šè¿‡ `agents.defaults.models` å¯ç”¨ç™½åå•ï¼Œè¯·æ·»åŠ ä½ è®¡åˆ’ä½¿ç”¨çš„æ¯ä¸ªæ¨¡åž‹ã€‚
- å¿«æ·æ–¹å¼ï¼š`krabkrab onboard --auth-choice opencode-zen`ã€‚

```json5
{
  agents: {
    defaults: {
      model: { primary: "opencode/claude-opus-4-5" },
      models: { "opencode/claude-opus-4-5": { alias: "Opus" } },
    },
  },
}
```

### Z.AIï¼ˆGLM-4.7ï¼‰â€” æä¾›å•†åˆ«åæ”¯æŒ

Z.AI æ¨¡åž‹é€šè¿‡å†…ç½®çš„ `zai` æä¾›å•†æä¾›ã€‚åœ¨çŽ¯å¢ƒä¸­è®¾ç½® `ZAI_API_KEY`
å¹¶é€šè¿‡ provider/model å¼•ç”¨æ¨¡åž‹ã€‚

å¿«æ·æ–¹å¼ï¼š`krabkrab onboard --auth-choice zai-api-key`ã€‚

```json5
{
  agents: {
    defaults: {
      model: { primary: "zai/glm-4.7" },
      models: { "zai/glm-4.7": {} },
    },
  },
}
```

è¯´æ˜Žï¼š

- `z.ai/*` å’Œ `z-ai/*` æ˜¯æŽ¥å—çš„åˆ«åï¼Œè§„èŒƒåŒ–ä¸º `zai/*`ã€‚
- å¦‚æžœç¼ºå°‘ `ZAI_API_KEY`ï¼Œå¯¹ `zai/*` çš„è¯·æ±‚å°†åœ¨è¿è¡Œæ—¶å› è®¤è¯é”™è¯¯å¤±è´¥ã€‚
- ç¤ºä¾‹é”™è¯¯ï¼š`No API key found for provider "zai".`
- Z.AI çš„é€šç”¨ API ç«¯ç‚¹æ˜¯ `https://api.z.ai/api/paas/v4`ã€‚GLM ç¼–ç 
  è¯·æ±‚ä½¿ç”¨ä¸“ç”¨ç¼–ç ç«¯ç‚¹ `https://api.z.ai/api/coding/paas/v4`ã€‚
  å†…ç½®çš„ `zai` æä¾›å•†ä½¿ç”¨ç¼–ç ç«¯ç‚¹ã€‚å¦‚æžœä½ éœ€è¦é€šç”¨
  ç«¯ç‚¹ï¼Œè¯·åœ¨ `models.providers` ä¸­å®šä¹‰è‡ªå®šä¹‰æä¾›å•†å¹¶è¦†ç›–åŸºç¡€ URL
  ï¼ˆå‚è§ä¸Šæ–¹è‡ªå®šä¹‰æä¾›å•†éƒ¨åˆ†ï¼‰ã€‚
- åœ¨æ–‡æ¡£/é…ç½®ä¸­ä½¿ç”¨å‡å ä½ç¬¦ï¼›åˆ‡å‹¿æäº¤çœŸå®ž API å¯†é’¥ã€‚

### Moonshot AIï¼ˆKimiï¼‰

ä½¿ç”¨ Moonshot çš„ OpenAI å…¼å®¹ç«¯ç‚¹ï¼š

```json5
{
  env: { MOONSHOT_API_KEY: "sk-..." },
  agents: {
    defaults: {
      model: { primary: "moonshot/kimi-k2.5" },
      models: { "moonshot/kimi-k2.5": { alias: "Kimi K2.5" } },
    },
  },
  models: {
    mode: "merge",
    providers: {
      moonshot: {
        baseUrl: "https://api.moonshot.ai/v1",
        apiKey: "${MOONSHOT_API_KEY}",
        api: "openai-completions",
        models: [
          {
            id: "kimi-k2.5",
            name: "Kimi K2.5",
            reasoning: false,
            input: ["text"],
            cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 },
            contextWindow: 256000,
            maxTokens: 8192,
          },
        ],
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- åœ¨çŽ¯å¢ƒä¸­è®¾ç½® `MOONSHOT_API_KEY` æˆ–ä½¿ç”¨ `krabkrab onboard --auth-choice moonshot-api-key`ã€‚
- æ¨¡åž‹å¼•ç”¨ï¼š`moonshot/kimi-k2.5`ã€‚
- å¦‚éœ€ä¸­å›½ç«¯ç‚¹ï¼Œä½¿ç”¨ `https://api.moonshot.cn/v1`ã€‚

### Kimi Coding

ä½¿ç”¨ Moonshot AI çš„ Kimi Coding ç«¯ç‚¹ï¼ˆAnthropic å…¼å®¹ï¼Œå†…ç½®æä¾›å•†ï¼‰ï¼š

```json5
{
  env: { KIMI_API_KEY: "sk-..." },
  agents: {
    defaults: {
      model: { primary: "kimi-coding/k2p5" },
      models: { "kimi-coding/k2p5": { alias: "Kimi K2.5" } },
    },
  },
}
```

è¯´æ˜Žï¼š

- åœ¨çŽ¯å¢ƒä¸­è®¾ç½® `KIMI_API_KEY` æˆ–ä½¿ç”¨ `krabkrab onboard --auth-choice kimi-code-api-key`ã€‚
- æ¨¡åž‹å¼•ç”¨ï¼š`kimi-coding/k2p5`ã€‚

### Syntheticï¼ˆAnthropic å…¼å®¹ï¼‰

ä½¿ç”¨ Synthetic çš„ Anthropic å…¼å®¹ç«¯ç‚¹ï¼š

```json5
{
  env: { SYNTHETIC_API_KEY: "sk-..." },
  agents: {
    defaults: {
      model: { primary: "synthetic/hf:MiniMaxAI/MiniMax-M2.1" },
      models: { "synthetic/hf:MiniMaxAI/MiniMax-M2.1": { alias: "MiniMax M2.1" } },
    },
  },
  models: {
    mode: "merge",
    providers: {
      synthetic: {
        baseUrl: "https://api.synthetic.new/anthropic",
        apiKey: "${SYNTHETIC_API_KEY}",
        api: "anthropic-messages",
        models: [
          {
            id: "hf:MiniMaxAI/MiniMax-M2.1",
            name: "MiniMax M2.1",
            reasoning: false,
            input: ["text"],
            cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 },
            contextWindow: 192000,
            maxTokens: 65536,
          },
        ],
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- è®¾ç½® `SYNTHETIC_API_KEY` æˆ–ä½¿ç”¨ `krabkrab onboard --auth-choice synthetic-api-key`ã€‚
- æ¨¡åž‹å¼•ç”¨ï¼š`synthetic/hf:MiniMaxAI/MiniMax-M2.1`ã€‚
- åŸºç¡€ URL åº”çœç•¥ `/v1`ï¼Œå› ä¸º Anthropic å®¢æˆ·ç«¯ä¼šè‡ªåŠ¨é™„åŠ ã€‚

### æœ¬åœ°æ¨¡åž‹ï¼ˆLM Studioï¼‰â€” æŽ¨èè®¾ç½®

å‚è§ [/gateway/local-models](/gateway/local-models) äº†è§£å½“å‰æœ¬åœ°æŒ‡å—ã€‚ç®€è€Œè¨€ä¹‹ï¼šåœ¨é«˜æ€§èƒ½ç¡¬ä»¶ä¸Šé€šè¿‡ LM Studio Responses API è¿è¡Œ MiniMax M2.1ï¼›ä¿ç•™æ‰˜ç®¡æ¨¡åž‹åˆå¹¶ä½œä¸ºå›žé€€ã€‚

### MiniMax M2.1

ä¸é€šè¿‡ LM Studio ç›´æŽ¥ä½¿ç”¨ MiniMax M2.1ï¼š

```json5
{
  agent: {
    model: { primary: "minimax/MiniMax-M2.1" },
    models: {
      "anthropic/claude-opus-4-5": { alias: "Opus" },
      "minimax/MiniMax-M2.1": { alias: "Minimax" },
    },
  },
  models: {
    mode: "merge",
    providers: {
      minimax: {
        baseUrl: "https://api.minimax.io/anthropic",
        apiKey: "${MINIMAX_API_KEY}",
        api: "anthropic-messages",
        models: [
          {
            id: "MiniMax-M2.1",
            name: "MiniMax M2.1",
            reasoning: false,
            input: ["text"],
            // å®šä»·ï¼šå¦‚éœ€ç²¾ç¡®è´¹ç”¨è·Ÿè¸ªï¼Œè¯·åœ¨ models.json ä¸­æ›´æ–°ã€‚
            cost: { input: 15, output: 60, cacheRead: 2, cacheWrite: 10 },
            contextWindow: 200000,
            maxTokens: 8192,
          },
        ],
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- è®¾ç½® `MINIMAX_API_KEY` çŽ¯å¢ƒå˜é‡æˆ–ä½¿ç”¨ `krabkrab onboard --auth-choice minimax-api`ã€‚
- å¯ç”¨æ¨¡åž‹ï¼š`MiniMax-M2.1`ï¼ˆé»˜è®¤ï¼‰ã€‚
- å¦‚éœ€ç²¾ç¡®è´¹ç”¨è·Ÿè¸ªï¼Œè¯·åœ¨ `models.json` ä¸­æ›´æ–°å®šä»·ã€‚

### Cerebrasï¼ˆGLM 4.6 / 4.7ï¼‰

é€šè¿‡ Cerebras çš„ OpenAI å…¼å®¹ç«¯ç‚¹ä½¿ç”¨ï¼š

```json5
{
  env: { CEREBRAS_API_KEY: "sk-..." },
  agents: {
    defaults: {
      model: {
        primary: "cerebras/zai-glm-4.7",
        fallbacks: ["cerebras/zai-glm-4.6"],
      },
      models: {
        "cerebras/zai-glm-4.7": { alias: "GLM 4.7 (Cerebras)" },
        "cerebras/zai-glm-4.6": { alias: "GLM 4.6 (Cerebras)" },
      },
    },
  },
  models: {
    mode: "merge",
    providers: {
      cerebras: {
        baseUrl: "https://api.cerebras.ai/v1",
        apiKey: "${CEREBRAS_API_KEY}",
        api: "openai-completions",
        models: [
          { id: "zai-glm-4.7", name: "GLM 4.7 (Cerebras)" },
          { id: "zai-glm-4.6", name: "GLM 4.6 (Cerebras)" },
        ],
      },
    },
  },
}
```

è¯´æ˜Žï¼š

- Cerebras ä½¿ç”¨ `cerebras/zai-glm-4.7`ï¼›Z.AI ç›´è¿žä½¿ç”¨ `zai/glm-4.7`ã€‚
- åœ¨çŽ¯å¢ƒæˆ–é…ç½®ä¸­è®¾ç½® `CEREBRAS_API_KEY`ã€‚

è¯´æ˜Žï¼š

- æ”¯æŒçš„ APIï¼š`openai-completions`ã€`openai-responses`ã€`anthropic-messages`ã€
  `google-generative-ai`
- å¯¹äºŽè‡ªå®šä¹‰è®¤è¯éœ€æ±‚ä½¿ç”¨ `authHeader: true` + `headers`ã€‚
- å¦‚æžœä½ å¸Œæœ› `models.json` å­˜å‚¨åœ¨å…¶ä»–ä½ç½®ï¼Œè¯·ä½¿ç”¨ `krabkrab_AGENT_DIR`ï¼ˆæˆ– `PI_CODING_AGENT_DIR`ï¼‰è¦†ç›–æ™ºèƒ½ä½“é…ç½®æ ¹ç›®å½•ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/agents/main/agent`ï¼‰ã€‚

### `session`

æŽ§åˆ¶ä¼šè¯ä½œç”¨åŸŸã€é‡ç½®ç­–ç•¥ã€é‡ç½®è§¦å‘å™¨ä»¥åŠä¼šè¯å­˜å‚¨çš„å†™å…¥ä½ç½®ã€‚

```json5
{
  session: {
    scope: "per-sender",
    dmScope: "main",
    identityLinks: {
      alice: ["telegram:123456789", "discord:987654321012345678"],
    },
    reset: {
      mode: "daily",
      atHour: 4,
      idleMinutes: 60,
    },
    resetByType: {
      thread: { mode: "daily", atHour: 4 },
      dm: { mode: "idle", idleMinutes: 240 },
      group: { mode: "idle", idleMinutes: 120 },
    },
    resetTriggers: ["/new", "/reset"],
    // é»˜è®¤å·²æŒ‰æ™ºèƒ½ä½“å­˜å‚¨åœ¨ ~/.krabkrab/agents/<agentId>/sessions/sessions.json
    // ä½ å¯ä»¥ä½¿ç”¨ {agentId} æ¨¡æ¿è¿›è¡Œè¦†ç›–ï¼š
    store: "~/.krabkrab/agents/{agentId}/sessions/sessions.json",
    // ç§èŠæŠ˜å åˆ° agent:<agentId>:<mainKey>ï¼ˆé»˜è®¤ï¼š"main"ï¼‰ã€‚
    mainKey: "main",
    agentToAgent: {
      // è¯·æ±‚è€…/ç›®æ ‡ä¹‹é—´çš„æœ€å¤§ä¹’ä¹“å›žå¤è½®æ¬¡ï¼ˆ0â€“5ï¼‰ã€‚
      maxPingPongTurns: 5,
    },
    sendPolicy: {
      rules: [{ action: "deny", match: { channel: "discord", chatType: "group" } }],
      default: "allow",
    },
  },
}
```

å­—æ®µï¼š

- `mainKey`ï¼šç§èŠæ¡¶é”®ï¼ˆé»˜è®¤ï¼š`"main"`ï¼‰ã€‚å½“ä½ æƒ³"é‡å‘½å"ä¸»ç§èŠçº¿ç¨‹è€Œä¸æ›´æ”¹ `agentId` æ—¶æœ‰ç”¨ã€‚
  - æ²™ç®±è¯´æ˜Žï¼š`agents.defaults.sandbox.mode: "non-main"` ä½¿ç”¨æ­¤é”®æ£€æµ‹ä¸»ä¼šè¯ã€‚ä»»ä½•ä¸åŒ¹é… `mainKey` çš„ä¼šè¯é”®ï¼ˆç¾¤ç»„/é¢‘é“ï¼‰éƒ½ä¼šè¢«æ²™ç®±åŒ–ã€‚
- `dmScope`ï¼šç§èŠä¼šè¯å¦‚ä½•åˆ†ç»„ï¼ˆé»˜è®¤ï¼š`"main"`ï¼‰ã€‚
  - `main`ï¼šæ‰€æœ‰ç§èŠå…±äº«ä¸»ä¼šè¯ä»¥ä¿æŒè¿žç»­æ€§ã€‚
  - `per-peer`ï¼šæŒ‰å‘é€è€… id è·¨æ¸ é“éš”ç¦»ç§èŠã€‚
  - `per-channel-peer`ï¼šæŒ‰æ¸ é“ + å‘é€è€…éš”ç¦»ç§èŠï¼ˆæŽ¨èç”¨äºŽå¤šç”¨æˆ·æ”¶ä»¶ç®±ï¼‰ã€‚
  - `per-account-channel-peer`ï¼šæŒ‰è´¦å· + æ¸ é“ + å‘é€è€…éš”ç¦»ç§èŠï¼ˆæŽ¨èç”¨äºŽå¤šè´¦å·æ”¶ä»¶ç®±ï¼‰ã€‚
- `identityLinks`ï¼šå°†è§„èŒƒ id æ˜ å°„åˆ°æä¾›å•†å‰ç¼€çš„å¯¹ç­‰æ–¹ï¼Œä»¥ä¾¿åœ¨ä½¿ç”¨ `per-peer`ã€`per-channel-peer` æˆ– `per-account-channel-peer` æ—¶åŒä¸€äººè·¨æ¸ é“å…±äº«ç§èŠä¼šè¯ã€‚
  - ç¤ºä¾‹ï¼š`alice: ["telegram:123456789", "discord:987654321012345678"]`ã€‚
- `reset`ï¼šä¸»é‡ç½®ç­–ç•¥ã€‚é»˜è®¤ä¸º Gateway ç½‘å…³ä¸»æœºä¸Šæœ¬åœ°æ—¶é—´å‡Œæ™¨ 4:00 æ¯æ—¥é‡ç½®ã€‚
  - `mode`ï¼š`daily` æˆ– `idle`ï¼ˆå½“å­˜åœ¨ `reset` æ—¶é»˜è®¤ï¼š`daily`ï¼‰ã€‚
  - `atHour`ï¼šæœ¬åœ°å°æ—¶ï¼ˆ0-23ï¼‰ä½œä¸ºæ¯æ—¥é‡ç½®è¾¹ç•Œã€‚
  - `idleMinutes`ï¼šæ»‘åŠ¨ç©ºé—²çª—å£ï¼ˆåˆ†é’Ÿï¼‰ã€‚å½“ daily + idle éƒ½é…ç½®æ—¶ï¼Œå…ˆåˆ°æœŸçš„èŽ·èƒœã€‚
- `resetByType`ï¼š`dm`ã€`group` å’Œ `thread` çš„æ¯ä¼šè¯è¦†ç›–ã€‚
  - å¦‚æžœä½ åªè®¾ç½®äº†æ—§ç‰ˆ `session.idleMinutes` è€Œæ²¡æœ‰ä»»ä½• `reset`/`resetByType`ï¼ŒKrabKrab ä¿æŒä»…ç©ºé—²æ¨¡å¼ä»¥å‘åŽå…¼å®¹ã€‚
- `heartbeatIdleMinutes`ï¼šå¯é€‰çš„å¿ƒè·³æ£€æŸ¥ç©ºé—²è¦†ç›–ï¼ˆå¯ç”¨æ—¶æ¯æ—¥é‡ç½®ä»ç„¶é€‚ç”¨ï¼‰ã€‚
- `agentToAgent.maxPingPongTurns`ï¼šè¯·æ±‚è€…/ç›®æ ‡ä¹‹é—´çš„æœ€å¤§å›žå¤è½®æ¬¡ï¼ˆ0â€“5ï¼Œé»˜è®¤ 5ï¼‰ã€‚
- `sendPolicy.default`ï¼šæ— è§„åˆ™åŒ¹é…æ—¶çš„ `allow` æˆ– `deny` å›žé€€ã€‚
- `sendPolicy.rules[]`ï¼šæŒ‰ `channel`ã€`chatType`ï¼ˆ`direct|group|room`ï¼‰æˆ– `keyPrefix`ï¼ˆä¾‹å¦‚ `cron:`ï¼‰åŒ¹é…ã€‚ç¬¬ä¸€ä¸ª deny èŽ·èƒœï¼›å¦åˆ™ allowã€‚

### `skills`ï¼ˆSkills é…ç½®ï¼‰

æŽ§åˆ¶å†…ç½®ç™½åå•ã€å®‰è£…åå¥½ã€é¢å¤– Skills æ–‡ä»¶å¤¹å’Œæ¯ Skills è¦†ç›–ã€‚é€‚ç”¨äºŽ**å†…ç½®**Skills å’Œ `~/.krabkrab/skills`ï¼ˆå·¥ä½œåŒº Skills åœ¨åç§°å†²çªæ—¶ä»ç„¶ä¼˜å…ˆï¼‰ã€‚

å­—æ®µï¼š

- `allowBundled`ï¼šå¯é€‰çš„**ä»…å†…ç½®**Skills ç™½åå•ã€‚å¦‚æžœè®¾ç½®ï¼Œä»…é‚£äº›å†…ç½® Skills ç¬¦åˆæ¡ä»¶ï¼ˆç®¡ç†/å·¥ä½œåŒº Skills ä¸å—å½±å“ï¼‰ã€‚
- `load.extraDirs`ï¼šé¢å¤–è¦æ‰«æçš„ Skills ç›®å½•ï¼ˆæœ€ä½Žä¼˜å…ˆçº§ï¼‰ã€‚
- `install.preferBrew`ï¼šå¯ç”¨æ—¶ä¼˜å…ˆä½¿ç”¨ brew å®‰è£…ç¨‹åºï¼ˆé»˜è®¤ï¼štrueï¼‰ã€‚
- `install.nodeManager`ï¼šnode å®‰è£…åå¥½ï¼ˆ`npm` | `pnpm` | `yarn`ï¼Œé»˜è®¤ï¼šnpmï¼‰ã€‚
- `entries.<skillKey>`ï¼šæ¯ Skills é…ç½®è¦†ç›–ã€‚

æ¯ Skills å­—æ®µï¼š

- `enabled`ï¼šè®¾ä¸º `false` ç¦ç”¨ Skillsï¼Œå³ä½¿å®ƒæ˜¯å†…ç½®/å·²å®‰è£…çš„ã€‚
- `env`ï¼šä¸ºæ™ºèƒ½ä½“è¿è¡Œæ³¨å…¥çš„çŽ¯å¢ƒå˜é‡ï¼ˆä»…åœ¨å°šæœªè®¾ç½®æ—¶ï¼‰ã€‚
- `apiKey`ï¼šå¯¹äºŽå£°æ˜Žäº†ä¸»çŽ¯å¢ƒå˜é‡çš„ Skills çš„å¯é€‰ä¾¿åˆ©å­—æ®µï¼ˆä¾‹å¦‚ `nano-banana-pro` â†’ `GEMINI_API_KEY`ï¼‰ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  skills: {
    allowBundled: ["gemini", "peekaboo"],
    load: {
      extraDirs: ["~/Projects/agent-scripts/skills", "~/Projects/oss/some-skill-pack/skills"],
    },
    install: {
      preferBrew: true,
      nodeManager: "npm",
    },
    entries: {
      "nano-banana-pro": {
        apiKey: "GEMINI_KEY_HERE",
        env: {
          GEMINI_API_KEY: "GEMINI_KEY_HERE",
        },
      },
      peekaboo: { enabled: true },
      sag: { enabled: false },
    },
  },
}
```

### `plugins`ï¼ˆæ‰©å±•ï¼‰

æŽ§åˆ¶æ’ä»¶å‘çŽ°ã€å…è®¸/æ‹’ç»å’Œæ¯æ’ä»¶é…ç½®ã€‚æ’ä»¶ä»Ž `~/.krabkrab/extensions`ã€`<workspace>/.krabkrab/extensions` ä»¥åŠä»»ä½• `plugins.load.paths` æ¡ç›®åŠ è½½ã€‚**é…ç½®æ›´æ”¹éœ€è¦é‡å¯ Gateway ç½‘å…³ã€‚**
å‚è§ [/plugin](/tools/plugin) äº†è§£è¯¦æƒ…ã€‚

å­—æ®µï¼š

- `enabled`ï¼šæ’ä»¶åŠ è½½çš„ä¸»å¼€å…³ï¼ˆé»˜è®¤ï¼štrueï¼‰ã€‚
- `allow`ï¼šå¯é€‰çš„æ’ä»¶ id ç™½åå•ï¼›è®¾ç½®åŽä»…åŠ è½½åˆ—å‡ºçš„æ’ä»¶ã€‚
- `deny`ï¼šå¯é€‰çš„æ’ä»¶ id æ‹’ç»åˆ—è¡¨ï¼ˆdeny ä¼˜å…ˆï¼‰ã€‚
- `load.paths`ï¼šè¦åŠ è½½çš„é¢å¤–æ’ä»¶æ–‡ä»¶æˆ–ç›®å½•ï¼ˆç»å¯¹è·¯å¾„æˆ– `~`ï¼‰ã€‚
- `entries.<pluginId>`ï¼šæ¯æ’ä»¶è¦†ç›–ã€‚
  - `enabled`ï¼šè®¾ä¸º `false` ç¦ç”¨ã€‚
  - `config`ï¼šæ’ä»¶ç‰¹å®šçš„é…ç½®å¯¹è±¡ï¼ˆå¦‚æžœæä¾›ï¼Œç”±æ’ä»¶éªŒè¯ï¼‰ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  plugins: {
    enabled: true,
    allow: ["voice-call"],
    load: {
      paths: ["~/Projects/oss/voice-call-extension"],
    },
    entries: {
      "voice-call": {
        enabled: true,
        config: {
          provider: "twilio",
        },
      },
    },
  },
}
```

### `browser`ï¼ˆKrabKrab ç®¡ç†çš„æµè§ˆå™¨ï¼‰

KrabKrab å¯ä»¥ä¸º KrabKrab å¯åŠ¨ä¸€ä¸ª**ä¸“ç”¨ã€éš”ç¦»çš„** Chrome/Brave/Edge/Chromium å®žä¾‹å¹¶æš´éœ²ä¸€ä¸ªå°åž‹ local loopback æŽ§åˆ¶æœåŠ¡ã€‚
é…ç½®æ–‡ä»¶å¯ä»¥é€šè¿‡ `profiles.<name>.cdpUrl` æŒ‡å‘**è¿œç¨‹** Chromium æµè§ˆå™¨ã€‚è¿œç¨‹é…ç½®æ–‡ä»¶ä¸ºä»…é™„åŠ æ¨¡å¼ï¼ˆstart/stop/reset è¢«ç¦ç”¨ï¼‰ã€‚

`browser.cdpUrl` ä¿ç•™ç”¨äºŽæ—§ç‰ˆå•é…ç½®æ–‡ä»¶é…ç½®ï¼Œä»¥åŠä½œä¸ºä»…è®¾ç½® `cdpPort` çš„é…ç½®æ–‡ä»¶çš„åŸºç¡€ scheme/hostã€‚

é»˜è®¤å€¼ï¼š

- enabledï¼š`true`
- evaluateEnabledï¼š`true`ï¼ˆè®¾ä¸º `false` ç¦ç”¨ `act:evaluate` å’Œ `wait --fn`ï¼‰
- æŽ§åˆ¶æœåŠ¡ï¼šä»… local loopbackï¼ˆç«¯å£ä»Ž `gateway.port` æ´¾ç”Ÿï¼Œé»˜è®¤ `18791`ï¼‰
- CDP URLï¼š`http://127.0.0.1:18792`ï¼ˆæŽ§åˆ¶æœåŠ¡ + 1ï¼Œæ—§ç‰ˆå•é…ç½®æ–‡ä»¶ï¼‰
- é…ç½®æ–‡ä»¶é¢œè‰²ï¼š`#FF4500`ï¼ˆé¾™è™¾æ©™ï¼‰
- æ³¨æ„ï¼šæŽ§åˆ¶æœåŠ¡å™¨ç”±è¿è¡Œä¸­çš„ Gateway ç½‘å…³ï¼ˆKrabKrab.app èœå•æ æˆ– `krabkrab gateway`ï¼‰å¯åŠ¨ã€‚
- è‡ªåŠ¨æ£€æµ‹é¡ºåºï¼šå¦‚æžœä¸º Chromium å†…æ ¸åˆ™ä½¿ç”¨é»˜è®¤æµè§ˆå™¨ï¼›å¦åˆ™ Chrome â†’ Brave â†’ Edge â†’ Chromium â†’ Chrome Canaryã€‚

```json5
{
  browser: {
    enabled: true,
    evaluateEnabled: true,
    // cdpUrl: "http://127.0.0.1:18792", // æ—§ç‰ˆå•é…ç½®æ–‡ä»¶è¦†ç›–
    defaultProfile: "chrome",
    profiles: {
      krabkrab: { cdpPort: 18800, color: "#FF4500" },
      work: { cdpPort: 18801, color: "#0066CC" },
      remote: { cdpUrl: "http://10.0.0.42:9222", color: "#00AA00" },
    },
    color: "#FF4500",
    // é«˜çº§ï¼š
    // headless: false,
    // noSandbox: false,
    // executablePath: "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
    // attachOnly: false, // å°†è¿œç¨‹ CDP éš§é“åˆ° localhost æ—¶è®¾ä¸º true
  },
}
```

### `ui`ï¼ˆå¤–è§‚ï¼‰

åŽŸç”Ÿåº”ç”¨ç”¨äºŽ UI å¤–è§‚çš„å¯é€‰å¼ºè°ƒè‰²ï¼ˆä¾‹å¦‚ Talk æ¨¡å¼æ°”æ³¡ç€è‰²ï¼‰ã€‚

å¦‚æžœæœªè®¾ç½®ï¼Œå®¢æˆ·ç«¯å›žé€€åˆ°æŸ”å’Œçš„æµ…è“è‰²ã€‚

```json5
{
  ui: {
    seamColor: "#FF4500", // åå…­è¿›åˆ¶ï¼ˆRRGGBB æˆ– #RRGGBBï¼‰
    // å¯é€‰ï¼šæŽ§åˆ¶å° UI åŠ©æ‰‹èº«ä»½è¦†ç›–ã€‚
    // å¦‚æžœæœªè®¾ç½®ï¼ŒæŽ§åˆ¶å° UI ä½¿ç”¨æ´»è·ƒæ™ºèƒ½ä½“çš„èº«ä»½ï¼ˆé…ç½®æˆ– IDENTITY.mdï¼‰ã€‚
    assistant: {
      name: "KrabKrab",
      avatar: "CB", // è¡¨æƒ…ã€çŸ­æ–‡æœ¬ï¼Œæˆ–å›¾ç‰‡ URL/data URI
    },
  },
}
```

### `gateway`ï¼ˆGateway ç½‘å…³æœåŠ¡å™¨æ¨¡å¼ + ç»‘å®šï¼‰

ä½¿ç”¨ `gateway.mode` æ˜Žç¡®å£°æ˜Žæ­¤æœºå™¨æ˜¯å¦åº”è¿è¡Œ Gateway ç½‘å…³ã€‚

é»˜è®¤å€¼ï¼š

- modeï¼š**æœªè®¾ç½®**ï¼ˆè§†ä¸º"ä¸è‡ªåŠ¨å¯åŠ¨"ï¼‰
- bindï¼š`loopback`
- portï¼š`18789`ï¼ˆWS + HTTP å•ç«¯å£ï¼‰

```json5
{
  gateway: {
    mode: "local", // æˆ– "remote"
    port: 18789, // WS + HTTP å¤šè·¯å¤ç”¨
    bind: "loopback",
    // controlUi: { enabled: true, basePath: "/krabkrab" }
    // auth: { mode: "token", token: "your-token" } // token æŽ§åˆ¶ WS + æŽ§åˆ¶å° UI è®¿é—®
    // tailscale: { mode: "off" | "serve" | "funnel" }
  },
}
```

æŽ§åˆ¶å° UI åŸºç¡€è·¯å¾„ï¼š

- `gateway.controlUi.basePath` è®¾ç½®æŽ§åˆ¶å° UI æä¾›æœåŠ¡çš„ URL å‰ç¼€ã€‚
- ç¤ºä¾‹ï¼š`"/ui"`ã€`"/krabkrab"`ã€`"/apps/krabkrab"`ã€‚
- é»˜è®¤ï¼šæ ¹è·¯å¾„ï¼ˆ`/`ï¼‰ï¼ˆä¸å˜ï¼‰ã€‚
- `gateway.controlUi.root` è®¾ç½®æŽ§åˆ¶å° UI èµ„äº§çš„æ–‡ä»¶ç³»ç»Ÿæ ¹ç›®å½•ï¼ˆé»˜è®¤ï¼š`dist/control-ui`ï¼‰ã€‚
- `gateway.controlUi.allowInsecureAuth` å…è®¸åœ¨çœç•¥è®¾å¤‡èº«ä»½æ—¶å¯¹æŽ§åˆ¶å° UI è¿›è¡Œä»… token è®¤è¯ï¼ˆé€šå¸¸é€šè¿‡ HTTPï¼‰ã€‚é»˜è®¤ï¼š`false`ã€‚å»ºè®®ä½¿ç”¨ HTTPSï¼ˆTailscale Serveï¼‰æˆ– `127.0.0.1`ã€‚
- `gateway.controlUi.dangerouslyDisableDeviceAuth` ç¦ç”¨æŽ§åˆ¶å° UI çš„è®¾å¤‡èº«ä»½æ£€æŸ¥ï¼ˆä»… token/å¯†ç ï¼‰ã€‚é»˜è®¤ï¼š`false`ã€‚ä»…ç”¨äºŽç´§æ€¥æƒ…å†µã€‚

ç›¸å…³æ–‡æ¡£ï¼š

- [æŽ§åˆ¶å° UI](/web/control-ui)
- [Web æ¦‚è¿°](/web)
- [Tailscale](/gateway/tailscale)
- [è¿œç¨‹è®¿é—®](/gateway/remote)

ä¿¡ä»»çš„ä»£ç†ï¼š

- `gateway.trustedProxies`ï¼šåœ¨ Gateway ç½‘å…³å‰é¢ç»ˆæ­¢ TLS çš„åå‘ä»£ç† IP åˆ—è¡¨ã€‚
- å½“è¿žæŽ¥æ¥è‡ªè¿™äº› IP ä¹‹ä¸€æ—¶ï¼ŒKrabKrab ä½¿ç”¨ `x-forwarded-for`ï¼ˆæˆ– `x-real-ip`ï¼‰æ¥ç¡®å®šå®¢æˆ·ç«¯ IPï¼Œç”¨äºŽæœ¬åœ°é…å¯¹æ£€æŸ¥å’Œ HTTP è®¤è¯/æœ¬åœ°æ£€æŸ¥ã€‚
- ä»…åˆ—å‡ºä½ å®Œå…¨æŽ§åˆ¶çš„ä»£ç†ï¼Œå¹¶ç¡®ä¿å®ƒä»¬**è¦†ç›–**ä¼ å…¥çš„ `x-forwarded-for`ã€‚

è¯´æ˜Žï¼š

- `krabkrab gateway` æ‹’ç»å¯åŠ¨ï¼Œé™¤éž `gateway.mode` è®¾ä¸º `local`ï¼ˆæˆ–ä½ ä¼ é€’äº†è¦†ç›–æ ‡å¿—ï¼‰ã€‚
- `gateway.port` æŽ§åˆ¶ç”¨äºŽ WebSocket + HTTPï¼ˆæŽ§åˆ¶å° UIã€hooksã€A2UIï¼‰çš„å•ä¸€å¤šè·¯å¤ç”¨ç«¯å£ã€‚
- OpenAI Chat Completions ç«¯ç‚¹ï¼š**é»˜è®¤ç¦ç”¨**ï¼›é€šè¿‡ `gateway.http.endpoints.chatCompletions.enabled: true` å¯ç”¨ã€‚
- ä¼˜å…ˆçº§ï¼š`--port` > `krabkrab_GATEWAY_PORT` > `gateway.port` > é»˜è®¤ `18789`ã€‚
- é»˜è®¤éœ€è¦ Gateway ç½‘å…³è®¤è¯ï¼ˆtoken/å¯†ç æˆ– Tailscale Serve èº«ä»½ï¼‰ã€‚éž local loopback ç»‘å®šéœ€è¦å…±äº« token/å¯†ç ã€‚
- æ–°æ‰‹å¼•å¯¼å‘å¯¼é»˜è®¤ç”Ÿæˆ gateway tokenï¼ˆå³ä½¿åœ¨ local loopback ä¸Šï¼‰ã€‚
- `gateway.remote.token` **ä»…**ç”¨äºŽè¿œç¨‹ CLI è°ƒç”¨ï¼›å®ƒä¸å¯ç”¨æœ¬åœ° gateway è®¤è¯ã€‚`gateway.token` è¢«å¿½ç•¥ã€‚

è®¤è¯å’Œ Tailscaleï¼š

- `gateway.auth.mode` è®¾ç½®æ¡æ‰‹è¦æ±‚ï¼ˆ`token` æˆ– `password`ï¼‰ã€‚æœªè®¾ç½®æ—¶ï¼Œå‡å®š token è®¤è¯ã€‚
- `gateway.auth.token` å­˜å‚¨ token è®¤è¯çš„å…±äº« tokenï¼ˆåŒä¸€æœºå™¨ä¸Šçš„ CLI ä½¿ç”¨ï¼‰ã€‚
- å½“è®¾ç½®äº† `gateway.auth.mode` æ—¶ï¼Œä»…æŽ¥å—è¯¥æ–¹æ³•ï¼ˆåŠ ä¸Šå¯é€‰çš„ Tailscale å¤´éƒ¨ï¼‰ã€‚
- `gateway.auth.password` å¯åœ¨æ­¤è®¾ç½®ï¼Œæˆ–é€šè¿‡ `krabkrab_GATEWAY_PASSWORD`ï¼ˆæŽ¨èï¼‰ã€‚
- `gateway.auth.allowTailscale` å…è®¸ Tailscale Serve èº«ä»½å¤´éƒ¨
  ï¼ˆ`tailscale-user-login`ï¼‰åœ¨è¯·æ±‚é€šè¿‡ local loopback åˆ°è¾¾ä¸”å¸¦æœ‰ `x-forwarded-for`ã€
  `x-forwarded-proto` å’Œ `x-forwarded-host` æ—¶æ»¡è¶³è®¤è¯ã€‚KrabKrab åœ¨æŽ¥å—ä¹‹å‰
  é€šè¿‡ `tailscale whois` è§£æž `x-forwarded-for` åœ°å€æ¥éªŒè¯èº«ä»½ã€‚ä¸º `true` æ—¶ï¼Œ
  Serve è¯·æ±‚ä¸éœ€è¦ token/å¯†ç ï¼›è®¾ä¸º `false` è¦æ±‚æ˜¾å¼å‡­æ®ã€‚å½“
  `tailscale.mode = "serve"` ä¸”è®¤è¯æ¨¡å¼ä¸æ˜¯ `password` æ—¶é»˜è®¤ä¸º `true`ã€‚
- `gateway.tailscale.mode: "serve"` ä½¿ç”¨ Tailscale Serveï¼ˆä»… tailnetï¼Œlocal loopback ç»‘å®šï¼‰ã€‚
- `gateway.tailscale.mode: "funnel"` å…¬å¼€æš´éœ²ä»ªè¡¨æ¿ï¼›éœ€è¦è®¤è¯ã€‚
- `gateway.tailscale.resetOnExit` åœ¨å…³é—­æ—¶é‡ç½® Serve/Funnel é…ç½®ã€‚

è¿œç¨‹å®¢æˆ·ç«¯é»˜è®¤å€¼ï¼ˆCLIï¼‰ï¼š

- `gateway.remote.url` è®¾ç½® `gateway.mode = "remote"` æ—¶ CLI è°ƒç”¨çš„é»˜è®¤ Gateway ç½‘å…³ WebSocket URLã€‚
- `gateway.remote.transport` é€‰æ‹© macOS è¿œç¨‹ä¼ è¾“ï¼ˆ`ssh` é»˜è®¤ï¼Œ`direct` ç”¨äºŽ ws/wssï¼‰ã€‚ä½¿ç”¨ `direct` æ—¶ï¼Œ`gateway.remote.url` å¿…é¡»ä¸º `ws://` æˆ– `wss://`ã€‚`ws://host` é»˜è®¤ç«¯å£ `18789`ã€‚
- `gateway.remote.token` æä¾›è¿œç¨‹è°ƒç”¨çš„ tokenï¼ˆä¸éœ€è¦è®¤è¯æ—¶ç•™ç©ºï¼‰ã€‚
- `gateway.remote.password` æä¾›è¿œç¨‹è°ƒç”¨çš„å¯†ç ï¼ˆä¸éœ€è¦è®¤è¯æ—¶ç•™ç©ºï¼‰ã€‚

macOS åº”ç”¨è¡Œä¸ºï¼š

- KrabKrab.app ç›‘è§† `~/.krabkrab/krabkrab.json`ï¼Œå½“ `gateway.mode` æˆ– `gateway.remote.url` å˜æ›´æ—¶å®žæ—¶åˆ‡æ¢æ¨¡å¼ã€‚
- å¦‚æžœ `gateway.mode` æœªè®¾ç½®ä½† `gateway.remote.url` å·²è®¾ç½®ï¼ŒmacOS åº”ç”¨å°†å…¶è§†ä¸ºè¿œç¨‹æ¨¡å¼ã€‚
- å½“ä½ åœ¨ macOS åº”ç”¨ä¸­æ›´æ”¹è¿žæŽ¥æ¨¡å¼æ—¶ï¼Œå®ƒä¼šå°† `gateway.mode`ï¼ˆä»¥åŠè¿œç¨‹æ¨¡å¼ä¸‹çš„ `gateway.remote.url` + `gateway.remote.transport`ï¼‰å†™å›žé…ç½®æ–‡ä»¶ã€‚

```json5
{
  gateway: {
    mode: "remote",
    remote: {
      url: "ws://gateway.tailnet:18789",
      token: "your-token",
      password: "your-password",
    },
  },
}
```

ç›´è¿žä¼ è¾“ç¤ºä¾‹ï¼ˆmacOS åº”ç”¨ï¼‰ï¼š

```json5
{
  gateway: {
    mode: "remote",
    remote: {
      transport: "direct",
      url: "wss://gateway.example.ts.net",
      token: "your-token",
    },
  },
}
```

### `gateway.reload`ï¼ˆé…ç½®çƒ­é‡è½½ï¼‰

Gateway ç½‘å…³ç›‘è§† `~/.krabkrab/krabkrab.json`ï¼ˆæˆ– `krabkrab_CONFIG_PATH`ï¼‰å¹¶è‡ªåŠ¨åº”ç”¨æ›´æ”¹ã€‚

æ¨¡å¼ï¼š

- `hybrid`ï¼ˆé»˜è®¤ï¼‰ï¼šå®‰å…¨æ›´æ”¹çƒ­åº”ç”¨ï¼›å…³é”®æ›´æ”¹é‡å¯ Gateway ç½‘å…³ã€‚
- `hot`ï¼šä»…åº”ç”¨çƒ­å®‰å…¨æ›´æ”¹ï¼›éœ€è¦é‡å¯æ—¶è®°å½•æ—¥å¿—ã€‚
- `restart`ï¼šä»»ä½•é…ç½®æ›´æ”¹éƒ½é‡å¯ Gateway ç½‘å…³ã€‚
- `off`ï¼šç¦ç”¨çƒ­é‡è½½ã€‚

```json5
{
  gateway: {
    reload: {
      mode: "hybrid",
      debounceMs: 300,
    },
  },
}
```

#### çƒ­é‡è½½çŸ©é˜µï¼ˆæ–‡ä»¶ + å½±å“ï¼‰

ç›‘è§†çš„æ–‡ä»¶ï¼š

- `~/.krabkrab/krabkrab.json`ï¼ˆæˆ– `krabkrab_CONFIG_PATH`ï¼‰

çƒ­åº”ç”¨ï¼ˆæ— éœ€å®Œå…¨é‡å¯ Gateway ç½‘å…³ï¼‰ï¼š

- `hooks`ï¼ˆwebhook è®¤è¯/è·¯å¾„/æ˜ å°„ï¼‰+ `hooks.gmail`ï¼ˆGmail ç›‘è§†å™¨é‡å¯ï¼‰
- `browser`ï¼ˆæµè§ˆå™¨æŽ§åˆ¶æœåŠ¡å™¨é‡å¯ï¼‰
- `cron`ï¼ˆcron æœåŠ¡é‡å¯ + å¹¶å‘æ›´æ–°ï¼‰
- `agents.defaults.heartbeat`ï¼ˆå¿ƒè·³è¿è¡Œå™¨é‡å¯ï¼‰
- `web`ï¼ˆWhatsApp Web æ¸ é“é‡å¯ï¼‰
- `telegram`ã€`discord`ã€`signal`ã€`imessage`ï¼ˆæ¸ é“é‡å¯ï¼‰
- `agent`ã€`models`ã€`routing`ã€`messages`ã€`session`ã€`whatsapp`ã€`logging`ã€`skills`ã€`ui`ã€`talk`ã€`identity`ã€`wizard`ï¼ˆåŠ¨æ€è¯»å–ï¼‰

éœ€è¦å®Œå…¨é‡å¯ Gateway ç½‘å…³ï¼š

- `gateway`ï¼ˆç«¯å£/ç»‘å®š/è®¤è¯/æŽ§åˆ¶å° UI/tailscaleï¼‰
- `bridge`ï¼ˆæ—§ç‰ˆï¼‰
- `discovery`
- `canvasHost`
- `plugins`
- ä»»ä½•æœªçŸ¥/ä¸æ”¯æŒçš„é…ç½®è·¯å¾„ï¼ˆä¸ºå®‰å…¨é»˜è®¤é‡å¯ï¼‰

### å¤šå®žä¾‹éš”ç¦»

è¦åœ¨ä¸€å°ä¸»æœºä¸Šè¿è¡Œå¤šä¸ª Gateway ç½‘å…³ï¼ˆç”¨äºŽå†—ä½™æˆ–æ•‘æ´æœºå™¨äººï¼‰ï¼Œè¯·éš”ç¦»æ¯ä¸ªå®žä¾‹çš„çŠ¶æ€ + é…ç½®å¹¶ä½¿ç”¨å”¯ä¸€ç«¯å£ï¼š

- `krabkrab_CONFIG_PATH`ï¼ˆæ¯å®žä¾‹é…ç½®ï¼‰
- `krabkrab_STATE_DIR`ï¼ˆä¼šè¯/å‡­æ®ï¼‰
- `agents.defaults.workspace`ï¼ˆè®°å¿†ï¼‰
- `gateway.port`ï¼ˆæ¯å®žä¾‹å”¯ä¸€ï¼‰

ä¾¿åˆ©æ ‡å¿—ï¼ˆCLIï¼‰ï¼š

- `krabkrab --dev â€¦` â†’ ä½¿ç”¨ `~/.krabkrab-dev` + ç«¯å£ä»ŽåŸºç¡€ `19001` åç§»
- `krabkrab --profile <name> â€¦` â†’ ä½¿ç”¨ `~/.krabkrab-<name>`ï¼ˆç«¯å£é€šè¿‡é…ç½®/çŽ¯å¢ƒå˜é‡/æ ‡å¿—ï¼‰

å‚è§ [Gateway ç½‘å…³è¿ç»´æ‰‹å†Œ](/gateway) äº†è§£æ´¾ç”Ÿçš„ç«¯å£æ˜ å°„ï¼ˆgateway/browser/canvasï¼‰ã€‚
å‚è§[å¤š Gateway ç½‘å…³](/gateway/multiple-gateways) äº†è§£æµè§ˆå™¨/CDP ç«¯å£éš”ç¦»ç»†èŠ‚ã€‚

ç¤ºä¾‹ï¼š

```bash
krabkrab_CONFIG_PATH=~/.krabkrab/a.json \
krabkrab_STATE_DIR=~/.krabkrab-a \
krabkrab gateway --port 19001
```

### `hooks`ï¼ˆGateway ç½‘å…³ webhookï¼‰

åœ¨ Gateway ç½‘å…³ HTTP æœåŠ¡å™¨ä¸Šå¯ç”¨ç®€å•çš„ HTTP webhook ç«¯ç‚¹ã€‚

é»˜è®¤å€¼ï¼š

- enabledï¼š`false`
- pathï¼š`/hooks`
- maxBodyBytesï¼š`262144`ï¼ˆ256 KBï¼‰

```json5
{
  hooks: {
    enabled: true,
    token: "shared-secret",
    path: "/hooks",
    presets: ["gmail"],
    transformsDir: "~/.krabkrab/hooks",
    mappings: [
      {
        match: { path: "gmail" },
        action: "agent",
        wakeMode: "now",
        name: "Gmail",
        sessionKey: "hook:gmail:{{messages[0].id}}",
        messageTemplate: "From: {{messages[0].from}}\nSubject: {{messages[0].subject}}\n{{messages[0].snippet}}",
        deliver: true,
        channel: "last",
        model: "openai/gpt-5.2-mini",
      },
    ],
  },
}
```

è¯·æ±‚å¿…é¡»åŒ…å« hook tokenï¼š

- `Authorization: Bearer <token>` **æˆ–**
- `x-krabkrab-token: <token>` **æˆ–**
- `?token=<token>`

ç«¯ç‚¹ï¼š

- `POST /hooks/wake` â†’ `{ text, mode?: "now"|"next-heartbeat" }`
- `POST /hooks/agent` â†’ `{ message, name?, sessionKey?, wakeMode?, deliver?, channel?, to?, model?, thinking?, timeoutSeconds? }`
- `POST /hooks/<name>` â†’ é€šè¿‡ `hooks.mappings` è§£æž

`/hooks/agent` å§‹ç»ˆå°†æ‘˜è¦å‘å¸ƒåˆ°ä¸»ä¼šè¯ï¼ˆå¹¶å¯é€šè¿‡ `wakeMode: "now"` å¯é€‰åœ°è§¦å‘å³æ—¶å¿ƒè·³ï¼‰ã€‚

æ˜ å°„è¯´æ˜Žï¼š

- `match.path` åŒ¹é… `/hooks` ä¹‹åŽçš„å­è·¯å¾„ï¼ˆä¾‹å¦‚ `/hooks/gmail` â†’ `gmail`ï¼‰ã€‚
- `match.source` åŒ¹é…è´Ÿè½½å­—æ®µï¼ˆä¾‹å¦‚ `{ source: "gmail" }`ï¼‰ï¼Œä»¥ä¾¿ä½¿ç”¨é€šç”¨çš„ `/hooks/ingest` è·¯å¾„ã€‚
- `{{messages[0].subject}}` ç­‰æ¨¡æ¿ä»Žè´Ÿè½½ä¸­è¯»å–ã€‚
- `transform` å¯ä»¥æŒ‡å‘è¿”å›ž hook åŠ¨ä½œçš„ JS/TS æ¨¡å—ã€‚
- `deliver: true` å°†æœ€ç»ˆå›žå¤å‘é€åˆ°æ¸ é“ï¼›`channel` é»˜è®¤ä¸º `last`ï¼ˆå›žé€€åˆ° WhatsAppï¼‰ã€‚
- å¦‚æžœæ²¡æœ‰å…ˆå‰çš„æŠ•é€’è·¯ç”±ï¼Œè¯·æ˜¾å¼è®¾ç½® `channel` + `to`ï¼ˆTelegram/Discord/Google Chat/Slack/Signal/iMessage/MS Teams å¿…éœ€ï¼‰ã€‚
- `model` è¦†ç›–æ­¤ hook è¿è¡Œçš„ LLMï¼ˆ`provider/model` æˆ–åˆ«åï¼›å¦‚æžœè®¾ç½®äº† `agents.defaults.models` åˆ™å¿…é¡»è¢«å…è®¸ï¼‰ã€‚

Gmail è¾…åŠ©é…ç½®ï¼ˆç”± `krabkrab webhooks gmail setup` / `run` ä½¿ç”¨ï¼‰ï¼š

```json5
{
  hooks: {
    gmail: {
      account: "krabkrab@gmail.com",
      topic: "projects/<project-id>/topics/gog-gmail-watch",
      subscription: "gog-gmail-watch-push",
      pushToken: "shared-push-token",
      hookUrl: "http://127.0.0.1:18789/hooks/gmail",
      includeBody: true,
      maxBytes: 20000,
      renewEveryMinutes: 720,
      serve: { bind: "127.0.0.1", port: 8788, path: "/" },
      tailscale: { mode: "funnel", path: "/gmail-pubsub" },

      // å¯é€‰ï¼šä¸º Gmail hook å¤„ç†ä½¿ç”¨æ›´ä¾¿å®œçš„æ¨¡åž‹
      // åœ¨è®¤è¯/é€ŸçŽ‡é™åˆ¶/è¶…æ—¶æ—¶å›žé€€åˆ° agents.defaults.model.fallbacksï¼Œç„¶åŽ primary
      model: "openrouter/meta-llama/llama-3.3-70b-instruct:free",
      // å¯é€‰ï¼šGmail hook çš„é»˜è®¤æ€è€ƒçº§åˆ«
      thinking: "off",
    },
  },
}
```

Gmail hook çš„æ¨¡åž‹è¦†ç›–ï¼š

- `hooks.gmail.model` æŒ‡å®šç”¨äºŽ Gmail hook å¤„ç†çš„æ¨¡åž‹ï¼ˆé»˜è®¤ä¸ºä¼šè¯ä¸»æ¨¡åž‹ï¼‰ã€‚
- æŽ¥å— `provider/model` å¼•ç”¨æˆ–æ¥è‡ª `agents.defaults.models` çš„åˆ«åã€‚
- åœ¨è®¤è¯/é€ŸçŽ‡é™åˆ¶/è¶…æ—¶æ—¶å›žé€€åˆ° `agents.defaults.model.fallbacks`ï¼Œç„¶åŽ `agents.defaults.model.primary`ã€‚
- å¦‚æžœè®¾ç½®äº† `agents.defaults.models`ï¼Œè¯·å°† hooks æ¨¡åž‹åŒ…å«åœ¨ç™½åå•ä¸­ã€‚
- å¯åŠ¨æ—¶ï¼Œå¦‚æžœé…ç½®çš„æ¨¡åž‹ä¸åœ¨æ¨¡åž‹ç›®å½•æˆ–ç™½åå•ä¸­ï¼Œä¼šå‘å‡ºè­¦å‘Šã€‚
- `hooks.gmail.thinking` è®¾ç½® Gmail hook çš„é»˜è®¤æ€è€ƒçº§åˆ«ï¼Œè¢«æ¯ hook çš„ `thinking` è¦†ç›–ã€‚

Gateway ç½‘å…³è‡ªåŠ¨å¯åŠ¨ï¼š

- å¦‚æžœ `hooks.enabled=true` ä¸” `hooks.gmail.account` å·²è®¾ç½®ï¼ŒGateway ç½‘å…³åœ¨å¯åŠ¨æ—¶
  å¯åŠ¨ `gog gmail watch serve` å¹¶è‡ªåŠ¨ç»­æœŸç›‘è§†ã€‚
- è®¾ç½® `krabkrab_SKIP_GMAIL_WATCHER=1` ç¦ç”¨è‡ªåŠ¨å¯åŠ¨ï¼ˆç”¨äºŽæ‰‹åŠ¨è¿è¡Œï¼‰ã€‚
- é¿å…åœ¨ Gateway ç½‘å…³æ—è¾¹å•ç‹¬è¿è¡Œ `gog gmail watch serve`ï¼›å®ƒä¼š
  å›  `listen tcp 127.0.0.1:8788: bind: address already in use` è€Œå¤±è´¥ã€‚

æ³¨æ„ï¼šå½“ `tailscale.mode` å¼€å¯æ—¶ï¼ŒKrabKrab å°† `serve.path` é»˜è®¤ä¸º `/`ï¼Œä»¥ä¾¿
Tailscale å¯ä»¥æ­£ç¡®ä»£ç† `/gmail-pubsub`ï¼ˆå®ƒä¼šåŽ»é™¤è®¾ç½®çš„è·¯å¾„å‰ç¼€ï¼‰ã€‚
å¦‚æžœä½ éœ€è¦åŽç«¯æŽ¥æ”¶å¸¦å‰ç¼€çš„è·¯å¾„ï¼Œè¯·å°†
`hooks.gmail.tailscale.target` è®¾ä¸ºå®Œæ•´ URLï¼ˆå¹¶å¯¹é½ `serve.path`ï¼‰ã€‚

### `canvasHost`ï¼ˆLAN/tailnet Canvas æ–‡ä»¶æœåŠ¡å™¨ + å®žæ—¶é‡è½½ï¼‰

Gateway ç½‘å…³é€šè¿‡ HTTP æä¾› HTML/CSS/JS ç›®å½•æœåŠ¡ï¼Œä»¥ä¾¿ iOS/Android èŠ‚ç‚¹å¯ä»¥ç®€å•åœ° `canvas.navigate` åˆ°å®ƒã€‚

é»˜è®¤æ ¹ç›®å½•ï¼š`~/.krabkrab/workspace/canvas`
é»˜è®¤ç«¯å£ï¼š`18793`ï¼ˆé€‰æ‹©æ­¤ç«¯å£ä»¥é¿å… KrabKrab æµè§ˆå™¨ CDP ç«¯å£ `18792`ï¼‰
æœåŠ¡å™¨ç›‘å¬ **Gateway ç½‘å…³ç»‘å®šä¸»æœº**ï¼ˆLAN æˆ– Tailnetï¼‰ï¼Œä»¥ä¾¿èŠ‚ç‚¹å¯ä»¥è®¿é—®ã€‚

æœåŠ¡å™¨ï¼š

- æä¾› `canvasHost.root` ä¸‹çš„æ–‡ä»¶
- å‘æä¾›çš„ HTML æ³¨å…¥å¾®åž‹å®žæ—¶é‡è½½å®¢æˆ·ç«¯
- ç›‘è§†ç›®å½•å¹¶é€šè¿‡ `/__krabkrab__/ws` çš„ WebSocket ç«¯ç‚¹å¹¿æ’­é‡è½½
- ç›®å½•ä¸ºç©ºæ—¶è‡ªåŠ¨åˆ›å»ºèµ·å§‹ `index.html`ï¼ˆä»¥ä¾¿ä½ ç«‹å³çœ‹åˆ°å†…å®¹ï¼‰
- åŒæ—¶åœ¨ `/__krabkrab__/a2ui/` æä¾› A2UIï¼Œå¹¶ä½œä¸º `canvasHostUrl` é€šå‘Šç»™èŠ‚ç‚¹
  ï¼ˆèŠ‚ç‚¹å§‹ç»ˆä½¿ç”¨å®ƒæ¥è®¿é—® Canvas/A2UIï¼‰

å¦‚æžœç›®å½•å¾ˆå¤§æˆ–é‡åˆ° `EMFILE`ï¼Œè¯·ç¦ç”¨å®žæ—¶é‡è½½ï¼ˆå’Œæ–‡ä»¶ç›‘è§†ï¼‰ï¼š

- é…ç½®ï¼š`canvasHost: { liveReload: false }`

```json5
{
  canvasHost: {
    root: "~/.krabkrab/workspace/canvas",
    port: 18793,
    liveReload: true,
  },
}
```

`canvasHost.*` çš„æ›´æ”¹éœ€è¦é‡å¯ Gateway ç½‘å…³ï¼ˆé…ç½®é‡è½½ä¼šè§¦å‘é‡å¯ï¼‰ã€‚

ç¦ç”¨æ–¹å¼ï¼š

- é…ç½®ï¼š`canvasHost: { enabled: false }`
- çŽ¯å¢ƒå˜é‡ï¼š`krabkrab_SKIP_CANVAS_HOST=1`

### `bridge`ï¼ˆæ—§ç‰ˆ TCP æ¡¥æŽ¥ï¼Œå·²ç§»é™¤ï¼‰

å½“å‰ç‰ˆæœ¬ä¸å†åŒ…å« TCP æ¡¥æŽ¥ç›‘å¬å™¨ï¼›`bridge.*` é…ç½®é”®ä¼šè¢«å¿½ç•¥ã€‚
èŠ‚ç‚¹é€šè¿‡ Gateway ç½‘å…³ WebSocket è¿žæŽ¥ã€‚æ­¤éƒ¨åˆ†ä»…ä¿ç•™ä¾›åŽ†å²å‚è€ƒã€‚

æ—§ç‰ˆè¡Œä¸ºï¼š

- Gateway ç½‘å…³å¯ä»¥ä¸ºèŠ‚ç‚¹ï¼ˆiOS/Androidï¼‰æš´éœ²ç®€å•çš„ TCP æ¡¥æŽ¥ï¼Œé€šå¸¸åœ¨ç«¯å£ `18790`ã€‚

é»˜è®¤å€¼ï¼š

- enabledï¼š`true`
- portï¼š`18790`
- bindï¼š`lan`ï¼ˆç»‘å®šåˆ° `0.0.0.0`ï¼‰

ç»‘å®šæ¨¡å¼ï¼š

- `lan`ï¼š`0.0.0.0`ï¼ˆå¯é€šè¿‡ä»»ä½•æŽ¥å£è®¿é—®ï¼ŒåŒ…æ‹¬ LAN/Wiâ€‘Fi å’Œ Tailscaleï¼‰
- `tailnet`ï¼šä»…ç»‘å®šåˆ°æœºå™¨çš„ Tailscale IPï¼ˆæŽ¨èç”¨äºŽè·¨åœ°åŸŸè®¿é—®ï¼‰
- `loopback`ï¼š`127.0.0.1`ï¼ˆä»…æœ¬åœ°ï¼‰
- `auto`ï¼šå¦‚æžœå­˜åœ¨ tailnet IP åˆ™ä¼˜å…ˆä½¿ç”¨ï¼Œå¦åˆ™ `lan`

TLSï¼š

- `bridge.tls.enabled`ï¼šä¸ºæ¡¥æŽ¥è¿žæŽ¥å¯ç”¨ TLSï¼ˆå¯ç”¨æ—¶ä»… TLSï¼‰ã€‚
- `bridge.tls.autoGenerate`ï¼šå½“æ— è¯ä¹¦/å¯†é’¥æ—¶ç”Ÿæˆè‡ªç­¾åè¯ä¹¦ï¼ˆé»˜è®¤ï¼štrueï¼‰ã€‚
- `bridge.tls.certPath` / `bridge.tls.keyPath`ï¼šæ¡¥æŽ¥è¯ä¹¦ + ç§é’¥çš„ PEM è·¯å¾„ã€‚
- `bridge.tls.caPath`ï¼šå¯é€‰çš„ PEM CA æ†ç»‘åŒ…ï¼ˆè‡ªå®šä¹‰æ ¹è¯ä¹¦æˆ–æœªæ¥çš„ mTLSï¼‰ã€‚

å¯ç”¨ TLS åŽï¼ŒGateway ç½‘å…³åœ¨å‘çŽ° TXT è®°å½•ä¸­é€šå‘Š `bridgeTls=1` å’Œ `bridgeTlsSha256`ï¼Œä»¥ä¾¿èŠ‚ç‚¹å¯ä»¥å›ºå®šè¯ä¹¦ã€‚å¦‚æžœå°šæœªå­˜å‚¨æŒ‡çº¹ï¼Œæ‰‹åŠ¨è¿žæŽ¥ä½¿ç”¨é¦–æ¬¡ä¿¡ä»»ã€‚
è‡ªåŠ¨ç”Ÿæˆçš„è¯ä¹¦éœ€è¦ PATH ä¸­æœ‰ `openssl`ï¼›å¦‚æžœç”Ÿæˆå¤±è´¥ï¼Œæ¡¥æŽ¥ä¸ä¼šå¯åŠ¨ã€‚

```json5
{
  bridge: {
    enabled: true,
    port: 18790,
    bind: "tailnet",
    tls: {
      enabled: true,
      // çœç•¥æ—¶ä½¿ç”¨ ~/.krabkrab/bridge/tls/bridge-{cert,key}.pemã€‚
      // certPath: "~/.krabkrab/bridge/tls/bridge-cert.pem",
      // keyPath: "~/.krabkrab/bridge/tls/bridge-key.pem"
    },
  },
}
```

### `discovery.mdns`ï¼ˆBonjour / mDNS å¹¿æ’­æ¨¡å¼ï¼‰

æŽ§åˆ¶ LAN mDNS å‘çŽ°å¹¿æ’­ï¼ˆ`_krabkrab-gw._tcp`ï¼‰ã€‚

- `minimal`ï¼ˆé»˜è®¤ï¼‰ï¼šä»Ž TXT è®°å½•ä¸­çœç•¥ `cliPath` + `sshPort`
- `full`ï¼šåœ¨ TXT è®°å½•ä¸­åŒ…å« `cliPath` + `sshPort`
- `off`ï¼šå®Œå…¨ç¦ç”¨ mDNS å¹¿æ’­
- ä¸»æœºåï¼šé»˜è®¤ä¸º `krabkrab`ï¼ˆé€šå‘Š `krabkrab.local`ï¼‰ã€‚é€šè¿‡ `krabkrab_MDNS_HOSTNAME` è¦†ç›–ã€‚

```json5
{
  discovery: { mdns: { mode: "minimal" } },
}
```

### `discovery.wideArea`ï¼ˆå¹¿åŸŸ Bonjour / å•æ’­ DNSâ€‘SDï¼‰

å¯ç”¨åŽï¼ŒGateway ç½‘å…³åœ¨ `~/.krabkrab/dns/` ä¸‹ä½¿ç”¨é…ç½®çš„å‘çŽ°åŸŸï¼ˆç¤ºä¾‹ï¼š`krabkrab.internal.`ï¼‰ä¸º `_krabkrab-gw._tcp` å†™å…¥å•æ’­ DNS-SD åŒºåŸŸã€‚

è¦ä½¿ iOS/Android è·¨ç½‘ç»œå‘çŽ°ï¼ˆè·¨åœ°åŸŸè®¿é—®ï¼‰ï¼Œè¯·é…åˆä»¥ä¸‹ä½¿ç”¨ï¼š

- åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè¿è¡Œ DNS æœåŠ¡å™¨ï¼Œä¸ºä½ é€‰æ‹©çš„åŸŸåæä¾›æœåŠ¡ï¼ˆæŽ¨è CoreDNSï¼‰
- Tailscale **split DNS**ï¼Œä½¿å®¢æˆ·ç«¯é€šè¿‡ Gateway ç½‘å…³ DNS æœåŠ¡å™¨è§£æžè¯¥åŸŸå

ä¸€æ¬¡æ€§è®¾ç½®åŠ©æ‰‹ï¼ˆGateway ç½‘å…³ä¸»æœºï¼‰ï¼š

```bash
krabkrab dns setup --apply
```

```json5
{
  discovery: { wideArea: { enabled: true } },
}
```

## æ¨¡æ¿å˜é‡

æ¨¡æ¿å ä½ç¬¦åœ¨ `tools.media.*.models[].args` å’Œ `tools.media.models[].args`ï¼ˆä»¥åŠæœªæ¥ä»»ä½•æ¨¡æ¿åŒ–å‚æ•°å­—æ®µï¼‰ä¸­å±•å¼€ã€‚

| å˜é‡               | æè¿°                                                  |
| ------------------ | ----------------------------------------------------- | -------- | ------- | ---------- | ----- | ------ | -------- | ------- | ------- | --- |
| `{{Body}}`         | å®Œæ•´çš„å…¥ç«™æ¶ˆæ¯æ­£æ–‡                                    |
| `{{RawBody}}`      | åŽŸå§‹å…¥ç«™æ¶ˆæ¯æ­£æ–‡ï¼ˆæ— åŽ†å²/å‘é€è€…åŒ…è£…ï¼›æœ€é€‚åˆå‘½ä»¤è§£æžï¼‰ |
| `{{BodyStripped}}` | åŽ»é™¤ç¾¤ç»„æåŠçš„æ­£æ–‡ï¼ˆæœ€é€‚åˆæ™ºèƒ½ä½“çš„é»˜è®¤å€¼ï¼‰            |
| `{{From}}`         | å‘é€è€…æ ‡è¯†ç¬¦ï¼ˆWhatsApp ä¸º E.164ï¼›æŒ‰æ¸ é“å¯èƒ½ä¸åŒï¼‰     |
| `{{To}}`           | ç›®æ ‡æ ‡è¯†ç¬¦                                            |
| `{{MessageSid}}`   | æ¸ é“æ¶ˆæ¯ idï¼ˆå¦‚æžœå¯ç”¨ï¼‰                               |
| `{{SessionId}}`    | å½“å‰ä¼šè¯ UUID                                         |
| `{{IsNewSession}}` | åˆ›å»ºæ–°ä¼šè¯æ—¶ä¸º `"true"`                               |
| `{{MediaUrl}}`     | å…¥ç«™åª’ä½“ä¼ª URLï¼ˆå¦‚æžœå­˜åœ¨ï¼‰                            |
| `{{MediaPath}}`    | æœ¬åœ°åª’ä½“è·¯å¾„ï¼ˆå¦‚æžœå·²ä¸‹è½½ï¼‰                            |
| `{{MediaType}}`    | åª’ä½“ç±»åž‹ï¼ˆimage/audio/document/â€¦ï¼‰                    |
| `{{Transcript}}`   | éŸ³é¢‘è½¬å½•ï¼ˆå¯ç”¨æ—¶ï¼‰                                    |
| `{{Prompt}}`       | CLI æ¡ç›®çš„å·²è§£æžåª’ä½“æç¤º                              |
| `{{MaxChars}}`     | CLI æ¡ç›®çš„å·²è§£æžæœ€å¤§è¾“å‡ºå­—ç¬¦æ•°                        |
| `{{ChatType}}`     | `"direct"` æˆ– `"group"`                               |
| `{{GroupSubject}}` | ç¾¤ç»„ä¸»é¢˜ï¼ˆå°½åŠ›è€Œä¸ºï¼‰                                  |
| `{{GroupMembers}}` | ç¾¤ç»„æˆå‘˜é¢„è§ˆï¼ˆå°½åŠ›è€Œä¸ºï¼‰                              |
| `{{SenderName}}`   | å‘é€è€…æ˜¾ç¤ºåç§°ï¼ˆå°½åŠ›è€Œä¸ºï¼‰                            |
| `{{SenderE164}}`   | å‘é€è€…ç”µè¯å·ç ï¼ˆå°½åŠ›è€Œä¸ºï¼‰                            |
| `{{Provider}}`     | æä¾›å•†æç¤ºï¼ˆwhatsapp                                  | telegram | discord | googlechat | slack | signal | imessage | msteams | webchat | â€¦ï¼‰ |

## Cronï¼ˆGateway ç½‘å…³è°ƒåº¦å™¨ï¼‰

Cron æ˜¯ Gateway ç½‘å…³è‡ªæœ‰çš„å”¤é†’å’Œå®šæ—¶ä»»åŠ¡è°ƒåº¦å™¨ã€‚å‚è§ [Cron ä»»åŠ¡](/automation/cron-jobs) äº†è§£åŠŸèƒ½æ¦‚è¿°å’Œ CLI ç¤ºä¾‹ã€‚

```json5
{
  cron: {
    enabled: true,
    maxConcurrentRuns: 2,
  },
}
```

---

_ä¸‹ä¸€æ­¥ï¼š[æ™ºèƒ½ä½“è¿è¡Œæ—¶](/concepts/agent)_ ðŸ¦ž

