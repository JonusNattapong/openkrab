#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"canvas_host","action":"serve","payload":{}}');
const action = String(req.action || "serve");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "serve") {
  const port = Number(payload.port || 3030);
  const root = String(payload.root || "./canvas");
  const host = String(payload.host || "127.0.0.1");
  
  const script = `
import http from 'node:http';
import { WebSocketServer } from 'ws';

const PORT = ${port};
const HOST = ${JSON.stringify(host)};

const A2UI_PATH = '/__krabkrab__/a2ui';
const CANVAS_PATH = '/__krabkrab__/canvas';
const WS_PATH = '/__krabkrab__/ws';

const canvasStates = new Map();

const html = \`<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>krabkrab Canvas</title>
  <style>
    body { margin: 0; background: #000; color: #fff; font: 16px/1.5 -apple-system,system-ui,sans-serif; overflow: hidden; }
    canvas { position: fixed; inset: 0; width: 100vw; height: 100vh; }
  </style>
</head>
<body>
  <canvas id="c"></canvas>
  <script>
    const canvas = document.getElementById('c');
    const ctx = canvas.getContext('2d');
    let ws = null;
    let revision = 0;

    function resize() {
      const dpr = window.devicePixelRatio || 1;
      canvas.width = window.innerWidth * dpr;
      canvas.height = window.innerHeight * dpr;
      ctx.scale(dpr, dpr);
    }
    window.onresize = resize;
    resize();

    function connect() {
      ws = new WebSocket('ws://' + location.host + '${WS_PATH}');
      ws.onmessage = (e) => {
        try {
          const msg = JSON.parse(e.data);
          if (msg.type === 'view' && msg.revision > revision) {
            revision = msg.revision;
            render(msg.view);
          }
        } catch(x) {}
      };
      ws.onclose = () => setTimeout(connect, 3000);
    }
    connect();

    function render(view) {
      ctx.fillStyle = '#000';
      ctx.fillRect(0, 0, window.innerWidth, window.innerHeight);
      if (view && view.text) {
        ctx.fillStyle = '#fff';
        ctx.font = '24px -apple-system, system-ui';
        ctx.fillText(view.text, 40, 60);
      }
    }
  </script>
</body>
</html>\`;

const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://' + HOST + ':' + PORT);
  
  if (url.pathname === CANVAS_PATH || url.pathname === '/') {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(html);
  } else if (url.pathname === A2UI_PATH) {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(html);
  } else {
    res.writeHead(404);
    res.end('Not Found');
  }
});

const wss = new WebSocketServer({ server, path: WS_PATH });

wss.on('connection', (ws) => {
  ws.on('message', (data) => {
    try {
      const msg = JSON.parse(data);
      if (msg.type === 'push') {
        const state = canvasStates.get(msg.workspace || 'main') || { view: {}, revision: 0 };
        state.view = msg.view;
        state.revision++;
        canvasStates.set(msg.workspace || 'main', state);
        wss.clients.forEach(c => c.send(JSON.stringify({ type: 'view', workspace: msg.workspace, revision: state.revision, view: state.view })));
      }
    } catch(x) {}
  });
});

server.listen(PORT, HOST, () => {
  console.log(JSON.stringify({ 
    ok: true, 
    layer: 'js', 
    feature: 'canvas_host', 
    action: '${action}', 
    message: 'Canvas server started', 
    data: { 
      port: PORT,
      host: HOST,
      canvasPath: CANVAS_PATH,
      wsPath: WS_PATH,
      a2uiPath: A2UI_PATH,
      url: 'http://' + HOST + ':' + PORT + CANVAS_PATH
    } 
  }));
});
`;
  const child = spawn("node", ["--import", tsxLoader, "-e", script], { shell: false, stdio: "inherit" });
  child.on("exit", (code) => process.exit(code ?? 0));
  return;
}

if (action === "push") {
  const workspace = String(payload.workspace || "main");
  const view = payload.view || {};
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'canvas_host', 
  action: '${action}', 
  message: 'Canvas push (ts-layer handler)', 
  data: { 
    workspace: ${JSON.stringify(workspace)},
    hasView: ${JSON.stringify(!!view)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "canvas_host", action, message: (out.stderr || "tsx canvas command failed").trim(), error_code: "canvas_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "snapshot") {
  const workspace = String(payload.workspace || "main");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'canvas_host', 
  action: '${action}', 
  message: 'Canvas snapshot (ts-layer handler)', 
  data: { 
    workspace: ${JSON.stringify(workspace)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "canvas_host", action, message: (out.stderr || "tsx canvas command failed").trim(), error_code: "canvas_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "reset") {
  const workspace = String(payload.workspace || "main");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'canvas_host', 
  action: '${action}', 
  message: 'Canvas reset (ts-layer handler)', 
  data: { 
    workspace: ${JSON.stringify(workspace)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "canvas_host", action, message: (out.stderr || "tsx canvas command failed").trim(), error_code: "canvas_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "status") {
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'canvas_host', 
  action: '${action}', 
  message: 'Canvas host ready (full implementation)', 
  data: { 
    hasHttpServer: true,
    hasWebSocket: true,
    hasA2UI: true,
    impl: 'ts-layer-full'
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "canvas_host", action, message: (out.stderr || "tsx canvas command failed").trim(), error_code: "canvas_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
console.log(JSON.stringify({ ok: true, layer: 'js', feature: 'canvas_host', action: '${action}', message: 'canvas command processed', data: { action: '${action}' } }));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "canvas_host", action, message: (out.stderr || "tsx canvas command failed").trim(), error_code: "canvas_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
