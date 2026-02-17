---
read_when:
  - ä½ æƒ³åœ¨ stable/beta/dev ä¹‹é—´åˆ‡æ¢
  - ä½ æ­£åœ¨æ ‡è®°æˆ–å‘å¸ƒé¢„å‘å¸ƒç‰ˆæœ¬
summary: stableã€beta å’Œ dev æ¸ é“ï¼šè¯­ä¹‰ã€åˆ‡æ¢å’Œæ ‡ç­¾
title: å¼€å‘æ¸ é“
x-i18n:
  generated_at: "2026-02-03T10:07:21Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 2b01219b7e705044ce39838a0da7c7fa65c719809ab2f8a51e14529064af81bf
  source_path: install/development-channels.md
  workflow: 15
---

# å¼€å‘æ¸ é“

æœ€åŽæ›´æ–°ï¼š2026-01-21

KrabKrab æä¾›ä¸‰ä¸ªæ›´æ–°æ¸ é“ï¼š

- **stable**ï¼šnpm dist-tag `latest`ã€‚
- **beta**ï¼šnpm dist-tag `beta`ï¼ˆæµ‹è¯•ä¸­çš„æž„å»ºï¼‰ã€‚
- **dev**ï¼š`main` çš„ç§»åŠ¨å¤´ï¼ˆgitï¼‰ã€‚npm dist-tagï¼š`dev`ï¼ˆå‘å¸ƒæ—¶ï¼‰ã€‚

æˆ‘ä»¬å°†æž„å»ºå‘å¸ƒåˆ° **beta**ï¼Œè¿›è¡Œæµ‹è¯•ï¼Œç„¶åŽ**å°†ç»è¿‡éªŒè¯çš„æž„å»ºæå‡åˆ° `latest`**ï¼Œ
ç‰ˆæœ¬å·ä¸å˜â€”â€”dist-tag æ˜¯ npm å®‰è£…çš„æ•°æ®æºã€‚

## åˆ‡æ¢æ¸ é“

Git checkoutï¼š

```bash
krabkrab update --channel stable
krabkrab update --channel beta
krabkrab update --channel dev
```

- `stable`/`beta` æ£€å‡ºæœ€æ–°åŒ¹é…çš„æ ‡ç­¾ï¼ˆé€šå¸¸æ˜¯åŒä¸€ä¸ªæ ‡ç­¾ï¼‰ã€‚
- `dev` åˆ‡æ¢åˆ° `main` å¹¶åœ¨ä¸Šæ¸¸åŸºç¡€ä¸Š rebaseã€‚

npm/pnpm å…¨å±€å®‰è£…ï¼š

```bash
krabkrab update --channel stable
krabkrab update --channel beta
krabkrab update --channel dev
```

è¿™ä¼šé€šè¿‡ç›¸åº”çš„ npm dist-tagï¼ˆ`latest`ã€`beta`ã€`dev`ï¼‰è¿›è¡Œæ›´æ–°ã€‚

å½“ä½ ä½¿ç”¨ `--channel` **æ˜¾å¼**åˆ‡æ¢æ¸ é“æ—¶ï¼ŒKrabKrab è¿˜ä¼šå¯¹é½å®‰è£…æ–¹å¼ï¼š

- `dev` ç¡®ä¿æœ‰ä¸€ä¸ª git checkoutï¼ˆé»˜è®¤ `~/krabkrab`ï¼Œå¯é€šè¿‡ `krabkrab_GIT_DIR` è¦†ç›–ï¼‰ï¼Œ
  æ›´æ–°å®ƒï¼Œå¹¶ä»Žè¯¥ checkout å®‰è£…å…¨å±€ CLIã€‚
- `stable`/`beta` ä½¿ç”¨åŒ¹é…çš„ dist-tag ä»Ž npm å®‰è£…ã€‚

æç¤ºï¼šå¦‚æžœä½ æƒ³åŒæ—¶ä½¿ç”¨ stable + devï¼Œä¿ç•™ä¸¤ä¸ªå…‹éš†å¹¶å°† Gateway ç½‘å…³æŒ‡å‘ stable é‚£ä¸ªã€‚

## æ’ä»¶å’Œæ¸ é“

å½“ä½ ä½¿ç”¨ `krabkrab update` åˆ‡æ¢æ¸ é“æ—¶ï¼ŒKrabKrab è¿˜ä¼šåŒæ­¥æ’ä»¶æ¥æºï¼š

- `dev` ä¼˜å…ˆä½¿ç”¨ git checkout ä¸­çš„å†…ç½®æ’ä»¶ã€‚
- `stable` å’Œ `beta` æ¢å¤ npm å®‰è£…çš„æ’ä»¶åŒ…ã€‚

## æ ‡ç­¾æœ€ä½³å®žè·µ

- ä¸ºä½ å¸Œæœ› git checkout è½åœ¨çš„å‘å¸ƒç‰ˆæœ¬æ‰“æ ‡ç­¾ï¼ˆ`vYYYY.M.D` æˆ– `vYYYY.M.D-<patch>`ï¼‰ã€‚
- ä¿æŒæ ‡ç­¾ä¸å¯å˜ï¼šæ°¸è¿œä¸è¦ç§»åŠ¨æˆ–é‡ç”¨æ ‡ç­¾ã€‚
- npm dist-tag ä»ç„¶æ˜¯ npm å®‰è£…çš„æ•°æ®æºï¼š
  - `latest` â†’ stable
  - `beta` â†’ å€™é€‰æž„å»º
  - `dev` â†’ main å¿«ç…§ï¼ˆå¯é€‰ï¼‰

## macOS åº”ç”¨å¯ç”¨æ€§

Beta å’Œ dev æž„å»ºå¯èƒ½**ä¸**åŒ…å« macOS åº”ç”¨å‘å¸ƒã€‚è¿™æ²¡é—®é¢˜ï¼š

- git æ ‡ç­¾å’Œ npm dist-tag ä»ç„¶å¯ä»¥å‘å¸ƒã€‚
- åœ¨å‘å¸ƒè¯´æ˜Žæˆ–å˜æ›´æ—¥å¿—ä¸­æ³¨æ˜Ž"æ­¤ beta æ—  macOS æž„å»º"ã€‚

