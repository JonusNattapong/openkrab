---
read_when:
  - å®šä¹‰æˆ–é‡æž„æ’ä»¶æž¶æž„
  - å°†æ¸ é“è¿žæŽ¥å™¨è¿ç§»åˆ°æ’ä»¶ SDK/è¿è¡Œæ—¶
summary: è®¡åˆ’ï¼šä¸ºæ‰€æœ‰æ¶ˆæ¯è¿žæŽ¥å™¨æä¾›ä¸€å¥—ç»Ÿä¸€çš„æ’ä»¶ SDK + è¿è¡Œæ—¶
title: æ’ä»¶ SDK é‡æž„
x-i18n:
  generated_at: "2026-02-01T21:36:45Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: d1964e2e47a19ee1d42ddaaa9cf1293c80bb0be463b049dc8468962f35bb6cb0
  source_path: refactor/plugin-sdk.md
  workflow: 15
---

# æ’ä»¶ SDK + è¿è¡Œæ—¶é‡æž„è®¡åˆ’

ç›®æ ‡ï¼šæ¯ä¸ªæ¶ˆæ¯è¿žæŽ¥å™¨éƒ½æ˜¯ä¸€ä¸ªæ’ä»¶ï¼ˆå†…ç½®æˆ–å¤–éƒ¨ï¼‰ï¼Œä½¿ç”¨ç»Ÿä¸€ç¨³å®šçš„ APIã€‚
æ’ä»¶ä¸ç›´æŽ¥ä»Ž `src/**` å¯¼å…¥ä»»ä½•å†…å®¹ã€‚æ‰€æœ‰ä¾èµ–é¡¹å‡é€šè¿‡ SDK æˆ–è¿è¡Œæ—¶èŽ·å–ã€‚

## ä¸ºä»€ä¹ˆçŽ°åœ¨åš

- å½“å‰è¿žæŽ¥å™¨æ··ç”¨å¤šç§æ¨¡å¼ï¼šç›´æŽ¥å¯¼å…¥æ ¸å¿ƒæ¨¡å—ã€ä»… dist çš„æ¡¥æŽ¥æ–¹å¼ä»¥åŠè‡ªå®šä¹‰è¾…åŠ©å‡½æ•°ã€‚
- è¿™ä½¿å¾—å‡çº§å˜å¾—è„†å¼±ï¼Œå¹¶é˜»ç¢äº†å¹²å‡€çš„å¤–éƒ¨æ’ä»¶æŽ¥å£ã€‚

## ç›®æ ‡æž¶æž„ï¼ˆä¸¤å±‚ï¼‰

### 1ï¼‰æ’ä»¶ SDKï¼ˆç¼–è¯‘æ—¶ï¼Œç¨³å®šï¼Œå¯å‘å¸ƒï¼‰

èŒƒå›´ï¼šç±»åž‹ã€è¾…åŠ©å‡½æ•°å’Œé…ç½®å·¥å…·ã€‚æ— è¿è¡Œæ—¶çŠ¶æ€ï¼Œæ— å‰¯ä½œç”¨ã€‚

å†…å®¹ï¼ˆç¤ºä¾‹ï¼‰ï¼š

- ç±»åž‹ï¼š`ChannelPlugin`ã€é€‚é…å™¨ã€`ChannelMeta`ã€`ChannelCapabilities`ã€`ChannelDirectoryEntry`ã€‚
- é…ç½®è¾…åŠ©å‡½æ•°ï¼š`buildChannelConfigSchema`ã€`setAccountEnabledInConfigSection`ã€`deleteAccountFromConfigSection`ã€
  `applyAccountNameToChannelSection`ã€‚
- é…å¯¹è¾…åŠ©å‡½æ•°ï¼š`PAIRING_APPROVED_MESSAGE`ã€`formatPairingApproveHint`ã€‚
- æ–°æ‰‹å¼•å¯¼è¾…åŠ©å‡½æ•°ï¼š`promptChannelAccessConfig`ã€`addWildcardAllowFrom`ã€æ–°æ‰‹å¼•å¯¼ç±»åž‹ã€‚
- å·¥å…·å‚æ•°è¾…åŠ©å‡½æ•°ï¼š`createActionGate`ã€`readStringParam`ã€`readNumberParam`ã€`readReactionParams`ã€`jsonResult`ã€‚
- æ–‡æ¡£é“¾æŽ¥è¾…åŠ©å‡½æ•°ï¼š`formatDocsLink`ã€‚

äº¤ä»˜æ–¹å¼ï¼š

- ä»¥ `krabkrab/plugin-sdk` å‘å¸ƒï¼ˆæˆ–ä»Žæ ¸å¿ƒä»¥ `krabkrab/plugin-sdk` å¯¼å‡ºï¼‰ã€‚
- ä½¿ç”¨è¯­ä¹‰åŒ–ç‰ˆæœ¬æŽ§åˆ¶ï¼Œæä¾›æ˜Žç¡®çš„ç¨³å®šæ€§ä¿è¯ã€‚

### 2ï¼‰æ’ä»¶è¿è¡Œæ—¶ï¼ˆæ‰§è¡Œå±‚ï¼Œæ³¨å…¥å¼ï¼‰

èŒƒå›´ï¼šæ‰€æœ‰æ¶‰åŠæ ¸å¿ƒè¿è¡Œæ—¶è¡Œä¸ºçš„å†…å®¹ã€‚
é€šè¿‡ `krabkrabPluginApi.runtime` è®¿é—®ï¼Œç¡®ä¿æ’ä»¶æ°¸è¿œä¸ä¼šå¯¼å…¥ `src/**`ã€‚

å»ºè®®çš„æŽ¥å£ï¼ˆæœ€å°ä½†å®Œæ•´ï¼‰ï¼š

```ts
export type PluginRuntime = {
  channel: {
    text: {
      chunkMarkdownText(text: string, limit: number): string[];
      resolveTextChunkLimit(cfg: krabkrabConfig, channel: string, accountId?: string): number;
      hasControlCommand(text: string, cfg: krabkrabConfig): boolean;
    };
    reply: {
      dispatchReplyWithBufferedBlockDispatcher(params: {
        ctx: unknown;
        cfg: unknown;
        dispatcherOptions: {
          deliver: (payload: {
            text?: string;
            mediaUrls?: string[];
            mediaUrl?: string;
          }) => void | Promise<void>;
          onError?: (err: unknown, info: { kind: string }) => void;
        };
      }): Promise<void>;
      createReplyDispatcherWithTyping?: unknown; // adapter for Teams-style flows
    };
    routing: {
      resolveAgentRoute(params: {
        cfg: unknown;
        channel: string;
        accountId: string;
        peer: { kind: RoutePeerKind; id: string };
      }): { sessionKey: string; accountId: string };
    };
    pairing: {
      buildPairingReply(params: { channel: string; idLine: string; code: string }): string;
      readAllowFromStore(channel: string): Promise<string[]>;
      upsertPairingRequest(params: {
        channel: string;
        id: string;
        meta?: { name?: string };
      }): Promise<{ code: string; created: boolean }>;
    };
    media: {
      fetchRemoteMedia(params: { url: string }): Promise<{ buffer: Buffer; contentType?: string }>;
      saveMediaBuffer(
        buffer: Uint8Array,
        contentType: string | undefined,
        direction: "inbound" | "outbound",
        maxBytes: number,
      ): Promise<{ path: string; contentType?: string }>;
    };
    mentions: {
      buildMentionRegexes(cfg: krabkrabConfig, agentId?: string): RegExp[];
      matchesMentionPatterns(text: string, regexes: RegExp[]): boolean;
    };
    groups: {
      resolveGroupPolicy(
        cfg: krabkrabConfig,
        channel: string,
        accountId: string,
        groupId: string,
      ): {
        allowlistEnabled: boolean;
        allowed: boolean;
        groupConfig?: unknown;
        defaultConfig?: unknown;
      };
      resolveRequireMention(
        cfg: krabkrabConfig,
        channel: string,
        accountId: string,
        groupId: string,
        override?: boolean,
      ): boolean;
    };
    debounce: {
      createInboundDebouncer<T>(opts: {
        debounceMs: number;
        buildKey: (v: T) => string | null;
        shouldDebounce: (v: T) => boolean;
        onFlush: (entries: T[]) => Promise<void>;
        onError?: (err: unknown) => void;
      }): { push: (v: T) => void; flush: () => Promise<void> };
      resolveInboundDebounceMs(cfg: krabkrabConfig, channel: string): number;
    };
    commands: {
      resolveCommandAuthorizedFromAuthorizers(params: {
        useAccessGroups: boolean;
        authorizers: Array<{ configured: boolean; allowed: boolean }>;
      }): boolean;
    };
  };
  logging: {
    shouldLogVerbose(): boolean;
    getChildLogger(name: string): PluginLogger;
  };
  state: {
    resolveStateDir(cfg: krabkrabConfig): string;
  };
};
```

å¤‡æ³¨ï¼š

- è¿è¡Œæ—¶æ˜¯è®¿é—®æ ¸å¿ƒè¡Œä¸ºçš„å”¯ä¸€æ–¹å¼ã€‚
- SDK æ•…æ„ä¿æŒå°å·§å’Œç¨³å®šã€‚
- æ¯ä¸ªè¿è¡Œæ—¶æ–¹æ³•éƒ½æ˜ å°„åˆ°çŽ°æœ‰çš„æ ¸å¿ƒå®žçŽ°ï¼ˆæ— é‡å¤ä»£ç ï¼‰ã€‚

## è¿ç§»è®¡åˆ’ï¼ˆåˆ†é˜¶æ®µï¼Œå®‰å…¨ï¼‰

### é˜¶æ®µ 0ï¼šåŸºç¡€æ­å»º

- å¼•å…¥ `krabkrab/plugin-sdk`ã€‚
- åœ¨ `krabkrabPluginApi` ä¸­æ·»åŠ å¸¦æœ‰ä¸Šè¿°æŽ¥å£çš„ `api.runtime`ã€‚
- åœ¨è¿‡æ¸¡æœŸå†…ä¿ç•™çŽ°æœ‰å¯¼å…¥æ–¹å¼ï¼ˆæ·»åŠ å¼ƒç”¨è­¦å‘Šï¼‰ã€‚

### é˜¶æ®µ 1ï¼šæ¡¥æŽ¥æ¸…ç†ï¼ˆä½Žé£Žé™©ï¼‰

- ç”¨ `api.runtime` æ›¿æ¢æ¯ä¸ªæ‰©å±•ä¸­çš„ `core-bridge.ts`ã€‚
- ä¼˜å…ˆè¿ç§» BlueBubblesã€Zaloã€Zalo Personalï¼ˆå·²ç»æŽ¥è¿‘å®Œæˆï¼‰ã€‚
- ç§»é™¤é‡å¤çš„æ¡¥æŽ¥ä»£ç ã€‚

### é˜¶æ®µ 2ï¼šè½»åº¦ç›´æŽ¥å¯¼å…¥çš„æ’ä»¶

- å°† Matrix è¿ç§»åˆ° SDK + è¿è¡Œæ—¶ã€‚
- éªŒè¯æ–°æ‰‹å¼•å¯¼ã€ç›®å½•ã€ç¾¤ç»„æåŠé€»è¾‘ã€‚

### é˜¶æ®µ 3ï¼šé‡åº¦ç›´æŽ¥å¯¼å…¥çš„æ’ä»¶

- è¿ç§» Microsoft Teamsï¼ˆä½¿ç”¨è¿è¡Œæ—¶è¾…åŠ©å‡½æ•°æœ€å¤šçš„æ’ä»¶ï¼‰ã€‚
- ç¡®ä¿å›žå¤/æ­£åœ¨è¾“å…¥çš„è¯­ä¹‰ä¸Žå½“å‰è¡Œä¸ºä¸€è‡´ã€‚

### é˜¶æ®µ 4ï¼šiMessage æ’ä»¶åŒ–

- å°† iMessage ç§»å…¥ `extensions/imessage`ã€‚
- ç”¨ `api.runtime` æ›¿æ¢ç›´æŽ¥çš„æ ¸å¿ƒè°ƒç”¨ã€‚
- ä¿æŒé…ç½®é”®ã€CLI è¡Œä¸ºå’Œæ–‡æ¡£ä¸å˜ã€‚

### é˜¶æ®µ 5ï¼šå¼ºåˆ¶æ‰§è¡Œ

- æ·»åŠ  lint è§„åˆ™ / CI æ£€æŸ¥ï¼šç¦æ­¢ `extensions/**` ä»Ž `src/**` å¯¼å…¥ã€‚
- æ·»åŠ æ’ä»¶ SDK/ç‰ˆæœ¬å…¼å®¹æ€§æ£€æŸ¥ï¼ˆè¿è¡Œæ—¶ + SDK è¯­ä¹‰åŒ–ç‰ˆæœ¬ï¼‰ã€‚

## å…¼å®¹æ€§ä¸Žç‰ˆæœ¬æŽ§åˆ¶

- SDKï¼šè¯­ä¹‰åŒ–ç‰ˆæœ¬æŽ§åˆ¶ï¼Œå·²å‘å¸ƒï¼Œå˜æ›´æœ‰æ–‡æ¡£è®°å½•ã€‚
- è¿è¡Œæ—¶ï¼šæŒ‰æ ¸å¿ƒç‰ˆæœ¬è¿›è¡Œç‰ˆæœ¬æŽ§åˆ¶ã€‚æ·»åŠ  `api.runtime.version`ã€‚
- æ’ä»¶å£°æ˜Žæ‰€éœ€çš„è¿è¡Œæ—¶ç‰ˆæœ¬èŒƒå›´ï¼ˆä¾‹å¦‚ `krabkrabRuntime: ">=2026.2.0"`ï¼‰ã€‚

## æµ‹è¯•ç­–ç•¥

- é€‚é…å™¨çº§å•å…ƒæµ‹è¯•ï¼ˆä½¿ç”¨çœŸå®žæ ¸å¿ƒå®žçŽ°éªŒè¯è¿è¡Œæ—¶å‡½æ•°ï¼‰ã€‚
- æ¯ä¸ªæ’ä»¶çš„é»„é‡‘æµ‹è¯•ï¼šç¡®ä¿è¡Œä¸ºæ— åå·®ï¼ˆè·¯ç”±ã€é…å¯¹ã€å…è®¸åˆ—è¡¨ã€æåŠè¿‡æ»¤ï¼‰ã€‚
- CI ä¸­ä½¿ç”¨å•ä¸ªç«¯åˆ°ç«¯æ’ä»¶ç¤ºä¾‹ï¼ˆå®‰è£… + è¿è¡Œ + å†’çƒŸæµ‹è¯•ï¼‰ã€‚

## å¾…è§£å†³é—®é¢˜

- SDK ç±»åž‹æ‰˜ç®¡åœ¨å“ªé‡Œï¼šç‹¬ç«‹åŒ…è¿˜æ˜¯æ ¸å¿ƒå¯¼å‡ºï¼Ÿ
- è¿è¡Œæ—¶ç±»åž‹åˆ†å‘ï¼šåœ¨ SDK ä¸­ï¼ˆä»…ç±»åž‹ï¼‰è¿˜æ˜¯åœ¨æ ¸å¿ƒä¸­ï¼Ÿ
- å¦‚ä½•ä¸ºå†…ç½®æ’ä»¶ä¸Žå¤–éƒ¨æ’ä»¶æš´éœ²æ–‡æ¡£é“¾æŽ¥ï¼Ÿ
- è¿‡æ¸¡æœŸé—´æ˜¯å¦å…è®¸ä»“åº“å†…æ’ä»¶æœ‰é™åœ°ç›´æŽ¥å¯¼å…¥æ ¸å¿ƒæ¨¡å—ï¼Ÿ

## æˆåŠŸæ ‡å‡†

- æ‰€æœ‰æ¸ é“è¿žæŽ¥å™¨éƒ½æ˜¯ä½¿ç”¨ SDK + è¿è¡Œæ—¶çš„æ’ä»¶ã€‚
- `extensions/**` ä¸å†ä»Ž `src/**` å¯¼å…¥ã€‚
- æ–°è¿žæŽ¥å™¨æ¨¡æ¿ä»…ä¾èµ– SDK + è¿è¡Œæ—¶ã€‚
- å¤–éƒ¨æ’ä»¶å¯ä»¥åœ¨æ— éœ€è®¿é—®æ ¸å¿ƒæºç çš„æƒ…å†µä¸‹è¿›è¡Œå¼€å‘å’Œæ›´æ–°ã€‚

ç›¸å…³æ–‡æ¡£ï¼š[æ’ä»¶](/tools/plugin)ã€[æ¸ é“](/channels/index)ã€[é…ç½®](/gateway/configuration)ã€‚

