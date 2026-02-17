---
read_when:
  - ä½ æƒ³äº†è§£ `krabkrab.ai/install.sh` çš„å·¥ä½œæœºåˆ¶
  - ä½ æƒ³è‡ªåŠ¨åŒ–å®‰è£…ï¼ˆCI / æ— å¤´çŽ¯å¢ƒï¼‰
  - ä½ æƒ³ä»Ž GitHub æ£€å‡ºå®‰è£…
summary: å®‰è£…å™¨è„šæœ¬çš„å·¥ä½œåŽŸç†ï¼ˆinstall.sh + install-cli.shï¼‰ã€å‚æ•°å’Œè‡ªåŠ¨åŒ–
title: å®‰è£…å™¨å†…éƒ¨æœºåˆ¶
x-i18n:
  generated_at: "2026-02-01T21:07:55Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 9e0a19ecb5da0a395030e1ccf0d4bedf16b83946b3432c5399d448fe5d298391
  source_path: install/installer.md
  workflow: 14
---

# å®‰è£…å™¨å†…éƒ¨æœºåˆ¶

KrabKrab æä¾›ä¸¤ä¸ªå®‰è£…å™¨è„šæœ¬ï¼ˆæ‰˜ç®¡åœ¨ `krabkrab.ai`ï¼‰ï¼š

- `https://krabkrab.ai/install.sh` â€” "æŽ¨è"å®‰è£…å™¨ï¼ˆé»˜è®¤å…¨å±€ npm å®‰è£…ï¼›ä¹Ÿå¯ä»Ž GitHub æ£€å‡ºå®‰è£…ï¼‰
- `https://krabkrab.ai/install-cli.sh` â€” æ— éœ€ root æƒé™çš„ CLI å®‰è£…å™¨ï¼ˆå®‰è£…åˆ°å¸¦æœ‰ç‹¬ç«‹ Node çš„å‰ç¼€ç›®å½•ï¼‰
- `https://krabkrab.ai/install.ps1` â€” Windows PowerShell å®‰è£…å™¨ï¼ˆé»˜è®¤ npmï¼›å¯é€‰ git å®‰è£…ï¼‰

æŸ¥çœ‹å½“å‰å‚æ•°/è¡Œä¸ºï¼Œè¿è¡Œï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --help
```

Windows (PowerShell) å¸®åŠ©ï¼š

```powershell
& ([scriptblock]::Create((iwr -useb https://krabkrab.ai/install.ps1))) -?
```

å¦‚æžœå®‰è£…å™¨å®Œæˆä½†åœ¨æ–°ç»ˆç«¯ä¸­æ‰¾ä¸åˆ° `krabkrab`ï¼Œé€šå¸¸æ˜¯ Node/npm PATH é—®é¢˜ã€‚å‚è§ï¼š[å®‰è£…](/install#nodejs--npm-path-sanity)ã€‚

## install.shï¼ˆæŽ¨èï¼‰

åŠŸèƒ½æ¦‚è¿°ï¼š

- æ£€æµ‹æ“ä½œç³»ç»Ÿï¼ˆmacOS / Linux / WSLï¼‰ã€‚
- ç¡®ä¿ Node.js **22+**ï¼ˆmacOS é€šè¿‡ Homebrewï¼›Linux é€šè¿‡ NodeSourceï¼‰ã€‚
- é€‰æ‹©å®‰è£…æ–¹å¼ï¼š
  - `npm`ï¼ˆé»˜è®¤ï¼‰ï¼š`npm install -g krabkrab@latest`
  - `git`ï¼šå…‹éš†/æž„å»ºæºç æ£€å‡ºå¹¶å®‰è£…åŒ…è£…è„šæœ¬
- åœ¨ Linux ä¸Šï¼šå¿…è¦æ—¶å°† npm å‰ç¼€åˆ‡æ¢åˆ° `~/.npm-global`ï¼Œä»¥é¿å…å…¨å±€ npm æƒé™é”™è¯¯ã€‚
- å¦‚æžœæ˜¯å‡çº§çŽ°æœ‰å®‰è£…ï¼šè¿è¡Œ `krabkrab doctor --non-interactive`ï¼ˆå°½åŠ›æ‰§è¡Œï¼‰ã€‚
- å¯¹äºŽ git å®‰è£…ï¼šå®‰è£…/æ›´æ–°åŽè¿è¡Œ `krabkrab doctor --non-interactive`ï¼ˆå°½åŠ›æ‰§è¡Œï¼‰ã€‚
- é€šè¿‡é»˜è®¤è®¾ç½® `SHARP_IGNORE_GLOBAL_LIBVIPS=1` æ¥ç¼“è§£ `sharp` åŽŸç”Ÿå®‰è£…é—®é¢˜ï¼ˆé¿å…ä½¿ç”¨ç³»ç»Ÿ libvips ç¼–è¯‘ï¼‰ã€‚

å¦‚æžœä½ *å¸Œæœ›* `sharp` é“¾æŽ¥åˆ°å…¨å±€å®‰è£…çš„ libvipsï¼ˆæˆ–ä½ æ­£åœ¨è°ƒè¯•ï¼‰ï¼Œè¯·è®¾ç½®ï¼š

```bash
SHARP_IGNORE_GLOBAL_LIBVIPS=0 curl -fsSL https://krabkrab.ai/install.sh | bash
```

### å¯å‘çŽ°æ€§ / "git å®‰è£…"æç¤º

å¦‚æžœä½ åœ¨**å·²æœ‰çš„ KrabKrab æºç æ£€å‡ºç›®å½•ä¸­**è¿è¡Œå®‰è£…å™¨ï¼ˆé€šè¿‡ `package.json` + `pnpm-workspace.yaml` æ£€æµ‹ï¼‰ï¼Œå®ƒä¼šæç¤ºï¼š

- æ›´æ–°å¹¶ä½¿ç”¨æ­¤æ£€å‡ºï¼ˆ`git`ï¼‰
- æˆ–è¿ç§»åˆ°å…¨å±€ npm å®‰è£…ï¼ˆ`npm`ï¼‰

åœ¨éžäº¤äº’å¼ä¸Šä¸‹æ–‡ä¸­ï¼ˆæ—  TTY / `--no-prompt`ï¼‰ï¼Œä½ å¿…é¡»ä¼ å…¥ `--install-method git|npm`ï¼ˆæˆ–è®¾ç½® `krabkrab_INSTALL_METHOD`ï¼‰ï¼Œå¦åˆ™è„šæœ¬å°†ä»¥é€€å‡ºç  `2` é€€å‡ºã€‚

### ä¸ºä»€ä¹ˆéœ€è¦ Git

`--install-method git` è·¯å¾„ï¼ˆå…‹éš† / æ‹‰å–ï¼‰éœ€è¦ Gitã€‚

å¯¹äºŽ `npm` å®‰è£…ï¼ŒGit *é€šå¸¸*ä¸æ˜¯å¿…éœ€çš„ï¼Œä½†æŸäº›çŽ¯å¢ƒä»ç„¶éœ€è¦å®ƒï¼ˆä¾‹å¦‚é€šè¿‡ git URL èŽ·å–è½¯ä»¶åŒ…æˆ–ä¾èµ–æ—¶ï¼‰ã€‚å®‰è£…å™¨ç›®å‰ä¼šç¡®ä¿ Git å­˜åœ¨ï¼Œä»¥é¿å…åœ¨å…¨æ–°å‘è¡Œç‰ˆä¸Šå‡ºçŽ° `spawn git ENOENT` é”™è¯¯ã€‚

### ä¸ºä»€ä¹ˆåœ¨å…¨æ–° Linux ä¸Š npm ä¼šæŠ¥ `EACCES`

åœ¨æŸäº› Linux è®¾ç½®ä¸­ï¼ˆå°¤å…¶æ˜¯é€šè¿‡ç³»ç»ŸåŒ…ç®¡ç†å™¨æˆ– NodeSource å®‰è£… Node åŽï¼‰ï¼Œnpm çš„å…¨å±€å‰ç¼€æŒ‡å‘ root æ‹¥æœ‰çš„ä½ç½®ã€‚æ­¤æ—¶ `npm install -g ...` ä¼šæŠ¥ `EACCES` / `mkdir` æƒé™é”™è¯¯ã€‚

`install.sh` é€šè¿‡å°†å‰ç¼€åˆ‡æ¢åˆ°ä»¥ä¸‹ä½ç½®æ¥ç¼“è§£æ­¤é—®é¢˜ï¼š

- `~/.npm-global`ï¼ˆå¹¶åœ¨å­˜åœ¨æ—¶å°†å…¶æ·»åŠ åˆ° `~/.bashrc` / `~/.zshrc` çš„ `PATH` ä¸­ï¼‰

## install-cli.shï¼ˆæ— éœ€ root æƒé™çš„ CLI å®‰è£…å™¨ï¼‰

æ­¤è„šæœ¬å°† `krabkrab` å®‰è£…åˆ°å‰ç¼€ç›®å½•ï¼ˆé»˜è®¤ï¼š`~/.krabkrab`ï¼‰ï¼ŒåŒæ—¶åœ¨è¯¥å‰ç¼€ä¸‹å®‰è£…ä¸“ç”¨çš„ Node è¿è¡Œæ—¶ï¼Œå› æ­¤å¯ä»¥åœ¨ä¸æƒ³æ”¹åŠ¨ç³»ç»Ÿ Node/npm çš„æœºå™¨ä¸Šä½¿ç”¨ã€‚

å¸®åŠ©ï¼š

```bash
curl -fsSL https://krabkrab.ai/install-cli.sh | bash -s -- --help
```

## install.ps1ï¼ˆWindows PowerShellï¼‰

åŠŸèƒ½æ¦‚è¿°ï¼š

- ç¡®ä¿ Node.js **22+**ï¼ˆwinget/Chocolatey/Scoop æˆ–æ‰‹åŠ¨å®‰è£…ï¼‰ã€‚
- é€‰æ‹©å®‰è£…æ–¹å¼ï¼š
  - `npm`ï¼ˆé»˜è®¤ï¼‰ï¼š`npm install -g krabkrab@latest`
  - `git`ï¼šå…‹éš†/æž„å»ºæºç æ£€å‡ºå¹¶å®‰è£…åŒ…è£…è„šæœ¬
- åœ¨å‡çº§å’Œ git å®‰è£…æ—¶è¿è¡Œ `krabkrab doctor --non-interactive`ï¼ˆå°½åŠ›æ‰§è¡Œï¼‰ã€‚

ç¤ºä¾‹ï¼š

```powershell
iwr -useb https://krabkrab.ai/install.ps1 | iex
```

```powershell
iwr -useb https://krabkrab.ai/install.ps1 | iex -InstallMethod git
```

```powershell
iwr -useb https://krabkrab.ai/install.ps1 | iex -InstallMethod git -GitDir "C:\\krabkrab"
```

çŽ¯å¢ƒå˜é‡ï¼š

- `krabkrab_INSTALL_METHOD=git|npm`
- `krabkrab_GIT_DIR=...`

Git è¦æ±‚ï¼š

å¦‚æžœä½ é€‰æ‹© `-InstallMethod git` ä½†æœªå®‰è£… Gitï¼Œå®‰è£…å™¨ä¼šæ‰“å° Git for Windows çš„é“¾æŽ¥ï¼ˆ`https://git-scm.com/download/win`ï¼‰å¹¶é€€å‡ºã€‚

å¸¸è§ Windows é—®é¢˜ï¼š

- **npm error spawn git / ENOENT**ï¼šå®‰è£… Git for Windows å¹¶é‡æ–°æ‰“å¼€ PowerShellï¼Œç„¶åŽé‡æ–°è¿è¡Œå®‰è£…å™¨ã€‚
- **"krabkrab" ä¸æ˜¯å¯è¯†åˆ«çš„å‘½ä»¤**ï¼šä½ çš„ npm å…¨å±€ bin æ–‡ä»¶å¤¹ä¸åœ¨ PATH ä¸­ã€‚å¤§å¤šæ•°ç³»ç»Ÿä½¿ç”¨ `%AppData%\\npm`ã€‚ä½ ä¹Ÿå¯ä»¥è¿è¡Œ `npm config get prefix` å¹¶å°† `\\bin` æ·»åŠ åˆ° PATHï¼Œç„¶åŽé‡æ–°æ‰“å¼€ PowerShellã€‚

