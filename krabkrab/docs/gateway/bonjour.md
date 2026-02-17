---
summary: "Bonjour/mDNS discovery + debugging (Gateway beacons, clients, and common failure modes)"
read_when:
  - Debugging Bonjour discovery issues on macOS/iOS
  - Changing mDNS service types, TXT records, or discovery UX
title: "Bonjour Discovery"
---

# Bonjour / mDNS discovery

KrabKrab uses Bonjour (mDNS / DNSâ€‘SD) as a **LANâ€‘only convenience** to discover
an active Gateway (WebSocket endpoint). It is bestâ€‘effort and does **not** replace SSH or
Tailnet-based connectivity.

## Wideâ€‘area Bonjour (Unicast DNSâ€‘SD) over Tailscale

If the node and gateway are on different networks, multicast mDNS wonâ€™t cross the
boundary. You can keep the same discovery UX by switching to **unicast DNSâ€‘SD**
("Wideâ€‘Area Bonjour") over Tailscale.

Highâ€‘level steps:

1. Run a DNS server on the gateway host (reachable over Tailnet).
2. Publish DNSâ€‘SD records for `_krabkrab-gw._tcp` under a dedicated zone
   (example: `krabkrab.internal.`).
3. Configure Tailscale **split DNS** so your chosen domain resolves via that
   DNS server for clients (including iOS).

KrabKrab supports any discovery domain; `krabkrab.internal.` is just an example.
iOS/Android nodes browse both `local.` and your configured wideâ€‘area domain.

### Gateway config (recommended)

```json5
{
  gateway: { bind: "tailnet" }, // tailnet-only (recommended)
  discovery: { wideArea: { enabled: true } }, // enables wide-area DNS-SD publishing
}
```

### Oneâ€‘time DNS server setup (gateway host)

```bash
krabkrab dns setup --apply
```

This installs CoreDNS and configures it to:

- listen on port 53 only on the gatewayâ€™s Tailscale interfaces
- serve your chosen domain (example: `krabkrab.internal.`) from `~/.krabkrab/dns/<domain>.db`

Validate from a tailnetâ€‘connected machine:

```bash
dns-sd -B _krabkrab-gw._tcp krabkrab.internal.
dig @<TAILNET_IPV4> -p 53 _krabkrab-gw._tcp.krabkrab.internal PTR +short
```

### Tailscale DNS settings

In the Tailscale admin console:

- Add a nameserver pointing at the gatewayâ€™s tailnet IP (UDP/TCP 53).
- Add split DNS so your discovery domain uses that nameserver.

Once clients accept tailnet DNS, iOS nodes can browse
`_krabkrab-gw._tcp` in your discovery domain without multicast.

### Gateway listener security (recommended)

The Gateway WS port (default `18789`) binds to loopback by default. For LAN/tailnet
access, bind explicitly and keep auth enabled.

For tailnetâ€‘only setups:

- Set `gateway.bind: "tailnet"` in `~/.krabkrab/krabkrab.json`.
- Restart the Gateway (or restart the macOS menubar app).

## What advertises

Only the Gateway advertises `_krabkrab-gw._tcp`.

## Service types

- `_krabkrab-gw._tcp` â€” gateway transport beacon (used by macOS/iOS/Android nodes).

## TXT keys (nonâ€‘secret hints)

The Gateway advertises small nonâ€‘secret hints to make UI flows convenient:

- `role=gateway`
- `displayName=<friendly name>`
- `lanHost=<hostname>.local`
- `gatewayPort=<port>` (Gateway WS + HTTP)
- `gatewayTls=1` (only when TLS is enabled)
- `gatewayTlsSha256=<sha256>` (only when TLS is enabled and fingerprint is available)
- `canvasPort=<port>` (only when the canvas host is enabled; currently the same as `gatewayPort`)
- `sshPort=<port>` (defaults to 22 when not overridden)
- `transport=gateway`
- `cliPath=<path>` (optional; absolute path to a runnable `krabkrab` entrypoint)
- `tailnetDns=<magicdns>` (optional hint when Tailnet is available)

Security notes:

- Bonjour/mDNS TXT records are **unauthenticated**. Clients must not treat TXT as authoritative routing.
- Clients should route using the resolved service endpoint (SRV + A/AAAA). Treat `lanHost`, `tailnetDns`, `gatewayPort`, and `gatewayTlsSha256` as hints only.
- TLS pinning must never allow an advertised `gatewayTlsSha256` to override a previously stored pin.
- iOS/Android nodes should treat discovery-based direct connects as **TLS-only** and require explicit user confirmation before trusting a first-time fingerprint.

## Debugging on macOS

Useful builtâ€‘in tools:

- Browse instances:

  ```bash
  dns-sd -B _krabkrab-gw._tcp local.
  ```

- Resolve one instance (replace `<instance>`):

  ```bash
  dns-sd -L "<instance>" _krabkrab-gw._tcp local.
  ```

If browsing works but resolving fails, youâ€™re usually hitting a LAN policy or
mDNS resolver issue.

## Debugging in Gateway logs

The Gateway writes a rolling log file (printed on startup as
`gateway log file: ...`). Look for `bonjour:` lines, especially:

- `bonjour: advertise failed ...`
- `bonjour: ... name conflict resolved` / `hostname conflict resolved`
- `bonjour: watchdog detected non-announced service ...`

## Debugging on iOS node

The iOS node uses `NWBrowser` to discover `_krabkrab-gw._tcp`.

To capture logs:

- Settings â†’ Gateway â†’ Advanced â†’ **Discovery Debug Logs**
- Settings â†’ Gateway â†’ Advanced â†’ **Discovery Logs** â†’ reproduce â†’ **Copy**

The log includes browser state transitions and resultâ€‘set changes.

## Common failure modes

- **Bonjour doesnâ€™t cross networks**: use Tailnet or SSH.
- **Multicast blocked**: some Wiâ€‘Fi networks disable mDNS.
- **Sleep / interface churn**: macOS may temporarily drop mDNS results; retry.
- **Browse works but resolve fails**: keep machine names simple (avoid emojis or
  punctuation), then restart the Gateway. The service instance name derives from
  the host name, so overly complex names can confuse some resolvers.

## Escaped instance names (`\032`)

Bonjour/DNSâ€‘SD often escapes bytes in service instance names as decimal `\DDD`
sequences (e.g. spaces become `\032`).

- This is normal at the protocol level.
- UIs should decode for display (iOS uses `BonjourEscapes.decode`).

## Disabling / configuration

- `krabkrab_DISABLE_BONJOUR=1` disables advertising (legacy: `krabkrab_DISABLE_BONJOUR`).
- `gateway.bind` in `~/.krabkrab/krabkrab.json` controls the Gateway bind mode.
- `krabkrab_SSH_PORT` overrides the SSH port advertised in TXT (legacy: `krabkrab_SSH_PORT`).
- `krabkrab_TAILNET_DNS` publishes a MagicDNS hint in TXT (legacy: `krabkrab_TAILNET_DNS`).
- `krabkrab_CLI_PATH` overrides the advertised CLI path (legacy: `krabkrab_CLI_PATH`).

## Related docs

- Discovery policy and transport selection: [Discovery](/gateway/discovery)
- Node pairing + approvals: [Gateway pairing](/gateway/pairing)

