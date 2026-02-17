---
read_when: You want per-agent sandboxing or per-agent tool allow/deny policies in a multi-agent gateway.
status: active
summary: æŒ‰æ™ºèƒ½ä½“çš„æ²™ç®± + å·¥å…·é™åˆ¶ã€ä¼˜å…ˆçº§å’Œç¤ºä¾‹
title: å¤šæ™ºèƒ½ä½“æ²™ç®±ä¸Žå·¥å…·
x-i18n:
  generated_at: "2026-02-03T07:50:39Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: f602cb6192b84b404cd7b6336562888a239d0fe79514edd51bd73c5b090131ef
  source_path: tools/multi-agent-sandbox-tools.md
  workflow: 15
---

# å¤šæ™ºèƒ½ä½“æ²™ç®±ä¸Žå·¥å…·é…ç½®

## æ¦‚è¿°

å¤šæ™ºèƒ½ä½“è®¾ç½®ä¸­çš„æ¯ä¸ªæ™ºèƒ½ä½“çŽ°åœ¨å¯ä»¥æ‹¥æœ‰è‡ªå·±çš„ï¼š

- **æ²™ç®±é…ç½®**ï¼ˆ`agents.list[].sandbox` è¦†ç›– `agents.defaults.sandbox`ï¼‰
- **å·¥å…·é™åˆ¶**ï¼ˆ`tools.allow` / `tools.deny`ï¼Œä»¥åŠ `agents.list[].tools`ï¼‰

è¿™å…è®¸ä½ è¿è¡Œå…·æœ‰ä¸åŒå®‰å…¨é…ç½®æ–‡ä»¶çš„å¤šä¸ªæ™ºèƒ½ä½“ï¼š

- å…·æœ‰å®Œå…¨è®¿é—®æƒé™çš„ä¸ªäººåŠ©æ‰‹
- å…·æœ‰å—é™å·¥å…·çš„å®¶åº­/å·¥ä½œæ™ºèƒ½ä½“
- åœ¨æ²™ç®±ä¸­è¿è¡Œçš„é¢å‘å…¬ä¼—çš„æ™ºèƒ½ä½“

`setupCommand` å±žäºŽ `sandbox.docker` ä¸‹ï¼ˆå…¨å±€æˆ–æŒ‰æ™ºèƒ½ä½“ï¼‰ï¼Œåœ¨å®¹å™¨åˆ›å»ºæ—¶è¿è¡Œä¸€æ¬¡ã€‚

è®¤è¯æ˜¯æŒ‰æ™ºèƒ½ä½“çš„ï¼šæ¯ä¸ªæ™ºèƒ½ä½“ä»Žå…¶è‡ªå·±çš„ `agentDir` è®¤è¯å­˜å‚¨è¯»å–ï¼š

```
~/.krabkrab/agents/<agentId>/agent/auth-profiles.json
```

å‡­è¯**ä¸ä¼š**åœ¨æ™ºèƒ½ä½“ä¹‹é—´å…±äº«ã€‚åˆ‡å‹¿åœ¨æ™ºèƒ½ä½“ä¹‹é—´é‡ç”¨ `agentDir`ã€‚
å¦‚æžœä½ æƒ³å…±äº«å‡­è¯ï¼Œè¯·å°† `auth-profiles.json` å¤åˆ¶åˆ°å…¶ä»–æ™ºèƒ½ä½“çš„ `agentDir` ä¸­ã€‚

æœ‰å…³æ²™ç®±éš”ç¦»åœ¨è¿è¡Œæ—¶çš„è¡Œä¸ºï¼Œè¯·å‚è§[æ²™ç®±éš”ç¦»](/gateway/sandboxing)ã€‚
æœ‰å…³è°ƒè¯•"ä¸ºä»€ä¹ˆè¿™è¢«é˜»æ­¢äº†ï¼Ÿ"ï¼Œè¯·å‚è§[æ²™ç®± vs å·¥å…·ç­–ç•¥ vs ææƒ](/gateway/sandbox-vs-tool-policy-vs-elevated) å’Œ `krabkrab sandbox explain`ã€‚

---

## é…ç½®ç¤ºä¾‹

### ç¤ºä¾‹ 1ï¼šä¸ªäºº + å—é™å®¶åº­æ™ºèƒ½ä½“

```json
{
  "agents": {
    "list": [
      {
        "id": "main",
        "default": true,
        "name": "Personal Assistant",
        "workspace": "~/.krabkrab/workspace",
        "sandbox": { "mode": "off" }
      },
      {
        "id": "family",
        "name": "Family Bot",
        "workspace": "~/.krabkrab/workspace-family",
        "sandbox": {
          "mode": "all",
          "scope": "agent"
        },
        "tools": {
          "allow": ["read"],
          "deny": ["exec", "write", "edit", "apply_patch", "process", "browser"]
        }
      }
    ]
  },
  "bindings": [
    {
      "agentId": "family",
      "match": {
        "provider": "whatsapp",
        "accountId": "*",
        "peer": {
          "kind": "group",
          "id": "120363424282127706@g.us"
        }
      }
    }
  ]
}
```

**ç»“æžœï¼š**

- `main` æ™ºèƒ½ä½“ï¼šåœ¨ä¸»æœºä¸Šè¿è¡Œï¼Œå®Œå…¨å·¥å…·è®¿é—®
- `family` æ™ºèƒ½ä½“ï¼šåœ¨ Docker ä¸­è¿è¡Œï¼ˆæ¯ä¸ªæ™ºèƒ½ä½“ä¸€ä¸ªå®¹å™¨ï¼‰ï¼Œä»…æœ‰ `read` å·¥å…·

---

### ç¤ºä¾‹ 2ï¼šå…·æœ‰å…±äº«æ²™ç®±çš„å·¥ä½œæ™ºèƒ½ä½“

```json
{
  "agents": {
    "list": [
      {
        "id": "personal",
        "workspace": "~/.krabkrab/workspace-personal",
        "sandbox": { "mode": "off" }
      },
      {
        "id": "work",
        "workspace": "~/.krabkrab/workspace-work",
        "sandbox": {
          "mode": "all",
          "scope": "shared",
          "workspaceRoot": "/tmp/work-sandboxes"
        },
        "tools": {
          "allow": ["read", "write", "apply_patch", "exec"],
          "deny": ["browser", "gateway", "discord"]
        }
      }
    ]
  }
}
```

---

### ç¤ºä¾‹ 2bï¼šå…¨å±€ç¼–ç é…ç½®æ–‡ä»¶ + ä»…æ¶ˆæ¯æ™ºèƒ½ä½“

```json
{
  "tools": { "profile": "coding" },
  "agents": {
    "list": [
      {
        "id": "support",
        "tools": { "profile": "messaging", "allow": ["slack"] }
      }
    ]
  }
}
```

**ç»“æžœï¼š**

- é»˜è®¤æ™ºèƒ½ä½“èŽ·å¾—ç¼–ç å·¥å…·
- `support` æ™ºèƒ½ä½“ä»…ç”¨äºŽæ¶ˆæ¯ï¼ˆ+ Slack å·¥å…·ï¼‰

---

### ç¤ºä¾‹ 3ï¼šæ¯ä¸ªæ™ºèƒ½ä½“ä¸åŒçš„æ²™ç®±æ¨¡å¼

```json
{
  "agents": {
    "defaults": {
      "sandbox": {
        "mode": "non-main", // å…¨å±€é»˜è®¤
        "scope": "session"
      }
    },
    "list": [
      {
        "id": "main",
        "workspace": "~/.krabkrab/workspace",
        "sandbox": {
          "mode": "off" // è¦†ç›–ï¼šmain æ°¸ä¸æ²™ç®±éš”ç¦»
        }
      },
      {
        "id": "public",
        "workspace": "~/.krabkrab/workspace-public",
        "sandbox": {
          "mode": "all", // è¦†ç›–ï¼špublic å§‹ç»ˆæ²™ç®±éš”ç¦»
          "scope": "agent"
        },
        "tools": {
          "allow": ["read"],
          "deny": ["exec", "write", "edit", "apply_patch"]
        }
      }
    ]
  }
}
```

---

## é…ç½®ä¼˜å…ˆçº§

å½“å…¨å±€ï¼ˆ`agents.defaults.*`ï¼‰å’Œæ™ºèƒ½ä½“ç‰¹å®šï¼ˆ`agents.list[].*`ï¼‰é…ç½®åŒæ—¶å­˜åœ¨æ—¶ï¼š

### æ²™ç®±é…ç½®

æ™ºèƒ½ä½“ç‰¹å®šè®¾ç½®è¦†ç›–å…¨å±€ï¼š

```
agents.list[].sandbox.mode > agents.defaults.sandbox.mode
agents.list[].sandbox.scope > agents.defaults.sandbox.scope
agents.list[].sandbox.workspaceRoot > agents.defaults.sandbox.workspaceRoot
agents.list[].sandbox.workspaceAccess > agents.defaults.sandbox.workspaceAccess
agents.list[].sandbox.docker.* > agents.defaults.sandbox.docker.*
agents.list[].sandbox.browser.* > agents.defaults.sandbox.browser.*
agents.list[].sandbox.prune.* > agents.defaults.sandbox.prune.*
```

**æ³¨æ„äº‹é¡¹ï¼š**

- `agents.list[].sandbox.{docker,browser,prune}.*` ä¸ºè¯¥æ™ºèƒ½ä½“è¦†ç›– `agents.defaults.sandbox.{docker,browser,prune}.*`ï¼ˆå½“æ²™ç®± scope è§£æžä¸º `"shared"` æ—¶å¿½ç•¥ï¼‰ã€‚

### å·¥å…·é™åˆ¶

è¿‡æ»¤é¡ºåºæ˜¯ï¼š

1. **å·¥å…·é…ç½®æ–‡ä»¶**ï¼ˆ`tools.profile` æˆ– `agents.list[].tools.profile`ï¼‰
2. **æä¾›å•†å·¥å…·é…ç½®æ–‡ä»¶**ï¼ˆ`tools.byProvider[provider].profile` æˆ– `agents.list[].tools.byProvider[provider].profile`ï¼‰
3. **å…¨å±€å·¥å…·ç­–ç•¥**ï¼ˆ`tools.allow` / `tools.deny`ï¼‰
4. **æä¾›å•†å·¥å…·ç­–ç•¥**ï¼ˆ`tools.byProvider[provider].allow/deny`ï¼‰
5. **æ™ºèƒ½ä½“ç‰¹å®šå·¥å…·ç­–ç•¥**ï¼ˆ`agents.list[].tools.allow/deny`ï¼‰
6. **æ™ºèƒ½ä½“æä¾›å•†ç­–ç•¥**ï¼ˆ`agents.list[].tools.byProvider[provider].allow/deny`ï¼‰
7. **æ²™ç®±å·¥å…·ç­–ç•¥**ï¼ˆ`tools.sandbox.tools` æˆ– `agents.list[].tools.sandbox.tools`ï¼‰
8. **å­æ™ºèƒ½ä½“å·¥å…·ç­–ç•¥**ï¼ˆ`tools.subagents.tools`ï¼Œå¦‚é€‚ç”¨ï¼‰

æ¯ä¸ªçº§åˆ«å¯ä»¥è¿›ä¸€æ­¥é™åˆ¶å·¥å…·ï¼Œä½†ä¸èƒ½æ¢å¤ä¹‹å‰çº§åˆ«æ‹’ç»çš„å·¥å…·ã€‚
å¦‚æžœè®¾ç½®äº† `agents.list[].tools.sandbox.tools`ï¼Œå®ƒå°†æ›¿æ¢è¯¥æ™ºèƒ½ä½“çš„ `tools.sandbox.tools`ã€‚
å¦‚æžœè®¾ç½®äº† `agents.list[].tools.profile`ï¼Œå®ƒå°†è¦†ç›–è¯¥æ™ºèƒ½ä½“çš„ `tools.profile`ã€‚
æä¾›å•†å·¥å…·é”®æŽ¥å— `provider`ï¼ˆä¾‹å¦‚ `google-antigravity`ï¼‰æˆ– `provider/model`ï¼ˆä¾‹å¦‚ `openai/gpt-5.2`ï¼‰ã€‚

### å·¥å…·ç»„ï¼ˆç®€å†™ï¼‰

å·¥å…·ç­–ç•¥ï¼ˆå…¨å±€ã€æ™ºèƒ½ä½“ã€æ²™ç®±ï¼‰æ”¯æŒ `group:*` æ¡ç›®ï¼Œå¯æ‰©å±•ä¸ºå¤šä¸ªå…·ä½“å·¥å…·ï¼š

- `group:runtime`ï¼š`exec`ã€`bash`ã€`process`
- `group:fs`ï¼š`read`ã€`write`ã€`edit`ã€`apply_patch`
- `group:sessions`ï¼š`sessions_list`ã€`sessions_history`ã€`sessions_send`ã€`sessions_spawn`ã€`session_status`
- `group:memory`ï¼š`memory_search`ã€`memory_get`
- `group:ui`ï¼š`browser`ã€`canvas`
- `group:automation`ï¼š`cron`ã€`gateway`
- `group:messaging`ï¼š`message`
- `group:nodes`ï¼š`nodes`
- `group:krabkrab`ï¼šæ‰€æœ‰å†…ç½® KrabKrab å·¥å…·ï¼ˆä¸åŒ…æ‹¬æä¾›å•†æ’ä»¶ï¼‰

### ææƒæ¨¡å¼

`tools.elevated` æ˜¯å…¨å±€åŸºçº¿ï¼ˆåŸºäºŽå‘é€è€…çš„å…è®¸åˆ—è¡¨ï¼‰ã€‚`agents.list[].tools.elevated` å¯ä»¥ä¸ºç‰¹å®šæ™ºèƒ½ä½“è¿›ä¸€æ­¥é™åˆ¶ææƒï¼ˆä¸¤è€…éƒ½å¿…é¡»å…è®¸ï¼‰ã€‚

ç¼“è§£æ¨¡å¼ï¼š

- ä¸ºä¸å—ä¿¡ä»»çš„æ™ºèƒ½ä½“æ‹’ç» `exec`ï¼ˆ`agents.list[].tools.deny: ["exec"]`ï¼‰
- é¿å…å°†å‘é€è€…åŠ å…¥å…è®¸åˆ—è¡¨åŽè·¯ç”±åˆ°å—é™æ™ºèƒ½ä½“
- å¦‚æžœä½ åªæƒ³è¦æ²™ç®±éš”ç¦»æ‰§è¡Œï¼Œå…¨å±€ç¦ç”¨ææƒï¼ˆ`tools.elevated.enabled: false`ï¼‰
- ä¸ºæ•æ„Ÿé…ç½®æ–‡ä»¶æŒ‰æ™ºèƒ½ä½“ç¦ç”¨ææƒï¼ˆ`agents.list[].tools.elevated.enabled: false`ï¼‰

---

## ä»Žå•æ™ºèƒ½ä½“è¿ç§»

**ä¹‹å‰ï¼ˆå•æ™ºèƒ½ä½“ï¼‰ï¼š**

```json
{
  "agents": {
    "defaults": {
      "workspace": "~/.krabkrab/workspace",
      "sandbox": {
        "mode": "non-main"
      }
    }
  },
  "tools": {
    "sandbox": {
      "tools": {
        "allow": ["read", "write", "apply_patch", "exec"],
        "deny": []
      }
    }
  }
}
```

**ä¹‹åŽï¼ˆå…·æœ‰ä¸åŒé…ç½®æ–‡ä»¶çš„å¤šæ™ºèƒ½ä½“ï¼‰ï¼š**

```json
{
  "agents": {
    "list": [
      {
        "id": "main",
        "default": true,
        "workspace": "~/.krabkrab/workspace",
        "sandbox": { "mode": "off" }
      }
    ]
  }
}
```

æ—§ç‰ˆ `agent.*` é…ç½®ç”± `krabkrab doctor` è¿ç§»ï¼›ä»ŠåŽè¯·ä¼˜å…ˆä½¿ç”¨ `agents.defaults` + `agents.list`ã€‚

---

## å·¥å…·é™åˆ¶ç¤ºä¾‹

### åªè¯»æ™ºèƒ½ä½“

```json
{
  "tools": {
    "allow": ["read"],
    "deny": ["exec", "write", "edit", "apply_patch", "process"]
  }
}
```

### å®‰å…¨æ‰§è¡Œæ™ºèƒ½ä½“ï¼ˆæ— æ–‡ä»¶ä¿®æ”¹ï¼‰

```json
{
  "tools": {
    "allow": ["read", "exec", "process"],
    "deny": ["write", "edit", "apply_patch", "browser", "gateway"]
  }
}
```

### ä»…é€šä¿¡æ™ºèƒ½ä½“

```json
{
  "tools": {
    "allow": ["sessions_list", "sessions_send", "sessions_history", "session_status"],
    "deny": ["exec", "write", "edit", "apply_patch", "read", "browser"]
  }
}
```

---

## å¸¸è§é™·é˜±ï¼š"non-main"

`agents.defaults.sandbox.mode: "non-main"` åŸºäºŽ `session.mainKey`ï¼ˆé»˜è®¤ `"main"`ï¼‰ï¼Œ
è€Œä¸æ˜¯æ™ºèƒ½ä½“ idã€‚ç¾¤ç»„/æ¸ é“ä¼šè¯å§‹ç»ˆèŽ·å¾—è‡ªå·±çš„é”®ï¼Œå› æ­¤å®ƒä»¬
è¢«è§†ä¸ºéž main å¹¶å°†è¢«æ²™ç®±éš”ç¦»ã€‚å¦‚æžœä½ å¸Œæœ›æ™ºèƒ½ä½“æ°¸ä¸
æ²™ç®±éš”ç¦»ï¼Œè¯·è®¾ç½® `agents.list[].sandbox.mode: "off"`ã€‚

---

## æµ‹è¯•

é…ç½®å¤šæ™ºèƒ½ä½“æ²™ç®±å’Œå·¥å…·åŽï¼š

1. **æ£€æŸ¥æ™ºèƒ½ä½“è§£æžï¼š**

   ```exec
   krabkrab agents list --bindings
   ```

2. **éªŒè¯æ²™ç®±å®¹å™¨ï¼š**

   ```exec
   docker ps --filter "name=krabkrab-sbx-"
   ```

3. **æµ‹è¯•å·¥å…·é™åˆ¶ï¼š**
   - å‘é€éœ€è¦å—é™å·¥å…·çš„æ¶ˆæ¯
   - éªŒè¯æ™ºèƒ½ä½“æ— æ³•ä½¿ç”¨è¢«æ‹’ç»çš„å·¥å…·

4. **ç›‘æŽ§æ—¥å¿—ï¼š**
   ```exec
   tail -f "${krabkrab_STATE_DIR:-$HOME/.krabkrab}/logs/gateway.log" | grep -E "routing|sandbox|tools"
   ```

---

## æ•…éšœæŽ’é™¤

### å°½ç®¡è®¾ç½®äº† `mode: "all"` ä½†æ™ºèƒ½ä½“æœªè¢«æ²™ç®±éš”ç¦»

- æ£€æŸ¥æ˜¯å¦æœ‰å…¨å±€ `agents.defaults.sandbox.mode` è¦†ç›–å®ƒ
- æ™ºèƒ½ä½“ç‰¹å®šé…ç½®ä¼˜å…ˆï¼Œå› æ­¤è®¾ç½® `agents.list[].sandbox.mode: "all"`

### å°½ç®¡æœ‰æ‹’ç»åˆ—è¡¨ä½†å·¥å…·ä»ç„¶å¯ç”¨

- æ£€æŸ¥å·¥å…·è¿‡æ»¤é¡ºåºï¼šå…¨å±€ â†’ æ™ºèƒ½ä½“ â†’ æ²™ç®± â†’ å­æ™ºèƒ½ä½“
- æ¯ä¸ªçº§åˆ«åªèƒ½è¿›ä¸€æ­¥é™åˆ¶ï¼Œä¸èƒ½æ¢å¤
- é€šè¿‡æ—¥å¿—éªŒè¯ï¼š`[tools] filtering tools for agent:${agentId}`

### å®¹å™¨æœªæŒ‰æ™ºèƒ½ä½“éš”ç¦»

- åœ¨æ™ºèƒ½ä½“ç‰¹å®šæ²™ç®±é…ç½®ä¸­è®¾ç½® `scope: "agent"`
- é»˜è®¤æ˜¯ `"session"`ï¼Œæ¯ä¸ªä¼šè¯åˆ›å»ºä¸€ä¸ªå®¹å™¨

---

## å¦è¯·å‚é˜…

- [å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)
- [æ²™ç®±é…ç½®](/gateway/configuration#agentsdefaults-sandbox)
- [ä¼šè¯ç®¡ç†](/concepts/session)

