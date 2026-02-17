---
read_when:
  - å®žçŽ°æˆ–æ›´æ–° Gateway ç½‘å…³ WS å®¢æˆ·ç«¯
  - è°ƒè¯•åè®®ä¸åŒ¹é…æˆ–è¿žæŽ¥å¤±è´¥
  - é‡æ–°ç”Ÿæˆåè®®æ¨¡å¼/æ¨¡åž‹
summary: Gateway ç½‘å…³ WebSocket åè®®ï¼šæ¡æ‰‹ã€å¸§ã€ç‰ˆæœ¬æŽ§åˆ¶
title: Gateway ç½‘å…³åè®®
x-i18n:
  generated_at: "2026-02-03T07:48:42Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: bdafac40d53565901b2df450617287664d77fe4ff52681fa00cab9046b2fd850
  source_path: gateway/protocol.md
  workflow: 15
---

# Gateway ç½‘å…³åè®®ï¼ˆWebSocketï¼‰

Gateway ç½‘å…³ WS åè®®æ˜¯ KrabKrab çš„**å•ä¸€æŽ§åˆ¶å¹³é¢ + èŠ‚ç‚¹ä¼ è¾“**ã€‚æ‰€æœ‰å®¢æˆ·ç«¯ï¼ˆCLIã€Web UIã€macOS åº”ç”¨ã€iOS/Android èŠ‚ç‚¹ã€æ— å¤´èŠ‚ç‚¹ï¼‰éƒ½é€šè¿‡ WebSocket è¿žæŽ¥ï¼Œå¹¶åœ¨æ¡æ‰‹æ—¶å£°æ˜Žå…¶**è§’è‰²** + **ä½œç”¨åŸŸ**ã€‚

## ä¼ è¾“

- WebSocketï¼Œå¸¦æœ‰ JSON è´Ÿè½½çš„æ–‡æœ¬å¸§ã€‚
- ç¬¬ä¸€å¸§**å¿…é¡»**æ˜¯ `connect` è¯·æ±‚ã€‚

## æ¡æ‰‹ï¼ˆconnectï¼‰

Gateway ç½‘å…³ â†’ å®¢æˆ·ç«¯ï¼ˆè¿žæŽ¥å‰è´¨è¯¢ï¼‰ï¼š

```json
{
  "type": "event",
  "event": "connect.challenge",
  "payload": { "nonce": "â€¦", "ts": 1737264000000 }
}
```

å®¢æˆ·ç«¯ â†’ Gateway ç½‘å…³ï¼š

```json
{
  "type": "req",
  "id": "â€¦",
  "method": "connect",
  "params": {
    "minProtocol": 3,
    "maxProtocol": 3,
    "client": {
      "id": "cli",
      "version": "1.2.3",
      "platform": "macos",
      "mode": "operator"
    },
    "role": "operator",
    "scopes": ["operator.read", "operator.write"],
    "caps": [],
    "commands": [],
    "permissions": {},
    "auth": { "token": "â€¦" },
    "locale": "en-US",
    "userAgent": "krabkrab-cli/1.2.3",
    "device": {
      "id": "device_fingerprint",
      "publicKey": "â€¦",
      "signature": "â€¦",
      "signedAt": 1737264000000,
      "nonce": "â€¦"
    }
  }
}
```

Gateway ç½‘å…³ â†’ å®¢æˆ·ç«¯ï¼š

```json
{
  "type": "res",
  "id": "â€¦",
  "ok": true,
  "payload": { "type": "hello-ok", "protocol": 3, "policy": { "tickIntervalMs": 15000 } }
}
```

å½“é¢å‘è®¾å¤‡ä»¤ç‰Œæ—¶ï¼Œ`hello-ok` è¿˜åŒ…å«ï¼š

```json
{
  "auth": {
    "deviceToken": "â€¦",
    "role": "operator",
    "scopes": ["operator.read", "operator.write"]
  }
}
```

### èŠ‚ç‚¹ç¤ºä¾‹

```json
{
  "type": "req",
  "id": "â€¦",
  "method": "connect",
  "params": {
    "minProtocol": 3,
    "maxProtocol": 3,
    "client": {
      "id": "ios-node",
      "version": "1.2.3",
      "platform": "ios",
      "mode": "node"
    },
    "role": "node",
    "scopes": [],
    "caps": ["camera", "canvas", "screen", "location", "voice"],
    "commands": ["camera.snap", "canvas.navigate", "screen.record", "location.get"],
    "permissions": { "camera.capture": true, "screen.record": false },
    "auth": { "token": "â€¦" },
    "locale": "en-US",
    "userAgent": "krabkrab-ios/1.2.3",
    "device": {
      "id": "device_fingerprint",
      "publicKey": "â€¦",
      "signature": "â€¦",
      "signedAt": 1737264000000,
      "nonce": "â€¦"
    }
  }
}
```

## å¸§æ ¼å¼

- **Request**ï¼š`{type:"req", id, method, params}`
- **Response**ï¼š`{type:"res", id, ok, payload|error}`
- **Event**ï¼š`{type:"event", event, payload, seq?, stateVersion?}`

æœ‰å‰¯ä½œç”¨çš„æ–¹æ³•éœ€è¦**å¹‚ç­‰é”®**ï¼ˆè§æ¨¡å¼ï¼‰ã€‚

## è§’è‰² + ä½œç”¨åŸŸ

### è§’è‰²

- `operator` = æŽ§åˆ¶å¹³é¢å®¢æˆ·ç«¯ï¼ˆCLI/UI/è‡ªåŠ¨åŒ–ï¼‰ã€‚
- `node` = èƒ½åŠ›å®¿ä¸»ï¼ˆcamera/screen/canvas/system.runï¼‰ã€‚

### ä½œç”¨åŸŸï¼ˆoperatorï¼‰

å¸¸ç”¨ä½œç”¨åŸŸï¼š

- `operator.read`
- `operator.write`
- `operator.admin`
- `operator.approvals`
- `operator.pairing`

### èƒ½åŠ›/å‘½ä»¤/æƒé™ï¼ˆnodeï¼‰

èŠ‚ç‚¹åœ¨è¿žæŽ¥æ—¶å£°æ˜Žèƒ½åŠ›å£°æ˜Žï¼š

- `caps`ï¼šé«˜çº§èƒ½åŠ›ç±»åˆ«ã€‚
- `commands`ï¼šinvoke çš„å‘½ä»¤å…è®¸åˆ—è¡¨ã€‚
- `permissions`ï¼šç»†ç²’åº¦å¼€å…³ï¼ˆä¾‹å¦‚ `screen.record`ã€`camera.capture`ï¼‰ã€‚

Gateway ç½‘å…³å°†è¿™äº›è§†ä¸º**å£°æ˜Ž**å¹¶å¼ºåˆ¶æ‰§è¡ŒæœåŠ¡å™¨ç«¯å…è®¸åˆ—è¡¨ã€‚

## åœ¨çº¿çŠ¶æ€

- `system-presence` è¿”å›žä»¥è®¾å¤‡èº«ä»½ä¸ºé”®çš„æ¡ç›®ã€‚
- åœ¨çº¿çŠ¶æ€æ¡ç›®åŒ…å« `deviceId`ã€`roles` å’Œ `scopes`ï¼Œä»¥ä¾¿ UI å¯ä»¥ä¸ºæ¯ä¸ªè®¾å¤‡æ˜¾ç¤ºå•è¡Œï¼Œ
  å³ä½¿å®ƒåŒæ—¶ä»¥ **operator** å’Œ **node** èº«ä»½è¿žæŽ¥ã€‚

### èŠ‚ç‚¹è¾…åŠ©æ–¹æ³•

- èŠ‚ç‚¹å¯ä»¥è°ƒç”¨ `skills.bins` æ¥èŽ·å–å½“å‰çš„ skill å¯æ‰§è¡Œæ–‡ä»¶åˆ—è¡¨ï¼Œ
  ç”¨äºŽè‡ªåŠ¨å…è®¸æ£€æŸ¥ã€‚

## Exec å®¡æ‰¹

- å½“ exec è¯·æ±‚éœ€è¦å®¡æ‰¹æ—¶ï¼ŒGateway ç½‘å…³å¹¿æ’­ `exec.approval.requested`ã€‚
- æ“ä½œè€…å®¢æˆ·ç«¯é€šè¿‡è°ƒç”¨ `exec.approval.resolve` æ¥è§£å†³ï¼ˆéœ€è¦ `operator.approvals` ä½œç”¨åŸŸï¼‰ã€‚

## ç‰ˆæœ¬æŽ§åˆ¶

- `PROTOCOL_VERSION` åœ¨ `src/gateway/protocol/schema.ts` ä¸­ã€‚
- å®¢æˆ·ç«¯å‘é€ `minProtocol` + `maxProtocol`ï¼›æœåŠ¡å™¨æ‹’ç»ä¸åŒ¹é…çš„ã€‚
- æ¨¡å¼ + æ¨¡åž‹ä»Ž TypeBox å®šä¹‰ç”Ÿæˆï¼š
  - `pnpm protocol:gen`
  - `pnpm protocol:gen:swift`
  - `pnpm protocol:check`

## è®¤è¯

- å¦‚æžœè®¾ç½®äº† `krabkrab_GATEWAY_TOKEN`ï¼ˆæˆ– `--token`ï¼‰ï¼Œ`connect.params.auth.token`
  å¿…é¡»åŒ¹é…ï¼Œå¦åˆ™å¥—æŽ¥å­—å°†è¢«å…³é—­ã€‚
- é…å¯¹åŽï¼ŒGateway ç½‘å…³ä¼šé¢å‘ä¸€ä¸ªä½œç”¨äºŽè¿žæŽ¥è§’è‰² + ä½œç”¨åŸŸçš„**è®¾å¤‡ä»¤ç‰Œ**ã€‚å®ƒåœ¨ `hello-ok.auth.deviceToken` ä¸­è¿”å›žï¼Œ
  å®¢æˆ·ç«¯åº”å°†å…¶æŒä¹…åŒ–ä»¥ä¾›å°†æ¥è¿žæŽ¥ä½¿ç”¨ã€‚
- è®¾å¤‡ä»¤ç‰Œå¯ä»¥é€šè¿‡ `device.token.rotate` å’Œ `device.token.revoke` è½®æ¢/æ’¤é”€ï¼ˆéœ€è¦ `operator.pairing` ä½œç”¨åŸŸï¼‰ã€‚

## è®¾å¤‡èº«ä»½ + é…å¯¹

- èŠ‚ç‚¹åº”åŒ…å«ä»Žå¯†é’¥å¯¹æŒ‡çº¹æ´¾ç”Ÿçš„ç¨³å®šè®¾å¤‡èº«ä»½ï¼ˆ`device.id`ï¼‰ã€‚
- Gateway ç½‘å…³ä¸ºæ¯ä¸ªè®¾å¤‡ + è§’è‰²é¢å‘ä»¤ç‰Œã€‚
- æ–°è®¾å¤‡ ID éœ€è¦é…å¯¹æ‰¹å‡†ï¼Œé™¤éžå¯ç”¨äº†æœ¬åœ°è‡ªåŠ¨æ‰¹å‡†ã€‚
- **æœ¬åœ°**è¿žæŽ¥åŒ…æ‹¬ loopback å’Œ Gateway ç½‘å…³ä¸»æœºè‡ªèº«çš„ tailnet åœ°å€
  ï¼ˆå› æ­¤åŒä¸»æœº tailnet ç»‘å®šä»å¯è‡ªåŠ¨æ‰¹å‡†ï¼‰ã€‚
- æ‰€æœ‰ WS å®¢æˆ·ç«¯åœ¨ `connect` æœŸé—´å¿…é¡»åŒ…å« `device` èº«ä»½ï¼ˆoperator + nodeï¼‰ã€‚
  æŽ§åˆ¶ UI **ä»…**åœ¨å¯ç”¨ `gateway.controlUi.allowInsecureAuth` æ—¶å¯ä»¥çœç•¥å®ƒ
  ï¼ˆæˆ–ä½¿ç”¨ `gateway.controlUi.dangerouslyDisableDeviceAuth` ç”¨äºŽç´§æ€¥æƒ…å†µï¼‰ã€‚
- éžæœ¬åœ°è¿žæŽ¥å¿…é¡»ç­¾ç½²æœåŠ¡å™¨æä¾›çš„ `connect.challenge` nonceã€‚

## TLS + å›ºå®š

- WS è¿žæŽ¥æ”¯æŒ TLSã€‚
- å®¢æˆ·ç«¯å¯ä»¥é€‰æ‹©æ€§åœ°å›ºå®š Gateway ç½‘å…³è¯ä¹¦æŒ‡çº¹ï¼ˆè§ `gateway.tls`
  é…ç½®åŠ ä¸Š `gateway.remote.tlsFingerprint` æˆ– CLI `--tls-fingerprint`ï¼‰ã€‚

## èŒƒå›´

æ­¤åè®®æš´éœ²**å®Œæ•´çš„ Gateway ç½‘å…³ API**ï¼ˆstatusã€channelsã€modelsã€chatã€
agentã€sessionsã€nodesã€approvals ç­‰ï¼‰ã€‚ç¡®åˆ‡çš„æŽ¥å£ç”± `src/gateway/protocol/schema.ts` ä¸­çš„ TypeBox æ¨¡å¼å®šä¹‰ã€‚

