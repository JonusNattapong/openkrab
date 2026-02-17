---
title: Pi é›†æˆæž¶æž„
x-i18n:
  generated_at: "2026-02-03T07:53:24Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 98b12f1211f70b1a25f58e68c7a4d0fe3827412ca53ba0ea2cd41ac9c0448458
  source_path: pi.md
  workflow: 15
---

# Pi é›†æˆæž¶æž„

æœ¬æ–‡æ¡£æè¿°äº† KrabKrab å¦‚ä½•ä¸Ž [pi-coding-agent](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent) åŠå…¶ç›¸å…³åŒ…ï¼ˆ`pi-ai`ã€`pi-agent-core`ã€`pi-tui`ï¼‰é›†æˆä»¥å®žçŽ°å…¶ AI æ™ºèƒ½ä½“èƒ½åŠ›ã€‚

## æ¦‚è¿°

KrabKrab ä½¿ç”¨ pi SDK å°† AI ç¼–ç æ™ºèƒ½ä½“åµŒå…¥åˆ°å…¶æ¶ˆæ¯ Gateway ç½‘å…³æž¶æž„ä¸­ã€‚KrabKrab ä¸æ˜¯å°† pi ä½œä¸ºå­è¿›ç¨‹ç”Ÿæˆæˆ–ä½¿ç”¨ RPC æ¨¡å¼ï¼Œè€Œæ˜¯é€šè¿‡ `createAgentSession()` ç›´æŽ¥å¯¼å…¥å¹¶å®žä¾‹åŒ– pi çš„ `AgentSession`ã€‚è¿™ç§åµŒå…¥å¼æ–¹æ³•æä¾›äº†ï¼š

- å¯¹ä¼šè¯ç”Ÿå‘½å‘¨æœŸå’Œäº‹ä»¶å¤„ç†çš„å®Œå…¨æŽ§åˆ¶
- è‡ªå®šä¹‰å·¥å…·æ³¨å…¥ï¼ˆæ¶ˆæ¯ã€æ²™ç®±ã€æ¸ é“ç‰¹å®šæ“ä½œï¼‰
- æ¯ä¸ªæ¸ é“/ä¸Šä¸‹æ–‡çš„ç³»ç»Ÿæç¤ºè‡ªå®šä¹‰
- æ”¯æŒåˆ†æ”¯/åŽ‹ç¼©çš„ä¼šè¯æŒä¹…åŒ–
- å¸¦æ•…éšœè½¬ç§»çš„å¤šè´¦æˆ·è®¤è¯é…ç½®æ–‡ä»¶è½®æ¢
- ä¸Žæä¾›å•†æ— å…³çš„æ¨¡åž‹åˆ‡æ¢

## åŒ…ä¾èµ–

```json
{
  "@mariozechner/pi-agent-core": "0.49.3",
  "@mariozechner/pi-ai": "0.49.3",
  "@mariozechner/pi-coding-agent": "0.49.3",
  "@mariozechner/pi-tui": "0.49.3"
}
```

| åŒ…                | ç”¨é€”                                                                                       |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `pi-ai`           | æ ¸å¿ƒ LLM æŠ½è±¡ï¼š`Model`ã€`streamSimple`ã€æ¶ˆæ¯ç±»åž‹ã€æä¾›å•† API                               |
| `pi-agent-core`   | æ™ºèƒ½ä½“å¾ªçŽ¯ã€å·¥å…·æ‰§è¡Œã€`AgentMessage` ç±»åž‹                                                  |
| `pi-coding-agent` | é«˜çº§ SDKï¼š`createAgentSession`ã€`SessionManager`ã€`AuthStorage`ã€`ModelRegistry`ã€å†…ç½®å·¥å…· |
| `pi-tui`          | ç»ˆç«¯ UI ç»„ä»¶ï¼ˆç”¨äºŽ KrabKrab çš„æœ¬åœ° TUI æ¨¡å¼ï¼‰                                              |

## æ–‡ä»¶ç»“æž„

```
src/agents/
â”œâ”€â”€ pi-embedded-runner.ts          # Re-exports from pi-embedded-runner/
â”œâ”€â”€ pi-embedded-runner/
â”‚   â”œâ”€â”€ run.ts                     # Main entry: runEmbeddedPiAgent()
â”‚   â”œâ”€â”€ run/
â”‚   â”‚   â”œâ”€â”€ attempt.ts             # Single attempt logic with session setup
â”‚   â”‚   â”œâ”€â”€ params.ts              # RunEmbeddedPiAgentParams type
â”‚   â”‚   â”œâ”€â”€ payloads.ts            # Build response payloads from run results
â”‚   â”‚   â”œâ”€â”€ images.ts              # Vision model image injection
â”‚   â”‚   â””â”€â”€ types.ts               # EmbeddedRunAttemptResult
â”‚   â”œâ”€â”€ abort.ts                   # Abort error detection
â”‚   â”œâ”€â”€ cache-ttl.ts               # Cache TTL tracking for context pruning
â”‚   â”œâ”€â”€ compact.ts                 # Manual/auto compaction logic
â”‚   â”œâ”€â”€ extensions.ts              # Load pi extensions for embedded runs
â”‚   â”œâ”€â”€ extra-params.ts            # Provider-specific stream params
â”‚   â”œâ”€â”€ google.ts                  # Google/Gemini turn ordering fixes
â”‚   â”œâ”€â”€ history.ts                 # History limiting (DM vs group)
â”‚   â”œâ”€â”€ lanes.ts                   # Session/global command lanes
â”‚   â”œâ”€â”€ logger.ts                  # Subsystem logger
â”‚   â”œâ”€â”€ model.ts                   # Model resolution via ModelRegistry
â”‚   â”œâ”€â”€ runs.ts                    # Active run tracking, abort, queue
â”‚   â”œâ”€â”€ sandbox-info.ts            # Sandbox info for system prompt
â”‚   â”œâ”€â”€ session-manager-cache.ts   # SessionManager instance caching
â”‚   â”œâ”€â”€ session-manager-init.ts    # Session file initialization
â”‚   â”œâ”€â”€ system-prompt.ts           # System prompt builder
â”‚   â”œâ”€â”€ tool-split.ts              # Split tools into builtIn vs custom
â”‚   â”œâ”€â”€ types.ts                   # EmbeddedPiAgentMeta, EmbeddedPiRunResult
â”‚   â””â”€â”€ utils.ts                   # ThinkLevel mapping, error description
â”œâ”€â”€ pi-embedded-subscribe.ts       # Session event subscription/dispatch
â”œâ”€â”€ pi-embedded-subscribe.types.ts # SubscribeEmbeddedPiSessionParams
â”œâ”€â”€ pi-embedded-subscribe.handlers.ts # Event handler factory
â”œâ”€â”€ pi-embedded-subscribe.handlers.lifecycle.ts
â”œâ”€â”€ pi-embedded-subscribe.handlers.types.ts
â”œâ”€â”€ pi-embedded-block-chunker.ts   # Streaming block reply chunking
â”œâ”€â”€ pi-embedded-messaging.ts       # Messaging tool sent tracking
â”œâ”€â”€ pi-embedded-helpers.ts         # Error classification, turn validation
â”œâ”€â”€ pi-embedded-helpers/           # Helper modules
â”œâ”€â”€ pi-embedded-utils.ts           # Formatting utilities
â”œâ”€â”€ pi-tools.ts                    # createkrabkrabCodingTools()
â”œâ”€â”€ pi-tools.abort.ts              # AbortSignal wrapping for tools
â”œâ”€â”€ pi-tools.policy.ts             # Tool allowlist/denylist policy
â”œâ”€â”€ pi-tools.read.ts               # Read tool customizations
â”œâ”€â”€ pi-tools.schema.ts             # Tool schema normalization
â”œâ”€â”€ pi-tools.types.ts              # AnyAgentTool type alias
â”œâ”€â”€ pi-tool-definition-adapter.ts  # AgentTool -> ToolDefinition adapter
â”œâ”€â”€ pi-settings.ts                 # Settings overrides
â”œâ”€â”€ pi-extensions/                 # Custom pi extensions
â”‚   â”œâ”€â”€ compaction-safeguard.ts    # Safeguard extension
â”‚   â”œâ”€â”€ compaction-safeguard-runtime.ts
â”‚   â”œâ”€â”€ context-pruning.ts         # Cache-TTL context pruning extension
â”‚   â””â”€â”€ context-pruning/
â”œâ”€â”€ model-auth.ts                  # Auth profile resolution
â”œâ”€â”€ auth-profiles.ts               # Profile store, cooldown, failover
â”œâ”€â”€ model-selection.ts             # Default model resolution
â”œâ”€â”€ models-config.ts               # models.json generation
â”œâ”€â”€ model-catalog.ts               # Model catalog cache
â”œâ”€â”€ context-window-guard.ts        # Context window validation
â”œâ”€â”€ failover-error.ts              # FailoverError class
â”œâ”€â”€ defaults.ts                    # DEFAULT_PROVIDER, DEFAULT_MODEL
â”œâ”€â”€ system-prompt.ts               # buildAgentSystemPrompt()
â”œâ”€â”€ system-prompt-params.ts        # System prompt parameter resolution
â”œâ”€â”€ system-prompt-report.ts        # Debug report generation
â”œâ”€â”€ tool-summaries.ts              # Tool description summaries
â”œâ”€â”€ tool-policy.ts                 # Tool policy resolution
â”œâ”€â”€ transcript-policy.ts           # Transcript validation policy
â”œâ”€â”€ skills.ts                      # Skill snapshot/prompt building
â”œâ”€â”€ skills/                        # Skill subsystem
â”œâ”€â”€ sandbox.ts                     # Sandbox context resolution
â”œâ”€â”€ sandbox/                       # Sandbox subsystem
â”œâ”€â”€ channel-tools.ts               # Channel-specific tool injection
â”œâ”€â”€ krabkrab-tools.ts              # KrabKrab-specific tools
â”œâ”€â”€ bash-tools.ts                  # exec/process tools
â”œâ”€â”€ apply-patch.ts                 # apply_patch tool (OpenAI)
â”œâ”€â”€ tools/                         # Individual tool implementations
â”‚   â”œâ”€â”€ browser-tool.ts
â”‚   â”œâ”€â”€ canvas-tool.ts
â”‚   â”œâ”€â”€ cron-tool.ts
â”‚   â”œâ”€â”€ discord-actions*.ts
â”‚   â”œâ”€â”€ gateway-tool.ts
â”‚   â”œâ”€â”€ image-tool.ts
â”‚   â”œâ”€â”€ message-tool.ts
â”‚   â”œâ”€â”€ nodes-tool.ts
â”‚   â”œâ”€â”€ session*.ts
â”‚   â”œâ”€â”€ slack-actions.ts
â”‚   â”œâ”€â”€ telegram-actions.ts
â”‚   â”œâ”€â”€ web-*.ts
â”‚   â””â”€â”€ whatsapp-actions.ts
â””â”€â”€ ...
```

## æ ¸å¿ƒé›†æˆæµç¨‹

### 1. è¿è¡ŒåµŒå…¥å¼æ™ºèƒ½ä½“

ä¸»å…¥å£ç‚¹æ˜¯ `pi-embedded-runner/run.ts` ä¸­çš„ `runEmbeddedPiAgent()`ï¼š

```typescript
import { runEmbeddedPiAgent } from "./agents/pi-embedded-runner.js";

const result = await runEmbeddedPiAgent({
  sessionId: "user-123",
  sessionKey: "main:whatsapp:+1234567890",
  sessionFile: "/path/to/session.jsonl",
  workspaceDir: "/path/to/workspace",
  config: krabkrabConfig,
  prompt: "Hello, how are you?",
  provider: "anthropic",
  model: "claude-sonnet-4-20250514",
  timeoutMs: 120_000,
  runId: "run-abc",
  onBlockReply: async (payload) => {
    await sendToChannel(payload.text, payload.mediaUrls);
  },
});
```

### 2. ä¼šè¯åˆ›å»º

åœ¨ `runEmbeddedAttempt()`ï¼ˆç”± `runEmbeddedPiAgent()` è°ƒç”¨ï¼‰å†…éƒ¨ï¼Œä½¿ç”¨ pi SDKï¼š

```typescript
import {
  createAgentSession,
  DefaultResourceLoader,
  SessionManager,
  SettingsManager,
} from "@mariozechner/pi-coding-agent";

const resourceLoader = new DefaultResourceLoader({
  cwd: resolvedWorkspace,
  agentDir,
  settingsManager,
  additionalExtensionPaths,
});
await resourceLoader.reload();

const { session } = await createAgentSession({
  cwd: resolvedWorkspace,
  agentDir,
  authStorage: params.authStorage,
  modelRegistry: params.modelRegistry,
  model: params.model,
  thinkingLevel: mapThinkingLevel(params.thinkLevel),
  tools: builtInTools,
  customTools: allCustomTools,
  sessionManager,
  settingsManager,
  resourceLoader,
});

applySystemPromptOverrideToSession(session, systemPromptOverride);
```

### 3. äº‹ä»¶è®¢é˜…

`subscribeEmbeddedPiSession()` è®¢é˜… pi çš„ `AgentSession` äº‹ä»¶ï¼š

```typescript
const subscription = subscribeEmbeddedPiSession({
  session: activeSession,
  runId: params.runId,
  verboseLevel: params.verboseLevel,
  reasoningMode: params.reasoningLevel,
  toolResultFormat: params.toolResultFormat,
  onToolResult: params.onToolResult,
  onReasoningStream: params.onReasoningStream,
  onBlockReply: params.onBlockReply,
  onPartialReply: params.onPartialReply,
  onAgentEvent: params.onAgentEvent,
});
```

å¤„ç†çš„äº‹ä»¶åŒ…æ‹¬ï¼š

- `message_start` / `message_end` / `message_update`ï¼ˆæµå¼æ–‡æœ¬/æ€è€ƒï¼‰
- `tool_execution_start` / `tool_execution_update` / `tool_execution_end`
- `turn_start` / `turn_end`
- `agent_start` / `agent_end`
- `auto_compaction_start` / `auto_compaction_end`

### 4. æç¤º

è®¾ç½®å®ŒæˆåŽï¼Œä¼šè¯è¢«æç¤ºï¼š

```typescript
await session.prompt(effectivePrompt, { images: imageResult.images });
```

SDK å¤„ç†å®Œæ•´çš„æ™ºèƒ½ä½“å¾ªçŽ¯ï¼šå‘é€åˆ° LLMã€æ‰§è¡Œå·¥å…·è°ƒç”¨ã€æµå¼å“åº”ã€‚

## å·¥å…·æž¶æž„

### å·¥å…·ç®¡é“

1. **åŸºç¡€å·¥å…·**ï¼špi çš„ `codingTools`ï¼ˆreadã€bashã€editã€writeï¼‰
2. **è‡ªå®šä¹‰æ›¿æ¢**ï¼šKrabKrab å°† bash æ›¿æ¢ä¸º `exec`/`process`ï¼Œä¸ºæ²™ç®±è‡ªå®šä¹‰ read/edit/write
3. **KrabKrab å·¥å…·**ï¼šæ¶ˆæ¯ã€æµè§ˆå™¨ã€ç”»å¸ƒã€ä¼šè¯ã€å®šæ—¶ä»»åŠ¡ã€Gateway ç½‘å…³ç­‰
4. **æ¸ é“å·¥å…·**ï¼šDiscord/Telegram/Slack/WhatsApp ç‰¹å®šçš„æ“ä½œå·¥å…·
5. **ç­–ç•¥è¿‡æ»¤**ï¼šå·¥å…·æŒ‰é…ç½®æ–‡ä»¶ã€æä¾›å•†ã€æ™ºèƒ½ä½“ã€ç¾¤ç»„ã€æ²™ç®±ç­–ç•¥è¿‡æ»¤
6. **Schema è§„èŒƒåŒ–**ï¼šä¸º Gemini/OpenAI çš„ç‰¹æ®Šæƒ…å†µæ¸…ç† Schema
7. **AbortSignal åŒ…è£…**ï¼šå·¥å…·è¢«åŒ…è£…ä»¥å°Šé‡ä¸­æ­¢ä¿¡å·

### å·¥å…·å®šä¹‰é€‚é…å™¨

pi-agent-core çš„ `AgentTool` ä¸Ž pi-coding-agent çš„ `ToolDefinition` æœ‰ä¸åŒçš„ `execute` ç­¾åã€‚`pi-tool-definition-adapter.ts` ä¸­çš„é€‚é…å™¨æ¡¥æŽ¥äº†è¿™ä¸€ç‚¹ï¼š

```typescript
export function toToolDefinitions(tools: AnyAgentTool[]): ToolDefinition[] {
  return tools.map((tool) => ({
    name: tool.name,
    label: tool.label ?? name,
    description: tool.description ?? "",
    parameters: tool.parameters,
    execute: async (toolCallId, params, onUpdate, _ctx, signal) => {
      // pi-coding-agent signature differs from pi-agent-core
      return await tool.execute(toolCallId, params, signal, onUpdate);
    },
  }));
}
```

### å·¥å…·æ‹†åˆ†ç­–ç•¥

`splitSdkTools()` é€šè¿‡ `customTools` ä¼ é€’æ‰€æœ‰å·¥å…·ï¼š

```typescript
export function splitSdkTools(options: { tools: AnyAgentTool[]; sandboxEnabled: boolean }) {
  return {
    builtInTools: [], // Empty. We override everything
    customTools: toToolDefinitions(options.tools),
  };
}
```

è¿™ç¡®ä¿ KrabKrab çš„ç­–ç•¥è¿‡æ»¤ã€æ²™ç®±é›†æˆå’Œæ‰©å±•å·¥å…·é›†åœ¨å„æä¾›å•†ä¹‹é—´ä¿æŒä¸€è‡´ã€‚

## ç³»ç»Ÿæç¤ºæž„å»º

ç³»ç»Ÿæç¤ºåœ¨ `buildAgentSystemPrompt()`ï¼ˆ`system-prompt.ts`ï¼‰ä¸­æž„å»ºã€‚å®ƒç»„è£…ä¸€ä¸ªå®Œæ•´çš„æç¤ºï¼ŒåŒ…å«å·¥å…·ã€å·¥å…·è°ƒç”¨é£Žæ ¼ã€å®‰å…¨æŠ¤æ ã€KrabKrab CLI å‚è€ƒã€Skillsã€æ–‡æ¡£ã€å·¥ä½œåŒºã€æ²™ç®±ã€æ¶ˆæ¯ã€å›žå¤æ ‡ç­¾ã€è¯­éŸ³ã€é™é»˜å›žå¤ã€å¿ƒè·³ã€è¿è¡Œæ—¶å…ƒæ•°æ®ç­‰éƒ¨åˆ†ï¼Œä»¥åŠå¯ç”¨æ—¶çš„è®°å¿†å’Œååº”ï¼Œè¿˜æœ‰å¯é€‰çš„ä¸Šä¸‹æ–‡æ–‡ä»¶å’Œé¢å¤–ç³»ç»Ÿæç¤ºå†…å®¹ã€‚éƒ¨åˆ†å†…å®¹åœ¨å­æ™ºèƒ½ä½“ä½¿ç”¨çš„æœ€å°æç¤ºæ¨¡å¼ä¸‹ä¼šè¢«è£å‰ªã€‚

æç¤ºåœ¨ä¼šè¯åˆ›å»ºåŽé€šè¿‡ `applySystemPromptOverrideToSession()` åº”ç”¨ï¼š

```typescript
const systemPromptOverride = createSystemPromptOverride(appendPrompt);
applySystemPromptOverrideToSession(session, systemPromptOverride);
```

## ä¼šè¯ç®¡ç†

### ä¼šè¯æ–‡ä»¶

ä¼šè¯æ˜¯å…·æœ‰æ ‘ç»“æž„ï¼ˆid/parentId é“¾æŽ¥ï¼‰çš„ JSONL æ–‡ä»¶ã€‚Pi çš„ `SessionManager` å¤„ç†æŒä¹…åŒ–ï¼š

```typescript
const sessionManager = SessionManager.open(params.sessionFile);
```

KrabKrab ç”¨ `guardSessionManager()` åŒ…è£…å®ƒä»¥ç¡®ä¿å·¥å…·ç»“æžœå®‰å…¨ã€‚

### ä¼šè¯ç¼“å­˜

`session-manager-cache.ts` ç¼“å­˜ SessionManager å®žä¾‹ä»¥é¿å…é‡å¤çš„æ–‡ä»¶è§£æžï¼š

```typescript
await prewarmSessionFile(params.sessionFile);
sessionManager = SessionManager.open(params.sessionFile);
trackSessionManagerAccess(params.sessionFile);
```

### åŽ†å²é™åˆ¶

`limitHistoryTurns()` æ ¹æ®æ¸ é“ç±»åž‹ï¼ˆç§ä¿¡ vs ç¾¤ç»„ï¼‰è£å‰ªå¯¹è¯åŽ†å²ã€‚

### åŽ‹ç¼©

è‡ªåŠ¨åŽ‹ç¼©åœ¨ä¸Šä¸‹æ–‡æº¢å‡ºæ—¶è§¦å‘ã€‚`compactEmbeddedPiSessionDirect()` å¤„ç†æ‰‹åŠ¨åŽ‹ç¼©ï¼š

```typescript
const compactResult = await compactEmbeddedPiSessionDirect({
  sessionId, sessionFile, provider, model, ...
});
```

## è®¤è¯ä¸Žæ¨¡åž‹è§£æž

### è®¤è¯é…ç½®æ–‡ä»¶

KrabKrab ç»´æŠ¤ä¸€ä¸ªè®¤è¯é…ç½®æ–‡ä»¶å­˜å‚¨ï¼Œæ¯ä¸ªæä¾›å•†æœ‰å¤šä¸ª API å¯†é’¥ï¼š

```typescript
const authStore = ensureAuthProfileStore(agentDir, { allowKeychainPrompt: false });
const profileOrder = resolveAuthProfileOrder({ cfg, store: authStore, provider, preferredProfile });
```

é…ç½®æ–‡ä»¶åœ¨å¤±è´¥æ—¶è½®æ¢ï¼Œå¹¶å¸¦æœ‰å†·å´è·Ÿè¸ªï¼š

```typescript
await markAuthProfileFailure({ store, profileId, reason, cfg, agentDir });
const rotated = await advanceAuthProfile();
```

### æ¨¡åž‹è§£æž

```typescript
import { resolveModel } from "./pi-embedded-runner/model.js";

const { model, error, authStorage, modelRegistry } = resolveModel(
  provider,
  modelId,
  agentDir,
  config,
);

// Uses pi's ModelRegistry and AuthStorage
authStorage.setRuntimeApiKey(model.provider, apiKeyInfo.apiKey);
```

### æ•…éšœè½¬ç§»

`FailoverError` åœ¨é…ç½®äº†å›žé€€æ—¶è§¦å‘æ¨¡åž‹å›žé€€ï¼š

```typescript
if (fallbackConfigured && isFailoverErrorMessage(errorText)) {
  throw new FailoverError(errorText, {
    reason: promptFailoverReason ?? "unknown",
    provider,
    model: modelId,
    profileId,
    status: resolveFailoverStatus(promptFailoverReason),
  });
}
```

## Pi æ‰©å±•

KrabKrab åŠ è½½è‡ªå®šä¹‰ pi æ‰©å±•ä»¥å®žçŽ°ç‰¹æ®Šè¡Œä¸ºï¼š

### åŽ‹ç¼©å®‰å…¨æŠ¤æ 

`pi-extensions/compaction-safeguard.ts` ä¸ºåŽ‹ç¼©æ·»åŠ æŠ¤æ ï¼ŒåŒ…æ‹¬è‡ªé€‚åº”ä»¤ç‰Œé¢„ç®—ä»¥åŠå·¥å…·å¤±è´¥å’Œæ–‡ä»¶æ“ä½œæ‘˜è¦ï¼š

```typescript
if (resolveCompactionMode(params.cfg) === "safeguard") {
  setCompactionSafeguardRuntime(params.sessionManager, { maxHistoryShare });
  paths.push(resolvePiExtensionPath("compaction-safeguard"));
}
```

### ä¸Šä¸‹æ–‡è£å‰ª

`pi-extensions/context-pruning.ts` å®žçŽ°åŸºäºŽç¼“å­˜ TTL çš„ä¸Šä¸‹æ–‡è£å‰ªï¼š

```typescript
if (cfg?.agents?.defaults?.contextPruning?.mode === "cache-ttl") {
  setContextPruningRuntime(params.sessionManager, {
    settings,
    contextWindowTokens,
    isToolPrunable,
    lastCacheTouchAt,
  });
  paths.push(resolvePiExtensionPath("context-pruning"));
}
```

## æµå¼ä¼ è¾“ä¸Žå—å›žå¤

### å—åˆ†å—

`EmbeddedBlockChunker` ç®¡ç†å°†æµå¼æ–‡æœ¬åˆ†æˆç¦»æ•£çš„å›žå¤å—ï¼š

```typescript
const blockChunker = blockChunking ? new EmbeddedBlockChunker(blockChunking) : null;
```

### æ€è€ƒ/æœ€ç»ˆæ ‡ç­¾å‰¥ç¦»

æµå¼è¾“å‡ºè¢«å¤„ç†ä»¥å‰¥ç¦» `<think>`/`<thinking>` å—å¹¶æå– `<final>` å†…å®¹ï¼š

```typescript
const stripBlockTags = (text: string, state: { thinking: boolean; final: boolean }) => {
  // Strip <think>...</think> content
  // If enforceFinalTag, only return <final>...</final> content
};
```

### å›žå¤æŒ‡ä»¤

å›žå¤æŒ‡ä»¤å¦‚ `[[media:url]]`ã€`[[voice]]`ã€`[[reply:id]]` è¢«è§£æžå’Œæå–ï¼š

```typescript
const { text: cleanedText, mediaUrls, audioAsVoice, replyToId } = consumeReplyDirectives(chunk);
```

## é”™è¯¯å¤„ç†

### é”™è¯¯åˆ†ç±»

`pi-embedded-helpers.ts` å¯¹é”™è¯¯è¿›è¡Œåˆ†ç±»ä»¥è¿›è¡Œé€‚å½“å¤„ç†ï¼š

```typescript
isContextOverflowError(errorText)     // Context too large
isCompactionFailureError(errorText)   // Compaction failed
isAuthAssistantError(lastAssistant)   // Auth failure
isRateLimitAssistantError(...)        // Rate limited
isFailoverAssistantError(...)         // Should failover
classifyFailoverReason(errorText)     // "auth" | "rate_limit" | "quota" | "timeout" | ...
```

### æ€è€ƒçº§åˆ«å›žé€€

å¦‚æžœæ€è€ƒçº§åˆ«ä¸å—æ”¯æŒï¼Œå®ƒä¼šå›žé€€ï¼š

```typescript
const fallbackThinking = pickFallbackThinkingLevel({
  message: errorText,
  attempted: attemptedThinking,
});
if (fallbackThinking) {
  thinkLevel = fallbackThinking;
  continue;
}
```

## æ²™ç®±é›†æˆ

å½“å¯ç”¨æ²™ç®±æ¨¡å¼æ—¶ï¼Œå·¥å…·å’Œè·¯å¾„å—åˆ°çº¦æŸï¼š

```typescript
const sandbox = await resolveSandboxContext({
  config: params.config,
  sessionKey: sandboxSessionKey,
  workspaceDir: resolvedWorkspace,
});

if (sandboxRoot) {
  // Use sandboxed read/edit/write tools
  // Exec runs in container
  // Browser uses bridge URL
}
```

## æä¾›å•†ç‰¹å®šå¤„ç†

### Anthropic

- æ‹’ç»é­”æœ¯å­—ç¬¦ä¸²æ¸…é™¤
- è¿žç»­è§’è‰²çš„å›žåˆéªŒè¯
- Claude Code å‚æ•°å…¼å®¹æ€§

### Google/Gemini

- å›žåˆæŽ’åºä¿®å¤ï¼ˆ`applyGoogleTurnOrderingFix`ï¼‰
- å·¥å…· schema æ¸…ç†ï¼ˆ`sanitizeToolsForGoogle`ï¼‰
- ä¼šè¯åŽ†å²æ¸…ç†ï¼ˆ`sanitizeSessionHistory`ï¼‰

### OpenAI

- Codex æ¨¡åž‹çš„ `apply_patch` å·¥å…·
- æ€è€ƒçº§åˆ«é™çº§å¤„ç†

## TUI é›†æˆ

KrabKrab è¿˜æœ‰ä¸€ä¸ªæœ¬åœ° TUI æ¨¡å¼ï¼Œç›´æŽ¥ä½¿ç”¨ pi-tui ç»„ä»¶ï¼š

```typescript
// src/tui/tui.ts
import { ... } from "@mariozechner/pi-tui";
```

è¿™æä¾›äº†ä¸Ž pi åŽŸç”Ÿæ¨¡å¼ç±»ä¼¼çš„äº¤äº’å¼ç»ˆç«¯ä½“éªŒã€‚

## ä¸Ž Pi CLI çš„ä¸»è¦åŒºåˆ«

| æ–¹é¢     | Pi CLI                  | KrabKrab åµŒå…¥å¼                                                                                 |
| -------- | ----------------------- | ----------------------------------------------------------------------------------------------- |
| è°ƒç”¨æ–¹å¼ | `pi` å‘½ä»¤ / RPC         | é€šè¿‡ `createAgentSession()` çš„ SDK                                                              |
| å·¥å…·     | é»˜è®¤ç¼–ç å·¥å…·            | è‡ªå®šä¹‰ KrabKrab å·¥å…·å¥—ä»¶                                                                        |
| ç³»ç»Ÿæç¤º | AGENTS.md + prompts     | æŒ‰æ¸ é“/ä¸Šä¸‹æ–‡åŠ¨æ€ç”Ÿæˆ                                                                           |
| ä¼šè¯å­˜å‚¨ | `~/.pi/agent/sessions/` | `~/.krabkrab/agents/<agentId>/sessions/`ï¼ˆæˆ– `$krabkrab_STATE_DIR/agents/<agentId>/sessions/`ï¼‰ |
| è®¤è¯     | å•ä¸€å‡­è¯                | å¸¦è½®æ¢çš„å¤šé…ç½®æ–‡ä»¶                                                                              |
| æ‰©å±•     | ä»Žç£ç›˜åŠ è½½              | ç¼–ç¨‹æ–¹å¼ + ç£ç›˜è·¯å¾„                                                                             |
| äº‹ä»¶å¤„ç† | TUI æ¸²æŸ“                | åŸºäºŽå›žè°ƒï¼ˆonBlockReply ç­‰ï¼‰                                                                     |

## æœªæ¥è€ƒè™‘

å¯èƒ½éœ€è¦é‡æž„çš„é¢†åŸŸï¼š

1. **å·¥å…·ç­¾åå¯¹é½**ï¼šç›®å‰åœ¨ pi-agent-core å’Œ pi-coding-agent ç­¾åä¹‹é—´é€‚é…
2. **ä¼šè¯ç®¡ç†å™¨åŒ…è£…**ï¼š`guardSessionManager` å¢žåŠ äº†å®‰å…¨æ€§ä½†å¢žåŠ äº†å¤æ‚æ€§
3. **æ‰©å±•åŠ è½½**ï¼šå¯ä»¥æ›´ç›´æŽ¥åœ°ä½¿ç”¨ pi çš„ `ResourceLoader`
4. **æµå¼å¤„ç†å™¨å¤æ‚æ€§**ï¼š`subscribeEmbeddedPiSession` å·²ç»å˜å¾—å¾ˆå¤§
5. **æä¾›å•†ç‰¹æ®Šæƒ…å†µ**ï¼šè®¸å¤šæä¾›å•†ç‰¹å®šçš„ä»£ç è·¯å¾„ï¼Œpi å¯èƒ½å¯ä»¥å¤„ç†

## æµ‹è¯•

æ‰€æœ‰æ¶µç›– pi é›†æˆåŠå…¶æ‰©å±•çš„çŽ°æœ‰æµ‹è¯•ï¼š

- `src/agents/pi-embedded-block-chunker.test.ts`
- `src/agents/pi-embedded-helpers.buildbootstrapcontextfiles.test.ts`
- `src/agents/pi-embedded-helpers.classifyfailoverreason.test.ts`
- `src/agents/pi-embedded-helpers.downgradeopenai-reasoning.test.ts`
- `src/agents/pi-embedded-helpers.formatassistanterrortext.test.ts`
- `src/agents/pi-embedded-helpers.formatrawassistanterrorforui.test.ts`
- `src/agents/pi-embedded-helpers.image-dimension-error.test.ts`
- `src/agents/pi-embedded-helpers.image-size-error.test.ts`
- `src/agents/pi-embedded-helpers.isautherrormessage.test.ts`
- `src/agents/pi-embedded-helpers.isbillingerrormessage.test.ts`
- `src/agents/pi-embedded-helpers.iscloudcodeassistformaterror.test.ts`
- `src/agents/pi-embedded-helpers.iscompactionfailureerror.test.ts`
- `src/agents/pi-embedded-helpers.iscontextoverflowerror.test.ts`
- `src/agents/pi-embedded-helpers.isfailovererrormessage.test.ts`
- `src/agents/pi-embedded-helpers.islikelycontextoverflowerror.test.ts`
- `src/agents/pi-embedded-helpers.ismessagingtoolduplicate.test.ts`
- `src/agents/pi-embedded-helpers.messaging-duplicate.test.ts`
- `src/agents/pi-embedded-helpers.normalizetextforcomparison.test.ts`
- `src/agents/pi-embedded-helpers.resolvebootstrapmaxchars.test.ts`
- `src/agents/pi-embedded-helpers.sanitize-session-messages-images.keeps-tool-call-tool-result-ids-unchanged.test.ts`
- `src/agents/pi-embedded-helpers.sanitize-session-messages-images.removes-empty-assistant-text-blocks-but-preserves.test.ts`
- `src/agents/pi-embedded-helpers.sanitizegoogleturnordering.test.ts`
- `src/agents/pi-embedded-helpers.sanitizesessionmessagesimages-thought-signature-stripping.test.ts`
- `src/agents/pi-embedded-helpers.sanitizetoolcallid.test.ts`
- `src/agents/pi-embedded-helpers.sanitizeuserfacingtext.test.ts`
- `src/agents/pi-embedded-helpers.stripthoughtsignatures.test.ts`
- `src/agents/pi-embedded-helpers.validate-turns.test.ts`
- `src/agents/pi-embedded-runner-extraparams.live.test.ts`ï¼ˆå®žæ—¶ï¼‰
- `src/agents/pi-embedded-runner-extraparams.test.ts`
- `src/agents/pi-embedded-runner.applygoogleturnorderingfix.test.ts`
- `src/agents/pi-embedded-runner.buildembeddedsandboxinfo.test.ts`
- `src/agents/pi-embedded-runner.createsystempromptoverride.test.ts`
- `src/agents/pi-embedded-runner.get-dm-history-limit-from-session-key.falls-back-provider-default-per-dm-not.test.ts`
- `src/agents/pi-embedded-runner.get-dm-history-limit-from-session-key.returns-undefined-sessionkey-is-undefined.test.ts`
- `src/agents/pi-embedded-runner.google-sanitize-thinking.test.ts`
- `src/agents/pi-embedded-runner.guard.test.ts`
- `src/agents/pi-embedded-runner.limithistoryturns.test.ts`
- `src/agents/pi-embedded-runner.resolvesessionagentids.test.ts`
- `src/agents/pi-embedded-runner.run-embedded-pi-agent.auth-profile-rotation.test.ts`
- `src/agents/pi-embedded-runner.sanitize-session-history.test.ts`
- `src/agents/pi-embedded-runner.splitsdktools.test.ts`
- `src/agents/pi-embedded-runner.test.ts`
- `src/agents/pi-embedded-subscribe.code-span-awareness.test.ts`
- `src/agents/pi-embedded-subscribe.reply-tags.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.calls-onblockreplyflush-before-tool-execution-start-preserve.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.does-not-append-text-end-content-is.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.does-not-call-onblockreplyflush-callback-is-not.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.does-not-duplicate-text-end-repeats-full.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.does-not-emit-duplicate-block-replies-text.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.emits-block-replies-text-end-does-not.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.emits-reasoning-as-separate-message-enabled.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.filters-final-suppresses-output-without-start-tag.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.includes-canvas-action-metadata-tool-summaries.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.keeps-assistanttexts-final-answer-block-replies-are.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.keeps-indented-fenced-blocks-intact.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.reopens-fenced-blocks-splitting-inside-them.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.splits-long-single-line-fenced-blocks-reopen.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.streams-soft-chunks-paragraph-preference.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.subscribeembeddedpisession.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.suppresses-message-end-block-replies-message-tool.test.ts`
- `src/agents/pi-embedded-subscribe.subscribe-embedded-pi-session.waits-multiple-compaction-retries-before-resolving.test.ts`
- `src/agents/pi-embedded-subscribe.tools.test.ts`
- `src/agents/pi-embedded-utils.test.ts`
- `src/agents/pi-extensions/compaction-safeguard.test.ts`
- `src/agents/pi-extensions/context-pruning.test.ts`
- `src/agents/pi-settings.test.ts`
- `src/agents/pi-tool-definition-adapter.test.ts`
- `src/agents/pi-tools-agent-config.test.ts`
- `src/agents/pi-tools.create-krabkrab-coding-tools.adds-claude-style-aliases-schemas-without-dropping-b.test.ts`
- `src/agents/pi-tools.create-krabkrab-coding-tools.adds-claude-style-aliases-schemas-without-dropping-d.test.ts`
- `src/agents/pi-tools.create-krabkrab-coding-tools.adds-claude-style-aliases-schemas-without-dropping-f.test.ts`
- `src/agents/pi-tools.create-krabkrab-coding-tools.adds-claude-style-aliases-schemas-without-dropping.test.ts`
- `src/agents/pi-tools.policy.test.ts`
- `src/agents/pi-tools.safe-bins.test.ts`
- `src/agents/pi-tools.workspace-paths.test.ts`

