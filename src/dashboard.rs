//! Web Dashboard — serves a browser UI for status/memory/chat.
//! Routes registered in `gateway.rs`:
//!   GET  /              → HTML dashboard
//!   GET  /api/status    → JSON gateway & connector status
//!   POST /api/chat      → Chat with the agent (JSON: {message: "..."})
//!   GET  /api/memory    → Memory search (query param: ?q=...)

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::gateway::GatewayState;
use crate::memory::HybridSearchOptions;

// ─── Request / Response types ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub ok: bool,
}

#[derive(Deserialize)]
pub struct MemoryQuery {
    pub q: Option<String>,
}

// ─── Dashboard HTML (self-contained, no external CDN) ─────────────────────────

const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>🦀 Krabkrab Dashboard</title>
<style>
  :root {
    --bg: #0f1117;
    --surface: #1a1d27;
    --border: #2d3048;
    --accent: #6c8fff;
    --accent2: #a78bfa;
    --green: #34d399;
    --red: #f87171;
    --yellow: #fbbf24;
    --text: #e2e8f0;
    --muted: #8892a4;
    --radius: 12px;
    --shadow: 0 4px 24px rgba(0,0,0,0.4);
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: 'Segoe UI', system-ui, sans-serif;
    background: var(--bg);
    color: var(--text);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
  header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 18px 28px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }
  header h1 { font-size: 1.3rem; font-weight: 700; }
  header .badge {
    font-size: 0.72rem;
    padding: 3px 10px;
    border-radius: 999px;
    background: rgba(108,143,255,0.15);
    color: var(--accent);
    border: 1px solid rgba(108,143,255,0.3);
  }
  .main {
    display: grid;
    grid-template-columns: 340px 1fr;
    gap: 0;
    flex: 1;
    min-height: 0;
  }
  .sidebar {
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 0;
    overflow-y: auto;
  }
  .panel {
    border-bottom: 1px solid var(--border);
    padding: 20px;
  }
  .panel-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--muted);
    margin-bottom: 14px;
  }
  .status-grid {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .status-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: var(--bg);
    border-radius: 8px;
    border: 1px solid var(--border);
    font-size: 0.85rem;
  }
  .status-row .label { color: var(--muted); }
  .dot {
    width: 8px; height: 8px;
    border-radius: 50%;
    display: inline-block;
    margin-right: 6px;
  }
  .dot.green { background: var(--green); box-shadow: 0 0 6px var(--green); }
  .dot.red   { background: var(--red);   box-shadow: 0 0 6px var(--red);   }
  .dot.yellow{ background: var(--yellow);box-shadow: 0 0 6px var(--yellow);}
  .chip {
    font-size: 0.72rem;
    padding: 2px 8px;
    border-radius: 999px;
    font-weight: 600;
  }
  .chip.on  { background: rgba(52,211,153,0.15); color: var(--green); border: 1px solid rgba(52,211,153,0.3); }
  .chip.off { background: rgba(248,113,113,0.15); color: var(--red);   border: 1px solid rgba(248,113,113,0.3); }
  /* Memory search */
  .search-box {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
  }
  input[type=text], textarea {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--text);
    padding: 9px 12px;
    font-size: 0.875rem;
    outline: none;
    width: 100%;
    transition: border-color 0.2s;
    font-family: inherit;
  }
  input[type=text]:focus, textarea:focus {
    border-color: var(--accent);
  }
  button {
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: 8px;
    padding: 9px 16px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s, transform 0.1s;
    white-space: nowrap;
  }
  button:hover { opacity: 0.88; }
  button:active { transform: scale(0.97); }
  button.secondary {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text);
  }
  .memory-results {
    max-height: 320px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .memory-card {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 10px 12px;
    font-size: 0.8rem;
    line-height: 1.5;
  }
  .memory-card .path {
    color: var(--accent);
    font-size: 0.72rem;
    margin-bottom: 4px;
    font-weight: 600;
    word-break: break-all;
  }
  .memory-card .score {
    color: var(--muted);
    float: right;
    font-size: 0.7rem;
  }
  .memory-card .snippet {
    color: var(--text);
    white-space: pre-wrap;
    word-break: break-word;
  }
  .empty-state {
    text-align: center;
    padding: 20px;
    color: var(--muted);
    font-size: 0.82rem;
  }
  /* Chat panel */
  .chat-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .chat-panel-header {
    padding: 16px 24px;
    border-bottom: 1px solid var(--border);
    font-size: 0.85rem;
    color: var(--muted);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .chat-panel-header strong { color: var(--text); }
  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .msg {
    display: flex;
    gap: 10px;
    max-width: 76%;
    animation: fadeIn 0.2s ease;
  }
  @keyframes fadeIn { from { opacity:0; transform: translateY(6px); } to { opacity:1; transform: none; } }
  .msg.user { align-self: flex-end; flex-direction: row-reverse; }
  .msg.bot  { align-self: flex-start; }
  .avatar {
    width: 32px; height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    flex-shrink: 0;
    font-weight: 700;
  }
  .avatar.user-av { background: linear-gradient(135deg, var(--accent), var(--accent2)); }
  .avatar.bot-av  { background: linear-gradient(135deg, #1e3a5f, #2563eb); }
  .bubble {
    padding: 10px 14px;
    border-radius: var(--radius);
    font-size: 0.88rem;
    line-height: 1.55;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .msg.user .bubble {
    background: linear-gradient(135deg, var(--accent), var(--accent2));
    color: #fff;
    border-bottom-right-radius: 3px;
  }
  .msg.bot .bubble {
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text);
    border-bottom-left-radius: 3px;
  }
  .msg.bot.thinking .bubble {
    color: var(--muted);
    font-style: italic;
  }
  .chat-input-bar {
    padding: 16px 24px;
    border-top: 1px solid var(--border);
    display: flex;
    gap: 10px;
    align-items: flex-end;
  }
  .chat-input-bar textarea {
    resize: none;
    min-height: 42px;
    max-height: 120px;
    overflow-y: auto;
  }
  .chat-input-bar button {
    padding: 10px 20px;
    flex-shrink: 0;
  }
  .connector-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .connector-badge {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 10px;
    border-radius: 8px;
    font-size: 0.78rem;
    font-weight: 600;
    border: 1px solid var(--border);
    background: var(--bg);
  }
  .connector-badge .icon { font-size: 14px; }
  ::-webkit-scrollbar { width: 6px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
  .refresh-btn {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--muted);
    padding: 4px 10px;
    font-size: 0.75rem;
    border-radius: 6px;
  }
  .refresh-btn:hover { color: var(--text); border-color: var(--accent); }
</style>
</head>
<body>

<header>
  <span style="font-size:24px">🦀</span>
  <h1>Krabkrab Dashboard</h1>
  <span class="badge">v0.1.0</span>
  <span style="flex:1"></span>
  <span id="uptime" style="font-size:0.78rem;color:var(--muted)">Loading…</span>
</header>

<div class="main">

  <!-- ── Sidebar ── -->
  <div class="sidebar">

    <!-- Status panel -->
    <div class="panel">
      <div class="panel-title" style="display:flex;align-items:center;justify-content:space-between;">
        Gateway Status
        <button class="refresh-btn" onclick="loadStatus()">↺ Refresh</button>
      </div>
      <div class="status-grid" id="status-grid">
        <div class="empty-state">Loading…</div>
      </div>
    </div>

    <!-- Connectors panel -->
    <div class="panel">
      <div class="panel-title">Active Connectors</div>
      <div class="connector-list" id="connector-list">
        <div class="empty-state">Loading…</div>
      </div>
    </div>

    <!-- Memory search panel -->
    <div class="panel" style="flex:1">
      <div class="panel-title">Memory Search</div>
      <div class="search-box">
        <input type="text" id="mem-query" placeholder="Search memory…" onkeydown="if(event.key==='Enter')searchMemory()"/>
        <button onclick="searchMemory()">Search</button>
      </div>
      <div class="memory-results" id="memory-results">
        <div class="empty-state">Enter a query to search agent memory</div>
      </div>
    </div>

  </div>

  <!-- ── Chat Panel ── -->
  <div class="chat-panel">
    <div class="chat-panel-header">
      <div><strong>Chat with Agent</strong> &nbsp;·&nbsp; ask anything via browser</div>
      <button class="secondary" style="font-size:0.75rem;padding:5px 12px;" onclick="clearChat()">Clear</button>
    </div>

    <div class="chat-messages" id="chat-messages">
      <div class="msg bot">
        <div class="avatar bot-av">🤖</div>
        <div class="bubble">Hello! I'm the Krabkrab agent. Ask me anything, or use the Memory Search panel on the left to explore stored knowledge.</div>
      </div>
    </div>

    <div class="chat-input-bar">
      <textarea id="chat-input" rows="1" placeholder="Type a message… (Enter to send, Shift+Enter for newline)"
        onkeydown="handleKey(event)" oninput="autoResize(this)"></textarea>
      <button id="send-btn" onclick="sendMessage()">Send ↑</button>
    </div>
  </div>

</div>

<script>
const startTime = Date.now();

// ── Uptime clock ──────────────────────────────────────────────────────────────
setInterval(() => {
  const s = Math.floor((Date.now() - startTime) / 1000);
  const h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60), sec = s % 60;
  document.getElementById('uptime').textContent =
    `Session: ${String(h).padStart(2,'0')}:${String(m).padStart(2,'0')}:${String(sec).padStart(2,'0')}`;
}, 1000);

// ── Status ────────────────────────────────────────────────────────────────────
async function loadStatus() {
  try {
    const r = await fetch('/api/status');
    const d = await r.json();
    renderStatus(d);
    renderConnectors(d.connectors || {});
  } catch(e) {
    document.getElementById('status-grid').innerHTML =
      `<div class="empty-state" style="color:var(--red)">⚠ Could not fetch status</div>`;
  }
}

function renderStatus(d) {
  const grid = document.getElementById('status-grid');
  grid.innerHTML = `
    <div class="status-row">
      <span class="label">Gateway</span>
      <span><span class="dot ${d.healthy ? 'green' : 'red'}"></span>${d.healthy ? 'Healthy' : 'Unhealthy'}</span>
    </div>
    <div class="status-row">
      <span class="label">Endpoint</span>
      <span style="font-size:0.78rem;color:var(--accent)">${d.endpoint || '—'}</span>
    </div>
    <div class="status-row">
      <span class="label">Agent</span>
      <span><span class="dot green"></span>${d.agent_name || 'Default'}</span>
    </div>
    <div class="status-row">
      <span class="label">Memory records</span>
      <span style="color:var(--accent2)">${d.memory_count ?? '—'}</span>
    </div>
    <div class="status-row">
      <span class="label">Version</span>
      <span style="color:var(--muted)">${d.version || '0.1.0'}</span>
    </div>
  `;
}

const CONNECTOR_META = {
  telegram:  { icon: '✈️',  label: 'Telegram'  },
  slack:     { icon: '💬',  label: 'Slack'     },
  discord:   { icon: '🎮',  label: 'Discord'   },
  line:      { icon: '💚',  label: 'Line'      },
  whatsapp:  { icon: '📱',  label: 'WhatsApp'  },
};

function renderConnectors(connectors) {
  const list = document.getElementById('connector-list');
  const entries = Object.entries(connectors);
  if (entries.length === 0) {
    list.innerHTML = '<div class="empty-state">No connectors configured</div>';
    return;
  }
  list.innerHTML = entries.map(([k, v]) => {
    const meta = CONNECTOR_META[k] || { icon: '🔌', label: k };
    const on = !!v;
    return `<div class="connector-badge">
      <span class="icon">${meta.icon}</span>
      <span>${meta.label}</span>
      <span class="chip ${on ? 'on' : 'off'}">${on ? 'ON' : 'OFF'}</span>
    </div>`;
  }).join('');
}

// ── Memory search ─────────────────────────────────────────────────────────────
async function searchMemory() {
  const q = document.getElementById('mem-query').value.trim();
  if (!q) return;
  const res = document.getElementById('memory-results');
  res.innerHTML = '<div class="empty-state">Searching…</div>';
  try {
    const r = await fetch(`/api/memory?q=${encodeURIComponent(q)}`);
    const d = await r.json();
    if (!d.results || d.results.length === 0) {
      res.innerHTML = '<div class="empty-state">No results found</div>';
      return;
    }
    res.innerHTML = d.results.map(item => `
      <div class="memory-card">
        <div class="path">
          ${item.path || 'unknown'}
          <span class="score">${item.score != null ? item.score.toFixed(3) : ''}</span>
        </div>
        <div class="snippet">${escapeHtml(truncate(item.text || '', 280))}</div>
      </div>
    `).join('');
  } catch(e) {
    res.innerHTML = `<div class="empty-state" style="color:var(--red)">⚠ Search failed</div>`;
  }
}

// ── Chat ──────────────────────────────────────────────────────────────────────
function handleKey(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}

function autoResize(el) {
  el.style.height = 'auto';
  el.style.height = Math.min(el.scrollHeight, 120) + 'px';
}

function clearChat() {
  const msgs = document.getElementById('chat-messages');
  msgs.innerHTML = `
    <div class="msg bot">
      <div class="avatar bot-av">🤖</div>
      <div class="bubble">Chat cleared. What would you like to know?</div>
    </div>`;
}

function appendMessage(role, text, thinking) {
  const msgs = document.getElementById('chat-messages');
  const div = document.createElement('div');
  div.className = `msg ${role}${thinking ? ' thinking' : ''}`;
  div.innerHTML = role === 'user'
    ? `<div class="avatar user-av">👤</div><div class="bubble">${escapeHtml(text)}</div>`
    : `<div class="avatar bot-av">🤖</div><div class="bubble">${escapeHtml(text)}</div>`;
  msgs.appendChild(div);
  msgs.scrollTop = msgs.scrollHeight;
  return div;
}

async function sendMessage() {
  const input = document.getElementById('chat-input');
  const btn = document.getElementById('send-btn');
  const text = input.value.trim();
  if (!text) return;

  input.value = '';
  input.style.height = 'auto';
  appendMessage('user', text);

  btn.disabled = true;
  const thinkingEl = appendMessage('bot', '⟳ Thinking…', true);

  try {
    const r = await fetch('/api/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: text }),
    });
    const d = await r.json();
    thinkingEl.remove();
    if (d.ok) {
      appendMessage('bot', d.reply);
    } else {
      appendMessage('bot', `⚠️ ${d.reply || 'Agent error'}`);
    }
  } catch(e) {
    thinkingEl.remove();
    appendMessage('bot', '⚠️ Network error — could not reach the gateway.');
  } finally {
    btn.disabled = false;
    input.focus();
  }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
function escapeHtml(s) {
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')
          .replace(/"/g,'&quot;').replace(/'/g,'&#39;');
}
function truncate(s, n) { return s.length > n ? s.slice(0, n) + '…' : s; }

// ── Init ──────────────────────────────────────────────────────────────────────
loadStatus();
setInterval(loadStatus, 30_000);
document.getElementById('chat-input').focus();
</script>
</body>
</html>"#;

// ─── Handler functions (registered in gateway.rs) ─────────────────────────────

/// Serve the main dashboard HTML page.
pub async fn dashboard_handler() -> impl IntoResponse {
    Html(DASHBOARD_HTML)
}

/// GET /api/status — returns gateway health + connector on/off flags.
pub async fn api_status_handler(State(state): State<Arc<GatewayState>>) -> impl IntoResponse {
    let connectors = json!({
        "telegram":  std::env::var("TELEGRAM_BOT_TOKEN").is_ok(),
        "slack":     std::env::var("SLACK_BOT_TOKEN").is_ok(),
        "discord":   std::env::var("DISCORD_BOT_TOKEN").is_ok(),
        "line":      std::env::var("LINE_CHANNEL_ACCESS_TOKEN").is_ok(),
        "whatsapp":  std::env::var("WHATSAPP_ACCESS_TOKEN").is_ok(),
    });

    // Try a trivial search just to confirm memory is reachable
    let memory_count: Option<usize> = match state.memory.as_ref() {
        Some(memory) => memory
            .search_hybrid(
                "status",
                HybridSearchOptions {
                    max_results: 1,
                    ..Default::default()
                },
            )
            .await
            .ok()
            .map(|_| 0),
        None => None,
    };

    let body = json!({
        "healthy": true,
        "endpoint": "http://127.0.0.1:3000",
        "version": env!("CARGO_PKG_VERSION"),
        "agent_name": "Krabkrab Agent",
        "memory_count": memory_count,
        "connectors": connectors,
    });

    Json(body)
}

/// POST /api/chat — send a message to the agent, get a reply.
pub async fn api_chat_handler(
    State(state): State<Arc<GatewayState>>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    if req.message.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "ok": false, "reply": "Message cannot be empty." })),
        )
            .into_response();
    }

    let agent = match state.agent.as_ref() {
        Some(a) => a,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "ok": false, "reply": "Agent not available" })),
            )
                .into_response();
        }
    };

    match agent.answer(&req.message).await {
        Ok(reply) => (StatusCode::OK, Json(json!({ "ok": true, "reply": reply }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "ok": false, "reply": format!("Agent error: {}", e) })),
        ).into_response(),
    }
}

/// GET /api/memory?q=... — search agent memory and return results as JSON.
pub async fn api_memory_handler(
    State(state): State<Arc<GatewayState>>,
    Query(params): Query<MemoryQuery>,
) -> Response {
    let query = match &params.q {
        Some(q) if !q.trim().is_empty() => q.clone(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "ok": false, "error": "Missing query param: ?q=..." })),
            )
                .into_response();
        }
    };

    let memory = match state.memory.as_ref() {
        Some(m) => m,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "ok": false, "error": "Memory not available" })),
            )
                .into_response();
        }
    };

    match memory
        .search_hybrid(&query, HybridSearchOptions::default())
        .await
    {
        Ok(results) => {
            let items: Vec<serde_json::Value> = results
                .iter()
                .take(10)
                .map(|r| {
                    json!({
                        "path": r.path,
                        "text": r.text,
                        "score": r.score,
                    })
                })
                .collect();
            (
                StatusCode::OK,
                Json(json!({ "ok": true, "results": items })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "ok": false, "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// Simple health check endpoint.
pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        r#"{"status":"ok"}"#,
    )
}
