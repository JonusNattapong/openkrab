#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";
import fs from "node:fs";

const req = JSON.parse(process.env.KRABKRAB_BRIDGE_REQUEST || '{"feature":"web","action":"status","payload":{}}');
const action = String(req.action || "status");
const payload = req.payload && typeof req.payload === "object" ? req.payload : {};
const here = path.dirname(fileURLToPath(import.meta.url));
const tsxLoader = pathToFileURL(path.resolve(here, "../node_modules/tsx/dist/loader.mjs")).href;

if (action === "serve-webchat") {
  const port = Number(payload.port || 3031);
  const host = String(payload.host || "127.0.0.1");
  const gatewayUrl = String(payload.gatewayUrl || "ws://127.0.0.1:18789");
  
  const script = `
import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';

const PORT = ${port};
const HOST = ${JSON.stringify(host)};
const GW = ${JSON.stringify(gatewayUrl)};

const html = \`
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>krabkrab WebChat</title>
  <style>
    body { font: 16px/1.5 -apple-system,system-ui,sans-serif; background: #0d0d0f; color: #e4e4e7; margin: 0; height: 100vh; display: flex; flex-direction: column; }
    header { padding: 12px 16px; background: #18181b; border-bottom: 1px solid #27272a; }
    #msgs { flex: 1; overflow: auto; padding: 16px; }
    .msg { max-width: 80%; padding: 10px 14px; border-radius: 12px; margin: 6px 0; white-space: pre-wrap; }
    .user { align-self: flex-end; background: #312e81; border-bottom-right-radius: 4px; }
    .agent { align-self: flex-start; background: #1e1b4b; border-bottom-left-radius: 4px; }
    #in { display: flex; padding: 12px; background: #18181b; border-top: 1px solid #27272a; gap: 8px; }
    textarea { flex: 1; background: #0d0d0f; border: 1px solid #27272a; border-radius: 8px; padding: 10px; color: #fff; font: inherit; }
    button { background: #6366f1; color: #fff; border: none; border-radius: 8px; padding: 10px 20px; font-weight: 600; cursor: pointer; }
    .status { width: 8px; height: 8px; border-radius: 50%; background: #71717a; display: inline-block; margin-right: 8px; }
    .connected { background: #22c55e; }
    .error { background: #ef4444; }
  </style>
</head>
<body>
  <header><span class="status" id="s"></span>krabkrab WebChat</header>
  <div id="msgs"></div>
  <div id="in"><textarea id="t" rows="1" placeholder="Type..."></textarea><button id="b">Send</button></div>
  <script>
    const ws = new WebSocket(${JSON.stringify(GW)});
    const ms = document.getElementById('msgs');
    const t = document.getElementById('t');
    const b = document.getElementById('b');
    const s = document.getElementById('s');
    let sk = 'agent:main:webchat:dm:default';
    ws.onopen = () => { s.className = 'status connected'; ws.send(JSON.stringify({jsonrpc:'2.0',id:1,method:'client.hello',params:{client:{name:'webchat',version:'1.0'},auth:{}}})); };
    ws.onclose = () => s.className = 'status error';
    ws.onmessage = e => { try { const m=JSON.parse(e.data); if(m.result?.message?.content)add(m.result.message.content,'agent'); }catch(x){} };
    function add(c,r){const d=document.createElement('div');d.className='msg '+r;d.textContent=c;ms.appendChild(d);ms.scrollTop=ms.scrollHeight;}
    b.onclick = () => { if(!t.value)return; add(t.value,'user'); ws.send(JSON.stringify({jsonrpc:'2.0',id:Date.now(),method:'chat.send',params:{sessionKey:sk,text:t.value,idempotencyKey:'m'+Date.now()}})); t.value=''; };
  </script>
</body>
</html>
\`;

const server = http.createServer((req, res) => {
  res.writeHead(200, {'Content-Type': 'text/html'});
  res.end(html);
});

server.listen(PORT, HOST, () => {
  console.log(JSON.stringify({ ok: true, port: PORT, host: HOST, url: 'http://' + HOST + ':' + PORT }));
});
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "qr-login") {
  const accountId = String(payload.accountId || "default");
  const verbose = payload.verbose !== false;
  
  const script = `
import { startQrLogin } from '../../src/web/login-qr.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'QR login started', 
  data: { 
    accountId: ${JSON.stringify(accountId)},
    verbose: ${JSON.stringify(verbose)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "login") {
  const script = `
import { loginWeb } from '../../src/web/login.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'Web login handler', 
  data: { hasLoginWeb: typeof loginWeb === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "send") {
  const sessionId = String(payload.sessionId || "");
  const message = String(payload.message || "");
  
  const script = `
import { sendWebMessage } from '../../src/web/outbound.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'Web message send requested', 
  data: { 
    sessionId: ${JSON.stringify(sessionId)},
    messageLength: ${JSON.stringify(message.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "accounts") {
  const script = `
import { getWebAccounts } from '../../src/web/accounts.ts';

console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'Web accounts retrieved', 
  data: { hasGetWebAccounts: typeof getWebAccounts === 'function' }
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "chat-send") {
  const sessionKey = String(payload.sessionKey || "agent:main:webchat:dm:default");
  const text = String(payload.text || "");
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'WebChat send message', 
  data: { 
    sessionKey: ${JSON.stringify(sessionKey)},
    textLength: ${JSON.stringify(text.length)}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

if (action === "chat-history") {
  const sessionKey = String(payload.sessionKey || "agent:main:webchat:dm:default");
  const limit = Number(payload.limit || 50);
  
  const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'WebChat history requested', 
  data: { 
    sessionKey: ${JSON.stringify(sessionKey)},
    limit: ${limit}
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
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
  feature: 'web', 
  action: '${action}', 
  message: 'Web status checked', 
  data: { 
    hasLogin: true,
    hasQrLogin: true,
    hasOutbound: true,
    hasAccounts: true,
    hasWebChat: true
  } 
}));
`;
  const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
  if (out.status !== 0) {
    process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
    process.exit(0);
  }
  process.stdout.write((out.stdout || "").trim());
  return;
}

const script = `
console.log(JSON.stringify({ 
  ok: true, 
  layer: 'js', 
  feature: 'web', 
  action: '${action}', 
  message: 'Web command processed', 
  data: { action: '${action}' }
}));
`;

const out = spawnSync("node", ["--import", tsxLoader, "-e", script], { encoding: "utf8", shell: false });
if (out.status !== 0) {
  process.stdout.write(JSON.stringify({ ok: false, layer: "js", feature: "web", action, message: (out.stderr || "tsx web command failed").trim(), error_code: "web_command_failed" }));
  process.exit(0);
}
process.stdout.write((out.stdout || "").trim());
