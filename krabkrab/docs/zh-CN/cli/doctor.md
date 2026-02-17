---
read_when:
  - ä½ é‡åˆ°è¿žæŽ¥/è®¤è¯é—®é¢˜ï¼Œéœ€è¦å¼•å¯¼å¼ä¿®å¤
  - ä½ æ›´æ–°åŽæƒ³è¿›è¡Œå®Œæ•´æ€§æ£€æŸ¥
summary: "`krabkrab doctor` çš„ CLI å‚è€ƒï¼ˆå¥åº·æ£€æŸ¥ + å¼•å¯¼å¼ä¿®å¤ï¼‰"
title: doctor
x-i18n:
  generated_at: "2026-02-03T10:04:15Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 92310aa3f3d111e91a74ce1150359d5d8a8d70a856666d9419e16c60d78209f2
  source_path: cli/doctor.md
  workflow: 15
---

# `krabkrab doctor`

Gateway ç½‘å…³å’Œæ¸ é“çš„å¥åº·æ£€æŸ¥ + å¿«é€Ÿä¿®å¤ã€‚

ç›¸å…³å†…å®¹ï¼š

- æ•…éšœæŽ’é™¤ï¼š[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)
- å®‰å…¨å®¡è®¡ï¼š[å®‰å…¨](/gateway/security)

## ç¤ºä¾‹

```bash
krabkrab doctor
krabkrab doctor --repair
krabkrab doctor --deep
```

æ³¨æ„äº‹é¡¹ï¼š

- äº¤äº’å¼æç¤ºï¼ˆå¦‚é’¥åŒ™ä¸²/OAuth ä¿®å¤ï¼‰ä»…åœ¨ stdin æ˜¯ TTY ä¸”**æœª**è®¾ç½® `--non-interactive` æ—¶è¿è¡Œã€‚æ— å¤´è¿è¡Œï¼ˆcronã€Telegramã€æ— ç»ˆç«¯ï¼‰å°†è·³è¿‡æç¤ºã€‚
- `--fix`ï¼ˆ`--repair` çš„åˆ«åï¼‰ä¼šå°†å¤‡ä»½å†™å…¥ `~/.krabkrab/krabkrab.json.bak`ï¼Œå¹¶åˆ é™¤æœªçŸ¥çš„é…ç½®é”®ï¼ŒåŒæ—¶åˆ—å‡ºæ¯ä¸ªåˆ é™¤é¡¹ã€‚

## macOSï¼š`launchctl` çŽ¯å¢ƒå˜é‡è¦†ç›–

å¦‚æžœä½ ä¹‹å‰è¿è¡Œè¿‡ `launchctl setenv krabkrab_GATEWAY_TOKEN ...`ï¼ˆæˆ– `...PASSWORD`ï¼‰ï¼Œè¯¥å€¼ä¼šè¦†ç›–ä½ çš„é…ç½®æ–‡ä»¶ï¼Œå¹¶å¯èƒ½å¯¼è‡´æŒç»­çš„"æœªæŽˆæƒ"é”™è¯¯ã€‚

```bash
launchctl getenv krabkrab_GATEWAY_TOKEN
launchctl getenv krabkrab_GATEWAY_PASSWORD

launchctl unsetenv krabkrab_GATEWAY_TOKEN
launchctl unsetenv krabkrab_GATEWAY_PASSWORD
```

