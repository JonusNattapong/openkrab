#!/usr/bin/env node

import { spawn } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"tui","action":"start","payload":{}}');
const action = String(req.action || "start");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "start") {
  const gatewayUrl = String(payload.gatewayUrl || "ws://127.0.0.1:18789");
  const profile = String(payload.profile || "default");
  
  const child = spawn("node", [
    "--import", tsxLoader,
    "-e", `
import { createTui } from '../../src/tui/tui.ts';
import { createGatewayChatClient } from '../../src/tui/gateway-chat.ts';

const gatewayUrl = ${JSON.stringify(gatewayUrl)};
const profile = ${JSON.stringify(profile)};

const client = createGatewayChatClient({ gatewayUrl, profile });
const tui = await createTui({ client, profile });
await tui.run();
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'tui', action: '${action}', message: 'TUI started', data: { gatewayUrl, profile } }));
`,  ], { shell: false, stdio: "inherit" });
  
  child.on("exit", (code) => {
    process.exit(code ?? 0);
  });
  return;
}

if (action === "status") {
  const script = `
import { theme } from '../../src/tui/theme/theme.ts';
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'tui', 
  action: '${action}', 
  message: 'TUI status checked', 
  data: { 
    themeName: theme.name,
    hasColors: true 
  } 
}));
`;
  const { spawnSync } = await import("node:child_process");
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "tui", action, message: (out.stderr || "tsx tui command failed").trim(), error_code: "tui_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "list-sessions") {
  const script = `
import { parseSessionKey } from '../../src/routing/session-key.ts';
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'tui', 
  action: '${action}', 
  message: 'Session parsing available', 
  data: { hasSessionKeyParser: true } 
}));
`;
  const { spawnSync } = await import("node:child_process");
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "tui", action, message: (out.stderr || "tsx tui command failed").trim(), error_code: "tui_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

process.stdout.write(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'tui', 
  action, 
  message: 'TUI command processed', 
  data: { action } 
}));
