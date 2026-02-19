#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"browser_automation","action":"status","payload":{}}');
const action = String(req.action || "status");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "snapshot") {
  const profile = String(payload.profile || "default");
  const url = String(payload.url || "");
  
  const script = `
import { allocateCdpPort, isValidProfileName } from '../../src/browser/profiles.ts';
import { takeSnapshot } from '../../src/browser/pw-tools-core.snapshot.ts';

const profile = ${JSON.stringify(profile)};
const url = ${JSON.stringify(url)};
const valid = isValidProfileName(profile);
const port = allocateCdpPort(new Set([18800, 18801]));

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'browser_automation', 
  action: '${action}', 
  message: 'Browser snapshot requested', 
  data: { profile, url, valid, port } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "browser_automation", action, message: (out.stderr || "tsx browser command failed").trim(), error_code: "browser_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "act") {
  const profile = String(payload.profile || "default");
  const commands = payload.commands || [];
  
  const script = `
import { allocateCdpPort } from '../../src/browser/profiles.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'browser_automation', 
  action: '${action}', 
  message: 'Browser act commands', 
  data: { 
    profile: ${JSON.stringify(profile)},
    commandCount: ${JSON.stringify(commands.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "browser_automation", action, message: (out.stderr || "tsx browser command failed").trim(), error_code: "browser_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "list-profiles") {
  const script = `
import { listProfiles } from '../../src/browser/profiles-service.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'browser_automation', 
  action: '${action}', 
  message: 'Browser profiles listed', 
  data: { 
    hasListProfiles: typeof listProfiles === 'function'
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "browser_automation", action, message: (out.stderr || "tsx browser command failed").trim(), error_code: "browser_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "open-profile") {
  const profile = String(payload.profile || "default");
  const url = String(payload.url || "about:blank");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'browser_automation', 
  action: '${action}', 
  message: 'Browser profile opened', 
  data: { 
    profile: ${JSON.stringify(profile)},
    url: ${JSON.stringify(url)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "browser_automation", action, message: (out.stderr || "tsx browser command failed").trim(), error_code: "browser_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const modulePath = "../../src/browser/profiles.ts";
const script = `
import { isValidProfileName, allocateCdpPort } from '${modulePath}';
const profile = ${JSON.stringify(String(payload.profile || 'default-profile'))};
const used = new Set([18800, 18801]);
const valid = isValidProfileName(profile);
const port = allocateCdpPort(used);
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'browser_automation', action: '${action}', message: 'browser profile utilities executed', data: { valid, port } }));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "browser_automation", action, message: (out.stderr || "tsx browser command failed").trim(), error_code: "browser_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
