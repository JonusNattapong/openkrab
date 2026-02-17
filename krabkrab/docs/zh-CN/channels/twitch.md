---
read_when:
  - ä¸º KrabKrab è®¾ç½® Twitch èŠå¤©é›†æˆ
summary: Twitch èŠå¤©æœºå™¨äººé…ç½®å’Œè®¾ç½®
title: Twitch
x-i18n:
  generated_at: "2026-02-03T07:44:41Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 0dd1c05bef570470d8b82c1f6dee5337e8b76b57269c5cad6aee2e711483f8ba
  source_path: channels/twitch.md
  workflow: 15
---

# Twitchï¼ˆæ’ä»¶ï¼‰

é€šè¿‡ IRC è¿žæŽ¥æ”¯æŒ Twitch èŠå¤©ã€‚KrabKrab ä»¥ Twitch ç”¨æˆ·ï¼ˆæœºå™¨äººè´¦æˆ·ï¼‰èº«ä»½è¿žæŽ¥ï¼Œåœ¨é¢‘é“ä¸­æŽ¥æ”¶å’Œå‘é€æ¶ˆæ¯ã€‚

## éœ€è¦æ’ä»¶

Twitch ä½œä¸ºæ’ä»¶å‘å¸ƒï¼Œæœªä¸Žæ ¸å¿ƒå®‰è£…æ†ç»‘ã€‚

é€šè¿‡ CLI å®‰è£…ï¼ˆnpm æ³¨å†Œè¡¨ï¼‰ï¼š

```bash
krabkrab plugins install @krabkrab/twitch
```

æœ¬åœ°æ£€å‡ºï¼ˆä»Ž git ä»“åº“è¿è¡Œæ—¶ï¼‰ï¼š

```bash
krabkrab plugins install ./extensions/twitch
```

è¯¦æƒ…ï¼š[æ’ä»¶](/tools/plugin)

## å¿«é€Ÿè®¾ç½®ï¼ˆæ–°æ‰‹ï¼‰

1. ä¸ºæœºå™¨äººåˆ›å»ºä¸€ä¸ªä¸“ç”¨çš„ Twitch è´¦æˆ·ï¼ˆæˆ–ä½¿ç”¨çŽ°æœ‰è´¦æˆ·ï¼‰ã€‚
2. ç”Ÿæˆå‡­è¯ï¼š[Twitch Token Generator](https://twitchtokengenerator.com/)
   - é€‰æ‹© **Bot Token**
   - ç¡®è®¤å·²é€‰æ‹© `chat:read` å’Œ `chat:write` æƒé™èŒƒå›´
   - å¤åˆ¶ **Client ID** å’Œ **Access Token**
3. æŸ¥æ‰¾ä½ çš„ Twitch ç”¨æˆ· IDï¼šhttps://www.streamweasels.com/tools/convert-twitch-username-to-user-id/
4. é…ç½®ä»¤ç‰Œï¼š
   - çŽ¯å¢ƒå˜é‡ï¼š`krabkrab_TWITCH_ACCESS_TOKEN=...`ï¼ˆä»…é™é»˜è®¤è´¦æˆ·ï¼‰
   - æˆ–é…ç½®ï¼š`channels.twitch.accessToken`
   - å¦‚æžœä¸¤è€…éƒ½è®¾ç½®ï¼Œé…ç½®ä¼˜å…ˆï¼ˆçŽ¯å¢ƒå˜é‡å›žé€€ä»…é€‚ç”¨äºŽé»˜è®¤è´¦æˆ·ï¼‰ã€‚
5. å¯åŠ¨ Gateway ç½‘å…³ã€‚

**âš ï¸ é‡è¦ï¼š** æ·»åŠ è®¿é—®æŽ§åˆ¶ï¼ˆ`allowFrom` æˆ– `allowedRoles`ï¼‰ä»¥é˜²æ­¢æœªæŽˆæƒç”¨æˆ·è§¦å‘æœºå™¨äººã€‚`requireMention` é»˜è®¤ä¸º `true`ã€‚

æœ€å°é…ç½®ï¼š

```json5
{
  channels: {
    twitch: {
      enabled: true,
      username: "krabkrab", // æœºå™¨äººçš„ Twitch è´¦æˆ·
      accessToken: "oauth:abc123...", // OAuth Access Tokenï¼ˆæˆ–ä½¿ç”¨ krabkrab_TWITCH_ACCESS_TOKEN çŽ¯å¢ƒå˜é‡ï¼‰
      clientId: "xyz789...", // Token Generator ä¸­çš„ Client ID
      channel: "vevisk", // è¦åŠ å…¥çš„ Twitch é¢‘é“èŠå¤©ï¼ˆå¿…å¡«ï¼‰
      allowFrom: ["123456789"], // ï¼ˆæŽ¨èï¼‰ä»…é™ä½ çš„ Twitch ç”¨æˆ· ID - ä»Ž https://www.streamweasels.com/tools/convert-twitch-username-to-user-id/ èŽ·å–
    },
  },
}
```

## å®ƒæ˜¯ä»€ä¹ˆ

- ç”± Gateway ç½‘å…³æ‹¥æœ‰çš„ Twitch æ¸ é“ã€‚
- ç¡®å®šæ€§è·¯ç”±ï¼šå›žå¤æ€»æ˜¯è¿”å›žåˆ° Twitchã€‚
- æ¯ä¸ªè´¦æˆ·æ˜ å°„åˆ°ä¸€ä¸ªéš”ç¦»çš„ä¼šè¯é”® `agent:<agentId>:twitch:<accountName>`ã€‚
- `username` æ˜¯æœºå™¨äººè´¦æˆ·ï¼ˆè¿›è¡Œèº«ä»½éªŒè¯çš„è´¦æˆ·ï¼‰ï¼Œ`channel` æ˜¯è¦åŠ å…¥çš„èŠå¤©å®¤ã€‚

## è®¾ç½®ï¼ˆè¯¦ç»†ï¼‰

### ç”Ÿæˆå‡­è¯

ä½¿ç”¨ [Twitch Token Generator](https://twitchtokengenerator.com/)ï¼š

- é€‰æ‹© **Bot Token**
- ç¡®è®¤å·²é€‰æ‹© `chat:read` å’Œ `chat:write` æƒé™èŒƒå›´
- å¤åˆ¶ **Client ID** å’Œ **Access Token**

æ— éœ€æ‰‹åŠ¨æ³¨å†Œåº”ç”¨ã€‚ä»¤ç‰Œåœ¨å‡ å°æ—¶åŽè¿‡æœŸã€‚

### é…ç½®æœºå™¨äºº

**çŽ¯å¢ƒå˜é‡ï¼ˆä»…é™é»˜è®¤è´¦æˆ·ï¼‰ï¼š**

```bash
krabkrab_TWITCH_ACCESS_TOKEN=oauth:abc123...
```

**æˆ–é…ç½®ï¼š**

```json5
{
  channels: {
    twitch: {
      enabled: true,
      username: "krabkrab",
      accessToken: "oauth:abc123...",
      clientId: "xyz789...",
      channel: "vevisk",
    },
  },
}
```

å¦‚æžœçŽ¯å¢ƒå˜é‡å’Œé…ç½®éƒ½è®¾ç½®äº†ï¼Œé…ç½®ä¼˜å…ˆã€‚

### è®¿é—®æŽ§åˆ¶ï¼ˆæŽ¨èï¼‰

```json5
{
  channels: {
    twitch: {
      allowFrom: ["123456789"], // ï¼ˆæŽ¨èï¼‰ä»…é™ä½ çš„ Twitch ç”¨æˆ· ID
    },
  },
}
```

ä¼˜å…ˆä½¿ç”¨ `allowFrom` ä½œä¸ºç¡¬æ€§å…è®¸åˆ—è¡¨ã€‚å¦‚æžœä½ æƒ³è¦åŸºäºŽè§’è‰²çš„è®¿é—®æŽ§åˆ¶ï¼Œè¯·æ”¹ç”¨ `allowedRoles`ã€‚

**å¯ç”¨è§’è‰²ï¼š** `"moderator"`ã€`"owner"`ã€`"vip"`ã€`"subscriber"`ã€`"all"`ã€‚

**ä¸ºä»€ä¹ˆç”¨ç”¨æˆ· IDï¼Ÿ** ç”¨æˆ·åå¯ä»¥æ›´æ”¹ï¼Œå…è®¸å†’å……ã€‚ç”¨æˆ· ID æ˜¯æ°¸ä¹…çš„ã€‚

æŸ¥æ‰¾ä½ çš„ Twitch ç”¨æˆ· IDï¼šhttps://www.streamweasels.com/tools/convert-twitch-username-%20to-user-id/ï¼ˆå°†ä½ çš„ Twitch ç”¨æˆ·åè½¬æ¢ä¸º IDï¼‰

## ä»¤ç‰Œåˆ·æ–°ï¼ˆå¯é€‰ï¼‰

æ¥è‡ª [Twitch Token Generator](https://twitchtokengenerator.com/) çš„ä»¤ç‰Œæ— æ³•è‡ªåŠ¨åˆ·æ–° - è¿‡æœŸæ—¶éœ€è¦é‡æ–°ç”Ÿæˆã€‚

è¦å®žçŽ°è‡ªåŠ¨ä»¤ç‰Œåˆ·æ–°ï¼Œè¯·åœ¨ [Twitch Developer Console](https://dev.twitch.tv/console) åˆ›å»ºä½ è‡ªå·±çš„ Twitch åº”ç”¨å¹¶æ·»åŠ åˆ°é…ç½®ä¸­ï¼š

```json5
{
  channels: {
    twitch: {
      clientSecret: "your_client_secret",
      refreshToken: "your_refresh_token",
    },
  },
}
```

æœºå™¨äººä¼šåœ¨ä»¤ç‰Œè¿‡æœŸå‰è‡ªåŠ¨åˆ·æ–°ï¼Œå¹¶è®°å½•åˆ·æ–°äº‹ä»¶ã€‚

## å¤šè´¦æˆ·æ”¯æŒ

ä½¿ç”¨ `channels.twitch.accounts` é…ç½®æ¯ä¸ªè´¦æˆ·çš„ä»¤ç‰Œã€‚å‚é˜… [`gateway/configuration`](/gateway/configuration) äº†è§£å…±äº«æ¨¡å¼ã€‚

ç¤ºä¾‹ï¼ˆä¸€ä¸ªæœºå™¨äººè´¦æˆ·åœ¨ä¸¤ä¸ªé¢‘é“ä¸­ï¼‰ï¼š

```json5
{
  channels: {
    twitch: {
      accounts: {
        channel1: {
          username: "krabkrab",
          accessToken: "oauth:abc123...",
          clientId: "xyz789...",
          channel: "vevisk",
        },
        channel2: {
          username: "krabkrab",
          accessToken: "oauth:def456...",
          clientId: "uvw012...",
          channel: "secondchannel",
        },
      },
    },
  },
}
```

**æ³¨æ„ï¼š** æ¯ä¸ªè´¦æˆ·éœ€è¦è‡ªå·±çš„ä»¤ç‰Œï¼ˆæ¯ä¸ªé¢‘é“ä¸€ä¸ªä»¤ç‰Œï¼‰ã€‚

## è®¿é—®æŽ§åˆ¶

### åŸºäºŽè§’è‰²çš„é™åˆ¶

```json5
{
  channels: {
    twitch: {
      accounts: {
        default: {
          allowedRoles: ["moderator", "vip"],
        },
      },
    },
  },
}
```

### æŒ‰ç”¨æˆ· ID å…è®¸åˆ—è¡¨ï¼ˆæœ€å®‰å…¨ï¼‰

```json5
{
  channels: {
    twitch: {
      accounts: {
        default: {
          allowFrom: ["123456789", "987654321"],
        },
      },
    },
  },
}
```

### åŸºäºŽè§’è‰²çš„è®¿é—®ï¼ˆæ›¿ä»£æ–¹æ¡ˆï¼‰

`allowFrom` æ˜¯ç¡¬æ€§å…è®¸åˆ—è¡¨ã€‚è®¾ç½®åŽï¼Œåªå…è®¸è¿™äº›ç”¨æˆ· IDã€‚
å¦‚æžœä½ æƒ³è¦åŸºäºŽè§’è‰²çš„è®¿é—®ï¼Œè¯·ä¸è®¾ç½® `allowFrom`ï¼Œæ”¹ä¸ºé…ç½® `allowedRoles`ï¼š

```json5
{
  channels: {
    twitch: {
      accounts: {
        default: {
          allowedRoles: ["moderator"],
        },
      },
    },
  },
}
```

### ç¦ç”¨ @æåŠè¦æ±‚

é»˜è®¤æƒ…å†µä¸‹ï¼Œ`requireMention` ä¸º `true`ã€‚è¦ç¦ç”¨å¹¶å“åº”æ‰€æœ‰æ¶ˆæ¯ï¼š

```json5
{
  channels: {
    twitch: {
      accounts: {
        default: {
          requireMention: false,
        },
      },
    },
  },
}
```

## æ•…éšœæŽ’é™¤

é¦–å…ˆï¼Œè¿è¡Œè¯Šæ–­å‘½ä»¤ï¼š

```bash
krabkrab doctor
krabkrab channels status --probe
```

### æœºå™¨äººä¸å“åº”æ¶ˆæ¯

**æ£€æŸ¥è®¿é—®æŽ§åˆ¶ï¼š** ç¡®ä¿ä½ çš„ç”¨æˆ· ID åœ¨ `allowFrom` ä¸­ï¼Œæˆ–ä¸´æ—¶ç§»é™¤ `allowFrom` å¹¶è®¾ç½® `allowedRoles: ["all"]` æ¥æµ‹è¯•ã€‚

**æ£€æŸ¥æœºå™¨äººæ˜¯å¦åœ¨é¢‘é“ä¸­ï¼š** æœºå™¨äººå¿…é¡»åŠ å…¥ `channel` ä¸­æŒ‡å®šçš„é¢‘é“ã€‚

### ä»¤ç‰Œé—®é¢˜

**"Failed to connect"æˆ–èº«ä»½éªŒè¯é”™è¯¯ï¼š**

- éªŒè¯ `accessToken` æ˜¯ OAuth è®¿é—®ä»¤ç‰Œå€¼ï¼ˆé€šå¸¸ä»¥ `oauth:` å‰ç¼€å¼€å¤´ï¼‰
- æ£€æŸ¥ä»¤ç‰Œå…·æœ‰ `chat:read` å’Œ `chat:write` æƒé™èŒƒå›´
- å¦‚æžœä½¿ç”¨ä»¤ç‰Œåˆ·æ–°ï¼ŒéªŒè¯ `clientSecret` å’Œ `refreshToken` å·²è®¾ç½®

### ä»¤ç‰Œåˆ·æ–°ä¸å·¥ä½œ

**æ£€æŸ¥æ—¥å¿—ä¸­çš„åˆ·æ–°äº‹ä»¶ï¼š**

```
Using env token source for mybot
Access token refreshed for user 123456 (expires in 14400s)
```

å¦‚æžœä½ çœ‹åˆ°"token refresh disabled (no refresh token)"ï¼š

- ç¡®ä¿æä¾›äº† `clientSecret`
- ç¡®ä¿æä¾›äº† `refreshToken`

## é…ç½®

**è´¦æˆ·é…ç½®ï¼š**

- `username` - æœºå™¨äººç”¨æˆ·å
- `accessToken` - å…·æœ‰ `chat:read` å’Œ `chat:write` æƒé™çš„ OAuth è®¿é—®ä»¤ç‰Œ
- `clientId` - Twitch Client IDï¼ˆæ¥è‡ª Token Generator æˆ–ä½ çš„åº”ç”¨ï¼‰
- `channel` - è¦åŠ å…¥çš„é¢‘é“ï¼ˆå¿…å¡«ï¼‰
- `enabled` - å¯ç”¨æ­¤è´¦æˆ·ï¼ˆé»˜è®¤ï¼š`true`ï¼‰
- `clientSecret` - å¯é€‰ï¼šç”¨äºŽè‡ªåŠ¨ä»¤ç‰Œåˆ·æ–°
- `refreshToken` - å¯é€‰ï¼šç”¨äºŽè‡ªåŠ¨ä»¤ç‰Œåˆ·æ–°
- `expiresIn` - ä»¤ç‰Œè¿‡æœŸæ—¶é—´ï¼ˆç§’ï¼‰
- `obtainmentTimestamp` - ä»¤ç‰ŒèŽ·å–æ—¶é—´æˆ³
- `allowFrom` - ç”¨æˆ· ID å…è®¸åˆ—è¡¨
- `allowedRoles` - åŸºäºŽè§’è‰²çš„è®¿é—®æŽ§åˆ¶ï¼ˆ`"moderator" | "owner" | "vip" | "subscriber" | "all"`ï¼‰
- `requireMention` - éœ€è¦ @æåŠï¼ˆé»˜è®¤ï¼š`true`ï¼‰

**æä¾›å•†é€‰é¡¹ï¼š**

- `channels.twitch.enabled` - å¯ç”¨/ç¦ç”¨æ¸ é“å¯åŠ¨
- `channels.twitch.username` - æœºå™¨äººç”¨æˆ·åï¼ˆç®€åŒ–çš„å•è´¦æˆ·é…ç½®ï¼‰
- `channels.twitch.accessToken` - OAuth è®¿é—®ä»¤ç‰Œï¼ˆç®€åŒ–çš„å•è´¦æˆ·é…ç½®ï¼‰
- `channels.twitch.clientId` - Twitch Client IDï¼ˆç®€åŒ–çš„å•è´¦æˆ·é…ç½®ï¼‰
- `channels.twitch.channel` - è¦åŠ å…¥çš„é¢‘é“ï¼ˆç®€åŒ–çš„å•è´¦æˆ·é…ç½®ï¼‰
- `channels.twitch.accounts.<accountName>` - å¤šè´¦æˆ·é…ç½®ï¼ˆä»¥ä¸Šæ‰€æœ‰è´¦æˆ·å­—æ®µï¼‰

å®Œæ•´ç¤ºä¾‹ï¼š

```json5
{
  channels: {
    twitch: {
      enabled: true,
      username: "krabkrab",
      accessToken: "oauth:abc123...",
      clientId: "xyz789...",
      channel: "vevisk",
      clientSecret: "secret123...",
      refreshToken: "refresh456...",
      allowFrom: ["123456789"],
      allowedRoles: ["moderator", "vip"],
      accounts: {
        default: {
          username: "mybot",
          accessToken: "oauth:abc123...",
          clientId: "xyz789...",
          channel: "your_channel",
          enabled: true,
          clientSecret: "secret123...",
          refreshToken: "refresh456...",
          expiresIn: 14400,
          obtainmentTimestamp: 1706092800000,
          allowFrom: ["123456789", "987654321"],
          allowedRoles: ["moderator"],
        },
      },
    },
  },
}
```

## å·¥å…·æ“ä½œ

æ™ºèƒ½ä½“å¯ä»¥è°ƒç”¨ `twitch` æ‰§è¡Œä»¥ä¸‹æ“ä½œï¼š

- `send` - å‘é¢‘é“å‘é€æ¶ˆæ¯

ç¤ºä¾‹ï¼š

```json5
{
  action: "twitch",
  params: {
    message: "Hello Twitch!",
    to: "#mychannel",
  },
}
```

## å®‰å…¨ä¸Žè¿ç»´

- **å°†ä»¤ç‰Œè§†ä¸ºå¯†ç ** - æ°¸è¿œä¸è¦å°†ä»¤ç‰Œæäº¤åˆ° git
- **ä½¿ç”¨è‡ªåŠ¨ä»¤ç‰Œåˆ·æ–°** ç”¨äºŽé•¿æ—¶é—´è¿è¡Œçš„æœºå™¨äºº
- **ä½¿ç”¨ç”¨æˆ· ID å…è®¸åˆ—è¡¨** è€Œä¸æ˜¯ç”¨æˆ·åè¿›è¡Œè®¿é—®æŽ§åˆ¶
- **ç›‘æŽ§æ—¥å¿—** æŸ¥çœ‹ä»¤ç‰Œåˆ·æ–°äº‹ä»¶å’Œè¿žæŽ¥çŠ¶æ€
- **æœ€å°åŒ–ä»¤ç‰Œæƒé™èŒƒå›´** - åªè¯·æ±‚ `chat:read` å’Œ `chat:write`
- **å¦‚æžœå¡ä½**ï¼šåœ¨ç¡®è®¤æ²¡æœ‰å…¶ä»–è¿›ç¨‹æ‹¥æœ‰ä¼šè¯åŽé‡å¯ Gateway ç½‘å…³

## é™åˆ¶

- æ¯æ¡æ¶ˆæ¯ **500 ä¸ªå­—ç¬¦**ï¼ˆåœ¨å•è¯è¾¹ç•Œè‡ªåŠ¨åˆ†å—ï¼‰
- åˆ†å—å‰ä¼šåŽ»é™¤ Markdown
- æ— é€ŸçŽ‡é™åˆ¶ï¼ˆä½¿ç”¨ Twitch å†…ç½®çš„é€ŸçŽ‡é™åˆ¶ï¼‰

