---
read_when:
  - ä½ æƒ³å®‰å…¨åœ°æ›´æ–°æºç æ£€å‡º
  - ä½ éœ€è¦äº†è§£ `--update` ç®€å†™è¡Œä¸º
summary: "`krabkrab update` çš„ CLI å‚è€ƒï¼ˆç›¸å¯¹å®‰å…¨çš„æºç æ›´æ–° + Gateway ç½‘å…³è‡ªåŠ¨é‡å¯ï¼‰"
title: update
x-i18n:
  generated_at: "2026-02-03T07:45:34Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 3a08e8ac797612c498eef54ecb83e61c9a1ee5de09162a01dbb4b3bd72897206
  source_path: cli/update.md
  workflow: 15
---

# `krabkrab update`

å®‰å…¨æ›´æ–° KrabKrab å¹¶åœ¨ stable/beta/dev æ¸ é“ä¹‹é—´åˆ‡æ¢ã€‚

å¦‚æžœä½ é€šè¿‡ **npm/pnpm** å®‰è£…ï¼ˆå…¨å±€å®‰è£…ï¼Œæ—  git å…ƒæ•°æ®ï¼‰ï¼Œæ›´æ–°é€šè¿‡ [æ›´æ–°](/install/updating) ä¸­çš„åŒ…ç®¡ç†å™¨æµç¨‹è¿›è¡Œã€‚

## ç”¨æ³•

```bash
krabkrab update
krabkrab update status
krabkrab update wizard
krabkrab update --channel beta
krabkrab update --channel dev
krabkrab update --tag beta
krabkrab update --no-restart
krabkrab update --json
krabkrab --update
```

## é€‰é¡¹

- `--no-restart`ï¼šæˆåŠŸæ›´æ–°åŽè·³è¿‡é‡å¯ Gateway ç½‘å…³æœåŠ¡ã€‚
- `--channel <stable|beta|dev>`ï¼šè®¾ç½®æ›´æ–°æ¸ é“ï¼ˆgit + npmï¼›æŒä¹…åŒ–åˆ°é…ç½®ä¸­ï¼‰ã€‚
- `--tag <dist-tag|version>`ï¼šä»…ä¸ºæœ¬æ¬¡æ›´æ–°è¦†ç›– npm dist-tag æˆ–ç‰ˆæœ¬ã€‚
- `--json`ï¼šæ‰“å°æœºå™¨å¯è¯»çš„ `UpdateRunResult` JSONã€‚
- `--timeout <seconds>`ï¼šæ¯æ­¥è¶…æ—¶æ—¶é—´ï¼ˆé»˜è®¤ 1200 ç§’ï¼‰ã€‚

æ³¨æ„ï¼šé™çº§éœ€è¦ç¡®è®¤ï¼Œå› ä¸ºæ—§ç‰ˆæœ¬å¯èƒ½ä¼šç ´åé…ç½®ã€‚

## `update status`

æ˜¾ç¤ºå½“å‰æ›´æ–°æ¸ é“ + git æ ‡ç­¾/åˆ†æ”¯/SHAï¼ˆå¯¹äºŽæºç æ£€å‡ºï¼‰ï¼Œä»¥åŠæ›´æ–°å¯ç”¨æ€§ã€‚

```bash
krabkrab update status
krabkrab update status --json
krabkrab update status --timeout 10
```

é€‰é¡¹ï¼š

- `--json`ï¼šæ‰“å°æœºå™¨å¯è¯»çš„çŠ¶æ€ JSONã€‚
- `--timeout <seconds>`ï¼šæ£€æŸ¥è¶…æ—¶æ—¶é—´ï¼ˆé»˜è®¤ 3 ç§’ï¼‰ã€‚

## `update wizard`

äº¤äº’å¼æµç¨‹ï¼Œç”¨äºŽé€‰æ‹©æ›´æ–°æ¸ é“å¹¶ç¡®è®¤æ˜¯å¦åœ¨æ›´æ–°åŽé‡å¯ Gateway ç½‘å…³ï¼ˆé»˜è®¤é‡å¯ï¼‰ã€‚å¦‚æžœä½ é€‰æ‹© `dev` ä½†æ²¡æœ‰ git æ£€å‡ºï¼Œå®ƒä¼šæä¾›åˆ›å»ºä¸€ä¸ªçš„é€‰é¡¹ã€‚

## å·¥ä½œåŽŸç†

å½“ä½ æ˜¾å¼åˆ‡æ¢æ¸ é“ï¼ˆ`--channel ...`ï¼‰æ—¶ï¼ŒKrabKrab ä¹Ÿä¼šä¿æŒå®‰è£…æ–¹å¼ä¸€è‡´ï¼š

- `dev` â†’ ç¡®ä¿å­˜åœ¨ git æ£€å‡ºï¼ˆé»˜è®¤ï¼š`~/krabkrab`ï¼Œå¯é€šè¿‡ `krabkrab_GIT_DIR` è¦†ç›–ï¼‰ï¼Œæ›´æ–°å®ƒï¼Œå¹¶ä»Žè¯¥æ£€å‡ºå®‰è£…å…¨å±€ CLIã€‚
- `stable`/`beta` â†’ ä½¿ç”¨åŒ¹é…çš„ dist-tag ä»Ž npm å®‰è£…ã€‚

## Git æ£€å‡ºæµç¨‹

æ¸ é“ï¼š

- `stable`ï¼šæ£€å‡ºæœ€æ–°çš„éž beta æ ‡ç­¾ï¼Œç„¶åŽæž„å»º + doctorã€‚
- `beta`ï¼šæ£€å‡ºæœ€æ–°çš„ `-beta` æ ‡ç­¾ï¼Œç„¶åŽæž„å»º + doctorã€‚
- `dev`ï¼šæ£€å‡º `main`ï¼Œç„¶åŽ fetch + rebaseã€‚

é«˜å±‚æ¦‚è¿°ï¼š

1. éœ€è¦å¹²å‡€çš„å·¥ä½œæ ‘ï¼ˆæ— æœªæäº¤çš„æ›´æ”¹ï¼‰ã€‚
2. åˆ‡æ¢åˆ°æ‰€é€‰æ¸ é“ï¼ˆæ ‡ç­¾æˆ–åˆ†æ”¯ï¼‰ã€‚
3. èŽ·å–ä¸Šæ¸¸ï¼ˆä»… devï¼‰ã€‚
4. ä»… devï¼šåœ¨ä¸´æ—¶å·¥ä½œæ ‘ä¸­é¢„æ£€ lint + TypeScript æž„å»ºï¼›å¦‚æžœæœ€æ–°æäº¤å¤±è´¥ï¼Œå›žé€€æœ€å¤š 10 ä¸ªæäº¤ä»¥æ‰¾åˆ°æœ€æ–°çš„å¹²å‡€æž„å»ºã€‚
5. Rebase åˆ°æ‰€é€‰æäº¤ï¼ˆä»… devï¼‰ã€‚
6. å®‰è£…ä¾èµ–ï¼ˆä¼˜å…ˆä½¿ç”¨ pnpmï¼›npm ä½œä¸ºå¤‡é€‰ï¼‰ã€‚
7. æž„å»º + æž„å»ºæŽ§åˆ¶ç•Œé¢ã€‚
8. è¿è¡Œ `krabkrab doctor` ä½œä¸ºæœ€ç»ˆçš„"å®‰å…¨æ›´æ–°"æ£€æŸ¥ã€‚
9. å°†æ’ä»¶åŒæ­¥åˆ°å½“å‰æ¸ é“ï¼ˆdev ä½¿ç”¨æ†ç»‘çš„æ‰©å±•ï¼›stable/beta ä½¿ç”¨ npmï¼‰å¹¶æ›´æ–° npm å®‰è£…çš„æ’ä»¶ã€‚

## `--update` ç®€å†™

`krabkrab --update` ä¼šé‡å†™ä¸º `krabkrab update`ï¼ˆä¾¿äºŽ shell å’Œå¯åŠ¨è„šæœ¬ä½¿ç”¨ï¼‰ã€‚

## å¦è¯·å‚é˜…

- `krabkrab doctor`ï¼ˆåœ¨ git æ£€å‡ºä¸Šä¼šæä¾›å…ˆè¿è¡Œæ›´æ–°çš„é€‰é¡¹ï¼‰
- [å¼€å‘æ¸ é“](/install/development-channels)
- [æ›´æ–°](/install/updating)
- [CLI å‚è€ƒ](/cli)

