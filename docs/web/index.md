---
summary: "Gateway web surfaces: Dashboard, bind modes, and security"
read_when:
  - You want to access the Gateway over a browser
  - You want the web dashboard and config editing
title: "Web"
---

# Web (Gateway)

The Gateway serves a **browser Dashboard** from the same port as the Gateway WebSocket:

- default: `http://<host>:18789/`

## Capabilities

- Chat interface with AI agent
- Configuration editor
- Session management
- Channel status
- Memory search
- Voice controls

## Tailscale Access

### Integrated Serve (recommended)

Keep the Gateway on loopback and let Tailscale Serve proxy it:

```toml
[gateway]
bind = "127.0.0.1"
port = 18789

[gateway.tailscale]
mode = "serve"
```

Then start the gateway:

```bash
krabkrab gateway
```

Open:

- `https://<magicdns>/`

### Tailnet Bind

```toml
[gateway]
bind = "100.x.x.x"  # your tailscale IP
port = 18789
```

### Public Internet (Funnel)

```toml
[gateway]
bind = "127.0.0.1"

[gateway.tailscale]
mode = "funnel"

[gateway.auth]
mode = "password"
password = "your-secure-password"
```

## Security Notes

- Gateway auth is required by default (token/password)
- Non-loopback binds require a shared token/password
- The UI sends authentication on WebSocket connect
- Set `gateway.auth.allow_tailscale = true` to use Tailscale identity

## Dashboard Access

Open the dashboard:

```bash
krabkrab dashboard
```

Or manually visit `http://127.0.0.1:18789/`

## WebChat

The WebChat interface allows browser-based messaging:

- Connect to Gateway WebSocket
- Send/receive messages
- View conversation history
- Upload media

See [WebChat](/web/webchat) for details.
