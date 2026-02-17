---
summary: "Run KrabKrab in a rootless Podman container"
read_when:
  - You want a containerized gateway with Podman instead of Docker
title: "Podman"
---

# Podman

Run the KrabKrab gateway in a **rootless** Podman container. Uses the same image as Docker (build from the repo [Dockerfile](https://github.com/krabkrab/krabkrab/blob/main/Dockerfile)).

## Requirements

- Podman (rootless)
- Sudo for one-time setup (create user, build image)

## Quick start

**1. One-time setup** (from repo root; creates user, builds image, installs launch script):

```bash
./setup-podman.sh
```

This also creates a minimal `~krabkrab/.krabkrab/krabkrab.json` (sets `gateway.mode="local"`) so the gateway can start without running the wizard.

By default the container is **not** installed as a systemd service, you start it manually (see below). For a production-style setup with auto-start and restarts, install it as a systemd Quadlet user service instead:

```bash
./setup-podman.sh --quadlet
```

(Or set `krabkrab_PODMAN_QUADLET=1`; use `--container` to install only the container and launch script.)

**2. Start gateway** (manual, for quick smoke testing):

```bash
./scripts/run-krabkrab-podman.sh launch
```

**3. Onboarding wizard** (e.g. to add channels or providers):

```bash
./scripts/run-krabkrab-podman.sh launch setup
```

Then open `http://127.0.0.1:18789/` and use the token from `~krabkrab/.krabkrab/.env` (or the value printed by setup).

## Systemd (Quadlet, optional)

If you ran `./setup-podman.sh --quadlet` (or `krabkrab_PODMAN_QUADLET=1`), a [Podman Quadlet](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html) unit is installed so the gateway runs as a systemd user service for the krabkrab user. The service is enabled and started at the end of setup.

- **Start:** `sudo systemctl --machine krabkrab@ --user start krabkrab.service`
- **Stop:** `sudo systemctl --machine krabkrab@ --user stop krabkrab.service`
- **Status:** `sudo systemctl --machine krabkrab@ --user status krabkrab.service`
- **Logs:** `sudo journalctl --machine krabkrab@ --user -u krabkrab.service -f`

The quadlet file lives at `~krabkrab/.config/containers/systemd/krabkrab.container`. To change ports or env, edit that file (or the `.env` it sources), then `sudo systemctl --machine krabkrab@ --user daemon-reload` and restart the service. On boot, the service starts automatically if lingering is enabled for krabkrab (setup does this when loginctl is available).

To add quadlet **after** an initial setup that did not use it, re-run: `./setup-podman.sh --quadlet`.

## The krabkrab user (non-login)

`setup-podman.sh` creates a dedicated system user `krabkrab`:

- **Shell:** `nologin` â€” no interactive login; reduces attack surface.
- **Home:** e.g. `/home/krabkrab` â€” holds `~/.krabkrab` (config, workspace) and the launch script `run-krabkrab-podman.sh`.
- **Rootless Podman:** The user must have a **subuid** and **subgid** range. Many distros assign these automatically when the user is created. If setup prints a warning, add lines to `/etc/subuid` and `/etc/subgid`:

  ```text
  krabkrab:100000:65536
  ```

  Then start the gateway as that user (e.g. from cron or systemd):

  ```bash
  sudo -u krabkrab /home/krabkrab/run-krabkrab-podman.sh
  sudo -u krabkrab /home/krabkrab/run-krabkrab-podman.sh setup
  ```

- **Config:** Only `krabkrab` and root can access `/home/krabkrab/.krabkrab`. To edit config: use the Control UI once the gateway is running, or `sudo -u krabkrab $EDITOR /home/krabkrab/.krabkrab/krabkrab.json`.

## Environment and config

- **Token:** Stored in `~krabkrab/.krabkrab/.env` as `krabkrab_GATEWAY_TOKEN`. `setup-podman.sh` and `run-krabkrab-podman.sh` generate it if missing (uses `openssl`, `python3`, or `od`).
- **Optional:** In that `.env` you can set provider keys (e.g. `GROQ_API_KEY`, `OLLAMA_API_KEY`) and other KrabKrab env vars.
- **Host ports:** By default the script maps `18789` (gateway) and `18790` (bridge). Override the **host** port mapping with `krabkrab_PODMAN_GATEWAY_HOST_PORT` and `krabkrab_PODMAN_BRIDGE_HOST_PORT` when launching.
- **Paths:** Host config and workspace default to `~krabkrab/.krabkrab` and `~krabkrab/.krabkrab/workspace`. Override the host paths used by the launch script with `krabkrab_CONFIG_DIR` and `krabkrab_WORKSPACE_DIR`.

## Useful commands

- **Logs:** With quadlet: `sudo journalctl --machine krabkrab@ --user -u krabkrab.service -f`. With script: `sudo -u krabkrab podman logs -f krabkrab`
- **Stop:** With quadlet: `sudo systemctl --machine krabkrab@ --user stop krabkrab.service`. With script: `sudo -u krabkrab podman stop krabkrab`
- **Start again:** With quadlet: `sudo systemctl --machine krabkrab@ --user start krabkrab.service`. With script: re-run the launch script or `podman start krabkrab`
- **Remove container:** `sudo -u krabkrab podman rm -f krabkrab` â€” config and workspace on the host are kept

## Troubleshooting

- **Permission denied (EACCES) on config or auth-profiles:** The container defaults to `--userns=keep-id` and runs as the same uid/gid as the host user running the script. Ensure your host `krabkrab_CONFIG_DIR` and `krabkrab_WORKSPACE_DIR` are owned by that user.
- **Gateway start blocked (missing `gateway.mode=local`):** Ensure `~krabkrab/.krabkrab/krabkrab.json` exists and sets `gateway.mode="local"`. `setup-podman.sh` creates this file if missing.
- **Rootless Podman fails for user krabkrab:** Check `/etc/subuid` and `/etc/subgid` contain a line for `krabkrab` (e.g. `krabkrab:100000:65536`). Add it if missing and restart.
- **Container name in use:** The launch script uses `podman run --replace`, so the existing container is replaced when you start again. To clean up manually: `podman rm -f krabkrab`.
- **Script not found when running as krabkrab:** Ensure `setup-podman.sh` was run so that `run-krabkrab-podman.sh` is copied to krabkrabâ€™s home (e.g. `/home/krabkrab/run-krabkrab-podman.sh`).
- **Quadlet service not found or fails to start:** Run `sudo systemctl --machine krabkrab@ --user daemon-reload` after editing the `.container` file. Quadlet requires cgroups v2: `podman info --format '{{.Host.CgroupsVersion}}'` should show `2`.

## Optional: run as your own user

To run the gateway as your normal user (no dedicated krabkrab user): build the image, create `~/.krabkrab/.env` with `krabkrab_GATEWAY_TOKEN`, and run the container with `--userns=keep-id` and mounts to your `~/.krabkrab`. The launch script is designed for the krabkrab-user flow; for a single-user setup you can instead run the `podman run` command from the script manually, pointing config and workspace to your home. Recommended for most users: use `setup-podman.sh` and run as the krabkrab user so config and process are isolated.

