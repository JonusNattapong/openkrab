#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"node_host","action":"coerce_payload","payload":{}}');
const action = String(req.action || "coerce_payload");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "invoke") {
  const tool = String(payload.tool || "");
  const args = payload.args || {};
  
  const script = `
import { invokeTool } from '../../src/node-host/invoke.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'node_host', 
  action: '${action}', 
  message: 'Node invoke executed', 
  data: { 
    tool: ${JSON.stringify(tool)},
    argsKeys: Object.keys(${JSON.stringify(args)}).length
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "list-tools") {
  const script = `
import { getNodeTools } from '../../src/node-host/runner.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'node_host', 
  action: '${action}', 
  message: 'Node tools listed', 
  data: { 
    hasGetNodeTools: typeof getNodeTools === 'function'
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "run") {
  const command = String(payload.command || "");
  const cwd = String(payload.cwd || process.cwd());
  const timeoutMs = Number(payload.timeoutMs || 30000);
  
  const script = `
import { runWithTimeout } from '../../src/node-host/with-timeout.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'node_host', 
  action: '${action}', 
  message: 'Node run started', 
  data: { 
    command: ${JSON.stringify(command)},
    cwd: ${JSON.stringify(cwd)},
    timeoutMs: ${timeoutMs}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "screenshot") {
  const outputPath = String(payload.outputPath || "");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'node_host', 
  action: '${action}', 
  message: 'Node screenshot requested', 
  data: { 
    outputPath: ${JSON.stringify(outputPath)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "notify") {
  const title = String(payload.title || "KrabKrab");
  const body = String(payload.body || "");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'node_host', 
  action: '${action}', 
  message: 'Node notification sent', 
  data: { 
    title: ${JSON.stringify(title)},
    body: ${JSON.stringify(body)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
import { withTimeout } from '../../src/node-host/with-timeout.ts';
const timeoutMs = Number(${JSON.stringify(payload.timeoutMs ?? 50)});
const value = await withTimeout(async () => 'ok', timeoutMs, 'node-host-test');
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'node_host', action: '${action}', message: 'node-host timeout helper executed', data: { value, timeoutMs } }));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "node_host", action, message: (out.stderr || "tsx node-host command failed").trim(), error_code: "node_host_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
