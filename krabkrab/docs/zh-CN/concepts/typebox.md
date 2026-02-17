---
read_when:
  - æ›´æ–°åè®®æ¨¡å¼æˆ–ä»£ç ç”Ÿæˆ
summary: TypeBox æ¨¡å¼ä½œä¸º Gateway ç½‘å…³åè®®çš„å”¯ä¸€äº‹å®žæ¥æº
title: TypeBox
x-i18n:
  generated_at: "2026-02-03T07:47:23Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 233800f4f5fabe8ed0e1b3d8aded2eca27252e08c9b0b24ea9c6293e9282c918
  source_path: concepts/typebox.md
  workflow: 15
---

# TypeBox ä½œä¸ºåè®®çš„äº‹å®žæ¥æº

æœ€åŽæ›´æ–°ï¼š2026-01-10

TypeBox æ˜¯ä¸€ä¸ª TypeScript ä¼˜å…ˆçš„æ¨¡å¼åº“ã€‚æˆ‘ä»¬ç”¨å®ƒæ¥å®šä¹‰ **Gateway ç½‘å…³ WebSocket åè®®**ï¼ˆæ¡æ‰‹ã€è¯·æ±‚/å“åº”ã€æœåŠ¡å™¨äº‹ä»¶ï¼‰ã€‚è¿™äº›æ¨¡å¼é©±åŠ¨**è¿è¡Œæ—¶éªŒè¯**ã€**JSON Schema å¯¼å‡º**å’Œ macOS åº”ç”¨çš„ **Swift ä»£ç ç”Ÿæˆ**ã€‚ä¸€ä¸ªäº‹å®žæ¥æºï¼›å…¶ä»–ä¸€åˆ‡éƒ½æ˜¯ç”Ÿæˆçš„ã€‚

å¦‚æžœä½ æƒ³äº†è§£æ›´é«˜å±‚æ¬¡çš„åè®®ä¸Šä¸‹æ–‡ï¼Œè¯·ä»Ž [Gateway ç½‘å…³æž¶æž„](/concepts/architecture)å¼€å§‹ã€‚

## å¿ƒæ™ºæ¨¡åž‹ï¼ˆ30 ç§’ï¼‰

æ¯ä¸ª Gateway ç½‘å…³ WS æ¶ˆæ¯éƒ½æ˜¯ä»¥ä¸‹ä¸‰ç§å¸§ä¹‹ä¸€ï¼š

- **Request**ï¼š`{ type: "req", id, method, params }`
- **Response**ï¼š`{ type: "res", id, ok, payload | error }`
- **Event**ï¼š`{ type: "event", event, payload, seq?, stateVersion? }`

ç¬¬ä¸€ä¸ªå¸§**å¿…é¡»**æ˜¯ `connect` è¯·æ±‚ã€‚ä¹‹åŽï¼Œå®¢æˆ·ç«¯å¯ä»¥è°ƒç”¨æ–¹æ³•ï¼ˆä¾‹å¦‚ `health`ã€`send`ã€`chat.send`ï¼‰å¹¶è®¢é˜…äº‹ä»¶ï¼ˆä¾‹å¦‚ `presence`ã€`tick`ã€`agent`ï¼‰ã€‚

è¿žæŽ¥æµç¨‹ï¼ˆæœ€å°ï¼‰ï¼š

```
Client                    Gateway
  |---- req:connect -------->|
  |<---- res:hello-ok --------|
  |<---- event:tick ----------|
  |---- req:health ---------->|
  |<---- res:health ----------|
```

å¸¸ç”¨æ–¹æ³• + äº‹ä»¶ï¼š

| ç±»åˆ« | ç¤ºä¾‹                                                      | è¯´æ˜Ž                            |
| ---- | --------------------------------------------------------- | ------------------------------- |
| æ ¸å¿ƒ | `connect`ã€`health`ã€`status`                             | `connect` å¿…é¡»æ˜¯ç¬¬ä¸€ä¸ª          |
| æ¶ˆæ¯ | `send`ã€`poll`ã€`agent`ã€`agent.wait`                     | æœ‰å‰¯ä½œç”¨çš„éœ€è¦ `idempotencyKey` |
| èŠå¤© | `chat.history`ã€`chat.send`ã€`chat.abort`ã€`chat.inject`  | WebChat ä½¿ç”¨è¿™äº›                |
| ä¼šè¯ | `sessions.list`ã€`sessions.patch`ã€`sessions.delete`      | ä¼šè¯ç®¡ç†                        |
| èŠ‚ç‚¹ | `node.list`ã€`node.invoke`ã€`node.pair.*`                 | Gateway ç½‘å…³ WS + èŠ‚ç‚¹æ“ä½œ      |
| äº‹ä»¶ | `tick`ã€`presence`ã€`agent`ã€`chat`ã€`health`ã€`shutdown` | æœåŠ¡å™¨æŽ¨é€                      |

æƒå¨åˆ—è¡¨åœ¨ `src/gateway/server.ts`ï¼ˆ`METHODS`ã€`EVENTS`ï¼‰ä¸­ã€‚

## æ¨¡å¼æ‰€åœ¨ä½ç½®

- æºç ï¼š`src/gateway/protocol/schema.ts`
- è¿è¡Œæ—¶éªŒè¯å™¨ï¼ˆAJVï¼‰ï¼š`src/gateway/protocol/index.ts`
- æœåŠ¡å™¨æ¡æ‰‹ + æ–¹æ³•åˆ†å‘ï¼š`src/gateway/server.ts`
- èŠ‚ç‚¹å®¢æˆ·ç«¯ï¼š`src/gateway/client.ts`
- ç”Ÿæˆçš„ JSON Schemaï¼š`dist/protocol.schema.json`
- ç”Ÿæˆçš„ Swift æ¨¡åž‹ï¼š`apps/macos/Sources/krabkrabProtocol/GatewayModels.swift`

## å½“å‰æµç¨‹

- `pnpm protocol:gen`
  - å°† JSON Schemaï¼ˆdraftâ€‘07ï¼‰å†™å…¥ `dist/protocol.schema.json`
- `pnpm protocol:gen:swift`
  - ç”Ÿæˆ Swift Gateway ç½‘å…³æ¨¡åž‹
- `pnpm protocol:check`
  - è¿è¡Œä¸¤ä¸ªç”Ÿæˆå™¨å¹¶éªŒè¯è¾“å‡ºå·²æäº¤

## æ¨¡å¼åœ¨è¿è¡Œæ—¶çš„ä½¿ç”¨æ–¹å¼

- **æœåŠ¡å™¨ç«¯**ï¼šæ¯ä¸ªå…¥ç«™å¸§éƒ½ç”¨ AJV éªŒè¯ã€‚æ¡æ‰‹ä»…æŽ¥å—å‚æ•°åŒ¹é… `ConnectParams` çš„ `connect` è¯·æ±‚ã€‚
- **å®¢æˆ·ç«¯**ï¼šJS å®¢æˆ·ç«¯åœ¨ä½¿ç”¨ä¹‹å‰éªŒè¯äº‹ä»¶å’Œå“åº”å¸§ã€‚
- **æ–¹æ³•è¡¨é¢**ï¼šGateway ç½‘å…³åœ¨ `hello-ok` ä¸­å…¬å¸ƒæ”¯æŒçš„ `methods` å’Œ `events`ã€‚

## ç¤ºä¾‹å¸§

Connectï¼ˆç¬¬ä¸€æ¡æ¶ˆæ¯ï¼‰ï¼š

```json
{
  "type": "req",
  "id": "c1",
  "method": "connect",
  "params": {
    "minProtocol": 2,
    "maxProtocol": 2,
    "client": {
      "id": "krabkrab-macos",
      "displayName": "macos",
      "version": "1.0.0",
      "platform": "macos 15.1",
      "mode": "ui",
      "instanceId": "A1B2"
    }
  }
}
```

Hello-ok å“åº”ï¼š

```json
{
  "type": "res",
  "id": "c1",
  "ok": true,
  "payload": {
    "type": "hello-ok",
    "protocol": 2,
    "server": { "version": "dev", "connId": "ws-1" },
    "features": { "methods": ["health"], "events": ["tick"] },
    "snapshot": {
      "presence": [],
      "health": {},
      "stateVersion": { "presence": 0, "health": 0 },
      "uptimeMs": 0
    },
    "policy": { "maxPayload": 1048576, "maxBufferedBytes": 1048576, "tickIntervalMs": 30000 }
  }
}
```

è¯·æ±‚ + å“åº”ï¼š

```json
{ "type": "req", "id": "r1", "method": "health" }
```

```json
{ "type": "res", "id": "r1", "ok": true, "payload": { "ok": true } }
```

äº‹ä»¶ï¼š

```json
{ "type": "event", "event": "tick", "payload": { "ts": 1730000000 }, "seq": 12 }
```

## æœ€å°å®¢æˆ·ç«¯ï¼ˆNode.jsï¼‰

æœ€å°å¯ç”¨æµç¨‹ï¼šconnect + healthã€‚

```ts
import { WebSocket } from "ws";

const ws = new WebSocket("ws://127.0.0.1:18789");

ws.on("open", () => {
  ws.send(
    JSON.stringify({
      type: "req",
      id: "c1",
      method: "connect",
      params: {
        minProtocol: 3,
        maxProtocol: 3,
        client: {
          id: "cli",
          displayName: "example",
          version: "dev",
          platform: "node",
          mode: "cli",
        },
      },
    }),
  );
});

ws.on("message", (data) => {
  const msg = JSON.parse(String(data));
  if (msg.type === "res" && msg.id === "c1" && msg.ok) {
    ws.send(JSON.stringify({ type: "req", id: "h1", method: "health" }));
  }
  if (msg.type === "res" && msg.id === "h1") {
    console.log("health:", msg.payload);
    ws.close();
  }
});
```

## å®žè·µç¤ºä¾‹ï¼šç«¯åˆ°ç«¯æ·»åŠ æ–¹æ³•

ç¤ºä¾‹ï¼šæ·»åŠ ä¸€ä¸ªæ–°çš„ `system.echo` è¯·æ±‚ï¼Œè¿”å›ž `{ ok: true, text }`ã€‚

1. **æ¨¡å¼ï¼ˆäº‹å®žæ¥æºï¼‰**

æ·»åŠ åˆ° `src/gateway/protocol/schema.ts`ï¼š

```ts
export const SystemEchoParamsSchema = Type.Object(
  { text: NonEmptyString },
  { additionalProperties: false },
);

export const SystemEchoResultSchema = Type.Object(
  { ok: Type.Boolean(), text: NonEmptyString },
  { additionalProperties: false },
);
```

å°†ä¸¤è€…æ·»åŠ åˆ° `ProtocolSchemas` å¹¶å¯¼å‡ºç±»åž‹ï¼š

```ts
  SystemEchoParams: SystemEchoParamsSchema,
  SystemEchoResult: SystemEchoResultSchema,
```

```ts
export type SystemEchoParams = Static<typeof SystemEchoParamsSchema>;
export type SystemEchoResult = Static<typeof SystemEchoResultSchema>;
```

2. **éªŒè¯**

åœ¨ `src/gateway/protocol/index.ts` ä¸­ï¼Œå¯¼å‡ºä¸€ä¸ª AJV éªŒè¯å™¨ï¼š

```ts
export const validateSystemEchoParams = ajv.compile<SystemEchoParams>(SystemEchoParamsSchema);
```

3. **æœåŠ¡å™¨è¡Œä¸º**

åœ¨ `src/gateway/server-methods/system.ts` ä¸­æ·»åŠ å¤„ç†ç¨‹åºï¼š

```ts
export const systemHandlers: GatewayRequestHandlers = {
  "system.echo": ({ params, respond }) => {
    const text = String(params.text ?? "");
    respond(true, { ok: true, text });
  },
};
```

åœ¨ `src/gateway/server-methods.ts` ä¸­æ³¨å†Œï¼ˆå·²åˆå¹¶ `systemHandlers`ï¼‰ï¼Œç„¶åŽå°† `"system.echo"` æ·»åŠ åˆ° `src/gateway/server.ts` ä¸­çš„ `METHODS`ã€‚

4. **é‡æ–°ç”Ÿæˆ**

```bash
pnpm protocol:check
```

5. **æµ‹è¯• + æ–‡æ¡£**

åœ¨ `src/gateway/server.*.test.ts` ä¸­æ·»åŠ æœåŠ¡å™¨æµ‹è¯•ï¼Œå¹¶åœ¨æ–‡æ¡£ä¸­è®°å½•è¯¥æ–¹æ³•ã€‚

## Swift ä»£ç ç”Ÿæˆè¡Œä¸º

Swift ç”Ÿæˆå™¨è¾“å‡ºï¼š

- å¸¦æœ‰ `req`ã€`res`ã€`event` å’Œ `unknown` æƒ…å†µçš„ `GatewayFrame` æžšä¸¾
- å¼ºç±»åž‹çš„ payload ç»“æž„ä½“/æžšä¸¾
- `ErrorCode` å€¼å’Œ `GATEWAY_PROTOCOL_VERSION`

æœªçŸ¥çš„å¸§ç±»åž‹ä¿ç•™ä¸ºåŽŸå§‹ payload ä»¥å®žçŽ°å‘å‰å…¼å®¹ã€‚

## ç‰ˆæœ¬æŽ§åˆ¶ + å…¼å®¹æ€§

- `PROTOCOL_VERSION` åœ¨ `src/gateway/protocol/schema.ts` ä¸­ã€‚
- å®¢æˆ·ç«¯å‘é€ `minProtocol` + `maxProtocol`ï¼›æœåŠ¡å™¨æ‹’ç»ä¸åŒ¹é…çš„ã€‚
- Swift æ¨¡åž‹ä¿ç•™æœªçŸ¥å¸§ç±»åž‹ä»¥é¿å…ç ´åæ—§å®¢æˆ·ç«¯ã€‚

## æ¨¡å¼æ¨¡å¼å’Œçº¦å®š

- å¤§å¤šæ•°å¯¹è±¡ä½¿ç”¨ `additionalProperties: false` ä»¥å®žçŽ°ä¸¥æ ¼çš„ payloadã€‚
- `NonEmptyString` æ˜¯ ID å’Œæ–¹æ³•/äº‹ä»¶åç§°çš„é»˜è®¤å€¼ã€‚
- é¡¶å±‚ `GatewayFrame` åœ¨ `type` ä¸Šä½¿ç”¨**é‰´åˆ«å™¨**ã€‚
- æœ‰å‰¯ä½œç”¨çš„æ–¹æ³•é€šå¸¸éœ€è¦åœ¨ params ä¸­åŒ…å« `idempotencyKey`ï¼ˆç¤ºä¾‹ï¼š`send`ã€`poll`ã€`agent`ã€`chat.send`ï¼‰ã€‚

## å®žæ—¶ schema JSON

ç”Ÿæˆçš„ JSON Schema åœ¨ä»“åº“çš„ `dist/protocol.schema.json` ä¸­ã€‚å‘å¸ƒçš„åŽŸå§‹æ–‡ä»¶é€šå¸¸å¯åœ¨ä»¥ä¸‹ä½ç½®èŽ·å–ï¼š

- https://raw.githubusercontent.com/krabkrab/krabkrab/main/dist/protocol.schema.json

## å½“ä½ æ›´æ”¹æ¨¡å¼æ—¶

1. æ›´æ–° TypeBox æ¨¡å¼ã€‚
2. è¿è¡Œ `pnpm protocol:check`ã€‚
3. æäº¤é‡æ–°ç”Ÿæˆçš„ schema + Swift æ¨¡åž‹ã€‚

