---
summary: "Enable real-time audio components using WebRTC for OpenClaw"
read_when:
  - You want to bridge inbound WebRTC voice calls directly to your Agent
  - You are looking to configure WebRTC signal server behavior
title: "WebRTC Voice Calls"
---

# WebRTC Voice Calls

OpenClaw supports native **WebRTC based Voice Calls**. The `webrtc` crate is embedded into the gateway server to accept and resolve SDP offers natively over endpoint `/webrtc/offer`. This allows a pure WebRTC signaling pipeline between frontend interfaces and the Agent.

With native server support, OpenClaw bypasses the need for an external VoIP bridge (like Twilio or Plivo APIs) for browser-based calls, saving complexity and cost while maximizing interaction speed.

## How it works

1. A WebRTC client (e.g. your browser or an external app) initiates an interaction by creating a standard WebRTC `RTCSessionDescription` (the "offer").
2. The client posts the JSON offer to OpenClaw's `/webrtc/offer` HTTP endpoint.
3. OpenClaw processes the offer, sets up a peer connection along with necessary media streams (such as `Opus` payload), resolving the ICE gathering.
4. The server returns a JSON signaling SDP answer to the client interface.
5. Once the stream starts, OpenClaw bridges the live audio using your configured Text-To-Speech pipeline directly into the WebRTC stream.

## Endpoint

OpenClaw hosts the `POST /webrtc/offer` endpoint inside the main Gateway HTTP server. Use this endpoint to send standard `sdp` requests:

```json
POST /webrtc/offer
Content-Type: application/json

{
  "sdp": "v=0\r\n...",
  "type": "offer"
}
```

The server responds with a serialized `sdp` answer required to start the WebRTC datachannel/audio tracks:

```json
{
  "sdp": "v=0\r\n...",
  "type": "answer"
}
```

## Security

WebRTC endpoints inherit the security constraints applied globally inside `openclaw.json` (such as checking `CORS` properties using the Gateway's internal permissive Layer config).

There is no native authentication requirement attached to the SDP offer. Ensure you place the endpoints behind a proper reverse proxy (like Cloudflare Zero Trust, Nginx with auth blocks) if exposing OpenClaw directly over the public web.
