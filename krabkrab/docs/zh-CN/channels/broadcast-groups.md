---
read_when:
  - é…ç½®å¹¿æ’­ç¾¤ç»„
  - è°ƒè¯• WhatsApp ä¸­çš„å¤šæ™ºèƒ½ä½“å›žå¤
status: experimental
summary: å‘å¤šä¸ªæ™ºèƒ½ä½“å¹¿æ’­ WhatsApp æ¶ˆæ¯
title: å¹¿æ’­ç¾¤ç»„
x-i18n:
  generated_at: "2026-02-03T07:43:43Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: eaeb4035912c49413e012177cf0bd28b348130d30d3317674418dca728229b70
  source_path: channels/broadcast-groups.md
  workflow: 15
---

# å¹¿æ’­ç¾¤ç»„

**çŠ¶æ€ï¼š** å®žéªŒæ€§åŠŸèƒ½  
**ç‰ˆæœ¬ï¼š** äºŽ 2026.1.9 ç‰ˆæœ¬æ–°å¢ž

## æ¦‚è¿°

å¹¿æ’­ç¾¤ç»„å…è®¸å¤šä¸ªæ™ºèƒ½ä½“åŒæ—¶å¤„ç†å¹¶å“åº”åŒä¸€æ¡æ¶ˆæ¯ã€‚è¿™ä½¿ä½ èƒ½å¤Ÿåœ¨å•ä¸ª WhatsApp ç¾¤ç»„æˆ–ç§ä¿¡ä¸­åˆ›å»ºååŒå·¥ä½œçš„ä¸“ä¸šæ™ºèƒ½ä½“å›¢é˜Ÿâ€”â€”å…¨éƒ¨ä½¿ç”¨åŒä¸€ä¸ªæ‰‹æœºå·ç ã€‚

å½“å‰èŒƒå›´ï¼š**ä»…é™ WhatsApp**ï¼ˆweb æ¸ é“ï¼‰ã€‚

å¹¿æ’­ç¾¤ç»„åœ¨æ¸ é“ç™½åå•å’Œç¾¤ç»„æ¿€æ´»è§„åˆ™ä¹‹åŽè¿›è¡Œè¯„ä¼°ã€‚åœ¨ WhatsApp ç¾¤ç»„ä¸­ï¼Œè¿™æ„å‘³ç€å¹¿æ’­ä¼šåœ¨ KrabKrab æ­£å¸¸å›žå¤æ—¶å‘ç”Ÿï¼ˆä¾‹å¦‚ï¼šè¢«æåŠæ—¶ï¼Œå…·ä½“å–å†³äºŽä½ çš„ç¾¤ç»„è®¾ç½®ï¼‰ã€‚

## ä½¿ç”¨åœºæ™¯

### 1. ä¸“ä¸šæ™ºèƒ½ä½“å›¢é˜Ÿ

éƒ¨ç½²å¤šä¸ªå…·æœ‰åŽŸå­åŒ–ã€ä¸“æ³¨èŒè´£çš„æ™ºèƒ½ä½“ï¼š

```
Group: "Development Team"
Agents:
  - CodeReviewer (reviews code snippets)
  - DocumentationBot (generates docs)
  - SecurityAuditor (checks for vulnerabilities)
  - TestGenerator (suggests test cases)
```

æ¯ä¸ªæ™ºèƒ½ä½“å¤„ç†ç›¸åŒçš„æ¶ˆæ¯å¹¶æä¾›å…¶ä¸“ä¸šè§†è§’ã€‚

### 2. å¤šè¯­è¨€æ”¯æŒ

```
Group: "International Support"
Agents:
  - Agent_EN (responds in English)
  - Agent_DE (responds in German)
  - Agent_ES (responds in Spanish)
```

### 3. è´¨é‡ä¿è¯å·¥ä½œæµ

```
Group: "Customer Support"
Agents:
  - SupportAgent (provides answer)
  - QAAgent (reviews quality, only responds if issues found)
```

### 4. ä»»åŠ¡è‡ªåŠ¨åŒ–

```
Group: "Project Management"
Agents:
  - TaskTracker (updates task database)
  - TimeLogger (logs time spent)
  - ReportGenerator (creates summaries)
```

## é…ç½®

### åŸºæœ¬è®¾ç½®

æ·»åŠ ä¸€ä¸ªé¡¶å±‚ `broadcast` éƒ¨åˆ†ï¼ˆä¸Ž `bindings` åŒçº§ï¼‰ã€‚é”®ä¸º WhatsApp peer idï¼š

- ç¾¤èŠï¼šç¾¤ç»„ JIDï¼ˆä¾‹å¦‚ `120363403215116621@g.us`ï¼‰
- ç§ä¿¡ï¼šE.164 æ ¼å¼çš„ç”µè¯å·ç ï¼ˆä¾‹å¦‚ `+15551234567`ï¼‰

```json
{
  "broadcast": {
    "120363403215116621@g.us": ["alfred", "baerbel", "assistant3"]
  }
}
```

**ç»“æžœï¼š** å½“ KrabKrab åœ¨æ­¤èŠå¤©ä¸­å›žå¤æ—¶ï¼Œå°†è¿è¡Œæ‰€æœ‰ä¸‰ä¸ªæ™ºèƒ½ä½“ã€‚

### å¤„ç†ç­–ç•¥

æŽ§åˆ¶æ™ºèƒ½ä½“å¦‚ä½•å¤„ç†æ¶ˆæ¯ï¼š

#### å¹¶è¡Œï¼ˆé»˜è®¤ï¼‰

æ‰€æœ‰æ™ºèƒ½ä½“åŒæ—¶å¤„ç†ï¼š

```json
{
  "broadcast": {
    "strategy": "parallel",
    "120363403215116621@g.us": ["alfred", "baerbel"]
  }
}
```

#### é¡ºåº

æ™ºèƒ½ä½“æŒ‰é¡ºåºå¤„ç†ï¼ˆåŽä¸€ä¸ªç­‰å¾…å‰ä¸€ä¸ªå®Œæˆï¼‰ï¼š

```json
{
  "broadcast": {
    "strategy": "sequential",
    "120363403215116621@g.us": ["alfred", "baerbel"]
  }
}
```

### å®Œæ•´ç¤ºä¾‹

```json
{
  "agents": {
    "list": [
      {
        "id": "code-reviewer",
        "name": "Code Reviewer",
        "workspace": "/path/to/code-reviewer",
        "sandbox": { "mode": "all" }
      },
      {
        "id": "security-auditor",
        "name": "Security Auditor",
        "workspace": "/path/to/security-auditor",
        "sandbox": { "mode": "all" }
      },
      {
        "id": "docs-generator",
        "name": "Documentation Generator",
        "workspace": "/path/to/docs-generator",
        "sandbox": { "mode": "all" }
      }
    ]
  },
  "broadcast": {
    "strategy": "parallel",
    "120363403215116621@g.us": ["code-reviewer", "security-auditor", "docs-generator"],
    "120363424282127706@g.us": ["support-en", "support-de"],
    "+15555550123": ["assistant", "logger"]
  }
}
```

## å·¥ä½œåŽŸç†

### æ¶ˆæ¯æµç¨‹

1. **æŽ¥æ”¶æ¶ˆæ¯** åˆ°è¾¾ WhatsApp ç¾¤ç»„
2. **å¹¿æ’­æ£€æŸ¥**ï¼šç³»ç»Ÿæ£€æŸ¥ peer ID æ˜¯å¦åœ¨ `broadcast` ä¸­
3. **å¦‚æžœåœ¨å¹¿æ’­åˆ—è¡¨ä¸­**ï¼š
   - æ‰€æœ‰åˆ—å‡ºçš„æ™ºèƒ½ä½“å¤„ç†è¯¥æ¶ˆæ¯
   - æ¯ä¸ªæ™ºèƒ½ä½“æœ‰è‡ªå·±çš„ä¼šè¯é”®å’Œéš”ç¦»çš„ä¸Šä¸‹æ–‡
   - æ™ºèƒ½ä½“å¹¶è¡Œå¤„ç†ï¼ˆé»˜è®¤ï¼‰æˆ–é¡ºåºå¤„ç†
4. **å¦‚æžœä¸åœ¨å¹¿æ’­åˆ—è¡¨ä¸­**ï¼š
   - åº”ç”¨æ­£å¸¸è·¯ç”±ï¼ˆç¬¬ä¸€ä¸ªåŒ¹é…çš„ç»‘å®šï¼‰

æ³¨æ„ï¼šå¹¿æ’­ç¾¤ç»„ä¸ä¼šç»•è¿‡æ¸ é“ç™½åå•æˆ–ç¾¤ç»„æ¿€æ´»è§„åˆ™ï¼ˆæåŠ/å‘½ä»¤ç­‰ï¼‰ã€‚å®ƒä»¬åªæ”¹å˜æ¶ˆæ¯ç¬¦åˆå¤„ç†æ¡ä»¶æ—¶*è¿è¡Œå“ªäº›æ™ºèƒ½ä½“*ã€‚

### ä¼šè¯éš”ç¦»

å¹¿æ’­ç¾¤ç»„ä¸­çš„æ¯ä¸ªæ™ºèƒ½ä½“å®Œå…¨ç‹¬ç«‹ç»´æŠ¤ï¼š

- **ä¼šè¯é”®**ï¼ˆ`agent:alfred:whatsapp:group:120363...` vs `agent:baerbel:whatsapp:group:120363...`ï¼‰
- **å¯¹è¯åŽ†å²**ï¼ˆæ™ºèƒ½ä½“çœ‹ä¸åˆ°å…¶ä»–æ™ºèƒ½ä½“çš„æ¶ˆæ¯ï¼‰
- **å·¥ä½œç©ºé—´**ï¼ˆå¦‚æžœé…ç½®äº†åˆ™ä½¿ç”¨ç‹¬ç«‹çš„æ²™ç®±ï¼‰
- **å·¥å…·è®¿é—®æƒé™**ï¼ˆä¸åŒçš„å…è®¸/æ‹’ç»åˆ—è¡¨ï¼‰
- **è®°å¿†/ä¸Šä¸‹æ–‡**ï¼ˆç‹¬ç«‹çš„ IDENTITY.mdã€SOUL.md ç­‰ï¼‰
- **ç¾¤ç»„ä¸Šä¸‹æ–‡ç¼“å†²åŒº**ï¼ˆç”¨äºŽä¸Šä¸‹æ–‡çš„æœ€è¿‘ç¾¤ç»„æ¶ˆæ¯ï¼‰æŒ‰ peer å…±äº«ï¼Œå› æ­¤æ‰€æœ‰å¹¿æ’­æ™ºèƒ½ä½“åœ¨è¢«è§¦å‘æ—¶çœ‹åˆ°ç›¸åŒçš„ä¸Šä¸‹æ–‡

è¿™å…è®¸æ¯ä¸ªæ™ºèƒ½ä½“æ‹¥æœ‰ï¼š

- ä¸åŒçš„ä¸ªæ€§
- ä¸åŒçš„å·¥å…·è®¿é—®æƒé™ï¼ˆä¾‹å¦‚åªè¯» vs è¯»å†™ï¼‰
- ä¸åŒçš„æ¨¡åž‹ï¼ˆä¾‹å¦‚ opus vs sonnetï¼‰
- ä¸åŒçš„å·²å®‰è£… Skills

### ç¤ºä¾‹ï¼šéš”ç¦»çš„ä¼šè¯

åœ¨ç¾¤ç»„ `120363403215116621@g.us` ä¸­ï¼Œæ™ºèƒ½ä½“ä¸º `["alfred", "baerbel"]`ï¼š

**Alfred çš„ä¸Šä¸‹æ–‡ï¼š**

```
Session: agent:alfred:whatsapp:group:120363403215116621@g.us
History: [user message, alfred's previous responses]
Workspace: /Users/pascal/krabkrab-alfred/
Tools: read, write, exec
```

**BÃ¤rbel çš„ä¸Šä¸‹æ–‡ï¼š**

```
Session: agent:baerbel:whatsapp:group:120363403215116621@g.us
History: [user message, baerbel's previous responses]
Workspace: /Users/pascal/krabkrab-baerbel/
Tools: read only
```

## æœ€ä½³å®žè·µ

### 1. ä¿æŒæ™ºèƒ½ä½“ä¸“æ³¨

å°†æ¯ä¸ªæ™ºèƒ½ä½“è®¾è®¡ä¸ºå…·æœ‰å•ä¸€ã€æ˜Žç¡®çš„èŒè´£ï¼š

```json
{
  "broadcast": {
    "DEV_GROUP": ["formatter", "linter", "tester"]
  }
}
```

âœ… **å¥½çš„åšæ³•ï¼š** æ¯ä¸ªæ™ºèƒ½ä½“åªæœ‰ä¸€ä¸ªä»»åŠ¡  
âŒ **ä¸å¥½çš„åšæ³•ï¼š** ä¸€ä¸ªé€šç”¨çš„"dev-helper"æ™ºèƒ½ä½“

### 2. ä½¿ç”¨æè¿°æ€§åç§°

æ˜Žç¡®æ¯ä¸ªæ™ºèƒ½ä½“çš„åŠŸèƒ½ï¼š

```json
{
  "agents": {
    "security-scanner": { "name": "Security Scanner" },
    "code-formatter": { "name": "Code Formatter" },
    "test-generator": { "name": "Test Generator" }
  }
}
```

### 3. é…ç½®ä¸åŒçš„å·¥å…·è®¿é—®æƒé™

åªç»™æ™ºèƒ½ä½“æä¾›å®ƒä»¬éœ€è¦çš„å·¥å…·ï¼š

```json
{
  "agents": {
    "reviewer": {
      "tools": { "allow": ["read", "exec"] } // Read-only
    },
    "fixer": {
      "tools": { "allow": ["read", "write", "edit", "exec"] } // Read-write
    }
  }
}
```

### 4. ç›‘æŽ§æ€§èƒ½

å½“æœ‰å¤šä¸ªæ™ºèƒ½ä½“æ—¶ï¼Œè¯·è€ƒè™‘ï¼š

- ä½¿ç”¨ `"strategy": "parallel"`ï¼ˆé»˜è®¤ï¼‰ä»¥æé«˜é€Ÿåº¦
- å°†å¹¿æ’­ç¾¤ç»„é™åˆ¶åœ¨ 5-10 ä¸ªæ™ºèƒ½ä½“
- ä¸ºè¾ƒç®€å•çš„æ™ºèƒ½ä½“ä½¿ç”¨è¾ƒå¿«çš„æ¨¡åž‹

### 5. ä¼˜é›…åœ°å¤„ç†å¤±è´¥

æ™ºèƒ½ä½“ç‹¬ç«‹å¤±è´¥ã€‚ä¸€ä¸ªæ™ºèƒ½ä½“çš„é”™è¯¯ä¸ä¼šé˜»å¡žå…¶ä»–æ™ºèƒ½ä½“ï¼š

```
Message â†’ [Agent A âœ“, Agent B âœ— error, Agent C âœ“]
Result: Agent A and C respond, Agent B logs error
```

## å…¼å®¹æ€§

### æä¾›å•†

å¹¿æ’­ç¾¤ç»„ç›®å‰æ”¯æŒï¼š

- âœ… WhatsAppï¼ˆå·²å®žçŽ°ï¼‰
- ðŸš§ Telegramï¼ˆè®¡åˆ’ä¸­ï¼‰
- ðŸš§ Discordï¼ˆè®¡åˆ’ä¸­ï¼‰
- ðŸš§ Slackï¼ˆè®¡åˆ’ä¸­ï¼‰

### è·¯ç”±

å¹¿æ’­ç¾¤ç»„ä¸ŽçŽ°æœ‰è·¯ç”±ä¸€èµ·å·¥ä½œï¼š

```json
{
  "bindings": [
    {
      "match": { "channel": "whatsapp", "peer": { "kind": "group", "id": "GROUP_A" } },
      "agentId": "alfred"
    }
  ],
  "broadcast": {
    "GROUP_B": ["agent1", "agent2"]
  }
}
```

- `GROUP_A`ï¼šåªæœ‰ alfred å“åº”ï¼ˆæ­£å¸¸è·¯ç”±ï¼‰
- `GROUP_B`ï¼šagent1 å’Œ agent2 éƒ½å“åº”ï¼ˆå¹¿æ’­ï¼‰

**ä¼˜å…ˆçº§ï¼š** `broadcast` ä¼˜å…ˆäºŽ `bindings`ã€‚

## æ•…éšœæŽ’é™¤

### æ™ºèƒ½ä½“ä¸å“åº”

**æ£€æŸ¥ï¼š**

1. æ™ºèƒ½ä½“ ID å­˜åœ¨äºŽ `agents.list` ä¸­
2. Peer ID æ ¼å¼æ­£ç¡®ï¼ˆä¾‹å¦‚ `120363403215116621@g.us`ï¼‰
3. æ™ºèƒ½ä½“ä¸åœ¨æ‹’ç»åˆ—è¡¨ä¸­

**è°ƒè¯•ï¼š**

```bash
tail -f ~/.krabkrab/logs/gateway.log | grep broadcast
```

### åªæœ‰ä¸€ä¸ªæ™ºèƒ½ä½“å“åº”

**åŽŸå› ï¼š** Peer ID å¯èƒ½åœ¨ `bindings` ä¸­ä½†ä¸åœ¨ `broadcast` ä¸­ã€‚

**ä¿®å¤ï¼š** æ·»åŠ åˆ°å¹¿æ’­é…ç½®æˆ–ä»Žç»‘å®šä¸­ç§»é™¤ã€‚

### æ€§èƒ½é—®é¢˜

**å¦‚æžœæ™ºèƒ½ä½“è¾ƒå¤šæ—¶é€Ÿåº¦è¾ƒæ…¢ï¼š**

- å‡å°‘æ¯ä¸ªç¾¤ç»„çš„æ™ºèƒ½ä½“æ•°é‡
- ä½¿ç”¨è¾ƒè½»çš„æ¨¡åž‹ï¼ˆsonnet è€Œéž opusï¼‰
- æ£€æŸ¥æ²™ç®±å¯åŠ¨æ—¶é—´

## ç¤ºä¾‹

### ç¤ºä¾‹ 1ï¼šä»£ç å®¡æŸ¥å›¢é˜Ÿ

```json
{
  "broadcast": {
    "strategy": "parallel",
    "120363403215116621@g.us": [
      "code-formatter",
      "security-scanner",
      "test-coverage",
      "docs-checker"
    ]
  },
  "agents": {
    "list": [
      {
        "id": "code-formatter",
        "workspace": "~/agents/formatter",
        "tools": { "allow": ["read", "write"] }
      },
      {
        "id": "security-scanner",
        "workspace": "~/agents/security",
        "tools": { "allow": ["read", "exec"] }
      },
      {
        "id": "test-coverage",
        "workspace": "~/agents/testing",
        "tools": { "allow": ["read", "exec"] }
      },
      { "id": "docs-checker", "workspace": "~/agents/docs", "tools": { "allow": ["read"] } }
    ]
  }
}
```

**ç”¨æˆ·å‘é€ï¼š** ä»£ç ç‰‡æ®µ  
**å“åº”ï¼š**

- code-formatterï¼š"ä¿®å¤äº†ç¼©è¿›å¹¶æ·»åŠ äº†ç±»åž‹æç¤º"
- security-scannerï¼š"âš ï¸ ç¬¬ 12 è¡Œå­˜åœ¨ SQL æ³¨å…¥æ¼æ´ž"
- test-coverageï¼š"è¦†ç›–çŽ‡ä¸º 45%ï¼Œç¼ºå°‘é”™è¯¯æƒ…å†µçš„æµ‹è¯•"
- docs-checkerï¼š"å‡½æ•° `process_data` ç¼ºå°‘æ–‡æ¡£å­—ç¬¦ä¸²"

### ç¤ºä¾‹ 2ï¼šå¤šè¯­è¨€æ”¯æŒ

```json
{
  "broadcast": {
    "strategy": "sequential",
    "+15555550123": ["detect-language", "translator-en", "translator-de"]
  },
  "agents": {
    "list": [
      { "id": "detect-language", "workspace": "~/agents/lang-detect" },
      { "id": "translator-en", "workspace": "~/agents/translate-en" },
      { "id": "translator-de", "workspace": "~/agents/translate-de" }
    ]
  }
}
```

## API å‚è€ƒ

### é…ç½®æ¨¡å¼

```typescript
interface krabkrabConfig {
  broadcast?: {
    strategy?: "parallel" | "sequential";
    [peerId: string]: string[];
  };
}
```

### å­—æ®µ

- `strategy`ï¼ˆå¯é€‰ï¼‰ï¼šå¦‚ä½•å¤„ç†æ™ºèƒ½ä½“
  - `"parallel"`ï¼ˆé»˜è®¤ï¼‰ï¼šæ‰€æœ‰æ™ºèƒ½ä½“åŒæ—¶å¤„ç†
  - `"sequential"`ï¼šæ™ºèƒ½ä½“æŒ‰æ•°ç»„é¡ºåºå¤„ç†
- `[peerId]`ï¼šWhatsApp ç¾¤ç»„ JIDã€E.164 å·ç æˆ–å…¶ä»– peer ID
  - å€¼ï¼šåº”å¤„ç†æ¶ˆæ¯çš„æ™ºèƒ½ä½“ ID æ•°ç»„

## é™åˆ¶

1. **æœ€å¤§æ™ºèƒ½ä½“æ•°ï¼š** æ— ç¡¬æ€§é™åˆ¶ï¼Œä½† 10 ä¸ªä»¥ä¸Šæ™ºèƒ½ä½“å¯èƒ½ä¼šè¾ƒæ…¢
2. **å…±äº«ä¸Šä¸‹æ–‡ï¼š** æ™ºèƒ½ä½“çœ‹ä¸åˆ°å½¼æ­¤çš„å“åº”ï¼ˆè®¾è®¡å¦‚æ­¤ï¼‰
3. **æ¶ˆæ¯é¡ºåºï¼š** å¹¶è¡Œå“åº”å¯èƒ½ä»¥ä»»æ„é¡ºåºåˆ°è¾¾
4. **é€ŸçŽ‡é™åˆ¶ï¼š** æ‰€æœ‰æ™ºèƒ½ä½“éƒ½è®¡å…¥ WhatsApp é€ŸçŽ‡é™åˆ¶

## æœªæ¥å¢žå¼º

è®¡åˆ’ä¸­çš„åŠŸèƒ½ï¼š

- [ ] å…±äº«ä¸Šä¸‹æ–‡æ¨¡å¼ï¼ˆæ™ºèƒ½ä½“å¯ä»¥çœ‹åˆ°å½¼æ­¤çš„å“åº”ï¼‰
- [ ] æ™ºèƒ½ä½“åè°ƒï¼ˆæ™ºèƒ½ä½“å¯ä»¥ç›¸äº’å‘ä¿¡å·ï¼‰
- [ ] åŠ¨æ€æ™ºèƒ½ä½“é€‰æ‹©ï¼ˆæ ¹æ®æ¶ˆæ¯å†…å®¹é€‰æ‹©æ™ºèƒ½ä½“ï¼‰
- [ ] æ™ºèƒ½ä½“ä¼˜å…ˆçº§ï¼ˆæŸäº›æ™ºèƒ½ä½“å…ˆäºŽå…¶ä»–æ™ºèƒ½ä½“å“åº”ï¼‰

## å¦è¯·å‚é˜…

- [å¤šæ™ºèƒ½ä½“é…ç½®](/tools/multi-agent-sandbox-tools)
- [è·¯ç”±é…ç½®](/channels/channel-routing)
- [ä¼šè¯ç®¡ç†](/concepts/sessions)

