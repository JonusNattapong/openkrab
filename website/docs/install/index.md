---
summary: "Install OpenKrab — from source, pre-built binaries, Docker, and more"
read_when:
  - You need an install method other than the Getting Started quickstart
  - You want to deploy to a cloud platform
  - You need to update, migrate, or uninstall
title: "Install"
---

# Install

Already followed [Getting Started](/start/getting-started)? You're all set — this page is for alternative install methods, platform-specific instructions, and maintenance.

## System requirements

- **Rust 1.75+** (if building from source)
- macOS, Linux, or Windows
- For pre-built binaries: No additional requirements

<Note>
On Windows, we strongly recommend running OpenKrab under [WSL2](https://learn.microsoft.com/en-us/windows/wsl/install) for best compatibility.
</Note>

## Install methods

<Tip>
Building from source is the recommended way to install OpenKrab. It provides the best performance and compatibility.
</Tip>

<AccordionGroup>
  <Accordion title="From Source" icon="github" defaultOpen>
    Clone the [OpenKrab repo](https://github.com/openkrab/openkrab) and build:

    ```bash
    git clone https://github.com/openkrab/openkrab.git
    cd openkrab
    cargo build --release
    ```

    The binary will be at `target/release/krabkrab`.

    To install globally:
    ```bash
    cargo install --path .
    ```

    Or copy the binary to your PATH:
    ```bash
    cp target/release/krabkrab /usr/local/bin/
    ```

  </Accordion>

  <Accordion title="Pre-built Binaries" icon="package">
    Download from [GitHub Releases](https://github.com/openkrab/openkrab/releases) for your platform:

    - Linux (x64, ARM64)
    - macOS (Intel, Apple Silicon)
    - Windows (x64)

    Extract and place the binary in your PATH.

  </Accordion>
</AccordionGroup>

## Build options

### Debug build (faster compile, slower runtime)
```bash
cargo build
```

### Release build (slower compile, optimized runtime)
```bash
cargo build --release
```

### Run tests
```bash
cargo test                    # All tests
cargo test --lib             # Library tests only
cargo test --release         # Release mode tests
```

### Code quality
```bash
cargo clippy                 # Linting
cargo fmt                    # Formatting
cargo doc --open             # Generate docs
```

## Other install methods

<CardGroup cols={2}>
  <Card title="Docker" href="/install/docker" icon="container">
    Containerized or headless deployments.
  </Card>
  <Card title="Podman" href="/install/podman" icon="container">
    Rootless container deployment.
  </Card>
</CardGroup>

## After install

Verify everything is working:

```bash
krabkrab doctor         # check for config issues
krabkrab status         # gateway status
krabkrab dashboard      # open the browser UI
```

If you need custom runtime paths, use:

- `KRABKRAB_CONFIG_DIR` for config directory
- `KRABKRAB_DATA_DIR` for data directory

See [Environment vars](/help/environment) for precedence and full details.

## Troubleshooting: `krabkrab` not found

<Accordion title="PATH diagnosis and fix">
  Quick diagnosis:

```bash
which krabkrab
echo "$PATH"
```

If the directory containing `krabkrab` is **not** in your `$PATH`, your shell can't find the binary.

Fix — add it to your shell startup file (`~/.zshrc` or `~/.bashrc`):

```bash
export PATH="/path/to/krabkrab:$PATH"
```

Then open a new terminal (or `source ~/.zshrc` / `source ~/.bashrc`).
</Accordion>

## Update / uninstall

<CardGroup cols={3}>
  <Card title="Updating" href="/install/updating" icon="refresh-cw">
    Keep OpenKrab up to date.
  </Card>
  <Card title="Migrating" href="/install/migrating" icon="arrow-right">
    Move to a new machine.
  </Card>
  <Card title="Uninstall" href="/install/uninstall" icon="trash-2">
    Remove OpenKrab completely.
  </Card>
</CardGroup>
