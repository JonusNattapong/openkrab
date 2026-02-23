---
summary: "Uninstall OpenKrab completely (CLI, service, state, workspace)"
read_when:
  - You want to remove OpenKrab from a machine
  - The gateway service is still running after uninstall
title: "Uninstall"
---

# Uninstall

Two paths:

- **Easy path** if `OpenKrab` is still installed.
- **Manual service removal** if the CLI is gone but the service is still running.

## Easy path (CLI still installed)

Recommended: use the built-in uninstaller:

```bash
OpenKrab uninstall
```

Non-interactive (automation / npx):

```bash
OpenKrab uninstall --all --yes --non-interactive
npx -y OpenKrab uninstall --all --yes --non-interactive
```

Manual steps (same result):

1. Stop the gateway service:

```bash
OpenKrab gateway stop
```

2. Uninstall the gateway service (launchd/systemd/schtasks):

```bash
OpenKrab gateway uninstall
```

3. Delete state + config:

```bash
rm -rf "${OPENKRAB_STATE_DIR:-$HOME/.OpenKrab}"
```

If you set `OPENKRAB_CONFIG_PATH` to a custom location outside the state dir, delete that file too.

4. Delete your workspace (optional, removes agent files):

```bash
rm -rf ~/.OpenKrab/workspace
```

5. Remove the CLI install (pick the one you used):

```bash
npm rm -g OpenKrab
pnpm remove -g OpenKrab
bun remove -g OpenKrab
```

6. If you installed the macOS app:

```bash
rm -rf /Applications/OpenKrab.app
```

Notes:

- If you used profiles (`--profile` / `OPENKRAB_PROFILE`), repeat step 3 for each state dir (defaults are `~/.OpenKrab-<profile>`).
- In remote mode, the state dir lives on the **gateway host**, so run steps 1-4 there too.

## Manual service removal (CLI not installed)

Use this if the gateway service keeps running but `OpenKrab` is missing.

### macOS (launchd)

Default label is `bot.molt.gateway` (or `bot.molt.<profile>`; legacy `com.OpenKrab.*` may still exist):

```bash
launchctl bootout gui/$UID/bot.molt.gateway
rm -f ~/Library/LaunchAgents/bot.molt.gateway.plist
```

If you used a profile, replace the label and plist name with `bot.molt.<profile>`. Remove any legacy `com.OpenKrab.*` plists if present.

### Linux (systemd user unit)

Default unit name is `OpenKrab-gateway.service` (or `OpenKrab-gateway-<profile>.service`):

```bash
systemctl --user disable --now OpenKrab-gateway.service
rm -f ~/.config/systemd/user/OpenKrab-gateway.service
systemctl --user daemon-reload
```

### Windows (Scheduled Task)

Default task name is `OpenKrab Gateway` (or `OpenKrab Gateway (<profile>)`).
The task script lives under your state dir.

```powershell
schtasks /Delete /F /TN "OpenKrab Gateway"
Remove-Item -Force "$env:USERPROFILE\.OpenKrab\gateway.cmd"
```

If you used a profile, delete the matching task name and `~\.OpenKrab-<profile>\gateway.cmd`.

## Normal install vs source checkout

### Normal install (install.sh / npm / pnpm / bun)

If you used `https://OpenKrab.ai/install.sh` or `install.ps1`, the CLI was installed with `npm install -g OpenKrab@latest`.
Remove it with `npm rm -g OpenKrab` (or `pnpm remove -g` / `bun remove -g` if you installed that way).

### Source checkout (git clone)

If you run from a repo checkout (`git clone` + `OpenKrab ...` / `bun run OpenKrab ...`):

1. Uninstall the gateway service **before** deleting the repo (use the easy path above or manual service removal).
2. Delete the repo directory.
3. Remove state + workspace as shown above.


