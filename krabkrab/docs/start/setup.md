---
summary: "Advanced setup and development workflows for KrabKrab"
read_when:
  - Setting up a new machine
  - You want “latest + greatest” without breaking your personal setup
title: "Setup"
---

# Setup

<Note>
If you are setting up for the first time, start with [Getting Started](/start/getting-started).
For wizard details, see [Onboarding Wizard](/start/wizard).
</Note>

Last updated: 2026-01-01

## TL;DR

- **Tailoring lives outside the repo:** `~/.krabkrab/workspace` (workspace) + `~/.krabkrab/krabkrab.json` (config).
- **Stable workflow:** install the macOS app; let it run the bundled Gateway.
- **Bleeding edge workflow:** run the Gateway yourself via `pnpm gateway:watch`, then let the macOS app attach in Local mode.

## Prereqs (from source)

- Node `>=22`
- `pnpm`
- Docker (optional; only for containerized setup/e2e — see [Docker](/install/docker))

## Tailoring strategy (so updates don’t hurt)

If you want “100% tailored to me” _and_ easy updates, keep your customization in:

- **Config:** `~/.krabkrab/krabkrab.json` (JSON/JSON5-ish)
- **Workspace:** `~/.krabkrab/workspace` (skills, prompts, memories; make it a private git repo)

Bootstrap once:

```bash
krabkrab setup
```

From inside this repo, use the local CLI entry:

```bash
krabkrab setup
```

If you don’t have a global install yet, run it via `pnpm krabkrab setup`.

## Run the Gateway from this repo

After `pnpm build`, you can run the packaged CLI directly:

```bash
node krabkrab.mjs gateway --port 18789 --verbose
```

## Stable workflow (macOS app first)

1. Install + launch **KrabKrab.app** (menu bar).
2. Complete the onboarding/permissions checklist (TCC prompts).
3. Ensure Gateway is **Local** and running (the app manages it).
4. Link surfaces (example: WhatsApp):

```bash
krabkrab channels login
```

5. Sanity check:

```bash
krabkrab health
```

If onboarding is not available in your build:

- Run `krabkrab setup`, then `krabkrab channels login`, then start the Gateway manually (`krabkrab gateway`).

## Bleeding edge workflow (Gateway in a terminal)

Goal: work on the TypeScript Gateway, get hot reload, keep the macOS app UI attached.

### 0) (Optional) Run the macOS app from source too

If you also want the macOS app on the bleeding edge:

```bash
./scripts/restart-mac.sh
```

### 1) Start the dev Gateway

```bash
pnpm install
pnpm gateway:watch
```

`gateway:watch` runs the gateway in watch mode and reloads on TypeScript changes.

### 2) Point the macOS app at your running Gateway

In **KrabKrab.app**:

- Connection Mode: **Local**
  The app will attach to the running gateway on the configured port.

### 3) Verify

- In-app Gateway status should read **“Using existing gateway …”**
- Or via CLI:

```bash
krabkrab health
```

### Common footguns

- **Wrong port:** Gateway WS defaults to `ws://127.0.0.1:18789`; keep app + CLI on the same port.
- **Where state lives:**
  - Credentials: `~/.krabkrab/credentials/`
  - Sessions: `~/.krabkrab/agents/<agentId>/sessions/`
  - Logs: `/tmp/krabkrab/`

## Credential storage map

Use this when debugging auth or deciding what to back up:

- **WhatsApp**: `~/.krabkrab/credentials/whatsapp/<accountId>/creds.json`
- **Telegram bot token**: config/env or `channels.telegram.tokenFile`
- **Discord bot token**: config/env (token file not yet supported)
- **Slack tokens**: config/env (`channels.slack.*`)
- **Pairing allowlists**: `~/.krabkrab/credentials/<channel>-allowFrom.json`
- **Model auth profiles**: `~/.krabkrab/agents/<agentId>/agent/auth-profiles.json`
- **Legacy OAuth import**: `~/.krabkrab/credentials/oauth.json`
  More detail: [Security](/gateway/security#credential-storage-map).

## Updating (without wrecking your setup)

- Keep `~/.krabkrab/workspace` and `~/.krabkrab/` as “your stuff”; don’t put personal prompts/config into the `krabkrab` repo.
- Updating source: `git pull` + `pnpm install` (when lockfile changed) + keep using `pnpm gateway:watch`.

## Linux (systemd user service)

Linux installs use a systemd **user** service. By default, systemd stops user
services on logout/idle, which kills the Gateway. Onboarding attempts to enable
lingering for you (may prompt for sudo). If it’s still off, run:

```bash
sudo loginctl enable-linger $USER
```

For always-on or multi-user servers, consider a **system** service instead of a
user service (no lingering needed). See [Gateway runbook](/gateway) for the systemd notes.

## Related docs

- [Gateway runbook](/gateway) (flags, supervision, ports)
- [Gateway configuration](/gateway/configuration) (config schema + examples)
- [Discord](/channels/discord) and [Telegram](/channels/telegram) (reply tags + replyToMode settings)
- [KrabKrab assistant setup](/start/krabkrab)
- [macOS app](/platforms/macos) (gateway lifecycle)
