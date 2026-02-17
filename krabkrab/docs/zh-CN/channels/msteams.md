---
read_when:
  - å¼€å‘ MS Teams æ¸ é“åŠŸèƒ½
summary: Microsoft Teams æœºå™¨äººæ”¯æŒçŠ¶æ€ã€åŠŸèƒ½å’Œé…ç½®
title: Microsoft Teams
x-i18n:
  generated_at: "2026-02-03T07:46:52Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 2046cb8fa3dd349f4b25a40c013a87188af8f75c1886a782698bff2bb9f70971
  source_path: channels/msteams.md
  workflow: 15
---

# Microsoft Teamsï¼ˆæ’ä»¶ï¼‰

> "è¿›å…¥æ­¤åœ°è€…ï¼Œæ”¾å¼ƒä¸€åˆ‡å¸Œæœ›ã€‚"

æ›´æ–°æ—¶é—´ï¼š2026-01-21

çŠ¶æ€ï¼šæ”¯æŒæ–‡æœ¬ + ç§ä¿¡é™„ä»¶ï¼›é¢‘é“/ç¾¤ç»„æ–‡ä»¶å‘é€éœ€è¦ `sharePointSiteId` + Graph æƒé™ï¼ˆå‚è§[åœ¨ç¾¤èŠä¸­å‘é€æ–‡ä»¶](#sending-files-in-group-chats)ï¼‰ã€‚æŠ•ç¥¨é€šè¿‡ Adaptive Cards å‘é€ã€‚

## éœ€è¦æ’ä»¶

Microsoft Teams ä½œä¸ºæ’ä»¶æä¾›ï¼Œä¸åŒ…å«åœ¨æ ¸å¿ƒå®‰è£…ä¸­ã€‚

**ç ´åæ€§å˜æ›´ï¼ˆ2026.1.15ï¼‰ï¼š** MS Teams å·²ä»Žæ ¸å¿ƒç§»å‡ºã€‚å¦‚æžœä½ ä½¿ç”¨å®ƒï¼Œå¿…é¡»å®‰è£…æ’ä»¶ã€‚

åŽŸå› è¯´æ˜Žï¼šä¿æŒæ ¸å¿ƒå®‰è£…æ›´è½»é‡ï¼Œå¹¶è®© MS Teams ä¾èµ–é¡¹å¯ä»¥ç‹¬ç«‹æ›´æ–°ã€‚

é€šè¿‡ CLI å®‰è£…ï¼ˆnpm æ³¨å†Œè¡¨ï¼‰ï¼š

```bash
krabkrab plugins install @krabkrab/msteams
```

æœ¬åœ°æ£€å‡ºï¼ˆä»Ž git ä»“åº“è¿è¡Œæ—¶ï¼‰ï¼š

```bash
krabkrab plugins install ./extensions/msteams
```

å¦‚æžœä½ åœ¨é…ç½®/æ–°æ‰‹å¼•å¯¼è¿‡ç¨‹ä¸­é€‰æ‹© Teams å¹¶æ£€æµ‹åˆ° git æ£€å‡ºï¼Œ
KrabKrab å°†è‡ªåŠ¨æä¾›æœ¬åœ°å®‰è£…è·¯å¾„ã€‚

è¯¦æƒ…ï¼š[æ’ä»¶](/tools/plugin)

## å¿«é€Ÿè®¾ç½®ï¼ˆåˆå­¦è€…ï¼‰

1. å®‰è£… Microsoft Teams æ’ä»¶ã€‚
2. åˆ›å»ºä¸€ä¸ª **Azure Bot**ï¼ˆApp ID + å®¢æˆ·ç«¯å¯†é’¥ + ç§Ÿæˆ· IDï¼‰ã€‚
3. ä½¿ç”¨è¿™äº›å‡­è¯é…ç½® KrabKrabã€‚
4. é€šè¿‡å…¬å…± URL æˆ–éš§é“æš´éœ² `/api/messages`ï¼ˆé»˜è®¤ç«¯å£ 3978ï¼‰ã€‚
5. å®‰è£… Teams åº”ç”¨åŒ…å¹¶å¯åŠ¨ Gateway ç½‘å…³ã€‚

æœ€å°é…ç½®ï¼š

```json5
{
  channels: {
    msteams: {
      enabled: true,
      appId: "<APP_ID>",
      appPassword: "<APP_PASSWORD>",
      tenantId: "<TENANT_ID>",
      webhook: { port: 3978, path: "/api/messages" },
    },
  },
}
```

æ³¨æ„ï¼šç¾¤èŠé»˜è®¤è¢«é˜»æ­¢ï¼ˆ`channels.msteams.groupPolicy: "allowlist"`ï¼‰ã€‚è¦å…è®¸ç¾¤ç»„å›žå¤ï¼Œè¯·è®¾ç½® `channels.msteams.groupAllowFrom`ï¼ˆæˆ–ä½¿ç”¨ `groupPolicy: "open"` å…è®¸ä»»ä½•æˆå‘˜ï¼Œéœ€è¦æåŠæ‰èƒ½è§¦å‘ï¼‰ã€‚

## ç›®æ ‡

- é€šè¿‡ Teams ç§ä¿¡ã€ç¾¤èŠæˆ–é¢‘é“ä¸Ž KrabKrab äº¤æµã€‚
- ä¿æŒè·¯ç”±ç¡®å®šæ€§ï¼šå›žå¤å§‹ç»ˆè¿”å›žåˆ°æ¶ˆæ¯åˆ°è¾¾çš„æ¸ é“ã€‚
- é»˜è®¤ä½¿ç”¨å®‰å…¨çš„æ¸ é“è¡Œä¸ºï¼ˆé™¤éžå¦æœ‰é…ç½®ï¼Œå¦åˆ™éœ€è¦æåŠï¼‰ã€‚

## é…ç½®å†™å…¥

é»˜è®¤æƒ…å†µä¸‹ï¼ŒMicrosoft Teams å…è®¸é€šè¿‡ `/config set|unset` è§¦å‘çš„é…ç½®æ›´æ–°å†™å…¥ï¼ˆéœ€è¦ `commands.config: true`ï¼‰ã€‚

ç¦ç”¨æ–¹å¼ï¼š

```json5
{
  channels: { msteams: { configWrites: false } },
}
```

## è®¿é—®æŽ§åˆ¶ï¼ˆç§ä¿¡ + ç¾¤ç»„ï¼‰

**ç§ä¿¡è®¿é—®**

- é»˜è®¤ï¼š`channels.msteams.dmPolicy = "pairing"`ã€‚æœªçŸ¥å‘é€è€…åœ¨èŽ·å¾—æ‰¹å‡†ä¹‹å‰å°†è¢«å¿½ç•¥ã€‚
- `channels.msteams.allowFrom` æŽ¥å— AAD å¯¹è±¡ IDã€UPN æˆ–æ˜¾ç¤ºåç§°ã€‚å½“å‡­è¯å…è®¸æ—¶ï¼Œå‘å¯¼ä¼šé€šè¿‡ Microsoft Graph å°†åç§°è§£æžä¸º IDã€‚

**ç¾¤ç»„è®¿é—®**

- é»˜è®¤ï¼š`channels.msteams.groupPolicy = "allowlist"`ï¼ˆé™¤éžæ·»åŠ  `groupAllowFrom`ï¼Œå¦åˆ™è¢«é˜»æ­¢ï¼‰ã€‚ä½¿ç”¨ `channels.defaults.groupPolicy` åœ¨æœªè®¾ç½®æ—¶è¦†ç›–é»˜è®¤å€¼ã€‚
- `channels.msteams.groupAllowFrom` æŽ§åˆ¶å“ªäº›å‘é€è€…å¯ä»¥åœ¨ç¾¤èŠ/é¢‘é“ä¸­è§¦å‘ï¼ˆå›žé€€åˆ° `channels.msteams.allowFrom`ï¼‰ã€‚
- è®¾ç½® `groupPolicy: "open"` å…è®¸ä»»ä½•æˆå‘˜ï¼ˆé»˜è®¤ä»éœ€æåŠæ‰èƒ½è§¦å‘ï¼‰ã€‚
- è¦**ä¸å…è®¸ä»»ä½•é¢‘é“**ï¼Œè®¾ç½® `channels.msteams.groupPolicy: "disabled"`ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  channels: {
    msteams: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["user@org.com"],
    },
  },
}
```

**å›¢é˜Ÿ + é¢‘é“å…è®¸åˆ—è¡¨**

- é€šè¿‡åœ¨ `channels.msteams.teams` ä¸‹åˆ—å‡ºå›¢é˜Ÿå’Œé¢‘é“æ¥é™å®šç¾¤ç»„/é¢‘é“å›žå¤çš„èŒƒå›´ã€‚
- é”®å¯ä»¥æ˜¯å›¢é˜Ÿ ID æˆ–åç§°ï¼›é¢‘é“é”®å¯ä»¥æ˜¯ä¼šè¯ ID æˆ–åç§°ã€‚
- å½“ `groupPolicy="allowlist"` ä¸”å­˜åœ¨å›¢é˜Ÿå…è®¸åˆ—è¡¨æ—¶ï¼Œä»…æŽ¥å—åˆ—å‡ºçš„å›¢é˜Ÿ/é¢‘é“ï¼ˆéœ€è¦æåŠæ‰èƒ½è§¦å‘ï¼‰ã€‚
- é…ç½®å‘å¯¼æŽ¥å— `Team/Channel` æ¡ç›®å¹¶ä¸ºä½ å­˜å‚¨ã€‚
- å¯åŠ¨æ—¶ï¼ŒKrabKrab å°†å›¢é˜Ÿ/é¢‘é“å’Œç”¨æˆ·å…è®¸åˆ—è¡¨åç§°è§£æžä¸º IDï¼ˆå½“ Graph æƒé™å…è®¸æ—¶ï¼‰
  å¹¶è®°å½•æ˜ å°„ï¼›æœªè§£æžçš„æ¡ç›®ä¿æŒåŽŸæ ·ã€‚

ç¤ºä¾‹ï¼š

```json5
{
  channels: {
    msteams: {
      groupPolicy: "allowlist",
      teams: {
        "My Team": {
          channels: {
            General: { requireMention: true },
          },
        },
      },
    },
  },
}
```

## å·¥ä½œåŽŸç†

1. å®‰è£… Microsoft Teams æ’ä»¶ã€‚
2. åˆ›å»ºä¸€ä¸ª **Azure Bot**ï¼ˆApp ID + å¯†é’¥ + ç§Ÿæˆ· IDï¼‰ã€‚
3. æž„å»ºä¸€ä¸ªå¼•ç”¨æœºå™¨äººå¹¶åŒ…å«ä»¥ä¸‹ RSC æƒé™çš„ **Teams åº”ç”¨åŒ…**ã€‚
4. å°† Teams åº”ç”¨ä¸Šä¼ /å®‰è£…åˆ°å›¢é˜Ÿä¸­ï¼ˆæˆ–ç”¨äºŽç§ä¿¡çš„ä¸ªäººèŒƒå›´ï¼‰ã€‚
5. åœ¨ `~/.krabkrab/krabkrab.json`ï¼ˆæˆ–çŽ¯å¢ƒå˜é‡ï¼‰ä¸­é…ç½® `msteams` å¹¶å¯åŠ¨ Gateway ç½‘å…³ã€‚
6. Gateway ç½‘å…³é»˜è®¤åœ¨ `/api/messages` ä¸Šç›‘å¬ Bot Framework webhook æµé‡ã€‚

## Azure Bot è®¾ç½®ï¼ˆå‰ææ¡ä»¶ï¼‰

åœ¨é…ç½® KrabKrab ä¹‹å‰ï¼Œä½ éœ€è¦åˆ›å»ºä¸€ä¸ª Azure Bot èµ„æºã€‚

### æ­¥éª¤ 1ï¼šåˆ›å»º Azure Bot

1. å‰å¾€[åˆ›å»º Azure Bot](https://portal.azure.com/#create/Microsoft.AzureBot)
2. å¡«å†™**åŸºæœ¬ä¿¡æ¯**é€‰é¡¹å¡ï¼š

   | å­—æ®µ               | å€¼                                                  |
   | ------------------ | --------------------------------------------------- |
   | **Bot handle**     | ä½ çš„æœºå™¨äººåç§°ï¼Œä¾‹å¦‚ `krabkrab-msteams`ï¼ˆå¿…é¡»å”¯ä¸€ï¼‰ |
   | **Subscription**   | é€‰æ‹©ä½ çš„ Azure è®¢é˜…                                 |
   | **Resource group** | æ–°å»ºæˆ–ä½¿ç”¨çŽ°æœ‰                                      |
   | **Pricing tier**   | **Free** ç”¨äºŽå¼€å‘/æµ‹è¯•                              |
   | **Type of App**    | **Single Tenant**ï¼ˆæŽ¨è - è§ä¸‹æ–¹è¯´æ˜Žï¼‰              |
   | **Creation type**  | **Create new Microsoft App ID**                     |

> **å¼ƒç”¨é€šçŸ¥ï¼š** 2025-07-31 ä¹‹åŽå·²å¼ƒç”¨åˆ›å»ºæ–°çš„å¤šç§Ÿæˆ·æœºå™¨äººã€‚æ–°æœºå™¨äººè¯·ä½¿ç”¨ **Single Tenant**ã€‚

3. ç‚¹å‡» **Review + create** â†’ **Create**ï¼ˆç­‰å¾…çº¦ 1-2 åˆ†é’Ÿï¼‰

### æ­¥éª¤ 2ï¼šèŽ·å–å‡­è¯

1. å‰å¾€ä½ çš„ Azure Bot èµ„æº â†’ **Configuration**
2. å¤åˆ¶ **Microsoft App ID** â†’ è¿™æ˜¯ä½ çš„ `appId`
3. ç‚¹å‡» **Manage Password** â†’ å‰å¾€åº”ç”¨æ³¨å†Œ
4. åœ¨ **Certificates & secrets** â†’ **New client secret** â†’ å¤åˆ¶ **Value** â†’ è¿™æ˜¯ä½ çš„ `appPassword`
5. å‰å¾€ **Overview** â†’ å¤åˆ¶ **Directory (tenant) ID** â†’ è¿™æ˜¯ä½ çš„ `tenantId`

### æ­¥éª¤ 3ï¼šé…ç½®æ¶ˆæ¯ç«¯ç‚¹

1. åœ¨ Azure Bot â†’ **Configuration**
2. å°† **Messaging endpoint** è®¾ç½®ä¸ºä½ çš„ webhook URLï¼š
   - ç”Ÿäº§çŽ¯å¢ƒï¼š`https://your-domain.com/api/messages`
   - æœ¬åœ°å¼€å‘ï¼šä½¿ç”¨éš§é“ï¼ˆè§ä¸‹æ–¹[æœ¬åœ°å¼€å‘](#local-development-tunneling)ï¼‰

### æ­¥éª¤ 4ï¼šå¯ç”¨ Teams æ¸ é“

1. åœ¨ Azure Bot â†’ **Channels**
2. ç‚¹å‡» **Microsoft Teams** â†’ Configure â†’ Save
3. æŽ¥å—æœåŠ¡æ¡æ¬¾

## æœ¬åœ°å¼€å‘ï¼ˆéš§é“ï¼‰

Teams æ— æ³•è®¿é—® `localhost`ã€‚æœ¬åœ°å¼€å‘è¯·ä½¿ç”¨éš§é“ï¼š

**é€‰é¡¹ Aï¼šngrok**

```bash
ngrok http 3978
# å¤åˆ¶ https URLï¼Œä¾‹å¦‚ https://abc123.ngrok.io
# å°†æ¶ˆæ¯ç«¯ç‚¹è®¾ç½®ä¸ºï¼šhttps://abc123.ngrok.io/api/messages
```

**é€‰é¡¹ Bï¼šTailscale Funnel**

```bash
tailscale funnel 3978
# ä½¿ç”¨ä½ çš„ Tailscale funnel URL ä½œä¸ºæ¶ˆæ¯ç«¯ç‚¹
```

## Teams å¼€å‘è€…é—¨æˆ·ï¼ˆæ›¿ä»£æ–¹æ¡ˆï¼‰

é™¤äº†æ‰‹åŠ¨åˆ›å»ºæ¸…å• ZIPï¼Œä½ å¯ä»¥ä½¿ç”¨ [Teams å¼€å‘è€…é—¨æˆ·](https://dev.teams.microsoft.com/apps)ï¼š

1. ç‚¹å‡» **+ New app**
2. å¡«å†™åŸºæœ¬ä¿¡æ¯ï¼ˆåç§°ã€æè¿°ã€å¼€å‘è€…ä¿¡æ¯ï¼‰
3. å‰å¾€ **App features** â†’ **Bot**
4. é€‰æ‹© **Enter a bot ID manually** å¹¶ç²˜è´´ä½ çš„ Azure Bot App ID
5. å‹¾é€‰èŒƒå›´ï¼š**Personal**ã€**Team**ã€**Group Chat**
6. ç‚¹å‡» **Distribute** â†’ **Download app package**
7. åœ¨ Teams ä¸­ï¼š**Apps** â†’ **Manage your apps** â†’ **Upload a custom app** â†’ é€‰æ‹© ZIP

è¿™é€šå¸¸æ¯”æ‰‹åŠ¨ç¼–è¾‘ JSON æ¸…å•æ›´å®¹æ˜“ã€‚

## æµ‹è¯•æœºå™¨äºº

**é€‰é¡¹ Aï¼šAzure Web Chatï¼ˆå…ˆéªŒè¯ webhookï¼‰**

1. åœ¨ Azure é—¨æˆ· â†’ ä½ çš„ Azure Bot èµ„æº â†’ **Test in Web Chat**
2. å‘é€ä¸€æ¡æ¶ˆæ¯ - ä½ åº”è¯¥çœ‹åˆ°å“åº”
3. è¿™ç¡®è®¤ä½ çš„ webhook ç«¯ç‚¹åœ¨ Teams è®¾ç½®ä¹‹å‰æ­£å¸¸å·¥ä½œ

**é€‰é¡¹ Bï¼šTeamsï¼ˆåº”ç”¨å®‰è£…åŽï¼‰**

1. å®‰è£… Teams åº”ç”¨ï¼ˆä¾§è½½æˆ–ç»„ç»‡ç›®å½•ï¼‰
2. åœ¨ Teams ä¸­æ‰¾åˆ°æœºå™¨äººå¹¶å‘é€ç§ä¿¡
3. æ£€æŸ¥ Gateway ç½‘å…³æ—¥å¿—ä¸­çš„ä¼ å…¥æ´»åŠ¨

## è®¾ç½®ï¼ˆæœ€å°çº¯æ–‡æœ¬ï¼‰

1. **å®‰è£… Microsoft Teams æ’ä»¶**
   - ä»Ž npmï¼š`krabkrab plugins install @krabkrab/msteams`
   - ä»Žæœ¬åœ°æ£€å‡ºï¼š`krabkrab plugins install ./extensions/msteams`

2. **æœºå™¨äººæ³¨å†Œ**
   - åˆ›å»ºä¸€ä¸ª Azure Botï¼ˆè§ä¸Šæ–‡ï¼‰å¹¶è®°å½•ï¼š
     - App ID
     - å®¢æˆ·ç«¯å¯†é’¥ï¼ˆApp passwordï¼‰
     - ç§Ÿæˆ· IDï¼ˆå•ç§Ÿæˆ·ï¼‰

3. **Teams åº”ç”¨æ¸…å•**
   - åŒ…å«ä¸€ä¸ª `bot` æ¡ç›®ï¼Œå…¶ä¸­ `botId = <App ID>`ã€‚
   - èŒƒå›´ï¼š`personal`ã€`team`ã€`groupChat`ã€‚
   - `supportsFiles: true`ï¼ˆä¸ªäººèŒƒå›´æ–‡ä»¶å¤„ç†æ‰€éœ€ï¼‰ã€‚
   - æ·»åŠ  RSC æƒé™ï¼ˆè§ä¸‹æ–‡ï¼‰ã€‚
   - åˆ›å»ºå›¾æ ‡ï¼š`outline.png`ï¼ˆ32x32ï¼‰å’Œ `color.png`ï¼ˆ192x192ï¼‰ã€‚
   - å°†ä¸‰ä¸ªæ–‡ä»¶ä¸€èµ·æ‰“åŒ…ï¼š`manifest.json`ã€`outline.png`ã€`color.png`ã€‚

4. **é…ç½® KrabKrab**

   ```json
   {
     "msteams": {
       "enabled": true,
       "appId": "<APP_ID>",
       "appPassword": "<APP_PASSWORD>",
       "tenantId": "<TENANT_ID>",
       "webhook": { "port": 3978, "path": "/api/messages" }
     }
   }
   ```

   ä½ ä¹Ÿå¯ä»¥ä½¿ç”¨çŽ¯å¢ƒå˜é‡ä»£æ›¿é…ç½®é”®ï¼š
   - `MSTEAMS_APP_ID`
   - `MSTEAMS_APP_PASSWORD`
   - `MSTEAMS_TENANT_ID`

5. **æœºå™¨äººç«¯ç‚¹**
   - å°† Azure Bot Messaging Endpoint è®¾ç½®ä¸ºï¼š
     - `https://<host>:3978/api/messages`ï¼ˆæˆ–ä½ é€‰æ‹©çš„è·¯å¾„/ç«¯å£ï¼‰ã€‚

6. **è¿è¡Œ Gateway ç½‘å…³**
   - å½“æ’ä»¶å·²å®‰è£…ä¸” `msteams` é…ç½®å­˜åœ¨å¹¶æœ‰å‡­è¯æ—¶ï¼ŒTeams æ¸ é“ä¼šè‡ªåŠ¨å¯åŠ¨ã€‚

## åŽ†å²ä¸Šä¸‹æ–‡

- `channels.msteams.historyLimit` æŽ§åˆ¶å°†å¤šå°‘æ¡æœ€è¿‘çš„é¢‘é“/ç¾¤ç»„æ¶ˆæ¯åŒ…å«åˆ°æç¤ºä¸­ã€‚
- å›žé€€åˆ° `messages.groupChat.historyLimit`ã€‚è®¾ç½® `0` ç¦ç”¨ï¼ˆé»˜è®¤ 50ï¼‰ã€‚
- ç§ä¿¡åŽ†å²å¯ä»¥é€šè¿‡ `channels.msteams.dmHistoryLimit`ï¼ˆç”¨æˆ·è½®æ¬¡ï¼‰é™åˆ¶ã€‚æ¯ç”¨æˆ·è¦†ç›–ï¼š`channels.msteams.dms["<user_id>"].historyLimit`ã€‚

## å½“å‰ Teams RSC æƒé™ï¼ˆæ¸…å•ï¼‰

è¿™äº›æ˜¯æˆ‘ä»¬ Teams åº”ç”¨æ¸…å•ä¸­**çŽ°æœ‰çš„ resourceSpecific æƒé™**ã€‚å®ƒä»¬ä»…é€‚ç”¨äºŽå®‰è£…äº†åº”ç”¨çš„å›¢é˜Ÿ/èŠå¤©å†…éƒ¨ã€‚

**å¯¹äºŽé¢‘é“ï¼ˆå›¢é˜ŸèŒƒå›´ï¼‰ï¼š**

- `ChannelMessage.Read.Group`ï¼ˆApplicationï¼‰- æ— éœ€ @æåŠå³å¯æŽ¥æ”¶æ‰€æœ‰é¢‘é“æ¶ˆæ¯
- `ChannelMessage.Send.Group`ï¼ˆApplicationï¼‰
- `Member.Read.Group`ï¼ˆApplicationï¼‰
- `Owner.Read.Group`ï¼ˆApplicationï¼‰
- `ChannelSettings.Read.Group`ï¼ˆApplicationï¼‰
- `TeamMember.Read.Group`ï¼ˆApplicationï¼‰
- `TeamSettings.Read.Group`ï¼ˆApplicationï¼‰

**å¯¹äºŽç¾¤èŠï¼š**

- `ChatMessage.Read.Chat`ï¼ˆApplicationï¼‰- æ— éœ€ @æåŠå³å¯æŽ¥æ”¶æ‰€æœ‰ç¾¤èŠæ¶ˆæ¯

## Teams æ¸…å•ç¤ºä¾‹ï¼ˆå·²è„±æ•ï¼‰

åŒ…å«å¿…éœ€å­—æ®µçš„æœ€å°æœ‰æ•ˆç¤ºä¾‹ã€‚è¯·æ›¿æ¢ ID å’Œ URLã€‚

```json
{
  "$schema": "https://developer.microsoft.com/en-us/json-schemas/teams/v1.23/MicrosoftTeams.schema.json",
  "manifestVersion": "1.23",
  "version": "1.0.0",
  "id": "00000000-0000-0000-0000-000000000000",
  "name": { "short": "KrabKrab" },
  "developer": {
    "name": "Your Org",
    "websiteUrl": "https://example.com",
    "privacyUrl": "https://example.com/privacy",
    "termsOfUseUrl": "https://example.com/terms"
  },
  "description": { "short": "KrabKrab in Teams", "full": "KrabKrab in Teams" },
  "icons": { "outline": "outline.png", "color": "color.png" },
  "accentColor": "#5B6DEF",
  "bots": [
    {
      "botId": "11111111-1111-1111-1111-111111111111",
      "scopes": ["personal", "team", "groupChat"],
      "isNotificationOnly": false,
      "supportsCalling": false,
      "supportsVideo": false,
      "supportsFiles": true
    }
  ],
  "webApplicationInfo": {
    "id": "11111111-1111-1111-1111-111111111111"
  },
  "authorization": {
    "permissions": {
      "resourceSpecific": [
        { "name": "ChannelMessage.Read.Group", "type": "Application" },
        { "name": "ChannelMessage.Send.Group", "type": "Application" },
        { "name": "Member.Read.Group", "type": "Application" },
        { "name": "Owner.Read.Group", "type": "Application" },
        { "name": "ChannelSettings.Read.Group", "type": "Application" },
        { "name": "TeamMember.Read.Group", "type": "Application" },
        { "name": "TeamSettings.Read.Group", "type": "Application" },
        { "name": "ChatMessage.Read.Chat", "type": "Application" }
      ]
    }
  }
}
```

### æ¸…å•æ³¨æ„äº‹é¡¹ï¼ˆå¿…å¡«å­—æ®µï¼‰

- `bots[].botId` **å¿…é¡»**ä¸Ž Azure Bot App ID åŒ¹é…ã€‚
- `webApplicationInfo.id` **å¿…é¡»**ä¸Ž Azure Bot App ID åŒ¹é…ã€‚
- `bots[].scopes` å¿…é¡»åŒ…å«ä½ è®¡åˆ’ä½¿ç”¨çš„ç•Œé¢ï¼ˆ`personal`ã€`team`ã€`groupChat`ï¼‰ã€‚
- `bots[].supportsFiles: true` æ˜¯ä¸ªäººèŒƒå›´æ–‡ä»¶å¤„ç†æ‰€éœ€çš„ã€‚
- `authorization.permissions.resourceSpecific` å¦‚æžœä½ éœ€è¦é¢‘é“æµé‡ï¼Œå¿…é¡»åŒ…å«é¢‘é“è¯»å–/å‘é€æƒé™ã€‚

### æ›´æ–°çŽ°æœ‰åº”ç”¨

è¦æ›´æ–°å·²å®‰è£…çš„ Teams åº”ç”¨ï¼ˆä¾‹å¦‚ï¼Œæ·»åŠ  RSC æƒé™ï¼‰ï¼š

1. ä½¿ç”¨æ–°è®¾ç½®æ›´æ–°ä½ çš„ `manifest.json`
2. **å¢žåŠ  `version` å­—æ®µ**ï¼ˆä¾‹å¦‚ï¼Œ`1.0.0` â†’ `1.1.0`ï¼‰
3. **é‡æ–°æ‰“åŒ…**æ¸…å•å’Œå›¾æ ‡ï¼ˆ`manifest.json`ã€`outline.png`ã€`color.png`ï¼‰
4. ä¸Šä¼ æ–°çš„ zipï¼š
   - **é€‰é¡¹ Aï¼ˆTeams ç®¡ç†ä¸­å¿ƒï¼‰ï¼š** Teams ç®¡ç†ä¸­å¿ƒ â†’ Teams apps â†’ Manage apps â†’ æ‰¾åˆ°ä½ çš„åº”ç”¨ â†’ Upload new version
   - **é€‰é¡¹ Bï¼ˆä¾§è½½ï¼‰ï¼š** åœ¨ Teams ä¸­ â†’ Apps â†’ Manage your apps â†’ Upload a custom app
5. **å¯¹äºŽå›¢é˜Ÿé¢‘é“ï¼š** åœ¨æ¯ä¸ªå›¢é˜Ÿä¸­é‡æ–°å®‰è£…åº”ç”¨ä»¥ä½¿æ–°æƒé™ç”Ÿæ•ˆ
6. **å®Œå…¨é€€å‡ºå¹¶é‡æ–°å¯åŠ¨ Teams**ï¼ˆä¸ä»…ä»…æ˜¯å…³é—­çª—å£ï¼‰ä»¥æ¸…é™¤ç¼“å­˜çš„åº”ç”¨å…ƒæ•°æ®

## åŠŸèƒ½ï¼šä»… RSC ä¸Ž Graph

### ä»…ä½¿ç”¨ **Teams RSC**ï¼ˆåº”ç”¨å·²å®‰è£…ï¼Œæ—  Graph API æƒé™ï¼‰

å¯ç”¨ï¼š

- è¯»å–é¢‘é“æ¶ˆæ¯**æ–‡æœ¬**å†…å®¹ã€‚
- å‘é€é¢‘é“æ¶ˆæ¯**æ–‡æœ¬**å†…å®¹ã€‚
- æŽ¥æ”¶**ä¸ªäººï¼ˆç§ä¿¡ï¼‰**æ–‡ä»¶é™„ä»¶ã€‚

ä¸å¯ç”¨ï¼š

- é¢‘é“/ç¾¤ç»„**å›¾ç‰‡æˆ–æ–‡ä»¶å†…å®¹**ï¼ˆè´Ÿè½½ä»…åŒ…å« HTML å­˜æ ¹ï¼‰ã€‚
- ä¸‹è½½å­˜å‚¨åœ¨ SharePoint/OneDrive ä¸­çš„é™„ä»¶ã€‚
- è¯»å–æ¶ˆæ¯åŽ†å²ï¼ˆè¶…å‡ºå®žæ—¶ webhook äº‹ä»¶ï¼‰ã€‚

### ä½¿ç”¨ **Teams RSC + Microsoft Graph Application æƒé™**

å¢žåŠ ï¼š

- ä¸‹è½½æ‰˜ç®¡å†…å®¹ï¼ˆç²˜è´´åˆ°æ¶ˆæ¯ä¸­çš„å›¾ç‰‡ï¼‰ã€‚
- ä¸‹è½½å­˜å‚¨åœ¨ SharePoint/OneDrive ä¸­çš„æ–‡ä»¶é™„ä»¶ã€‚
- é€šè¿‡ Graph è¯»å–é¢‘é“/èŠå¤©æ¶ˆæ¯åŽ†å²ã€‚

### RSC ä¸Ž Graph API å¯¹æ¯”

| åŠŸèƒ½           | RSC æƒé™           | Graph API                 |
| -------------- | ------------------ | ------------------------- |
| **å®žæ—¶æ¶ˆæ¯**   | æ˜¯ï¼ˆé€šè¿‡ webhookï¼‰ | å¦ï¼ˆä»…è½®è¯¢ï¼‰              |
| **åŽ†å²æ¶ˆæ¯**   | å¦                 | æ˜¯ï¼ˆå¯æŸ¥è¯¢åŽ†å²ï¼‰          |
| **è®¾ç½®å¤æ‚åº¦** | ä»…åº”ç”¨æ¸…å•         | éœ€è¦ç®¡ç†å‘˜åŒæ„ + ä»¤ç‰Œæµç¨‹ |
| **ç¦»çº¿å·¥ä½œ**   | å¦ï¼ˆå¿…é¡»è¿è¡Œï¼‰     | æ˜¯ï¼ˆéšæ—¶æŸ¥è¯¢ï¼‰            |

**ç»“è®ºï¼š** RSC ç”¨äºŽå®žæ—¶ç›‘å¬ï¼›Graph API ç”¨äºŽåŽ†å²è®¿é—®ã€‚è¦åœ¨ç¦»çº¿æ—¶è¡¥ä¸Šé”™è¿‡çš„æ¶ˆæ¯ï¼Œä½ éœ€è¦å¸¦æœ‰ `ChannelMessage.Read.All` çš„ Graph APIï¼ˆéœ€è¦ç®¡ç†å‘˜åŒæ„ï¼‰ã€‚

## å¯ç”¨ Graph çš„åª’ä½“ + åŽ†å²ï¼ˆé¢‘é“æ‰€éœ€ï¼‰

å¦‚æžœä½ éœ€è¦**é¢‘é“**ä¸­çš„å›¾ç‰‡/æ–‡ä»¶æˆ–æƒ³è¦èŽ·å–**æ¶ˆæ¯åŽ†å²**ï¼Œä½ å¿…é¡»å¯ç”¨ Microsoft Graph æƒé™å¹¶æŽˆäºˆç®¡ç†å‘˜åŒæ„ã€‚

1. åœ¨ Entra IDï¼ˆAzure ADï¼‰**App Registration** ä¸­ï¼Œæ·»åŠ  Microsoft Graph **Application æƒé™**ï¼š
   - `ChannelMessage.Read.All`ï¼ˆé¢‘é“é™„ä»¶ + åŽ†å²ï¼‰
   - `Chat.Read.All` æˆ– `ChatMessage.Read.All`ï¼ˆç¾¤èŠï¼‰
2. ä¸ºç§Ÿæˆ·**æŽˆäºˆç®¡ç†å‘˜åŒæ„**ã€‚
3. æå‡ Teams åº”ç”¨**æ¸…å•ç‰ˆæœ¬**ï¼Œé‡æ–°ä¸Šä¼ ï¼Œå¹¶**åœ¨ Teams ä¸­é‡æ–°å®‰è£…åº”ç”¨**ã€‚
4. **å®Œå…¨é€€å‡ºå¹¶é‡æ–°å¯åŠ¨ Teams** ä»¥æ¸…é™¤ç¼“å­˜çš„åº”ç”¨å…ƒæ•°æ®ã€‚

## å·²çŸ¥é™åˆ¶

### Webhook è¶…æ—¶

Teams é€šè¿‡ HTTP webhook ä¼ é€’æ¶ˆæ¯ã€‚å¦‚æžœå¤„ç†æ—¶é—´è¿‡é•¿ï¼ˆä¾‹å¦‚ï¼ŒLLM å“åº”ç¼“æ…¢ï¼‰ï¼Œä½ å¯èƒ½ä¼šçœ‹åˆ°ï¼š

- Gateway ç½‘å…³è¶…æ—¶
- Teams é‡è¯•æ¶ˆæ¯ï¼ˆå¯¼è‡´é‡å¤ï¼‰
- ä¸¢å¤±çš„å›žå¤

KrabKrab é€šè¿‡å¿«é€Ÿè¿”å›žå¹¶ä¸»åŠ¨å‘é€å›žå¤æ¥å¤„ç†è¿™ä¸ªé—®é¢˜ï¼Œä½†éžå¸¸æ…¢çš„å“åº”ä»å¯èƒ½å¯¼è‡´é—®é¢˜ã€‚

### æ ¼å¼åŒ–

Teams markdown æ¯” Slack æˆ– Discord æ›´æœ‰é™ï¼š

- åŸºæœ¬æ ¼å¼åŒ–æœ‰æ•ˆï¼š**ç²—ä½“**ã€_æ–œä½“_ã€`ä»£ç `ã€é“¾æŽ¥
- å¤æ‚çš„ markdownï¼ˆè¡¨æ ¼ã€åµŒå¥—åˆ—è¡¨ï¼‰å¯èƒ½æ— æ³•æ­£ç¡®æ¸²æŸ“
- æ”¯æŒ Adaptive Cards ç”¨äºŽæŠ•ç¥¨å’Œä»»æ„å¡ç‰‡å‘é€ï¼ˆè§ä¸‹æ–‡ï¼‰

## é…ç½®

å…³é”®è®¾ç½®ï¼ˆå…±äº«æ¸ é“æ¨¡å¼è§ `/gateway/configuration`ï¼‰ï¼š

- `channels.msteams.enabled`ï¼šå¯ç”¨/ç¦ç”¨æ¸ é“ã€‚
- `channels.msteams.appId`ã€`channels.msteams.appPassword`ã€`channels.msteams.tenantId`ï¼šæœºå™¨äººå‡­è¯ã€‚
- `channels.msteams.webhook.port`ï¼ˆé»˜è®¤ `3978`ï¼‰
- `channels.msteams.webhook.path`ï¼ˆé»˜è®¤ `/api/messages`ï¼‰
- `channels.msteams.dmPolicy`ï¼š`pairing | allowlist | open | disabled`ï¼ˆé»˜è®¤ï¼špairingï¼‰
- `channels.msteams.allowFrom`ï¼šç§ä¿¡å…è®¸åˆ—è¡¨ï¼ˆAAD å¯¹è±¡ IDã€UPN æˆ–æ˜¾ç¤ºåç§°ï¼‰ã€‚å½“ Graph è®¿é—®å¯ç”¨æ—¶ï¼Œå‘å¯¼åœ¨è®¾ç½®æœŸé—´å°†åç§°è§£æžä¸º IDã€‚
- `channels.msteams.textChunkLimit`ï¼šå‡ºç«™æ–‡æœ¬åˆ†å—å¤§å°ã€‚
- `channels.msteams.chunkMode`ï¼š`length`ï¼ˆé»˜è®¤ï¼‰æˆ– `newline` åœ¨é•¿åº¦åˆ†å—ä¹‹å‰æŒ‰ç©ºè¡Œï¼ˆæ®µè½è¾¹ç•Œï¼‰åˆ†å‰²ã€‚
- `channels.msteams.mediaAllowHosts`ï¼šå…¥ç«™é™„ä»¶ä¸»æœºå…è®¸åˆ—è¡¨ï¼ˆé»˜è®¤ä¸º Microsoft/Teams åŸŸåï¼‰ã€‚
- `channels.msteams.mediaAuthAllowHosts`ï¼šåœ¨åª’ä½“é‡è¯•æ—¶é™„åŠ  Authorization å¤´çš„å…è®¸åˆ—è¡¨ï¼ˆé»˜è®¤ä¸º Graph + Bot Framework ä¸»æœºï¼‰ã€‚
- `channels.msteams.requireMention`ï¼šåœ¨é¢‘é“/ç¾¤ç»„ä¸­éœ€è¦ @æåŠï¼ˆé»˜è®¤ trueï¼‰ã€‚
- `channels.msteams.replyStyle`ï¼š`thread | top-level`ï¼ˆè§[å›žå¤æ ·å¼](#reply-style-threads-vs-posts)ï¼‰ã€‚
- `channels.msteams.teams.<teamId>.replyStyle`ï¼šæ¯å›¢é˜Ÿè¦†ç›–ã€‚
- `channels.msteams.teams.<teamId>.requireMention`ï¼šæ¯å›¢é˜Ÿè¦†ç›–ã€‚
- `channels.msteams.teams.<teamId>.tools`ï¼šå½“ç¼ºå°‘é¢‘é“è¦†ç›–æ—¶ä½¿ç”¨çš„é»˜è®¤æ¯å›¢é˜Ÿå·¥å…·ç­–ç•¥è¦†ç›–ï¼ˆ`allow`/`deny`/`alsoAllow`ï¼‰ã€‚
- `channels.msteams.teams.<teamId>.toolsBySender`ï¼šé»˜è®¤æ¯å›¢é˜Ÿæ¯å‘é€è€…å·¥å…·ç­–ç•¥è¦†ç›–ï¼ˆæ”¯æŒ `"*"` é€šé…ç¬¦ï¼‰ã€‚
- `channels.msteams.teams.<teamId>.channels.<conversationId>.replyStyle`ï¼šæ¯é¢‘é“è¦†ç›–ã€‚
- `channels.msteams.teams.<teamId>.channels.<conversationId>.requireMention`ï¼šæ¯é¢‘é“è¦†ç›–ã€‚
- `channels.msteams.teams.<teamId>.channels.<conversationId>.tools`ï¼šæ¯é¢‘é“å·¥å…·ç­–ç•¥è¦†ç›–ï¼ˆ`allow`/`deny`/`alsoAllow`ï¼‰ã€‚
- `channels.msteams.teams.<teamId>.channels.<conversationId>.toolsBySender`ï¼šæ¯é¢‘é“æ¯å‘é€è€…å·¥å…·ç­–ç•¥è¦†ç›–ï¼ˆæ”¯æŒ `"*"` é€šé…ç¬¦ï¼‰ã€‚
- `channels.msteams.sharePointSiteId`ï¼šç”¨äºŽç¾¤èŠ/é¢‘é“æ–‡ä»¶ä¸Šä¼ çš„ SharePoint ç«™ç‚¹ IDï¼ˆè§[åœ¨ç¾¤èŠä¸­å‘é€æ–‡ä»¶](#sending-files-in-group-chats)ï¼‰ã€‚

## è·¯ç”±å’Œä¼šè¯

- ä¼šè¯é”®éµå¾ªæ ‡å‡†æ™ºèƒ½ä½“æ ¼å¼ï¼ˆè§ [/concepts/session](/concepts/session)ï¼‰ï¼š
  - ç§ä¿¡å…±äº«ä¸»ä¼šè¯ï¼ˆ`agent:<agentId>:<mainKey>`ï¼‰ã€‚
  - é¢‘é“/ç¾¤ç»„æ¶ˆæ¯ä½¿ç”¨ä¼šè¯ IDï¼š
    - `agent:<agentId>:msteams:channel:<conversationId>`
    - `agent:<agentId>:msteams:group:<conversationId>`

## å›žå¤æ ·å¼ï¼šè¯é¢˜ vs å¸–å­

Teams æœ€è¿‘åœ¨ç›¸åŒçš„åº•å±‚æ•°æ®æ¨¡åž‹ä¸Šå¼•å…¥äº†ä¸¤ç§é¢‘é“ UI æ ·å¼ï¼š

| æ ·å¼                    | æè¿°                           | æŽ¨èçš„ `replyStyle` |
| ----------------------- | ------------------------------ | ------------------- |
| **Posts**ï¼ˆç»å…¸ï¼‰       | æ¶ˆæ¯æ˜¾ç¤ºä¸ºå¡ç‰‡ï¼Œä¸‹æ–¹æœ‰è¯é¢˜å›žå¤ | `thread`ï¼ˆé»˜è®¤ï¼‰    |
| **Threads**ï¼ˆç±» Slackï¼‰ | æ¶ˆæ¯çº¿æ€§æµåŠ¨ï¼Œæ›´åƒ Slack       | `top-level`         |

**é—®é¢˜ï¼š** Teams API ä¸æš´éœ²é¢‘é“ä½¿ç”¨çš„ UI æ ·å¼ã€‚å¦‚æžœä½ ä½¿ç”¨é”™è¯¯çš„ `replyStyle`ï¼š

- åœ¨ Threads æ ·å¼é¢‘é“ä¸­ä½¿ç”¨ `thread` â†’ å›žå¤åµŒå¥—æ˜¾ç¤ºå¾ˆåˆ«æ‰­
- åœ¨ Posts æ ·å¼é¢‘é“ä¸­ä½¿ç”¨ `top-level` â†’ å›žå¤æ˜¾ç¤ºä¸ºå•ç‹¬çš„é¡¶çº§å¸–å­è€Œä¸æ˜¯åœ¨è¯é¢˜ä¸­

**è§£å†³æ–¹æ¡ˆï¼š** æ ¹æ®é¢‘é“çš„è®¾ç½®æ–¹å¼ä¸ºæ¯ä¸ªé¢‘é“é…ç½® `replyStyle`ï¼š

```json
{
  "msteams": {
    "replyStyle": "thread",
    "teams": {
      "19:abc...@thread.tacv2": {
        "channels": {
          "19:xyz...@thread.tacv2": {
            "replyStyle": "top-level"
          }
        }
      }
    }
  }
}
```

## é™„ä»¶å’Œå›¾ç‰‡

**å½“å‰é™åˆ¶ï¼š**

- **ç§ä¿¡ï¼š** å›¾ç‰‡å’Œæ–‡ä»¶é™„ä»¶é€šè¿‡ Teams bot file API å·¥ä½œã€‚
- **é¢‘é“/ç¾¤ç»„ï¼š** é™„ä»¶å­˜å‚¨åœ¨ M365 å­˜å‚¨ï¼ˆSharePoint/OneDriveï¼‰ä¸­ã€‚webhook è´Ÿè½½ä»…åŒ…å« HTML å­˜æ ¹ï¼Œè€Œéžå®žé™…æ–‡ä»¶å­—èŠ‚ã€‚**éœ€è¦ Graph API æƒé™**æ‰èƒ½ä¸‹è½½é¢‘é“é™„ä»¶ã€‚

æ²¡æœ‰ Graph æƒé™ï¼Œå¸¦å›¾ç‰‡çš„é¢‘é“æ¶ˆæ¯å°†ä½œä¸ºçº¯æ–‡æœ¬æŽ¥æ”¶ï¼ˆæœºå™¨äººæ— æ³•è®¿é—®å›¾ç‰‡å†…å®¹ï¼‰ã€‚
é»˜è®¤æƒ…å†µä¸‹ï¼ŒKrabKrab ä»…ä»Ž Microsoft/Teams ä¸»æœºåä¸‹è½½åª’ä½“ã€‚ä½¿ç”¨ `channels.msteams.mediaAllowHosts` è¦†ç›–ï¼ˆä½¿ç”¨ `["*"]` å…è®¸ä»»ä½•ä¸»æœºï¼‰ã€‚
Authorization å¤´ä»…é™„åŠ åˆ° `channels.msteams.mediaAuthAllowHosts` ä¸­çš„ä¸»æœºï¼ˆé»˜è®¤ä¸º Graph + Bot Framework ä¸»æœºï¼‰ã€‚ä¿æŒæ­¤åˆ—è¡¨ä¸¥æ ¼ï¼ˆé¿å…å¤šç§Ÿæˆ·åŽç¼€ï¼‰ã€‚

## åœ¨ç¾¤èŠä¸­å‘é€æ–‡ä»¶

æœºå™¨äººå¯ä»¥ä½¿ç”¨ FileConsentCard æµç¨‹åœ¨ç§ä¿¡ä¸­å‘é€æ–‡ä»¶ï¼ˆå†…ç½®ï¼‰ã€‚ä½†æ˜¯ï¼Œ**åœ¨ç¾¤èŠ/é¢‘é“ä¸­å‘é€æ–‡ä»¶**éœ€è¦é¢å¤–è®¾ç½®ï¼š

| ä¸Šä¸‹æ–‡                 | æ–‡ä»¶å‘é€æ–¹å¼                            | æ‰€éœ€è®¾ç½®                             |
| ---------------------- | --------------------------------------- | ------------------------------------ |
| **ç§ä¿¡**               | FileConsentCard â†’ ç”¨æˆ·æŽ¥å— â†’ æœºå™¨äººä¸Šä¼  | å¼€ç®±å³ç”¨                             |
| **ç¾¤èŠ/é¢‘é“**          | ä¸Šä¼ åˆ° SharePoint â†’ å…±äº«é“¾æŽ¥            | éœ€è¦ `sharePointSiteId` + Graph æƒé™ |
| **å›¾ç‰‡ï¼ˆä»»ä½•ä¸Šä¸‹æ–‡ï¼‰** | Base64 ç¼–ç å†…è”                         | å¼€ç®±å³ç”¨                             |

### ä¸ºä»€ä¹ˆç¾¤èŠéœ€è¦ SharePoint

æœºå™¨äººæ²¡æœ‰ä¸ªäºº OneDrive é©±åŠ¨å™¨ï¼ˆ`/me/drive` Graph API ç«¯ç‚¹å¯¹åº”ç”¨ç¨‹åºèº«ä»½ä¸èµ·ä½œç”¨ï¼‰ã€‚è¦åœ¨ç¾¤èŠ/é¢‘é“ä¸­å‘é€æ–‡ä»¶ï¼Œæœºå™¨äººä¸Šä¼ åˆ° **SharePoint ç«™ç‚¹**å¹¶åˆ›å»ºå…±äº«é“¾æŽ¥ã€‚

### è®¾ç½®

1. **åœ¨ Entra IDï¼ˆAzure ADï¼‰â†’ App Registration ä¸­æ·»åŠ  Graph API æƒé™**ï¼š
   - `Sites.ReadWrite.All`ï¼ˆApplicationï¼‰- ä¸Šä¼ æ–‡ä»¶åˆ° SharePoint
   - `Chat.Read.All`ï¼ˆApplicationï¼‰- å¯é€‰ï¼Œå¯ç”¨æ¯ç”¨æˆ·å…±äº«é“¾æŽ¥

2. ä¸ºç§Ÿæˆ·**æŽˆäºˆç®¡ç†å‘˜åŒæ„**ã€‚

3. **èŽ·å–ä½ çš„ SharePoint ç«™ç‚¹ IDï¼š**

   ```bash
   # é€šè¿‡ Graph Explorer æˆ–å¸¦æœ‰æ•ˆä»¤ç‰Œçš„ curlï¼š
   curl -H "Authorization: Bearer $TOKEN" \
     "https://graph.microsoft.com/v1.0/sites/{hostname}:/{site-path}"

   # ç¤ºä¾‹ï¼šå¯¹äºŽ "contoso.sharepoint.com/sites/BotFiles" çš„ç«™ç‚¹
   curl -H "Authorization: Bearer $TOKEN" \
     "https://graph.microsoft.com/v1.0/sites/contoso.sharepoint.com:/sites/BotFiles"

   # å“åº”åŒ…å«ï¼š"id": "contoso.sharepoint.com,guid1,guid2"
   ```

4. **é…ç½® KrabKrabï¼š**
   ```json5
   {
     channels: {
       msteams: {
         // ... å…¶ä»–é…ç½® ...
         sharePointSiteId: "contoso.sharepoint.com,guid1,guid2",
       },
     },
   }
   ```

### å…±äº«è¡Œä¸º

| æƒé™                                    | å…±äº«è¡Œä¸º                                   |
| --------------------------------------- | ------------------------------------------ |
| ä»… `Sites.ReadWrite.All`                | ç»„ç»‡èŒƒå›´å…±äº«é“¾æŽ¥ï¼ˆç»„ç»‡ä¸­ä»»ä½•äººéƒ½å¯ä»¥è®¿é—®ï¼‰ |
| `Sites.ReadWrite.All` + `Chat.Read.All` | æ¯ç”¨æˆ·å…±äº«é“¾æŽ¥ï¼ˆä»…èŠå¤©æˆå‘˜å¯ä»¥è®¿é—®ï¼‰       |

æ¯ç”¨æˆ·å…±äº«æ›´å®‰å…¨ï¼Œå› ä¸ºåªæœ‰èŠå¤©å‚ä¸Žè€…æ‰èƒ½è®¿é—®æ–‡ä»¶ã€‚å¦‚æžœç¼ºå°‘ `Chat.Read.All` æƒé™ï¼Œæœºå™¨äººå›žé€€åˆ°ç»„ç»‡èŒƒå›´å…±äº«ã€‚

### å›žé€€è¡Œä¸º

| åœºæ™¯                                    | ç»“æžœ                                             |
| --------------------------------------- | ------------------------------------------------ |
| ç¾¤èŠ + æ–‡ä»¶ + å·²é…ç½® `sharePointSiteId` | ä¸Šä¼ åˆ° SharePointï¼Œå‘é€å…±äº«é“¾æŽ¥                  |
| ç¾¤èŠ + æ–‡ä»¶ + æ—  `sharePointSiteId`     | å°è¯• OneDrive ä¸Šä¼ ï¼ˆå¯èƒ½å¤±è´¥ï¼‰ï¼Œä»…å‘é€æ–‡æœ¬       |
| ä¸ªäººèŠå¤© + æ–‡ä»¶                         | FileConsentCard æµç¨‹ï¼ˆæ— éœ€ SharePoint å³å¯å·¥ä½œï¼‰ |
| ä»»ä½•ä¸Šä¸‹æ–‡ + å›¾ç‰‡                       | Base64 ç¼–ç å†…è”ï¼ˆæ— éœ€ SharePoint å³å¯å·¥ä½œï¼‰      |

### æ–‡ä»¶å­˜å‚¨ä½ç½®

ä¸Šä¼ çš„æ–‡ä»¶å­˜å‚¨åœ¨é…ç½®çš„ SharePoint ç«™ç‚¹é»˜è®¤æ–‡æ¡£åº“ä¸­çš„ `/krabkrabShared/` æ–‡ä»¶å¤¹ä¸­ã€‚

## æŠ•ç¥¨ï¼ˆAdaptive Cardsï¼‰

KrabKrab å°† Teams æŠ•ç¥¨ä½œä¸º Adaptive Cards å‘é€ï¼ˆæ²¡æœ‰åŽŸç”Ÿ Teams æŠ•ç¥¨ APIï¼‰ã€‚

- CLIï¼š`krabkrab message poll --channel msteams --target conversation:<id> ...`
- æŠ•ç¥¨ç”± Gateway ç½‘å…³è®°å½•åœ¨ `~/.krabkrab/msteams-polls.json` ä¸­ã€‚
- Gateway ç½‘å…³å¿…é¡»ä¿æŒåœ¨çº¿æ‰èƒ½è®°å½•æŠ•ç¥¨ã€‚
- æŠ•ç¥¨å°šä¸è‡ªåŠ¨å‘å¸ƒç»“æžœæ‘˜è¦ï¼ˆå¦‚éœ€è¦è¯·æ£€æŸ¥å­˜å‚¨æ–‡ä»¶ï¼‰ã€‚

## Adaptive Cardsï¼ˆä»»æ„ï¼‰

ä½¿ç”¨ `message` å·¥å…·æˆ– CLI å‘ Teams ç”¨æˆ·æˆ–ä¼šè¯å‘é€ä»»æ„ Adaptive Card JSONã€‚

`card` å‚æ•°æŽ¥å— Adaptive Card JSON å¯¹è±¡ã€‚å½“æä¾› `card` æ—¶ï¼Œæ¶ˆæ¯æ–‡æœ¬æ˜¯å¯é€‰çš„ã€‚

**æ™ºèƒ½ä½“å·¥å…·ï¼š**

```json
{
  "action": "send",
  "channel": "msteams",
  "target": "user:<id>",
  "card": {
    "type": "AdaptiveCard",
    "version": "1.5",
    "body": [{ "type": "TextBlock", "text": "Hello!" }]
  }
}
```

**CLIï¼š**

```bash
krabkrab message send --channel msteams \
  --target "conversation:19:abc...@thread.tacv2" \
  --card '{"type":"AdaptiveCard","version":"1.5","body":[{"type":"TextBlock","text":"Hello!"}]}'
```

å‚è§ [Adaptive Cards æ–‡æ¡£](https://adaptivecards.io/)äº†è§£å¡ç‰‡æ¨¡å¼å’Œç¤ºä¾‹ã€‚ç›®æ ‡æ ¼å¼è¯¦æƒ…è§ä¸‹æ–¹[ç›®æ ‡æ ¼å¼](#target-formats)ã€‚

## ç›®æ ‡æ ¼å¼

MSTeams ç›®æ ‡ä½¿ç”¨å‰ç¼€æ¥åŒºåˆ†ç”¨æˆ·å’Œä¼šè¯ï¼š

| ç›®æ ‡ç±»åž‹          | æ ¼å¼                             | ç¤ºä¾‹                                              |
| ----------------- | -------------------------------- | ------------------------------------------------- |
| ç”¨æˆ·ï¼ˆæŒ‰ IDï¼‰     | `user:<aad-object-id>`           | `user:40a1a0ed-4ff2-4164-a219-55518990c197`       |
| ç”¨æˆ·ï¼ˆæŒ‰åç§°ï¼‰    | `user:<display-name>`            | `user:John Smith`ï¼ˆéœ€è¦ Graph APIï¼‰               |
| ç¾¤ç»„/é¢‘é“         | `conversation:<conversation-id>` | `conversation:19:abc123...@thread.tacv2`          |
| ç¾¤ç»„/é¢‘é“ï¼ˆåŽŸå§‹ï¼‰ | `<conversation-id>`              | `19:abc123...@thread.tacv2`ï¼ˆå¦‚æžœåŒ…å« `@thread`ï¼‰ |

**CLI ç¤ºä¾‹ï¼š**

```bash
# æŒ‰ ID å‘é€ç»™ç”¨æˆ·
krabkrab message send --channel msteams --target "user:40a1a0ed-..." --message "Hello"

# æŒ‰æ˜¾ç¤ºåç§°å‘é€ç»™ç”¨æˆ·ï¼ˆè§¦å‘ Graph API æŸ¥æ‰¾ï¼‰
krabkrab message send --channel msteams --target "user:John Smith" --message "Hello"

# å‘é€åˆ°ç¾¤èŠæˆ–é¢‘é“
krabkrab message send --channel msteams --target "conversation:19:abc...@thread.tacv2" --message "Hello"

# å‘ä¼šè¯å‘é€ Adaptive Card
krabkrab message send --channel msteams --target "conversation:19:abc...@thread.tacv2" \
  --card '{"type":"AdaptiveCard","version":"1.5","body":[{"type":"TextBlock","text":"Hello"}]}'
```

**æ™ºèƒ½ä½“å·¥å…·ç¤ºä¾‹ï¼š**

```json
{
  "action": "send",
  "channel": "msteams",
  "target": "user:John Smith",
  "message": "Hello!"
}
```

```json
{
  "action": "send",
  "channel": "msteams",
  "target": "conversation:19:abc...@thread.tacv2",
  "card": {
    "type": "AdaptiveCard",
    "version": "1.5",
    "body": [{ "type": "TextBlock", "text": "Hello" }]
  }
}
```

æ³¨æ„ï¼šæ²¡æœ‰ `user:` å‰ç¼€æ—¶ï¼Œåç§°é»˜è®¤è§£æžä¸ºç¾¤ç»„/å›¢é˜Ÿã€‚æŒ‰æ˜¾ç¤ºåç§°å®šä½äººå‘˜æ—¶å§‹ç»ˆä½¿ç”¨ `user:`ã€‚

## ä¸»åŠ¨æ¶ˆæ¯

- ä¸»åŠ¨æ¶ˆæ¯ä»…åœ¨ç”¨æˆ·äº¤äº’**ä¹‹åŽ**æ‰å¯èƒ½ï¼Œå› ä¸ºæˆ‘ä»¬åœ¨é‚£æ—¶å­˜å‚¨ä¼šè¯å¼•ç”¨ã€‚
- æœ‰å…³ `dmPolicy` å’Œå…è®¸åˆ—è¡¨æŽ§åˆ¶ï¼Œè¯·å‚è§ `/gateway/configuration`ã€‚

## å›¢é˜Ÿå’Œé¢‘é“ IDï¼ˆå¸¸è§é™·é˜±ï¼‰

Teams URL ä¸­çš„ `groupId` æŸ¥è¯¢å‚æ•°**ä¸æ˜¯**ç”¨äºŽé…ç½®çš„å›¢é˜Ÿ IDã€‚è¯·ä»Ž URL è·¯å¾„ä¸­æå– IDï¼š

**å›¢é˜Ÿ URLï¼š**

```
https://teams.microsoft.com/l/team/19%3ABk4j...%40thread.tacv2/conversations?groupId=...
                                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                    å›¢é˜Ÿ IDï¼ˆURL è§£ç æ­¤éƒ¨åˆ†ï¼‰
```

**é¢‘é“ URLï¼š**

```
https://teams.microsoft.com/l/channel/19%3A15bc...%40thread.tacv2/ChannelName?groupId=...
                                      â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                                      é¢‘é“ IDï¼ˆURL è§£ç æ­¤éƒ¨åˆ†ï¼‰
```

**ç”¨äºŽé…ç½®ï¼š**

- å›¢é˜Ÿ ID = `/team/` åŽçš„è·¯å¾„æ®µï¼ˆURL è§£ç ï¼Œä¾‹å¦‚ `19:Bk4j...@thread.tacv2`ï¼‰
- é¢‘é“ ID = `/channel/` åŽçš„è·¯å¾„æ®µï¼ˆURL è§£ç ï¼‰
- **å¿½ç•¥** `groupId` æŸ¥è¯¢å‚æ•°

## ç§æœ‰é¢‘é“

æœºå™¨äººåœ¨ç§æœ‰é¢‘é“ä¸­çš„æ”¯æŒæœ‰é™ï¼š

| åŠŸèƒ½                | æ ‡å‡†é¢‘é“ | ç§æœ‰é¢‘é“         |
| ------------------- | -------- | ---------------- |
| æœºå™¨äººå®‰è£…          | æ˜¯       | æœ‰é™             |
| å®žæ—¶æ¶ˆæ¯ï¼ˆwebhookï¼‰ | æ˜¯       | å¯èƒ½ä¸å·¥ä½œ       |
| RSC æƒé™            | æ˜¯       | è¡Œä¸ºå¯èƒ½ä¸åŒ     |
| @æåŠ               | æ˜¯       | å¦‚æžœæœºå™¨äººå¯è®¿é—® |
| Graph API åŽ†å²      | æ˜¯       | æ˜¯ï¼ˆæœ‰æƒé™ï¼‰     |

**å¦‚æžœç§æœ‰é¢‘é“ä¸å·¥ä½œçš„å˜é€šæ–¹æ³•ï¼š**

1. ä½¿ç”¨æ ‡å‡†é¢‘é“è¿›è¡Œæœºå™¨äººäº¤äº’
2. ä½¿ç”¨ç§ä¿¡ - ç”¨æˆ·å§‹ç»ˆå¯ä»¥ç›´æŽ¥ç»™æœºå™¨äººå‘æ¶ˆæ¯
3. ä½¿ç”¨ Graph API è¿›è¡ŒåŽ†å²è®¿é—®ï¼ˆéœ€è¦ `ChannelMessage.Read.All`ï¼‰

## æ•…éšœæŽ’é™¤

### å¸¸è§é—®é¢˜

- **é¢‘é“ä¸­å›¾ç‰‡ä¸æ˜¾ç¤ºï¼š** ç¼ºå°‘ Graph æƒé™æˆ–ç®¡ç†å‘˜åŒæ„ã€‚é‡æ–°å®‰è£… Teams åº”ç”¨å¹¶å®Œå…¨é€€å‡º/é‡æ–°æ‰“å¼€ Teamsã€‚
- **é¢‘é“ä¸­æ— å“åº”ï¼š** é»˜è®¤éœ€è¦æåŠï¼›è®¾ç½® `channels.msteams.requireMention=false` æˆ–æŒ‰å›¢é˜Ÿ/é¢‘é“é…ç½®ã€‚
- **ç‰ˆæœ¬ä¸åŒ¹é…ï¼ˆTeams ä»æ˜¾ç¤ºæ—§æ¸…å•ï¼‰ï¼š** ç§»é™¤ + é‡æ–°æ·»åŠ åº”ç”¨å¹¶å®Œå…¨é€€å‡º Teams ä»¥åˆ·æ–°ã€‚
- **æ¥è‡ª webhook çš„ 401 Unauthorizedï¼š** åœ¨æ²¡æœ‰ Azure JWT çš„æƒ…å†µä¸‹æ‰‹åŠ¨æµ‹è¯•æ—¶å±žäºŽé¢„æœŸæƒ…å†µ - æ„å‘³ç€ç«¯ç‚¹å¯è¾¾ä½†è®¤è¯å¤±è´¥ã€‚ä½¿ç”¨ Azure Web Chat æ­£ç¡®æµ‹è¯•ã€‚

### æ¸…å•ä¸Šä¼ é”™è¯¯

- **"Icon file cannot be empty"ï¼š** æ¸…å•å¼•ç”¨çš„å›¾æ ‡æ–‡ä»¶ä¸º 0 å­—èŠ‚ã€‚åˆ›å»ºæœ‰æ•ˆçš„ PNG å›¾æ ‡ï¼ˆ`outline.png` ä¸º 32x32ï¼Œ`color.png` ä¸º 192x192ï¼‰ã€‚
- **"webApplicationInfo.Id already in use"ï¼š** åº”ç”¨ä»å®‰è£…åœ¨å¦ä¸€ä¸ªå›¢é˜Ÿ/èŠå¤©ä¸­ã€‚å…ˆæ‰¾åˆ°å¹¶å¸è½½å®ƒï¼Œæˆ–ç­‰å¾… 5-10 åˆ†é’Ÿè®©å…¶ä¼ æ’­ã€‚
- **ä¸Šä¼ æ—¶"Something went wrong"ï¼š** æ”¹ä¸ºé€šè¿‡ https://admin.teams.microsoft.com ä¸Šä¼ ï¼Œæ‰“å¼€æµè§ˆå™¨ DevToolsï¼ˆF12ï¼‰â†’ Network é€‰é¡¹å¡ï¼Œæ£€æŸ¥å“åº”æ­£æ–‡ä¸­çš„å®žé™…é”™è¯¯ã€‚
- **ä¾§è½½å¤±è´¥ï¼š** å°è¯•"Upload an app to your org's app catalog"è€Œä¸æ˜¯"Upload a custom app" - è¿™é€šå¸¸å¯ä»¥ç»•è¿‡ä¾§è½½é™åˆ¶ã€‚

### RSC æƒé™ä¸å·¥ä½œ

1. éªŒè¯ `webApplicationInfo.id` ä¸Žä½ çš„æœºå™¨äºº App ID å®Œå…¨åŒ¹é…
2. é‡æ–°ä¸Šä¼ åº”ç”¨å¹¶åœ¨å›¢é˜Ÿ/èŠå¤©ä¸­é‡æ–°å®‰è£…
3. æ£€æŸ¥ä½ çš„ç»„ç»‡ç®¡ç†å‘˜æ˜¯å¦é˜»æ­¢äº† RSC æƒé™
4. ç¡®è®¤ä½ ä½¿ç”¨çš„æ˜¯æ­£ç¡®çš„èŒƒå›´ï¼šå›¢é˜Ÿä½¿ç”¨ `ChannelMessage.Read.Group`ï¼Œç¾¤èŠä½¿ç”¨ `ChatMessage.Read.Chat`

## å‚è€ƒèµ„æ–™

- [åˆ›å»º Azure Bot](https://learn.microsoft.com/en-us/azure/bot-service/bot-service-quickstart-registration) - Azure Bot è®¾ç½®æŒ‡å—
- [Teams å¼€å‘è€…é—¨æˆ·](https://dev.teams.microsoft.com/apps) - åˆ›å»º/ç®¡ç† Teams åº”ç”¨
- [Teams åº”ç”¨æ¸…å•æ¨¡å¼](https://learn.microsoft.com/en-us/microsoftteams/platform/resources/schema/manifest-schema)
- [ä½¿ç”¨ RSC æŽ¥æ”¶é¢‘é“æ¶ˆæ¯](https://learn.microsoft.com/en-us/microsoftteams/platform/bots/how-to/conversations/channel-messages-with-rsc)
- [RSC æƒé™å‚è€ƒ](https://learn.microsoft.com/en-us/microsoftteams/platform/graph-api/rsc/resource-specific-consent)
- [Teams æœºå™¨äººæ–‡ä»¶å¤„ç†](https://learn.microsoft.com/en-us/microsoftteams/platform/bots/how-to/bots-filesv4)ï¼ˆé¢‘é“/ç¾¤ç»„éœ€è¦ Graphï¼‰
- [ä¸»åŠ¨æ¶ˆæ¯](https://learn.microsoft.com/en-us/microsoftteams/platform/bots/how-to/conversations/send-proactive-messages)

