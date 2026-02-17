# OpenClaw Full Rewrite: TypeScript → Rust
## Complete Migration Plan & Architecture

**Version**: 2026.2.16  
**Status**: Phase 1-2 Complete, Phase 3-9 In Progress  
**Target**: 100% Rust with strategic bridges

---

## Executive Summary

การ rewrite ทั้งหมดเป็น Rust แบ่งเป็น 9 phases:
- **Phase 1-2** (Month 1-2): ✅ Core foundation - DONE
- **Phase 3** (Month 2-3): 🔄 Storage & Channels - IN PROGRESS
- **Phase 4** (Month 3-5): 📋 Agent Runtime - PLANNED
- **Phase 5** (Month 4-6): 📋 Tools & Media - PLANNED
- **Phase 6** (Month 5-7): 📋 Browser & Extensions - PLANNED
- **Phase 7** (Month 6-8): 📋 Mobile & FFI - PLANNED
- **Phase 8** (Month 7-9): 📋 Testing & Optimization - PLANNED
- **Phase 9** (Month 8-10): 📋 Deployment & Release - PLANNED

**Total Timeline**: 10 months  
**Team Size**: 3-4 senior Rust developers

**Current Team (2026-02-17)**:
- **Max1**: Lead Developer (Rust expert) - Gateway, Core, CLI, Authentication
- **Max2**: Backend Engineer (Rust, Systems, Storage) - Telegram, Storage Integration, Tooling
- **Max3**: Channel Specialist (Messaging APIs, Integration) - Discord, Slack, WhatsApp, Signal
- **Max4**: Async & Storage Specialist (Rust, tokio, sqlx) - Storage Layer, SQLite/PostgreSQL, Integration Testing, CI/CD

---

**Progress Update (2026-02-17)**: 
- Phase 1-2 completed ✅
- Phase 3 (Storage & Channels) in progress 🔄
  - **Gateway integration**: ✅ COMPLETE (ChannelRegistry, auto-start, health checks, message routing)
  - **Telegram channel**: ✅ COMPLETE (teloxide polling, media support, error handling) - integration tests in progress
  - **Discord channel**: 🔄 PARTIAL (basic send/receive, async fixes) - needs config parsing & testing
  - **Storage layer**: ✅ COMPLETE (trait + SQLite + Memory backends, migrations, unit tests)
  - **Authentication**: 🔄 PLANNED (API key + JWT design)
- Next: Gateway-Storage integration, Discord completion, authentication implementation

---

## Phase 1: Core Foundation ✅ COMPLETE

### 1.1 Workspace Structure
```
crates/
├── openclaw-core/          # Core types, entities, config
├── openclaw-errors/        # Error handling
├── openclaw-config/        # Configuration management
├── openclaw-storage/       # Database abstraction (SQLite, Postgres, Memory)
├── openclaw-gateway/       # WebSocket server
├── openclaw-sessions/      # Session management
├── openclaw-routing/       # Message routing
├── openclaw-agents/        # Agent runtime
├── openclaw-tools/         # Tool system
├── openclaw-media/         # Media processing
├── openclaw-browser/       # Browser automation
├── openclaw-cli/           # Command-line interface
├── openclaw-tui/           # Terminal UI
├── openclaw-plugin-sdk/    # Plugin SDK
├── openclaw-plugin-host/   # Plugin host
├── channels/
│   ├── openclaw-channel-traits/    # Channel abstractions
│   ├── openclaw-telegram/          # Telegram (teloxide)
│   ├── openclaw-discord/           # Discord (serenity)
│   ├── openclaw-slack/             # Slack (slack-morphism)
│   ├── openclaw-whatsapp/          # WhatsApp (bridge/pure)
│   ├── openclaw-signal/            # Signal (libsignal)
│   ├── openclaw-imessage/          # iMessage (bridge)
│   ├── openclaw-web/               # Web/WebSocket
│   ├── openclaw-matrix/            # Matrix
│   └── openclaw-msteams/           # Microsoft Teams
└── mobile/
    ├── openclaw-mobile-core/       # Shared mobile logic
    ├── openclaw-mobile-ios/        # iOS FFI
    └── openclaw-mobile-android/    # Android FFI
```

### 1.2 Core Types ✅

#### Message System
```rust
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub direction: Direction,
    pub sender: Sender,
    pub chat: Chat,
    pub content: MessageContent,
    pub reply_to: Option<MessageId>,
    pub metadata: Metadata,
    pub created_at: Timestamp,
    pub edited_at: Option<Timestamp>,
}

pub enum MessageContent {
    Text(String),
    Media(MediaContent),
    Location(LocationContent),
    Contact(ContactContent),
    Poll(PollContent),
    System(SystemMessage),
}
```

#### Session Management
```rust
pub struct Session {
    pub id: SessionId,
    pub channel_id: ChannelId,
    pub user_id: String,
    pub chat_id: String,
    pub config: SessionConfig,
    pub state: SessionState,
    pub context: SessionContext,
}

pub struct SessionConfig {
    pub model: String,
    pub sandbox_mode: SandboxMode,      // Never | NonMain | Always
    pub tools_enabled: bool,
    pub reply_back: ReplyBackMode,      // Always | Mention | DirectOnly | Never
    pub queue_mode: QueueMode,          // Immediate | Queue | Batch
    pub max_context_length: usize,
    pub timeout_seconds: u64,
}
```

### 1.3 Error Handling ✅

```rust
#[derive(Error, Debug)]
pub enum OpenClawError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Channel error: {channel} - {message}")]
    Channel { channel: String, message: String },
    
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("Agent error: {0}")]
    Agent(String),
    
    #[error("Tool error: {0}")]
    Tool(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    // ... more variants
}
```

### 1.4 Configuration ✅

**TOML Format** (replacing JSON):
```toml
[gateway]
bind_address = "0.0.0.0"
port = 18789
max_connections = 1000

[agents]
model = "gpt-4"

[agents.sandbox]
mode = "non_main"
timeout_secs = 300

[storage]
backend = "sqlite"  # or "postgres" | "memory"

[channels.telegram]
enabled = true
token = "${TELEGRAM_BOT_TOKEN}"

[channels.discord]
enabled = true
token = "${DISCORD_BOT_TOKEN}"
```

---

## Phase 2: Gateway & Infrastructure ✅ COMPLETE

### 2.1 WebSocket Server ✅

**Axum-based implementation**:
```rust
pub struct GatewayServer {
    state: GatewayState,
    router: Router,
}

impl GatewayServer {
    pub fn new(config: Config) -> Self {
        let state = GatewayState::new(config);
        let router = Self::build_router(state.clone());
        Self { state, router }
    }
    
    pub async fn run(self, addr: &str) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
```

**Features**:
- ✅ WebSocket upgrade handling
- ✅ JSON-RPC 2.0 protocol
- ✅ Connection management (DashMap)
- ✅ Event broadcasting (tokio::sync::broadcast)
- ✅ Health checks
- ✅ Rate limiting (tower middleware)
- ✅ CORS support

### 2.2 Protocol ✅

**JSON-RPC Methods**:
```rust
pub enum GatewayMethod {
    // Auth
    Authenticate,
    
    // Sessions
    SessionCreate,
    SessionGet,
    SessionList,
    SessionUpdate,
    SessionDelete,
    
    // Messages
    MessageSend,
    MessageReceive,
    MessageHistory,
    
    // Channels
    ChannelList,
    ChannelStatus,
    ChannelConnect,
    ChannelDisconnect,
    
    // Agents
    AgentRun,
    AgentStop,
    AgentStatus,
    ToolCall,
    
    // System
    Ping,
    Subscribe,
    Unsubscribe,
}
```

### 2.3 CLI ✅

**Commands implemented**:
```bash
openclaw gateway run --bind 0.0.0.0 --port 18789
openclaw gateway stop
openclaw gateway status --deep

openclaw config show
openclaw config get gateway.port
openclaw config set agents.model gpt-4
openclaw config edit
openclaw config validate

openclaw channels list
openclaw channels connect telegram
openclaw channels status --all

openclaw agent run --message "Hello" --thinking medium
openclaw agent status

openclaw doctor
openclaw wizard
```

---

## Phase 3: Storage & Channels 🔄 IN PROGRESS

### 3.1 Storage Layer 🔄

**Multi-backend support**:

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    // Sessions
    async fn get_session(&self, session_id: SessionId) -> Result<Option<Session>>;
    async fn save_session(&self, session: &Session) -> Result<()>;
    
    // Messages
    async fn get_message(&self, message_id: &str) -> Result<Option<Message>>;
    async fn save_message(&self, message: &Message) -> Result<()>;
    
    // Users
    async fn get_user(&self, channel_id: &str, user_id: &str) -> Result<Option<User>>;
    async fn save_user(&self, user: &User) -> Result<()>;
    
    // Config
    async fn get_config_value(&self, key: &str) -> Result<Option<String>>;
    async fn set_config_value(&self, key: &str, value: &str) -> Result<()>;
    
    // Health
    async fn health_check(&self) -> Result<()>;
    async fn migrate(&self) -> Result<()>;
}
```

**Backends**:
- ✅ SQLite (sqlx) - Default, file-based
- 🔄 PostgreSQL (sqlx) - Production scale
- ✅ Memory (DashMap) - Testing

**Database Schema**:
```sql
-- Sessions
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    config JSON NOT NULL,
    state TEXT NOT NULL,
    context JSON NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    last_activity_at TIMESTAMP NOT NULL
);

-- Messages
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    session_id TEXT REFERENCES sessions(id),
    channel_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    user_id TEXT,
    direction TEXT NOT NULL,
    content JSON NOT NULL,
    metadata JSON,
    created_at TIMESTAMP NOT NULL
);

-- Users
CREATE TABLE users (
    channel_id TEXT NOT NULL,
    channel_user_id TEXT NOT NULL,
    global_user_id TEXT,
    display_name TEXT NOT NULL,
    metadata JSON,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (channel_id, channel_user_id)
);

-- Config
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
```

### 3.2 Channel Implementations 🔄

#### Channel Trait
```rust
#[async_trait]
pub trait Channel: Send + Sync {
    fn channel_type(&self) -> &'static str;
    fn name(&self) -> &str;
    
    async fn is_connected(&self) -> bool;
    async fn connect(&self) -> Result<()>;
    async fn disconnect(&self) -> Result<()>;
    
    async fn send_message(&self, chat_id: &str, content: OutgoingContent) -> Result<Message>;
    async fn edit_message(&self, chat_id: &str, message_id: &str, content: OutgoingContent) -> Result<Message>;
    async fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<()>;
    
    async fn get_chat(&self, chat_id: &str) -> Result<Chat>;
    async fn get_user(&self, user_id: &str) -> Result<User>;
    
    async fn health_check(&self) -> HealthStatus;
}
```

#### Implementation Matrix (Updated 2026-02-17)

| Channel | Library | Status | Difficulty | ETA |
|---------|---------|--------|------------|-----|
| **Telegram** | teloxide | ✅ COMPLETE (needs integration tests) | Easy | Week 3 ✅ |
| **Discord** | serenity | 🔄 WIP (basic send/receive, needs config parsing) | Easy | Week 4 |
| **Slack** | slack-morphism | 📋 Planned | Easy | Week 4-5 |
| **Signal** | libsignal-client | 📋 Planned | Medium | Week 6-7 |
| **WhatsApp** | bridge/custom | 🔴 Blocked | Hard | TBD |
| **iMessage** | bridge | 📋 Planned | Hard | Week 8 |
| **Web** | axum/ws | 📋 Planned | Medium | Week 5-6 |
| **Matrix** | matrix-rust-sdk | 📋 Planned | Medium | Week 7-8 |
| **Web** | axum/ws | 📋 Planned | Medium | Week 5-6 |
| **Matrix** | matrix-rust-sdk | 📋 Planned | Medium | Week 7-8 |

#### Telegram Implementation (teloxide)

```rust
use teloxide::prelude::*;

pub struct TelegramChannel {
    bot: Bot,
    handler: Arc<dyn MessageHandler>,
}

#[async_trait]
impl Channel for TelegramChannel {
    fn channel_type(&self) -> &'static str {
        "telegram"
    }
    
    async fn connect(&self) -> Result<()> {
        // Start update polling
        let handler = Update::filter_message()
            .endpoint(|msg: Message, bot: AutoSend<Bot>| async move {
                // Convert and handle message
                Ok::<_, teloxide::RequestError>(())
            });
        
        Dispatcher::builder(self.bot.clone(), handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        
        Ok(())
    }
    
    async fn send_message(&self, chat_id: &str, content: OutgoingContent) -> Result<Message> {
        let chat_id = ChatId(chat_id.parse()?);
        
        let msg = if let Some(text) = content.text {
            self.bot.send_message(chat_id, text).await?
        } else if let Some(media) = content.media {
            // Handle media
            self.bot.send_photo(chat_id, InputFile::memory(media.data.unwrap())).await?
        } else {
            return Err(OpenClawError::Channel {
                channel: "telegram".to_string(),
                message: "Empty content".to_string(),
            });
        };
        
        // Convert teloxide Message to openclaw Message
        Ok(convert_message(msg)?)
    }
}
```

#### Discord Implementation (serenity)

```rust
use serenity::prelude::*;

pub struct DiscordChannel {
    client: Client,
    handler: Arc<dyn MessageHandler>,
}

#[async_trait]
impl Channel for DiscordChannel {
    fn channel_type(&self) -> &'static str {
        "discord"
    }
    
    async fn send_message(&self, channel_id: &str, content: OutgoingContent) -> Result<Message> {
        let channel_id = channel_id.parse::<u64>()?;
        let channel = self.client.cache.guild_channel(channel_id)
            .ok_or_else(|| OpenClawError::not_found("channel"))?;
        
        let builder = CreateMessage::new();
        let builder = if let Some(text) = content.text {
            builder.content(text)
        } else {
            builder
        };
        
        let msg = channel.send_message(&self.client.http, builder).await?;
        Ok(convert_message(msg)?)
    }
}
```

#### WhatsApp Strategy (CRITICAL)

**Problem**: No mature Rust WhatsApp Web library

**Options**:
1. **Bridge Approach** (Recommended for v1)
   ```rust
   // Use Baileys (TypeScript) via Node-API or gRPC
   pub struct WhatsAppBridge {
       node_runtime: NodeRuntime,
       event_receiver: mpsc::Receiver<WhatsAppEvent>,
   }
   
   impl WhatsAppBridge {
       async fn connect(&self) -> Result<()> {
           // Spawn Node.js process with Baileys
           // Communicate via IPC/gRPC
       }
   }
   ```

2. **Pure Rust** (Long-term)
   - Implement WhatsApp Web protocol from scratch
   - Curve25519 encryption
   - WebSocket binary protocol
   - QR code pairing
   - **ETA**: 6+ months

**Decision**: Start with bridge, migrate to pure Rust later

---

## Phase 4: Agent Runtime (Month 3-5)

### 4.1 Pi Integration

**Challenge**: Pi is proprietary (Mario Zechner)

**Options**:

1. **Keep Pi** (Bridge)
   - RPC communication with TypeScript Pi
   - Minimal changes to existing behavior

2. **Migrate to Open Source** (Recommended)
   ```rust
   // Use rig framework
   use rig::providers::openai;
   
   pub struct RigAgent {
       client: openai::Client,
       model: String,
       tools: ToolRegistry,
   }
   
   impl RigAgent {
       pub async fn run(&self, message: &str, context: &SessionContext) -> Result<String> {
           let agent = self.client
               .agent(&self.model)
               .preamble(&self.build_system_prompt())
               .build();
           
           let response = agent.prompt(message).await?;
           Ok(response)
       }
   }
   ```

**Recommendation**: Phase out Pi, adopt `rig` or similar

### 4.2 Context Management

```rust
pub struct ContextManager {
    max_tokens: usize,
    summarization_threshold: usize,
}

impl ContextManager {
    pub fn build_context(&self, session: &Session) -> Vec<ContextMessage> {
        let messages = &session.context.messages;
        
        // Check token limit
        let token_count = self.estimate_tokens(messages);
        
        if token_count > self.max_tokens {
            // Summarize older messages
            self.summarize_and_trim(messages)
        } else {
            messages.clone()
        }
    }
    
    fn estimate_tokens(&self, messages: &[ContextMessage]) -> usize {
        // Simple estimation: 4 chars ≈ 1 token
        messages.iter()
            .map(|m| m.content.len() / 4)
            .sum()
    }
}
```

### 4.3 Tool System

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolHandler>>,
}

#[async_trait]
pub trait ToolHandler: Send + Sync {
    fn definition(&self) -> &Tool;
    async fn execute(&self, args: Value) -> Result<ToolResult>;
}

// Built-in tools
pub struct BashTool;
pub struct ReadFileTool;
pub struct WriteFileTool;
pub struct SearchTool;
pub struct BrowserTool;
```

---

## Phase 5: Tools & Media (Month 4-6)

### 5.1 Tool Implementations

#### Bash Tool (with sandboxing)
```rust
pub struct BashTool {
    sandbox: SandboxConfig,
}

#[async_trait]
impl ToolHandler for BashTool {
    fn definition(&self) -> &Tool {
        &Tool {
            name: "bash",
            description: "Execute shell commands",
            parameters: vec![
                ToolParameter {
                    name: "command".to_string(),
                    description: "Shell command to execute".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
            ],
            dangerous: true,
        }
    }
    
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let command = args.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing command".to_string()))?;
        
        match self.sandbox.mode {
            SandboxMode::Never => {
                // Run directly (main session only)
                run_command(command).await
            }
            SandboxMode::NonMain => {
                // Docker sandbox
                run_in_docker(command).await
            }
            SandboxMode::Always => {
                // Always sandbox
                run_in_docker(command).await
            }
        }
    }
}
```

### 5.2 Media Pipeline

**Replace Sharp/ImageMagick**:

| Function | TypeScript | Rust |
|----------|-----------|------|
| Image resize | sharp | `image` crate |
| Image format | sharp | `image` crate |
| SVG render | resvg | `resvg` crate |
| Audio decode | ffmpeg | `symphonia` |
| Audio encode | ffmpeg | `symphonia` |
| Video | ffmpeg | `ffmpeg-next` |

```rust
pub struct MediaProcessor {
    image_ops: ImageOps,
    audio_ops: AudioOps,
}

impl MediaProcessor {
    pub async fn process_image(&self, input: &[u8], ops: ImageOperations) -> Result<Vec<u8>> {
        let img = image::load_from_memory(input)?;
        
        let processed = match ops {
            ImageOperations::Resize { width, height } => {
                img.resize(width, height, image::imageops::FilterType::Lanczos3)
            }
            ImageOperations::Crop { x, y, width, height } => {
                img.crop(x, y, width, height)
            }
            ImageOperations::Format(format) => {
                // Convert format
                img
            }
        };
        
        let mut output = Vec::new();
        processed.write_to(&mut std::io::Cursor::new(&mut output), image::ImageFormat::Jpeg)?;
        
        Ok(output)
    }
}
```

---

## Phase 6: Browser & Extensions (Month 5-7)

### 6.1 Browser Automation

**Replace Playwright** → `fantoccini` (WebDriver)

```rust
use fantoccini::{Client, Locator};

pub struct BrowserClient {
    client: Client,
}

impl BrowserClient {
    pub async fn new() -> Result<Self> {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await?;
        
        Ok(Self { client })
    }
    
    pub async fn goto(&self, url: &str) -> Result<()> {
        self.client.goto(url).await?;
        Ok(())
    }
    
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        let screenshot = self.client.screenshot().await?;
        Ok(screenshot)
    }
    
    pub async fn click(&self, selector: &str) -> Result<()> {
        let elem = self.client.find(Locator::Css(selector)).await?;
        elem.click().await?;
        Ok(())
    }
    
    pub async fn fill(&self, selector: &str, text: &str) -> Result<()> {
        let elem = self.client.find(Locator::Css(selector)).await?;
        elem.send_keys(text).await?;
        Ok(())
    }
}
```

### 6.2 Plugin System

**WASM-based plugins**:

```rust
use wasmtime::{Engine, Module, Store, Instance};

pub struct PluginHost {
    engine: Engine,
}

impl PluginHost {
    pub fn load_plugin(&self, wasm_bytes: &[u8]) -> Result<Plugin> {
        let module = Module::new(&self.engine, wasm_bytes)?;
        
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        
        Ok(Plugin {
            instance,
            store,
        })
    }
}

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_message(&mut self, message: &Message) -> Result<()>;
}
```

---

## Phase 7: Mobile & FFI (Month 6-8)

### 7.1 iOS/macOS (uniffi)

```rust
// crates/openclaw-mobile-ios/src/lib.rs
use uniffi;

#[uniffi::export]
pub fn init_gateway(config_json: String) -> Result<Arc<Gateway>, String> {
    let config: Config = serde_json::from_str(&config_json)
        .map_err(|e| e.to_string())?;
    
    let gateway = Gateway::new(config);
    Ok(Arc::new(gateway))
}

#[uniffi::export]
pub async fn send_message(
    gateway: Arc<Gateway>,
    channel_id: String,
    chat_id: String,
    text: String
) -> Result<String, String> {
    let content = OutgoingContent::text(text);
    
    let message = gateway
        .send_message(&channel_id, &chat_id, content)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(message.id.to_string())
}
```

### 7.2 Android (JNI or flutter_rust_bridge)

```rust
// crates/openclaw-mobile-android/src/lib.rs
use jni::JNIEnv;
use jni::objects::JString;
use jni::signature::JavaType;

#[no_mangle]
pub extern "C" fn Java_com_openclaw_Gateway_init(
    env: JNIEnv,
    _class: JClass,
    config_json: JString,
) -> jlong {
    let config_str: String = env.get_string(config_json).unwrap().into();
    let config: Config = serde_json::from_str(&config_str).unwrap();
    
    let gateway = Box::new(Gateway::new(config));
    Box::into_raw(gateway) as jlong
}
```

---

## Phase 8: Testing (Month 7-9)

### 8.1 Testing Strategy

```rust
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_session_creation() {
        let storage = MemoryStorage::new();
        let session = Session::new(channel_id, "user123", "chat456");
        
        storage.save_session(&session).await.unwrap();
        
        let retrieved = storage.get_session(session.id).await.unwrap();
        assert!(retrieved.is_some());
    }
}

// Integration tests
#[tokio::test]
async fn test_gateway_websocket() {
    let gateway = TestGateway::new().await;
    let mut client = gateway.connect_ws().await;
    
    client.send(json!({"method": "ping"})).await;
    let response = client.recv().await;
    
    assert_eq!(response["result"]["pong"], true);
}

// Channel mocks
mockall::mock! {
    pub Channel {}
    
    #[async_trait]
    impl Channel for Channel {
        fn channel_type(&self) -> &'static str;
        async fn send_message(&self, chat_id: &str, content: OutgoingContent) -> Result<Message>;
    }
}
```

### 8.2 Performance Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_message_routing(c: &mut Criterion) {
    let router = Router::new(vec![/* rules */]);
    let message = create_test_message();
    
    c.bench_function("route_message", |b| {
        b.iter(|| {
            router.route(black_box(&message))
        })
    });
}

criterion_group!(benches, benchmark_message_routing);
criterion_main!(benches);
```

---

## Phase 9: Deployment (Month 8-10)

### 9.1 Binary Releases

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin
          - x86_64-pc-windows-msvc
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload
        uses: actions/upload-release-asset@v1
        with:
          asset_path: target/${{ matrix.target }}/release/openclaw
```

### 9.2 Docker

```dockerfile
# Dockerfile
FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/openclaw /usr/local/bin/
EXPOSE 18789
CMD ["openclaw", "gateway", "run"]
```

### 9.3 Package Managers

- **Homebrew**: `brew install openclaw`
- **APT**: `.deb` packages
- **YUM**: `.rpm` packages
- **Cargo**: `cargo install openclaw-cli`
- **npm**: Wrapper for `openclaw` binary

---

## Migration Strategy

### Incremental Migration

```
Month 1-2:   Core + Gateway (Rust)     |  Channels (TypeScript bridge)
Month 3-4:   Storage (Rust)            |  Channels (Rust - Telegram/Discord)
Month 5-6:   Agent (Rust or bridge)    |  Tools (Rust)
Month 7-8:   Media (Rust)              |  Browser (Rust)
Month 9-10:  Mobile (FFI)              |  Extensions (WASM)
```

### Dual Runtime

During migration, run both:
```
TypeScript Gateway (legacy)  ←→  Rust Gateway (new)
     ↓                              ↓
   Channels                      Channels (Rust)
   Agent (Pi)                    Agent (Rust/rig)
```

Use feature flags:
```toml
[features]
default = ["rust-gateway"]
legacy-gateway = ["ts-bridge"]
```

---

## Risk Assessment

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| WhatsApp complexity | High | High | Bridge approach |
| Pi dependency | High | Medium | Migrate to rig |
| Performance regression | High | Low | Benchmark early |
| Team Rust expertise | Medium | Medium | Training/hiring |
| Extension compatibility | Medium | Medium | WASM sandbox |
| Timeline slip | Medium | Medium | Phased approach |

---

## Success Criteria

### Performance
- [ ] 2x faster than TypeScript
- [ ] 50% less memory usage
- [ ] <10ms p99 latency for message routing

### Reliability
- [ ] 99.9% uptime
- [ ] Zero data loss during migration
- [ ] Graceful degradation

### Compatibility
- [ ] All 36 extensions work (via WASM or bridge)
- [ ] Mobile apps fully functional
- [ ] CLI parity with TypeScript

### Developer Experience
- [ ] Comprehensive documentation
- [ ] Easy plugin development
- [ ] Clear error messages

---

## Resources

### Team
- 3-4 Senior Rust developers
- 1 DevOps engineer
- 1 QA engineer
- Part-time TypeScript maintenance

### Infrastructure
- CI/CD (GitHub Actions)
- Test environments
- Staging gateway
- Package repositories

### Budget Estimate
- Development: $$$$ (6-12 months salaries)
- Infrastructure: $$ (servers, CI minutes)
- Tools/Services: $ (Sentry, monitoring)

---

## Team Coordination Update (2026-02-17 - Session 14)

**Coordinator**: Max4 (Async & Storage Specialist)

### Current Sprint Focus: Phase 3 Completion (Storage & Channels Integration)

**Team Status**:
- **Max1**: Gateway-Storage Integration + Authentication (HIGH priority)
- **Max2**: Telegram Testing + Storage Schema Review (HIGH priority)
- **Max3**: Discord Completion + Slack Preparation (MEDIUM-HIGH priority)
- **Max4**: Storage Integration Support + CI/CD + Coordination (HIGH priority)

### Immediate Actions (Session 14 - 2026-02-17)

| Priority | Task | Owner | Deliverable |
|----------|------|-------|-------------|
| CRITICAL | Gateway-Storage Integration | Max1 + Max4 | PR with integrated GatewayState |
| CRITICAL | Session JSON-RPC Methods | Max1 | `session_create`, `session_list`, `session_get` |
| CRITICAL | Telegram Mock Tests | Max2 | Passing tests with mockall |
| CRITICAL | Discord Config Parsing | Max3 | Complete TOML parsing + channel mapping |
| HIGH | API Key Authentication | Max1 | Auth middleware skeleton |
| HIGH | CI/CD Pipeline | Max4 | GitHub Actions workflow |
| HIGH | Storage Benchmarks | Max4 | Performance tests |

### Coordination Mechanism
- **Progress Updates**: Every 2 hours in `RUST_MIGRATION_PROGRESS.md`
- **Blocker Escalation**: Report within 1 hour if blocked
- **Code Review**: PR + team review for all features
- **Daily Sync**: End-of-day summary in progress file

---

## Next Steps

### Immediate (Session 14 - 2026-02-17) ⏰ IN PROGRESS
1. 🔄 **Gateway-Storage Integration** - Max4 + Max1 (integrate Storage trait into GatewayState)
2. 🔄 **Telegram Mock Testing** - Max2 (complete teloxide mocks, polling tests, integration tests)
3. 🔄 **Discord Completion** - Max3 (config parsing, channel mapping, real token testing)
4. 🔄 **Authentication Implementation** - Max1 (API key + JWT design and implementation)
5. 🔄 **Storage Schema Review** - Max4 + Max2 (finalize schema for Gateway integration)
6. 🔄 **CI/CD Setup** - Max4 (GitHub Actions for Rust builds)

### Week 3-4 (2026-02-19 to 2026-03-04)
1. 🔜 Slack Channel Implementation - Max3 (after Discord completion)
2. 🔜 Agent Runtime Decision - Team (rig vs Pi vs custom)
3. 🔜 Tool System Implementation - Max2 + Max4 (bash, read, write tools with sandboxing)
4. 🔜 Integration Testing Suite - Team (gateway + channels + storage integration tests)
5. 🔜 Media Pipeline Prototype - Max1 (image/audio processing with Rust crates)

### Month 2 (2026-03-05 to 2026-04-05)
1. Agent Runtime Implementation (based on decision)
2. Tool System Completion (all core tools)
3. Media Pipeline Implementation (replace Sharp/ffmpeg)
4. Browser Automation (fantoccini integration)
5. Plugin System Design (WASM vs native)

---

## Conclusion

Full rewrite เป็นไปได้และมีประโยชน์:

**Pros**:
- Performance improvement (2x+)
- Memory efficiency (50% less)
- Type safety (eliminate runtime errors)
- Better concurrency (tokio)
- Single binary deployment

**Cons**:
- 10-month timeline
- WhatsApp complexity
- Learning curve for team

**Recommendation**: Proceed with phased approach, start with Gateway + Telegram + Discord.

---

**Document Version**: 1.1  
**Last Updated**: 2026-02-17  
**Next Review**: 2026-02-24
