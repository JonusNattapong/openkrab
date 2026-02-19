#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"whatsapp","action":"status","payload":{}}');
const action = String(req.action || "status");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "send") {
  const to = String(payload.to || "");
  const message = String(payload.message || "");
  
  const script = `
import { normalizeWhatsAppNumber } from '../../src/whatsapp/normalize.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'whatsapp', 
  action: '${action}', 
  message: 'WhatsApp message send requested', 
  data: { 
    to: ${JSON.stringify(to)},
    normalized: normalizeWhatsAppNumber(${JSON.stringify(to)}),
    messageLength: ${JSON.stringify(message.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "whatsapp", action, message: (out.stderr || "tsx whatsapp command failed").trim(), error_code: "whatsapp_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "connect") {
  const script = `
import { createWhatsAppConnection } from '../../src/whatsapp/index.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'whatsapp', 
  action: '${action}', 
  message: 'WhatsApp connection requested', 
  data: { hasCreateWhatsAppConnection: typeof createWhatsAppConnection === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "whatsapp", action, message: (out.stderr || "tsx whatsapp command failed").trim(), error_code: "whatsapp_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
import { normalizeWhatsAppNumber } from '../../src/whatsapp/normalize.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'whatsapp', 
  action: '${action}', 
  message: 'WhatsApp status checked', 
  data: { hasNormalizeWhatsAppNumber: typeof normalizeWhatsAppNumber === 'function' }
}));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "whatsapp", action, message: (out.stderr || "tsx whatsapp command failed").trim(), error_code: "whatsapp_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
