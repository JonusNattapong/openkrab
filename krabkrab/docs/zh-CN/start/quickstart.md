---
read_when:
  - ä½ å¸Œæœ›ä»¥æœ€å¿«çš„æ–¹å¼ä»Žå®‰è£…åˆ°è¿è¡Œä¸€ä¸ªå¯ç”¨çš„ Gateway ç½‘å…³
summary: å®‰è£… KrabKrabï¼Œå®Œæˆ Gateway ç½‘å…³æ–°æ‰‹å¼•å¯¼ï¼Œå¹¶é…å¯¹ä½ çš„ç¬¬ä¸€ä¸ªæ¸ é“ã€‚
title: å¿«é€Ÿå¼€å§‹
x-i18n:
  generated_at: "2026-02-04T17:53:21Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 3c5da65996f89913cd115279ae21dcab794eadd14595951b676d8f7864fbbe2d
  source_path: start/quickstart.md
  workflow: 15
---

<Note>
KrabKrab éœ€è¦ Node 22 æˆ–æ›´æ–°ç‰ˆæœ¬ã€‚
</Note>

## å®‰è£…

<Tabs>
  <Tab title="npm">
    ```bash
    npm install -g krabkrab@latest
    ```
  </Tab>
  <Tab title="pnpm">
    ```bash
    pnpm add -g krabkrab@latest
    ```
  </Tab>
</Tabs>

## æ–°æ‰‹å¼•å¯¼å¹¶è¿è¡Œ Gateway ç½‘å…³

<Steps>
  <Step title="æ–°æ‰‹å¼•å¯¼å¹¶å®‰è£…æœåŠ¡">
    ```bash
    krabkrab onboard --install-daemon
    ```
  </Step>
  <Step title="é…å¯¹ WhatsApp">
    ```bash
    krabkrab channels login
    ```
  </Step>
  <Step title="å¯åŠ¨ Gateway ç½‘å…³">
    ```bash
    krabkrab gateway --port 18789
    ```
  </Step>
</Steps>

å®Œæˆæ–°æ‰‹å¼•å¯¼åŽï¼ŒGateway ç½‘å…³å°†é€šè¿‡ç”¨æˆ·æœåŠ¡è¿è¡Œã€‚ä½ ä¹Ÿå¯ä»¥ä½¿ç”¨ `krabkrab gateway` æ‰‹åŠ¨å¯åŠ¨ã€‚

<Info>
ä¹‹åŽåœ¨ npm å®‰è£…å’Œ git å®‰è£…ä¹‹é—´åˆ‡æ¢éžå¸¸ç®€å•ã€‚å®‰è£…å¦ä¸€ç§æ–¹å¼åŽï¼Œè¿è¡Œ
`krabkrab doctor` å³å¯æ›´æ–° Gateway ç½‘å…³æœåŠ¡å…¥å£ç‚¹ã€‚
</Info>

## ä»Žæºç å®‰è£…ï¼ˆå¼€å‘ï¼‰

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
pnpm install
pnpm ui:build # é¦–æ¬¡è¿è¡Œæ—¶ä¼šè‡ªåŠ¨å®‰è£… UI ä¾èµ–
pnpm build
krabkrab onboard --install-daemon
```

å¦‚æžœä½ è¿˜æ²¡æœ‰å…¨å±€å®‰è£…ï¼Œå¯ä»¥åœ¨ä»“åº“ç›®å½•ä¸­é€šè¿‡ `pnpm krabkrab ...` è¿è¡Œæ–°æ‰‹å¼•å¯¼ã€‚

## å¤šå®žä¾‹å¿«é€Ÿå¼€å§‹ï¼ˆå¯é€‰ï¼‰

```bash
krabkrab_CONFIG_PATH=~/.krabkrab/a.json \
krabkrab_STATE_DIR=~/.krabkrab-a \
krabkrab gateway --port 19001
```

## å‘é€æµ‹è¯•æ¶ˆæ¯

éœ€è¦ä¸€ä¸ªæ­£åœ¨è¿è¡Œçš„ Gateway ç½‘å…³ã€‚

```bash
krabkrab message send --target +15555550123 --message "Hello from KrabKrab"
```

