---
read_when:
  - æ–°è¦ãƒ¦ãƒ¼ã‚¶ãƒ¼ã«krabkrabã‚’ç´¹ä»‹ã™ã‚‹ã¨ã
summary: krabkrabã¯ã€ã‚ã‚‰ã‚†ã‚‹OSã§å‹•ä½œã™ã‚‹AIã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆå‘ã‘ã®ãƒžãƒ«ãƒãƒãƒ£ãƒãƒ«gatewayã§ã™ã€‚
title: KrabKrab
x-i18n:
  generated_at: "2026-02-08T17:15:47Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: fc8babf7885ef91d526795051376d928599c4cf8aff75400138a0d7d9fa3b75f
  source_path: index.md
  workflow: 15
---

# KrabKrab ðŸ¦ž

<p align="center">
    <img
        src="/assets/krabkrab-logo-text-dark.png"
        alt="KrabKrab"
        width="500"
        class="dark:hidden"
    />
    <img
        src="/assets/krabkrab-logo-text.png"
        alt="KrabKrab"
        width="500"
        class="hidden dark:block"
    />
</p>

> _ã€ŒEXFOLIATE! EXFOLIATE!ã€_ â€” ãŸã¶ã‚“å®‡å®™ãƒ­ãƒ–ã‚¹ã‚¿ãƒ¼

<p align="center">
  <strong>WhatsAppã€Telegramã€Discordã€iMessageãªã©ã«å¯¾å¿œã—ãŸã€ã‚ã‚‰ã‚†ã‚‹OSå‘ã‘ã®AIã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆgatewayã€‚</strong><br />
  ãƒ¡ãƒƒã‚»ãƒ¼ã‚¸ã‚’é€ä¿¡ã™ã‚Œã°ã€ãƒã‚±ãƒƒãƒˆã‹ã‚‰ã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆã®å¿œç­”ã‚’å—ã‘å–ã‚Œã¾ã™ã€‚ãƒ—ãƒ©ã‚°ã‚¤ãƒ³ã§Mattermostãªã©ã‚’è¿½åŠ ã§ãã¾ã™ã€‚
</p>

<Columns>
  <Card title="ã¯ã˜ã‚ã«" href="/start/getting-started" icon="rocket">
    krabkrabã‚’ã‚¤ãƒ³ã‚¹ãƒˆãƒ¼ãƒ«ã—ã€æ•°åˆ†ã§Gatewayã‚’èµ·å‹•ã§ãã¾ã™ã€‚
  </Card>
  <Card title="ã‚¦ã‚£ã‚¶ãƒ¼ãƒ‰ã‚’å®Ÿè¡Œ" href="/start/wizard" icon="sparkles">
    `krabkrab onboard`ã¨ãƒšã‚¢ãƒªãƒ³ã‚°ãƒ•ãƒ­ãƒ¼ã«ã‚ˆã‚‹ã‚¬ã‚¤ãƒ‰ä»˜ãã‚»ãƒƒãƒˆã‚¢ãƒƒãƒ—ã€‚
  </Card>
  <Card title="Control UIã‚’é–‹ã" href="/web/control-ui" icon="layout-dashboard">
    ãƒãƒ£ãƒƒãƒˆã€è¨­å®šã€ã‚»ãƒƒã‚·ãƒ§ãƒ³ç”¨ã®ãƒ–ãƒ©ã‚¦ã‚¶ãƒ€ãƒƒã‚·ãƒ¥ãƒœãƒ¼ãƒ‰ã‚’èµ·å‹•ã—ã¾ã™ã€‚
  </Card>
</Columns>

krabkrabã¯ã€å˜ä¸€ã®Gatewayãƒ—ãƒ­ã‚»ã‚¹ã‚’é€šã˜ã¦ãƒãƒ£ãƒƒãƒˆã‚¢ãƒ—ãƒªã‚’Piã®ã‚ˆã†ãªã‚³ãƒ¼ãƒ‡ã‚£ãƒ³ã‚°ã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆã«æŽ¥ç¶šã—ã¾ã™ã€‚krabkrabã‚¢ã‚·ã‚¹ã‚¿ãƒ³ãƒˆã‚’é§†å‹•ã—ã€ãƒ­ãƒ¼ã‚«ãƒ«ã¾ãŸã¯ãƒªãƒ¢ãƒ¼ãƒˆã®ã‚»ãƒƒãƒˆã‚¢ãƒƒãƒ—ã‚’ã‚µãƒãƒ¼ãƒˆã—ã¾ã™ã€‚

## ä»•çµ„ã¿

```mermaid
flowchart LR
  A["ãƒãƒ£ãƒƒãƒˆã‚¢ãƒ—ãƒª + ãƒ—ãƒ©ã‚°ã‚¤ãƒ³"] --> B["Gateway"]
  B --> C["Piã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆ"]
  B --> D["CLI"]
  B --> E["Web Control UI"]
  B --> F["macOSã‚¢ãƒ—ãƒª"]
  B --> G["iOSãŠã‚ˆã³AndroidãƒŽãƒ¼ãƒ‰"]
```

Gatewayã¯ã€ã‚»ãƒƒã‚·ãƒ§ãƒ³ã€ãƒ«ãƒ¼ãƒ†ã‚£ãƒ³ã‚°ã€ãƒãƒ£ãƒãƒ«æŽ¥ç¶šã®ä¿¡é ¼ã§ãã‚‹å”¯ä¸€ã®æƒ…å ±æºã§ã™ã€‚

## ä¸»ãªæ©Ÿèƒ½

<Columns>
  <Card title="ãƒžãƒ«ãƒãƒãƒ£ãƒãƒ«gateway" icon="network">
    å˜ä¸€ã®Gatewayãƒ—ãƒ­ã‚»ã‚¹ã§WhatsAppã€Telegramã€Discordã€iMessageã«å¯¾å¿œã€‚
  </Card>
  <Card title="ãƒ—ãƒ©ã‚°ã‚¤ãƒ³ãƒãƒ£ãƒãƒ«" icon="plug">
    æ‹¡å¼µãƒ‘ãƒƒã‚±ãƒ¼ã‚¸ã§Mattermostãªã©ã‚’è¿½åŠ ã€‚
  </Card>
  <Card title="ãƒžãƒ«ãƒã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆãƒ«ãƒ¼ãƒ†ã‚£ãƒ³ã‚°" icon="route">
    ã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆã€ãƒ¯ãƒ¼ã‚¯ã‚¹ãƒšãƒ¼ã‚¹ã€é€ä¿¡è€…ã”ã¨ã«åˆ†é›¢ã•ã‚ŒãŸã‚»ãƒƒã‚·ãƒ§ãƒ³ã€‚
  </Card>
  <Card title="ãƒ¡ãƒ‡ã‚£ã‚¢ã‚µãƒãƒ¼ãƒˆ" icon="image">
    ç”»åƒã€éŸ³å£°ã€ãƒ‰ã‚­ãƒ¥ãƒ¡ãƒ³ãƒˆã®é€å—ä¿¡ã€‚
  </Card>
  <Card title="Web Control UI" icon="monitor">
    ãƒãƒ£ãƒƒãƒˆã€è¨­å®šã€ã‚»ãƒƒã‚·ãƒ§ãƒ³ã€ãƒŽãƒ¼ãƒ‰ç”¨ã®ãƒ–ãƒ©ã‚¦ã‚¶ãƒ€ãƒƒã‚·ãƒ¥ãƒœãƒ¼ãƒ‰ã€‚
  </Card>
  <Card title="ãƒ¢ãƒã‚¤ãƒ«ãƒŽãƒ¼ãƒ‰" icon="smartphone">
    Canvaså¯¾å¿œã®iOSãŠã‚ˆã³AndroidãƒŽãƒ¼ãƒ‰ã‚’ãƒšã‚¢ãƒªãƒ³ã‚°ã€‚
  </Card>
</Columns>

## ã‚¯ã‚¤ãƒƒã‚¯ã‚¹ã‚¿ãƒ¼ãƒˆ

<Steps>
  <Step title="krabkrabã‚’ã‚¤ãƒ³ã‚¹ãƒˆãƒ¼ãƒ«">
    ```bash
    npm install -g krabkrab@latest
    ```
  </Step>
  <Step title="ã‚ªãƒ³ãƒœãƒ¼ãƒ‡ã‚£ãƒ³ã‚°ã¨ã‚µãƒ¼ãƒ“ã‚¹ã®ã‚¤ãƒ³ã‚¹ãƒˆãƒ¼ãƒ«">
    ```bash
    krabkrab onboard --install-daemon
    ```
  </Step>
  <Step title="WhatsAppã‚’ãƒšã‚¢ãƒªãƒ³ã‚°ã—ã¦Gatewayã‚’èµ·å‹•">
    ```bash
    krabkrab channels login
    krabkrab gateway --port 18789
    ```
  </Step>
</Steps>

å®Œå…¨ãªã‚¤ãƒ³ã‚¹ãƒˆãƒ¼ãƒ«ã¨é–‹ç™ºã‚»ãƒƒãƒˆã‚¢ãƒƒãƒ—ãŒå¿…è¦ã§ã™ã‹ï¼Ÿ[ã‚¯ã‚¤ãƒƒã‚¯ã‚¹ã‚¿ãƒ¼ãƒˆ](/start/quickstart)ã‚’ã”è¦§ãã ã•ã„ã€‚

## ãƒ€ãƒƒã‚·ãƒ¥ãƒœãƒ¼ãƒ‰

Gatewayã®èµ·å‹•å¾Œã€ãƒ–ãƒ©ã‚¦ã‚¶ã§Control UIã‚’é–‹ãã¾ã™ã€‚

- ãƒ­ãƒ¼ã‚«ãƒ«ãƒ‡ãƒ•ã‚©ãƒ«ãƒˆ: [http://127.0.0.1:18789/](http://127.0.0.1:18789/)
- ãƒªãƒ¢ãƒ¼ãƒˆã‚¢ã‚¯ã‚»ã‚¹: [Webã‚µãƒ¼ãƒ•ã‚§ã‚¹](/web)ãŠã‚ˆã³[Tailscale](/gateway/tailscale)

<p align="center">
  <img src="whatsapp-krabkrab.jpg" alt="KrabKrab" width="420" />
</p>

## è¨­å®šï¼ˆã‚ªãƒ—ã‚·ãƒ§ãƒ³ï¼‰

è¨­å®šã¯`~/.krabkrab/krabkrab.json`ã«ã‚ã‚Šã¾ã™ã€‚

- **ä½•ã‚‚ã—ãªã‘ã‚Œã°**ã€krabkrabã¯ãƒãƒ³ãƒ‰ãƒ«ã•ã‚ŒãŸPiãƒã‚¤ãƒŠãƒªã‚’RPCãƒ¢ãƒ¼ãƒ‰ã§ä½¿ç”¨ã—ã€é€ä¿¡è€…ã”ã¨ã®ã‚»ãƒƒã‚·ãƒ§ãƒ³ã‚’ä½œæˆã—ã¾ã™ã€‚
- åˆ¶é™ã‚’è¨­ã‘ãŸã„å ´åˆã¯ã€`channels.whatsapp.allowFrom`ã¨ï¼ˆã‚°ãƒ«ãƒ¼ãƒ—ã®å ´åˆï¼‰ãƒ¡ãƒ³ã‚·ãƒ§ãƒ³ãƒ«ãƒ¼ãƒ«ã‹ã‚‰å§‹ã‚ã¦ãã ã•ã„ã€‚

ä¾‹ï¼š

```json5
{
  channels: {
    whatsapp: {
      allowFrom: ["+15555550123"],
      groups: { "*": { requireMention: true } },
    },
  },
  messages: { groupChat: { mentionPatterns: ["@krabkrab"] } },
}
```

## ã“ã“ã‹ã‚‰å§‹ã‚ã‚‹

<Columns>
  <Card title="ãƒ‰ã‚­ãƒ¥ãƒ¡ãƒ³ãƒˆãƒãƒ–" href="/start/hubs" icon="book-open">
    ãƒ¦ãƒ¼ã‚¹ã‚±ãƒ¼ã‚¹åˆ¥ã«æ•´ç†ã•ã‚ŒãŸã™ã¹ã¦ã®ãƒ‰ã‚­ãƒ¥ãƒ¡ãƒ³ãƒˆã¨ã‚¬ã‚¤ãƒ‰ã€‚
  </Card>
  <Card title="è¨­å®š" href="/gateway/configuration" icon="settings">
    Gatewayã®ã‚³ã‚¢è¨­å®šã€ãƒˆãƒ¼ã‚¯ãƒ³ã€ãƒ—ãƒ­ãƒã‚¤ãƒ€ãƒ¼è¨­å®šã€‚
  </Card>
  <Card title="ãƒªãƒ¢ãƒ¼ãƒˆã‚¢ã‚¯ã‚»ã‚¹" href="/gateway/remote" icon="globe">
    SSHãŠã‚ˆã³tailnetã‚¢ã‚¯ã‚»ã‚¹ãƒ‘ã‚¿ãƒ¼ãƒ³ã€‚
  </Card>
  <Card title="ãƒãƒ£ãƒãƒ«" href="/channels/telegram" icon="message-square">
    WhatsAppã€Telegramã€Discordãªã©ã®ãƒãƒ£ãƒãƒ«å›ºæœ‰ã®ã‚»ãƒƒãƒˆã‚¢ãƒƒãƒ—ã€‚
  </Card>
  <Card title="ãƒŽãƒ¼ãƒ‰" href="/nodes" icon="smartphone">
    ãƒšã‚¢ãƒªãƒ³ã‚°ã¨Canvaså¯¾å¿œã®iOSãŠã‚ˆã³AndroidãƒŽãƒ¼ãƒ‰ã€‚
  </Card>
  <Card title="ãƒ˜ãƒ«ãƒ—" href="/help" icon="life-buoy">
    ä¸€èˆ¬çš„ãªä¿®æ­£ã¨ãƒˆãƒ©ãƒ–ãƒ«ã‚·ãƒ¥ãƒ¼ãƒ†ã‚£ãƒ³ã‚°ã®ã‚¨ãƒ³ãƒˆãƒªãƒ¼ãƒã‚¤ãƒ³ãƒˆã€‚
  </Card>
</Columns>

## è©³ç´°

<Columns>
  <Card title="å…¨æ©Ÿèƒ½ãƒªã‚¹ãƒˆ" href="/concepts/features" icon="list">
    ãƒãƒ£ãƒãƒ«ã€ãƒ«ãƒ¼ãƒ†ã‚£ãƒ³ã‚°ã€ãƒ¡ãƒ‡ã‚£ã‚¢æ©Ÿèƒ½ã®å®Œå…¨ãªä¸€è¦§ã€‚
  </Card>
  <Card title="ãƒžãƒ«ãƒã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆãƒ«ãƒ¼ãƒ†ã‚£ãƒ³ã‚°" href="/concepts/multi-agent" icon="route">
    ãƒ¯ãƒ¼ã‚¯ã‚¹ãƒšãƒ¼ã‚¹ã®åˆ†é›¢ã¨ã‚¨ãƒ¼ã‚¸ã‚§ãƒ³ãƒˆã”ã¨ã®ã‚»ãƒƒã‚·ãƒ§ãƒ³ã€‚
  </Card>
  <Card title="ã‚»ã‚­ãƒ¥ãƒªãƒ†ã‚£" href="/gateway/security" icon="shield">
    ãƒˆãƒ¼ã‚¯ãƒ³ã€è¨±å¯ãƒªã‚¹ãƒˆã€å®‰å…¨åˆ¶å¾¡ã€‚
  </Card>
  <Card title="ãƒˆãƒ©ãƒ–ãƒ«ã‚·ãƒ¥ãƒ¼ãƒ†ã‚£ãƒ³ã‚°" href="/gateway/troubleshooting" icon="wrench">
    Gatewayã®è¨ºæ–­ã¨ä¸€èˆ¬çš„ãªã‚¨ãƒ©ãƒ¼ã€‚
  </Card>
  <Card title="æ¦‚è¦ã¨ã‚¯ãƒ¬ã‚¸ãƒƒãƒˆ" href="/reference/credits" icon="info">
    ãƒ—ãƒ­ã‚¸ã‚§ã‚¯ãƒˆã®èµ·æºã€è²¢çŒ®è€…ã€ãƒ©ã‚¤ã‚»ãƒ³ã‚¹ã€‚
  </Card>
</Columns>

