---
summary: "为 OpenClaw 启用借助 WebRTC 代理构建的实时语音组件"
read_when:
  - 你希望通过 WebRTC 语音通道绕过普通代理并将端点关联到 Agent
  - 你希望了解如何发起呼叫和接收网关 SDP 回应
title: "WebRTC 语音呼叫"
---

# WebRTC 语音呼叫

OpenClaw 目前原生支持 **基于 WebRTC 的语音呼叫 (Voice Calls)**。`webrtc` crate 被内置到了网关服务器之中，主要用于在端点 `/webrtc/offer` 接收并处理原生的 SDP 请求提案。它将用于为前端应用以及代理客户端创建一个可靠、高并发的 WebRTC 纯信令管道。

通过原生处理 SDP 功能，OpenClaw 不再需要借助或绑定外部的 VoIP 网络（如 Twilio、Plivo）来建立基于浏览器的呼叫节点。它显著减少了项目延迟与开销。

## 它的运作方式

1. 一个 WebRTC 客户端（你的本地浏览器或外部端点）通过构建 WebRTC `RTCSessionDescription` 来启动连接提案。
2. 客户端向 OpenClaw 发送 `POST /webrtc/offer` 的 HTTP API 端口的请求（使用 JSON 返回包含 `sdp` 回应）。
3. 你的服务器收到提案后分配一个新的 WebRTC Peer 媒体音频源（并设定解码至 `Opus`）。
4. 解析完成 ICE 后服务器将提供 JSON 信令（SDP Answer）以便你的客户端能顺利握手并收发音轨流。
5. 连接一旦启动，OpenClaw 会接入基于 TTS 引擎获取到的声音并在 WebRTC 音轨下双频返回。

## 访问端点

在启动 OpenClaw 网关时，系统会自动发布位于主服务器的 `POST /webrtc/offer` 服务。

你可以直接使用该 API 端口传输标准的 JSON `sdp` 请求以交换签名：

```json
POST /webrtc/offer
Content-Type: application/json

{
  "sdp": "v=0\r\n...",
  "type": "offer"
}
```

响应包同样是一个带有音频握手所需字段的 JSON 请求：

```json
{
  "sdp": "v=0\r\n...",
  "type": "answer"
}
```

## 安全准则

由于基于 WebRTC 的本地端口无需传递额外凭证，该操作当前将默认继承全双工中设置在 `openclaw.json` (或系统层面上宽容模式下建立的 `CORS` 等规则)。

如果系统运行在公网上，必须确保你利用了可靠的 Nginx 反向代理校验规则或如 Cloudflare Zero Trust 墙在建立 WebRTC 会话前对其接口进行了限制。
