# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 2026.2.19 | ✅ |
| < 2026.2.19 | ❌ (development versions) |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email: security@openkrab.dev (or open a private security advisory on GitHub)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and provide a timeline for the fix.

## Security Features

### DM Access Control

krabkrab connects to real messaging surfaces. Treat inbound DMs as **untrusted input**.

Default behavior on Telegram/WhatsApp/Signal/Discord/Slack:

- **DM pairing** (`dm_policy = "pairing"`): Unknown senders receive a pairing code
- Approve with: `krabkrab pairing approve <channel> <code>`
- Public DMs require explicit opt-in: `dm_policy = "open"` with `"*"` in allowlist

### Authentication

- **PKCE OAuth 2.0**: Secure code exchange with SHA-256 challenge
- **Webhook signatures**: Verified for LINE, Slack, and others
- **Token caching**: Secure file-based storage with restricted permissions

### Secrets Management

- **Environment variables**: Preferred for sensitive tokens
- **Config files**: Should have restricted permissions (600)
- **Never commit secrets**: Use `.env` files (gitignored)

## Security Best Practices

### Configuration

```toml
# Restrict DM access
[channels.discord]
dm_policy = "pairing"  # or "closed" for maximum security

[channels.telegram]
allow_from = ["your_user_id"]  # Whitelist specific users
```

### Deployment

1. **Run as non-root user**
2. **Restrict config file permissions**: `chmod 600 ~/.config/krabkrab/krabkrab.toml`
3. **Use environment variables for secrets**
4. **Keep the binary updated**
5. **Monitor logs for suspicious activity**

### Gateway Security

- Default bind: `127.0.0.1` (loopback only)
- For remote access, use:
  - Tailscale Serve/Funnel
  - SSH tunnels
  - VPN
- Never expose directly to the internet without authentication

## Known Security Considerations

### Intentional Design Decisions

1. **Main session has full tool access**: By default, the `main` session can run any tool including `bash`. This is intentional for a personal assistant.

2. **Group safety**: Set `agents.defaults.sandbox.mode = "non-main"` to run non-main sessions in Docker sandboxes.

3. **Web UI**: Designed for local use only. Not hardened for public internet exposure.

### Sandbox Configuration

For production deployments with group/channel access:

```toml
[agents.defaults.sandbox]
mode = "non-main"  # Sandbox group/channel sessions

[agents.defaults.sandbox.allow]
tools = ["bash", "read", "write", "edit"]

[agents.defaults.sandbox.deny]
tools = ["browser", "canvas", "nodes"]
```

## Security Checklist

Before deploying:

- [ ] Config file has restricted permissions (600)
- [ ] Secrets stored in environment variables
- [ ] DM policy configured appropriately
- [ ] Gateway bound to loopback (default)
- [ ] Remote access uses secure tunnel (Tailscale/SSH)
- [ ] Regular updates applied

## Audit History

| Date | Auditor | Scope | Result |
|------|---------|-------|--------|
| 2026-02-19 | Internal | Initial release review | Pass |

## Contact

- Security issues: security@openkrab.dev
- General issues: https://github.com/openkrab/krabkrab/issues
