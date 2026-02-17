---
read_when:
  - æ·»åŠ æˆ–ä¿®æ”¹æ’ä»¶/æ‰©å±•
  - è®°å½•æ’ä»¶å®‰è£…æˆ–åŠ è½½è§„åˆ™
summary: KrabKrab æ’ä»¶/æ‰©å±•ï¼šå‘çŽ°ã€é…ç½®å’Œå®‰å…¨
title: æ’ä»¶
x-i18n:
  generated_at: "2026-02-03T07:55:25Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: b36ca6b90ca03eaae25c00f9b12f2717fcd17ac540ba616ee03b398b234c2308
  source_path: tools/plugin.md
  workflow: 15
---

# æ’ä»¶ï¼ˆæ‰©å±•ï¼‰

## å¿«é€Ÿå¼€å§‹ï¼ˆæ’ä»¶æ–°æ‰‹ï¼Ÿï¼‰

æ’ä»¶åªæ˜¯ä¸€ä¸ª**å°åž‹ä»£ç æ¨¡å—**ï¼Œç”¨é¢å¤–åŠŸèƒ½ï¼ˆå‘½ä»¤ã€å·¥å…·å’Œ Gateway ç½‘å…³ RPCï¼‰æ‰©å±• KrabKrabã€‚

å¤§å¤šæ•°æ—¶å€™ï¼Œå½“ä½ æƒ³è¦ä¸€ä¸ªå°šæœªå†…ç½®åˆ°æ ¸å¿ƒ KrabKrab çš„åŠŸèƒ½ï¼ˆæˆ–ä½ æƒ³å°†å¯é€‰åŠŸèƒ½æŽ’é™¤åœ¨ä¸»å®‰è£…ä¹‹å¤–ï¼‰æ—¶ï¼Œä½ ä¼šä½¿ç”¨æ’ä»¶ã€‚

å¿«é€Ÿè·¯å¾„ï¼š

1. æŸ¥çœ‹å·²åŠ è½½çš„å†…å®¹ï¼š

```bash
krabkrab plugins list
```

2. å®‰è£…å®˜æ–¹æ’ä»¶ï¼ˆä¾‹å¦‚ï¼šVoice Callï¼‰ï¼š

```bash
krabkrab plugins install @krabkrab/voice-call
```

3. é‡å¯ Gateway ç½‘å…³ï¼Œç„¶åŽåœ¨ `plugins.entries.<id>.config` ä¸‹é…ç½®ã€‚

å‚è§ [Voice Call](/plugins/voice-call) äº†è§£å…·ä½“çš„æ’ä»¶ç¤ºä¾‹ã€‚

## å¯ç”¨æ’ä»¶ï¼ˆå®˜æ–¹ï¼‰

- ä»Ž 2026.1.15 èµ· Microsoft Teams ä»…ä½œä¸ºæ’ä»¶æä¾›ï¼›å¦‚æžœä½¿ç”¨ Teamsï¼Œè¯·å®‰è£… `@krabkrab/msteams`ã€‚
- Memory (Core) â€” æ†ç»‘çš„è®°å¿†æœç´¢æ’ä»¶ï¼ˆé€šè¿‡ `plugins.slots.memory` é»˜è®¤å¯ç”¨ï¼‰
- Memory (LanceDB) â€” æ†ç»‘çš„é•¿æœŸè®°å¿†æ’ä»¶ï¼ˆè‡ªåŠ¨å¬å›ž/æ•èŽ·ï¼›è®¾ç½® `plugins.slots.memory = "memory-lancedb"`ï¼‰
- [Voice Call](/plugins/voice-call) â€” `@krabkrab/voice-call`
- [Zalo Personal](/plugins/zalouser) â€” `@krabkrab/zalouser`
- [Matrix](/channels/matrix) â€” `@krabkrab/matrix`
- [Nostr](/channels/nostr) â€” `@krabkrab/nostr`
- [Zalo](/channels/zalo) â€” `@krabkrab/zalo`
- [Microsoft Teams](/channels/msteams) â€” `@krabkrab/msteams`
- Google Antigravity OAuthï¼ˆæä¾›å•†è®¤è¯ï¼‰â€” ä½œä¸º `google-antigravity-auth` æ†ç»‘ï¼ˆé»˜è®¤ç¦ç”¨ï¼‰
- Gemini CLI OAuthï¼ˆæä¾›å•†è®¤è¯ï¼‰â€” ä½œä¸º `google-gemini-cli-auth` æ†ç»‘ï¼ˆé»˜è®¤ç¦ç”¨ï¼‰
- Qwen OAuthï¼ˆæä¾›å•†è®¤è¯ï¼‰â€” ä½œä¸º `qwen-portal-auth` æ†ç»‘ï¼ˆé»˜è®¤ç¦ç”¨ï¼‰
- Copilot Proxyï¼ˆæä¾›å•†è®¤è¯ï¼‰â€” æœ¬åœ° VS Code Copilot Proxy æ¡¥æŽ¥ï¼›ä¸Žå†…ç½® `github-copilot` è®¾å¤‡ç™»å½•ä¸åŒï¼ˆæ†ç»‘ï¼Œé»˜è®¤ç¦ç”¨ï¼‰

KrabKrab æ’ä»¶æ˜¯é€šè¿‡ jiti åœ¨è¿è¡Œæ—¶åŠ è½½çš„ **TypeScript æ¨¡å—**ã€‚**é…ç½®éªŒè¯ä¸ä¼šæ‰§è¡Œæ’ä»¶ä»£ç **ï¼›å®ƒä½¿ç”¨æ’ä»¶æ¸…å•å’Œ JSON Schemaã€‚å‚è§ [æ’ä»¶æ¸…å•](/plugins/manifest)ã€‚

æ’ä»¶å¯ä»¥æ³¨å†Œï¼š

- Gateway ç½‘å…³ RPC æ–¹æ³•
- Gateway ç½‘å…³ HTTP å¤„ç†ç¨‹åº
- æ™ºèƒ½ä½“å·¥å…·
- CLI å‘½ä»¤
- åŽå°æœåŠ¡
- å¯é€‰çš„é…ç½®éªŒè¯
- **Skills**ï¼ˆé€šè¿‡åœ¨æ’ä»¶æ¸…å•ä¸­åˆ—å‡º `skills` ç›®å½•ï¼‰
- **è‡ªåŠ¨å›žå¤å‘½ä»¤**ï¼ˆä¸è°ƒç”¨ AI æ™ºèƒ½ä½“å³å¯æ‰§è¡Œï¼‰

æ’ä»¶ä¸Ž Gateway ç½‘å…³**åœ¨åŒä¸€è¿›ç¨‹ä¸­**è¿è¡Œï¼Œå› æ­¤å°†å®ƒä»¬è§†ä¸ºå—ä¿¡ä»»çš„ä»£ç ã€‚
å·¥å…·ç¼–å†™æŒ‡å—ï¼š[æ’ä»¶æ™ºèƒ½ä½“å·¥å…·](/plugins/agent-tools)ã€‚

## è¿è¡Œæ—¶è¾…åŠ©å·¥å…·

æ’ä»¶å¯ä»¥é€šè¿‡ `api.runtime` è®¿é—®é€‰å®šçš„æ ¸å¿ƒè¾…åŠ©å·¥å…·ã€‚å¯¹äºŽç”µè¯ TTSï¼š

```ts
const result = await api.runtime.tts.textToSpeechTelephony({
  text: "Hello from KrabKrab",
  cfg: api.config,
});
```

æ³¨æ„äº‹é¡¹ï¼š

- ä½¿ç”¨æ ¸å¿ƒ `messages.tts` é…ç½®ï¼ˆOpenAI æˆ– ElevenLabsï¼‰ã€‚
- è¿”å›ž PCM éŸ³é¢‘ç¼“å†²åŒº + é‡‡æ ·çŽ‡ã€‚æ’ä»¶å¿…é¡»ä¸ºæä¾›å•†é‡æ–°é‡‡æ ·/ç¼–ç ã€‚
- Edge TTS ä¸æ”¯æŒç”µè¯ã€‚

## å‘çŽ°å’Œä¼˜å…ˆçº§

KrabKrab æŒ‰é¡ºåºæ‰«æï¼š

1. é…ç½®è·¯å¾„

- `plugins.load.paths`ï¼ˆæ–‡ä»¶æˆ–ç›®å½•ï¼‰

2. å·¥ä½œåŒºæ‰©å±•

- `<workspace>/.krabkrab/extensions/*.ts`
- `<workspace>/.krabkrab/extensions/*/index.ts`

3. å…¨å±€æ‰©å±•

- `~/.krabkrab/extensions/*.ts`
- `~/.krabkrab/extensions/*/index.ts`

4. æ†ç»‘æ‰©å±•ï¼ˆéš KrabKrab ä¸€èµ·å‘å¸ƒï¼Œ**é»˜è®¤ç¦ç”¨**ï¼‰

- `<krabkrab>/extensions/*`

æ†ç»‘æ’ä»¶å¿…é¡»é€šè¿‡ `plugins.entries.<id>.enabled` æˆ– `krabkrab plugins enable <id>` æ˜¾å¼å¯ç”¨ã€‚å·²å®‰è£…çš„æ’ä»¶é»˜è®¤å¯ç”¨ï¼Œä½†å¯ä»¥ç”¨ç›¸åŒæ–¹å¼ç¦ç”¨ã€‚

æ¯ä¸ªæ’ä»¶å¿…é¡»åœ¨å…¶æ ¹ç›®å½•ä¸­åŒ…å« `krabkrab.plugin.json` æ–‡ä»¶ã€‚å¦‚æžœè·¯å¾„æŒ‡å‘æ–‡ä»¶ï¼Œåˆ™æ’ä»¶æ ¹ç›®å½•æ˜¯æ–‡ä»¶çš„ç›®å½•ï¼Œå¿…é¡»åŒ…å«æ¸…å•ã€‚

å¦‚æžœå¤šä¸ªæ’ä»¶è§£æžåˆ°ç›¸åŒçš„ idï¼Œä¸Šè¿°é¡ºåºä¸­çš„ç¬¬ä¸€ä¸ªåŒ¹é…é¡¹èŽ·èƒœï¼Œè¾ƒä½Žä¼˜å…ˆçº§çš„å‰¯æœ¬è¢«å¿½ç•¥ã€‚

### åŒ…é›†åˆ

æ’ä»¶ç›®å½•å¯ä»¥åŒ…å«å¸¦æœ‰ `krabkrab.extensions` çš„ `package.json`ï¼š

```json
{
  "name": "my-pack",
  "krabkrab": {
    "extensions": ["./src/safety.ts", "./src/tools.ts"]
  }
}
```

æ¯ä¸ªæ¡ç›®æˆä¸ºä¸€ä¸ªæ’ä»¶ã€‚å¦‚æžœåŒ…åˆ—å‡ºå¤šä¸ªæ‰©å±•ï¼Œæ’ä»¶ id å˜ä¸º `name/<fileBase>`ã€‚

å¦‚æžœä½ çš„æ’ä»¶å¯¼å…¥ npm ä¾èµ–ï¼Œè¯·åœ¨è¯¥ç›®å½•ä¸­å®‰è£…å®ƒä»¬ä»¥ä¾¿ `node_modules` å¯ç”¨ï¼ˆ`npm install` / `pnpm install`ï¼‰ã€‚

### æ¸ é“ç›®å½•å…ƒæ•°æ®

æ¸ é“æ’ä»¶å¯ä»¥é€šè¿‡ `krabkrab.channel` å¹¿æ’­æ–°æ‰‹å¼•å¯¼å…ƒæ•°æ®ï¼Œé€šè¿‡ `krabkrab.install` å¹¿æ’­å®‰è£…æç¤ºã€‚è¿™ä½¿æ ¸å¿ƒç›®å½•ä¿æŒæ— æ•°æ®ã€‚

ç¤ºä¾‹ï¼š

```json
{
  "name": "@krabkrab/nextcloud-talk",
  "krabkrab": {
    "extensions": ["./index.ts"],
    "channel": {
      "id": "nextcloud-talk",
      "label": "Nextcloud Talk",
      "selectionLabel": "Nextcloud Talk (self-hosted)",
      "docsPath": "/channels/nextcloud-talk",
      "docsLabel": "nextcloud-talk",
      "blurb": "Self-hosted chat via Nextcloud Talk webhook bots.",
      "order": 65,
      "aliases": ["nc-talk", "nc"]
    },
    "install": {
      "npmSpec": "@krabkrab/nextcloud-talk",
      "localPath": "extensions/nextcloud-talk",
      "defaultChoice": "npm"
    }
  }
}
```

KrabKrab è¿˜å¯ä»¥åˆå¹¶**å¤–éƒ¨æ¸ é“ç›®å½•**ï¼ˆä¾‹å¦‚ï¼ŒMPM æ³¨å†Œè¡¨å¯¼å‡ºï¼‰ã€‚å°† JSON æ–‡ä»¶æ”¾åœ¨ä»¥ä¸‹ä½ç½®ä¹‹ä¸€ï¼š

- `~/.krabkrab/mpm/plugins.json`
- `~/.krabkrab/mpm/catalog.json`
- `~/.krabkrab/plugins/catalog.json`

æˆ–å°† `krabkrab_PLUGIN_CATALOG_PATHS`ï¼ˆæˆ– `krabkrab_MPM_CATALOG_PATHS`ï¼‰æŒ‡å‘ä¸€ä¸ªæˆ–å¤šä¸ª JSON æ–‡ä»¶ï¼ˆé€—å·/åˆ†å·/`PATH` åˆ†éš”ï¼‰ã€‚æ¯ä¸ªæ–‡ä»¶åº”åŒ…å« `{ "entries": [ { "name": "@scope/pkg", "krabkrab": { "channel": {...}, "install": {...} } } ] }`ã€‚

## æ’ä»¶ ID

é»˜è®¤æ’ä»¶ idï¼š

- åŒ…é›†åˆï¼š`package.json` çš„ `name`
- ç‹¬ç«‹æ–‡ä»¶ï¼šæ–‡ä»¶åŸºæœ¬åç§°ï¼ˆ`~/.../voice-call.ts` â†’ `voice-call`ï¼‰

å¦‚æžœæ’ä»¶å¯¼å‡º `id`ï¼ŒKrabKrab ä¼šä½¿ç”¨å®ƒï¼Œä½†å½“å®ƒä¸Žé…ç½®çš„ id ä¸åŒ¹é…æ—¶ä¼šå‘å‡ºè­¦å‘Šã€‚

## é…ç½®

```json5
{
  plugins: {
    enabled: true,
    allow: ["voice-call"],
    deny: ["untrusted-plugin"],
    load: { paths: ["~/Projects/oss/voice-call-extension"] },
    entries: {
      "voice-call": { enabled: true, config: { provider: "twilio" } },
    },
  },
}
```

å­—æ®µï¼š

- `enabled`ï¼šä¸»å¼€å…³ï¼ˆé»˜è®¤ï¼štrueï¼‰
- `allow`ï¼šå…è®¸åˆ—è¡¨ï¼ˆå¯é€‰ï¼‰
- `deny`ï¼šæ‹’ç»åˆ—è¡¨ï¼ˆå¯é€‰ï¼›deny ä¼˜å…ˆï¼‰
- `load.paths`ï¼šé¢å¤–çš„æ’ä»¶æ–‡ä»¶/ç›®å½•
- `entries.<id>`ï¼šæ¯ä¸ªæ’ä»¶çš„å¼€å…³ + é…ç½®

é…ç½®æ›´æ”¹**éœ€è¦é‡å¯ Gateway ç½‘å…³**ã€‚

éªŒè¯è§„åˆ™ï¼ˆä¸¥æ ¼ï¼‰ï¼š

- `entries`ã€`allow`ã€`deny` æˆ– `slots` ä¸­çš„æœªçŸ¥æ’ä»¶ id æ˜¯**é”™è¯¯**ã€‚
- æœªçŸ¥çš„ `channels.<id>` é”®æ˜¯**é”™è¯¯**ï¼Œé™¤éžæ’ä»¶æ¸…å•å£°æ˜Žäº†æ¸ é“ idã€‚
- æ’ä»¶é…ç½®ä½¿ç”¨åµŒå…¥åœ¨ `krabkrab.plugin.json`ï¼ˆ`configSchema`ï¼‰ä¸­çš„ JSON Schema è¿›è¡ŒéªŒè¯ã€‚
- å¦‚æžœæ’ä»¶è¢«ç¦ç”¨ï¼Œå…¶é…ç½®ä¼šä¿ç•™å¹¶å‘å‡º**è­¦å‘Š**ã€‚

## æ’ä»¶æ§½ä½ï¼ˆç‹¬å ç±»åˆ«ï¼‰

æŸäº›æ’ä»¶ç±»åˆ«æ˜¯**ç‹¬å çš„**ï¼ˆä¸€æ¬¡åªæœ‰ä¸€ä¸ªæ´»è·ƒï¼‰ã€‚ä½¿ç”¨ `plugins.slots` é€‰æ‹©å“ªä¸ªæ’ä»¶æ‹¥æœ‰è¯¥æ§½ä½ï¼š

```json5
{
  plugins: {
    slots: {
      memory: "memory-core", // or "none" to disable memory plugins
    },
  },
}
```

å¦‚æžœå¤šä¸ªæ’ä»¶å£°æ˜Ž `kind: "memory"`ï¼Œåªæœ‰é€‰å®šçš„é‚£ä¸ªåŠ è½½ã€‚å…¶ä»–çš„è¢«ç¦ç”¨å¹¶å¸¦æœ‰è¯Šæ–­ä¿¡æ¯ã€‚

## æŽ§åˆ¶ç•Œé¢ï¼ˆschema + æ ‡ç­¾ï¼‰

æŽ§åˆ¶ç•Œé¢ä½¿ç”¨ `config.schema`ï¼ˆJSON Schema + `uiHints`ï¼‰æ¥æ¸²æŸ“æ›´å¥½çš„è¡¨å•ã€‚

KrabKrab åœ¨è¿è¡Œæ—¶æ ¹æ®å‘çŽ°çš„æ’ä»¶å¢žå¼º `uiHints`ï¼š

- ä¸º `plugins.entries.<id>` / `.enabled` / `.config` æ·»åŠ æ¯æ’ä»¶æ ‡ç­¾
- åœ¨ä»¥ä¸‹ä½ç½®åˆå¹¶å¯é€‰çš„æ’ä»¶æä¾›çš„é…ç½®å­—æ®µæç¤ºï¼š
  `plugins.entries.<id>.config.<field>`

å¦‚æžœä½ å¸Œæœ›æ’ä»¶é…ç½®å­—æ®µæ˜¾ç¤ºè‰¯å¥½çš„æ ‡ç­¾/å ä½ç¬¦ï¼ˆå¹¶å°†å¯†é’¥æ ‡è®°ä¸ºæ•æ„Ÿï¼‰ï¼Œè¯·åœ¨æ’ä»¶æ¸…å•ä¸­æä¾› `uiHints` å’Œ JSON Schemaã€‚

ç¤ºä¾‹ï¼š

```json
{
  "id": "my-plugin",
  "configSchema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "apiKey": { "type": "string" },
      "region": { "type": "string" }
    }
  },
  "uiHints": {
    "apiKey": { "label": "API Key", "sensitive": true },
    "region": { "label": "Region", "placeholder": "us-east-1" }
  }
}
```

## CLI

```bash
krabkrab plugins list
krabkrab plugins info <id>
krabkrab plugins install <path>                 # copy a local file/dir into ~/.krabkrab/extensions/<id>
krabkrab plugins install ./extensions/voice-call # relative path ok
krabkrab plugins install ./plugin.tgz           # install from a local tarball
krabkrab plugins install ./plugin.zip           # install from a local zip
krabkrab plugins install -l ./extensions/voice-call # link (no copy) for dev
krabkrab plugins install @krabkrab/voice-call # install from npm
krabkrab plugins update <id>
krabkrab plugins update --all
krabkrab plugins enable <id>
krabkrab plugins disable <id>
krabkrab plugins doctor
```

`plugins update` ä»…é€‚ç”¨äºŽåœ¨ `plugins.installs` ä¸‹è·Ÿè¸ªçš„ npm å®‰è£…ã€‚

æ’ä»¶ä¹Ÿå¯ä»¥æ³¨å†Œè‡ªå·±çš„é¡¶çº§å‘½ä»¤ï¼ˆä¾‹å¦‚ï¼š`krabkrab voicecall`ï¼‰ã€‚

## æ’ä»¶ APIï¼ˆæ¦‚è¿°ï¼‰

æ’ä»¶å¯¼å‡ºä»¥ä¸‹ä¹‹ä¸€ï¼š

- å‡½æ•°ï¼š`(api) => { ... }`
- å¯¹è±¡ï¼š`{ id, name, configSchema, register(api) { ... } }`

## æ’ä»¶é’©å­

æ’ä»¶å¯ä»¥é™„å¸¦é’©å­å¹¶åœ¨è¿è¡Œæ—¶æ³¨å†Œå®ƒä»¬ã€‚è¿™è®©æ’ä»¶å¯ä»¥æ†ç»‘äº‹ä»¶é©±åŠ¨çš„è‡ªåŠ¨åŒ–ï¼Œè€Œæ— éœ€å•ç‹¬å®‰è£…é’©å­åŒ…ã€‚

### ç¤ºä¾‹

```
import { registerPluginHooksFromDir } from "krabkrab/plugin-sdk";

export default function register(api) {
  registerPluginHooksFromDir(api, "./hooks");
}
```

æ³¨æ„äº‹é¡¹ï¼š

- é’©å­ç›®å½•éµå¾ªæ­£å¸¸çš„é’©å­ç»“æž„ï¼ˆ`HOOK.md` + `handler.ts`ï¼‰ã€‚
- é’©å­èµ„æ ¼è§„åˆ™ä»ç„¶é€‚ç”¨ï¼ˆæ“ä½œç³»ç»Ÿ/äºŒè¿›åˆ¶æ–‡ä»¶/çŽ¯å¢ƒ/é…ç½®è¦æ±‚ï¼‰ã€‚
- æ’ä»¶ç®¡ç†çš„é’©å­åœ¨ `krabkrab hooks list` ä¸­æ˜¾ç¤ºä¸º `plugin:<id>`ã€‚
- ä½ ä¸èƒ½é€šè¿‡ `krabkrab hooks` å¯ç”¨/ç¦ç”¨æ’ä»¶ç®¡ç†çš„é’©å­ï¼›è€Œæ˜¯å¯ç”¨/ç¦ç”¨æ’ä»¶ã€‚

## æä¾›å•†æ’ä»¶ï¼ˆæ¨¡åž‹è®¤è¯ï¼‰

æ’ä»¶å¯ä»¥æ³¨å†Œ**æ¨¡åž‹æä¾›å•†è®¤è¯**æµç¨‹ï¼Œä»¥ä¾¿ç”¨æˆ·å¯ä»¥åœ¨ KrabKrab å†…è¿è¡Œ OAuth æˆ– API å¯†é’¥è®¾ç½®ï¼ˆæ— éœ€å¤–éƒ¨è„šæœ¬ï¼‰ã€‚

é€šè¿‡ `api.registerProvider(...)` æ³¨å†Œæä¾›å•†ã€‚æ¯ä¸ªæä¾›å•†æš´éœ²ä¸€ä¸ªæˆ–å¤šä¸ªè®¤è¯æ–¹æ³•ï¼ˆOAuthã€API å¯†é’¥ã€è®¾å¤‡ç ç­‰ï¼‰ã€‚è¿™äº›æ–¹æ³•é©±åŠ¨ï¼š

- `krabkrab models auth login --provider <id> [--method <id>]`

ç¤ºä¾‹ï¼š

```ts
api.registerProvider({
  id: "acme",
  label: "AcmeAI",
  auth: [
    {
      id: "oauth",
      label: "OAuth",
      kind: "oauth",
      run: async (ctx) => {
        // Run OAuth flow and return auth profiles.
        return {
          profiles: [
            {
              profileId: "acme:default",
              credential: {
                type: "oauth",
                provider: "acme",
                access: "...",
                refresh: "...",
                expires: Date.now() + 3600 * 1000,
              },
            },
          ],
          defaultModel: "acme/opus-1",
        };
      },
    },
  ],
});
```

æ³¨æ„äº‹é¡¹ï¼š

- `run` æŽ¥æ”¶å¸¦æœ‰ `prompter`ã€`runtime`ã€`openUrl` å’Œ `oauth.createVpsAwareHandlers` è¾…åŠ©å·¥å…·çš„ `ProviderAuthContext`ã€‚
- å½“éœ€è¦æ·»åŠ é»˜è®¤æ¨¡åž‹æˆ–æä¾›å•†é…ç½®æ—¶è¿”å›ž `configPatch`ã€‚
- è¿”å›ž `defaultModel` ä»¥ä¾¿ `--set-default` å¯ä»¥æ›´æ–°æ™ºèƒ½ä½“é»˜è®¤å€¼ã€‚

### æ³¨å†Œæ¶ˆæ¯æ¸ é“

æ’ä»¶å¯ä»¥æ³¨å†Œ**æ¸ é“æ’ä»¶**ï¼Œå…¶è¡Œä¸ºç±»ä¼¼äºŽå†…ç½®æ¸ é“ï¼ˆWhatsAppã€Telegram ç­‰ï¼‰ã€‚æ¸ é“é…ç½®ä½äºŽ `channels.<id>` ä¸‹ï¼Œç”±ä½ çš„æ¸ é“æ’ä»¶ä»£ç éªŒè¯ã€‚

```ts
const myChannel = {
  id: "acmechat",
  meta: {
    id: "acmechat",
    label: "AcmeChat",
    selectionLabel: "AcmeChat (API)",
    docsPath: "/channels/acmechat",
    blurb: "demo channel plugin.",
    aliases: ["acme"],
  },
  capabilities: { chatTypes: ["direct"] },
  config: {
    listAccountIds: (cfg) => Object.keys(cfg.channels?.acmechat?.accounts ?? {}),
    resolveAccount: (cfg, accountId) =>
      cfg.channels?.acmechat?.accounts?.[accountId ?? "default"] ?? {
        accountId,
      },
  },
  outbound: {
    deliveryMode: "direct",
    sendText: async () => ({ ok: true }),
  },
};

export default function (api) {
  api.registerChannel({ plugin: myChannel });
}
```

æ³¨æ„äº‹é¡¹ï¼š

- å°†é…ç½®æ”¾åœ¨ `channels.<id>` ä¸‹ï¼ˆè€Œä¸æ˜¯ `plugins.entries`ï¼‰ã€‚
- `meta.label` ç”¨äºŽ CLI/UI åˆ—è¡¨ä¸­çš„æ ‡ç­¾ã€‚
- `meta.aliases` æ·»åŠ ç”¨äºŽè§„èŒƒåŒ–å’Œ CLI è¾“å…¥çš„å¤‡ç”¨ idã€‚
- `meta.preferOver` åˆ—å‡ºå½“ä¸¤è€…éƒ½é…ç½®æ—¶è¦è·³è¿‡è‡ªåŠ¨å¯ç”¨çš„æ¸ é“ idã€‚
- `meta.detailLabel` å’Œ `meta.systemImage` è®© UI æ˜¾ç¤ºæ›´ä¸°å¯Œçš„æ¸ é“æ ‡ç­¾/å›¾æ ‡ã€‚

### ç¼–å†™æ–°çš„æ¶ˆæ¯æ¸ é“ï¼ˆåˆ†æ­¥æŒ‡å—ï¼‰

å½“ä½ æƒ³è¦ä¸€ä¸ª**æ–°çš„èŠå¤©ç•Œé¢**ï¼ˆ"æ¶ˆæ¯æ¸ é“"ï¼‰è€Œä¸æ˜¯æ¨¡åž‹æä¾›å•†æ—¶ä½¿ç”¨æ­¤æ–¹æ³•ã€‚
æ¨¡åž‹æä¾›å•†æ–‡æ¡£ä½äºŽ `/providers/*` ä¸‹ã€‚

1. é€‰æ‹© id + é…ç½®ç»“æž„

- æ‰€æœ‰æ¸ é“é…ç½®ä½äºŽ `channels.<id>` ä¸‹ã€‚
- å¯¹äºŽå¤šè´¦æˆ·è®¾ç½®ï¼Œä¼˜å…ˆä½¿ç”¨ `channels.<id>.accounts.<accountId>`ã€‚

2. å®šä¹‰æ¸ é“å…ƒæ•°æ®

- `meta.label`ã€`meta.selectionLabel`ã€`meta.docsPath`ã€`meta.blurb` æŽ§åˆ¶ CLI/UI åˆ—è¡¨ã€‚
- `meta.docsPath` åº”æŒ‡å‘åƒ `/channels/<id>` è¿™æ ·çš„æ–‡æ¡£é¡µé¢ã€‚
- `meta.preferOver` è®©æ’ä»¶æ›¿æ¢å¦ä¸€ä¸ªæ¸ é“ï¼ˆè‡ªåŠ¨å¯ç”¨ä¼˜å…ˆé€‰æ‹©å®ƒï¼‰ã€‚
- `meta.detailLabel` å’Œ `meta.systemImage` è¢« UI ç”¨äºŽè¯¦ç»†æ–‡æœ¬/å›¾æ ‡ã€‚

3. å®žçŽ°å¿…éœ€çš„é€‚é…å™¨

- `config.listAccountIds` + `config.resolveAccount`
- `capabilities`ï¼ˆèŠå¤©ç±»åž‹ã€åª’ä½“ã€çº¿ç¨‹ç­‰ï¼‰
- `outbound.deliveryMode` + `outbound.sendText`ï¼ˆç”¨äºŽåŸºæœ¬å‘é€ï¼‰

4. æ ¹æ®éœ€è¦æ·»åŠ å¯é€‰é€‚é…å™¨

- `setup`ï¼ˆå‘å¯¼ï¼‰ã€`security`ï¼ˆç§ä¿¡ç­–ç•¥ï¼‰ã€`status`ï¼ˆå¥åº·/è¯Šæ–­ï¼‰
- `gateway`ï¼ˆå¯åŠ¨/åœæ­¢/ç™»å½•ï¼‰ã€`mentions`ã€`threading`ã€`streaming`
- `actions`ï¼ˆæ¶ˆæ¯æ“ä½œï¼‰ã€`commands`ï¼ˆåŽŸç”Ÿå‘½ä»¤è¡Œä¸ºï¼‰

5. åœ¨æ’ä»¶ä¸­æ³¨å†Œæ¸ é“

- `api.registerChannel({ plugin })`

æœ€å°é…ç½®ç¤ºä¾‹ï¼š

```json5
{
  channels: {
    acmechat: {
      accounts: {
        default: { token: "ACME_TOKEN", enabled: true },
      },
    },
  },
}
```

æœ€å°æ¸ é“æ’ä»¶ï¼ˆä»…å‡ºç«™ï¼‰ï¼š

```ts
const plugin = {
  id: "acmechat",
  meta: {
    id: "acmechat",
    label: "AcmeChat",
    selectionLabel: "AcmeChat (API)",
    docsPath: "/channels/acmechat",
    blurb: "AcmeChat messaging channel.",
    aliases: ["acme"],
  },
  capabilities: { chatTypes: ["direct"] },
  config: {
    listAccountIds: (cfg) => Object.keys(cfg.channels?.acmechat?.accounts ?? {}),
    resolveAccount: (cfg, accountId) =>
      cfg.channels?.acmechat?.accounts?.[accountId ?? "default"] ?? {
        accountId,
      },
  },
  outbound: {
    deliveryMode: "direct",
    sendText: async ({ text }) => {
      // deliver `text` to your channel here
      return { ok: true };
    },
  },
};

export default function (api) {
  api.registerChannel({ plugin });
}
```

åŠ è½½æ’ä»¶ï¼ˆæ‰©å±•ç›®å½•æˆ– `plugins.load.paths`ï¼‰ï¼Œé‡å¯ Gateway ç½‘å…³ï¼Œç„¶åŽåœ¨é…ç½®ä¸­é…ç½® `channels.<id>`ã€‚

### æ™ºèƒ½ä½“å·¥å…·

å‚è§ä¸“é—¨æŒ‡å—ï¼š[æ’ä»¶æ™ºèƒ½ä½“å·¥å…·](/plugins/agent-tools)ã€‚

### æ³¨å†Œ Gateway ç½‘å…³ RPC æ–¹æ³•

```ts
export default function (api) {
  api.registerGatewayMethod("myplugin.status", ({ respond }) => {
    respond(true, { ok: true });
  });
}
```

### æ³¨å†Œ CLI å‘½ä»¤

```ts
export default function (api) {
  api.registerCli(
    ({ program }) => {
      program.command("mycmd").action(() => {
        console.log("Hello");
      });
    },
    { commands: ["mycmd"] },
  );
}
```

### æ³¨å†Œè‡ªåŠ¨å›žå¤å‘½ä»¤

æ’ä»¶å¯ä»¥æ³¨å†Œè‡ªå®šä¹‰æ–œæ å‘½ä»¤ï¼Œ**æ— éœ€è°ƒç”¨ AI æ™ºèƒ½ä½“**å³å¯æ‰§è¡Œã€‚è¿™å¯¹äºŽåˆ‡æ¢å‘½ä»¤ã€çŠ¶æ€æ£€æŸ¥æˆ–ä¸éœ€è¦ LLM å¤„ç†çš„å¿«é€Ÿæ“ä½œå¾ˆæœ‰ç”¨ã€‚

```ts
export default function (api) {
  api.registerCommand({
    name: "mystatus",
    description: "Show plugin status",
    handler: (ctx) => ({
      text: `Plugin is running! Channel: ${ctx.channel}`,
    }),
  });
}
```

å‘½ä»¤å¤„ç†ç¨‹åºä¸Šä¸‹æ–‡ï¼š

- `senderId`ï¼šå‘é€è€…çš„ IDï¼ˆå¦‚å¯ç”¨ï¼‰
- `channel`ï¼šå‘é€å‘½ä»¤çš„æ¸ é“
- `isAuthorizedSender`ï¼šå‘é€è€…æ˜¯å¦æ˜¯æŽˆæƒç”¨æˆ·
- `args`ï¼šå‘½ä»¤åŽä¼ é€’çš„å‚æ•°ï¼ˆå¦‚æžœ `acceptsArgs: true`ï¼‰
- `commandBody`ï¼šå®Œæ•´çš„å‘½ä»¤æ–‡æœ¬
- `config`ï¼šå½“å‰ KrabKrab é…ç½®

å‘½ä»¤é€‰é¡¹ï¼š

- `name`ï¼šå‘½ä»¤åç§°ï¼ˆä¸å¸¦å‰å¯¼ `/`ï¼‰
- `description`ï¼šå‘½ä»¤åˆ—è¡¨ä¸­æ˜¾ç¤ºçš„å¸®åŠ©æ–‡æœ¬
- `acceptsArgs`ï¼šå‘½ä»¤æ˜¯å¦æŽ¥å—å‚æ•°ï¼ˆé»˜è®¤ï¼šfalseï¼‰ã€‚å¦‚æžœä¸º false ä¸”æä¾›äº†å‚æ•°ï¼Œå‘½ä»¤ä¸ä¼šåŒ¹é…ï¼Œæ¶ˆæ¯ä¼šä¼ é€’ç»™å…¶ä»–å¤„ç†ç¨‹åº
- `requireAuth`ï¼šæ˜¯å¦éœ€è¦æŽˆæƒå‘é€è€…ï¼ˆé»˜è®¤ï¼štrueï¼‰
- `handler`ï¼šè¿”å›ž `{ text: string }` çš„å‡½æ•°ï¼ˆå¯ä»¥æ˜¯å¼‚æ­¥çš„ï¼‰

å¸¦æŽˆæƒå’Œå‚æ•°çš„ç¤ºä¾‹ï¼š

```ts
api.registerCommand({
  name: "setmode",
  description: "Set plugin mode",
  acceptsArgs: true,
  requireAuth: true,
  handler: async (ctx) => {
    const mode = ctx.args?.trim() || "default";
    await saveMode(mode);
    return { text: `Mode set to: ${mode}` };
  },
});
```

æ³¨æ„äº‹é¡¹ï¼š

- æ’ä»¶å‘½ä»¤åœ¨å†…ç½®å‘½ä»¤å’Œ AI æ™ºèƒ½ä½“**ä¹‹å‰**å¤„ç†
- å‘½ä»¤å…¨å±€æ³¨å†Œï¼Œé€‚ç”¨äºŽæ‰€æœ‰æ¸ é“
- å‘½ä»¤åç§°ä¸åŒºåˆ†å¤§å°å†™ï¼ˆ`/MyStatus` åŒ¹é… `/mystatus`ï¼‰
- å‘½ä»¤åç§°å¿…é¡»ä»¥å­—æ¯å¼€å¤´ï¼Œåªèƒ½åŒ…å«å­—æ¯ã€æ•°å­—ã€è¿žå­—ç¬¦å’Œä¸‹åˆ’çº¿
- ä¿ç•™çš„å‘½ä»¤åç§°ï¼ˆå¦‚ `help`ã€`status`ã€`reset` ç­‰ï¼‰ä¸èƒ½è¢«æ’ä»¶è¦†ç›–
- è·¨æ’ä»¶çš„é‡å¤å‘½ä»¤æ³¨å†Œå°†å¤±è´¥å¹¶æ˜¾ç¤ºè¯Šæ–­é”™è¯¯

### æ³¨å†ŒåŽå°æœåŠ¡

```ts
export default function (api) {
  api.registerService({
    id: "my-service",
    start: () => api.logger.info("ready"),
    stop: () => api.logger.info("bye"),
  });
}
```

## å‘½åçº¦å®š

- Gateway ç½‘å…³æ–¹æ³•ï¼š`pluginId.action`ï¼ˆä¾‹å¦‚ï¼š`voicecall.status`ï¼‰
- å·¥å…·ï¼š`snake_case`ï¼ˆä¾‹å¦‚ï¼š`voice_call`ï¼‰
- CLI å‘½ä»¤ï¼škebab æˆ– camelï¼Œä½†é¿å…ä¸Žæ ¸å¿ƒå‘½ä»¤å†²çª

## Skills

æ’ä»¶å¯ä»¥åœ¨ä»“åº“ä¸­é™„å¸¦ Skillsï¼ˆ`skills/<name>/SKILL.md`ï¼‰ã€‚
ä½¿ç”¨ `plugins.entries.<id>.enabled`ï¼ˆæˆ–å…¶ä»–é…ç½®é—¨æŽ§ï¼‰å¯ç”¨å®ƒï¼Œå¹¶ç¡®ä¿å®ƒå­˜åœ¨äºŽä½ çš„å·¥ä½œåŒº/æ‰˜ç®¡ Skills ä½ç½®ã€‚

## åˆ†å‘ï¼ˆnpmï¼‰

æŽ¨èçš„æ‰“åŒ…æ–¹å¼ï¼š

- ä¸»åŒ…ï¼š`krabkrab`ï¼ˆæœ¬ä»“åº“ï¼‰
- æ’ä»¶ï¼š`@krabkrab/*` ä¸‹çš„ç‹¬ç«‹ npm åŒ…ï¼ˆä¾‹å¦‚ï¼š`@krabkrab/voice-call`ï¼‰

å‘å¸ƒå¥‘çº¦ï¼š

- æ’ä»¶ `package.json` å¿…é¡»åŒ…å«å¸¦æœ‰ä¸€ä¸ªæˆ–å¤šä¸ªå…¥å£æ–‡ä»¶çš„ `krabkrab.extensions`ã€‚
- å…¥å£æ–‡ä»¶å¯ä»¥æ˜¯ `.js` æˆ– `.ts`ï¼ˆjiti åœ¨è¿è¡Œæ—¶åŠ è½½ TSï¼‰ã€‚
- `krabkrab plugins install <npm-spec>` ä½¿ç”¨ `npm pack`ï¼Œæå–åˆ° `~/.krabkrab/extensions/<id>/`ï¼Œå¹¶åœ¨é…ç½®ä¸­å¯ç”¨å®ƒã€‚
- é…ç½®é”®ç¨³å®šæ€§ï¼šä½œç”¨åŸŸåŒ…è¢«è§„èŒƒåŒ–ä¸º `plugins.entries.*` çš„**æ— ä½œç”¨åŸŸ** idã€‚

## ç¤ºä¾‹æ’ä»¶ï¼šVoice Call

æœ¬ä»“åº“åŒ…å«ä¸€ä¸ªè¯­éŸ³é€šè¯æ’ä»¶ï¼ˆTwilio æˆ– log å›žé€€ï¼‰ï¼š

- æºç ï¼š`extensions/voice-call`
- Skillsï¼š`skills/voice-call`
- CLIï¼š`krabkrab voicecall start|status`
- å·¥å…·ï¼š`voice_call`
- RPCï¼š`voicecall.start`ã€`voicecall.status`
- é…ç½®ï¼ˆtwilioï¼‰ï¼š`provider: "twilio"` + `twilio.accountSid/authToken/from`ï¼ˆå¯é€‰ `statusCallbackUrl`ã€`twimlUrl`ï¼‰
- é…ç½®ï¼ˆdevï¼‰ï¼š`provider: "log"`ï¼ˆæ— ç½‘ç»œï¼‰

å‚è§ [Voice Call](/plugins/voice-call) å’Œ `extensions/voice-call/README.md` äº†è§£è®¾ç½®å’Œç”¨æ³•ã€‚

## å®‰å…¨æ³¨æ„äº‹é¡¹

æ’ä»¶ä¸Ž Gateway ç½‘å…³åœ¨åŒä¸€è¿›ç¨‹ä¸­è¿è¡Œã€‚å°†å®ƒä»¬è§†ä¸ºå—ä¿¡ä»»çš„ä»£ç ï¼š

- åªå®‰è£…ä½ ä¿¡ä»»çš„æ’ä»¶ã€‚
- ä¼˜å…ˆä½¿ç”¨ `plugins.allow` å…è®¸åˆ—è¡¨ã€‚
- æ›´æ”¹åŽé‡å¯ Gateway ç½‘å…³ã€‚

## æµ‹è¯•æ’ä»¶

æ’ä»¶å¯ä»¥ï¼ˆä¹Ÿåº”è¯¥ï¼‰é™„å¸¦æµ‹è¯•ï¼š

- ä»“åº“å†…æ’ä»¶å¯ä»¥åœ¨ `src/**` ä¸‹ä¿ç•™ Vitest æµ‹è¯•ï¼ˆä¾‹å¦‚ï¼š`src/plugins/voice-call.plugin.test.ts`ï¼‰ã€‚
- å•ç‹¬å‘å¸ƒçš„æ’ä»¶åº”è¿è¡Œè‡ªå·±çš„ CIï¼ˆlint/æž„å»º/æµ‹è¯•ï¼‰å¹¶éªŒè¯ `krabkrab.extensions` æŒ‡å‘æž„å»ºçš„å…¥å£ç‚¹ï¼ˆ`dist/index.js`ï¼‰ã€‚

