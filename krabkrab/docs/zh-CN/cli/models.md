---
read_when:
  - ä½ æƒ³æ›´æ”¹é»˜è®¤æ¨¡åž‹æˆ–æŸ¥çœ‹æä¾›å•†è®¤è¯çŠ¶æ€
  - ä½ æƒ³æ‰«æå¯ç”¨çš„æ¨¡åž‹/æä¾›å•†å¹¶è°ƒè¯•è®¤è¯é…ç½®
summary: "`krabkrab models` çš„ CLI å‚è€ƒï¼ˆstatus/list/set/scanã€åˆ«åã€å›žé€€ã€è®¤è¯ï¼‰"
title: models
x-i18n:
  generated_at: "2026-02-01T20:21:16Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 923b6ffc7de382ba25bc6e699f0515607e74877b39f2136ccdba2d99e1b1e9c3
  source_path: cli/models.md
  workflow: 14
---

# `krabkrab models`

æ¨¡åž‹å‘çŽ°ã€æ‰«æå’Œé…ç½®ï¼ˆé»˜è®¤æ¨¡åž‹ã€å›žé€€ã€è®¤è¯é…ç½®ï¼‰ã€‚

ç›¸å…³å†…å®¹ï¼š

- æä¾›å•† + æ¨¡åž‹ï¼š[æ¨¡åž‹](/providers/models)
- æä¾›å•†è®¤è¯è®¾ç½®ï¼š[å¿«é€Ÿå¼€å§‹](/start/getting-started)

## å¸¸ç”¨å‘½ä»¤

```bash
krabkrab models status
krabkrab models list
krabkrab models set <model-or-alias>
krabkrab models scan
```

`krabkrab models status` æ˜¾ç¤ºå·²è§£æžçš„é»˜è®¤æ¨¡åž‹/å›žé€€é…ç½®ä»¥åŠè®¤è¯æ¦‚è§ˆã€‚
å½“æä¾›å•†ä½¿ç”¨å¿«ç…§å¯ç”¨æ—¶ï¼ŒOAuth/ä»¤ç‰ŒçŠ¶æ€éƒ¨åˆ†ä¼šåŒ…å«æä¾›å•†ä½¿ç”¨å¤´ä¿¡æ¯ã€‚
æ·»åŠ  `--probe` å¯å¯¹æ¯ä¸ªå·²é…ç½®çš„æä¾›å•†é…ç½®è¿è¡Œå®žæ—¶è®¤è¯æŽ¢æµ‹ã€‚
æŽ¢æµ‹ä¼šå‘é€çœŸå®žè¯·æ±‚ï¼ˆå¯èƒ½æ¶ˆè€—ä»¤ç‰Œå¹¶è§¦å‘é€ŸçŽ‡é™åˆ¶ï¼‰ã€‚
ä½¿ç”¨ `--agent <id>` å¯æ£€æŸ¥å·²é…ç½®æ™ºèƒ½ä½“çš„æ¨¡åž‹/è®¤è¯çŠ¶æ€ã€‚çœç•¥æ—¶ï¼Œ
å‘½ä»¤ä¼šä½¿ç”¨ `krabkrab_AGENT_DIR`/`PI_CODING_AGENT_DIR`ï¼ˆå¦‚å·²è®¾ç½®ï¼‰ï¼Œå¦åˆ™ä½¿ç”¨
å·²é…ç½®çš„é»˜è®¤æ™ºèƒ½ä½“ã€‚

æ³¨æ„äº‹é¡¹ï¼š

- `models set <model-or-alias>` æŽ¥å— `provider/model` æˆ–åˆ«åã€‚
- æ¨¡åž‹å¼•ç”¨é€šè¿‡åœ¨**ç¬¬ä¸€ä¸ª** `/` å¤„æ‹†åˆ†æ¥è§£æžã€‚å¦‚æžœæ¨¡åž‹ ID åŒ…å« `/`ï¼ˆOpenRouter é£Žæ ¼ï¼‰ï¼Œéœ€åŒ…å«æä¾›å•†å‰ç¼€ï¼ˆç¤ºä¾‹ï¼š`openrouter/moonshotai/kimi-k2`ï¼‰ã€‚
- å¦‚æžœçœç•¥æä¾›å•†ï¼ŒKrabKrab ä¼šå°†è¾“å…¥è§†ä¸ºåˆ«åæˆ–**é»˜è®¤æä¾›å•†**çš„æ¨¡åž‹ï¼ˆä»…åœ¨æ¨¡åž‹ ID ä¸åŒ…å« `/` æ—¶æœ‰æ•ˆï¼‰ã€‚

### `models status`

é€‰é¡¹ï¼š

- `--json`
- `--plain`
- `--check`ï¼ˆé€€å‡ºç  1=å·²è¿‡æœŸ/ç¼ºå¤±ï¼Œ2=å³å°†è¿‡æœŸï¼‰
- `--probe`ï¼ˆå¯¹å·²é…ç½®çš„è®¤è¯é…ç½®è¿›è¡Œå®žæ—¶æŽ¢æµ‹ï¼‰
- `--probe-provider <name>`ï¼ˆæŽ¢æµ‹å•ä¸ªæä¾›å•†ï¼‰
- `--probe-profile <id>`ï¼ˆå¯é‡å¤æˆ–é€—å·åˆ†éš”çš„é…ç½® IDï¼‰
- `--probe-timeout <ms>`
- `--probe-concurrency <n>`
- `--probe-max-tokens <n>`
- `--agent <id>`ï¼ˆå·²é…ç½®çš„æ™ºèƒ½ä½“ IDï¼›è¦†ç›– `krabkrab_AGENT_DIR`/`PI_CODING_AGENT_DIR`ï¼‰

## åˆ«å + å›žé€€

```bash
krabkrab models aliases list
krabkrab models fallbacks list
```

## è®¤è¯é…ç½®

```bash
krabkrab models auth add
krabkrab models auth login --provider <id>
krabkrab models auth setup-token
krabkrab models auth paste-token
```

`models auth login` è¿è¡Œæä¾›å•†æ’ä»¶çš„è®¤è¯æµç¨‹ï¼ˆOAuth/API å¯†é’¥ï¼‰ã€‚ä½¿ç”¨
`krabkrab plugins list` æŸ¥çœ‹å·²å®‰è£…çš„æä¾›å•†ã€‚

æ³¨æ„äº‹é¡¹ï¼š

- `setup-token` ä¼šæç¤ºè¾“å…¥ setup-token å€¼ï¼ˆåœ¨ä»»æ„æœºå™¨ä¸Šä½¿ç”¨ `claude setup-token` ç”Ÿæˆï¼‰ã€‚
- `paste-token` æŽ¥å—åœ¨å…¶ä»–åœ°æ–¹æˆ–é€šè¿‡è‡ªåŠ¨åŒ–ç”Ÿæˆçš„ä»¤ç‰Œå­—ç¬¦ä¸²ã€‚

