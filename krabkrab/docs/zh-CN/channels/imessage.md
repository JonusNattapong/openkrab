---
read_when:
  - è®¾ç½® iMessage æ”¯æŒ
  - è°ƒè¯• iMessage å‘é€/æŽ¥æ”¶
summary: é€šè¿‡ imsgï¼ˆåŸºäºŽ stdio çš„ JSON-RPCï¼‰å®žçŽ° iMessage æ”¯æŒã€è®¾ç½®åŠ chat_id è·¯ç”±
title: iMessage
x-i18n:
  generated_at: "2026-02-03T07:44:18Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: bc19756a42ead80a0845f18c4830c3f1f40948f69b2b016a4026598cfb8fef0d
  source_path: channels/imessage.md
  workflow: 15
---

# iMessage (imsg)

çŠ¶æ€ï¼šå¤–éƒ¨ CLI é›†æˆã€‚Gateway ç½‘å…³ç”Ÿæˆ `imsg rpc`ï¼ˆåŸºäºŽ stdio çš„ JSON-RPCï¼‰ã€‚

## å¿«é€Ÿè®¾ç½®ï¼ˆæ–°æ‰‹ï¼‰

1. ç¡®ä¿åœ¨æ­¤ Mac ä¸Šå·²ç™»å½•"ä¿¡æ¯"ã€‚
2. å®‰è£… `imsg`ï¼š
   - `brew install steipete/tap/imsg`
3. é…ç½® KrabKrab çš„ `channels.imessage.cliPath` å’Œ `channels.imessage.dbPath`ã€‚
4. å¯åŠ¨ Gateway ç½‘å…³å¹¶æ‰¹å‡†æ‰€æœ‰ macOS æç¤ºï¼ˆè‡ªåŠ¨åŒ– + å®Œå…¨ç£ç›˜è®¿é—®æƒé™ï¼‰ã€‚

æœ€å°é…ç½®ï¼š

```json5
{
  channels: {
    imessage: {
      enabled: true,
      cliPath: "/usr/local/bin/imsg",
      dbPath: "/Users/<you>/Library/Messages/chat.db",
    },
  },
}
```

## ç®€ä»‹

- åŸºäºŽ macOS ä¸Š `imsg` çš„ iMessage æ¸ é“ã€‚
- ç¡®å®šæ€§è·¯ç”±ï¼šå›žå¤å§‹ç»ˆè¿”å›žåˆ° iMessageã€‚
- ç§ä¿¡å…±äº«æ™ºèƒ½ä½“çš„ä¸»ä¼šè¯ï¼›ç¾¤ç»„æ˜¯éš”ç¦»çš„ï¼ˆ`agent:<agentId>:imessage:group:<chat_id>`ï¼‰ã€‚
- å¦‚æžœå¤šå‚ä¸Žè€…ä¼šè¯ä»¥ `is_group=false` åˆ°è¾¾ï¼Œä½ ä»å¯ä½¿ç”¨ `channels.imessage.groups` æŒ‰ `chat_id` éš”ç¦»ï¼ˆå‚è§ä¸‹æ–¹"ç±»ç¾¤ç»„ä¼šè¯"ï¼‰ã€‚

## é…ç½®å†™å…¥

é»˜è®¤æƒ…å†µä¸‹ï¼ŒiMessage å…è®¸å†™å…¥ç”± `/config set|unset` è§¦å‘çš„é…ç½®æ›´æ–°ï¼ˆéœ€è¦ `commands.config: true`ï¼‰ã€‚

ç¦ç”¨æ–¹å¼ï¼š

```json5
{
  channels: { imessage: { configWrites: false } },
}
```

## è¦æ±‚

- å·²ç™»å½•"ä¿¡æ¯"çš„ macOSã€‚
- KrabKrab + `imsg` çš„å®Œå…¨ç£ç›˜è®¿é—®æƒé™ï¼ˆè®¿é—®"ä¿¡æ¯"æ•°æ®åº“ï¼‰ã€‚
- å‘é€æ—¶éœ€è¦è‡ªåŠ¨åŒ–æƒé™ã€‚
- `channels.imessage.cliPath` å¯ä»¥æŒ‡å‘ä»»ä½•ä»£ç† stdin/stdout çš„å‘½ä»¤ï¼ˆä¾‹å¦‚ï¼Œé€šè¿‡ SSH è¿žæŽ¥åˆ°å¦ä¸€å° Mac å¹¶è¿è¡Œ `imsg rpc` çš„åŒ…è£…è„šæœ¬ï¼‰ã€‚

## è®¾ç½®ï¼ˆå¿«é€Ÿè·¯å¾„ï¼‰

1. ç¡®ä¿åœ¨æ­¤ Mac ä¸Šå·²ç™»å½•"ä¿¡æ¯"ã€‚
2. é…ç½® iMessage å¹¶å¯åŠ¨ Gateway ç½‘å…³ã€‚

### ä¸“ç”¨æœºå™¨äºº macOS ç”¨æˆ·ï¼ˆç”¨äºŽéš”ç¦»èº«ä»½ï¼‰

å¦‚æžœä½ å¸Œæœ›æœºå™¨äººä»Ž**ç‹¬ç«‹çš„ iMessage èº«ä»½**å‘é€ï¼ˆå¹¶ä¿æŒä½ çš„ä¸ªäºº"ä¿¡æ¯"æ•´æ´ï¼‰ï¼Œè¯·ä½¿ç”¨ä¸“ç”¨ Apple ID + ä¸“ç”¨ macOS ç”¨æˆ·ã€‚

1. åˆ›å»ºä¸“ç”¨ Apple IDï¼ˆä¾‹å¦‚ï¼š`my-cool-bot@icloud.com`ï¼‰ã€‚
   - Apple å¯èƒ½éœ€è¦ç”µè¯å·ç è¿›è¡ŒéªŒè¯ / 2FAã€‚
2. åˆ›å»º macOS ç”¨æˆ·ï¼ˆä¾‹å¦‚ï¼š`krabkrabhome`ï¼‰å¹¶ç™»å½•ã€‚
3. åœ¨è¯¥ macOS ç”¨æˆ·ä¸­æ‰“å¼€"ä¿¡æ¯"å¹¶ä½¿ç”¨æœºå™¨äºº Apple ID ç™»å½• iMessageã€‚
4. å¯ç”¨è¿œç¨‹ç™»å½•ï¼ˆç³»ç»Ÿè®¾ç½® â†’ é€šç”¨ â†’ å…±äº« â†’ è¿œç¨‹ç™»å½•ï¼‰ã€‚
5. å®‰è£… `imsg`ï¼š
   - `brew install steipete/tap/imsg`
6. è®¾ç½® SSH ä½¿ `ssh <bot-macos-user>@localhost true` æ— éœ€å¯†ç å³å¯å·¥ä½œã€‚
7. å°† `channels.imessage.accounts.bot.cliPath` æŒ‡å‘ä»¥æœºå™¨äººç”¨æˆ·èº«ä»½è¿è¡Œ `imsg` çš„ SSH åŒ…è£…è„šæœ¬ã€‚

é¦–æ¬¡è¿è¡Œæ³¨æ„äº‹é¡¹ï¼šå‘é€/æŽ¥æ”¶å¯èƒ½éœ€è¦åœ¨*æœºå™¨äºº macOS ç”¨æˆ·*ä¸­è¿›è¡Œ GUI æ‰¹å‡†ï¼ˆè‡ªåŠ¨åŒ– + å®Œå…¨ç£ç›˜è®¿é—®æƒé™ï¼‰ã€‚å¦‚æžœ `imsg rpc` çœ‹èµ·æ¥å¡ä½æˆ–é€€å‡ºï¼Œè¯·ç™»å½•è¯¥ç”¨æˆ·ï¼ˆå±å¹•å…±äº«å¾ˆæœ‰å¸®åŠ©ï¼‰ï¼Œè¿è¡Œä¸€æ¬¡ `imsg chats --limit 1` / `imsg send ...`ï¼Œæ‰¹å‡†æç¤ºï¼Œç„¶åŽé‡è¯•ã€‚

ç¤ºä¾‹åŒ…è£…è„šæœ¬ï¼ˆ`chmod +x`ï¼‰ã€‚å°† `<bot-macos-user>` æ›¿æ¢ä¸ºä½ çš„å®žé™… macOS ç”¨æˆ·åï¼š

```bash
#!/usr/bin/env bash
set -euo pipefail

# Run an interactive SSH once first to accept host keys:
#   ssh <bot-macos-user>@localhost true
exec /usr/bin/ssh -o BatchMode=yes -o ConnectTimeout=5 -T <bot-macos-user>@localhost \
  "/usr/local/bin/imsg" "$@"
```

ç¤ºä¾‹é…ç½®ï¼š

```json5
{
  channels: {
    imessage: {
      enabled: true,
      accounts: {
        bot: {
          name: "Bot",
          enabled: true,
          cliPath: "/path/to/imsg-bot",
          dbPath: "/Users/<bot-macos-user>/Library/Messages/chat.db",
        },
      },
    },
  },
}
```

å¯¹äºŽå•è´¦æˆ·è®¾ç½®ï¼Œä½¿ç”¨æ‰å¹³é€‰é¡¹ï¼ˆ`channels.imessage.cliPath`ã€`channels.imessage.dbPath`ï¼‰è€Œä¸æ˜¯ `accounts` æ˜ å°„ã€‚

### è¿œç¨‹/SSH å˜ä½“ï¼ˆå¯é€‰ï¼‰

å¦‚æžœä½ æƒ³åœ¨å¦ä¸€å° Mac ä¸Šä½¿ç”¨ iMessageï¼Œè¯·å°† `channels.imessage.cliPath` è®¾ç½®ä¸ºé€šè¿‡ SSH åœ¨è¿œç¨‹ macOS ä¸»æœºä¸Šè¿è¡Œ `imsg` çš„åŒ…è£…è„šæœ¬ã€‚KrabKrab åªéœ€è¦ stdioã€‚

ç¤ºä¾‹åŒ…è£…è„šæœ¬ï¼š

```bash
#!/usr/bin/env bash
exec ssh -T gateway-host imsg "$@"
```

**è¿œç¨‹é™„ä»¶ï¼š** å½“ `cliPath` é€šè¿‡ SSH æŒ‡å‘è¿œç¨‹ä¸»æœºæ—¶ï¼Œ"ä¿¡æ¯"æ•°æ®åº“ä¸­çš„é™„ä»¶è·¯å¾„å¼•ç”¨çš„æ˜¯è¿œç¨‹æœºå™¨ä¸Šçš„æ–‡ä»¶ã€‚KrabKrab å¯ä»¥é€šè¿‡è®¾ç½® `channels.imessage.remoteHost` è‡ªåŠ¨é€šè¿‡ SCP èŽ·å–è¿™äº›æ–‡ä»¶ï¼š

```json5
{
  channels: {
    imessage: {
      cliPath: "~/imsg-ssh", // SSH wrapper to remote Mac
      remoteHost: "user@gateway-host", // for SCP file transfer
      includeAttachments: true,
    },
  },
}
```

å¦‚æžœæœªè®¾ç½® `remoteHost`ï¼ŒKrabKrab ä¼šå°è¯•é€šè¿‡è§£æžåŒ…è£…è„šæœ¬ä¸­çš„ SSH å‘½ä»¤è‡ªåŠ¨æ£€æµ‹ã€‚å»ºè®®æ˜¾å¼é…ç½®ä»¥æé«˜å¯é æ€§ã€‚

#### é€šè¿‡ Tailscale è¿žæŽ¥è¿œç¨‹ Macï¼ˆç¤ºä¾‹ï¼‰

å¦‚æžœ Gateway ç½‘å…³è¿è¡Œåœ¨ Linux ä¸»æœº/è™šæ‹Ÿæœºä¸Šä½† iMessage å¿…é¡»è¿è¡Œåœ¨ Mac ä¸Šï¼ŒTailscale æ˜¯æœ€ç®€å•çš„æ¡¥æŽ¥æ–¹å¼ï¼šGateway ç½‘å…³é€šè¿‡ tailnet ä¸Ž Mac é€šä¿¡ï¼Œé€šè¿‡ SSH è¿è¡Œ `imsg`ï¼Œå¹¶é€šè¿‡ SCP èŽ·å–é™„ä»¶ã€‚

æž¶æž„ï¼š

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”          SSH (imsg rpc)          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Gateway host (Linux/VM)      â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¶â”‚ Mac with Messages + imsg â”‚
â”‚ - krabkrab gateway           â”‚          SCP (attachments)        â”‚ - Messages signed in     â”‚
â”‚ - channels.imessage.cliPath  â”‚â—€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚ - Remote Login enabled   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                   â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
              â–²
              â”‚ Tailscale tailnet (hostname or 100.x.y.z)
              â–¼
        user@gateway-host
```

å…·ä½“é…ç½®ç¤ºä¾‹ï¼ˆTailscale ä¸»æœºåï¼‰ï¼š

```json5
{
  channels: {
    imessage: {
      enabled: true,
      cliPath: "~/.krabkrab/scripts/imsg-ssh",
      remoteHost: "bot@mac-mini.tailnet-1234.ts.net",
      includeAttachments: true,
      dbPath: "/Users/bot/Library/Messages/chat.db",
    },
  },
}
```

ç¤ºä¾‹åŒ…è£…è„šæœ¬ï¼ˆ`~/.krabkrab/scripts/imsg-ssh`ï¼‰ï¼š

```bash
#!/usr/bin/env bash
exec ssh -T bot@mac-mini.tailnet-1234.ts.net imsg "$@"
```

æ³¨æ„äº‹é¡¹ï¼š

- ç¡®ä¿ Mac å·²ç™»å½•"ä¿¡æ¯"ï¼Œå¹¶å·²å¯ç”¨è¿œç¨‹ç™»å½•ã€‚
- ä½¿ç”¨ SSH å¯†é’¥ä½¿ `ssh bot@mac-mini.tailnet-1234.ts.net` æ— éœ€æç¤ºå³å¯å·¥ä½œã€‚
- `remoteHost` åº”ä¸Ž SSH ç›®æ ‡åŒ¹é…ï¼Œä»¥ä¾¿ SCP å¯ä»¥èŽ·å–é™„ä»¶ã€‚

å¤šè´¦æˆ·æ”¯æŒï¼šä½¿ç”¨ `channels.imessage.accounts` é…ç½®æ¯ä¸ªè´¦æˆ·åŠå¯é€‰çš„ `name`ã€‚å‚è§ [`gateway/configuration`](/gateway/configuration#telegramaccounts--discordaccounts--slackaccounts--signalaccounts--imessageaccounts) äº†è§£å…±äº«æ¨¡å¼ã€‚ä¸è¦æäº¤ `~/.krabkrab/krabkrab.json`ï¼ˆå®ƒé€šå¸¸åŒ…å«ä»¤ç‰Œï¼‰ã€‚

## è®¿é—®æŽ§åˆ¶ï¼ˆç§ä¿¡ + ç¾¤ç»„ï¼‰

ç§ä¿¡ï¼š

- é»˜è®¤ï¼š`channels.imessage.dmPolicy = "pairing"`ã€‚
- æœªçŸ¥å‘é€è€…ä¼šæ”¶åˆ°é…å¯¹ç ï¼›æ¶ˆæ¯åœ¨æ‰¹å‡†å‰ä¼šè¢«å¿½ç•¥ï¼ˆé…å¯¹ç åœ¨ 1 å°æ—¶åŽè¿‡æœŸï¼‰ã€‚
- æ‰¹å‡†æ–¹å¼ï¼š
  - `krabkrab pairing list imessage`
  - `krabkrab pairing approve imessage <CODE>`
- é…å¯¹æ˜¯ iMessage ç§ä¿¡çš„é»˜è®¤ä»¤ç‰Œäº¤æ¢æ–¹å¼ã€‚è¯¦æƒ…ï¼š[é…å¯¹](/channels/pairing)

ç¾¤ç»„ï¼š

- `channels.imessage.groupPolicy = open | allowlist | disabled`ã€‚
- è®¾ç½® `allowlist` æ—¶ï¼Œ`channels.imessage.groupAllowFrom` æŽ§åˆ¶è°å¯ä»¥åœ¨ç¾¤ç»„ä¸­è§¦å‘ã€‚
- æåŠæ£€æµ‹ä½¿ç”¨ `agents.list[].groupChat.mentionPatterns`ï¼ˆæˆ– `messages.groupChat.mentionPatterns`ï¼‰ï¼Œå› ä¸º iMessage æ²¡æœ‰åŽŸç”ŸæåŠå…ƒæ•°æ®ã€‚
- å¤šæ™ºèƒ½ä½“è¦†ç›–ï¼šåœ¨ `agents.list[].groupChat.mentionPatterns` ä¸Šè®¾ç½®æ¯ä¸ªæ™ºèƒ½ä½“çš„æ¨¡å¼ã€‚

## å·¥ä½œåŽŸç†ï¼ˆè¡Œä¸ºï¼‰

- `imsg` æµå¼ä¼ è¾“æ¶ˆæ¯äº‹ä»¶ï¼›Gateway ç½‘å…³å°†å®ƒä»¬è§„èŒƒåŒ–ä¸ºå…±äº«æ¸ é“ä¿¡å°ã€‚
- å›žå¤å§‹ç»ˆè·¯ç”±å›žç›¸åŒçš„ chat id æˆ– handleã€‚

## ç±»ç¾¤ç»„ä¼šè¯ï¼ˆ`is_group=false`ï¼‰

æŸäº› iMessage ä¼šè¯å¯èƒ½æœ‰å¤šä¸ªå‚ä¸Žè€…ï¼Œä½†æ ¹æ®"ä¿¡æ¯"å­˜å‚¨èŠå¤©æ ‡è¯†ç¬¦çš„æ–¹å¼ï¼Œä»ä»¥ `is_group=false` åˆ°è¾¾ã€‚

å¦‚æžœä½ åœ¨ `channels.imessage.groups` ä¸‹æ˜¾å¼é…ç½®äº† `chat_id`ï¼ŒKrabKrab ä¼šå°†è¯¥ä¼šè¯è§†ä¸º"ç¾¤ç»„"ç”¨äºŽï¼š

- ä¼šè¯éš”ç¦»ï¼ˆç‹¬ç«‹çš„ `agent:<agentId>:imessage:group:<chat_id>` ä¼šè¯é”®ï¼‰
- ç¾¤ç»„å…è®¸åˆ—è¡¨ / æåŠæ£€æµ‹è¡Œä¸º

ç¤ºä¾‹ï¼š

```json5
{
  channels: {
    imessage: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["+15555550123"],
      groups: {
        "42": { requireMention: false },
      },
    },
  },
}
```

å½“ä½ æƒ³ä¸ºç‰¹å®šä¼šè¯ä½¿ç”¨éš”ç¦»çš„ä¸ªæ€§/æ¨¡åž‹æ—¶è¿™å¾ˆæœ‰ç”¨ï¼ˆå‚è§[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ï¼‰ã€‚å…³äºŽæ–‡ä»¶ç³»ç»Ÿéš”ç¦»ï¼Œå‚è§[æ²™ç®±éš”ç¦»](/gateway/sandboxing)ã€‚

## åª’ä½“ + é™åˆ¶

- é€šè¿‡ `channels.imessage.includeAttachments` å¯é€‰é™„ä»¶æ‘„å–ã€‚
- é€šè¿‡ `channels.imessage.mediaMaxMb` è®¾ç½®åª’ä½“ä¸Šé™ã€‚

## é™åˆ¶

- å‡ºç«™æ–‡æœ¬æŒ‰ `channels.imessage.textChunkLimit` åˆ†å—ï¼ˆé»˜è®¤ 4000ï¼‰ã€‚
- å¯é€‰æ¢è¡Œåˆ†å—ï¼šè®¾ç½® `channels.imessage.chunkMode="newline"` åœ¨é•¿åº¦åˆ†å—å‰æŒ‰ç©ºè¡Œï¼ˆæ®µè½è¾¹ç•Œï¼‰åˆ†å‰²ã€‚
- åª’ä½“ä¸Šä¼ å— `channels.imessage.mediaMaxMb` é™åˆ¶ï¼ˆé»˜è®¤ 16ï¼‰ã€‚

## å¯»å€ / æŠ•é€’ç›®æ ‡

ä¼˜å…ˆä½¿ç”¨ `chat_id` è¿›è¡Œç¨³å®šè·¯ç”±ï¼š

- `chat_id:123`ï¼ˆæŽ¨èï¼‰
- `chat_guid:...`
- `chat_identifier:...`
- ç›´æŽ¥ handleï¼š`imessage:+1555` / `sms:+1555` / `user@example.com`

åˆ—å‡ºèŠå¤©ï¼š

```
imsg chats --limit 20
```

## é…ç½®å‚è€ƒï¼ˆiMessageï¼‰

å®Œæ•´é…ç½®ï¼š[é…ç½®](/gateway/configuration)

æä¾›å•†é€‰é¡¹ï¼š

- `channels.imessage.enabled`ï¼šå¯ç”¨/ç¦ç”¨æ¸ é“å¯åŠ¨ã€‚
- `channels.imessage.cliPath`ï¼š`imsg` è·¯å¾„ã€‚
- `channels.imessage.dbPath`ï¼š"ä¿¡æ¯"æ•°æ®åº“è·¯å¾„ã€‚
- `channels.imessage.remoteHost`ï¼šå½“ `cliPath` æŒ‡å‘è¿œç¨‹ Mac æ—¶ç”¨äºŽ SCP é™„ä»¶ä¼ è¾“çš„ SSH ä¸»æœºï¼ˆä¾‹å¦‚ `user@gateway-host`ï¼‰ã€‚å¦‚æœªè®¾ç½®åˆ™ä»Ž SSH åŒ…è£…è„šæœ¬è‡ªåŠ¨æ£€æµ‹ã€‚
- `channels.imessage.service`ï¼š`imessage | sms | auto`ã€‚
- `channels.imessage.region`ï¼šçŸ­ä¿¡åŒºåŸŸã€‚
- `channels.imessage.dmPolicy`ï¼š`pairing | allowlist | open | disabled`ï¼ˆé»˜è®¤ï¼špairingï¼‰ã€‚
- `channels.imessage.allowFrom`ï¼šç§ä¿¡å…è®¸åˆ—è¡¨ï¼ˆhandleã€é‚®ç®±ã€E.164 å·ç æˆ– `chat_id:*`ï¼‰ã€‚`open` éœ€è¦ `"*"`ã€‚iMessage æ²¡æœ‰ç”¨æˆ·åï¼›ä½¿ç”¨ handle æˆ–èŠå¤©ç›®æ ‡ã€‚
- `channels.imessage.groupPolicy`ï¼š`open | allowlist | disabled`ï¼ˆé»˜è®¤ï¼šallowlistï¼‰ã€‚
- `channels.imessage.groupAllowFrom`ï¼šç¾¤ç»„å‘é€è€…å…è®¸åˆ—è¡¨ã€‚
- `channels.imessage.historyLimit` / `channels.imessage.accounts.*.historyLimit`ï¼šä½œä¸ºä¸Šä¸‹æ–‡åŒ…å«çš„æœ€å¤§ç¾¤ç»„æ¶ˆæ¯æ•°ï¼ˆ0 ç¦ç”¨ï¼‰ã€‚
- `channels.imessage.dmHistoryLimit`ï¼šç§ä¿¡åŽ†å²é™åˆ¶ï¼ˆç”¨æˆ·è½®æ¬¡ï¼‰ã€‚æ¯ç”¨æˆ·è¦†ç›–ï¼š`channels.imessage.dms["<handle>"].historyLimit`ã€‚
- `channels.imessage.groups`ï¼šæ¯ç¾¤ç»„é»˜è®¤å€¼ + å…è®¸åˆ—è¡¨ï¼ˆä½¿ç”¨ `"*"` ä½œä¸ºå…¨å±€é»˜è®¤å€¼ï¼‰ã€‚
- `channels.imessage.includeAttachments`ï¼šå°†é™„ä»¶æ‘„å–åˆ°ä¸Šä¸‹æ–‡ã€‚
- `channels.imessage.mediaMaxMb`ï¼šå…¥ç«™/å‡ºç«™åª’ä½“ä¸Šé™ï¼ˆMBï¼‰ã€‚
- `channels.imessage.textChunkLimit`ï¼šå‡ºç«™åˆ†å—å¤§å°ï¼ˆå­—ç¬¦ï¼‰ã€‚
- `channels.imessage.chunkMode`ï¼š`length`ï¼ˆé»˜è®¤ï¼‰æˆ– `newline` åœ¨é•¿åº¦åˆ†å—å‰æŒ‰ç©ºè¡Œï¼ˆæ®µè½è¾¹ç•Œï¼‰åˆ†å‰²ã€‚

ç›¸å…³å…¨å±€é€‰é¡¹ï¼š

- `agents.list[].groupChat.mentionPatterns`ï¼ˆæˆ– `messages.groupChat.mentionPatterns`ï¼‰ã€‚
- `messages.responsePrefix`ã€‚

