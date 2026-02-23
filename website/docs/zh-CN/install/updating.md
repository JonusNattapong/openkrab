---
read_when:
  - æ›´æ–° OpenKrab
  - æ›´æ–°åŽå‡ºçŽ°é—®é¢˜
summary: å®‰å…¨æ›´æ–° OpenKrabï¼ˆå…¨å±€å®‰è£…æˆ–æºç ï¼‰ï¼Œä»¥åŠå›žæ»šç­–ç•¥
title: æ›´æ–°
x-i18n:
  generated_at: "2026-02-03T07:50:25Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 38cccac0839f0f22403b6508cd94ba1b401133ffc1d92d4f7640b8d04e082317
  source_path: install/updating.md
  workflow: 15
---

# æ›´æ–°

OpenKrab å‘å±•è¿…é€Ÿï¼ˆå°šæœªåˆ°"1.0"ï¼‰ã€‚å°†æ›´æ–°è§†ä¸ºå‘å¸ƒåŸºç¡€è®¾æ–½ï¼šæ›´æ–° â†’ è¿è¡Œæ£€æŸ¥ â†’ é‡å¯ï¼ˆæˆ–ä½¿ç”¨ä¼šé‡å¯çš„ `OpenKrab update`ï¼‰â†’ éªŒè¯ã€‚

## æŽ¨èï¼šé‡æ–°è¿è¡Œç½‘ç«™å®‰è£…ç¨‹åºï¼ˆåŽŸåœ°å‡çº§ï¼‰

**é¦–é€‰**çš„æ›´æ–°è·¯å¾„æ˜¯é‡æ–°è¿è¡Œç½‘ç«™ä¸Šçš„å®‰è£…ç¨‹åºã€‚å®ƒä¼šæ£€æµ‹çŽ°æœ‰å®‰è£…ã€åŽŸåœ°å‡çº§ï¼Œå¹¶åœ¨éœ€è¦æ—¶è¿è¡Œ `OpenKrab doctor`ã€‚

```bash
curl -fsSL https://OpenKrab.ai/install.sh | bash
```

è¯´æ˜Žï¼š

- å¦‚æžœä½ ä¸æƒ³å†æ¬¡è¿è¡Œæ–°æ‰‹å¼•å¯¼å‘å¯¼ï¼Œæ·»åŠ  `--no-onboard`ã€‚
- å¯¹äºŽ**æºç å®‰è£…**ï¼Œä½¿ç”¨ï¼š
  ```bash
  curl -fsSL https://OpenKrab.ai/install.sh | bash -s -- --install-method git --no-onboard
  ```
  å®‰è£…ç¨‹åº**ä»…**åœ¨ä»“åº“å¹²å‡€æ—¶æ‰ä¼šæ‰§è¡Œ `git pull --rebase`ã€‚
- å¯¹äºŽ**å…¨å±€å®‰è£…**ï¼Œè„šæœ¬åº•å±‚ä½¿ç”¨ `npm install -g OpenKrab@latest`ã€‚
- æ—§ç‰ˆè¯´æ˜Žï¼š.openkrab` ä»å¯ä½œä¸ºå…¼å®¹æ€§åž«ç‰‡ä½¿ç”¨ã€‚

## æ›´æ–°ä¹‹å‰

- äº†è§£ä½ çš„å®‰è£…æ–¹å¼ï¼š**å…¨å±€**ï¼ˆnpm/pnpmï¼‰è¿˜æ˜¯**æºç **ï¼ˆgit cloneï¼‰ã€‚
- äº†è§£ä½ çš„ Gateway ç½‘å…³è¿è¡Œæ–¹å¼ï¼š**å‰å°ç»ˆç«¯**è¿˜æ˜¯**å—ç®¡ç†æœåŠ¡**ï¼ˆlaunchd/systemdï¼‰ã€‚
- å¿«ç…§ä½ çš„å®šåˆ¶å†…å®¹ï¼š
  - é…ç½®ï¼š`~/.OpenKrab/OpenKrab.json`
  - å‡­è¯ï¼š`~/.OpenKrab/credentials/`
  - å·¥ä½œåŒºï¼š`~/.OpenKrab/workspace`

## æ›´æ–°ï¼ˆå…¨å±€å®‰è£…ï¼‰

å…¨å±€å®‰è£…ï¼ˆé€‰æ‹©ä¸€ä¸ªï¼‰ï¼š

```bash
npm i -g OpenKrab@latest
```

```bash
pnpm add -g OpenKrab@latest
```

æˆ‘ä»¬**ä¸**æŽ¨èå°† Bun ç”¨äºŽ Gateway ç½‘å…³è¿è¡Œæ—¶ï¼ˆWhatsApp/Telegram æœ‰ bugï¼‰ã€‚

åˆ‡æ¢æ›´æ–°æ¸ é“ï¼ˆgit + npm å®‰è£…ï¼‰ï¼š

```bash
OpenKrab update --channel beta
OpenKrab update --channel dev
OpenKrab update --channel stable
```

ä½¿ç”¨ `--tag <dist-tag|version>` è¿›è¡Œä¸€æ¬¡æ€§å®‰è£…æŒ‡å®šæ ‡ç­¾/ç‰ˆæœ¬ã€‚

æ¸ é“è¯­ä¹‰å’Œå‘å¸ƒè¯´æ˜Žå‚è§[å¼€å‘æ¸ é“](/install/development-channels)ã€‚

æ³¨æ„ï¼šåœ¨ npm å®‰è£…ä¸Šï¼ŒGateway ç½‘å…³åœ¨å¯åŠ¨æ—¶ä¼šè®°å½•æ›´æ–°æç¤ºï¼ˆæ£€æŸ¥å½“å‰æ¸ é“æ ‡ç­¾ï¼‰ã€‚é€šè¿‡ `update.checkOnStart: false` ç¦ç”¨ã€‚

ç„¶åŽï¼š

```bash
OpenKrab doctor
OpenKrab gateway restart
OpenKrab health
```

è¯´æ˜Žï¼š

- å¦‚æžœä½ çš„ Gateway ç½‘å…³ä½œä¸ºæœåŠ¡è¿è¡Œï¼Œ`OpenKrab gateway restart` ä¼˜äºŽæ€æ­» PIDã€‚
- å¦‚æžœä½ å›ºå®šåœ¨ç‰¹å®šç‰ˆæœ¬ï¼Œå‚è§ä¸‹é¢çš„"å›žæ»š/å›ºå®š"ã€‚

## æ›´æ–°ï¼ˆ`OpenKrab update`ï¼‰

å¯¹äºŽ**æºç å®‰è£…**ï¼ˆgit checkoutï¼‰ï¼Œé¦–é€‰ï¼š

```bash
OpenKrab update
```

å®ƒè¿è¡Œä¸€ä¸ªç›¸å¯¹å®‰å…¨çš„æ›´æ–°æµç¨‹ï¼š

- éœ€è¦å¹²å‡€çš„å·¥ä½œæ ‘ã€‚
- åˆ‡æ¢åˆ°é€‰å®šçš„æ¸ é“ï¼ˆæ ‡ç­¾æˆ–åˆ†æ”¯ï¼‰ã€‚
- èŽ·å–å¹¶ rebase åˆ°é…ç½®çš„ä¸Šæ¸¸ï¼ˆdev æ¸ é“ï¼‰ã€‚
- å®‰è£…ä¾èµ–ã€æž„å»ºã€æž„å»ºæŽ§åˆ¶ UIï¼Œå¹¶è¿è¡Œ `OpenKrab doctor`ã€‚
- é»˜è®¤é‡å¯ Gateway ç½‘å…³ï¼ˆä½¿ç”¨ `--no-restart` è·³è¿‡ï¼‰ã€‚

å¦‚æžœä½ é€šè¿‡ **npm/pnpm** å®‰è£…ï¼ˆæ²¡æœ‰ git å…ƒæ•°æ®ï¼‰ï¼Œ`OpenKrab update` å°†å°è¯•é€šè¿‡ä½ çš„åŒ…ç®¡ç†å™¨æ›´æ–°ã€‚å¦‚æžœæ— æ³•æ£€æµ‹åˆ°å®‰è£…ï¼Œè¯·æ”¹ç”¨"æ›´æ–°ï¼ˆå…¨å±€å®‰è£…ï¼‰"ã€‚

## æ›´æ–°ï¼ˆæŽ§åˆ¶ UI / RPCï¼‰

æŽ§åˆ¶ UI æœ‰**æ›´æ–°å¹¶é‡å¯**ï¼ˆRPCï¼š`update.run`ï¼‰ã€‚å®ƒï¼š

1. è¿è¡Œä¸Ž `OpenKrab update` ç›¸åŒçš„æºç æ›´æ–°æµç¨‹ï¼ˆä»…é™ git checkoutï¼‰ã€‚
2. å†™å…¥å¸¦æœ‰ç»“æž„åŒ–æŠ¥å‘Šï¼ˆstdout/stderr å°¾éƒ¨ï¼‰çš„é‡å¯å“¨å…µã€‚
3. é‡å¯ Gateway ç½‘å…³å¹¶å‘æœ€åŽæ´»è·ƒçš„ä¼šè¯ ping æŠ¥å‘Šã€‚

å¦‚æžœ rebase å¤±è´¥ï¼ŒGateway ç½‘å…³ä¼šä¸­æ­¢å¹¶åœ¨ä¸åº”ç”¨æ›´æ–°çš„æƒ…å†µä¸‹é‡å¯ã€‚

## æ›´æ–°ï¼ˆä»Žæºç ï¼‰

ä»Žä»“åº“ checkoutï¼š

é¦–é€‰ï¼š

```bash
OpenKrab update
```

æ‰‹åŠ¨ï¼ˆå¤§è‡´ç­‰æ•ˆï¼‰ï¼š

```bash
git pull
pnpm install
pnpm build
pnpm ui:build # é¦–æ¬¡è¿è¡Œæ—¶è‡ªåŠ¨å®‰è£… UI ä¾èµ–
OpenKrab doctor
OpenKrab health
```

è¯´æ˜Žï¼š

- å½“ä½ è¿è¡Œæ‰“åŒ…çš„ `OpenKrab` äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆ[`OpenKrab.mjs`](https://github.com/OpenKrab/OpenKrab/blob/main/OpenKrab.mjs)ï¼‰æˆ–ä½¿ç”¨ Node è¿è¡Œ `dist/` æ—¶ï¼Œ`pnpm build` å¾ˆé‡è¦ã€‚
- å¦‚æžœä½ ä»Žä»“åº“ checkout è¿è¡Œè€Œæ²¡æœ‰å…¨å±€å®‰è£…ï¼ŒCLI å‘½ä»¤ä½¿ç”¨ `pnpm OpenKrab ...`ã€‚
- å¦‚æžœä½ ç›´æŽ¥ä»Ž TypeScript è¿è¡Œï¼ˆ`pnpm OpenKrab ...`ï¼‰ï¼Œé€šå¸¸ä¸éœ€è¦é‡æ–°æž„å»ºï¼Œä½†**é…ç½®è¿ç§»ä»ç„¶é€‚ç”¨** â†’ è¿è¡Œ doctorã€‚
- åœ¨å…¨å±€å’Œ git å®‰è£…ä¹‹é—´åˆ‡æ¢å¾ˆå®¹æ˜“ï¼šå®‰è£…å¦ä¸€ç§æ–¹å¼ï¼Œç„¶åŽè¿è¡Œ `OpenKrab doctor` ä»¥ä¾¿å°† Gateway ç½‘å…³æœåŠ¡å…¥å£ç‚¹é‡å†™ä¸ºå½“å‰å®‰è£…ã€‚

## å§‹ç»ˆè¿è¡Œï¼š`OpenKrab doctor`

Doctor æ˜¯"å®‰å…¨æ›´æ–°"å‘½ä»¤ã€‚å®ƒæ•…æ„å¾ˆæ— èŠï¼šä¿®å¤ + è¿ç§» + è­¦å‘Šã€‚

æ³¨æ„ï¼šå¦‚æžœä½ æ˜¯**æºç å®‰è£…**ï¼ˆgit checkoutï¼‰ï¼Œ`OpenKrab doctor` ä¼šæä¾›å…ˆè¿è¡Œ `OpenKrab update`ã€‚

å®ƒé€šå¸¸åšçš„äº‹æƒ…ï¼š

- è¿ç§»å·²å¼ƒç”¨çš„é…ç½®é”®/æ—§ç‰ˆé…ç½®æ–‡ä»¶ä½ç½®ã€‚
- å®¡è®¡ç§ä¿¡ç­–ç•¥å¹¶å¯¹æœ‰é£Žé™©çš„"å¼€æ”¾"è®¾ç½®å‘å‡ºè­¦å‘Šã€‚
- æ£€æŸ¥ Gateway ç½‘å…³å¥åº·çŠ¶å†µï¼Œå¯ä»¥æä¾›é‡å¯ã€‚
- æ£€æµ‹å¹¶å°†æ—§ç‰ˆ Gateway ç½‘å…³æœåŠ¡ï¼ˆlaunchd/systemdï¼›æ—§ç‰ˆ schtasksï¼‰è¿ç§»åˆ°å½“å‰ OpenKrab æœåŠ¡ã€‚
- åœ¨ Linux ä¸Šï¼Œç¡®ä¿ systemd ç”¨æˆ· lingeringï¼ˆè¿™æ · Gateway ç½‘å…³åœ¨ç™»å‡ºåŽä»èƒ½å­˜æ´»ï¼‰ã€‚

è¯¦æƒ…ï¼š[Doctor](/gateway/doctor)

## å¯åŠ¨/åœæ­¢/é‡å¯ Gateway ç½‘å…³

CLIï¼ˆæ— è®ºæ“ä½œç³»ç»Ÿéƒ½é€‚ç”¨ï¼‰ï¼š

```bash
OpenKrab gateway status
OpenKrab gateway stop
OpenKrab gateway restart
OpenKrab gateway --port 18789
OpenKrab logs --follow
```

å¦‚æžœä½ ä½¿ç”¨å—ç®¡ç†æœåŠ¡ï¼š

- macOS launchdï¼ˆåº”ç”¨æ†ç»‘çš„ LaunchAgentï¼‰ï¼š`launchctl kickstart -k gui/$UID/bot.molt.gateway`ï¼ˆä½¿ç”¨ `bot.molt.<profile>`ï¼›æ—§ç‰ˆ `com.OpenKrab.*` ä»ç„¶æœ‰æ•ˆï¼‰
- Linux systemd ç”¨æˆ·æœåŠ¡ï¼š`systemctl --user restart OpenKrab-gateway[-<profile>].service`
- Windowsï¼ˆWSL2ï¼‰ï¼š`systemctl --user restart OpenKrab-gateway[-<profile>].service`
  - `launchctl`/`systemctl` ä»…åœ¨æœåŠ¡å·²å®‰è£…æ—¶æœ‰æ•ˆï¼›å¦åˆ™è¿è¡Œ `OpenKrab gateway install`ã€‚

è¿è¡Œæ‰‹å†Œ + ç¡®åˆ‡çš„æœåŠ¡æ ‡ç­¾ï¼š[Gateway ç½‘å…³è¿è¡Œæ‰‹å†Œ](/gateway)

## å›žæ»š/å›ºå®šï¼ˆå½“å‡ºé—®é¢˜æ—¶ï¼‰

### å›ºå®šï¼ˆå…¨å±€å®‰è£…ï¼‰

å®‰è£…å·²çŸ¥è‰¯å¥½çš„ç‰ˆæœ¬ï¼ˆå°† `<version>` æ›¿æ¢ä¸ºæœ€åŽå¯ç”¨çš„ç‰ˆæœ¬ï¼‰ï¼š

```bash
npm i -g OpenKrab@<version>
```

```bash
pnpm add -g OpenKrab@<version>
```

æç¤ºï¼šè¦æŸ¥çœ‹å½“å‰å‘å¸ƒçš„ç‰ˆæœ¬ï¼Œè¿è¡Œ `npm view OpenKrab version`ã€‚

ç„¶åŽé‡å¯ + é‡æ–°è¿è¡Œ doctorï¼š

```bash
OpenKrab doctor
OpenKrab gateway restart
```

### æŒ‰æ—¥æœŸå›ºå®šï¼ˆæºç ï¼‰

é€‰æ‹©æŸä¸ªæ—¥æœŸçš„æäº¤ï¼ˆç¤ºä¾‹ï¼š"2026-01-01 æ—¶ main çš„çŠ¶æ€"ï¼‰ï¼š

```bash
git fetch origin
git checkout "$(git rev-list -n 1 --before=\"2026-01-01\" origin/main)"
```

ç„¶åŽé‡æ–°å®‰è£…ä¾èµ– + é‡å¯ï¼š

```bash
pnpm install
pnpm build
OpenKrab gateway restart
```

å¦‚æžœä½ ä¹‹åŽæƒ³å›žåˆ°æœ€æ–°ç‰ˆæœ¬ï¼š

```bash
git checkout main
git pull
```

## å¦‚æžœä½ å¡ä½äº†

- å†æ¬¡è¿è¡Œ `OpenKrab doctor` å¹¶ä»”ç»†é˜…è¯»è¾“å‡ºï¼ˆå®ƒé€šå¸¸ä¼šå‘Šè¯‰ä½ ä¿®å¤æ–¹æ³•ï¼‰ã€‚
- æŸ¥çœ‹ï¼š[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)
- åœ¨ Discord ä¸Šæé—®ï¼šhttps://discord.gg/Krabd



