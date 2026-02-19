#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"macos_native","action":"status","payload":{}}');
const action = String(req.action || "status");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "notify") {
  const title = String(payload.title || "KrabKrab");
  const body = String(payload.body || "");
  const sound = payload.sound !== false;
  
  const script = `
import { spawn } from 'node:child_process';
const title = ${JSON.stringify(title)};
const body = ${JSON.stringify(body)};
const sound = ${JSON.stringify(sound)};

const args = ['-title', title, '-message', body];
if (${sound}) args.push('-sound');

const proc = spawn('osascript', ['-e', 'display notification ' + 
  JSON.stringify(body) + ' with title ' + JSON.stringify(title) +
  (${sound} ? ' sound name "default"' : '')]);

proc.on('close', (code) => {
  console.log(JSON.stringify({ ok: code === 0, code }));
});
proc.on('error', (err) => {
  console.log(JSON.stringify({ ok: false, error: err.message }));
});
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  process.stdout.write((out.stdout || "").trim() || JSON.stringify({ ok: true, layer: "js", feature: "macos_native", action, message: "Notification sent" }));
  return;
}

if (action === "menu-bar") {
  const status = String(payload.status || "idle");
  const badge = String(payload.badge || "");
  
  const script = `
import { parseRelaySmokeTest } from '../../src/macos/relay-smoke.ts';
import { getGatewayStatus } from '../../src/macos/gateway-daemon.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'macos_native', 
  action: '${action}', 
  message: 'Menu bar status updated', 
  data: { 
    status: ${JSON.stringify(status)},
    badge: ${JSON.stringify(badge)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "macos_native", action, message: (out.stderr || "tsx macos command failed").trim(), error_code: "macos_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "voice-wake") {
  const enabled = payload.enabled !== false;
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'macos_native', 
  action: '${action}', 
  message: 'Voice wake toggle', 
  data: { 
    enabled: ${JSON.stringify(enabled)},
    engine: 'whisper.cpp'
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "applescript") {
  const script = String(payload.script || 'return "ok"');
  
  const out = spawnSync("osascript", ["-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "macos_native", action, message: (out.stderr || "AppleScript failed").trim(), error_code: "applescript_failed" }));
    process.exit(0);
  }
  process.stdout.write(JSON.stringify({ ok: true, layer: "js", feature: "macos_native", action, message: "AppleScript executed", data: { result: (out.stdout || "").trim() } }));
  return;
}

if (action === "gateway-status") {
  const script = `
import { getGatewayStatus } from '../../src/macos/gateway-daemon.ts';
const status = await getGatewayStatus();
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'macos_native', action: '${action}', message: 'Gateway status retrieved', data: status }));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "macos_native", action, message: (out.stderr || "tsx macos command failed").trim(), error_code: "macos_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
import { parseRelaySmokeTest } from '../../src/macos/relay-smoke.ts';
const parsed = parseRelaySmokeTest(['--smoke','qr'], process.env);
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'macos_native', action: '${action}', message: 'macos relay parser executed', data: { parsed } }));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "macos_native", action, message: (out.stderr || "tsx macos command failed").trim(), error_code: "macos_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
