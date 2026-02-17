# OpenClaw Rust Architecture
## Full System Architecture Document

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         OpenClaw Gateway                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   WebSocket │  │   HTTP API  │  │      Health/Status      │  │
│  │   Server    │  │   REST      │  │                         │  │
│  └──────┬──────┘  └──────┬──────┘  └─────────────────────────┘  │
│         └─────────────────┘                                      │
│                      │                                           │
│  ┌───────────────────┴───────────────────┐                      │
│  │           Connection Manager            │                      │
│  │           (WebSocket conns)             │                      │
│  └───────────────────┬───────────────────┘                      │
│                      │                                           │
│  ┌───────────────────┴───────────────────┐                      │
│  │         JSON-RPC Protocol Handler       │                      │
│  └───────────────────┬───────────────────┘                      │
│                      │                                           │
└──────────────────────┼───────────────────────────────────────────┘
                       │
       ┌───────────────┼───────────────┐
       │               │               │
┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
│   Session   │ │   Channel   │ │   Agent     │
│   Manager   │ │   Registry  │ │   Runtime   │
└──────┬──────┘ └──────┬──────┘ └──────┬──────┘
       │               │               │
       │    ┌──────────┴──────────┐    │
       │    │                     │    │
┌──────▼────▼──────┐   ┌──────────▼────▼──────┐
│     Storage      │   │       Router         │
│  ┌────────────┐  │   │  ┌────────────────┐  │
│  │  SQLite    │  │   │  │ Routing Rules  │  │
│  │  Postgres  │  │   │  │ Pattern Match  │  │
│  │  Memory    │  │   │  │ Action Handler │  │
│  └────────────┘  │   │  └────────────────┘  │
└──────────────────┘   └──────────────────────┘
                              │
       ┌──────────────────────┼──────────────────────┐
       │                      │                      │
┌──────▼──────┐    ┌──────────▼──────────┐  ┌──────▼──────┐
│  Telegram   │    │      Discord        │  │    Slack    │
│  (teloxide) │    │     (serenity)      │  │(slack-morph)│
└─────────────┘    └─────────────────────┘  └─────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                         Agent Runtime                            │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   LLM Client    │  │   Tool System   │  │  Context Mgmt   │  │
│  │  (rig/openai)   │  │  ┌───────────┐  │  │  ┌───────────┐  │  │
│  │                 │  │  │  bash     │  │  │  │  History  │  │  │
│  │  - GPT-4        │  │  │  read     │  │  │  │  Summary  │  │  │
│  │  - Claude       │  │  │  write    │  │  │  │  Window   │  │  │
│  │  - Local        │  │  │  search   │  │  │  └───────────┘  │  │
│  │                 │  │  │  browser  │  │  │                 │  │
│  └─────────────────┘  │  └───────────┘  │  └─────────────────┘  │
│                       └─────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                      Infrastructure Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Media     │  │   Browser   │  │    Plugin System        │  │
│  │  Pipeline   │  │  (fantocc)  │  │    ┌─────────────────┐  │  │
│  │  ┌───────┐  │  │             │  │    │  WASM Runtime   │  │  │
│  │  │Image  │  │  │  - Chrome   │  │    │  ┌───────────┐  │  │  │
│  │  │Audio  │  │  │  - Firefox  │  │    │  │ Memory    │  │  │  │
│  │  │Video  │  │  │  - Safari   │  │    │  │ Auth      │  │  │  │
│  │  └───────┘  │  │             │  │    │  │ ...       │  │  │  │
│  └─────────────┘  └─────────────┘  │    │  └───────────┘  │  │  │
│                                    │    └─────────────────┘  │  │
└────────────────────────────────────┴──────────────────────────┘  │

┌─────────────────────────────────────────────────────────────────┐
│                        Mobile Layer                              │
│  ┌─────────────────┐  ┌─────────────────┐                      │
│  │   iOS (Swift)   │  │ Android (Kotlin)│                      │
│  │  ┌───────────┐  │  │  ┌───────────┐  │                      │
│  │  │  uniffi   │  │  │  │   JNI     │  │                      │
│  │  │  Bridge   │──┼──┼──┤   Bridge  │  │                      │
│  │  └───────────┘  │  │  └───────────┘  │                      │
│  └─────────────────┘  └─────────────────┘                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Crate Dependency Graph

```
openclaw-core
    ├── openclaw-errors
    └── (serde, chrono, uuid, etc.)

openclaw-storage
    ├── openclaw-core
    ├── openclaw-errors
    └── sqlx

openclaw-gateway
    ├── openclaw-core
    ├── openclaw-errors
    ├── openclaw-storage
    ├── axum
    ├── tokio
    └── tower

openclaw-channel-traits
    ├── openclaw-core
    └── async-trait

openclaw-telegram
    ├── openclaw-core
    ├── openclaw-channel-traits
    └── teloxide

openclaw-discord
    ├── openclaw-core
    ├── openclaw-channel-traits
    └── serenity

openclaw-agents
    ├── openclaw-core
    ├── openclaw-storage
    └── rig (or pi-bridge)

openclaw-tools
    ├── openclaw-core
    └── openclaw-errors

openclaw-cli
    ├── openclaw-core
    ├── openclaw-gateway
    ├── openclaw-config
    └── clap

openclaw-config
    ├── openclaw-core
    └── toml
```

---

## Data Flow

### Incoming Message Flow

```
1. Telegram Bot receives message
   ↓
2. TelegramChannel converts to openclaw::Message
   ↓
3. Gateway receives via ChannelRegistry
   ↓
4. Router applies routing rules
   ↓
5. SessionManager gets/ creates Session
   ↓
6. Storage persists message
   ↓
7. AgentRuntime processes with context
   ↓
8. ToolSystem executes any tool calls
   ↓
9. Response sent back via TelegramChannel
```

### WebSocket Client Flow

```
1. Client connects to /ws
   ↓
2. ConnectionManager creates Connection
   ↓
3. Handler authenticates client
   ↓
4. Client sends JSON-RPC request
   ↓
5. Gateway routes to appropriate handler
   ↓
6. Session/Agent/Channel operation
   ↓
7. JSON-RPC response returned
   ↓
8. Events broadcast to all clients
```

---

## Configuration Hierarchy

```
~/.config/openclaw/
├── openclaw.toml          # Main config
├── credentials/
│   ├── telegram.json      # Encrypted tokens
│   ├── discord.json
│   └── ...
├── sessions/
│   └── <session-id>.json  # Session data (SQLite: in DB)
└── plugins/
    └── <plugin>.wasm      # WASM plugins
```

### Config Precedence (high to low)

1. Environment variables (`OPENCLAW_*`)
2. Command-line flags
3. Config file (~/.config/openclaw/openclaw.toml)
4. Defaults

---

## Security Model

### Authentication

```rust
// Token-based auth for CLI/API
pub struct AuthToken {
    user_id: String,
    scopes: Vec<Scope>,
    expires_at: Timestamp,
}

// Channel-specific auth
pub struct ChannelAuth {
    channel_type: String,
    token: String,
    // Encrypted at rest
}
```

### Sandboxing

```rust
pub enum SandboxMode {
    Never,      // Main session only
    NonMain,    // Groups get Docker sandbox
    Always,     // Everything sandboxed
}

// Docker sandbox for tools
pub struct DockerSandbox {
    image: String,
    timeout: Duration,
    resource_limits: ResourceLimits,
}
```

---

## Performance Targets

| Metric | TypeScript | Rust Target | Improvement |
|--------|-----------|-------------|-------------|
| Message latency | 50ms | 10ms | 5x |
| Memory usage | 500MB | 200MB | 2.5x |
| CPU usage | 100% | 40% | 2.5x |
| Startup time | 3s | 500ms | 6x |
| Concurrent conns | 1,000 | 10,000 | 10x |

---

## API Compatibility

### TypeScript → Rust API Mapping

| TypeScript | Rust | Notes |
|------------|------|-------|
| `Message` | `Message` | Same structure |
| `Session` | `Session` | + Context |
| `Channel` | `Channel` trait | Async |
| `Tool` | `ToolHandler` | + Registry |
| `Config` | `Config` | TOML format |
| `Storage` | `Storage` trait | Multi-backend |

### WebSocket Protocol

**No changes required** - JSON-RPC 2.0 protocol identical

```json
// Request (same)
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "message.send",
  "params": {
    "channel_id": "telegram",
    "chat_id": "123456",
    "text": "Hello"
  }
}

// Response (same)
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "message_id": "...",
    "sent": true
  }
}
```

---

## Deployment Options

### 1. Single Binary

```bash
# Build
$ cargo build --release

# Run
$ ./openclaw gateway run
```

### 2. Docker

```bash
$ docker run -p 18789:18789 openclaw/gateway:latest
```

### 3. Systemd Service

```ini
# /etc/systemd/system/openclaw.service
[Unit]
Description=OpenClaw Gateway

[Service]
ExecStart=/usr/local/bin/openclaw gateway run
Restart=always
User=openclaw

[Install]
WantedBy=multi-user.target
```

### 4. Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: openclaw-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: openclaw
  template:
    spec:
      containers:
      - name: gateway
        image: openclaw/gateway:latest
        ports:
        - containerPort: 18789
```

---

## Monitoring & Observability

### Metrics (Prometheus)

```rust
// Gateway metrics
pub struct GatewayMetrics {
    connections_total: Counter,
    messages_total: CounterVec,  // by channel
    latency_histogram: HistogramVec,  // by operation
    errors_total: CounterVec,  // by type
}

// Channel metrics
pub struct ChannelMetrics {
    messages_sent: Counter,
    messages_received: Counter,
    errors: Counter,
    latency: Histogram,
}
```

### Tracing (OpenTelemetry)

```rust
#[tracing::instrument(skip(self, message))]
pub async fn handle_message(&self, message: Message) -> Result<()> {
    tracing::info!(message_id = %message.id, "Handling message");
    
    let result = self.process(message).await;
    
    tracing::info!(success = result.is_ok(), "Message handled");
    result
}
```

### Health Checks

```bash
$ curl http://localhost:18789/health
{
  "status": "healthy",
  "components": {
    "gateway": "healthy",
    "storage": "healthy",
    "telegram": "healthy",
    "discord": "degraded"
  }
}
```

---

## Development Workflow

### Local Development

```bash
# 1. Clone and build
$ git clone https://github.com/openclaw/openclaw.git
$ cd openclaw
$ cargo build

# 2. Run tests
$ cargo test --workspace

# 3. Run gateway
$ cargo run --bin openclaw -- gateway run --port 18789

# 4. Test WebSocket
$ wscat -c ws://localhost:18789/ws
> {"jsonrpc":"2.0","id":1,"method":"ping"}
```

### Testing Strategy

```
Unit Tests:       crates/*/src/*.rs (inline)
Integration:      tests/integration/*.rs
E2E Tests:        tests/e2e/*.rs
Channel Tests:    crates/channels/*/tests/*.rs
Benchmarks:       benches/*.rs
```

---

## Migration Checklist

### Phase 1-2: Foundation ✅
- [x] Core types
- [x] Error handling
- [x] Config system
- [x] Gateway server
- [x] CLI

### Phase 3: Storage & Channels 🔄
- [ ] SQLite storage
- [ ] PostgreSQL storage
- [ ] Telegram channel
- [ ] Discord channel
- [ ] Slack channel

### Phase 4: Agent & Tools
- [ ] Agent runtime
- [ ] Context management
- [ ] Tool system
- [ ] Bash tool
- [ ] File tools

### Phase 5-9: Complete
- [ ] All channels
- [ ] Media pipeline
- [ ] Browser automation
- [ ] Plugin system
- [ ] Mobile FFI
- [ ] Tests
- [ ] Deployment

---

**Architecture Version**: 1.0  
**Last Updated**: 2026-02-16  
**Maintainers**: OpenClaw Team
