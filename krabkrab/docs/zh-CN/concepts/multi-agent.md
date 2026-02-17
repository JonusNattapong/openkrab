---
read_when: You want multiple isolated agents (workspaces + auth) in one gateway process.
status: active
summary: å¤šæ™ºèƒ½ä½“è·¯ç”±ï¼šéš”ç¦»çš„æ™ºèƒ½ä½“ã€æ¸ é“è´¦æˆ·å’Œç»‘å®š
title: å¤šæ™ºèƒ½ä½“è·¯ç”±
x-i18n:
  generated_at: "2026-02-03T07:47:38Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 1848266c632cd6c96ff99ea9eb9c17bbfe6d35fa1f90450853083e7c548d5324
  source_path: concepts/multi-agent.md
  workflow: 15
---

# å¤šæ™ºèƒ½ä½“è·¯ç”±

ç›®æ ‡ï¼šå¤šä¸ª*éš”ç¦»çš„*æ™ºèƒ½ä½“ï¼ˆç‹¬ç«‹çš„å·¥ä½œåŒº + `agentDir` + ä¼šè¯ï¼‰ï¼ŒåŠ ä¸Šå¤šä¸ªæ¸ é“è´¦æˆ·ï¼ˆä¾‹å¦‚ä¸¤ä¸ª WhatsAppï¼‰åœ¨ä¸€ä¸ªè¿è¡Œçš„ Gateway ç½‘å…³ä¸­ã€‚å…¥ç«™æ¶ˆæ¯é€šè¿‡ç»‘å®šè·¯ç”±åˆ°æ™ºèƒ½ä½“ã€‚

## ä»€ä¹ˆæ˜¯"ä¸€ä¸ªæ™ºèƒ½ä½“"ï¼Ÿ

ä¸€ä¸ª**æ™ºèƒ½ä½“**æ˜¯ä¸€ä¸ªå®Œå…¨ç‹¬ç«‹ä½œç”¨åŸŸçš„å¤§è„‘ï¼Œæ‹¥æœ‰è‡ªå·±çš„ï¼š

- **å·¥ä½œåŒº**ï¼ˆæ–‡ä»¶ã€AGENTS.md/SOUL.md/USER.mdã€æœ¬åœ°ç¬”è®°ã€äººè®¾è§„åˆ™ï¼‰ã€‚
- **çŠ¶æ€ç›®å½•**ï¼ˆ`agentDir`ï¼‰ç”¨äºŽè®¤è¯é…ç½®æ–‡ä»¶ã€æ¨¡åž‹æ³¨å†Œè¡¨å’Œæ¯æ™ºèƒ½ä½“é…ç½®ã€‚
- **ä¼šè¯å­˜å‚¨**ï¼ˆèŠå¤©åŽ†å² + è·¯ç”±çŠ¶æ€ï¼‰ä½äºŽ `~/.krabkrab/agents/<agentId>/sessions` ä¸‹ã€‚

è®¤è¯é…ç½®æ–‡ä»¶æ˜¯**æ¯æ™ºèƒ½ä½“ç‹¬ç«‹çš„**ã€‚æ¯ä¸ªæ™ºèƒ½ä½“ä»Žè‡ªå·±çš„ä½ç½®è¯»å–ï¼š

```
~/.krabkrab/agents/<agentId>/agent/auth-profiles.json
```

ä¸»æ™ºèƒ½ä½“å‡­è¯**ä¸ä¼š**è‡ªåŠ¨å…±äº«ã€‚åˆ‡å‹¿åœ¨æ™ºèƒ½ä½“ä¹‹é—´é‡ç”¨ `agentDir`ï¼ˆè¿™ä¼šå¯¼è‡´è®¤è¯/ä¼šè¯å†²çªï¼‰ã€‚å¦‚æžœä½ æƒ³å…±äº«å‡­è¯ï¼Œè¯·å°† `auth-profiles.json` å¤åˆ¶åˆ°å¦ä¸€ä¸ªæ™ºèƒ½ä½“çš„ `agentDir`ã€‚

Skills é€šè¿‡æ¯ä¸ªå·¥ä½œåŒºçš„ `skills/` æ–‡ä»¶å¤¹å®žçŽ°æ¯æ™ºèƒ½ä½“ç‹¬ç«‹ï¼Œå…±äº«çš„ Skills å¯ä»Ž `~/.krabkrab/skills` èŽ·å–ã€‚å‚è§ [Skillsï¼šæ¯æ™ºèƒ½ä½“ vs å…±äº«](/tools/skills#per-agent-vs-shared-skills)ã€‚

Gateway ç½‘å…³å¯ä»¥æ‰˜ç®¡**ä¸€ä¸ªæ™ºèƒ½ä½“**ï¼ˆé»˜è®¤ï¼‰æˆ–**å¤šä¸ªæ™ºèƒ½ä½“**å¹¶è¡Œã€‚

**å·¥ä½œåŒºæ³¨æ„äº‹é¡¹ï¼š** æ¯ä¸ªæ™ºèƒ½ä½“çš„å·¥ä½œåŒºæ˜¯**é»˜è®¤ cwd**ï¼Œè€Œä¸æ˜¯ç¡¬æ€§æ²™ç®±ã€‚ç›¸å¯¹è·¯å¾„åœ¨å·¥ä½œåŒºå†…è§£æžï¼Œä½†ç»å¯¹è·¯å¾„å¯ä»¥è®¿é—®ä¸»æœºçš„å…¶ä»–ä½ç½®ï¼Œé™¤éžå¯ç”¨äº†æ²™ç®±éš”ç¦»ã€‚å‚è§ [æ²™ç®±éš”ç¦»](/gateway/sandboxing)ã€‚

## è·¯å¾„ï¼ˆå¿«é€Ÿæ˜ å°„ï¼‰

- é…ç½®ï¼š`~/.krabkrab/krabkrab.json`ï¼ˆæˆ– `krabkrab_CONFIG_PATH`ï¼‰
- çŠ¶æ€ç›®å½•ï¼š`~/.krabkrab`ï¼ˆæˆ– `krabkrab_STATE_DIR`ï¼‰
- å·¥ä½œåŒºï¼š`~/.krabkrab/workspace`ï¼ˆæˆ– `~/.krabkrab/workspace-<agentId>`ï¼‰
- æ™ºèƒ½ä½“ç›®å½•ï¼š`~/.krabkrab/agents/<agentId>/agent`ï¼ˆæˆ– `agents.list[].agentDir`ï¼‰
- ä¼šè¯ï¼š`~/.krabkrab/agents/<agentId>/sessions`

### å•æ™ºèƒ½ä½“æ¨¡å¼ï¼ˆé»˜è®¤ï¼‰

å¦‚æžœä½ ä»€ä¹ˆéƒ½ä¸åšï¼ŒKrabKrab è¿è¡Œå•ä¸ªæ™ºèƒ½ä½“ï¼š

- `agentId` é»˜è®¤ä¸º **`main`**ã€‚
- ä¼šè¯é”®ä¸º `agent:main:<mainKey>`ã€‚
- å·¥ä½œåŒºé»˜è®¤ä¸º `~/.krabkrab/workspace`ï¼ˆæˆ–å½“è®¾ç½®äº† `krabkrab_PROFILE` æ—¶ä¸º `~/.krabkrab/workspace-<profile>`ï¼‰ã€‚
- çŠ¶æ€é»˜è®¤ä¸º `~/.krabkrab/agents/main/agent`ã€‚

## æ™ºèƒ½ä½“åŠ©æ‰‹

ä½¿ç”¨æ™ºèƒ½ä½“å‘å¯¼æ·»åŠ æ–°çš„éš”ç¦»æ™ºèƒ½ä½“ï¼š

```bash
krabkrab agents add work
```

ç„¶åŽæ·»åŠ  `bindings`ï¼ˆæˆ–è®©å‘å¯¼å®Œæˆï¼‰æ¥è·¯ç”±å…¥ç«™æ¶ˆæ¯ã€‚

éªŒè¯ï¼š

```bash
krabkrab agents list --bindings
```

## å¤šä¸ªæ™ºèƒ½ä½“ = å¤šä¸ªäººã€å¤šç§äººæ ¼

ä½¿ç”¨**å¤šä¸ªæ™ºèƒ½ä½“**ï¼Œæ¯ä¸ª `agentId` æˆä¸ºä¸€ä¸ª**å®Œå…¨éš”ç¦»çš„äººæ ¼**ï¼š

- **ä¸åŒçš„ç”µè¯å·ç /è´¦æˆ·**ï¼ˆæ¯æ¸ é“ `accountId`ï¼‰ã€‚
- **ä¸åŒçš„äººæ ¼**ï¼ˆæ¯æ™ºèƒ½ä½“å·¥ä½œåŒºæ–‡ä»¶å¦‚ `AGENTS.md` å’Œ `SOUL.md`ï¼‰ã€‚
- **ç‹¬ç«‹çš„è®¤è¯ + ä¼šè¯**ï¼ˆé™¤éžæ˜Žç¡®å¯ç”¨ï¼Œå¦åˆ™æ— äº¤å‰é€šä¿¡ï¼‰ã€‚

è¿™è®©**å¤šä¸ªäºº**å…±äº«ä¸€ä¸ª Gateway ç½‘å…³æœåŠ¡å™¨ï¼ŒåŒæ—¶ä¿æŒä»–ä»¬çš„ AI"å¤§è„‘"å’Œæ•°æ®éš”ç¦»ã€‚

## ä¸€ä¸ª WhatsApp å·ç ï¼Œå¤šä¸ªäººï¼ˆç§ä¿¡åˆ†å‰²ï¼‰

ä½ å¯ä»¥å°†**ä¸åŒçš„ WhatsApp ç§ä¿¡**è·¯ç”±åˆ°ä¸åŒçš„æ™ºèƒ½ä½“ï¼ŒåŒæ—¶ä¿æŒ**ä¸€ä¸ª WhatsApp è´¦æˆ·**ã€‚ä½¿ç”¨ `peer.kind: "dm"` åŒ¹é…å‘é€è€… E.164ï¼ˆå¦‚ `+15551234567`ï¼‰ã€‚å›žå¤ä»ç„¶æ¥è‡ªåŒä¸€ä¸ª WhatsApp å·ç ï¼ˆæ— æ¯æ™ºèƒ½ä½“å‘é€è€…èº«ä»½ï¼‰ã€‚

é‡è¦ç»†èŠ‚ï¼šç›´æŽ¥èŠå¤©æŠ˜å åˆ°æ™ºèƒ½ä½“çš„**ä¸»ä¼šè¯é”®**ï¼Œå› æ­¤çœŸæ­£çš„éš”ç¦»éœ€è¦**æ¯äººä¸€ä¸ªæ™ºèƒ½ä½“**ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  agents: {
    list: [
      { id: "alex", workspace: "~/.krabkrab/workspace-alex" },
      { id: "mia", workspace: "~/.krabkrab/workspace-mia" },
    ],
  },
  bindings: [
    { agentId: "alex", match: { channel: "whatsapp", peer: { kind: "dm", id: "+15551230001" } } },
    { agentId: "mia", match: { channel: "whatsapp", peer: { kind: "dm", id: "+15551230002" } } },
  ],
  channels: {
    whatsapp: {
      dmPolicy: "allowlist",
      allowFrom: ["+15551230001", "+15551230002"],
    },
  },
}
```

æ³¨æ„äº‹é¡¹ï¼š

- ç§ä¿¡è®¿é—®æŽ§åˆ¶æ˜¯**æ¯ WhatsApp è´¦æˆ·å…¨å±€çš„**ï¼ˆé…å¯¹/å…è®¸åˆ—è¡¨ï¼‰ï¼Œè€Œä¸æ˜¯æ¯æ™ºèƒ½ä½“ã€‚
- å¯¹äºŽå…±äº«ç¾¤ç»„ï¼Œå°†ç¾¤ç»„ç»‘å®šåˆ°ä¸€ä¸ªæ™ºèƒ½ä½“æˆ–ä½¿ç”¨ [å¹¿æ’­ç¾¤ç»„](/channels/broadcast-groups)ã€‚

## è·¯ç”±è§„åˆ™ï¼ˆæ¶ˆæ¯å¦‚ä½•é€‰æ‹©æ™ºèƒ½ä½“ï¼‰

ç»‘å®šæ˜¯**ç¡®å®šæ€§çš„**ï¼Œ**æœ€å…·ä½“çš„ä¼˜å…ˆ**ï¼š

1. `peer` åŒ¹é…ï¼ˆç²¾ç¡®ç§ä¿¡/ç¾¤ç»„/é¢‘é“ idï¼‰
2. `guildId`ï¼ˆDiscordï¼‰
3. `teamId`ï¼ˆSlackï¼‰
4. æ¸ é“çš„ `accountId` åŒ¹é…
5. æ¸ é“çº§åŒ¹é…ï¼ˆ`accountId: "*"`ï¼‰
6. å›žé€€åˆ°é»˜è®¤æ™ºèƒ½ä½“ï¼ˆ`agents.list[].default`ï¼Œå¦åˆ™åˆ—è¡¨ä¸­çš„ç¬¬ä¸€ä¸ªæ¡ç›®ï¼Œé»˜è®¤ï¼š`main`ï¼‰

## å¤šè´¦æˆ·/ç”µè¯å·ç 

æ”¯æŒ**å¤šè´¦æˆ·**çš„æ¸ é“ï¼ˆå¦‚ WhatsAppï¼‰ä½¿ç”¨ `accountId` æ¥è¯†åˆ«æ¯ä¸ªç™»å½•ã€‚æ¯ä¸ª `accountId` å¯ä»¥è·¯ç”±åˆ°ä¸åŒçš„æ™ºèƒ½ä½“ï¼Œå› æ­¤ä¸€ä¸ªæœåŠ¡å™¨å¯ä»¥æ‰˜ç®¡å¤šä¸ªç”µè¯å·ç è€Œä¸æ··åˆä¼šè¯ã€‚

## æ¦‚å¿µ

- `agentId`ï¼šä¸€ä¸ª"å¤§è„‘"ï¼ˆå·¥ä½œåŒºã€æ¯æ™ºèƒ½ä½“è®¤è¯ã€æ¯æ™ºèƒ½ä½“ä¼šè¯å­˜å‚¨ï¼‰ã€‚
- `accountId`ï¼šä¸€ä¸ªæ¸ é“è´¦æˆ·å®žä¾‹ï¼ˆä¾‹å¦‚ WhatsApp è´¦æˆ· `"personal"` vs `"biz"`ï¼‰ã€‚
- `binding`ï¼šé€šè¿‡ `(channel, accountId, peer)` ä»¥åŠå¯é€‰çš„ guild/team id å°†å…¥ç«™æ¶ˆæ¯è·¯ç”±åˆ° `agentId`ã€‚
- ç›´æŽ¥èŠå¤©æŠ˜å åˆ° `agent:<agentId>:<mainKey>`ï¼ˆæ¯æ™ºèƒ½ä½“"ä¸»"ï¼›`session.mainKey`ï¼‰ã€‚

## ç¤ºä¾‹ï¼šä¸¤ä¸ª WhatsApp â†’ ä¸¤ä¸ªæ™ºèƒ½ä½“

`~/.krabkrab/krabkrab.json`ï¼ˆJSON5ï¼‰ï¼š

```js
{
  agents: {
    list: [
      {
        id: "home",
        default: true,
        name: "Home",
        workspace: "~/.krabkrab/workspace-home",
        agentDir: "~/.krabkrab/agents/home/agent",
      },
      {
        id: "work",
        name: "Work",
        workspace: "~/.krabkrab/workspace-work",
        agentDir: "~/.krabkrab/agents/work/agent",
      },
    ],
  },

  // ç¡®å®šæ€§è·¯ç”±ï¼šç¬¬ä¸€ä¸ªåŒ¹é…èŽ·èƒœï¼ˆæœ€å…·ä½“çš„ä¼˜å…ˆï¼‰ã€‚
  bindings: [
    { agentId: "home", match: { channel: "whatsapp", accountId: "personal" } },
    { agentId: "work", match: { channel: "whatsapp", accountId: "biz" } },

    // å¯é€‰çš„æ¯å¯¹ç­‰æ–¹è¦†ç›–ï¼ˆç¤ºä¾‹ï¼šå°†ç‰¹å®šç¾¤ç»„å‘é€åˆ° work æ™ºèƒ½ä½“ï¼‰ã€‚
    {
      agentId: "work",
      match: {
        channel: "whatsapp",
        accountId: "personal",
        peer: { kind: "group", id: "1203630...@g.us" },
      },
    },
  ],

  // é»˜è®¤å…³é—­ï¼šæ™ºèƒ½ä½“åˆ°æ™ºèƒ½ä½“çš„æ¶ˆæ¯å¿…é¡»æ˜Žç¡®å¯ç”¨ + åŠ å…¥å…è®¸åˆ—è¡¨ã€‚
  tools: {
    agentToAgent: {
      enabled: false,
      allow: ["home", "work"],
    },
  },

  channels: {
    whatsapp: {
      accounts: {
        personal: {
          // å¯é€‰è¦†ç›–ã€‚é»˜è®¤ï¼š~/.krabkrab/credentials/whatsapp/personal
          // authDir: "~/.krabkrab/credentials/whatsapp/personal",
        },
        biz: {
          // å¯é€‰è¦†ç›–ã€‚é»˜è®¤ï¼š~/.krabkrab/credentials/whatsapp/biz
          // authDir: "~/.krabkrab/credentials/whatsapp/biz",
        },
      },
    },
  },
}
```

## ç¤ºä¾‹ï¼šWhatsApp æ—¥å¸¸èŠå¤© + Telegram æ·±åº¦å·¥ä½œ

æŒ‰æ¸ é“åˆ†å‰²ï¼šå°† WhatsApp è·¯ç”±åˆ°å¿«é€Ÿæ—¥å¸¸æ™ºèƒ½ä½“ï¼ŒTelegram è·¯ç”±åˆ° Opus æ™ºèƒ½ä½“ã€‚

```json5
{
  agents: {
    list: [
      {
        id: "chat",
        name: "Everyday",
        workspace: "~/.krabkrab/workspace-chat",
        model: "anthropic/claude-sonnet-4-5",
      },
      {
        id: "opus",
        name: "Deep Work",
        workspace: "~/.krabkrab/workspace-opus",
        model: "anthropic/claude-opus-4-5",
      },
    ],
  },
  bindings: [
    { agentId: "chat", match: { channel: "whatsapp" } },
    { agentId: "opus", match: { channel: "telegram" } },
  ],
}
```

æ³¨æ„äº‹é¡¹ï¼š

- å¦‚æžœä½ æœ‰ä¸€ä¸ªæ¸ é“çš„å¤šä¸ªè´¦æˆ·ï¼Œè¯·åœ¨ç»‘å®šä¸­æ·»åŠ  `accountId`ï¼ˆä¾‹å¦‚ `{ channel: "whatsapp", accountId: "personal" }`ï¼‰ã€‚
- è¦å°†å•ä¸ªç§ä¿¡/ç¾¤ç»„è·¯ç”±åˆ° Opus è€Œä¿æŒå…¶ä½™åœ¨ chat ä¸Šï¼Œè¯·ä¸ºè¯¥å¯¹ç­‰æ–¹æ·»åŠ  `match.peer` ç»‘å®šï¼›å¯¹ç­‰æ–¹åŒ¹é…å§‹ç»ˆä¼˜å…ˆäºŽæ¸ é“çº§è§„åˆ™ã€‚

## ç¤ºä¾‹ï¼šåŒä¸€æ¸ é“ï¼Œä¸€ä¸ªå¯¹ç­‰æ–¹åˆ° Opus

ä¿æŒ WhatsApp åœ¨å¿«é€Ÿæ™ºèƒ½ä½“ä¸Šï¼Œä½†å°†ä¸€ä¸ªç§ä¿¡è·¯ç”±åˆ° Opusï¼š

```json5
{
  agents: {
    list: [
      {
        id: "chat",
        name: "Everyday",
        workspace: "~/.krabkrab/workspace-chat",
        model: "anthropic/claude-sonnet-4-5",
      },
      {
        id: "opus",
        name: "Deep Work",
        workspace: "~/.krabkrab/workspace-opus",
        model: "anthropic/claude-opus-4-5",
      },
    ],
  },
  bindings: [
    { agentId: "opus", match: { channel: "whatsapp", peer: { kind: "dm", id: "+15551234567" } } },
    { agentId: "chat", match: { channel: "whatsapp" } },
  ],
}
```

å¯¹ç­‰æ–¹ç»‘å®šå§‹ç»ˆèŽ·èƒœï¼Œå› æ­¤å°†å®ƒä»¬æ”¾åœ¨æ¸ é“çº§è§„åˆ™ä¹‹ä¸Šã€‚

## ç»‘å®šåˆ° WhatsApp ç¾¤ç»„çš„å®¶åº­æ™ºèƒ½ä½“

å°†ä¸“ç”¨å®¶åº­æ™ºèƒ½ä½“ç»‘å®šåˆ°å•ä¸ª WhatsApp ç¾¤ç»„ï¼Œä½¿ç”¨æåŠé™åˆ¶å’Œæ›´ä¸¥æ ¼çš„å·¥å…·ç­–ç•¥ï¼š

```json5
{
  agents: {
    list: [
      {
        id: "family",
        name: "Family",
        workspace: "~/.krabkrab/workspace-family",
        identity: { name: "Family Bot" },
        groupChat: {
          mentionPatterns: ["@family", "@familybot", "@Family Bot"],
        },
        sandbox: {
          mode: "all",
          scope: "agent",
        },
        tools: {
          allow: [
            "exec",
            "read",
            "sessions_list",
            "sessions_history",
            "sessions_send",
            "sessions_spawn",
            "session_status",
          ],
          deny: ["write", "edit", "apply_patch", "browser", "canvas", "nodes", "cron"],
        },
      },
    ],
  },
  bindings: [
    {
      agentId: "family",
      match: {
        channel: "whatsapp",
        peer: { kind: "group", id: "120363999999999999@g.us" },
      },
    },
  ],
}
```

æ³¨æ„äº‹é¡¹ï¼š

- å·¥å…·å…è®¸/æ‹’ç»åˆ—è¡¨æ˜¯**å·¥å…·**ï¼Œä¸æ˜¯ Skillsã€‚å¦‚æžœ skill éœ€è¦è¿è¡ŒäºŒè¿›åˆ¶æ–‡ä»¶ï¼Œè¯·ç¡®ä¿ `exec` è¢«å…è®¸ä¸”äºŒè¿›åˆ¶æ–‡ä»¶å­˜åœ¨äºŽæ²™ç®±ä¸­ã€‚
- å¯¹äºŽæ›´ä¸¥æ ¼çš„é™åˆ¶ï¼Œè®¾ç½® `agents.list[].groupChat.mentionPatterns` å¹¶ä¸ºæ¸ é“ä¿æŒç¾¤ç»„å…è®¸åˆ—è¡¨å¯ç”¨ã€‚

## æ¯æ™ºèƒ½ä½“æ²™ç®±å’Œå·¥å…·é…ç½®

ä»Ž v2026.1.6 å¼€å§‹ï¼Œæ¯ä¸ªæ™ºèƒ½ä½“å¯ä»¥æœ‰è‡ªå·±çš„æ²™ç®±å’Œå·¥å…·é™åˆ¶ï¼š

```js
{
  agents: {
    list: [
      {
        id: "personal",
        workspace: "~/.krabkrab/workspace-personal",
        sandbox: {
          mode: "off",  // ä¸ªäººæ™ºèƒ½ä½“æ— æ²™ç®±
        },
        // æ— å·¥å…·é™åˆ¶ - æ‰€æœ‰å·¥å…·å¯ç”¨
      },
      {
        id: "family",
        workspace: "~/.krabkrab/workspace-family",
        sandbox: {
          mode: "all",     // å§‹ç»ˆæ²™ç®±éš”ç¦»
          scope: "agent",  // æ¯æ™ºèƒ½ä½“ä¸€ä¸ªå®¹å™¨
          docker: {
            // å®¹å™¨åˆ›å»ºåŽçš„å¯é€‰ä¸€æ¬¡æ€§è®¾ç½®
            setupCommand: "apt-get update && apt-get install -y git curl",
          },
        },
        tools: {
          allow: ["read"],                    // ä»… read å·¥å…·
          deny: ["exec", "write", "edit", "apply_patch"],    // æ‹’ç»å…¶ä»–
        },
      },
    ],
  },
}
```

æ³¨æ„ï¼š`setupCommand` ä½äºŽ `sandbox.docker` ä¸‹ï¼Œåœ¨å®¹å™¨åˆ›å»ºæ—¶è¿è¡Œä¸€æ¬¡ã€‚
å½“è§£æžçš„ scope ä¸º `"shared"` æ—¶ï¼Œæ¯æ™ºèƒ½ä½“ `sandbox.docker.*` è¦†ç›–ä¼šè¢«å¿½ç•¥ã€‚

**å¥½å¤„ï¼š**

- **å®‰å…¨éš”ç¦»**ï¼šé™åˆ¶ä¸å—ä¿¡ä»»æ™ºèƒ½ä½“çš„å·¥å…·
- **èµ„æºæŽ§åˆ¶**ï¼šæ²™ç®±éš”ç¦»ç‰¹å®šæ™ºèƒ½ä½“åŒæ—¶ä¿æŒå…¶ä»–æ™ºèƒ½ä½“åœ¨ä¸»æœºä¸Š
- **çµæ´»ç­–ç•¥**ï¼šæ¯æ™ºèƒ½ä½“ä¸åŒçš„æƒé™

æ³¨æ„ï¼š`tools.elevated` æ˜¯**å…¨å±€çš„**ä¸”åŸºäºŽå‘é€è€…ï¼›ä¸èƒ½æŒ‰æ™ºèƒ½ä½“é…ç½®ã€‚
å¦‚æžœä½ éœ€è¦æ¯æ™ºèƒ½ä½“è¾¹ç•Œï¼Œä½¿ç”¨ `agents.list[].tools` æ‹’ç» `exec`ã€‚
å¯¹äºŽç¾¤ç»„å®šå‘ï¼Œä½¿ç”¨ `agents.list[].groupChat.mentionPatterns` ä½¿ @æåŠæ¸…æ™°åœ°æ˜ å°„åˆ°ç›®æ ‡æ™ºèƒ½ä½“ã€‚

å‚è§ [å¤šæ™ºèƒ½ä½“æ²™ç®±å’Œå·¥å…·](/tools/multi-agent-sandbox-tools) äº†è§£è¯¦ç»†ç¤ºä¾‹ã€‚

