---
summary: "Run KrabKrab Gateway 24/7 on a cheap Hetzner VPS (Docker) with durable state and baked-in binaries"
read_when:
  - You want KrabKrab running 24/7 on a cloud VPS (not your laptop)
  - You want a production-grade, always-on Gateway on your own VPS
  - You want full control over persistence, binaries, and restart behavior
  - You are running KrabKrab in Docker on Hetzner or a similar provider
title: "Hetzner"
---

# KrabKrab on Hetzner (Docker, Production VPS Guide)

## Goal

Run a persistent KrabKrab Gateway on a Hetzner VPS using Docker, with durable state, baked-in binaries, and safe restart behavior.

If you want â€œKrabKrab 24/7 for ~$5â€, this is the simplest reliable setup.
Hetzner pricing changes; pick the smallest Debian/Ubuntu VPS and scale up if you hit OOMs.

## What are we doing (simple terms)?

- Rent a small Linux server (Hetzner VPS)
- Install Docker (isolated app runtime)
- Start the KrabKrab Gateway in Docker
- Persist `~/.krabkrab` + `~/.krabkrab/workspace` on the host (survives restarts/rebuilds)
- Access the Control UI from your laptop via an SSH tunnel

The Gateway can be accessed via:

- SSH port forwarding from your laptop
- Direct port exposure if you manage firewalling and tokens yourself

This guide assumes Ubuntu or Debian on Hetzner.  
If you are on another Linux VPS, map packages accordingly.
For the generic Docker flow, see [Docker](/install/docker).

---

## Quick path (experienced operators)

1. Provision Hetzner VPS
2. Install Docker
3. Clone KrabKrab repository
4. Create persistent host directories
5. Configure `.env` and `docker-compose.yml`
6. Bake required binaries into the image
7. `docker compose up -d`
8. Verify persistence and Gateway access

---

## What you need

- Hetzner VPS with root access
- SSH access from your laptop
- Basic comfort with SSH + copy/paste
- ~20 minutes
- Docker and Docker Compose
- Model auth credentials
- Optional provider credentials
  - WhatsApp QR
  - Telegram bot token
  - Gmail OAuth

---

## 1) Provision the VPS

Create an Ubuntu or Debian VPS in Hetzner.

Connect as root:

```bash
ssh root@YOUR_VPS_IP
```

This guide assumes the VPS is stateful.
Do not treat it as disposable infrastructure.

---

## 2) Install Docker (on the VPS)

```bash
apt-get update
apt-get install -y git curl ca-certificates
curl -fsSL https://get.docker.com | sh
```

Verify:

```bash
docker --version
docker compose version
```

---

## 3) Clone the KrabKrab repository

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
```

This guide assumes you will build a custom image to guarantee binary persistence.

---

## 4) Create persistent host directories

Docker containers are ephemeral.
All long-lived state must live on the host.

```bash
mkdir -p /root/.krabkrab/workspace

# Set ownership to the container user (uid 1000):
chown -R 1000:1000 /root/.krabkrab
```

---

## 5) Configure environment variables

Create `.env` in the repository root.

```bash
krabkrab_IMAGE=krabkrab:latest
krabkrab_GATEWAY_TOKEN=change-me-now
krabkrab_GATEWAY_BIND=lan
krabkrab_GATEWAY_PORT=18789

krabkrab_CONFIG_DIR=/root/.krabkrab
krabkrab_WORKSPACE_DIR=/root/.krabkrab/workspace

GOG_KEYRING_PASSWORD=change-me-now
XDG_CONFIG_HOME=/home/node/.krabkrab
```

Generate strong secrets:

```bash
openssl rand -hex 32
```

**Do not commit this file.**

---

## 6) Docker Compose configuration

Create or update `docker-compose.yml`.

```yaml
services:
  krabkrab-gateway:
    image: ${krabkrab_IMAGE}
    build: .
    restart: unless-stopped
    env_file:
      - .env
    environment:
      - HOME=/home/node
      - NODE_ENV=production
      - TERM=xterm-256color
      - krabkrab_GATEWAY_BIND=${krabkrab_GATEWAY_BIND}
      - krabkrab_GATEWAY_PORT=${krabkrab_GATEWAY_PORT}
      - krabkrab_GATEWAY_TOKEN=${krabkrab_GATEWAY_TOKEN}
      - GOG_KEYRING_PASSWORD=${GOG_KEYRING_PASSWORD}
      - XDG_CONFIG_HOME=${XDG_CONFIG_HOME}
      - PATH=/home/linuxbrew/.linuxbrew/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
    volumes:
      - ${krabkrab_CONFIG_DIR}:/home/node/.krabkrab
      - ${krabkrab_WORKSPACE_DIR}:/home/node/.krabkrab/workspace
    ports:
      # Recommended: keep the Gateway loopback-only on the VPS; access via SSH tunnel.
      # To expose it publicly, remove the `127.0.0.1:` prefix and firewall accordingly.
      - "127.0.0.1:${krabkrab_GATEWAY_PORT}:18789"
    command:
      [
        "node",
        "dist/index.js",
        "gateway",
        "--bind",
        "${krabkrab_GATEWAY_BIND}",
        "--port",
        "${krabkrab_GATEWAY_PORT}",
        "--allow-unconfigured",
      ]
```

`--allow-unconfigured` is only for bootstrap convenience, it is not a replacement for a proper gateway configuration. Still set auth (`gateway.auth.token` or password) and use safe bind settings for your deployment.

---

## 7) Bake required binaries into the image (critical)

Installing binaries inside a running container is a trap.
Anything installed at runtime will be lost on restart.

All external binaries required by skills must be installed at image build time.

The examples below show three common binaries only:

- `gog` for Gmail access
- `goplaces` for Google Places
- `wacli` for WhatsApp

These are examples, not a complete list.
You may install as many binaries as needed using the same pattern.

If you add new skills later that depend on additional binaries, you must:

1. Update the Dockerfile
2. Rebuild the image
3. Restart the containers

**Example Dockerfile**

```dockerfile
FROM node:22-bookworm

RUN apt-get update && apt-get install -y socat && rm -rf /var/lib/apt/lists/*

# Example binary 1: Gmail CLI
RUN curl -L https://github.com/steipete/gog/releases/latest/download/gog_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/gog

# Example binary 2: Google Places CLI
RUN curl -L https://github.com/steipete/goplaces/releases/latest/download/goplaces_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/goplaces

# Example binary 3: WhatsApp CLI
RUN curl -L https://github.com/steipete/wacli/releases/latest/download/wacli_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/wacli

# Add more binaries below using the same pattern

WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml .npmrc ./
COPY ui/package.json ./ui/package.json
COPY scripts ./scripts

RUN corepack enable
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build
RUN pnpm ui:install
RUN pnpm ui:build

ENV NODE_ENV=production

CMD ["node","dist/index.js"]
```

---

## 8) Build and launch

```bash
docker compose build
docker compose up -d krabkrab-gateway
```

Verify binaries:

```bash
docker compose exec krabkrab-gateway which gog
docker compose exec krabkrab-gateway which goplaces
docker compose exec krabkrab-gateway which wacli
```

Expected output:

```
/usr/local/bin/gog
/usr/local/bin/goplaces
/usr/local/bin/wacli
```

---

## 9) Verify Gateway

```bash
docker compose logs -f krabkrab-gateway
```

Success:

```
[gateway] listening on ws://0.0.0.0:18789
```

From your laptop:

```bash
ssh -N -L 18789:127.0.0.1:18789 root@YOUR_VPS_IP
```

Open:

`http://127.0.0.1:18789/`

Paste your gateway token.

---

## What persists where (source of truth)

KrabKrab runs in Docker, but Docker is not the source of truth.
All long-lived state must survive restarts, rebuilds, and reboots.

| Component           | Location                          | Persistence mechanism  | Notes                            |
| ------------------- | --------------------------------- | ---------------------- | -------------------------------- |
| Gateway config      | `/home/node/.krabkrab/`           | Host volume mount      | Includes `krabkrab.json`, tokens |
| Model auth profiles | `/home/node/.krabkrab/`           | Host volume mount      | OAuth tokens, API keys           |
| Skill configs       | `/home/node/.krabkrab/skills/`    | Host volume mount      | Skill-level state                |
| Agent workspace     | `/home/node/.krabkrab/workspace/` | Host volume mount      | Code and agent artifacts         |
| WhatsApp session    | `/home/node/.krabkrab/`           | Host volume mount      | Preserves QR login               |
| Gmail keyring       | `/home/node/.krabkrab/`           | Host volume + password | Requires `GOG_KEYRING_PASSWORD`  |
| External binaries   | `/usr/local/bin/`                 | Docker image           | Must be baked at build time      |
| Node runtime        | Container filesystem              | Docker image           | Rebuilt every image build        |
| OS packages         | Container filesystem              | Docker image           | Do not install at runtime        |
| Docker container    | Ephemeral                         | Restartable            | Safe to destroy                  |

---

## Infrastructure as Code (Terraform)

For teams preferring infrastructure-as-code workflows, a community-maintained Terraform setup provides:

- Modular Terraform configuration with remote state management
- Automated provisioning via cloud-init
- Deployment scripts (bootstrap, deploy, backup/restore)
- Security hardening (firewall, UFW, SSH-only access)
- SSH tunnel configuration for gateway access

**Repositories:**

- Infrastructure: [krabkrab-terraform-hetzner](https://github.com/andreesg/krabkrab-terraform-hetzner)
- Docker config: [krabkrab-docker-config](https://github.com/andreesg/krabkrab-docker-config)

This approach complements the Docker setup above with reproducible deployments, version-controlled infrastructure, and automated disaster recovery.

> **Note:** Community-maintained. For issues or contributions, see the repository links above.

