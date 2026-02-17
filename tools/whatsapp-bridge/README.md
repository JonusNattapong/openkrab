WhatsApp Bridge (Node + Baileys)
================================

Purpose
-------
Provide a lightweight Node-based bridge using the Baileys WhatsApp Web library. The bridge exposes a simple RPC over Unix domain socket (or named pipe on Windows) to allow the Rust gateway to send/receive messages without embedding WhatsApp logic in Rust.

Design
------
- Node process runs Baileys and maintains WhatsApp session.
- Exposes an IPC endpoint (gRPC over Unix socket / named pipe or simple JSON-over-socket) with methods:
  - sendMessage(channel, chatId, content)
  - onMessage(callback)
  - status()
- The Rust gateway will call the bridge via a small client library (IPC) to send messages and receive incoming messages forwarded to storage/agents.

Development
-----------
Create a new Node project (TypeScript) under `tools/whatsapp-bridge/` using `baileys` and `@grpc/grpc-js` or a lightweight Unix-socket JSON transport. Provide example config and scripts to run and autoconnect.

Next steps
----------
1. Initialize Node project with `npm init -y` and TypeScript tooling.
2. Implement IPC protocol (proto file if using gRPC).
3. Add example `bridge-client` in Rust using sockets if not gRPC.
