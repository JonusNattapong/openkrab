---
summary: "Messaging platforms OpenKrab can connect to"
read_when:
  - You want to choose a chat channel for OpenKrab
  - You need a quick overview of supported messaging platforms
title: "Chat Channels"
---

# Chat Channels

OpenKrab can talk to you on any chat app you already use. Each channel connects via the Gateway.
Text is supported everywhere; media and reactions vary by channel.

## Supported channels

- [Telegram](/channels/telegram) — Bot API; supports groups and media.
- [Discord](/channels/discord) — Discord Bot API + Gateway; supports servers, channels, and DMs.
- [Slack](/channels/slack) — Bolt SDK; workspace apps.
- [WhatsApp](/channels/whatsapp) — Cloud API and Business API.
- [Signal](/channels/signal) — signal-cli integration; privacy-focused.
- [iMessage](/channels/imessage) — BlueBubbles bridge for macOS.
- [Matrix](/channels/matrix) — Matrix protocol support.
- [Google Chat](/channels/googlechat) — Google Chat API.
- [IRC](/channels/irc) — Classic IRC servers.
- [Mattermost](/channels/mattermost) — Bot API + WebSocket.
- [Microsoft Teams](/channels/msteams) — Bot Framework.
- [LINE](/channels/line) — LINE Messaging API.
- [Zalo](/channels/zalo) — Zalo Bot API.
- [Feishu/Lark](/channels/feishu) — Lark/Feishu bot.
- [Nextcloud Talk](/channels/nextcloud-talk) — Self-hosted chat.
- [Nostr](/channels/nostr) — Decentralized DMs.
- [Twitch](/channels/twitch) — Twitch chat via IRC.
- [WebChat](/web/webchat) — Gateway WebChat UI over WebSocket.

## Notes

- Channels can run simultaneously; configure multiple and OpenKrab will route per chat.
- Fastest setup is usually **Telegram** (simple bot token).
- Group behavior varies by channel; see [Groups](/channels/groups).
- DM pairing and allowlists are enforced for safety; see [Security](/gateway/security).
- Troubleshooting: [Channel troubleshooting](/channels/troubleshooting).
- Model providers are documented separately; see [Model Providers](/providers).
