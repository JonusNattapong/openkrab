---
read_when: Connecting the macOS app to a remote gateway over SSH
summary: KrabKrab.app è¿žæŽ¥è¿œç¨‹ Gateway ç½‘å…³çš„ SSH éš§é“è®¾ç½®
title: è¿œç¨‹ Gateway ç½‘å…³è®¾ç½®
x-i18n:
  generated_at: "2026-02-03T07:48:37Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: b1ae266a7cb4911b82ae3ec6cb98b1b57aca592aeb1dc8b74bbce9b0ea9dd1d1
  source_path: gateway/remote-gateway-readme.md
  workflow: 15
---

# ä½¿ç”¨è¿œç¨‹ Gateway ç½‘å…³è¿è¡Œ KrabKrab.app

KrabKrab.app ä½¿ç”¨ SSH éš§é“è¿žæŽ¥åˆ°è¿œç¨‹ Gateway ç½‘å…³ã€‚æœ¬æŒ‡å—å‘ä½ å±•ç¤ºå¦‚ä½•è®¾ç½®ã€‚

## æ¦‚è¿°

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        Client Machine                          â”‚
â”‚                                                              â”‚
â”‚  KrabKrab.app â”€â”€â–º ws://127.0.0.1:18789 (local port)           â”‚
â”‚                     â”‚                                        â”‚
â”‚                     â–¼                                        â”‚
â”‚  SSH Tunnel â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”‚
â”‚                     â”‚                                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                      â”‚
                      â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                         Remote Machine                        â”‚
â”‚                                                              â”‚
â”‚  Gateway WebSocket â”€â”€â–º ws://127.0.0.1:18789 â”€â”€â–º              â”‚
â”‚                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

## å¿«é€Ÿè®¾ç½®

### æ­¥éª¤ 1ï¼šæ·»åŠ  SSH é…ç½®

ç¼–è¾‘ `~/.ssh/config` å¹¶æ·»åŠ ï¼š

```ssh
Host remote-gateway
    HostName <REMOTE_IP>          # e.g., 172.27.187.184
    User <REMOTE_USER>            # e.g., jefferson
    LocalForward 18789 127.0.0.1:18789
    IdentityFile ~/.ssh/id_rsa
```

å°† `<REMOTE_IP>` å’Œ `<REMOTE_USER>` æ›¿æ¢ä¸ºä½ çš„å€¼ã€‚

### æ­¥éª¤ 2ï¼šå¤åˆ¶ SSH å¯†é’¥

å°†ä½ çš„å…¬é’¥å¤åˆ¶åˆ°è¿œç¨‹æœºå™¨ï¼ˆè¾“å…¥ä¸€æ¬¡å¯†ç ï¼‰ï¼š

```bash
ssh-copy-id -i ~/.ssh/id_rsa <REMOTE_USER>@<REMOTE_IP>
```

### æ­¥éª¤ 3ï¼šè®¾ç½® Gateway ç½‘å…³ä»¤ç‰Œ

```bash
launchctl setenv krabkrab_GATEWAY_TOKEN "<your-token>"
```

### æ­¥éª¤ 4ï¼šå¯åŠ¨ SSH éš§é“

```bash
ssh -N remote-gateway &
```

### æ­¥éª¤ 5ï¼šé‡å¯ KrabKrab.app

```bash
# Quit KrabKrab.app (âŒ˜Q), then reopen:
open /path/to/KrabKrab.app
```

åº”ç”¨çŽ°åœ¨å°†é€šè¿‡ SSH éš§é“è¿žæŽ¥åˆ°è¿œç¨‹ Gateway ç½‘å…³ã€‚

---

## ç™»å½•æ—¶è‡ªåŠ¨å¯åŠ¨éš§é“

è¦åœ¨ç™»å½•æ—¶è‡ªåŠ¨å¯åŠ¨ SSH éš§é“ï¼Œè¯·åˆ›å»ºä¸€ä¸ª Launch Agentã€‚

### åˆ›å»º PLIST æ–‡ä»¶

å°†æ­¤ä¿å­˜ä¸º `~/Library/LaunchAgents/bot.molt.ssh-tunnel.plist`ï¼š

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>bot.molt.ssh-tunnel</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/ssh</string>
        <string>-N</string>
        <string>remote-gateway</string>
    </array>
    <key>KeepAlive</key>
    <true/>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
```

### åŠ è½½ Launch Agent

```bash
launchctl bootstrap gui/$UID ~/Library/LaunchAgents/bot.molt.ssh-tunnel.plist
```

éš§é“çŽ°åœ¨å°†ï¼š

- ç™»å½•æ—¶è‡ªåŠ¨å¯åŠ¨
- å´©æºƒæ—¶é‡æ–°å¯åŠ¨
- åœ¨åŽå°æŒç»­è¿è¡Œ

æ—§ç‰ˆæ³¨æ„äº‹é¡¹ï¼šå¦‚æžœå­˜åœ¨ä»»ä½•é—ç•™çš„ `com.krabkrab.ssh-tunnel` LaunchAgentï¼Œè¯·å°†å…¶åˆ é™¤ã€‚

---

## æ•…éšœæŽ’é™¤

**æ£€æŸ¥éš§é“æ˜¯å¦æ­£åœ¨è¿è¡Œï¼š**

```bash
ps aux | grep "ssh -N remote-gateway" | grep -v grep
lsof -i :18789
```

**é‡å¯éš§é“ï¼š**

```bash
launchctl kickstart -k gui/$UID/bot.molt.ssh-tunnel
```

**åœæ­¢éš§é“ï¼š**

```bash
launchctl bootout gui/$UID/bot.molt.ssh-tunnel
```

---

## å·¥ä½œåŽŸç†

| ç»„ä»¶                                 | åŠŸèƒ½                                  |
| ------------------------------------ | ------------------------------------- |
| `LocalForward 18789 127.0.0.1:18789` | å°†æœ¬åœ°ç«¯å£ 18789 è½¬å‘åˆ°è¿œç¨‹ç«¯å£ 18789 |
| `ssh -N`                             | SSH ä¸æ‰§è¡Œè¿œç¨‹å‘½ä»¤ï¼ˆä»…ç«¯å£è½¬å‘ï¼‰      |
| `KeepAlive`                          | éš§é“å´©æºƒæ—¶è‡ªåŠ¨é‡å¯                    |
| `RunAtLoad`                          | ä»£ç†åŠ è½½æ—¶å¯åŠ¨éš§é“                    |

KrabKrab.app è¿žæŽ¥åˆ°ä½ çš„å®¢æˆ·ç«¯æœºå™¨ä¸Šçš„ `ws://127.0.0.1:18789`ã€‚SSH éš§é“å°†è¯¥è¿žæŽ¥è½¬å‘åˆ°è¿è¡Œ Gateway ç½‘å…³çš„è¿œç¨‹æœºå™¨çš„ç«¯å£ 18789ã€‚

