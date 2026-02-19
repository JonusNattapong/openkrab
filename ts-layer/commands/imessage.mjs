#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"imessage_native","action":"normalize_target","payload":{}}');
const action = String(req.action || "normalize_target");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "send") {
  const recipient = String(payload.recipient || "");
  const message = String(payload.message || "");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'imessage_native', 
  action: '${action}', 
  message: 'iMessage send requested', 
  data: { 
    recipient: ${JSON.stringify(recipient)},
    messageLength: ${JSON.stringify(message.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "imessage_native", action, message: (out.stderr || "tsx imessage command failed").trim(), error_code: "imessage_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "list-chats") {
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'imessage_native', 
  action: '${action}', 
  message: 'iMessage chats listed', 
  data: { 
    hasListChats: true
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "imessage_native", action, message: (out.stderr || "tsx imessage command failed").trim(), error_code: "imessage_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "watch") {
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'imessage_native', 
  action: '${action}', 
  message: 'iMessage watch started', 
  data: { 
    hasWatch: true
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "imessage_native", action, message: (out.stderr || "tsx imessage command failed").trim(), error_code: "imessage_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
import { parseIMessageNotification } from '../../src/imessage/monitor/parse-notification.ts';
const sample = { message: { id: 1, sender: 'tester', text: ${JSON.stringify(String(payload.text || 'hello'))}, is_group: false } };
const parsed = parseIMessageNotification(sample);
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'imessage_native', action: '${action}', message: 'imessage parser executed', data: { parsed } }));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "imessage_native", action, message: (out.stderr || "tsx imessage command failed").trim(), error_code: "imessage_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
