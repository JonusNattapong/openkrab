---
read_when:
  - ä½ æƒ³è¦å¯å¤çŽ°ã€å¯å›žæ»šçš„å®‰è£…
  - ä½ å·²ç»åœ¨ä½¿ç”¨ Nix/NixOS/Home Manager
  - ä½ æƒ³è¦æ‰€æœ‰å†…å®¹éƒ½å›ºå®šå¹¶ä»¥å£°æ˜Žå¼ç®¡ç†
summary: ä½¿ç”¨ Nix å£°æ˜Žå¼å®‰è£… KrabKrab
title: Nix
x-i18n:
  generated_at: "2026-02-03T07:49:51Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: f1452194cfdd74613b5b3ab90b0d506eaea2d16b147497987710d6ad658312ba
  source_path: install/nix.md
  workflow: 15
---

# Nix å®‰è£…

ä½¿ç”¨ Nix è¿è¡Œ KrabKrab çš„æŽ¨èæ–¹å¼æ˜¯é€šè¿‡ **[nix-krabkrab](https://github.com/krabkrab/nix-krabkrab)** â€” ä¸€ä¸ªå¼€ç®±å³ç”¨çš„ Home Manager æ¨¡å—ã€‚

## å¿«é€Ÿå¼€å§‹

å°†æ­¤ç²˜è´´ç»™ä½ çš„ AI æ™ºèƒ½ä½“ï¼ˆClaudeã€Cursor ç­‰ï¼‰ï¼š

```text
I want to set up nix-krabkrab on my Mac.
Repository: github:krabkrab/nix-krabkrab

What I need you to do:
1. Check if Determinate Nix is installed (if not, install it)
2. Create a local flake at ~/code/krabkrab-local using templates/agent-first/flake.nix
3. Help me create a Telegram bot (@BotFather) and get my chat ID (@userinfobot)
4. Set up secrets (bot token, Anthropic key) - plain files at ~/.secrets/ is fine
5. Fill in the template placeholders and run home-manager switch
6. Verify: launchd running, bot responds to messages

Reference the nix-krabkrab README for module options.
```

> **ðŸ“¦ å®Œæ•´æŒ‡å—ï¼š[github.com/krabkrab/nix-krabkrab](https://github.com/krabkrab/nix-krabkrab)**
>
> nix-krabkrab ä»“åº“æ˜¯ Nix å®‰è£…çš„æƒå¨æ¥æºã€‚æœ¬é¡µåªæ˜¯ä¸€ä¸ªå¿«é€Ÿæ¦‚è¿°ã€‚

## ä½ å°†èŽ·å¾—

- Gateway ç½‘å…³ + macOS åº”ç”¨ + å·¥å…·ï¼ˆwhisperã€spotifyã€camerasï¼‰â€” å…¨éƒ¨å›ºå®šç‰ˆæœ¬
- é‡å¯åŽä»èƒ½è¿è¡Œçš„ Launchd æœåŠ¡
- å¸¦æœ‰å£°æ˜Žå¼é…ç½®çš„æ’ä»¶ç³»ç»Ÿ
- å³æ—¶å›žæ»šï¼š`home-manager switch --rollback`

---

## Nix æ¨¡å¼è¿è¡Œæ—¶è¡Œä¸º

å½“è®¾ç½® `krabkrab_NIX_MODE=1` æ—¶ï¼ˆnix-krabkrab ä¼šè‡ªåŠ¨è®¾ç½®ï¼‰ï¼š

KrabKrab æ”¯æŒ **Nix æ¨¡å¼**ï¼Œä½¿é…ç½®ç¡®å®šæ€§å¹¶ç¦ç”¨è‡ªåŠ¨å®‰è£…æµç¨‹ã€‚
é€šè¿‡å¯¼å‡ºä»¥ä¸‹çŽ¯å¢ƒå˜é‡å¯ç”¨ï¼š

```bash
krabkrab_NIX_MODE=1
```

åœ¨ macOS ä¸Šï¼ŒGUI åº”ç”¨ä¸ä¼šè‡ªåŠ¨ç»§æ‰¿ shell çŽ¯å¢ƒå˜é‡ã€‚ä½ ä¹Ÿå¯ä»¥é€šè¿‡ defaults å¯ç”¨ Nix æ¨¡å¼ï¼š

```bash
defaults write bot.molt.mac krabkrab.nixMode -bool true
```

### é…ç½® + çŠ¶æ€è·¯å¾„

KrabKrab ä»Ž `krabkrab_CONFIG_PATH` è¯»å– JSON5 é…ç½®ï¼Œå¹¶å°†å¯å˜æ•°æ®å­˜å‚¨åœ¨ `krabkrab_STATE_DIR` ä¸­ã€‚

- `krabkrab_STATE_DIR`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab`ï¼‰
- `krabkrab_CONFIG_PATH`ï¼ˆé»˜è®¤ï¼š`$krabkrab_STATE_DIR/krabkrab.json`ï¼‰

åœ¨ Nix ä¸‹è¿è¡Œæ—¶ï¼Œå°†è¿™äº›æ˜¾å¼è®¾ç½®ä¸º Nix ç®¡ç†çš„ä½ç½®ï¼Œä»¥ä¾¿è¿è¡Œæ—¶çŠ¶æ€å’Œé…ç½®ä¸ä¼šè¿›å…¥ä¸å¯å˜å­˜å‚¨ã€‚

### Nix æ¨¡å¼ä¸‹çš„è¿è¡Œæ—¶è¡Œä¸º

- è‡ªåŠ¨å®‰è£…å’Œè‡ªæˆ‘ä¿®æ”¹æµç¨‹è¢«ç¦ç”¨
- ç¼ºå¤±çš„ä¾èµ–ä¼šæ˜¾ç¤º Nix ç‰¹å®šçš„ä¿®å¤æ¶ˆæ¯
- å­˜åœ¨æ—¶ UI ä¼šæ˜¾ç¤ºåªè¯» Nix æ¨¡å¼æ¨ªå¹…

## æ‰“åŒ…æ³¨æ„äº‹é¡¹ï¼ˆmacOSï¼‰

macOS æ‰“åŒ…æµç¨‹æœŸæœ›åœ¨ä»¥ä¸‹ä½ç½®æœ‰ä¸€ä¸ªç¨³å®šçš„ Info.plist æ¨¡æ¿ï¼š

```
apps/macos/Sources/KrabKrab/Resources/Info.plist
```

[`scripts/package-mac-app.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/package-mac-app.sh) å°†æ­¤æ¨¡æ¿å¤åˆ¶åˆ°åº”ç”¨åŒ…ä¸­å¹¶ä¿®è¡¥åŠ¨æ€å­—æ®µï¼ˆbundle IDã€ç‰ˆæœ¬/æž„å»ºå·ã€Git SHAã€Sparkle å¯†é’¥ï¼‰ã€‚è¿™ä½¿ plist å¯¹äºŽ SwiftPM æ‰“åŒ…å’Œ Nix æž„å»ºä¿æŒç¡®å®šæ€§ï¼ˆå®ƒä»¬ä¸ä¾èµ–å®Œæ•´çš„ Xcode å·¥å…·é“¾ï¼‰ã€‚

## ç›¸å…³å†…å®¹

- [nix-krabkrab](https://github.com/krabkrab/nix-krabkrab) â€” å®Œæ•´è®¾ç½®æŒ‡å—
- [å‘å¯¼](/start/wizard) â€” éž Nix CLI è®¾ç½®
- [Docker](/install/docker) â€” å®¹å™¨åŒ–è®¾ç½®

