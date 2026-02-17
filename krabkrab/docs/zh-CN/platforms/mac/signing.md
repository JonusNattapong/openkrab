---
read_when:
  - æž„å»ºæˆ–ç­¾å Mac è°ƒè¯•æž„å»º
summary: æ‰“åŒ…è„šæœ¬ç”Ÿæˆçš„ macOS è°ƒè¯•æž„å»ºçš„ç­¾åæ­¥éª¤
title: macOS ç­¾å
x-i18n:
  generated_at: "2026-02-01T21:33:15Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 403b92f9a0ecdb7cb42ec097c684b7a696be3696d6eece747314a4dc90d8797e
  source_path: platforms/mac/signing.md
  workflow: 15
---

# Mac ç­¾åï¼ˆè°ƒè¯•æž„å»ºï¼‰

æ­¤åº”ç”¨é€šå¸¸ä»Ž [`scripts/package-mac-app.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/package-mac-app.sh) æž„å»ºï¼Œè¯¥è„šæœ¬ç›®å‰ä¼šï¼š

- è®¾ç½®ç¨³å®šçš„è°ƒè¯• Bundle æ ‡è¯†ç¬¦ï¼š`ai.krabkrab.mac.debug`
- ä½¿ç”¨è¯¥ Bundle ID å†™å…¥ Info.plistï¼ˆå¯é€šè¿‡ `BUNDLE_ID=...` è¦†ç›–ï¼‰
- è°ƒç”¨ [`scripts/codesign-mac-app.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/codesign-mac-app.sh) å¯¹ä¸»äºŒè¿›åˆ¶æ–‡ä»¶å’Œåº”ç”¨åŒ…è¿›è¡Œç­¾åï¼Œä½¿ macOS å°†æ¯æ¬¡é‡æ–°æž„å»ºè§†ä¸ºç›¸åŒçš„å·²ç­¾ååŒ…ï¼Œå¹¶ä¿ç•™ TCC æƒé™ï¼ˆé€šçŸ¥ã€è¾…åŠ©åŠŸèƒ½ã€å±å¹•å½•åˆ¶ã€éº¦å…‹é£Žã€è¯­éŸ³ï¼‰ã€‚è¦èŽ·å¾—ç¨³å®šçš„æƒé™ï¼Œè¯·ä½¿ç”¨çœŸå®žç­¾åèº«ä»½ï¼›ä¸´æ—¶ç­¾åæ˜¯å¯é€‰çš„ä¸”ä¸ç¨³å®šï¼ˆå‚é˜… [macOS æƒé™](/platforms/mac/permissions)ï¼‰ã€‚
- é»˜è®¤ä½¿ç”¨ `CODESIGN_TIMESTAMP=auto`ï¼›ä¸º Developer ID ç­¾åå¯ç”¨å—ä¿¡ä»»çš„æ—¶é—´æˆ³ã€‚è®¾ç½® `CODESIGN_TIMESTAMP=off` å¯è·³è¿‡æ—¶é—´æˆ³ï¼ˆç¦»çº¿è°ƒè¯•æž„å»ºï¼‰ã€‚
- å°†æž„å»ºå…ƒæ•°æ®æ³¨å…¥ Info.plistï¼š`krabkrabBuildTimestamp`ï¼ˆUTCï¼‰å’Œ `krabkrabGitCommit`ï¼ˆçŸ­å“ˆå¸Œï¼‰ï¼Œä»¥ä¾¿"å…³äºŽ"é¢æ¿å¯ä»¥æ˜¾ç¤ºæž„å»ºä¿¡æ¯ã€git ä¿¡æ¯å’Œè°ƒè¯•/å‘å¸ƒæ¸ é“ã€‚
- **æ‰“åŒ…éœ€è¦ Node 22+**ï¼šè„šæœ¬ä¼šè¿è¡Œ TS æž„å»ºå’Œ Control UI æž„å»ºã€‚
- ä»ŽçŽ¯å¢ƒå˜é‡ä¸­è¯»å– `SIGN_IDENTITY`ã€‚å°† `export SIGN_IDENTITY="Apple Development: Your Name (TEAMID)"`ï¼ˆæˆ–ä½ çš„ Developer ID Application è¯ä¹¦ï¼‰æ·»åŠ åˆ° shell é…ç½®æ–‡ä»¶ä¸­ï¼Œä»¥å§‹ç»ˆä½¿ç”¨ä½ çš„è¯ä¹¦ç­¾åã€‚ä¸´æ—¶ç­¾åéœ€è¦é€šè¿‡ `ALLOW_ADHOC_SIGNING=1` æˆ– `SIGN_IDENTITY="-"` æ˜¾å¼å¯ç”¨ï¼ˆä¸å»ºè®®ç”¨äºŽæƒé™æµ‹è¯•ï¼‰ã€‚
- ç­¾ååŽè¿è¡Œ Team ID å®¡è®¡ï¼Œå¦‚æžœåº”ç”¨åŒ…å†…çš„ä»»ä½• Mach-O æ–‡ä»¶ç”±ä¸åŒçš„ Team ID ç­¾ååˆ™ä¼šå¤±è´¥ã€‚è®¾ç½® `SKIP_TEAM_ID_CHECK=1` å¯è·³è¿‡æ­¤æ£€æŸ¥ã€‚

## ç”¨æ³•

```bash
# ä»Žä»“åº“æ ¹ç›®å½•
scripts/package-mac-app.sh               # è‡ªåŠ¨é€‰æ‹©èº«ä»½ï¼›æœªæ‰¾åˆ°æ—¶æŠ¥é”™
SIGN_IDENTITY="Developer ID Application: Your Name" scripts/package-mac-app.sh   # çœŸå®žè¯ä¹¦
ALLOW_ADHOC_SIGNING=1 scripts/package-mac-app.sh    # ä¸´æ—¶ç­¾åï¼ˆæƒé™ä¸ä¼šæŒä¹…åŒ–ï¼‰
SIGN_IDENTITY="-" scripts/package-mac-app.sh        # æ˜¾å¼ä¸´æ—¶ç­¾åï¼ˆåŒæ ·çš„é™åˆ¶ï¼‰
DISABLE_LIBRARY_VALIDATION=1 scripts/package-mac-app.sh   # ä»…é™å¼€å‘çš„ Sparkle Team ID ä¸åŒ¹é…è§£å†³æ–¹æ¡ˆ
```

### ä¸´æ—¶ç­¾åæ³¨æ„äº‹é¡¹

ä½¿ç”¨ `SIGN_IDENTITY="-"`ï¼ˆä¸´æ—¶ç­¾åï¼‰ç­¾åæ—¶ï¼Œè„šæœ¬ä¼šè‡ªåŠ¨ç¦ç”¨**å¼ºåŒ–è¿è¡Œæ—¶**ï¼ˆ`--options runtime`ï¼‰ã€‚è¿™æ˜¯ä¸ºäº†é˜²æ­¢åº”ç”¨åœ¨å°è¯•åŠ è½½ä¸å…±äº«ç›¸åŒ Team ID çš„åµŒå…¥å¼æ¡†æž¶ï¼ˆå¦‚ Sparkleï¼‰æ—¶å´©æºƒã€‚ä¸´æ—¶ç­¾åè¿˜ä¼šç ´å TCC æƒé™æŒä¹…åŒ–ï¼›å‚é˜… [macOS æƒé™](/platforms/mac/permissions) äº†è§£æ¢å¤æ­¥éª¤ã€‚

## å…³äºŽé¢æ¿çš„æž„å»ºå…ƒæ•°æ®

`package-mac-app.sh` ä¼šåœ¨åŒ…ä¸­æ ‡è®°ä»¥ä¸‹ä¿¡æ¯ï¼š

- `krabkrabBuildTimestamp`ï¼šæ‰“åŒ…æ—¶çš„ ISO8601 UTC æ—¶é—´
- `krabkrabGitCommit`ï¼šçŸ­ git å“ˆå¸Œï¼ˆä¸å¯ç”¨æ—¶ä¸º `unknown`ï¼‰

"å…³äºŽ"é€‰é¡¹å¡è¯»å–è¿™äº›é”®ä»¥æ˜¾ç¤ºç‰ˆæœ¬ã€æž„å»ºæ—¥æœŸã€git æäº¤ä»¥åŠæ˜¯å¦ä¸ºè°ƒè¯•æž„å»ºï¼ˆé€šè¿‡ `#if DEBUG`ï¼‰ã€‚ä»£ç æ›´æ”¹åŽè¿è¡Œæ‰“åŒ…ç¨‹åºä»¥åˆ·æ–°è¿™äº›å€¼ã€‚

## åŽŸå› 

TCC æƒé™ä¸Ž Bundle æ ‡è¯†ç¬¦*å’Œ*ä»£ç ç­¾åç»‘å®šã€‚ä½¿ç”¨ä¸æ–­å˜åŒ–çš„ UUID çš„æœªç­¾åè°ƒè¯•æž„å»ºä¼šå¯¼è‡´ macOS åœ¨æ¯æ¬¡é‡æ–°æž„å»ºåŽå¿˜è®°æŽˆæƒã€‚å¯¹äºŒè¿›åˆ¶æ–‡ä»¶è¿›è¡Œç­¾åï¼ˆé»˜è®¤ä¸´æ—¶ç­¾åï¼‰å¹¶ä¿æŒå›ºå®šçš„ Bundle ID/è·¯å¾„ï¼ˆ`dist/KrabKrab.app`ï¼‰å¯ä»¥åœ¨æž„å»ºä¹‹é—´ä¿ç•™æŽˆæƒï¼Œä¸Ž VibeTunnel çš„æ–¹æ¡ˆä¸€è‡´ã€‚

