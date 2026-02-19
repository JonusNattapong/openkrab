import { spawnSync } from "node:child_process";

type BridgeRequest = {
  feature: string;
  action: string;
  payload: Record<string, unknown>;
};

type BridgeResponse = {
  ok: boolean;
  layer: "js";
  feature: string;
  action: string;
  message: string;
  error_code?: string;
  data?: Record<string, unknown>;
};

const DEFAULT_EXTERNAL_COMMANDS: Record<string, string> = {
  browser_automation: "node ts-layer/commands/browser.mjs",
  canvas_host: "node ts-layer/commands/canvas-host.mjs",
  macos_native: "node ts-layer/commands/macos.mjs",
  node_host: "node ts-layer/commands/node-host.mjs",
  imessage_native: "node ts-layer/commands/imessage.mjs",
};

function readRequest(): BridgeRequest {
  const raw = process.env.KRABKRAB_BRIDGE_REQUEST;
  if (!raw) {
    return { feature: "unknown", action: "run", payload: {} };
  }
  try {
    const parsed = JSON.parse(raw) as Partial<BridgeRequest>;
    return {
      feature: String(parsed.feature || "unknown"),
      action: String(parsed.action || "run"),
      payload:
        parsed.payload && typeof parsed.payload === "object"
          ? (parsed.payload as Record<string, unknown>)
          : {},
    };
  } catch {
    return { feature: "unknown", action: "run", payload: {} };
  }
}

function commandEnvKey(feature: string): string {
  switch (feature) {
    case "browser_automation":
      return "KRABKRAB_JS_BROWSER_CMD";
    case "canvas_host":
      return "KRABKRAB_JS_CANVAS_CMD";
    case "macos_native":
      return "KRABKRAB_JS_MACOS_CMD";
    case "node_host":
      return "KRABKRAB_JS_NODE_HOST_CMD";
    case "imessage_native":
      return "KRABKRAB_JS_IMESSAGE_CMD";
    case "whatsapp_full":
      return "KRABKRAB_JS_WHATSAPP_CMD";
    case "line_full":
      return "KRABKRAB_JS_LINE_CMD";
    default:
      return "";
  }
}

function response(req: BridgeRequest, message: string, data?: Record<string, unknown>): BridgeResponse {
  return {
    ok: true,
    layer: "js",
    feature: req.feature,
    action: req.action,
    message,
    data,
  };
}

function errorResponse(req: BridgeRequest, message: string, errorCode: string): BridgeResponse {
  return {
    ok: false,
    layer: "js",
    feature: req.feature,
    action: req.action,
    message,
    error_code: errorCode,
  };
}

function runExternalBridge(req: BridgeRequest, command: string): BridgeResponse {
  const out = spawnSync(command, {
    shell: true,
    encoding: "utf8",
    env: process.env,
  });

  if (out.status !== 0) {
    return errorResponse(
      req,
      `external JS bridge failed: ${(out.stderr || "").trim() || "unknown error"}`,
      "external_bridge_failed",
    );
  }

  const raw = (out.stdout || "").trim();
  if (!raw) {
    return errorResponse(req, "external JS bridge returned empty output", "empty_external_output");
  }

  try {
    const parsed = JSON.parse(raw) as Record<string, unknown>;
    if (typeof parsed.ok === "boolean") {
      return {
        ok: parsed.ok,
        layer: "js",
        feature: String(parsed.feature || req.feature),
        action: String(parsed.action || req.action),
        message: String(parsed.message || (parsed.ok ? "handled by external JS bridge" : "external JS bridge failed")),
        error_code: typeof parsed.error_code === "string" ? parsed.error_code : undefined,
        data: typeof parsed.data === "object" && parsed.data !== null ? (parsed.data as Record<string, unknown>) : parsed,
      };
    }
    return response(req, "handled by external JS bridge", parsed);
  } catch {
    return response(req, "handled by external JS bridge (raw output)", { raw_output: raw });
  }
}

function runLocalRoute(req: BridgeRequest): BridgeResponse {
  if (req.action === "health") {
    return response(req, "js bridge runtime healthy", { runtime: "ts-layer" });
  }

  const featureMap: Record<string, string> = {
    browser_automation: "ts-layer/browser",
    canvas_host: "ts-layer/canvas-host",
    macos_native: "ts-layer/macos",
    node_host: "ts-layer/node-host",
    imessage_native: "ts-layer/imessage",
    whatsapp_full: "ts-layer/whatsapp",
    line_full: "ts-layer/line",
  };

  const route = featureMap[req.feature];
  if (!route) {
    return errorResponse(req, `unknown feature: ${req.feature}`, "unknown_feature");
  }

  return response(req, `handled by JS layer route=${route}`, {
    route,
    passthrough: true,
    payload: req.payload,
  });
}

function main(): void {
  const req = readRequest();
  const envKey = commandEnvKey(req.feature);
  const explicitCommand = envKey ? process.env[envKey] : undefined;
  const defaultCommand = DEFAULT_EXTERNAL_COMMANDS[req.feature];
  const command = explicitCommand || defaultCommand;

  let out: BridgeResponse;
  if (command) {
    out = runExternalBridge(req, command);
    if (!out.ok && !explicitCommand) {
      const fallback = runLocalRoute(req);
      fallback.message = `${fallback.message} (default external command failed, using local route)`;
      fallback.data = {
        ...(fallback.data || {}),
        external_error: out.message,
        external_command: command,
      };
      out = fallback;
    }
  } else {
    out = runLocalRoute(req);
  }
  process.stdout.write(JSON.stringify(out));
}

main();
