---
read_when:
  - å°† Gmail æ”¶ä»¶ç®±è§¦å‘å™¨æŽ¥å…¥ KrabKrab
  - ä¸ºæ™ºèƒ½ä½“å”¤é†’è®¾ç½® Pub/Sub æŽ¨é€
summary: é€šè¿‡ gogcli å°† Gmail Pub/Sub æŽ¨é€æŽ¥å…¥ KrabKrab webhooks
title: Gmail PubSub
x-i18n:
  generated_at: "2026-02-03T07:43:25Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: dfb92133b69177e4e984b7d072f5dc28aa53a9e0cf984a018145ed811aa96195
  source_path: automation/gmail-pubsub.md
  workflow: 15
---

# Gmail Pub/Sub -> KrabKrab

ç›®æ ‡ï¼šGmail watch -> Pub/Sub æŽ¨é€ -> `gog gmail watch serve` -> KrabKrab webhookã€‚

## å‰ç½®æ¡ä»¶

- å·²å®‰è£…å¹¶ç™»å½• `gcloud`ï¼ˆ[å®‰è£…æŒ‡å—](https://docs.cloud.google.com/sdk/docs/install-sdk)ï¼‰ã€‚
- å·²å®‰è£… `gog` (gogcli) å¹¶ä¸º Gmail è´¦æˆ·æŽˆæƒï¼ˆ[gogcli.sh](https://gogcli.sh/)ï¼‰ã€‚
- å·²å¯ç”¨ KrabKrab hooksï¼ˆå‚è§ [Webhooks](/automation/webhook)ï¼‰ã€‚
- å·²ç™»å½• `tailscale`ï¼ˆ[tailscale.com](https://tailscale.com/)ï¼‰ã€‚æ”¯æŒçš„è®¾ç½®ä½¿ç”¨ Tailscale Funnel ä½œä¸ºå…¬å…± HTTPS ç«¯ç‚¹ã€‚
  å…¶ä»–éš§é“æœåŠ¡ä¹Ÿå¯ä»¥ä½¿ç”¨ï¼Œä½†éœ€è¦è‡ªè¡Œé…ç½®/ä¸å—æ”¯æŒï¼Œéœ€è¦æ‰‹åŠ¨æŽ¥å…¥ã€‚
  ç›®å‰ï¼Œæˆ‘ä»¬æ”¯æŒçš„æ˜¯ Tailscaleã€‚

ç¤ºä¾‹ hook é…ç½®ï¼ˆå¯ç”¨ Gmail é¢„è®¾æ˜ å°„ï¼‰ï¼š

```json5
{
  hooks: {
    enabled: true,
    token: "krabkrab_HOOK_TOKEN",
    path: "/hooks",
    presets: ["gmail"],
  },
}
```

è¦å°† Gmail æ‘˜è¦æŠ•é€’åˆ°èŠå¤©ç•Œé¢ï¼Œè¯·ç”¨è®¾ç½®äº† `deliver` ä»¥åŠå¯é€‰çš„ `channel`/`to` çš„æ˜ å°„è¦†ç›–é¢„è®¾ï¼š

```json5
{
  hooks: {
    enabled: true,
    token: "krabkrab_HOOK_TOKEN",
    presets: ["gmail"],
    mappings: [
      {
        match: { path: "gmail" },
        action: "agent",
        wakeMode: "now",
        name: "Gmail",
        sessionKey: "hook:gmail:{{messages[0].id}}",
        messageTemplate: "New email from {{messages[0].from}}\nSubject: {{messages[0].subject}}\n{{messages[0].snippet}}\n{{messages[0].body}}",
        model: "openai/gpt-5.2-mini",
        deliver: true,
        channel: "last",
        // to: "+15551234567"
      },
    ],
  },
}
```

å¦‚æžœä½ æƒ³ä½¿ç”¨å›ºå®šæ¸ é“ï¼Œè¯·è®¾ç½® `channel` + `to`ã€‚å¦åˆ™ `channel: "last"` ä¼šä½¿ç”¨ä¸Šæ¬¡çš„æŠ•é€’è·¯ç”±ï¼ˆé»˜è®¤å›žé€€åˆ° WhatsAppï¼‰ã€‚

è¦ä¸º Gmail è¿è¡Œå¼ºåˆ¶ä½¿ç”¨æ›´ä¾¿å®œçš„æ¨¡åž‹ï¼Œè¯·åœ¨æ˜ å°„ä¸­è®¾ç½® `model`ï¼ˆ`provider/model` æˆ–åˆ«åï¼‰ã€‚å¦‚æžœä½ å¼ºåˆ¶å¯ç”¨äº† `agents.defaults.models`ï¼Œè¯·å°†å…¶åŒ…å«åœ¨å†…ã€‚

è¦ä¸“é—¨ä¸º Gmail hooks è®¾ç½®é»˜è®¤æ¨¡åž‹å’Œæ€è€ƒçº§åˆ«ï¼Œè¯·åœ¨é…ç½®ä¸­æ·»åŠ  `hooks.gmail.model` / `hooks.gmail.thinking`ï¼š

```json5
{
  hooks: {
    gmail: {
      model: "openrouter/meta-llama/llama-3.3-70b-instruct:free",
      thinking: "off",
    },
  },
}
```

æ³¨æ„äº‹é¡¹ï¼š

- æ˜ å°„ä¸­çš„æ¯ä¸ª hook çš„ `model`/`thinking` ä»ä¼šè¦†ç›–è¿™äº›é»˜è®¤å€¼ã€‚
- å›žé€€é¡ºåºï¼š`hooks.gmail.model` â†’ `agents.defaults.model.fallbacks` â†’ ä¸»æ¨¡åž‹ï¼ˆè®¤è¯/é€ŸçŽ‡é™åˆ¶/è¶…æ—¶ï¼‰ã€‚
- å¦‚æžœè®¾ç½®äº† `agents.defaults.models`ï¼ŒGmail æ¨¡åž‹å¿…é¡»åœ¨å…è®¸åˆ—è¡¨ä¸­ã€‚
- Gmail hook å†…å®¹é»˜è®¤ä½¿ç”¨å¤–éƒ¨å†…å®¹å®‰å…¨è¾¹ç•ŒåŒ…è£…ã€‚
  è¦ç¦ç”¨ï¼ˆå±é™©ï¼‰ï¼Œè¯·è®¾ç½® `hooks.gmail.allowUnsafeExternalContent: true`ã€‚

è¦è¿›ä¸€æ­¥è‡ªå®šä¹‰è´Ÿè½½å¤„ç†ï¼Œè¯·æ·»åŠ  `hooks.mappings` æˆ–åœ¨ `hooks.transformsDir` ä¸‹æ·»åŠ  JS/TS è½¬æ¢æ¨¡å—ï¼ˆå‚è§ [Webhooks](/automation/webhook)ï¼‰ã€‚

## å‘å¯¼ï¼ˆæŽ¨èï¼‰

ä½¿ç”¨ KrabKrab åŠ©æ‰‹å°†æ‰€æœ‰å†…å®¹æŽ¥å…¥åœ¨ä¸€èµ·ï¼ˆåœ¨ macOS ä¸Šé€šè¿‡ brew å®‰è£…ä¾èµ–ï¼‰ï¼š

```bash
krabkrab webhooks gmail setup \
  --account krabkrab@gmail.com
```

é»˜è®¤è®¾ç½®ï¼š

- ä½¿ç”¨ Tailscale Funnel ä½œä¸ºå…¬å…±æŽ¨é€ç«¯ç‚¹ã€‚
- ä¸º `krabkrab webhooks gmail run` å†™å…¥ `hooks.gmail` é…ç½®ã€‚
- å¯ç”¨ Gmail hook é¢„è®¾ï¼ˆ`hooks.presets: ["gmail"]`ï¼‰ã€‚

è·¯å¾„è¯´æ˜Žï¼šå½“å¯ç”¨ `tailscale.mode` æ—¶ï¼ŒKrabKrab ä¼šè‡ªåŠ¨å°† `hooks.gmail.serve.path` è®¾ç½®ä¸º `/`ï¼Œå¹¶å°†å…¬å…±è·¯å¾„ä¿æŒåœ¨ `hooks.gmail.tailscale.path`ï¼ˆé»˜è®¤ `/gmail-pubsub`ï¼‰ï¼Œå› ä¸º Tailscale åœ¨ä»£ç†ä¹‹å‰ä¼šå‰¥ç¦»è®¾ç½®çš„è·¯å¾„å‰ç¼€ã€‚
å¦‚æžœä½ éœ€è¦åŽç«¯æŽ¥æ”¶å¸¦å‰ç¼€çš„è·¯å¾„ï¼Œè¯·å°† `hooks.gmail.tailscale.target`ï¼ˆæˆ– `--tailscale-target`ï¼‰è®¾ç½®ä¸ºå®Œæ•´ URLï¼Œå¦‚ `http://127.0.0.1:8788/gmail-pubsub`ï¼Œå¹¶åŒ¹é… `hooks.gmail.serve.path`ã€‚

æƒ³è¦è‡ªå®šä¹‰ç«¯ç‚¹ï¼Ÿä½¿ç”¨ `--push-endpoint <url>` æˆ– `--tailscale off`ã€‚

å¹³å°è¯´æ˜Žï¼šåœ¨ macOS ä¸Šï¼Œå‘å¯¼é€šè¿‡ Homebrew å®‰è£… `gcloud`ã€`gogcli` å’Œ `tailscale`ï¼›åœ¨ Linux ä¸Šè¯·å…ˆæ‰‹åŠ¨å®‰è£…å®ƒä»¬ã€‚

Gateway ç½‘å…³è‡ªåŠ¨å¯åŠ¨ï¼ˆæŽ¨èï¼‰ï¼š

- å½“ `hooks.enabled=true` ä¸”è®¾ç½®äº† `hooks.gmail.account` æ—¶ï¼ŒGateway ç½‘å…³ä¼šåœ¨å¯åŠ¨æ—¶è¿è¡Œ `gog gmail watch serve` å¹¶è‡ªåŠ¨ç»­æœŸ watchã€‚
- è®¾ç½® `krabkrab_SKIP_GMAIL_WATCHER=1` å¯é€€å‡ºï¼ˆå¦‚æžœä½ è‡ªå·±è¿è¡Œå®ˆæŠ¤è¿›ç¨‹åˆ™å¾ˆæœ‰ç”¨ï¼‰ã€‚
- ä¸è¦åŒæ—¶è¿è¡Œæ‰‹åŠ¨å®ˆæŠ¤è¿›ç¨‹ï¼Œå¦åˆ™ä¼šé‡åˆ° `listen tcp 127.0.0.1:8788: bind: address already in use`ã€‚

æ‰‹åŠ¨å®ˆæŠ¤è¿›ç¨‹ï¼ˆå¯åŠ¨ `gog gmail watch serve` + è‡ªåŠ¨ç»­æœŸï¼‰ï¼š

```bash
krabkrab webhooks gmail run
```

## ä¸€æ¬¡æ€§è®¾ç½®

1. é€‰æ‹©**æ‹¥æœ‰ `gog` ä½¿ç”¨çš„ OAuth å®¢æˆ·ç«¯**çš„ GCP é¡¹ç›®ã€‚

```bash
gcloud auth login
gcloud config set project <project-id>
```

æ³¨æ„ï¼šGmail watch è¦æ±‚ Pub/Sub ä¸»é¢˜ä¸Ž OAuth å®¢æˆ·ç«¯ä½äºŽåŒä¸€é¡¹ç›®ä¸­ã€‚

2. å¯ç”¨ APIï¼š

```bash
gcloud services enable gmail.googleapis.com pubsub.googleapis.com
```

3. åˆ›å»ºä¸»é¢˜ï¼š

```bash
gcloud pubsub topics create gog-gmail-watch
```

4. å…è®¸ Gmail push å‘å¸ƒï¼š

```bash
gcloud pubsub topics add-iam-policy-binding gog-gmail-watch \
  --member=serviceAccount:gmail-api-push@system.gserviceaccount.com \
  --role=roles/pubsub.publisher
```

## å¯åŠ¨ watch

```bash
gog gmail watch start \
  --account krabkrab@gmail.com \
  --label INBOX \
  --topic projects/<project-id>/topics/gog-gmail-watch
```

ä¿å­˜è¾“å‡ºä¸­çš„ `history_id`ï¼ˆç”¨äºŽè°ƒè¯•ï¼‰ã€‚

## è¿è¡ŒæŽ¨é€å¤„ç†ç¨‹åº

æœ¬åœ°ç¤ºä¾‹ï¼ˆå…±äº« token è®¤è¯ï¼‰ï¼š

```bash
gog gmail watch serve \
  --account krabkrab@gmail.com \
  --bind 127.0.0.1 \
  --port 8788 \
  --path /gmail-pubsub \
  --token <shared> \
  --hook-url http://127.0.0.1:18789/hooks/gmail \
  --hook-token krabkrab_HOOK_TOKEN \
  --include-body \
  --max-bytes 20000
```

æ³¨æ„äº‹é¡¹ï¼š

- `--token` ä¿æŠ¤æŽ¨é€ç«¯ç‚¹ï¼ˆ`x-gog-token` æˆ– `?token=`ï¼‰ã€‚
- `--hook-url` æŒ‡å‘ KrabKrab `/hooks/gmail`ï¼ˆå·²æ˜ å°„ï¼›éš”ç¦»è¿è¡Œ + æ‘˜è¦å‘é€åˆ°ä¸»çº¿ç¨‹ï¼‰ã€‚
- `--include-body` å’Œ `--max-bytes` æŽ§åˆ¶å‘é€åˆ° KrabKrab çš„æ­£æ–‡ç‰‡æ®µã€‚

æŽ¨èï¼š`krabkrab webhooks gmail run` å°è£…äº†ç›¸åŒçš„æµç¨‹å¹¶è‡ªåŠ¨ç»­æœŸ watchã€‚

## æš´éœ²å¤„ç†ç¨‹åºï¼ˆé«˜çº§ï¼Œä¸å—æ”¯æŒï¼‰

å¦‚æžœä½ éœ€è¦éž Tailscale éš§é“ï¼Œè¯·æ‰‹åŠ¨æŽ¥å…¥å¹¶åœ¨æŽ¨é€è®¢é˜…ä¸­ä½¿ç”¨å…¬å…± URLï¼ˆä¸å—æ”¯æŒï¼Œæ— ä¿æŠ¤æŽªæ–½ï¼‰ï¼š

```bash
cloudflared tunnel --url http://127.0.0.1:8788 --no-autoupdate
```

ä½¿ç”¨ç”Ÿæˆçš„ URL ä½œä¸ºæŽ¨é€ç«¯ç‚¹ï¼š

```bash
gcloud pubsub subscriptions create gog-gmail-watch-push \
  --topic gog-gmail-watch \
  --push-endpoint "https://<public-url>/gmail-pubsub?token=<shared>"
```

ç”Ÿäº§çŽ¯å¢ƒï¼šä½¿ç”¨ç¨³å®šçš„ HTTPS ç«¯ç‚¹å¹¶é…ç½® Pub/Sub OIDC JWTï¼Œç„¶åŽè¿è¡Œï¼š

```bash
gog gmail watch serve --verify-oidc --oidc-email <svc@...>
```

## æµ‹è¯•

å‘è¢«ç›‘è§†çš„æ”¶ä»¶ç®±å‘é€ä¸€æ¡æ¶ˆæ¯ï¼š

```bash
gog gmail send \
  --account krabkrab@gmail.com \
  --to krabkrab@gmail.com \
  --subject "watch test" \
  --body "ping"
```

æ£€æŸ¥ watch çŠ¶æ€å’ŒåŽ†å²è®°å½•ï¼š

```bash
gog gmail watch status --account krabkrab@gmail.com
gog gmail history --account krabkrab@gmail.com --since <historyId>
```

## æ•…éšœæŽ’é™¤

- `Invalid topicName`ï¼šé¡¹ç›®ä¸åŒ¹é…ï¼ˆä¸»é¢˜ä¸åœ¨ OAuth å®¢æˆ·ç«¯é¡¹ç›®ä¸­ï¼‰ã€‚
- `User not authorized`ï¼šä¸»é¢˜ç¼ºå°‘ `roles/pubsub.publisher`ã€‚
- ç©ºæ¶ˆæ¯ï¼šGmail push ä»…æä¾› `historyId`ï¼›é€šè¿‡ `gog gmail history` èŽ·å–ã€‚

## æ¸…ç†

```bash
gog gmail watch stop --account krabkrab@gmail.com
gcloud pubsub subscriptions delete gog-gmail-watch-push
gcloud pubsub topics delete gog-gmail-watch
```

