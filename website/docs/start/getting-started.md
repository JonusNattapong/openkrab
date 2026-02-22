---
summary: "Get OpenKrab installed and run your first chat in minutes."
read_when:
  - First time setup from zero
  - You want the fastest path to a working chat
title: "Getting Started"
---

# Getting Started

Goal: go from zero to a first working chat with minimal setup.

<Info>
Fastest chat: open the Web Dashboard (no channel setup needed). Run `krabkrab dashboard`
and chat in the browser, or open `http://127.0.0.1:18789/` on the
<Tooltip headline="Gateway host" tip="The machine running the OpenKrab gateway service.">gateway host</Tooltip>.
Docs: [Dashboard](/web/dashboard).
</Info>

## Prereqs

- Rust 1.75+ (for building from source)
- Or download pre-built binary

<Tip>
Check your Rust version with `rustc --version` if building from source.
</Tip>

## Quick setup (CLI)

<Steps>
  <Step title="Install OpenKrab">
    <Tabs>
      <Tab title="From Source">
        ```bash
        git clone https://github.com/openkrab/openkrab.git
        cd openkrab
        cargo build --release
        ```
        <Note>
        Binary will be at `target/release/krabkrab`
        </Note>
      </Tab>
      <Tab title="Pre-built Binary">
        Download from [GitHub Releases](https://github.com/openkrab/openkrab/releases) for your platform:
        - Linux (x64, ARM64)
        - macOS (Intel, Apple Silicon)
        - Windows (x64)
      </Tab>
    </Tabs>

    <Note>
    Other install methods: [Install](/install).
    </Note>

  </Step>
  <Step title="Configure your AI provider">
    ```bash
    krabkrab config set providers.openai.api_key "sk-..."
    ```

    Or use other providers (Anthropic, Gemini, Ollama, etc.)

  </Step>
  <Step title="Start the Gateway">
    ```bash
    krabkrab gateway --port 18789
    ```

  </Step>
  <Step title="Open the Dashboard">
    ```bash
    krabkrab dashboard
    ```
    Or open `http://127.0.0.1:18789/` in your browser.
  </Step>
</Steps>

<Check>
If the Dashboard loads, your Gateway is ready for use.
</Check>

## Optional checks and extras

<AccordionGroup>
  <Accordion title="Send a test message">
    Requires a configured channel.

    ```bash
    krabkrab telegram --to @username --text "Hello from OpenKrab!"
    krabkrab ask "What's the weather today?"
    ```

  </Accordion>
  <Accordion title="Check system status">
    ```bash
    krabkrab doctor         # check for config issues
    krabkrab status         # gateway status
    ```
  </Accordion>
</AccordionGroup>

## Useful environment variables

If you want custom config/state locations:

- `KRABKRAB_CONFIG_DIR` sets the config directory (default: `~/.config/krabkrab/`)
- `KRABKRAB_DATA_DIR` sets the data directory

Full environment variable reference: [Environment vars](/help/environment).

## Go deeper

<Columns>
  <Card title="Configuration" href="/gateway/configuration">
    Core Gateway settings, tokens, and provider config.
  </Card>
  <Card title="Channels" href="/channels">
    Connect Telegram, Slack, Discord, WhatsApp, and more.
  </Card>
  <Card title="Voice System" href="/tools/voice">
    Setup wake word detection and talk mode.
  </Card>
</Columns>

## What you will have

- A running Gateway
- AI provider configured
- Dashboard access or a connected channel

## Next steps

- DM safety and approvals: [Pairing](/channels/pairing)
- Connect more channels: [Channels](/channels)
- Memory system: [Memory](/concepts/memory)
- Plugin system: [Plugins](/tools/plugin)
