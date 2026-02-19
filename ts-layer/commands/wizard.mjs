#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"wizard","action":"start","payload":{}}');
const action = String(req.action || "start");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "start") {
  const script = `
import { runOnboarding } from '../../src/wizard/onboarding.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'wizard', 
  action: '${action}', 
  message: 'Wizard started', 
  data: { hasRunOnboarding: typeof runOnboarding === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "wizard", action, message: (out.stderr || "tsx wizard command failed").trim(), error_code: "wizard_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "status") {
  const script = `
import { getOnboardingStatus } from '../../src/wizard/session.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'wizard', 
  action: '${action}', 
  message: 'Wizard status checked', 
  data: { hasGetOnboardingStatus: typeof getOnboardingStatus === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "wizard", action, message: (out.stderr || "tsx wizard command failed").trim(), error_code: "wizard_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

process.stdout.write(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'wizard', 
  action, 
  message: 'Wizard command processed', 
  data: { action } 
}));
