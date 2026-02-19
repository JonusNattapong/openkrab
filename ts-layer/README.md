# krabkrab TS Layer

This directory provides the JavaScript fallback runtime used by Rust bridge commands.

## Runtime

- Entry launcher: `.claude/js-bridge.mjs`
- Runtime executable: `ts-layer/dist/bridge-runtime.js`
- TypeScript source: `ts-layer/src/bridge-runtime.ts`

Bridge requests are passed in `KRABKRAB_BRIDGE_REQUEST` as a JSON object.

## Optional External Feature Commands

You can wire real JS implementations per feature by setting env vars:

- `KRABKRAB_JS_BROWSER_CMD`
- `KRABKRAB_JS_CANVAS_CMD`
- `KRABKRAB_JS_MACOS_CMD`
- `KRABKRAB_JS_NODE_HOST_CMD`
- `KRABKRAB_JS_IMESSAGE_CMD`
- `KRABKRAB_JS_WHATSAPP_CMD`
- `KRABKRAB_JS_LINE_CMD`
- `KRABKRAB_JS_TUI_CMD`
- `KRABKRAB_JS_WIZARD_CMD`
- `KRABKRAB_JS_WEB_CMD`

When set, the runtime executes that command and forwards JSON response.

Default external command mapping (used when env vars are not set):

- `browser_automation` -> `node ts-layer/commands/browser.mjs`
- `canvas_host` -> `node ts-layer/commands/canvas-host.mjs`
- `macos_native` -> `node ts-layer/commands/macos.mjs`
- `node_host` -> `node ts-layer/commands/node-host.mjs`
- `imessage_native` -> `node ts-layer/commands/imessage.mjs`
- `tui` -> `node ts-layer/commands/tui.mjs`
- `wizard` -> `node ts-layer/commands/wizard.mjs`
- `line` -> `node ts-layer/commands/line.mjs`
- `whatsapp` -> `node ts-layer/commands/whatsapp.mjs`
- `web` -> `node ts-layer/commands/web.mjs`

If a default external command fails, runtime falls back to local route mapping.

## Snapshot Source

TypeScript modules are stored in `src/` directory.
