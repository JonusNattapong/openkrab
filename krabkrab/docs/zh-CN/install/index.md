---
read_when:
  - å®‰è£… KrabKrab
  - ä½ æƒ³ä»Ž GitHub å®‰è£…
summary: å®‰è£… KrabKrabï¼ˆæŽ¨èå®‰è£…å™¨ã€å…¨å±€å®‰è£…æˆ–ä»Žæºä»£ç å®‰è£…ï¼‰
title: å®‰è£…
x-i18n:
  generated_at: "2026-02-03T10:07:43Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: b26f48c116c26c163ee0090fb4c3e29622951bd427ecaeccba7641d97cfdf17a
  source_path: install/index.md
  workflow: 15
---

# å®‰è£…

é™¤éžæœ‰ç‰¹æ®ŠåŽŸå› ï¼Œå¦åˆ™è¯·ä½¿ç”¨å®‰è£…å™¨ã€‚å®ƒä¼šè®¾ç½® CLI å¹¶è¿è¡Œæ–°æ‰‹å¼•å¯¼ã€‚

## å¿«é€Ÿå®‰è£…ï¼ˆæŽ¨èï¼‰

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash
```

Windowsï¼ˆPowerShellï¼‰ï¼š

```powershell
iwr -useb https://krabkrab.ai/install.ps1 | iex
```

ä¸‹ä¸€æ­¥ï¼ˆå¦‚æžœä½ è·³è¿‡äº†æ–°æ‰‹å¼•å¯¼ï¼‰ï¼š

```bash
krabkrab onboard --install-daemon
```

## ç³»ç»Ÿè¦æ±‚

- **Node >=22**
- macOSã€Linux æˆ–é€šè¿‡ WSL2 çš„ Windows
- `pnpm` ä»…åœ¨ä»Žæºä»£ç æž„å»ºæ—¶éœ€è¦

## é€‰æ‹©å®‰è£…è·¯å¾„

### 1ï¼‰å®‰è£…å™¨è„šæœ¬ï¼ˆæŽ¨èï¼‰

é€šè¿‡ npm å…¨å±€å®‰è£… `krabkrab` å¹¶è¿è¡Œæ–°æ‰‹å¼•å¯¼ã€‚

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash
```

å®‰è£…å™¨æ ‡å¿—ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --help
```

è¯¦æƒ…ï¼š[å®‰è£…å™¨å†…éƒ¨åŽŸç†](/install/installer)ã€‚

éžäº¤äº’å¼ï¼ˆè·³è¿‡æ–°æ‰‹å¼•å¯¼ï¼‰ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --no-onboard
```

### 2ï¼‰å…¨å±€å®‰è£…ï¼ˆæ‰‹åŠ¨ï¼‰

å¦‚æžœä½ å·²ç»æœ‰ Nodeï¼š

```bash
npm install -g krabkrab@latest
```

å¦‚æžœä½ å…¨å±€å®‰è£…äº† libvipsï¼ˆmacOS ä¸Šé€šè¿‡ Homebrew å®‰è£…å¾ˆå¸¸è§ï¼‰ä¸” `sharp` å®‰è£…å¤±è´¥ï¼Œè¯·å¼ºåˆ¶ä½¿ç”¨é¢„æž„å»ºäºŒè¿›åˆ¶æ–‡ä»¶ï¼š

```bash
SHARP_IGNORE_GLOBAL_LIBVIPS=1 npm install -g krabkrab@latest
```

å¦‚æžœä½ çœ‹åˆ° `sharp: Please add node-gyp to your dependencies`ï¼Œè¦ä¹ˆå®‰è£…æž„å»ºå·¥å…·ï¼ˆmacOSï¼šXcode CLT + `npm install -g node-gyp`ï¼‰ï¼Œè¦ä¹ˆä½¿ç”¨ä¸Šé¢çš„ `SHARP_IGNORE_GLOBAL_LIBVIPS=1` å˜é€šæ–¹æ³•æ¥è·³è¿‡åŽŸç”Ÿæž„å»ºã€‚

æˆ–ä½¿ç”¨ pnpmï¼š

```bash
pnpm add -g krabkrab@latest
pnpm approve-builds -g                # æ‰¹å‡† krabkrabã€node-llama-cppã€sharp ç­‰
pnpm add -g krabkrab@latest           # é‡æ–°è¿è¡Œä»¥æ‰§è¡Œ postinstall è„šæœ¬
```

pnpm éœ€è¦æ˜¾å¼æ‰¹å‡†å¸¦æœ‰æž„å»ºè„šæœ¬çš„åŒ…ã€‚åœ¨é¦–æ¬¡å®‰è£…æ˜¾ç¤º"Ignored build scripts"è­¦å‘ŠåŽï¼Œè¿è¡Œ `pnpm approve-builds -g` å¹¶é€‰æ‹©åˆ—å‡ºçš„åŒ…ï¼Œç„¶åŽé‡æ–°è¿è¡Œå®‰è£…ä»¥æ‰§è¡Œ postinstall è„šæœ¬ã€‚

ç„¶åŽï¼š

```bash
krabkrab onboard --install-daemon
```

### 3ï¼‰ä»Žæºä»£ç ï¼ˆè´¡çŒ®è€…/å¼€å‘ï¼‰

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
pnpm install
pnpm ui:build # é¦–æ¬¡è¿è¡Œæ—¶è‡ªåŠ¨å®‰è£… UI ä¾èµ–
pnpm build
krabkrab onboard --install-daemon
```

æç¤ºï¼šå¦‚æžœä½ è¿˜æ²¡æœ‰å…¨å±€å®‰è£…ï¼Œè¯·é€šè¿‡ `pnpm krabkrab ...` è¿è¡Œä»“åº“å‘½ä»¤ã€‚

### 4ï¼‰å…¶ä»–å®‰è£…é€‰é¡¹

- Dockerï¼š[Docker](/install/docker)
- Nixï¼š[Nix](/install/nix)
- Ansibleï¼š[Ansible](/install/ansible)
- Bunï¼ˆä»… CLIï¼‰ï¼š[Bun](/install/bun)

## å®‰è£…åŽ

- è¿è¡Œæ–°æ‰‹å¼•å¯¼ï¼š`krabkrab onboard --install-daemon`
- å¿«é€Ÿæ£€æŸ¥ï¼š`krabkrab doctor`
- æ£€æŸ¥ Gateway ç½‘å…³å¥åº·çŠ¶æ€ï¼š`krabkrab status` + `krabkrab health`
- æ‰“å¼€ä»ªè¡¨æ¿ï¼š`krabkrab dashboard`

## å®‰è£…æ–¹å¼ï¼šnpm vs gitï¼ˆå®‰è£…å™¨ï¼‰

å®‰è£…å™¨æ”¯æŒä¸¤ç§æ–¹å¼ï¼š

- `npm`ï¼ˆé»˜è®¤ï¼‰ï¼š`npm install -g krabkrab@latest`
- `git`ï¼šä»Ž GitHub å…‹éš†/æž„å»ºå¹¶ä»Žæºä»£ç  checkout è¿è¡Œ

### CLI æ ‡å¿—

```bash
# æ˜¾å¼ npm
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method npm

# ä»Ž GitHub å®‰è£…ï¼ˆæºä»£ç  checkoutï¼‰
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git
```

å¸¸ç”¨æ ‡å¿—ï¼š

- `--install-method npm|git`
- `--git-dir <path>`ï¼ˆé»˜è®¤ï¼š`~/krabkrab`ï¼‰
- `--no-git-update`ï¼ˆä½¿ç”¨çŽ°æœ‰ checkout æ—¶è·³è¿‡ `git pull`ï¼‰
- `--no-prompt`ï¼ˆç¦ç”¨æç¤ºï¼›CI/è‡ªåŠ¨åŒ–ä¸­å¿…éœ€ï¼‰
- `--dry-run`ï¼ˆæ‰“å°å°†è¦æ‰§è¡Œçš„æ“ä½œï¼›ä¸åšä»»ä½•æ›´æ”¹ï¼‰
- `--no-onboard`ï¼ˆè·³è¿‡æ–°æ‰‹å¼•å¯¼ï¼‰

### çŽ¯å¢ƒå˜é‡

ç­‰æ•ˆçš„çŽ¯å¢ƒå˜é‡ï¼ˆå¯¹è‡ªåŠ¨åŒ–æœ‰ç”¨ï¼‰ï¼š

- `krabkrab_INSTALL_METHOD=git|npm`
- `krabkrab_GIT_DIR=...`
- `krabkrab_GIT_UPDATE=0|1`
- `krabkrab_NO_PROMPT=1`
- `krabkrab_DRY_RUN=1`
- `krabkrab_NO_ONBOARD=1`
- `SHARP_IGNORE_GLOBAL_LIBVIPS=0|1`ï¼ˆé»˜è®¤ï¼š`1`ï¼›é¿å… `sharp` é’ˆå¯¹ç³»ç»Ÿ libvips æž„å»ºï¼‰

## æ•…éšœæŽ’é™¤ï¼šæ‰¾ä¸åˆ° `krabkrab`ï¼ˆPATHï¼‰

å¿«é€Ÿè¯Šæ–­ï¼š

```bash
node -v
npm -v
npm prefix -g
echo "$PATH"
```

å¦‚æžœ `$(npm prefix -g)/bin`ï¼ˆmacOS/Linuxï¼‰æˆ– `$(npm prefix -g)`ï¼ˆWindowsï¼‰**ä¸**åœ¨ `echo "$PATH"` çš„è¾“å‡ºä¸­ï¼Œä½ çš„ shell æ— æ³•æ‰¾åˆ°å…¨å±€ npm äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆåŒ…æ‹¬ `krabkrab`ï¼‰ã€‚

ä¿®å¤ï¼šå°†å…¶æ·»åŠ åˆ°ä½ çš„ shell å¯åŠ¨æ–‡ä»¶ï¼ˆzshï¼š`~/.zshrc`ï¼Œbashï¼š`~/.bashrc`ï¼‰ï¼š

```bash
# macOS / Linux
export PATH="$(npm prefix -g)/bin:$PATH"
```

åœ¨ Windows ä¸Šï¼Œå°† `npm prefix -g` çš„è¾“å‡ºæ·»åŠ åˆ°ä½ çš„ PATHã€‚

ç„¶åŽæ‰“å¼€æ–°ç»ˆç«¯ï¼ˆæˆ–åœ¨ zsh ä¸­æ‰§è¡Œ `rehash` / åœ¨ bash ä¸­æ‰§è¡Œ `hash -r`ï¼‰ã€‚

## æ›´æ–°/å¸è½½

- æ›´æ–°ï¼š[æ›´æ–°](/install/updating)
- è¿ç§»åˆ°æ–°æœºå™¨ï¼š[è¿ç§»](/install/migrating)
- å¸è½½ï¼š[å¸è½½](/install/uninstall)

