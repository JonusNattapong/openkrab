#!/usr/bin/env node

import module from "node:module";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// https://nodejs.org/api/module.html#module-compile-cache
if (module.enableCompileCache && !process.env.NODE_DISABLE_COMPILE_CACHE) {
  try {
    module.enableCompileCache();
  } catch {
    // Ignore errors
  }
}

const runRustCli = () => {

  const rootDir = path.dirname(fileURLToPath(import.meta.url));
  const executableName = process.platform === "win32" ? "openclaw.exe" : "openclaw";
  const candidates = [];

  if (process.env.OPENCLAW_RUST_BIN) {
    candidates.push(process.env.OPENCLAW_RUST_BIN);
  }

  candidates.push(
    path.join(rootDir, "target", "release", executableName),
    path.join(rootDir, "target", "debug", executableName),
  );

  const rustBin = candidates.find((candidate) => {
    try {
      return fs.statSync(candidate).isFile();
    } catch {
      return false;
    }
  });

  if (!rustBin) {
    const searched = candidates.join(", ");
    throw new Error(
      `openclaw: Rust binary not found. Build it with 'cargo build --release'. Looked in: ${searched}. Node fallback has been disabled; reference: openclaw.node-reference.mjs`,
    );
  }

  const result = spawnSync(rustBin, process.argv.slice(2), {
    stdio: "inherit",
    env: process.env,
  });

  if (result.error) {
    throw result.error;
  }

  process.exit(result.status ?? 1);
};

runRustCli();
