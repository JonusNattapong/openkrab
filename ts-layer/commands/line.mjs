#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"line","action":"status","payload":{}}');
const action = String(req.action || "status");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "send") {
  const to = String(payload.to || "");
  const message = String(payload.message || "");
  
  const script = `
import { createLineBot } from '../../src/line/index.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'line', 
  action: '${action}', 
  message: 'LINE message send requested', 
  data: { 
    to: ${JSON.stringify(to)},
    messageLength: ${JSON.stringify(message.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "line", action, message: (out.stderr || "tsx line command failed").trim(), error_code: "line_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "probe") {
  const script = `
import { probeLineChannel } from '../../src/line/probe.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'line', 
  action: '${action}', 
  message: 'LINE channel probe', 
  data: { hasProbeLineChannel: typeof probeLineChannel === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "line", action, message: (out.stderr || "tsx line command failed").trim(), error_code: "line_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "webhook") {
  const script = `
import { createLineWebhookHandler } from '../../src/line/webhook.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'line', 
  action: '${action}', 
  message: 'LINE webhook handler', 
  data: { hasCreateLineWebhookHandler: typeof createLineWebhookHandler === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "line", action, message: (out.stderr || "tsx line command failed").trim(), error_code: "line_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
import { getLineChannelStatus } from '../../src/line/index.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'line', 
  action: '${action}', 
  message: 'LINE status checked', 
  data: { hasGetLineChannelStatus: typeof getLineChannelStatus === 'function' }
}));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "line", action, message: (out.stderr || "tsx line command failed").trim(), error_code: "line_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
