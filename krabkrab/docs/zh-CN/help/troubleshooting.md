---
read_when:
  - ä½ çœ‹åˆ°é”™è¯¯å¹¶æƒ³è¦ä¿®å¤è·¯å¾„
  - å®‰è£…ç¨‹åºæ˜¾ç¤ºâ€œæˆåŠŸâ€ä½† CLI ä¸å·¥ä½œ
summary: æ•…éšœæŽ’é™¤ä¸­å¿ƒï¼šç—‡çŠ¶ â†’ æ£€æŸ¥ â†’ ä¿®å¤
title: æ•…éšœæŽ’é™¤
x-i18n:
  generated_at: "2026-02-03T07:49:14Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 00ba2a20732fa22ccf9bcba264ab06ea940e9d6e96b31290811ff21a670eaad2
  source_path: help/troubleshooting.md
  workflow: 15
---

# æ•…éšœæŽ’é™¤

## æœ€åˆçš„å…­åç§’

æŒ‰é¡ºåºè¿è¡Œè¿™äº›å‘½ä»¤ï¼š

```bash
krabkrab status
krabkrab status --all
krabkrab gateway probe
krabkrab logs --follow
krabkrab doctor
```

å¦‚æžœ Gateway ç½‘å…³å¯è¾¾ï¼Œè¿›è¡Œæ·±åº¦æŽ¢æµ‹ï¼š

```bash
krabkrab status --deep
```

## å¸¸è§çš„â€œå®ƒåäº†â€æƒ…å†µ

### `krabkrab: command not found`

å‡ ä¹Žæ€»æ˜¯ Node/npm PATH é—®é¢˜ã€‚ä»Žè¿™é‡Œå¼€å§‹ï¼š

- [å®‰è£…ï¼ˆNode/npm PATH å®‰è£…å®Œæ•´æ€§æ£€æŸ¥ï¼‰](/install#nodejs--npm-path-sanity)

### å®‰è£…ç¨‹åºå¤±è´¥ï¼ˆæˆ–ä½ éœ€è¦å®Œæ•´æ—¥å¿—ï¼‰

ä»¥è¯¦ç»†æ¨¡å¼é‡æ–°è¿è¡Œå®‰è£…ç¨‹åºä»¥æŸ¥çœ‹å®Œæ•´è·Ÿè¸ªå’Œ npm è¾“å‡ºï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --verbose
```

å¯¹äºŽ beta å®‰è£…ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --beta --verbose
```

ä½ ä¹Ÿå¯ä»¥è®¾ç½® `krabkrab_VERBOSE=1` ä»£æ›¿æ ‡å¿—ã€‚

### Gateway ç½‘å…³â€œunauthorizedâ€ã€æ— æ³•è¿žæŽ¥æˆ–æŒç»­é‡è¿ž

- [Gateway ç½‘å…³æ•…éšœæŽ’é™¤](/gateway/troubleshooting)
- [Gateway ç½‘å…³è®¤è¯](/gateway/authentication)

### æŽ§åˆ¶ UI åœ¨ HTTP ä¸Šå¤±è´¥ï¼ˆéœ€è¦è®¾å¤‡èº«ä»½ï¼‰

- [Gateway ç½‘å…³æ•…éšœæŽ’é™¤](/gateway/troubleshooting)
- [æŽ§åˆ¶ UI](/web/control-ui#insecure-http)

### `docs.krabkrab.ai` æ˜¾ç¤º SSL é”™è¯¯ï¼ˆComcast/Xfinityï¼‰

ä¸€äº› Comcast/Xfinity è¿žæŽ¥é€šè¿‡ Xfinity Advanced Security é˜»æ­¢ `docs.krabkrab.ai`ã€‚
ç¦ç”¨ Advanced Security æˆ–å°† `docs.krabkrab.ai` æ·»åŠ åˆ°å…è®¸åˆ—è¡¨ï¼Œç„¶åŽé‡è¯•ã€‚

- Xfinity Advanced Security å¸®åŠ©ï¼šhttps://www.xfinity.com/support/articles/using-xfinity-xfi-advanced-security
- å¿«é€Ÿæ£€æŸ¥ï¼šå°è¯•ç§»åŠ¨çƒ­ç‚¹æˆ– VPN ä»¥ç¡®è®¤è¿™æ˜¯ ISP çº§åˆ«çš„è¿‡æ»¤

### æœåŠ¡æ˜¾ç¤ºè¿è¡Œä¸­ï¼Œä½† RPC æŽ¢æµ‹å¤±è´¥

- [Gateway ç½‘å…³æ•…éšœæŽ’é™¤](/gateway/troubleshooting)
- [åŽå°è¿›ç¨‹/æœåŠ¡](/gateway/background-process)

### æ¨¡åž‹/è®¤è¯å¤±è´¥ï¼ˆé€ŸçŽ‡é™åˆ¶ã€è´¦å•ã€â€œall models failedâ€ï¼‰

- [æ¨¡åž‹](/cli/models)
- [OAuth / è®¤è¯æ¦‚å¿µ](/concepts/oauth)

### `/model` æ˜¾ç¤º `model not allowed`

è¿™é€šå¸¸æ„å‘³ç€ `agents.defaults.models` é…ç½®ä¸ºå…è®¸åˆ—è¡¨ã€‚å½“å®ƒéžç©ºæ—¶ï¼Œåªèƒ½é€‰æ‹©é‚£äº›æä¾›å•†/æ¨¡åž‹é”®ã€‚

- æ£€æŸ¥å…è®¸åˆ—è¡¨ï¼š`krabkrab config get agents.defaults.models`
- æ·»åŠ ä½ æƒ³è¦çš„æ¨¡åž‹ï¼ˆæˆ–æ¸…é™¤å…è®¸åˆ—è¡¨ï¼‰ç„¶åŽé‡è¯• `/model`
- ä½¿ç”¨ `/models` æµè§ˆå…è®¸çš„æä¾›å•†/æ¨¡åž‹

### æäº¤é—®é¢˜æ—¶

ç²˜è´´ä¸€ä»½å®‰å…¨æŠ¥å‘Šï¼š

```bash
krabkrab status --all
```

å¦‚æžœå¯ä»¥çš„è¯ï¼ŒåŒ…å«æ¥è‡ª `krabkrab logs --follow` çš„ç›¸å…³æ—¥å¿—å°¾éƒ¨ã€‚

