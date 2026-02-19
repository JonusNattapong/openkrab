#!/usr/bin/env node

import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

function fallbackResponse(message) {
  return {
    ok: true,
    layer: "js",
    message,
    data: { passthrough: true },
  };
}

function main() {
  const runtime = resolve(process.cwd(), "ts-layer", "dist", "bridge-runtime.js");
  if (!existsSync(runtime)) {
    process.stdout.write(JSON.stringify(fallbackResponse("ts-layer runtime missing; run npm run -C ts-layer build")));
    return;
  }

  const out = spawnSync(process.execPath, [runtime], {
    env: process.env,
    shell: false,
    encoding: "utf8",
  });

  if (out.status !== 0) {
    const err = out.stderr?.trim() || `runtime exited with code ${String(out.status)}`;
    process.stderr.write(err);
    process.exit(1);
    return;
  }

  const stdout = (out.stdout || "").trim();
  if (!stdout) {
    process.stdout.write(JSON.stringify(fallbackResponse("ts-layer runtime produced empty output")));
    return;
  }

  process.stdout.write(stdout);
}

main();
