# Deployment Guide

This guide covers all deployment methods for krabkrab.

---

## Table of Contents

- [Docker (Recommended)](#docker-recommended)
- [Docker Compose](#docker-compose)
- [Pre-built Binaries](#pre-built-binaries)
- [Build from Source](#build-from-source)
- [Cross-Compilation](#cross-compilation)
- [Raspberry Pi Setup](#raspberry-pi-setup)
- [FreeBSD / OpenBSD](#freebsd--openbsd)
- [Systemd Service](#systemd-service)
- [Environment Variables](#environment-variables)

---

## Docker (Recommended)

### Quick Start

```bash
docker run -d \
  --name krabkrab \
  -p 4120:4120 \
  -v krab-config:/data/config \
  -v krab-memory:/data/memory \
  -e OPENAI_API_KEY="sk-..." \
  ghcr.io/openkrab/openkrab:latest
```

### Build Locally

```bash
git clone https://github.com/openkrab/openkrab.git
cd openkrab
docker build -t krabkrab:local .
docker run -d -p 4120:4120 krabkrab:local
```

### Available Tags

| Tag | Description |
|-----|-------------|
| `latest` | Latest stable release |
| `edge` | Latest main branch build |
| `vX.Y.Z` | Specific version |
| `sha-XXXXXXX` | Specific commit |

### Supported Architectures

The Docker images are multi-arch and support:

- `linux/amd64` — Standard x86_64 servers
- `linux/arm64` — ARM64 servers (AWS Graviton, Apple Silicon VMs)

---

## Docker Compose

The included `docker-compose.yml` provides a production-ready setup:

```bash
# Start with defaults
docker compose up -d

# Start with custom env
OPENAI_API_KEY="sk-..." KRABKRAB_PORT=8080 docker compose up -d

# View logs
docker compose logs -f krabkrab

# Stop
docker compose down

# Stop and remove data
docker compose down -v
```

### Custom `.env` File

Create a `.env` file in the project root:

```env
# Gateway
KRABKRAB_PORT=4120
KRABKRAB_GATEWAY_TOKEN=your-secret-token
RUST_LOG=info

# AI Providers
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
OPENROUTER_API_KEY=sk-or-...

# Messaging
TELEGRAM_BOT_TOKEN=123456:ABC-DEF
SLACK_BOT_TOKEN=xoxb-...
DISCORD_TOKEN=...
MATRIX_ACCESS_TOKEN=...
```

Then run:

```bash
docker compose --env-file .env up -d
```

---

## Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/openkrab/openkrab/releases).

### Available Platforms

| Platform | Architecture | File |
|----------|-------------|------|
| Linux | x86_64 (glibc) | `krabkrab-vX.Y.Z-linux-x86_64.tar.gz` |
| Linux | x86_64 (static) | `krabkrab-vX.Y.Z-linux-x86_64-static.tar.gz` |
| Linux | ARM64 | `krabkrab-vX.Y.Z-linux-arm64.tar.gz` |
| Linux | ARMv7 (RPi) | `krabkrab-vX.Y.Z-linux-armv7.tar.gz` |
| macOS | ARM64 (Apple Silicon) | `krabkrab-vX.Y.Z-macos-arm64.tar.gz` |
| macOS | x86_64 (Intel) | `krabkrab-vX.Y.Z-macos-x86_64.tar.gz` |
| Windows | x86_64 | `krabkrab-vX.Y.Z-windows-x86_64.zip` |
| FreeBSD | x86_64 | `krabkrab-vX.Y.Z-freebsd-x86_64.tar.gz` |
| OpenBSD | x86_64 (compat) | `krabkrab-vX.Y.Z-openbsd-x86_64-compat.tar.gz` |

### Installation (Linux / macOS)

```bash
# Download and extract
VERSION="2026.2.20"
curl -LO "https://github.com/openkrab/openkrab/releases/download/v${VERSION}/krabkrab-v${VERSION}-linux-x86_64.tar.gz"

# Verify checksum
curl -LO "https://github.com/openkrab/openkrab/releases/download/v${VERSION}/krabkrab-v${VERSION}-linux-x86_64.tar.gz.sha256"
sha256sum -c "krabkrab-v${VERSION}-linux-x86_64.tar.gz.sha256"

# Extract and install
tar xzf "krabkrab-v${VERSION}-linux-x86_64.tar.gz"
sudo mv krabkrab-cli /usr/local/bin/krabkrab
chmod +x /usr/local/bin/krabkrab

# Verify
krabkrab --version
```

### Installation (Windows)

1. Download `krabkrab-vX.Y.Z-windows-x86_64.zip`
2. Extract to a folder (e.g., `C:\Program Files\krabkrab\`)
3. Add the folder to your `PATH` environment variable
4. Open a new terminal and run `krabkrab --version`

---

## Build from Source

### Prerequisites

- **Rust** 1.83+ (install from [rustup.rs](https://rustup.rs))
- **Git**
- **C compiler** (gcc/clang on Linux, Xcode CLI tools on macOS, MSVC on Windows)

### Build

```bash
git clone https://github.com/openkrab/openkrab.git
cd openkrab
cargo build --workspace --release

# Binary at: target/release/krabkrab-cli
sudo cp target/release/krabkrab-cli /usr/local/bin/krabkrab
```

### Optional Features

```bash
# Build with native plugins (libloading)
cargo build --release --features native-plugins

# Build with WASM plugins (wasmtime)
cargo build --release --features wasm-plugins

# Build without unix signals (for Windows)
cargo build --release --no-default-features
```

---

## Cross-Compilation

For cross-compiling to non-native platforms, use the [`cross`](https://github.com/cross-rs/cross) tool:

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Build for Raspberry Pi (ARMv7)
cross build --workspace --release --target armv7-unknown-linux-gnueabihf

# Build for ARM64
cross build --workspace --release --target aarch64-unknown-linux-gnu

# Build for FreeBSD
cross build --workspace --release --target x86_64-unknown-freebsd

# Static musl build
cross build --workspace --release --target x86_64-unknown-linux-musl
```

The `Cross.toml` in the repository root configures the Docker images used for each target.

---

## Raspberry Pi Setup

### Method 1: Pre-built Binary

```bash
# Download ARMv7 binary
VERSION="2026.2.20"
curl -LO "https://github.com/openkrab/openkrab/releases/download/v${VERSION}/krabkrab-v${VERSION}-linux-armv7.tar.gz"
tar xzf "krabkrab-v${VERSION}-linux-armv7.tar.gz"
sudo mv krabkrab-cli /usr/local/bin/krabkrab
chmod +x /usr/local/bin/krabkrab
```

### Method 2: Docker (RPi 3/4/5 — ARM64)

```bash
docker run -d \
  --name krabkrab \
  -p 4120:4120 \
  -v krab-data:/data \
  ghcr.io/openkrab/openkrab:latest
```

### Method 3: Cross-compile from another machine

```bash
# On your development machine (x86_64)
cross build --workspace --release --target armv7-unknown-linux-gnueabihf

# Copy to Pi
scp target/armv7-unknown-linux-gnueabihf/release/krabkrab-cli pi@raspberrypi:~/krabkrab
```

---

## FreeBSD / OpenBSD

### FreeBSD

```bash
# Download pre-built binary
VERSION="2026.2.20"
fetch "https://github.com/openkrab/openkrab/releases/download/v${VERSION}/krabkrab-v${VERSION}-freebsd-x86_64.tar.gz"
tar xzf "krabkrab-v${VERSION}-freebsd-x86_64.tar.gz"
cp krabkrab-cli /usr/local/bin/krabkrab
chmod +x /usr/local/bin/krabkrab
```

### OpenBSD

OpenBSD doesn't have a native Rust target, but you can run the static musl binary
under OpenBSD's Linux compatibility layer:

```bash
# Enable Linux compat
doas sysctl kern.emul.linux=1

# Download the static binary
VERSION="2026.2.20"
ftp "https://github.com/openkrab/openkrab/releases/download/v${VERSION}/krabkrab-v${VERSION}-openbsd-x86_64-compat.tar.gz"
tar xzf "krabkrab-v${VERSION}-openbsd-x86_64-compat.tar.gz"
cp krabkrab-cli /usr/local/bin/krabkrab
chmod +x /usr/local/bin/krabkrab
```

---

## Systemd Service

For Linux servers, create a systemd unit file:

```bash
sudo tee /etc/systemd/system/krabkrab.service << 'EOF'
[Unit]
Description=krabkrab Personal AI Assistant Gateway
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=krab
Group=krab
ExecStart=/usr/local/bin/krabkrab gateway start
Restart=on-failure
RestartSec=5
TimeoutStartSec=30

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=/var/lib/krabkrab /var/log/krabkrab
PrivateTmp=true

# Environment
Environment=RUST_LOG=info
Environment=KRABKRAB_DATA_DIR=/var/lib/krabkrab
Environment=KRABKRAB_LOG_DIR=/var/log/krabkrab
EnvironmentFile=-/etc/krabkrab/env

[Install]
WantedBy=multi-user.target
EOF
```

```bash
# Create user and directories
sudo useradd -r -s /usr/sbin/nologin krab
sudo mkdir -p /var/lib/krabkrab /var/log/krabkrab /etc/krabkrab
sudo chown krab:krab /var/lib/krabkrab /var/log/krabkrab

# Put API keys in /etc/krabkrab/env
sudo tee /etc/krabkrab/env << 'EOF'
OPENAI_API_KEY=sk-...
KRABKRAB_GATEWAY_TOKEN=your-secret-token
EOF
sudo chmod 600 /etc/krabkrab/env

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable krabkrab
sudo systemctl start krabkrab

# Check status
sudo systemctl status krabkrab
journalctl -u krabkrab -f
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `KRABKRAB_DATA_DIR` | `~/.openkrab` | Data storage root |
| `KRABKRAB_CONFIG_DIR` | `$DATA_DIR/config` | Configuration files |
| `KRABKRAB_LOG_DIR` | `$DATA_DIR/logs` | Log files |
| `KRABKRAB_GATEWAY_HOST` | `127.0.0.1` | Gateway bind address |
| `KRABKRAB_GATEWAY_PORT` | `4120` | Gateway port |
| `KRABKRAB_GATEWAY_TOKEN` | _(none)_ | Auth token for gateway |
| `RUST_LOG` | `info` | Log level (`trace`/`debug`/`info`/`warn`/`error`) |
| `OPENAI_API_KEY` | _(none)_ | OpenAI API key |
| `ANTHROPIC_API_KEY` | _(none)_ | Anthropic API key |
| `OPENROUTER_API_KEY` | _(none)_ | OpenRouter API key |
| `TELEGRAM_BOT_TOKEN` | _(none)_ | Telegram bot token |
| `SLACK_BOT_TOKEN` | _(none)_ | Slack bot token |
| `DISCORD_TOKEN` | _(none)_ | Discord bot token |
| `MATRIX_ACCESS_TOKEN` | _(none)_ | Matrix access token |

---

## Health Check

All deployment methods expose a health endpoint:

```bash
curl http://localhost:4120/health
```

Expected response: `200 OK`
