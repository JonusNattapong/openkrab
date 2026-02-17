---
read_when:
  - æ›´æ”¹ä»ªè¡¨æ¿è®¤è¯æˆ–æš´éœ²æ¨¡å¼
summary: Gateway ç½‘å…³ä»ªè¡¨æ¿ï¼ˆæŽ§åˆ¶ UIï¼‰è®¿é—®å’Œè®¤è¯
title: ä»ªè¡¨æ¿
x-i18n:
  generated_at: "2026-02-03T10:13:14Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: e6876d50e17d3dd741471ed78bef6ac175b2fdbdc1c45dd52d9d2bd013e17f31
  source_path: web/dashboard.md
  workflow: 15
---

# ä»ªè¡¨æ¿ï¼ˆæŽ§åˆ¶ UIï¼‰

Gateway ç½‘å…³ä»ªè¡¨æ¿æ˜¯é»˜è®¤åœ¨ `/` æä¾›çš„æµè§ˆå™¨æŽ§åˆ¶ UI
ï¼ˆé€šè¿‡ `gateway.controlUi.basePath` è¦†ç›–ï¼‰ã€‚

å¿«é€Ÿæ‰“å¼€ï¼ˆæœ¬åœ° Gateway ç½‘å…³ï¼‰ï¼š

- http://127.0.0.1:18789/ï¼ˆæˆ– http://localhost:18789/ï¼‰

å…³é”®å‚è€ƒï¼š

- [æŽ§åˆ¶ UI](/web/control-ui) äº†è§£ä½¿ç”¨æ–¹æ³•å’Œ UI åŠŸèƒ½ã€‚
- [Tailscale](/gateway/tailscale) äº†è§£ Serve/Funnel è‡ªåŠ¨åŒ–ã€‚
- [Web ç•Œé¢](/web) äº†è§£ç»‘å®šæ¨¡å¼å’Œå®‰å…¨æ³¨æ„äº‹é¡¹ã€‚

è®¤è¯é€šè¿‡ `connect.params.auth`ï¼ˆtoken æˆ–å¯†ç ï¼‰åœ¨ WebSocket æ¡æ‰‹æ—¶å¼ºåˆ¶æ‰§è¡Œã€‚
å‚è§ [Gateway ç½‘å…³é…ç½®](/gateway/configuration) ä¸­çš„ `gateway.auth`ã€‚

å®‰å…¨æ³¨æ„äº‹é¡¹ï¼šæŽ§åˆ¶ UI æ˜¯ä¸€ä¸ª**ç®¡ç†ç•Œé¢**ï¼ˆèŠå¤©ã€é…ç½®ã€æ‰§è¡Œå®¡æ‰¹ï¼‰ã€‚
ä¸è¦å…¬å¼€æš´éœ²å®ƒã€‚UI åœ¨é¦–æ¬¡åŠ è½½åŽå°† token å­˜å‚¨åœ¨ `localStorage` ä¸­ã€‚
ä¼˜å…ˆä½¿ç”¨ localhostã€Tailscale Serve æˆ– SSH éš§é“ã€‚

## å¿«é€Ÿè·¯å¾„ï¼ˆæŽ¨èï¼‰

- æ–°æ‰‹å¼•å¯¼åŽï¼ŒCLI çŽ°åœ¨ä¼šè‡ªåŠ¨æ‰“å¼€å¸¦æœ‰ä½ çš„ token çš„ä»ªè¡¨æ¿ï¼Œå¹¶æ‰“å°ç›¸åŒçš„å¸¦ token é“¾æŽ¥ã€‚
- éšæ—¶é‡æ–°æ‰“å¼€ï¼š`krabkrab dashboard`ï¼ˆå¤åˆ¶é“¾æŽ¥ï¼Œå¦‚æžœå¯èƒ½åˆ™æ‰“å¼€æµè§ˆå™¨ï¼Œå¦‚æžœæ˜¯æ— å¤´çŽ¯å¢ƒåˆ™æ˜¾ç¤º SSH æç¤ºï¼‰ã€‚
- token ä¿æŒæœ¬åœ°ï¼ˆä»…æŸ¥è¯¢å‚æ•°ï¼‰ï¼›UI åœ¨é¦–æ¬¡åŠ è½½åŽç§»é™¤å®ƒå¹¶ä¿å­˜åˆ° localStorageã€‚

## Token åŸºç¡€ï¼ˆæœ¬åœ° vs è¿œç¨‹ï¼‰

- **Localhost**ï¼šæ‰“å¼€ `http://127.0.0.1:18789/`ã€‚å¦‚æžœä½ çœ‹åˆ°"unauthorized"ï¼Œè¿è¡Œ `krabkrab dashboard` å¹¶ä½¿ç”¨å¸¦ token çš„é“¾æŽ¥ï¼ˆ`?token=...`ï¼‰ã€‚
- **Token æ¥æº**ï¼š`gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ï¼›UI åœ¨é¦–æ¬¡åŠ è½½åŽå­˜å‚¨å®ƒã€‚
- **éž localhost**ï¼šä½¿ç”¨ Tailscale Serveï¼ˆå¦‚æžœ `gateway.auth.allowTailscale: true` åˆ™æ— éœ€ tokenï¼‰ã€å¸¦ token çš„ tailnet ç»‘å®šï¼Œæˆ– SSH éš§é“ã€‚å‚è§ [Web ç•Œé¢](/web)ã€‚

## å¦‚æžœä½ çœ‹åˆ°"unauthorized" / 1008

- è¿è¡Œ `krabkrab dashboard` èŽ·å–æ–°çš„å¸¦ token é“¾æŽ¥ã€‚
- ç¡®ä¿ Gateway ç½‘å…³å¯è¾¾ï¼ˆæœ¬åœ°ï¼š`krabkrab status`ï¼›è¿œç¨‹ï¼šSSH éš§é“ `ssh -N -L 18789:127.0.0.1:18789 user@host` ç„¶åŽæ‰“å¼€ `http://127.0.0.1:18789/?token=...`ï¼‰ã€‚
- åœ¨ä»ªè¡¨æ¿è®¾ç½®ä¸­ï¼Œç²˜è´´ä½ åœ¨ `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ä¸­é…ç½®çš„ç›¸åŒ tokenã€‚

