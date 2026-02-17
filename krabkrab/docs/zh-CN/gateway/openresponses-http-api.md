---
read_when:
  - é›†æˆä½¿ç”¨ OpenResponses API çš„å®¢æˆ·ç«¯
  - ä½ éœ€è¦åŸºäºŽ item çš„è¾“å…¥ã€å®¢æˆ·ç«¯å·¥å…·è°ƒç”¨æˆ– SSE äº‹ä»¶
summary: ä»Ž Gateway ç½‘å…³æš´éœ²å…¼å®¹ OpenResponses çš„ /v1/responses HTTP ç«¯ç‚¹
title: OpenResponses API
x-i18n:
  generated_at: "2026-02-03T07:48:43Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 0597714837f8b210c38eeef53561894220c1473e54c56a5c69984847685d518c
  source_path: gateway/openresponses-http-api.md
  workflow: 15
---

# OpenResponses APIï¼ˆHTTPï¼‰

KrabKrab çš„ Gateway ç½‘å…³å¯ä»¥æä¾›å…¼å®¹ OpenResponses çš„ `POST /v1/responses` ç«¯ç‚¹ã€‚

æ­¤ç«¯ç‚¹**é»˜è®¤ç¦ç”¨**ã€‚è¯·å…ˆåœ¨é…ç½®ä¸­å¯ç”¨ã€‚

- `POST /v1/responses`
- ä¸Ž Gateway ç½‘å…³ç›¸åŒçš„ç«¯å£ï¼ˆWS + HTTP å¤šè·¯å¤ç”¨ï¼‰ï¼š`http://<gateway-host>:<port>/v1/responses`

åº•å±‚å®žçŽ°ä¸­ï¼Œè¯·æ±‚ä½œä¸ºæ­£å¸¸çš„ Gateway ç½‘å…³æ™ºèƒ½ä½“è¿è¡Œæ‰§è¡Œï¼ˆä¸Ž `krabkrab agent` ç›¸åŒçš„ä»£ç è·¯å¾„ï¼‰ï¼Œå› æ­¤è·¯ç”±/æƒé™/é…ç½®ä¸Žä½ çš„ Gateway ç½‘å…³ä¸€è‡´ã€‚

## è®¤è¯

ä½¿ç”¨ Gateway ç½‘å…³è®¤è¯é…ç½®ã€‚å‘é€ bearer ä»¤ç‰Œï¼š

- `Authorization: Bearer <token>`

è¯´æ˜Žï¼š

- å½“ `gateway.auth.mode="token"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- å½“ `gateway.auth.mode="password"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.password`ï¼ˆæˆ– `krabkrab_GATEWAY_PASSWORD`ï¼‰ã€‚

## é€‰æ‹©æ™ºèƒ½ä½“

æ— éœ€è‡ªå®šä¹‰å¤´ï¼šåœ¨ OpenResponses `model` å­—æ®µä¸­ç¼–ç æ™ºèƒ½ä½“ idï¼š

- `model: "krabkrab:<agentId>"`ï¼ˆç¤ºä¾‹ï¼š`"krabkrab:main"`ã€`"krabkrab:beta"`ï¼‰
- `model: "agent:<agentId>"`ï¼ˆåˆ«åï¼‰

æˆ–é€šè¿‡å¤´æŒ‡å®šç‰¹å®šçš„ KrabKrab æ™ºèƒ½ä½“ï¼š

- `x-krabkrab-agent-id: <agentId>`ï¼ˆé»˜è®¤ï¼š`main`ï¼‰

é«˜çº§ï¼š

- `x-krabkrab-session-key: <sessionKey>` å®Œå…¨æŽ§åˆ¶ä¼šè¯è·¯ç”±ã€‚

## å¯ç”¨ç«¯ç‚¹

å°† `gateway.http.endpoints.responses.enabled` è®¾ç½®ä¸º `true`ï¼š

```json5
{
  gateway: {
    http: {
      endpoints: {
        responses: { enabled: true },
      },
    },
  },
}
```

## ç¦ç”¨ç«¯ç‚¹

å°† `gateway.http.endpoints.responses.enabled` è®¾ç½®ä¸º `false`ï¼š

```json5
{
  gateway: {
    http: {
      endpoints: {
        responses: { enabled: false },
      },
    },
  },
}
```

## ä¼šè¯è¡Œä¸º

é»˜è®¤æƒ…å†µä¸‹ï¼Œç«¯ç‚¹**æ¯ä¸ªè¯·æ±‚éƒ½æ˜¯æ— çŠ¶æ€çš„**ï¼ˆæ¯æ¬¡è°ƒç”¨ç”Ÿæˆæ–°çš„ä¼šè¯é”®ï¼‰ã€‚

å¦‚æžœè¯·æ±‚åŒ…å« OpenResponses `user` å­—ç¬¦ä¸²ï¼ŒGateway ç½‘å…³ä¼šä»Žä¸­æ´¾ç”Ÿç¨³å®šçš„ä¼šè¯é”®ï¼Œè¿™æ ·é‡å¤è°ƒç”¨å¯ä»¥å…±äº«æ™ºèƒ½ä½“ä¼šè¯ã€‚

## è¯·æ±‚ç»“æž„ï¼ˆæ”¯æŒçš„ï¼‰

è¯·æ±‚éµå¾ª OpenResponses APIï¼Œä½¿ç”¨åŸºäºŽ item çš„è¾“å…¥ã€‚å½“å‰æ”¯æŒï¼š

- `input`ï¼šå­—ç¬¦ä¸²æˆ– item å¯¹è±¡æ•°ç»„ã€‚
- `instructions`ï¼šåˆå¹¶åˆ°ç³»ç»Ÿæç¤ºä¸­ã€‚
- `tools`ï¼šå®¢æˆ·ç«¯å·¥å…·å®šä¹‰ï¼ˆå‡½æ•°å·¥å…·ï¼‰ã€‚
- `tool_choice`ï¼šè¿‡æ»¤æˆ–è¦æ±‚å®¢æˆ·ç«¯å·¥å…·ã€‚
- `stream`ï¼šå¯ç”¨ SSE æµå¼ä¼ è¾“ã€‚
- `max_output_tokens`ï¼šå°½åŠ›è€Œä¸ºçš„è¾“å‡ºé™åˆ¶ï¼ˆå–å†³äºŽæä¾›å•†ï¼‰ã€‚
- `user`ï¼šç¨³å®šçš„ä¼šè¯è·¯ç”±ã€‚

æŽ¥å—ä½†**å½“å‰å¿½ç•¥**ï¼š

- `max_tool_calls`
- `reasoning`
- `metadata`
- `store`
- `previous_response_id`
- `truncation`

## Itemï¼ˆè¾“å…¥ï¼‰

### `message`

è§’è‰²ï¼š`system`ã€`developer`ã€`user`ã€`assistant`ã€‚

- `system` å’Œ `developer` è¿½åŠ åˆ°ç³»ç»Ÿæç¤ºã€‚
- æœ€è¿‘çš„ `user` æˆ– `function_call_output` item æˆä¸º"å½“å‰æ¶ˆæ¯"ã€‚
- è¾ƒæ—©çš„ user/assistant æ¶ˆæ¯ä½œä¸ºä¸Šä¸‹æ–‡åŽ†å²åŒ…å«ã€‚

### `function_call_output`ï¼ˆåŸºäºŽå›žåˆçš„å·¥å…·ï¼‰

å°†å·¥å…·ç»“æžœå‘é€å›žæ¨¡åž‹ï¼š

```json
{
  "type": "function_call_output",
  "call_id": "call_123",
  "output": "{\"temperature\": \"72F\"}"
}
```

### `reasoning` å’Œ `item_reference`

ä¸ºäº† schema å…¼å®¹æ€§è€ŒæŽ¥å—ï¼Œä½†åœ¨æž„å»ºæç¤ºæ—¶å¿½ç•¥ã€‚

## å·¥å…·ï¼ˆå®¢æˆ·ç«¯å‡½æ•°å·¥å…·ï¼‰

ä½¿ç”¨ `tools: [{ type: "function", function: { name, description?, parameters? } }]` æä¾›å·¥å…·ã€‚

å¦‚æžœæ™ºèƒ½ä½“å†³å®šè°ƒç”¨å·¥å…·ï¼Œå“åº”è¿”å›žä¸€ä¸ª `function_call` è¾“å‡º itemã€‚ç„¶åŽä½ å‘é€å¸¦æœ‰ `function_call_output` çš„åŽç»­è¯·æ±‚ä»¥ç»§ç»­å›žåˆã€‚

## å›¾åƒï¼ˆ`input_image`ï¼‰

æ”¯æŒ base64 æˆ– URL æ¥æºï¼š

```json
{
  "type": "input_image",
  "source": { "type": "url", "url": "https://example.com/image.png" }
}
```

å…è®¸çš„ MIME ç±»åž‹ï¼ˆå½“å‰ï¼‰ï¼š`image/jpeg`ã€`image/png`ã€`image/gif`ã€`image/webp`ã€‚
æœ€å¤§å¤§å°ï¼ˆå½“å‰ï¼‰ï¼š10MBã€‚

## æ–‡ä»¶ï¼ˆ`input_file`ï¼‰

æ”¯æŒ base64 æˆ– URL æ¥æºï¼š

```json
{
  "type": "input_file",
  "source": {
    "type": "base64",
    "media_type": "text/plain",
    "data": "SGVsbG8gV29ybGQh",
    "filename": "hello.txt"
  }
}
```

å…è®¸çš„ MIME ç±»åž‹ï¼ˆå½“å‰ï¼‰ï¼š`text/plain`ã€`text/markdown`ã€`text/html`ã€`text/csv`ã€`application/json`ã€`application/pdf`ã€‚

æœ€å¤§å¤§å°ï¼ˆå½“å‰ï¼‰ï¼š5MBã€‚

å½“å‰è¡Œä¸ºï¼š

- æ–‡ä»¶å†…å®¹è¢«è§£ç å¹¶æ·»åŠ åˆ°**ç³»ç»Ÿæç¤º**ï¼Œè€Œä¸æ˜¯ç”¨æˆ·æ¶ˆæ¯ï¼Œæ‰€ä»¥å®ƒä¿æŒä¸´æ—¶æ€§ï¼ˆä¸æŒä¹…åŒ–åœ¨ä¼šè¯åŽ†å²ä¸­ï¼‰ã€‚
- PDF è¢«è§£æžæå–æ–‡æœ¬ã€‚å¦‚æžœå‘çŽ°çš„æ–‡æœ¬å¾ˆå°‘ï¼Œå‰å‡ é¡µä¼šè¢«æ …æ ¼åŒ–ä¸ºå›¾åƒå¹¶ä¼ é€’ç»™æ¨¡åž‹ã€‚

PDF è§£æžä½¿ç”¨ Node å‹å¥½çš„ `pdfjs-dist` legacy æž„å»ºï¼ˆæ—  workerï¼‰ã€‚çŽ°ä»£ PDF.js æž„å»ºæœŸæœ›æµè§ˆå™¨ worker/DOM å…¨å±€å˜é‡ï¼Œå› æ­¤ä¸åœ¨ Gateway ç½‘å…³ä¸­ä½¿ç”¨ã€‚

URL èŽ·å–é»˜è®¤å€¼ï¼š

- `files.allowUrl`ï¼š`true`
- `images.allowUrl`ï¼š`true`
- è¯·æ±‚å—åˆ°ä¿æŠ¤ï¼ˆDNS è§£æžã€ç§æœ‰ IP é˜»æ­¢ã€é‡å®šå‘é™åˆ¶ã€è¶…æ—¶ï¼‰ã€‚

## æ–‡ä»¶ + å›¾åƒé™åˆ¶ï¼ˆé…ç½®ï¼‰

é»˜è®¤å€¼å¯åœ¨ `gateway.http.endpoints.responses` ä¸‹è°ƒæ•´ï¼š

```json5
{
  gateway: {
    http: {
      endpoints: {
        responses: {
          enabled: true,
          maxBodyBytes: 20000000,
          files: {
            allowUrl: true,
            allowedMimes: [
              "text/plain",
              "text/markdown",
              "text/html",
              "text/csv",
              "application/json",
              "application/pdf",
            ],
            maxBytes: 5242880,
            maxChars: 200000,
            maxRedirects: 3,
            timeoutMs: 10000,
            pdf: {
              maxPages: 4,
              maxPixels: 4000000,
              minTextChars: 200,
            },
          },
          images: {
            allowUrl: true,
            allowedMimes: ["image/jpeg", "image/png", "image/gif", "image/webp"],
            maxBytes: 10485760,
            maxRedirects: 3,
            timeoutMs: 10000,
          },
        },
      },
    },
  },
}
```

çœç•¥æ—¶çš„é»˜è®¤å€¼ï¼š

- `maxBodyBytes`ï¼š20MB
- `files.maxBytes`ï¼š5MB
- `files.maxChars`ï¼š200k
- `files.maxRedirects`ï¼š3
- `files.timeoutMs`ï¼š10s
- `files.pdf.maxPages`ï¼š4
- `files.pdf.maxPixels`ï¼š4,000,000
- `files.pdf.minTextChars`ï¼š200
- `images.maxBytes`ï¼š10MB
- `images.maxRedirects`ï¼š3
- `images.timeoutMs`ï¼š10s

## æµå¼ä¼ è¾“ï¼ˆSSEï¼‰

è®¾ç½® `stream: true` æŽ¥æ”¶ Server-Sent Eventsï¼ˆSSEï¼‰ï¼š

- `Content-Type: text/event-stream`
- æ¯ä¸ªäº‹ä»¶è¡Œæ˜¯ `event: <type>` å’Œ `data: <json>`
- æµä»¥ `data: [DONE]` ç»“æŸ

å½“å‰å‘å‡ºçš„äº‹ä»¶ç±»åž‹ï¼š

- `response.created`
- `response.in_progress`
- `response.output_item.added`
- `response.content_part.added`
- `response.output_text.delta`
- `response.output_text.done`
- `response.content_part.done`
- `response.output_item.done`
- `response.completed`
- `response.failed`ï¼ˆå‡ºé”™æ—¶ï¼‰

## ç”¨é‡

å½“åº•å±‚æä¾›å•†æŠ¥å‘Šä»¤ç‰Œè®¡æ•°æ—¶ï¼Œ`usage` ä¼šè¢«å¡«å……ã€‚

## é”™è¯¯

é”™è¯¯ä½¿ç”¨å¦‚ä¸‹ JSON å¯¹è±¡ï¼š

```json
{ "error": { "message": "...", "type": "invalid_request_error" } }
```

å¸¸è§æƒ…å†µï¼š

- `401` ç¼ºå°‘/æ— æ•ˆè®¤è¯
- `400` æ— æ•ˆè¯·æ±‚ä½“
- `405` é”™è¯¯çš„æ–¹æ³•

## ç¤ºä¾‹

éžæµå¼ï¼š

```bash
curl -sS http://127.0.0.1:18789/v1/responses \
  -H 'Authorization: Bearer YOUR_TOKEN' \
  -H 'Content-Type: application/json' \
  -H 'x-krabkrab-agent-id: main' \
  -d '{
    "model": "krabkrab",
    "input": "hi"
  }'
```

æµå¼ï¼š

```bash
curl -N http://127.0.0.1:18789/v1/responses \
  -H 'Authorization: Bearer YOUR_TOKEN' \
  -H 'Content-Type: application/json' \
  -H 'x-krabkrab-agent-id: main' \
  -d '{
    "model": "krabkrab",
    "stream": true,
    "input": "hi"
  }'
```

