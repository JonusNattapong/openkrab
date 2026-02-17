---
read_when:
  - ä¸è¿è¡Œå®Œæ•´æ™ºèƒ½ä½“å›žåˆç›´æŽ¥è°ƒç”¨å·¥å…·
  - æž„å»ºéœ€è¦å·¥å…·ç­–ç•¥å¼ºåˆ¶æ‰§è¡Œçš„è‡ªåŠ¨åŒ–
summary: é€šè¿‡ Gateway ç½‘å…³ HTTP ç«¯ç‚¹ç›´æŽ¥è°ƒç”¨å•ä¸ªå·¥å…·
title: å·¥å…·è°ƒç”¨ API
x-i18n:
  generated_at: "2026-02-03T07:48:58Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 17ccfbe0b0d9bb61cc46fb21f5c09b106ba6e8e4c2c14135a11ca8d5b77b8a88
  source_path: gateway/tools-invoke-http-api.md
  workflow: 15
---

# å·¥å…·è°ƒç”¨ï¼ˆHTTPï¼‰

KrabKrab çš„ Gateway ç½‘å…³æš´éœ²äº†ä¸€ä¸ªç®€å•çš„ HTTP ç«¯ç‚¹ç”¨äºŽç›´æŽ¥è°ƒç”¨å•ä¸ªå·¥å…·ã€‚å®ƒå§‹ç»ˆå¯ç”¨ï¼Œä½†å— Gateway ç½‘å…³è®¤è¯å’Œå·¥å…·ç­–ç•¥é™åˆ¶ã€‚

- `POST /tools/invoke`
- ä¸Ž Gateway ç½‘å…³ç›¸åŒçš„ç«¯å£ï¼ˆWS + HTTP å¤šè·¯å¤ç”¨ï¼‰ï¼š`http://<gateway-host>:<port>/tools/invoke`

é»˜è®¤æœ€å¤§è´Ÿè½½å¤§å°ä¸º 2 MBã€‚

## è®¤è¯

ä½¿ç”¨ Gateway ç½‘å…³è®¤è¯é…ç½®ã€‚å‘é€ bearer ä»¤ç‰Œï¼š

- `Authorization: Bearer <token>`

è¯´æ˜Žï¼š

- å½“ `gateway.auth.mode="token"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- å½“ `gateway.auth.mode="password"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.password`ï¼ˆæˆ– `krabkrab_GATEWAY_PASSWORD`ï¼‰ã€‚

## è¯·æ±‚ä½“

```json
{
  "tool": "sessions_list",
  "action": "json",
  "args": {},
  "sessionKey": "main",
  "dryRun": false
}
```

å­—æ®µï¼š

- `tool`ï¼ˆstringï¼Œå¿…éœ€ï¼‰ï¼šè¦è°ƒç”¨çš„å·¥å…·åç§°ã€‚
- `action`ï¼ˆstringï¼Œå¯é€‰ï¼‰ï¼šå¦‚æžœå·¥å…· schema æ”¯æŒ `action` ä¸” args è´Ÿè½½çœç•¥äº†å®ƒï¼Œåˆ™æ˜ å°„åˆ° argsã€‚
- `args`ï¼ˆobjectï¼Œå¯é€‰ï¼‰ï¼šå·¥å…·ç‰¹å®šçš„å‚æ•°ã€‚
- `sessionKey`ï¼ˆstringï¼Œå¯é€‰ï¼‰ï¼šç›®æ ‡ä¼šè¯é”®ã€‚å¦‚æžœçœç•¥æˆ–ä¸º `"main"`ï¼ŒGateway ç½‘å…³ä½¿ç”¨é…ç½®çš„ä¸»ä¼šè¯é”®ï¼ˆéµå¾ª `session.mainKey` å’Œé»˜è®¤æ™ºèƒ½ä½“ï¼Œæˆ–åœ¨å…¨å±€èŒƒå›´ä¸­ä½¿ç”¨ `global`ï¼‰ã€‚
- `dryRun`ï¼ˆbooleanï¼Œå¯é€‰ï¼‰ï¼šä¿ç•™ä¾›å°†æ¥ä½¿ç”¨ï¼›å½“å‰å¿½ç•¥ã€‚

## ç­–ç•¥ + è·¯ç”±è¡Œä¸º

å·¥å…·å¯ç”¨æ€§é€šè¿‡ Gateway ç½‘å…³æ™ºèƒ½ä½“ä½¿ç”¨çš„ç›¸åŒç­–ç•¥é“¾è¿‡æ»¤ï¼š

- `tools.profile` / `tools.byProvider.profile`
- `tools.allow` / `tools.byProvider.allow`
- `agents.<id>.tools.allow` / `agents.<id>.tools.byProvider.allow`
- ç¾¤ç»„ç­–ç•¥ï¼ˆå¦‚æžœä¼šè¯é”®æ˜ å°„åˆ°ç¾¤ç»„æˆ–æ¸ é“ï¼‰
- å­æ™ºèƒ½ä½“ç­–ç•¥ï¼ˆä½¿ç”¨å­æ™ºèƒ½ä½“ä¼šè¯é”®è°ƒç”¨æ—¶ï¼‰

å¦‚æžœå·¥å…·ä¸è¢«ç­–ç•¥å…è®¸ï¼Œç«¯ç‚¹è¿”å›ž **404**ã€‚

ä¸ºå¸®åŠ©ç¾¤ç»„ç­–ç•¥è§£æžä¸Šä¸‹æ–‡ï¼Œä½ å¯ä»¥é€‰æ‹©è®¾ç½®ï¼š

- `x-krabkrab-message-channel: <channel>`ï¼ˆç¤ºä¾‹ï¼š`slack`ã€`telegram`ï¼‰
- `x-krabkrab-account-id: <accountId>`ï¼ˆå½“å­˜åœ¨å¤šä¸ªè´¦æˆ·æ—¶ï¼‰

## å“åº”

- `200` â†’ `{ ok: true, result }`
- `400` â†’ `{ ok: false, error: { type, message } }`ï¼ˆæ— æ•ˆè¯·æ±‚æˆ–å·¥å…·é”™è¯¯ï¼‰
- `401` â†’ æœªæŽˆæƒ
- `404` â†’ å·¥å…·ä¸å¯ç”¨ï¼ˆæœªæ‰¾åˆ°æˆ–æœªåœ¨å…è®¸åˆ—è¡¨ä¸­ï¼‰
- `405` â†’ æ–¹æ³•ä¸å…è®¸

## ç¤ºä¾‹

```bash
curl -sS http://127.0.0.1:18789/tools/invoke \
  -H 'Authorization: Bearer YOUR_TOKEN' \
  -H 'Content-Type: application/json' \
  -d '{
    "tool": "sessions_list",
    "action": "json",
    "args": {}
  }'
```

