---
read_when:
  - é›†æˆéœ€è¦ OpenAI Chat Completions çš„å·¥å…·
summary: ä»Ž Gateway ç½‘å…³æš´éœ² OpenAI å…¼å®¹çš„ /v1/chat/completions HTTP ç«¯ç‚¹
title: OpenAI Chat Completions
x-i18n:
  generated_at: "2026-02-03T07:48:15Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 6f935777f489bff925a3bf18b1e4b7493f83ae7b1e581890092e5779af59b732
  source_path: gateway/openai-http-api.md
  workflow: 15
---

# OpenAI Chat Completionsï¼ˆHTTPï¼‰

KrabKrab çš„ Gateway ç½‘å…³å¯ä»¥æä¾›ä¸€ä¸ªå°åž‹çš„ OpenAI å…¼å®¹ Chat Completions ç«¯ç‚¹ã€‚

æ­¤ç«¯ç‚¹**é»˜è®¤ç¦ç”¨**ã€‚è¯·å…ˆåœ¨é…ç½®ä¸­å¯ç”¨å®ƒã€‚

- `POST /v1/chat/completions`
- ä¸Ž Gateway ç½‘å…³ç›¸åŒçš„ç«¯å£ï¼ˆWS + HTTP å¤šè·¯å¤ç”¨ï¼‰ï¼š`http://<gateway-host>:<port>/v1/chat/completions`

åº•å±‚å®žçŽ°ä¸­ï¼Œè¯·æ±‚ä½œä¸ºæ™®é€šçš„ Gateway ç½‘å…³æ™ºèƒ½ä½“è¿è¡Œæ‰§è¡Œï¼ˆä¸Ž `krabkrab agent` ç›¸åŒçš„ä»£ç è·¯å¾„ï¼‰ï¼Œå› æ­¤è·¯ç”±/æƒé™/é…ç½®ä¸Žä½ çš„ Gateway ç½‘å…³ä¸€è‡´ã€‚

## è®¤è¯

ä½¿ç”¨ Gateway ç½‘å…³è®¤è¯é…ç½®ã€‚å‘é€ bearer ä»¤ç‰Œï¼š

- `Authorization: Bearer <token>`

æ³¨æ„äº‹é¡¹ï¼š

- å½“ `gateway.auth.mode="token"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- å½“ `gateway.auth.mode="password"` æ—¶ï¼Œä½¿ç”¨ `gateway.auth.password`ï¼ˆæˆ– `krabkrab_GATEWAY_PASSWORD`ï¼‰ã€‚

## é€‰æ‹©æ™ºèƒ½ä½“

æ— éœ€è‡ªå®šä¹‰å¤´ï¼šåœ¨ OpenAI `model` å­—æ®µä¸­ç¼–ç æ™ºèƒ½ä½“ IDï¼š

- `model: "krabkrab:<agentId>"`ï¼ˆä¾‹å¦‚ï¼š`"krabkrab:main"`ã€`"krabkrab:beta"`ï¼‰
- `model: "agent:<agentId>"`ï¼ˆåˆ«åï¼‰

æˆ–é€šè¿‡å¤´æŒ‡å®šç‰¹å®šçš„ KrabKrab æ™ºèƒ½ä½“ï¼š

- `x-krabkrab-agent-id: <agentId>`ï¼ˆé»˜è®¤ï¼š`main`ï¼‰

é«˜çº§é€‰é¡¹ï¼š

- `x-krabkrab-session-key: <sessionKey>` å®Œå…¨æŽ§åˆ¶ä¼šè¯è·¯ç”±ã€‚

## å¯ç”¨ç«¯ç‚¹

å°† `gateway.http.endpoints.chatCompletions.enabled` è®¾ç½®ä¸º `true`ï¼š

```json5
{
  gateway: {
    http: {
      endpoints: {
        chatCompletions: { enabled: true },
      },
    },
  },
}
```

## ç¦ç”¨ç«¯ç‚¹

å°† `gateway.http.endpoints.chatCompletions.enabled` è®¾ç½®ä¸º `false`ï¼š

```json5
{
  gateway: {
    http: {
      endpoints: {
        chatCompletions: { enabled: false },
      },
    },
  },
}
```

## ä¼šè¯è¡Œä¸º

é»˜è®¤æƒ…å†µä¸‹ï¼Œç«¯ç‚¹æ˜¯**æ¯è¯·æ±‚æ— çŠ¶æ€**çš„ï¼ˆæ¯æ¬¡è°ƒç”¨ç”Ÿæˆæ–°çš„ä¼šè¯é”®ï¼‰ã€‚

å¦‚æžœè¯·æ±‚åŒ…å« OpenAI `user` å­—ç¬¦ä¸²ï¼ŒGateway ç½‘å…³ä¼šä»Žä¸­æ´¾ç”Ÿä¸€ä¸ªç¨³å®šçš„ä¼šè¯é”®ï¼Œå› æ­¤é‡å¤è°ƒç”¨å¯ä»¥å…±äº«æ™ºèƒ½ä½“ä¼šè¯ã€‚

## æµå¼ä¼ è¾“ï¼ˆSSEï¼‰

è®¾ç½® `stream: true` ä»¥æŽ¥æ”¶ Server-Sent Eventsï¼ˆSSEï¼‰ï¼š

- `Content-Type: text/event-stream`
- æ¯ä¸ªäº‹ä»¶è¡Œæ˜¯ `data: <json>`
- æµä»¥ `data: [DONE]` ç»“æŸ

## ç¤ºä¾‹

éžæµå¼ï¼š

```bash
curl -sS http://127.0.0.1:18789/v1/chat/completions \
  -H 'Authorization: Bearer YOUR_TOKEN' \
  -H 'Content-Type: application/json' \
  -H 'x-krabkrab-agent-id: main' \
  -d '{
    "model": "krabkrab",
    "messages": [{"role":"user","content":"hi"}]
  }'
```

æµå¼ï¼š

```bash
curl -N http://127.0.0.1:18789/v1/chat/completions \
  -H 'Authorization: Bearer YOUR_TOKEN' \
  -H 'Content-Type: application/json' \
  -H 'x-krabkrab-agent-id: main' \
  -d '{
    "model": "krabkrab",
    "stream": true,
    "messages": [{"role":"user","content":"hi"}]
  }'
```

