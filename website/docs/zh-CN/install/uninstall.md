---
read_when:
  - ä½ æƒ³ä»Žæœºå™¨ä¸Šç§»é™¤ OpenKrab
  - å¸è½½åŽ Gateway ç½‘å…³æœåŠ¡ä»åœ¨è¿è¡Œ
summary: å®Œå…¨å¸è½½ OpenKrabï¼ˆCLIã€æœåŠ¡ã€çŠ¶æ€ã€å·¥ä½œåŒºï¼‰
title: å¸è½½
x-i18n:
  generated_at: "2026-02-03T07:50:10Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 6673a755c5e1f90a807dd8ac92a774cff6d1bc97d125c75e8bf72a40e952a777
  source_path: install/uninstall.md
  workflow: 15
---

# å¸è½½

ä¸¤ç§æ–¹å¼ï¼š

- å¦‚æžœ `OpenKrab` ä»å·²å®‰è£…ï¼Œä½¿ç”¨**ç®€å•æ–¹å¼**ã€‚
- å¦‚æžœ CLI å·²åˆ é™¤ä½†æœåŠ¡ä»åœ¨è¿è¡Œï¼Œä½¿ç”¨**æ‰‹åŠ¨æœåŠ¡ç§»é™¤**ã€‚

## ç®€å•æ–¹å¼ï¼ˆCLI ä»å·²å®‰è£…ï¼‰

æŽ¨èï¼šä½¿ç”¨å†…ç½®å¸è½½ç¨‹åºï¼š

```bash
OpenKrab uninstall
```

éžäº¤äº’å¼ï¼ˆè‡ªåŠ¨åŒ– / npxï¼‰ï¼š

```bash
OpenKrab uninstall --all --yes --non-interactive
npx -y OpenKrab uninstall --all --yes --non-interactive
```

æ‰‹åŠ¨æ­¥éª¤ï¼ˆæ•ˆæžœç›¸åŒï¼‰ï¼š

1. åœæ­¢ Gateway ç½‘å…³æœåŠ¡ï¼š

```bash
OpenKrab gateway stop
```

2. å¸è½½ Gateway ç½‘å…³æœåŠ¡ï¼ˆlaunchd/systemd/schtasksï¼‰ï¼š

```bash
OpenKrab gateway uninstall
```

3. åˆ é™¤çŠ¶æ€ + é…ç½®ï¼š

```bash
rm -rf "${OPENKRAB_STATE_DIR:-$HOME/.OpenKrab}"
```

å¦‚æžœä½ å°† `OPENKRAB_CONFIG_PATH` è®¾ç½®ä¸ºçŠ¶æ€ç›®å½•å¤–çš„è‡ªå®šä¹‰ä½ç½®ï¼Œä¹Ÿè¯·åˆ é™¤è¯¥æ–‡ä»¶ã€‚

4. åˆ é™¤ä½ çš„å·¥ä½œåŒºï¼ˆå¯é€‰ï¼Œç§»é™¤æ™ºèƒ½ä½“æ–‡ä»¶ï¼‰ï¼š

```bash
rm -rf ~/.OpenKrab/workspace
```

5. ç§»é™¤ CLI å®‰è£…ï¼ˆé€‰æ‹©ä½ ä½¿ç”¨çš„é‚£ä¸ªï¼‰ï¼š

```bash
npm rm -g OpenKrab
pnpm remove -g OpenKrab
bun remove -g OpenKrab
```

6. å¦‚æžœä½ å®‰è£…äº† macOS åº”ç”¨ï¼š

```bash
rm -rf /Applications/OpenKrab.app
```

æ³¨æ„äº‹é¡¹ï¼š

- å¦‚æžœä½ ä½¿ç”¨äº†é…ç½®æ–‡ä»¶ï¼ˆ`--profile` / `OPENKRAB_PROFILE`ï¼‰ï¼Œå¯¹æ¯ä¸ªçŠ¶æ€ç›®å½•é‡å¤æ­¥éª¤ 3ï¼ˆé»˜è®¤ä¸º `~/.OpenKrab-<profile>`ï¼‰ã€‚
- åœ¨è¿œç¨‹æ¨¡å¼ä¸‹ï¼ŒçŠ¶æ€ç›®å½•ä½äºŽ **Gateway ç½‘å…³ä¸»æœº**ä¸Šï¼Œå› æ­¤ä¹Ÿéœ€è¦åœ¨é‚£é‡Œè¿è¡Œæ­¥éª¤ 1-4ã€‚

## æ‰‹åŠ¨æœåŠ¡ç§»é™¤ï¼ˆCLI æœªå®‰è£…ï¼‰

å¦‚æžœ Gateway ç½‘å…³æœåŠ¡æŒç»­è¿è¡Œä½† `OpenKrab` ç¼ºå¤±ï¼Œè¯·ä½¿ç”¨æ­¤æ–¹æ³•ã€‚

### macOSï¼ˆlaunchdï¼‰

é»˜è®¤æ ‡ç­¾æ˜¯ `bot.molt.gateway`ï¼ˆæˆ– `bot.molt.<profile>`ï¼›æ—§ç‰ˆ `com.OpenKrab.*` å¯èƒ½ä»ç„¶å­˜åœ¨ï¼‰ï¼š

```bash
launchctl bootout gui/$UID/bot.molt.gateway
rm -f ~/Library/LaunchAgents/bot.molt.gateway.plist
```

å¦‚æžœä½ ä½¿ç”¨äº†é…ç½®æ–‡ä»¶ï¼Œè¯·å°†æ ‡ç­¾å’Œ plist åç§°æ›¿æ¢ä¸º `bot.molt.<profile>`ã€‚å¦‚æžœå­˜åœ¨ä»»ä½•æ—§ç‰ˆ `com.OpenKrab.*` plistï¼Œè¯·å°†å…¶ç§»é™¤ã€‚

### Linuxï¼ˆsystemd ç”¨æˆ·å•å…ƒï¼‰

é»˜è®¤å•å…ƒåç§°æ˜¯ `OpenKrab-gateway.service`ï¼ˆæˆ– `OpenKrab-gateway-<profile>.service`ï¼‰ï¼š

```bash
systemctl --user disable --now OpenKrab-gateway.service
rm -f ~/.config/systemd/user/OpenKrab-gateway.service
systemctl --user daemon-reload
```

### Windowsï¼ˆè®¡åˆ’ä»»åŠ¡ï¼‰

é»˜è®¤ä»»åŠ¡åç§°æ˜¯ `OpenKrab Gateway`ï¼ˆæˆ– `OpenKrab Gateway (<profile>)`ï¼‰ã€‚
ä»»åŠ¡è„šæœ¬ä½äºŽä½ çš„çŠ¶æ€ç›®å½•ä¸‹ã€‚

```powershell
schtasks /Delete /F /TN "OpenKrab Gateway"
Remove-Item -Force "$env:USERPROFILE\.OpenKrab\gateway.cmd"
```

å¦‚æžœä½ ä½¿ç”¨äº†é…ç½®æ–‡ä»¶ï¼Œè¯·åˆ é™¤åŒ¹é…çš„ä»»åŠ¡åç§°å’Œ `~\.OpenKrab-<profile>\gateway.cmd`ã€‚

## æ™®é€šå®‰è£… vs æºç æ£€å‡º

### æ™®é€šå®‰è£…ï¼ˆinstall.sh / npm / pnpm / bunï¼‰

å¦‚æžœä½ ä½¿ç”¨äº† `https://OpenKrab.ai/install.sh` æˆ– `install.ps1`ï¼ŒCLI æ˜¯é€šè¿‡ `npm install -g OpenKrab@latest` å®‰è£…çš„ã€‚
ä½¿ç”¨ `npm rm -g OpenKrab` ç§»é™¤ï¼ˆæˆ– `pnpm remove -g` / `bun remove -g`ï¼Œå¦‚æžœä½ æ˜¯ç”¨é‚£ç§æ–¹å¼å®‰è£…çš„ï¼‰ã€‚

### æºç æ£€å‡ºï¼ˆgit cloneï¼‰

å¦‚æžœä½ ä»Žä»“åº“æ£€å‡ºè¿è¡Œï¼ˆ`git clone` + `OpenKrab ...` / `bun run OpenKrab ...`ï¼‰ï¼š

1. åœ¨åˆ é™¤ä»“åº“**ä¹‹å‰**å¸è½½ Gateway ç½‘å…³æœåŠ¡ï¼ˆä½¿ç”¨ä¸Šé¢çš„ç®€å•æ–¹å¼æˆ–æ‰‹åŠ¨æœåŠ¡ç§»é™¤ï¼‰ã€‚
2. åˆ é™¤ä»“åº“ç›®å½•ã€‚
3. æŒ‰ä¸Šè¿°æ–¹å¼ç§»é™¤çŠ¶æ€ + å·¥ä½œåŒºã€‚


