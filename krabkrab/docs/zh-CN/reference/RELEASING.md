---
read_when:
  - å‘å¸ƒæ–°çš„ npm ç‰ˆæœ¬
  - å‘å¸ƒæ–°çš„ macOS åº”ç”¨ç‰ˆæœ¬
  - å‘å¸ƒå‰éªŒè¯å…ƒæ•°æ®
summary: npm + macOS åº”ç”¨çš„é€æ­¥å‘å¸ƒæ¸…å•
x-i18n:
  generated_at: "2026-02-03T10:09:28Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 1a684bc26665966eb3c9c816d58d18eead008fd710041181ece38c21c5ff1c62
  source_path: reference/RELEASING.md
  workflow: 15
---

# å‘å¸ƒæ¸…å•ï¼ˆnpm + macOSï¼‰

ä»Žä»“åº“æ ¹ç›®å½•ä½¿ç”¨ `pnpm`ï¼ˆNode 22+ï¼‰ã€‚åœ¨æ‰“æ ‡ç­¾/å‘å¸ƒå‰ä¿æŒå·¥ä½œæ ‘å¹²å‡€ã€‚

## æ“ä½œå‘˜è§¦å‘

å½“æ“ä½œå‘˜è¯´"release"æ—¶ï¼Œç«‹å³æ‰§è¡Œæ­¤é¢„æ£€ï¼ˆé™¤éžé‡åˆ°é˜»ç¢å¦åˆ™ä¸è¦é¢å¤–æé—®ï¼‰ï¼š

- é˜…è¯»æœ¬æ–‡æ¡£å’Œ `docs/platforms/mac/release.md`ã€‚
- ä»Ž `~/.profile` åŠ è½½çŽ¯å¢ƒå˜é‡å¹¶ç¡®è®¤ `SPARKLE_PRIVATE_KEY_FILE` + App Store Connect å˜é‡å·²è®¾ç½®ï¼ˆSPARKLE_PRIVATE_KEY_FILE åº”ä½äºŽ `~/.profile` ä¸­ï¼‰ã€‚
- å¦‚éœ€è¦ï¼Œä½¿ç”¨ `~/Library/CloudStorage/Dropbox/Backup/Sparkle` ä¸­çš„ Sparkle å¯†é’¥ã€‚

1. **ç‰ˆæœ¬å’Œå…ƒæ•°æ®**

- [ ] æ›´æ–° `package.json` ç‰ˆæœ¬ï¼ˆä¾‹å¦‚ `2026.1.29`ï¼‰ã€‚
- [ ] è¿è¡Œ `pnpm plugins:sync` ä»¥å¯¹é½æ‰©å±•åŒ…ç‰ˆæœ¬å’Œå˜æ›´æ—¥å¿—ã€‚
- [ ] æ›´æ–° CLI/ç‰ˆæœ¬å­—ç¬¦ä¸²ï¼š[`src/cli/program.ts`](https://github.com/krabkrab/krabkrab/blob/main/src/cli/program.ts) å’Œ [`src/provider-web.ts`](https://github.com/krabkrab/krabkrab/blob/main/src/provider-web.ts) ä¸­çš„ Baileys user agentã€‚
- [ ] ç¡®è®¤åŒ…å…ƒæ•°æ®ï¼ˆnameã€descriptionã€repositoryã€keywordsã€licenseï¼‰ä»¥åŠ `bin` æ˜ å°„æŒ‡å‘ [`krabkrab.mjs`](https://github.com/krabkrab/krabkrab/blob/main/krabkrab.mjs) ä½œä¸º `krabkrab`ã€‚
- [ ] å¦‚æžœä¾èµ–é¡¹æœ‰å˜åŒ–ï¼Œè¿è¡Œ `pnpm install` ç¡®ä¿ `pnpm-lock.yaml` æ˜¯æœ€æ–°çš„ã€‚

2. **æž„å»ºå’Œäº§ç‰©**

- [ ] å¦‚æžœ A2UI è¾“å…¥æœ‰å˜åŒ–ï¼Œè¿è¡Œ `pnpm canvas:a2ui:bundle` å¹¶æäº¤æ›´æ–°åŽçš„ [`src/canvas-host/a2ui/a2ui.bundle.js`](https://github.com/krabkrab/krabkrab/blob/main/src/canvas-host/a2ui/a2ui.bundle.js)ã€‚
- [ ] `pnpm run build`ï¼ˆé‡æ–°ç”Ÿæˆ `dist/`ï¼‰ã€‚
- [ ] éªŒè¯ npm åŒ…çš„ `files` åŒ…å«æ‰€æœ‰å¿…éœ€çš„ `dist/*` æ–‡ä»¶å¤¹ï¼ˆç‰¹åˆ«æ˜¯ç”¨äºŽ headless node + ACP CLI çš„ `dist/node-host/**` å’Œ `dist/acp/**`ï¼‰ã€‚
- [ ] ç¡®è®¤ `dist/build-info.json` å­˜åœ¨å¹¶åŒ…å«é¢„æœŸçš„ `commit` å“ˆå¸Œï¼ˆCLI æ¨ªå¹…åœ¨ npm å®‰è£…æ—¶ä½¿ç”¨æ­¤ä¿¡æ¯ï¼‰ã€‚
- [ ] å¯é€‰ï¼šæž„å»ºåŽè¿è¡Œ `npm pack --pack-destination /tmp`ï¼›æ£€æŸ¥ tarball å†…å®¹å¹¶ä¿ç•™ä»¥å¤‡ GitHub å‘å¸ƒä½¿ç”¨ï¼ˆ**ä¸è¦**æäº¤å®ƒï¼‰ã€‚

3. **å˜æ›´æ—¥å¿—å’Œæ–‡æ¡£**

- [ ] æ›´æ–° `CHANGELOG.md`ï¼Œæ·»åŠ é¢å‘ç”¨æˆ·çš„äº®ç‚¹ï¼ˆå¦‚æžœæ–‡ä»¶ä¸å­˜åœ¨åˆ™åˆ›å»ºï¼‰ï¼›æŒ‰ç‰ˆæœ¬ä¸¥æ ¼é™åºæŽ’åˆ—æ¡ç›®ã€‚
- [ ] ç¡®ä¿ README ç¤ºä¾‹/æ ‡å¿—ä¸Žå½“å‰ CLI è¡Œä¸ºåŒ¹é…ï¼ˆç‰¹åˆ«æ˜¯æ–°å‘½ä»¤æˆ–é€‰é¡¹ï¼‰ã€‚

4. **éªŒè¯**

- [ ] `pnpm build`
- [ ] `pnpm check`
- [ ] `pnpm test`ï¼ˆå¦‚éœ€è¦†ç›–çŽ‡è¾“å‡ºåˆ™ä½¿ç”¨ `pnpm test:coverage`ï¼‰
- [ ] `pnpm release:check`ï¼ˆéªŒè¯ npm pack å†…å®¹ï¼‰
- [ ] `krabkrab_INSTALL_SMOKE_SKIP_NONROOT=1 pnpm test:install:smoke`ï¼ˆDocker å®‰è£…å†’çƒŸæµ‹è¯•ï¼Œå¿«é€Ÿè·¯å¾„ï¼›å‘å¸ƒå‰å¿…éœ€ï¼‰
  - å¦‚æžœå·²çŸ¥ä¸Šä¸€ä¸ª npm å‘å¸ƒç‰ˆæœ¬æœ‰é—®é¢˜ï¼Œä¸ºé¢„å®‰è£…æ­¥éª¤è®¾ç½® `krabkrab_INSTALL_SMOKE_PREVIOUS=<last-good-version>` æˆ– `krabkrab_INSTALL_SMOKE_SKIP_PREVIOUS=1`ã€‚
- [ ]ï¼ˆå¯é€‰ï¼‰å®Œæ•´å®‰è£…ç¨‹åºå†’çƒŸæµ‹è¯•ï¼ˆæ·»åŠ éž root + CLI è¦†ç›–ï¼‰ï¼š`pnpm test:install:smoke`
- [ ]ï¼ˆå¯é€‰ï¼‰å®‰è£…ç¨‹åº E2Eï¼ˆDockerï¼Œè¿è¡Œ `curl -fsSL https://krabkrab.ai/install.sh | bash`ï¼Œæ–°æ‰‹å¼•å¯¼ï¼Œç„¶åŽè¿è¡ŒçœŸå®žå·¥å…·è°ƒç”¨ï¼‰ï¼š
  - `pnpm test:install:e2e:openai`ï¼ˆéœ€è¦ `OPENAI_API_KEY`ï¼‰
  - `pnpm test:install:e2e:anthropic`ï¼ˆéœ€è¦ `ANTHROPIC_API_KEY`ï¼‰
  - `pnpm test:install:e2e`ï¼ˆéœ€è¦ä¸¤ä¸ªå¯†é’¥ï¼›è¿è¡Œä¸¤ä¸ªæä¾›å•†ï¼‰
- [ ]ï¼ˆå¯é€‰ï¼‰å¦‚æžœä½ çš„æ›´æ”¹å½±å“å‘é€/æŽ¥æ”¶è·¯å¾„ï¼ŒæŠ½æŸ¥ Web Gateway ç½‘å…³ã€‚

5. **macOS åº”ç”¨ï¼ˆSparkleï¼‰**

- [ ] æž„å»ºå¹¶ç­¾å macOS åº”ç”¨ï¼Œç„¶åŽåŽ‹ç¼©ä»¥ä¾›åˆ†å‘ã€‚
- [ ] ç”Ÿæˆ Sparkle appcastï¼ˆé€šè¿‡ [`scripts/make_appcast.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/make_appcast.sh) ç”Ÿæˆ HTML æ³¨é‡Šï¼‰å¹¶æ›´æ–° `appcast.xml`ã€‚
- [ ] ä¿ç•™åº”ç”¨ zipï¼ˆå’Œå¯é€‰çš„ dSYM zipï¼‰ä»¥ä¾¿é™„åŠ åˆ° GitHub å‘å¸ƒã€‚
- [ ] æŒ‰ç…§ [macOS å‘å¸ƒ](/platforms/mac/release) èŽ·å–ç¡®åˆ‡å‘½ä»¤å’Œæ‰€éœ€çŽ¯å¢ƒå˜é‡ã€‚
  - `APP_BUILD` å¿…é¡»æ˜¯æ•°å­—ä¸”å•è°ƒé€’å¢žï¼ˆä¸å¸¦ `-beta`ï¼‰ï¼Œä»¥ä¾¿ Sparkle æ­£ç¡®æ¯”è¾ƒç‰ˆæœ¬ã€‚
  - å¦‚æžœè¿›è¡Œå…¬è¯ï¼Œä½¿ç”¨ä»Ž App Store Connect API çŽ¯å¢ƒå˜é‡åˆ›å»ºçš„ `krabkrab-notary` é’¥åŒ™ä¸²é…ç½®æ–‡ä»¶ï¼ˆå‚è§ [macOS å‘å¸ƒ](/platforms/mac/release)ï¼‰ã€‚

6. **å‘å¸ƒï¼ˆnpmï¼‰**

- [ ] ç¡®è®¤ git çŠ¶æ€å¹²å‡€ï¼›æ ¹æ®éœ€è¦æäº¤å¹¶æŽ¨é€ã€‚
- [ ] å¦‚éœ€è¦ï¼Œ`npm login`ï¼ˆéªŒè¯ 2FAï¼‰ã€‚
- [ ] `npm publish --access public`ï¼ˆé¢„å‘å¸ƒç‰ˆæœ¬ä½¿ç”¨ `--tag beta`ï¼‰ã€‚
- [ ] éªŒè¯æ³¨å†Œè¡¨ï¼š`npm view krabkrab version`ã€`npm view krabkrab dist-tags` å’Œ `npx -y krabkrab@X.Y.Z --version`ï¼ˆæˆ– `--help`ï¼‰ã€‚

### æ•…éšœæŽ’é™¤ï¼ˆæ¥è‡ª 2.0.0-beta2 å‘å¸ƒçš„ç¬”è®°ï¼‰

- **npm pack/publish æŒ‚èµ·æˆ–äº§ç”Ÿå·¨å¤§ tarball**ï¼š`dist/KrabKrab.app` ä¸­çš„ macOS åº”ç”¨åŒ…ï¼ˆå’Œå‘å¸ƒ zipï¼‰è¢«æ‰«å…¥åŒ…ä¸­ã€‚é€šè¿‡ `package.json` çš„ `files` ç™½åå•å‘å¸ƒå†…å®¹æ¥ä¿®å¤ï¼ˆåŒ…å« dist å­ç›®å½•ã€docsã€skillsï¼›æŽ’é™¤åº”ç”¨åŒ…ï¼‰ã€‚ç”¨ `npm pack --dry-run` ç¡®è®¤ `dist/KrabKrab.app` æœªåˆ—å‡ºã€‚
- **npm auth dist-tags çš„ Web å¾ªçŽ¯**ï¼šä½¿ç”¨æ—§ç‰ˆè®¤è¯ä»¥èŽ·å– OTP æç¤ºï¼š
  - `NPM_CONFIG_AUTH_TYPE=legacy npm dist-tag add krabkrab@X.Y.Z latest`
- **`npx` éªŒè¯å¤±è´¥å¹¶æ˜¾ç¤º `ECOMPROMISED: Lock compromised`**ï¼šä½¿ç”¨æ–°ç¼“å­˜é‡è¯•ï¼š
  - `NPM_CONFIG_CACHE=/tmp/npm-cache-$(date +%s) npx -y krabkrab@X.Y.Z --version`
- **å»¶è¿Ÿä¿®å¤åŽéœ€è¦é‡æ–°æŒ‡å‘æ ‡ç­¾**ï¼šå¼ºåˆ¶æ›´æ–°å¹¶æŽ¨é€æ ‡ç­¾ï¼Œç„¶åŽç¡®ä¿ GitHub å‘å¸ƒèµ„äº§ä»ç„¶åŒ¹é…ï¼š
  - `git tag -f vX.Y.Z && git push -f origin vX.Y.Z`

7. **GitHub å‘å¸ƒ + appcast**

- [ ] æ‰“æ ‡ç­¾å¹¶æŽ¨é€ï¼š`git tag vX.Y.Z && git push origin vX.Y.Z`ï¼ˆæˆ– `git push --tags`ï¼‰ã€‚
- [ ] ä¸º `vX.Y.Z` åˆ›å»º/åˆ·æ–° GitHub å‘å¸ƒï¼Œ**æ ‡é¢˜ä¸º `krabkrab X.Y.Z`**ï¼ˆä¸ä»…ä»…æ˜¯æ ‡ç­¾ï¼‰ï¼›æ­£æ–‡åº”åŒ…å«è¯¥ç‰ˆæœ¬çš„**å®Œæ•´**å˜æ›´æ—¥å¿—éƒ¨åˆ†ï¼ˆäº®ç‚¹ + æ›´æ”¹ + ä¿®å¤ï¼‰ï¼Œå†…è”æ˜¾ç¤ºï¼ˆæ— è£¸é“¾æŽ¥ï¼‰ï¼Œä¸”**ä¸å¾—åœ¨æ­£æ–‡ä¸­é‡å¤æ ‡é¢˜**ã€‚
- [ ] é™„åŠ äº§ç‰©ï¼š`npm pack` tarballï¼ˆå¯é€‰ï¼‰ã€`KrabKrab-X.Y.Z.zip` å’Œ `KrabKrab-X.Y.Z.dSYM.zip`ï¼ˆå¦‚æžœç”Ÿæˆï¼‰ã€‚
- [ ] æäº¤æ›´æ–°åŽçš„ `appcast.xml` å¹¶æŽ¨é€ï¼ˆSparkle ä»Ž main èŽ·å–æºï¼‰ã€‚
- [ ] ä»Žå¹²å‡€çš„ä¸´æ—¶ç›®å½•ï¼ˆæ—  `package.json`ï¼‰ï¼Œè¿è¡Œ `npx -y krabkrab@X.Y.Z send --help` ç¡®è®¤å®‰è£…/CLI å…¥å£ç‚¹æ­£å¸¸å·¥ä½œã€‚
- [ ] å®£å¸ƒ/åˆ†äº«å‘å¸ƒè¯´æ˜Žã€‚

## æ’ä»¶å‘å¸ƒèŒƒå›´ï¼ˆnpmï¼‰

æˆ‘ä»¬åªå‘å¸ƒ `@krabkrab/*` èŒƒå›´ä¸‹çš„**çŽ°æœ‰ npm æ’ä»¶**ã€‚ä¸åœ¨ npm ä¸Šçš„å†…ç½®æ’ä»¶ä¿æŒ**ä»…ç£ç›˜æ ‘**ï¼ˆä»åœ¨ `extensions/**` ä¸­å‘å¸ƒï¼‰ã€‚

èŽ·å–åˆ—è¡¨çš„æµç¨‹ï¼š

1. `npm search @krabkrab --json` å¹¶æ•èŽ·åŒ…åã€‚
2. ä¸Ž `extensions/*/package.json` åç§°æ¯”è¾ƒã€‚
3. åªå‘å¸ƒ**äº¤é›†**ï¼ˆå·²åœ¨ npm ä¸Šï¼‰ã€‚

å½“å‰ npm æ’ä»¶åˆ—è¡¨ï¼ˆæ ¹æ®éœ€è¦æ›´æ–°ï¼‰ï¼š

- @krabkrab/bluebubbles
- @krabkrab/diagnostics-otel
- @krabkrab/discord
- @krabkrab/lobster
- @krabkrab/matrix
- @krabkrab/msteams
- @krabkrab/nextcloud-talk
- @krabkrab/nostr
- @krabkrab/voice-call
- @krabkrab/zalo
- @krabkrab/zalouser

å‘å¸ƒè¯´æ˜Žè¿˜å¿…é¡»æ ‡æ³¨**é»˜è®¤æœªå¯ç”¨**çš„**æ–°å¯é€‰å†…ç½®æ’ä»¶**ï¼ˆä¾‹å¦‚ï¼š`tlon`ï¼‰ã€‚

