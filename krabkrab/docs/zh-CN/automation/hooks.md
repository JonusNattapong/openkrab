---
read_when:
  - ä½ æƒ³ä¸º /newã€/resetã€/stop å’Œæ™ºèƒ½ä½“ç”Ÿå‘½å‘¨æœŸäº‹ä»¶å®žçŽ°äº‹ä»¶é©±åŠ¨è‡ªåŠ¨åŒ–
  - ä½ æƒ³æž„å»ºã€å®‰è£…æˆ–è°ƒè¯• hooks
summary: Hooksï¼šç”¨äºŽå‘½ä»¤å’Œç”Ÿå‘½å‘¨æœŸäº‹ä»¶çš„äº‹ä»¶é©±åŠ¨è‡ªåŠ¨åŒ–
title: Hooks
x-i18n:
  generated_at: "2026-02-03T07:50:59Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 853227a0f1abd20790b425fa64dda60efc6b5f93c1b13ecd2dcb788268f71d79
  source_path: automation/hooks.md
  workflow: 15
---

# Hooks

Hooks æä¾›äº†ä¸€ä¸ªå¯æ‰©å±•çš„äº‹ä»¶é©±åŠ¨ç³»ç»Ÿï¼Œç”¨äºŽå“åº”æ™ºèƒ½ä½“å‘½ä»¤å’Œäº‹ä»¶è‡ªåŠ¨æ‰§è¡Œæ“ä½œã€‚Hooks ä»Žç›®å½•ä¸­è‡ªåŠ¨å‘çŽ°ï¼Œå¯ä»¥é€šè¿‡ CLI å‘½ä»¤ç®¡ç†ï¼Œç±»ä¼¼äºŽ KrabKrab ä¸­ Skills çš„å·¥ä½œæ–¹å¼ã€‚

## å…¥é—¨æŒ‡å—

Hooks æ˜¯åœ¨äº‹ä»¶å‘ç”Ÿæ—¶è¿è¡Œçš„å°è„šæœ¬ã€‚æœ‰ä¸¤ç§ç±»åž‹ï¼š

- **Hooks**ï¼ˆæœ¬é¡µï¼‰ï¼šå½“æ™ºèƒ½ä½“äº‹ä»¶è§¦å‘æ—¶åœ¨ Gateway ç½‘å…³å†…è¿è¡Œï¼Œå¦‚ `/new`ã€`/reset`ã€`/stop` æˆ–ç”Ÿå‘½å‘¨æœŸäº‹ä»¶ã€‚
- **Webhooks**ï¼šå¤–éƒ¨ HTTP webhooksï¼Œè®©å…¶ä»–ç³»ç»Ÿè§¦å‘ KrabKrab ä¸­çš„å·¥ä½œã€‚å‚è§ [Webhook Hooks](/automation/webhook) æˆ–ä½¿ç”¨ `krabkrab webhooks` èŽ·å– Gmail åŠ©æ‰‹å‘½ä»¤ã€‚

Hooks ä¹Ÿå¯ä»¥æ†ç»‘åœ¨æ’ä»¶ä¸­ï¼›å‚è§ [æ’ä»¶](/tools/plugin#plugin-hooks)ã€‚

å¸¸è§ç”¨é€”ï¼š

- é‡ç½®ä¼šè¯æ—¶ä¿å­˜è®°å¿†å¿«ç…§
- ä¿ç•™å‘½ä»¤å®¡è®¡è·Ÿè¸ªç”¨äºŽæ•…éšœæŽ’é™¤æˆ–åˆè§„
- ä¼šè¯å¼€å§‹æˆ–ç»“æŸæ—¶è§¦å‘åŽç»­è‡ªåŠ¨åŒ–
- äº‹ä»¶è§¦å‘æ—¶å‘æ™ºèƒ½ä½“å·¥ä½œåŒºå†™å…¥æ–‡ä»¶æˆ–è°ƒç”¨å¤–éƒ¨ API

å¦‚æžœä½ èƒ½å†™ä¸€ä¸ªå°çš„ TypeScript å‡½æ•°ï¼Œä½ å°±èƒ½å†™ä¸€ä¸ª hookã€‚Hooks ä¼šè‡ªåŠ¨å‘çŽ°ï¼Œä½ å¯ä»¥é€šè¿‡ CLI å¯ç”¨æˆ–ç¦ç”¨å®ƒä»¬ã€‚

## æ¦‚è¿°

hooks ç³»ç»Ÿå…è®¸ä½ ï¼š

- åœ¨å‘å‡º `/new` æ—¶å°†ä¼šè¯ä¸Šä¸‹æ–‡ä¿å­˜åˆ°è®°å¿†
- è®°å½•æ‰€æœ‰å‘½ä»¤ä»¥ä¾›å®¡è®¡
- åœ¨æ™ºèƒ½ä½“ç”Ÿå‘½å‘¨æœŸäº‹ä»¶ä¸Šè§¦å‘è‡ªå®šä¹‰è‡ªåŠ¨åŒ–
- åœ¨ä¸ä¿®æ”¹æ ¸å¿ƒä»£ç çš„æƒ…å†µä¸‹æ‰©å±• KrabKrab çš„è¡Œä¸º

## å…¥é—¨

### æ†ç»‘çš„ Hooks

KrabKrab é™„å¸¦ä¸‰ä¸ªè‡ªåŠ¨å‘çŽ°çš„æ†ç»‘ hooksï¼š

- **ðŸ’¾ session-memory**ï¼šå½“ä½ å‘å‡º `/new` æ—¶å°†ä¼šè¯ä¸Šä¸‹æ–‡ä¿å­˜åˆ°æ™ºèƒ½ä½“å·¥ä½œåŒºï¼ˆé»˜è®¤ `~/.krabkrab/workspace/memory/`ï¼‰
- **ðŸ“ command-logger**ï¼šå°†æ‰€æœ‰å‘½ä»¤äº‹ä»¶è®°å½•åˆ° `~/.krabkrab/logs/commands.log`
- **ðŸš€ boot-md**ï¼šå½“ Gateway ç½‘å…³å¯åŠ¨æ—¶è¿è¡Œ `BOOT.md`ï¼ˆéœ€è¦å¯ç”¨å†…éƒ¨ hooksï¼‰

åˆ—å‡ºå¯ç”¨çš„ hooksï¼š

```bash
krabkrab hooks list
```

å¯ç”¨ä¸€ä¸ª hookï¼š

```bash
krabkrab hooks enable session-memory
```

æ£€æŸ¥ hook çŠ¶æ€ï¼š

```bash
krabkrab hooks check
```

èŽ·å–è¯¦ç»†ä¿¡æ¯ï¼š

```bash
krabkrab hooks info session-memory
```

### æ–°æ‰‹å¼•å¯¼

åœ¨æ–°æ‰‹å¼•å¯¼æœŸé—´ï¼ˆ`krabkrab onboard`ï¼‰ï¼Œä½ å°†è¢«æç¤ºå¯ç”¨æŽ¨èçš„ hooksã€‚å‘å¯¼ä¼šè‡ªåŠ¨å‘çŽ°ç¬¦åˆæ¡ä»¶çš„ hooks å¹¶å‘ˆçŽ°ä¾›é€‰æ‹©ã€‚

## Hook å‘çŽ°

Hooks ä»Žä¸‰ä¸ªç›®å½•è‡ªåŠ¨å‘çŽ°ï¼ˆæŒ‰ä¼˜å…ˆçº§é¡ºåºï¼‰ï¼š

1. **å·¥ä½œåŒº hooks**ï¼š`<workspace>/hooks/`ï¼ˆæ¯æ™ºèƒ½ä½“ï¼Œæœ€é«˜ä¼˜å…ˆçº§ï¼‰
2. **æ‰˜ç®¡ hooks**ï¼š`~/.krabkrab/hooks/`ï¼ˆç”¨æˆ·å®‰è£…ï¼Œè·¨å·¥ä½œåŒºå…±äº«ï¼‰
3. **æ†ç»‘ hooks**ï¼š`<krabkrab>/dist/hooks/bundled/`ï¼ˆéš KrabKrab é™„å¸¦ï¼‰

æ‰˜ç®¡ hook ç›®å½•å¯ä»¥æ˜¯**å•ä¸ª hook** æˆ– **hook åŒ…**ï¼ˆåŒ…ç›®å½•ï¼‰ã€‚

æ¯ä¸ª hook æ˜¯ä¸€ä¸ªåŒ…å«ä»¥ä¸‹å†…å®¹çš„ç›®å½•ï¼š

```
my-hook/
â”œâ”€â”€ HOOK.md          # å…ƒæ•°æ® + æ–‡æ¡£
â””â”€â”€ handler.ts       # å¤„ç†ç¨‹åºå®žçŽ°
```

## Hook åŒ…ï¼ˆnpm/archivesï¼‰

Hook åŒ…æ˜¯æ ‡å‡†çš„ npm åŒ…ï¼Œé€šè¿‡ `package.json` ä¸­çš„ `krabkrab.hooks` å¯¼å‡ºä¸€ä¸ªæˆ–å¤šä¸ª hooksã€‚ä½¿ç”¨ä»¥ä¸‹å‘½ä»¤å®‰è£…ï¼š

```bash
krabkrab hooks install <path-or-spec>
```

ç¤ºä¾‹ `package.json`ï¼š

```json
{
  "name": "@acme/my-hooks",
  "version": "0.1.0",
  "krabkrab": {
    "hooks": ["./hooks/my-hook", "./hooks/other-hook"]
  }
}
```

æ¯ä¸ªæ¡ç›®æŒ‡å‘åŒ…å« `HOOK.md` å’Œ `handler.ts`ï¼ˆæˆ– `index.ts`ï¼‰çš„ hook ç›®å½•ã€‚
Hook åŒ…å¯ä»¥é™„å¸¦ä¾èµ–ï¼›å®ƒä»¬å°†å®‰è£…åœ¨ `~/.krabkrab/hooks/<id>` ä¸‹ã€‚

## Hook ç»“æž„

### HOOK.md æ ¼å¼

`HOOK.md` æ–‡ä»¶åœ¨ YAML frontmatter ä¸­åŒ…å«å…ƒæ•°æ®ï¼ŒåŠ ä¸Š Markdown æ–‡æ¡£ï¼š

```markdown
---
name: my-hook
description: "Short description of what this hook does"
homepage: https://docs.krabkrab.ai/automation/hooks#my-hook
metadata:
  { "krabkrab": { "emoji": "ðŸ”—", "events": ["command:new"], "requires": { "bins": ["node"] } } }
---

# My Hook

Detailed documentation goes here...

## What It Does

- Listens for `/new` commands
- Performs some action
- Logs the result

## Requirements

- Node.js must be installed

## Configuration

No configuration needed.
```

### å…ƒæ•°æ®å­—æ®µ

`metadata.krabkrab` å¯¹è±¡æ”¯æŒï¼š

- **`emoji`**ï¼šCLI çš„æ˜¾ç¤ºè¡¨æƒ…ç¬¦å·ï¼ˆä¾‹å¦‚ `"ðŸ’¾"`ï¼‰
- **`events`**ï¼šè¦ç›‘å¬çš„äº‹ä»¶æ•°ç»„ï¼ˆä¾‹å¦‚ `["command:new", "command:reset"]`ï¼‰
- **`export`**ï¼šè¦ä½¿ç”¨çš„å‘½åå¯¼å‡ºï¼ˆé»˜è®¤ä¸º `"default"`ï¼‰
- **`homepage`**ï¼šæ–‡æ¡£ URL
- **`requires`**ï¼šå¯é€‰è¦æ±‚
  - **`bins`**ï¼šPATH ä¸­éœ€è¦çš„äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆä¾‹å¦‚ `["git", "node"]`ï¼‰
  - **`anyBins`**ï¼šè¿™äº›äºŒè¿›åˆ¶æ–‡ä»¶ä¸­è‡³å°‘æœ‰ä¸€ä¸ªå¿…é¡»å­˜åœ¨
  - **`env`**ï¼šéœ€è¦çš„çŽ¯å¢ƒå˜é‡
  - **`config`**ï¼šéœ€è¦çš„é…ç½®è·¯å¾„ï¼ˆä¾‹å¦‚ `["workspace.dir"]`ï¼‰
  - **`os`**ï¼šéœ€è¦çš„å¹³å°ï¼ˆä¾‹å¦‚ `["darwin", "linux"]`ï¼‰
- **`always`**ï¼šç»•è¿‡èµ„æ ¼æ£€æŸ¥ï¼ˆå¸ƒå°”å€¼ï¼‰
- **`install`**ï¼šå®‰è£…æ–¹æ³•ï¼ˆå¯¹äºŽæ†ç»‘ hooksï¼š`[{"id":"bundled","kind":"bundled"}]`ï¼‰

### å¤„ç†ç¨‹åºå®žçŽ°

`handler.ts` æ–‡ä»¶å¯¼å‡ºä¸€ä¸ª `HookHandler` å‡½æ•°ï¼š

```typescript
import type { HookHandler } from "../../src/hooks/hooks.js";

const myHandler: HookHandler = async (event) => {
  // Only trigger on 'new' command
  if (event.type !== "command" || event.action !== "new") {
    return;
  }

  console.log(`[my-hook] New command triggered`);
  console.log(`  Session: ${event.sessionKey}`);
  console.log(`  Timestamp: ${event.timestamp.toISOString()}`);

  // Your custom logic here

  // Optionally send message to user
  event.messages.push("âœ¨ My hook executed!");
};

export default myHandler;
```

#### äº‹ä»¶ä¸Šä¸‹æ–‡

æ¯ä¸ªäº‹ä»¶åŒ…å«ï¼š

```typescript
{
  type: 'command' | 'session' | 'agent' | 'gateway',
  action: string,              // e.g., 'new', 'reset', 'stop'
  sessionKey: string,          // Session identifier
  timestamp: Date,             // When the event occurred
  messages: string[],          // Push messages here to send to user
  context: {
    sessionEntry?: SessionEntry,
    sessionId?: string,
    sessionFile?: string,
    commandSource?: string,    // e.g., 'whatsapp', 'telegram'
    senderId?: string,
    workspaceDir?: string,
    bootstrapFiles?: WorkspaceBootstrapFile[],
    cfg?: krabkrabConfig
  }
}
```

## äº‹ä»¶ç±»åž‹

### å‘½ä»¤äº‹ä»¶

å½“å‘å‡ºæ™ºèƒ½ä½“å‘½ä»¤æ—¶è§¦å‘ï¼š

- **`command`**ï¼šæ‰€æœ‰å‘½ä»¤äº‹ä»¶ï¼ˆé€šç”¨ç›‘å¬å™¨ï¼‰
- **`command:new`**ï¼šå½“å‘å‡º `/new` å‘½ä»¤æ—¶
- **`command:reset`**ï¼šå½“å‘å‡º `/reset` å‘½ä»¤æ—¶
- **`command:stop`**ï¼šå½“å‘å‡º `/stop` å‘½ä»¤æ—¶

### æ™ºèƒ½ä½“äº‹ä»¶

- **`agent:bootstrap`**ï¼šåœ¨æ³¨å…¥å·¥ä½œåŒºå¼•å¯¼æ–‡ä»¶ä¹‹å‰ï¼ˆhooks å¯ä»¥ä¿®æ”¹ `context.bootstrapFiles`ï¼‰

### Gateway ç½‘å…³äº‹ä»¶

å½“ Gateway ç½‘å…³å¯åŠ¨æ—¶è§¦å‘ï¼š

- **`gateway:startup`**ï¼šåœ¨æ¸ é“å¯åŠ¨å’Œ hooks åŠ è½½ä¹‹åŽ

### å·¥å…·ç»“æžœ Hooksï¼ˆæ’ä»¶ APIï¼‰

è¿™äº› hooks ä¸æ˜¯äº‹ä»¶æµç›‘å¬å™¨ï¼›å®ƒä»¬è®©æ’ä»¶åœ¨ KrabKrab æŒä¹…åŒ–å·¥å…·ç»“æžœä¹‹å‰åŒæ­¥è°ƒæ•´å®ƒä»¬ã€‚

- **`tool_result_persist`**ï¼šåœ¨å·¥å…·ç»“æžœå†™å…¥ä¼šè¯è®°å½•ä¹‹å‰è½¬æ¢å®ƒä»¬ã€‚å¿…é¡»æ˜¯åŒæ­¥çš„ï¼›è¿”å›žæ›´æ–°åŽçš„å·¥å…·ç»“æžœè´Ÿè½½æˆ– `undefined` ä¿æŒåŽŸæ ·ã€‚å‚è§ [æ™ºèƒ½ä½“å¾ªçŽ¯](/concepts/agent-loop)ã€‚

### æœªæ¥äº‹ä»¶

è®¡åˆ’ä¸­çš„äº‹ä»¶ç±»åž‹ï¼š

- **`session:start`**ï¼šå½“æ–°ä¼šè¯å¼€å§‹æ—¶
- **`session:end`**ï¼šå½“ä¼šè¯ç»“æŸæ—¶
- **`agent:error`**ï¼šå½“æ™ºèƒ½ä½“é‡åˆ°é”™è¯¯æ—¶
- **`message:sent`**ï¼šå½“æ¶ˆæ¯è¢«å‘é€æ—¶
- **`message:received`**ï¼šå½“æ¶ˆæ¯è¢«æŽ¥æ”¶æ—¶

## åˆ›å»ºè‡ªå®šä¹‰ Hooks

### 1. é€‰æ‹©ä½ç½®

- **å·¥ä½œåŒº hooks**ï¼ˆ`<workspace>/hooks/`ï¼‰ï¼šæ¯æ™ºèƒ½ä½“ï¼Œæœ€é«˜ä¼˜å…ˆçº§
- **æ‰˜ç®¡ hooks**ï¼ˆ`~/.krabkrab/hooks/`ï¼‰ï¼šè·¨å·¥ä½œåŒºå…±äº«

### 2. åˆ›å»ºç›®å½•ç»“æž„

```bash
mkdir -p ~/.krabkrab/hooks/my-hook
cd ~/.krabkrab/hooks/my-hook
```

### 3. åˆ›å»º HOOK.md

```markdown
---
name: my-hook
description: "Does something useful"
metadata: { "krabkrab": { "emoji": "ðŸŽ¯", "events": ["command:new"] } }
---

# My Custom Hook

This hook does something useful when you issue `/new`.
```

### 4. åˆ›å»º handler.ts

```typescript
import type { HookHandler } from "../../src/hooks/hooks.js";

const handler: HookHandler = async (event) => {
  if (event.type !== "command" || event.action !== "new") {
    return;
  }

  console.log("[my-hook] Running!");
  // Your logic here
};

export default handler;
```

### 5. å¯ç”¨å¹¶æµ‹è¯•

```bash
# Verify hook is discovered
krabkrab hooks list

# Enable it
krabkrab hooks enable my-hook

# Restart your gateway process (menu bar app restart on macOS, or restart your dev process)

# Trigger the event
# Send /new via your messaging channel
```

## é…ç½®

### æ–°é…ç½®æ ¼å¼ï¼ˆæŽ¨èï¼‰

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "entries": {
        "session-memory": { "enabled": true },
        "command-logger": { "enabled": false }
      }
    }
  }
}
```

### æ¯ Hook é…ç½®

Hooks å¯ä»¥æœ‰è‡ªå®šä¹‰é…ç½®ï¼š

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "entries": {
        "my-hook": {
          "enabled": true,
          "env": {
            "MY_CUSTOM_VAR": "value"
          }
        }
      }
    }
  }
}
```

### é¢å¤–ç›®å½•

ä»Žé¢å¤–ç›®å½•åŠ è½½ hooksï¼š

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "load": {
        "extraDirs": ["/path/to/more/hooks"]
      }
    }
  }
}
```

### é—ç•™é…ç½®æ ¼å¼ï¼ˆä»ç„¶æ”¯æŒï¼‰

æ—§é…ç½®æ ¼å¼ä»ç„¶æœ‰æ•ˆä»¥ä¿æŒå‘åŽå…¼å®¹ï¼š

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "handlers": [
        {
          "event": "command:new",
          "module": "./hooks/handlers/my-handler.ts",
          "export": "default"
        }
      ]
    }
  }
}
```

**è¿ç§»**ï¼šå¯¹æ–° hooks ä½¿ç”¨åŸºäºŽå‘çŽ°çš„æ–°ç³»ç»Ÿã€‚é—ç•™å¤„ç†ç¨‹åºåœ¨åŸºäºŽç›®å½•çš„ hooks ä¹‹åŽåŠ è½½ã€‚

## CLI å‘½ä»¤

### åˆ—å‡º Hooks

```bash
# List all hooks
krabkrab hooks list

# Show only eligible hooks
krabkrab hooks list --eligible

# Verbose output (show missing requirements)
krabkrab hooks list --verbose

# JSON output
krabkrab hooks list --json
```

### Hook ä¿¡æ¯

```bash
# Show detailed info about a hook
krabkrab hooks info session-memory

# JSON output
krabkrab hooks info session-memory --json
```

### æ£€æŸ¥èµ„æ ¼

```bash
# Show eligibility summary
krabkrab hooks check

# JSON output
krabkrab hooks check --json
```

### å¯ç”¨/ç¦ç”¨

```bash
# Enable a hook
krabkrab hooks enable session-memory

# Disable a hook
krabkrab hooks disable command-logger
```

## æ†ç»‘çš„ Hooks

### session-memory

å½“ä½ å‘å‡º `/new` æ—¶å°†ä¼šè¯ä¸Šä¸‹æ–‡ä¿å­˜åˆ°è®°å¿†ã€‚

**äº‹ä»¶**ï¼š`command:new`

**è¦æ±‚**ï¼šå¿…é¡»é…ç½® `workspace.dir`

**è¾“å‡º**ï¼š`<workspace>/memory/YYYY-MM-DD-slug.md`ï¼ˆé»˜è®¤ä¸º `~/.krabkrab/workspace`ï¼‰

**åŠŸèƒ½**ï¼š

1. ä½¿ç”¨é¢„é‡ç½®ä¼šè¯æ¡ç›®å®šä½æ­£ç¡®çš„è®°å½•
2. æå–æœ€åŽ 15 è¡Œå¯¹è¯
3. ä½¿ç”¨ LLM ç”Ÿæˆæè¿°æ€§æ–‡ä»¶å slug
4. å°†ä¼šè¯å…ƒæ•°æ®ä¿å­˜åˆ°å¸¦æ—¥æœŸçš„è®°å¿†æ–‡ä»¶

**ç¤ºä¾‹è¾“å‡º**ï¼š

```markdown
# Session: 2026-01-16 14:30:00 UTC

- **Session Key**: agent:main:main
- **Session ID**: abc123def456
- **Source**: telegram
```

**æ–‡ä»¶åç¤ºä¾‹**ï¼š

- `2026-01-16-vendor-pitch.md`
- `2026-01-16-api-design.md`
- `2026-01-16-1430.md`ï¼ˆå¦‚æžœ slug ç”Ÿæˆå¤±è´¥åˆ™å›žé€€åˆ°æ—¶é—´æˆ³ï¼‰

**å¯ç”¨**ï¼š

```bash
krabkrab hooks enable session-memory
```

### command-logger

å°†æ‰€æœ‰å‘½ä»¤äº‹ä»¶è®°å½•åˆ°é›†ä¸­å®¡è®¡æ–‡ä»¶ã€‚

**äº‹ä»¶**ï¼š`command`

**è¦æ±‚**ï¼šæ— 

**è¾“å‡º**ï¼š`~/.krabkrab/logs/commands.log`

**åŠŸèƒ½**ï¼š

1. æ•èŽ·äº‹ä»¶è¯¦æƒ…ï¼ˆå‘½ä»¤æ“ä½œã€æ—¶é—´æˆ³ã€ä¼šè¯é”®ã€å‘é€è€… IDã€æ¥æºï¼‰
2. ä»¥ JSONL æ ¼å¼è¿½åŠ åˆ°æ—¥å¿—æ–‡ä»¶
3. åœ¨åŽå°é™é»˜è¿è¡Œ

**ç¤ºä¾‹æ—¥å¿—æ¡ç›®**ï¼š

```jsonl
{"timestamp":"2026-01-16T14:30:00.000Z","action":"new","sessionKey":"agent:main:main","senderId":"+1234567890","source":"telegram"}
{"timestamp":"2026-01-16T15:45:22.000Z","action":"stop","sessionKey":"agent:main:main","senderId":"user@example.com","source":"whatsapp"}
```

**æŸ¥çœ‹æ—¥å¿—**ï¼š

```bash
# View recent commands
tail -n 20 ~/.krabkrab/logs/commands.log

# Pretty-print with jq
cat ~/.krabkrab/logs/commands.log | jq .

# Filter by action
grep '"action":"new"' ~/.krabkrab/logs/commands.log | jq .
```

**å¯ç”¨**ï¼š

```bash
krabkrab hooks enable command-logger
```

### boot-md

å½“ Gateway ç½‘å…³å¯åŠ¨æ—¶è¿è¡Œ `BOOT.md`ï¼ˆåœ¨æ¸ é“å¯åŠ¨ä¹‹åŽï¼‰ã€‚
å¿…é¡»å¯ç”¨å†…éƒ¨ hooks æ‰èƒ½è¿è¡Œã€‚

**äº‹ä»¶**ï¼š`gateway:startup`

**è¦æ±‚**ï¼šå¿…é¡»é…ç½® `workspace.dir`

**åŠŸèƒ½**ï¼š

1. ä»Žä½ çš„å·¥ä½œåŒºè¯»å– `BOOT.md`
2. é€šè¿‡æ™ºèƒ½ä½“è¿è¡Œå™¨è¿è¡ŒæŒ‡ä»¤
3. é€šè¿‡ message å·¥å…·å‘é€ä»»ä½•è¯·æ±‚çš„å‡ºç«™æ¶ˆæ¯

**å¯ç”¨**ï¼š

```bash
krabkrab hooks enable boot-md
```

## æœ€ä½³å®žè·µ

### ä¿æŒå¤„ç†ç¨‹åºå¿«é€Ÿ

Hooks åœ¨å‘½ä»¤å¤„ç†æœŸé—´è¿è¡Œã€‚ä¿æŒå®ƒä»¬è½»é‡ï¼š

```typescript
// âœ“ Good - async work, returns immediately
const handler: HookHandler = async (event) => {
  void processInBackground(event); // Fire and forget
};

// âœ— Bad - blocks command processing
const handler: HookHandler = async (event) => {
  await slowDatabaseQuery(event);
  await evenSlowerAPICall(event);
};
```

### ä¼˜é›…å¤„ç†é”™è¯¯

å§‹ç»ˆåŒ…è£…æœ‰é£Žé™©çš„æ“ä½œï¼š

```typescript
const handler: HookHandler = async (event) => {
  try {
    await riskyOperation(event);
  } catch (err) {
    console.error("[my-handler] Failed:", err instanceof Error ? err.message : String(err));
    // Don't throw - let other handlers run
  }
};
```

### å°½æ—©è¿‡æ»¤äº‹ä»¶

å¦‚æžœäº‹ä»¶ä¸ç›¸å…³åˆ™å°½æ—©è¿”å›žï¼š

```typescript
const handler: HookHandler = async (event) => {
  // Only handle 'new' commands
  if (event.type !== "command" || event.action !== "new") {
    return;
  }

  // Your logic here
};
```

### ä½¿ç”¨ç‰¹å®šäº‹ä»¶é”®

å°½å¯èƒ½åœ¨å…ƒæ•°æ®ä¸­æŒ‡å®šç¡®åˆ‡äº‹ä»¶ï¼š

```yaml
metadata: { "krabkrab": { "events": ["command:new"] } } # Specific
```

è€Œä¸æ˜¯ï¼š

```yaml
metadata: { "krabkrab": { "events": ["command"] } } # General - more overhead
```

## è°ƒè¯•

### å¯ç”¨ Hook æ—¥å¿—

Gateway ç½‘å…³åœ¨å¯åŠ¨æ—¶è®°å½• hook åŠ è½½ï¼š

```
Registered hook: session-memory -> command:new
Registered hook: command-logger -> command
Registered hook: boot-md -> gateway:startup
```

### æ£€æŸ¥å‘çŽ°

åˆ—å‡ºæ‰€æœ‰å‘çŽ°çš„ hooksï¼š

```bash
krabkrab hooks list --verbose
```

### æ£€æŸ¥æ³¨å†Œ

åœ¨ä½ çš„å¤„ç†ç¨‹åºä¸­ï¼Œè®°å½•å®ƒè¢«è°ƒç”¨çš„æ—¶é—´ï¼š

```typescript
const handler: HookHandler = async (event) => {
  console.log("[my-handler] Triggered:", event.type, event.action);
  // Your logic
};
```

### éªŒè¯èµ„æ ¼

æ£€æŸ¥ä¸ºä»€ä¹ˆ hook ä¸ç¬¦åˆæ¡ä»¶ï¼š

```bash
krabkrab hooks info my-hook
```

åœ¨è¾“å‡ºä¸­æŸ¥æ‰¾ç¼ºå¤±çš„è¦æ±‚ã€‚

## æµ‹è¯•

### Gateway ç½‘å…³æ—¥å¿—

ç›‘æŽ§ Gateway ç½‘å…³æ—¥å¿—ä»¥æŸ¥çœ‹ hook æ‰§è¡Œï¼š

```bash
# macOS
./scripts/clawlog.sh -f

# Other platforms
tail -f ~/.krabkrab/gateway.log
```

### ç›´æŽ¥æµ‹è¯• Hooks

éš”ç¦»æµ‹è¯•ä½ çš„å¤„ç†ç¨‹åºï¼š

```typescript
import { test } from "vitest";
import { createHookEvent } from "./src/hooks/hooks.js";
import myHandler from "./hooks/my-hook/handler.js";

test("my handler works", async () => {
  const event = createHookEvent("command", "new", "test-session", {
    foo: "bar",
  });

  await myHandler(event);

  // Assert side effects
});
```

## æž¶æž„

### æ ¸å¿ƒç»„ä»¶

- **`src/hooks/types.ts`**ï¼šç±»åž‹å®šä¹‰
- **`src/hooks/workspace.ts`**ï¼šç›®å½•æ‰«æå’ŒåŠ è½½
- **`src/hooks/frontmatter.ts`**ï¼šHOOK.md å…ƒæ•°æ®è§£æž
- **`src/hooks/config.ts`**ï¼šèµ„æ ¼æ£€æŸ¥
- **`src/hooks/hooks-status.ts`**ï¼šçŠ¶æ€æŠ¥å‘Š
- **`src/hooks/loader.ts`**ï¼šåŠ¨æ€æ¨¡å—åŠ è½½å™¨
- **`src/cli/hooks-cli.ts`**ï¼šCLI å‘½ä»¤
- **`src/gateway/server-startup.ts`**ï¼šåœ¨ Gateway ç½‘å…³å¯åŠ¨æ—¶åŠ è½½ hooks
- **`src/auto-reply/reply/commands-core.ts`**ï¼šè§¦å‘å‘½ä»¤äº‹ä»¶

### å‘çŽ°æµç¨‹

```
Gateway ç½‘å…³å¯åŠ¨
    â†“
æ‰«æç›®å½•ï¼ˆå·¥ä½œåŒº â†’ æ‰˜ç®¡ â†’ æ†ç»‘ï¼‰
    â†“
è§£æž HOOK.md æ–‡ä»¶
    â†“
æ£€æŸ¥èµ„æ ¼ï¼ˆbinsã€envã€configã€osï¼‰
    â†“
ä»Žç¬¦åˆæ¡ä»¶çš„ hooks åŠ è½½å¤„ç†ç¨‹åº
    â†“
ä¸ºäº‹ä»¶æ³¨å†Œå¤„ç†ç¨‹åº
```

### äº‹ä»¶æµç¨‹

```
ç”¨æˆ·å‘é€ /new
    â†“
å‘½ä»¤éªŒè¯
    â†“
åˆ›å»º hook äº‹ä»¶
    â†“
è§¦å‘ hookï¼ˆæ‰€æœ‰æ³¨å†Œçš„å¤„ç†ç¨‹åºï¼‰
    â†“
å‘½ä»¤å¤„ç†ç»§ç»­
    â†“
ä¼šè¯é‡ç½®
```

## æ•…éšœæŽ’é™¤

### Hook æœªè¢«å‘çŽ°

1. æ£€æŸ¥ç›®å½•ç»“æž„ï¼š

   ```bash
   ls -la ~/.krabkrab/hooks/my-hook/
   # Should show: HOOK.md, handler.ts
   ```

2. éªŒè¯ HOOK.md æ ¼å¼ï¼š

   ```bash
   cat ~/.krabkrab/hooks/my-hook/HOOK.md
   # Should have YAML frontmatter with name and metadata
   ```

3. åˆ—å‡ºæ‰€æœ‰å‘çŽ°çš„ hooksï¼š
   ```bash
   krabkrab hooks list
   ```

### Hook ä¸ç¬¦åˆæ¡ä»¶

æ£€æŸ¥è¦æ±‚ï¼š

```bash
krabkrab hooks info my-hook
```

æŸ¥æ‰¾ç¼ºå¤±çš„ï¼š

- äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆæ£€æŸ¥ PATHï¼‰
- çŽ¯å¢ƒå˜é‡
- é…ç½®å€¼
- æ“ä½œç³»ç»Ÿå…¼å®¹æ€§

### Hook æœªæ‰§è¡Œ

1. éªŒè¯ hook å·²å¯ç”¨ï¼š

   ```bash
   krabkrab hooks list
   # Should show âœ“ next to enabled hooks
   ```

2. é‡å¯ä½ çš„ Gateway ç½‘å…³è¿›ç¨‹ä»¥é‡æ–°åŠ è½½ hooksã€‚

3. æ£€æŸ¥ Gateway ç½‘å…³æ—¥å¿—ä¸­çš„é”™è¯¯ï¼š
   ```bash
   ./scripts/clawlog.sh | grep hook
   ```

### å¤„ç†ç¨‹åºé”™è¯¯

æ£€æŸ¥ TypeScript/import é”™è¯¯ï¼š

```bash
# Test import directly
node -e "import('./path/to/handler.ts').then(console.log)"
```

## è¿ç§»æŒ‡å—

### ä»Žé—ç•™é…ç½®åˆ°å‘çŽ°

**ä¹‹å‰**ï¼š

```json
{
  "hooks": {
    "internal": {
      "enabled": true,
      "handlers": [
        {
          "event": "command:new",
          "module": "./hooks/handlers/my-handler.ts"
        }
      ]
    }
  }
}
```

**ä¹‹åŽ**ï¼š

1. åˆ›å»º hook ç›®å½•ï¼š

   ```bash
   mkdir -p ~/.krabkrab/hooks/my-hook
   mv ./hooks/handlers/my-handler.ts ~/.krabkrab/hooks/my-hook/handler.ts
   ```

2. åˆ›å»º HOOK.mdï¼š

   ```markdown
   ---
   name: my-hook
   description: "My custom hook"
   metadata: { "krabkrab": { "emoji": "ðŸŽ¯", "events": ["command:new"] } }
   ---

   # My Hook

   Does something useful.
   ```

3. æ›´æ–°é…ç½®ï¼š

   ```json
   {
     "hooks": {
       "internal": {
         "enabled": true,
         "entries": {
           "my-hook": { "enabled": true }
         }
       }
     }
   }
   ```

4. éªŒè¯å¹¶é‡å¯ä½ çš„ Gateway ç½‘å…³è¿›ç¨‹ï¼š
   ```bash
   krabkrab hooks list
   # Should show: ðŸŽ¯ my-hook âœ“
   ```

**è¿ç§»çš„å¥½å¤„**ï¼š

- è‡ªåŠ¨å‘çŽ°
- CLI ç®¡ç†
- èµ„æ ¼æ£€æŸ¥
- æ›´å¥½çš„æ–‡æ¡£
- ä¸€è‡´çš„ç»“æž„

## å¦è¯·å‚é˜…

- [CLI å‚è€ƒï¼šhooks](/cli/hooks)
- [æ†ç»‘ Hooks README](https://github.com/krabkrab/krabkrab/tree/main/src/hooks/bundled)
- [Webhook Hooks](/automation/webhook)
- [é…ç½®](/gateway/configuration#hooks)

