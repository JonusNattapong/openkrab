---
summary: å…³äºŽ KrabKrab å®‰è£…ã€é…ç½®å’Œä½¿ç”¨çš„å¸¸è§é—®é¢˜
title: å¸¸è§é—®é¢˜
x-i18n:
  generated_at: "2026-02-01T21:32:04Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 5a611f2fda3325b1c7a9ec518616d87c78be41e2bfbe86244ae4f48af3815a26
  source_path: help/faq.md
  workflow: 15
---

# å¸¸è§é—®é¢˜

å¿«é€Ÿè§£ç­”åŠé’ˆå¯¹å®žé™…éƒ¨ç½²åœºæ™¯ï¼ˆæœ¬åœ°å¼€å‘ã€VPSã€å¤šæ™ºèƒ½ä½“ã€OAuth/API å¯†é’¥ã€æ¨¡åž‹æ•…éšœè½¬ç§»ï¼‰çš„æ·±å…¥æ•…éšœæŽ’é™¤ã€‚è¿è¡Œæ—¶è¯Šæ–­è¯·å‚é˜…[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)ã€‚å®Œæ•´é…ç½®å‚è€ƒè¯·å‚é˜…[é…ç½®](/gateway/configuration)ã€‚

## ç›®å½•

- [å¿«é€Ÿå¼€å§‹ä¸Žé¦–æ¬¡è¿è¡Œè®¾ç½®](#quick-start-and-firstrun-setup)
  - [æˆ‘å¡ä½äº†ï¼Œæœ€å¿«çš„æŽ’éšœæ–¹æ³•æ˜¯ä»€ä¹ˆï¼Ÿ](#im-stuck-whats-the-fastest-way-to-get-unstuck)
  - [å®‰è£…å’Œè®¾ç½® KrabKrab çš„æŽ¨èæ–¹å¼æ˜¯ä»€ä¹ˆï¼Ÿ](#whats-the-recommended-way-to-install-and-set-up-krabkrab)
  - [æ–°æ‰‹å¼•å¯¼åŽå¦‚ä½•æ‰“å¼€ä»ªè¡¨æ¿ï¼Ÿ](#how-do-i-open-the-dashboard-after-onboarding)
  - [å¦‚ä½•åœ¨æœ¬åœ°å’Œè¿œç¨‹çŽ¯å¢ƒä¸­éªŒè¯ä»ªè¡¨æ¿ï¼ˆä»¤ç‰Œï¼‰ï¼Ÿ](#how-do-i-authenticate-the-dashboard-token-on-localhost-vs-remote)
  - [æˆ‘éœ€è¦ä»€ä¹ˆè¿è¡Œæ—¶ï¼Ÿ](#what-runtime-do-i-need)
  - [èƒ½åœ¨ Raspberry Pi ä¸Šè¿è¡Œå—ï¼Ÿ](#does-it-run-on-raspberry-pi)
  - [Raspberry Pi å®‰è£…æœ‰ä»€ä¹ˆå»ºè®®ï¼Ÿ](#any-tips-for-raspberry-pi-installs)
  - [å¡åœ¨ "wake up my friend" / æ–°æ‰‹å¼•å¯¼æ— æ³•å¯åŠ¨ï¼Œæ€Žä¹ˆåŠžï¼Ÿ](#it-is-stuck-on-wake-up-my-friend-onboarding-will-not-hatch-what-now)
  - [èƒ½å¦å°†æˆ‘çš„è®¾ç½®è¿ç§»åˆ°æ–°æœºå™¨ï¼ˆMac miniï¼‰è€Œä¸é‡æ–°è¿›è¡Œæ–°æ‰‹å¼•å¯¼ï¼Ÿ](#can-i-migrate-my-setup-to-a-new-machine-mac-mini-without-redoing-onboarding)
  - [åœ¨å“ªé‡ŒæŸ¥çœ‹æœ€æ–°ç‰ˆæœ¬çš„æ›´æ–°å†…å®¹ï¼Ÿ](#where-do-i-see-whats-new-in-the-latest-version)
  - [æ— æ³•è®¿é—® docs.krabkrab.aiï¼ˆSSL é”™è¯¯ï¼‰ï¼Œæ€Žä¹ˆåŠžï¼Ÿ](#i-cant-access-docskrabkrabai-ssl-error-what-now)
  - [stable å’Œ beta æœ‰ä»€ä¹ˆåŒºåˆ«ï¼Ÿ](#whats-the-difference-between-stable-and-beta)
- [å¦‚ä½•å®‰è£… beta ç‰ˆæœ¬ï¼Œbeta å’Œ dev æœ‰ä»€ä¹ˆåŒºåˆ«ï¼Ÿ](#how-do-i-install-the-beta-version-and-whats-the-difference-between-beta-and-dev)
  - [å¦‚ä½•è¯•ç”¨æœ€æ–°ä»£ç ï¼Ÿ](#how-do-i-try-the-latest-bits)
  - [å®‰è£…å’Œæ–°æ‰‹å¼•å¯¼é€šå¸¸éœ€è¦å¤šé•¿æ—¶é—´ï¼Ÿ](#how-long-does-install-and-onboarding-usually-take)
  - [å®‰è£…ç¨‹åºå¡ä½äº†ï¼Ÿå¦‚ä½•èŽ·å–æ›´å¤šåé¦ˆï¼Ÿ](#installer-stuck-how-do-i-get-more-feedback)
  - [Windows å®‰è£…æç¤ºæ‰¾ä¸åˆ° git æˆ–æ— æ³•è¯†åˆ« krabkrab](#windows-install-says-git-not-found-or-krabkrab-not-recognized)
  - [æ–‡æ¡£æ²¡æœ‰è§£ç­”æˆ‘çš„é—®é¢˜â€”â€”å¦‚ä½•èŽ·å¾—æ›´å¥½çš„ç­”æ¡ˆï¼Ÿ](#the-docs-didnt-answer-my-question-how-do-i-get-a-better-answer)
  - [å¦‚ä½•åœ¨ Linux ä¸Šå®‰è£… KrabKrabï¼Ÿ](#how-do-i-install-krabkrab-on-linux)
  - [å¦‚ä½•åœ¨ VPS ä¸Šå®‰è£… KrabKrabï¼Ÿ](#how-do-i-install-krabkrab-on-a-vps)
  - [äº‘/VPS å®‰è£…æŒ‡å—åœ¨å“ªé‡Œï¼Ÿ](#where-are-the-cloudvps-install-guides)
  - [å¯ä»¥è®© KrabKrab è‡ªè¡Œæ›´æ–°å—ï¼Ÿ](#can-i-ask-krabkrab-to-update-itself)
  - [æ–°æ‰‹å¼•å¯¼å‘å¯¼å…·ä½“åšäº†ä»€ä¹ˆï¼Ÿ](#what-does-the-onboarding-wizard-actually-do)
  - [è¿è¡Œ KrabKrab éœ€è¦ Claude æˆ– OpenAI è®¢é˜…å—ï¼Ÿ](#do-i-need-a-claude-or-openai-subscription-to-run-this)
  - [èƒ½å¦ä½¿ç”¨ Claude Max è®¢é˜…è€Œä¸éœ€è¦ API å¯†é’¥ï¼Ÿ](#can-i-use-claude-max-subscription-without-an-api-key)
  - [Anthropic "setup-token" è®¤è¯å¦‚ä½•å·¥ä½œï¼Ÿ](#how-does-anthropic-setuptoken-auth-work)
  - [åœ¨å“ªé‡ŒèŽ·å– Anthropic setup-tokenï¼Ÿ](#where-do-i-find-an-anthropic-setuptoken)
  - [æ˜¯å¦æ”¯æŒ Claude è®¢é˜…è®¤è¯ï¼ˆClaude Code OAuthï¼‰ï¼Ÿ](#do-you-support-claude-subscription-auth-claude-code-oauth)
  - [ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ° `HTTP 429: rate_limit_error`ï¼ˆæ¥è‡ª Anthropicï¼‰ï¼Ÿ](#why-am-i-seeing-http-429-ratelimiterror-from-anthropic)
  - [æ”¯æŒ AWS Bedrock å—ï¼Ÿ](#is-aws-bedrock-supported)
  - [Codex è®¤è¯å¦‚ä½•å·¥ä½œï¼Ÿ](#how-does-codex-auth-work)
  - [æ˜¯å¦æ”¯æŒ OpenAI è®¢é˜…è®¤è¯ï¼ˆCodex OAuthï¼‰ï¼Ÿ](#do-you-support-openai-subscription-auth-codex-oauth)
  - [å¦‚ä½•è®¾ç½® Gemini CLI OAuthï¼Ÿ](#how-do-i-set-up-gemini-cli-oauth)
  - [æœ¬åœ°æ¨¡åž‹é€‚åˆæ—¥å¸¸èŠå¤©å—ï¼Ÿ](#is-a-local-model-ok-for-casual-chats)
  - [å¦‚ä½•å°†æ‰˜ç®¡æ¨¡åž‹æµé‡é™åˆ¶åœ¨ç‰¹å®šåŒºåŸŸï¼Ÿ](#how-do-i-keep-hosted-model-traffic-in-a-specific-region)
  - [æˆ‘å¿…é¡»è´­ä¹° Mac Mini æ‰èƒ½å®‰è£…å—ï¼Ÿ](#do-i-have-to-buy-a-mac-mini-to-install-this)
  - [iMessage æ”¯æŒéœ€è¦ Mac mini å—ï¼Ÿ](#do-i-need-a-mac-mini-for-imessage-support)
  - [å¦‚æžœæˆ‘ä¹°äº† Mac mini è¿è¡Œ KrabKrabï¼Œèƒ½è¿žæŽ¥åˆ°æˆ‘çš„ MacBook Pro å—ï¼Ÿ](#if-i-buy-a-mac-mini-to-run-krabkrab-can-i-connect-it-to-my-macbook-pro)
  - [å¯ä»¥ä½¿ç”¨ Bun å—ï¼Ÿ](#can-i-use-bun)
  - [Telegramï¼š`allowFrom` å¡«ä»€ä¹ˆï¼Ÿ](#telegram-what-goes-in-allowfrom)
  - [å¤šäººèƒ½å¦ä½¿ç”¨åŒä¸€ä¸ª WhatsApp å·ç é…åˆä¸åŒçš„ KrabKrab å®žä¾‹ï¼Ÿ](#can-multiple-people-use-one-whatsapp-number-with-different-krabkrab-instances)
  - [èƒ½å¦åŒæ—¶è¿è¡Œä¸€ä¸ªâ€œå¿«é€ŸèŠå¤©â€æ™ºèƒ½ä½“å’Œä¸€ä¸ªâ€œç”¨ Opus ç¼–ç¨‹â€çš„æ™ºèƒ½ä½“ï¼Ÿ](#can-i-run-a-fast-chat-agent-and-an-opus-for-coding-agent)
  - [Homebrew åœ¨ Linux ä¸Šå¯ç”¨å—ï¼Ÿ](#does-homebrew-work-on-linux)
  - [å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…å’Œ npm å®‰è£…æœ‰ä»€ä¹ˆåŒºåˆ«ï¼Ÿ](#whats-the-difference-between-the-hackable-git-install-and-npm-install)
  - [ä¹‹åŽå¯ä»¥åœ¨ npm å’Œ git å®‰è£…ä¹‹é—´åˆ‡æ¢å—ï¼Ÿ](#can-i-switch-between-npm-and-git-installs-later)
  - [åº”è¯¥åœ¨ç¬”è®°æœ¬ç”µè„‘è¿˜æ˜¯ VPS ä¸Šè¿è¡Œ Gateway ç½‘å…³ï¼Ÿ](#should-i-run-the-gateway-on-my-laptop-or-a-vps)
  - [åœ¨ä¸“ç”¨æœºå™¨ä¸Šè¿è¡Œ KrabKrab æœ‰å¤šé‡è¦ï¼Ÿ](#how-important-is-it-to-run-krabkrab-on-a-dedicated-machine)
  - [VPS çš„æœ€ä½Žè¦æ±‚å’ŒæŽ¨èæ“ä½œç³»ç»Ÿæ˜¯ä»€ä¹ˆï¼Ÿ](#what-are-the-minimum-vps-requirements-and-recommended-os)
  - [å¯ä»¥åœ¨è™šæ‹Ÿæœºä¸­è¿è¡Œ KrabKrab å—ï¼Ÿæœ‰ä»€ä¹ˆè¦æ±‚ï¼Ÿ](#can-i-run-krabkrab-in-a-vm-and-what-are-the-requirements)
- [ä»€ä¹ˆæ˜¯ KrabKrabï¼Ÿ](#what-is-krabkrab)
  - [ç”¨ä¸€æ®µè¯æè¿° KrabKrabï¼Ÿ](#what-is-krabkrab-in-one-paragraph)
  - [ä»·å€¼ä¸»å¼ æ˜¯ä»€ä¹ˆï¼Ÿ](#whats-the-value-proposition)
  - [åˆšè®¾ç½®å¥½ï¼Œåº”è¯¥å…ˆåšä»€ä¹ˆï¼Ÿ](#i-just-set-it-up-what-should-i-do-first)
  - [KrabKrab æ—¥å¸¸æœ€å¸¸ç”¨çš„äº”ä¸ªåœºæ™¯æ˜¯ä»€ä¹ˆï¼Ÿ](#what-are-the-top-five-everyday-use-cases-for-krabkrab)
  - [KrabKrab èƒ½å¦å¸®åŠ© SaaS è¿›è¡ŒèŽ·å®¢ã€å¤–è”ã€å¹¿å‘Šå’Œåšå®¢ï¼Ÿ](#can-krabkrab-help-with-lead-gen-outreach-ads-and-blogs-for-a-saas)
  - [ç›¸æ¯” Claude Codeï¼Œåœ¨ Web å¼€å‘æ–¹é¢æœ‰ä»€ä¹ˆä¼˜åŠ¿ï¼Ÿ](#what-are-the-advantages-vs-claude-code-for-web-development)
- [Skills ä¸Žè‡ªåŠ¨åŒ–](#skills-and-automation)
  - [å¦‚ä½•è‡ªå®šä¹‰ Skills è€Œä¸å¼„è„ä»“åº“ï¼Ÿ](#how-do-i-customize-skills-without-keeping-the-repo-dirty)
  - [å¯ä»¥ä»Žè‡ªå®šä¹‰æ–‡ä»¶å¤¹åŠ è½½ Skills å—ï¼Ÿ](#can-i-load-skills-from-a-custom-folder)
  - [å¦‚ä½•ä¸ºä¸åŒä»»åŠ¡ä½¿ç”¨ä¸åŒæ¨¡åž‹ï¼Ÿ](#how-can-i-use-different-models-for-different-tasks)
  - [æœºå™¨äººåœ¨æ‰§è¡Œç¹é‡å·¥ä½œæ—¶å¡ä½äº†ï¼Œå¦‚ä½•å¸è½½ä»»åŠ¡ï¼Ÿ](#the-bot-freezes-while-doing-heavy-work-how-do-i-offload-that)
  - [å®šæ—¶ä»»åŠ¡æˆ–æé†’æ²¡æœ‰è§¦å‘ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆï¼Ÿ](#cron-or-reminders-do-not-fire-what-should-i-check)
  - [å¦‚ä½•åœ¨ Linux ä¸Šå®‰è£… Skillsï¼Ÿ](#how-do-i-install-skills-on-linux)
  - [KrabKrab èƒ½å¦æŒ‰è®¡åˆ’æˆ–åœ¨åŽå°æŒç»­è¿è¡Œä»»åŠ¡ï¼Ÿ](#can-krabkrab-run-tasks-on-a-schedule-or-continuously-in-the-background)
  - [èƒ½å¦ä»Ž Linux è¿è¡Œä»…é™ Apple/macOS çš„ Skillsï¼Ÿ](#can-i-run-applemacosonly-skills-from-linux)
  - [æœ‰ Notion æˆ– HeyGen é›†æˆå—ï¼Ÿ](#do-you-have-a-notion-or-heygen-integration)
  - [å¦‚ä½•å®‰è£…ç”¨äºŽæµè§ˆå™¨æŽ¥ç®¡çš„ Chrome æ‰©å±•ï¼Ÿ](#how-do-i-install-the-chrome-extension-for-browser-takeover)
- [æ²™ç®±ä¸Žè®°å¿†](#sandboxing-and-memory)
  - [æœ‰ä¸“é—¨çš„æ²™ç®±æ–‡æ¡£å—ï¼Ÿ](#is-there-a-dedicated-sandboxing-doc)
  - [å¦‚ä½•å°†ä¸»æœºæ–‡ä»¶å¤¹ç»‘å®šåˆ°æ²™ç®±ä¸­ï¼Ÿ](#how-do-i-bind-a-host-folder-into-the-sandbox)
  - [è®°å¿†æ˜¯å¦‚ä½•å·¥ä½œçš„ï¼Ÿ](#how-does-memory-work)
  - [è®°å¿†æ€»æ˜¯é—å¿˜ï¼Œå¦‚ä½•è®©å®ƒæŒä¹…ä¿å­˜ï¼Ÿ](#memory-keeps-forgetting-things-how-do-i-make-it-stick)
  - [è®°å¿†æ˜¯å¦æ°¸ä¹…ä¿ç•™ï¼Ÿæœ‰ä»€ä¹ˆé™åˆ¶ï¼Ÿ](#does-memory-persist-forever-what-are-the-limits)
  - [è¯­ä¹‰è®°å¿†æœç´¢éœ€è¦ OpenAI API å¯†é’¥å—ï¼Ÿ](#does-semantic-memory-search-require-an-openai-api-key)
- [ç£ç›˜ä¸Šçš„æ–‡ä»¶ä½ç½®](#where-things-live-on-disk)
  - [KrabKrab ä½¿ç”¨çš„æ‰€æœ‰æ•°æ®éƒ½ä¿å­˜åœ¨æœ¬åœ°å—ï¼Ÿ](#is-all-data-used-with-krabkrab-saved-locally)
  - [KrabKrab å°†æ•°æ®å­˜å‚¨åœ¨å“ªé‡Œï¼Ÿ](#where-does-krabkrab-store-its-data)
  - [AGENTS.md / SOUL.md / USER.md / MEMORY.md åº”è¯¥æ”¾åœ¨å“ªé‡Œï¼Ÿ](#where-should-agentsmd-soulmd-usermd-memorymd-live)
  - [æŽ¨èçš„å¤‡ä»½ç­–ç•¥æ˜¯ä»€ä¹ˆï¼Ÿ](#whats-the-recommended-backup-strategy)
  - [å¦‚ä½•å®Œå…¨å¸è½½ KrabKrabï¼Ÿ](#how-do-i-completely-uninstall-krabkrab)
  - [æ™ºèƒ½ä½“å¯ä»¥åœ¨å·¥ä½œåŒºå¤–å·¥ä½œå—ï¼Ÿ](#can-agents-work-outside-the-workspace)
  - [æˆ‘å¤„äºŽè¿œç¨‹æ¨¡å¼â€”â€”ä¼šè¯å­˜å‚¨åœ¨å“ªé‡Œï¼Ÿ](#im-in-remote-mode-where-is-the-session-store)
- [é…ç½®åŸºç¡€](#config-basics)
  - [é…ç½®æ–‡ä»¶æ˜¯ä»€ä¹ˆæ ¼å¼ï¼Ÿåœ¨å“ªé‡Œï¼Ÿ](#what-format-is-the-config-where-is-it)
  - [æˆ‘è®¾ç½®äº† `gateway.bind: "lan"`ï¼ˆæˆ– `"tailnet"`ï¼‰ï¼ŒçŽ°åœ¨ä»€ä¹ˆéƒ½ç›‘å¬ä¸äº† / UI æ˜¾ç¤ºæœªæŽˆæƒ](#i-set-gatewaybind-lan-or-tailnet-and-now-nothing-listens-the-ui-says-unauthorized)
  - [ä¸ºä»€ä¹ˆçŽ°åœ¨åœ¨ localhost ä¹Ÿéœ€è¦ä»¤ç‰Œï¼Ÿ](#why-do-i-need-a-token-on-localhost-now)
  - [æ›´æ”¹é…ç½®åŽéœ€è¦é‡å¯å—ï¼Ÿ](#do-i-have-to-restart-after-changing-config)
  - [å¦‚ä½•å¯ç”¨ç½‘ç»œæœç´¢ï¼ˆå’Œç½‘é¡µæŠ“å–ï¼‰ï¼Ÿ](#how-do-i-enable-web-search-and-web-fetch)
  - [config.apply æ¸…ç©ºäº†æˆ‘çš„é…ç½®ï¼Œå¦‚ä½•æ¢å¤å’Œé¿å…ï¼Ÿ](#configapply-wiped-my-config-how-do-i-recover-and-avoid-this)
  - [å¦‚ä½•è¿è¡Œä¸€ä¸ªä¸­å¿ƒ Gateway ç½‘å…³é…åˆè·¨è®¾å¤‡çš„ä¸“ç”¨å·¥ä½œèŠ‚ç‚¹ï¼Ÿ](#how-do-i-run-a-central-gateway-with-specialized-workers-across-devices)
  - [KrabKrab æµè§ˆå™¨å¯ä»¥æ— å¤´è¿è¡Œå—ï¼Ÿ](#can-the-krabkrab-browser-run-headless)
  - [å¦‚ä½•ä½¿ç”¨ Brave è¿›è¡Œæµè§ˆå™¨æŽ§åˆ¶ï¼Ÿ](#how-do-i-use-brave-for-browser-control)
- [è¿œç¨‹ Gateway ç½‘å…³ä¸ŽèŠ‚ç‚¹](#remote-gateways-nodes)
  - [å‘½ä»¤å¦‚ä½•åœ¨ Telegramã€Gateway ç½‘å…³å’ŒèŠ‚ç‚¹ä¹‹é—´ä¼ æ’­ï¼Ÿ](#how-do-commands-propagate-between-telegram-the-gateway-and-nodes)
  - [å¦‚æžœ Gateway ç½‘å…³æ‰˜ç®¡åœ¨è¿œç¨‹ï¼Œæˆ‘çš„æ™ºèƒ½ä½“å¦‚ä½•è®¿é—®æˆ‘çš„ç”µè„‘ï¼Ÿ](#how-can-my-agent-access-my-computer-if-the-gateway-is-hosted-remotely)
  - [Tailscale å·²è¿žæŽ¥ä½†æ”¶ä¸åˆ°å›žå¤ï¼Œæ€Žä¹ˆåŠžï¼Ÿ](#tailscale-is-connected-but-i-get-no-replies-what-now)
  - [ä¸¤ä¸ª KrabKrab å®žä¾‹ï¼ˆæœ¬åœ° + VPSï¼‰å¯ä»¥äº’ç›¸é€šä¿¡å—ï¼Ÿ](#can-two-krabkrab-instances-talk-to-each-other-local-vps)
  - [å¤šä¸ªæ™ºèƒ½ä½“éœ€è¦ç‹¬ç«‹çš„ VPS å—ï¼Ÿ](#do-i-need-separate-vpses-for-multiple-agents)
  - [åœ¨ä¸ªäººç¬”è®°æœ¬ç”µè„‘ä¸Šä½¿ç”¨èŠ‚ç‚¹è€Œä¸æ˜¯ä»Ž VPS SSH æœ‰ä»€ä¹ˆå¥½å¤„ï¼Ÿ](#is-there-a-benefit-to-using-a-node-on-my-personal-laptop-instead-of-ssh-from-a-vps)
  - [èŠ‚ç‚¹ä¼šè¿è¡Œ Gateway ç½‘å…³æœåŠ¡å—ï¼Ÿ](#do-nodes-run-a-gateway-service)
  - [æœ‰ API / RPC æ–¹å¼æ¥åº”ç”¨é…ç½®å—ï¼Ÿ](#is-there-an-api-rpc-way-to-apply-config)
  - [é¦–æ¬¡å®‰è£…çš„æœ€å°â€œåˆç†â€é…ç½®æ˜¯ä»€ä¹ˆï¼Ÿ](#whats-a-minimal-sane-config-for-a-first-install)
  - [å¦‚ä½•åœ¨ VPS ä¸Šè®¾ç½® Tailscale å¹¶ä»Ž Mac è¿žæŽ¥ï¼Ÿ](#how-do-i-set-up-tailscale-on-a-vps-and-connect-from-my-mac)
  - [å¦‚ä½•å°† Mac èŠ‚ç‚¹è¿žæŽ¥åˆ°è¿œç¨‹ Gateway ç½‘å…³ï¼ˆTailscale Serveï¼‰ï¼Ÿ](#how-do-i-connect-a-mac-node-to-a-remote-gateway-tailscale-serve)
  - [åº”è¯¥åœ¨ç¬¬äºŒå°ç¬”è®°æœ¬ä¸Šå®‰è£…è¿˜æ˜¯åªæ·»åŠ ä¸€ä¸ªèŠ‚ç‚¹ï¼Ÿ](#should-i-install-on-a-second-laptop-or-just-add-a-node)
- [çŽ¯å¢ƒå˜é‡å’Œ .env åŠ è½½](#env-vars-and-env-loading)
  - [KrabKrab å¦‚ä½•åŠ è½½çŽ¯å¢ƒå˜é‡ï¼Ÿ](#how-does-krabkrab-load-environment-variables)
  - [â€œæˆ‘é€šè¿‡æœåŠ¡å¯åŠ¨äº† Gateway ç½‘å…³ï¼Œä½†çŽ¯å¢ƒå˜é‡æ¶ˆå¤±äº†ã€‚â€æ€Žä¹ˆåŠžï¼Ÿ](#i-started-the-gateway-via-the-service-and-my-env-vars-disappeared-what-now)
  - [æˆ‘è®¾ç½®äº† `COPILOT_GITHUB_TOKEN`ï¼Œä½† models status æ˜¾ç¤º"Shell env: off"ï¼Œä¸ºä»€ä¹ˆï¼Ÿ](#i-set-copilotgithubtoken-but-models-status-shows-shell-env-off-why)
- [ä¼šè¯ä¸Žå¤šèŠå¤©](#sessions-multiple-chats)
  - [å¦‚ä½•å¼€å§‹ä¸€ä¸ªæ–°å¯¹è¯ï¼Ÿ](#how-do-i-start-a-fresh-conversation)
  - [å¦‚æžœæˆ‘ä»Žä¸å‘é€ `/new`ï¼Œä¼šè¯ä¼šè‡ªåŠ¨é‡ç½®å—ï¼Ÿ](#do-sessions-reset-automatically-if-i-never-send-new)
  - [èƒ½å¦åˆ›å»ºä¸€ä¸ª KrabKrab å®žä¾‹å›¢é˜Ÿâ€”â€”ä¸€ä¸ª CEO å’Œå¤šä¸ªæ™ºèƒ½ä½“ï¼Ÿ](#is-there-a-way-to-make-a-team-of-krabkrab-instances-one-ceo-and-many-agents)
  - [ä¸ºä»€ä¹ˆä¸Šä¸‹æ–‡åœ¨ä»»åŠ¡ä¸­é€”è¢«æˆªæ–­äº†ï¼Ÿå¦‚ä½•é˜²æ­¢ï¼Ÿ](#why-did-context-get-truncated-midtask-how-do-i-prevent-it)
  - [å¦‚ä½•å®Œå…¨é‡ç½® KrabKrab ä½†ä¿ç•™å®‰è£…ï¼Ÿ](#how-do-i-completely-reset-krabkrab-but-keep-it-installed)
  - [æˆ‘é‡åˆ°äº†"context too large"é”™è¯¯â€”â€”å¦‚ä½•é‡ç½®æˆ–åŽ‹ç¼©ï¼Ÿ](#im-getting-context-too-large-errors-how-do-i-reset-or-compact)
  - [ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ°"LLM request rejected: messages.N.content.X.tool_use.input: Field required"ï¼Ÿ](#why-am-i-seeing-llm-request-rejected-messagesncontentxtooluseinput-field-required)
  - [ä¸ºä»€ä¹ˆæ¯ 30 åˆ†é’Ÿæ”¶åˆ°ä¸€æ¬¡å¿ƒè·³æ¶ˆæ¯ï¼Ÿ](#why-am-i-getting-heartbeat-messages-every-30-minutes)
  - [éœ€è¦åœ¨ WhatsApp ç¾¤ç»„ä¸­æ·»åŠ â€œæœºå™¨äººè´¦å·â€å—ï¼Ÿ](#do-i-need-to-add-a-bot-account-to-a-whatsapp-group)
  - [å¦‚ä½•èŽ·å– WhatsApp ç¾¤ç»„çš„ JIDï¼Ÿ](#how-do-i-get-the-jid-of-a-whatsapp-group)
  - [ä¸ºä»€ä¹ˆ KrabKrab ä¸åœ¨ç¾¤ç»„ä¸­å›žå¤ï¼Ÿ](#why-doesnt-krabkrab-reply-in-a-group)
  - [ç¾¤ç»„/çº¿ç¨‹ä¸Žç§èŠå…±äº«ä¸Šä¸‹æ–‡å—ï¼Ÿ](#do-groupsthreads-share-context-with-dms)
  - [å¯ä»¥åˆ›å»ºå¤šå°‘ä¸ªå·¥ä½œåŒºå’Œæ™ºèƒ½ä½“ï¼Ÿ](#how-many-workspaces-and-agents-can-i-create)
  - [å¯ä»¥åŒæ—¶è¿è¡Œå¤šä¸ªæœºå™¨äººæˆ–èŠå¤©ï¼ˆSlackï¼‰å—ï¼Ÿåº”è¯¥å¦‚ä½•è®¾ç½®ï¼Ÿ](#can-i-run-multiple-bots-or-chats-at-the-same-time-slack-and-how-should-i-set-that-up)
- [æ¨¡åž‹ï¼šé»˜è®¤å€¼ã€é€‰æ‹©ã€åˆ«åã€åˆ‡æ¢](#models-defaults-selection-aliases-switching)
  - [ä»€ä¹ˆæ˜¯â€œé»˜è®¤æ¨¡åž‹â€ï¼Ÿ](#what-is-the-default-model)
  - [æŽ¨èä»€ä¹ˆæ¨¡åž‹ï¼Ÿ](#what-model-do-you-recommend)
  - [å¦‚ä½•åœ¨ä¸æ¸…ç©ºé…ç½®çš„æƒ…å†µä¸‹åˆ‡æ¢æ¨¡åž‹ï¼Ÿ](#how-do-i-switch-models-without-wiping-my-config)
  - [å¯ä»¥ä½¿ç”¨è‡ªæ‰˜ç®¡æ¨¡åž‹ï¼ˆllama.cppã€vLLMã€Ollamaï¼‰å—ï¼Ÿ](#can-i-use-selfhosted-models-llamacpp-vllm-ollama)
  - [KrabKrabã€Flawd å’Œ Krill ä½¿ç”¨ä»€ä¹ˆæ¨¡åž‹ï¼Ÿ](#what-do-krabkrab-flawd-and-krill-use-for-models)
  - [å¦‚ä½•åœ¨è¿è¡Œä¸­åˆ‡æ¢æ¨¡åž‹ï¼ˆæ— éœ€é‡å¯ï¼‰ï¼Ÿ](#how-do-i-switch-models-on-the-fly-without-restarting)
  - [èƒ½å¦æ—¥å¸¸ä»»åŠ¡ç”¨ GPT 5.2ï¼Œç¼–ç¨‹ç”¨ Codex 5.2ï¼Ÿ](#can-i-use-gpt-52-for-daily-tasks-and-codex-52-for-coding)
  - [ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ°"Model â€¦ is not allowed"ç„¶åŽæ²¡æœ‰å›žå¤ï¼Ÿ](#why-do-i-see-model-is-not-allowed-and-then-no-reply)
  - [ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ°"Unknown model: minimax/MiniMax-M2.1"ï¼Ÿ](#why-do-i-see-unknown-model-minimaxminimaxm21)
  - [èƒ½å¦å°† MiniMax è®¾ä¸ºé»˜è®¤ï¼Œå¤æ‚ä»»åŠ¡ç”¨ OpenAIï¼Ÿ](#can-i-use-minimax-as-my-default-and-openai-for-complex-tasks)
  - [opus / sonnet / gpt æ˜¯å†…ç½®å¿«æ·æ–¹å¼å—ï¼Ÿ](#are-opus-sonnet-gpt-builtin-shortcuts)
  - [å¦‚ä½•å®šä¹‰/è¦†ç›–æ¨¡åž‹å¿«æ·æ–¹å¼ï¼ˆåˆ«åï¼‰ï¼Ÿ](#how-do-i-defineoverride-model-shortcuts-aliases)
  - [å¦‚ä½•æ·»åŠ å…¶ä»–æä¾›å•†ï¼ˆå¦‚ OpenRouter æˆ– Z.AIï¼‰çš„æ¨¡åž‹ï¼Ÿ](#how-do-i-add-models-from-other-providers-like-openrouter-or-zai)
- [æ¨¡åž‹æ•…éšœè½¬ç§»ä¸Ž"All models failed"](#model-failover-and-all-models-failed)
  - [æ•…éšœè½¬ç§»æ˜¯å¦‚ä½•å·¥ä½œçš„ï¼Ÿ](#how-does-failover-work)
  - [è¿™ä¸ªé”™è¯¯æ˜¯ä»€ä¹ˆæ„æ€ï¼Ÿ](#what-does-this-error-mean)
  - [`No credentials found for profile "anthropic:default"` çš„ä¿®å¤æ¸…å•](#fix-checklist-for-no-credentials-found-for-profile-anthropicdefault)
  - [ä¸ºä»€ä¹ˆè¿˜å°è¯•äº† Google Gemini å¹¶ä¸”å¤±è´¥äº†ï¼Ÿ](#why-did-it-also-try-google-gemini-and-fail)
- [è®¤è¯é…ç½®æ–‡ä»¶ï¼šæ¦‚å¿µå’Œç®¡ç†æ–¹å¼](#auth-profiles-what-they-are-and-how-to-manage-them)
  - [ä»€ä¹ˆæ˜¯è®¤è¯é…ç½®æ–‡ä»¶ï¼Ÿ](#what-is-an-auth-profile)
  - [å…¸åž‹çš„é…ç½®æ–‡ä»¶ ID æœ‰å“ªäº›ï¼Ÿ](#what-are-typical-profile-ids)
  - [å¯ä»¥æŽ§åˆ¶é¦–å…ˆå°è¯•å“ªä¸ªè®¤è¯é…ç½®æ–‡ä»¶å—ï¼Ÿ](#can-i-control-which-auth-profile-is-tried-first)
  - [OAuth ä¸Ž API å¯†é’¥ï¼šæœ‰ä»€ä¹ˆåŒºåˆ«ï¼Ÿ](#oauth-vs-api-key-whats-the-difference)
- [Gateway ç½‘å…³ï¼šç«¯å£ã€â€œå·²åœ¨è¿è¡Œâ€å’Œè¿œç¨‹æ¨¡å¼](#gateway-ports-already-running-and-remote-mode)
  - [Gateway ç½‘å…³ä½¿ç”¨ä»€ä¹ˆç«¯å£ï¼Ÿ](#what-port-does-the-gateway-use)
  - [ä¸ºä»€ä¹ˆ `krabkrab gateway status` æ˜¾ç¤º `Runtime: running` ä½† `RPC probe: failed`ï¼Ÿ](#why-does-krabkrab-gateway-status-say-runtime-running-but-rpc-probe-failed)
  - [ä¸ºä»€ä¹ˆ `krabkrab gateway status` æ˜¾ç¤º `Config (cli)` å’Œ `Config (service)` ä¸åŒï¼Ÿ](#why-does-krabkrab-gateway-status-show-config-cli-and-config-service-different)
  - ["another gateway instance is already listening"æ˜¯ä»€ä¹ˆæ„æ€ï¼Ÿ](#what-does-another-gateway-instance-is-already-listening-mean)
  - [å¦‚ä½•ä»¥è¿œç¨‹æ¨¡å¼è¿è¡Œ KrabKrabï¼ˆå®¢æˆ·ç«¯è¿žæŽ¥åˆ°å…¶ä»–ä½ç½®çš„ Gateway ç½‘å…³ï¼‰ï¼Ÿ](#how-do-i-run-krabkrab-in-remote-mode-client-connects-to-a-gateway-elsewhere)
  - [æŽ§åˆ¶ UI æ˜¾ç¤º"unauthorized"ï¼ˆæˆ–æŒç»­é‡è¿žï¼‰ï¼Œæ€Žä¹ˆåŠžï¼Ÿ](#the-control-ui-says-unauthorized-or-keeps-reconnecting-what-now)
  - [æˆ‘è®¾ç½®äº† `gateway.bind: "tailnet"` ä½†æ— æ³•ç»‘å®š / ä»€ä¹ˆéƒ½æ²¡ç›‘å¬](#i-set-gatewaybind-tailnet-but-it-cant-bind-nothing-listens)
  - [å¯ä»¥åœ¨åŒä¸€ä¸»æœºä¸Šè¿è¡Œå¤šä¸ª Gateway ç½‘å…³å—ï¼Ÿ](#can-i-run-multiple-gateways-on-the-same-host)
  - ["invalid handshake" / code 1008 æ˜¯ä»€ä¹ˆæ„æ€ï¼Ÿ](#what-does-invalid-handshake-code-1008-mean)
- [æ—¥å¿—ä¸Žè°ƒè¯•](#logging-and-debugging)
  - [æ—¥å¿—åœ¨å“ªé‡Œï¼Ÿ](#where-are-logs)
  - [å¦‚ä½•å¯åŠ¨/åœæ­¢/é‡å¯ Gateway ç½‘å…³æœåŠ¡ï¼Ÿ](#how-do-i-startstoprestart-the-gateway-service)
  - [æˆ‘åœ¨ Windows ä¸Šå…³é—­äº†ç»ˆç«¯â€”â€”å¦‚ä½•é‡å¯ KrabKrabï¼Ÿ](#i-closed-my-terminal-on-windows-how-do-i-restart-krabkrab)
  - [Gateway ç½‘å…³å·²å¯åŠ¨ä½†å›žå¤å§‹ç»ˆä¸åˆ°è¾¾ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆï¼Ÿ](#the-gateway-is-up-but-replies-never-arrive-what-should-i-check)
  - ["Disconnected from gateway: no reason"â€”â€”æ€Žä¹ˆåŠžï¼Ÿ](#disconnected-from-gateway-no-reason-what-now)
  - [Telegram setMyCommands å› ç½‘ç»œé”™è¯¯å¤±è´¥ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆï¼Ÿ](#telegram-setmycommands-fails-with-network-errors-what-should-i-check)
  - [TUI æ²¡æœ‰è¾“å‡ºï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆï¼Ÿ](#tui-shows-no-output-what-should-i-check)
  - [å¦‚ä½•å®Œå…¨åœæ­¢ç„¶åŽå¯åŠ¨ Gateway ç½‘å…³ï¼Ÿ](#how-do-i-completely-stop-then-start-the-gateway)
  - [é€šä¿—è§£é‡Šï¼š`krabkrab gateway restart` ä¸Ž `krabkrab gateway`](#eli5-krabkrab-gateway-restart-vs-krabkrab-gateway)
  - [å‡ºçŽ°æ•…éšœæ—¶èŽ·å–æ›´å¤šè¯¦æƒ…çš„æœ€å¿«æ–¹æ³•æ˜¯ä»€ä¹ˆï¼Ÿ](#whats-the-fastest-way-to-get-more-details-when-something-fails)
- [åª’ä½“ä¸Žé™„ä»¶](#media-attachments)
  - [æˆ‘çš„ Skills ç”Ÿæˆäº†å›¾ç‰‡/PDFï¼Œä½†ä»€ä¹ˆéƒ½æ²¡å‘é€](#my-skill-generated-an-imagepdf-but-nothing-was-sent)
- [å®‰å…¨ä¸Žè®¿é—®æŽ§åˆ¶](#security-and-access-control)
  - [å°† KrabKrab æš´éœ²ç»™å…¥ç«™ç§ä¿¡å®‰å…¨å—ï¼Ÿ](#is-it-safe-to-expose-krabkrab-to-inbound-dms)
  - [æç¤ºæ³¨å…¥åªå¯¹å…¬å¼€æœºå™¨äººæœ‰å½±å“å—ï¼Ÿ](#is-prompt-injection-only-a-concern-for-public-bots)
  - [æˆ‘çš„æœºå™¨äººåº”è¯¥æœ‰è‡ªå·±çš„é‚®ç®±ã€GitHub è´¦æˆ·æˆ–ç”µè¯å·ç å—ï¼Ÿ](#should-my-bot-have-its-own-email-github-account-or-phone-number)
  - [æˆ‘èƒ½è®©å®ƒè‡ªä¸»ç®¡ç†æˆ‘çš„çŸ­ä¿¡å—ï¼Ÿè¿™å®‰å…¨å—ï¼Ÿ](#can-i-give-it-autonomy-over-my-text-messages-and-is-that-safe)
  - [ä¸ªäººåŠ©ç†ä»»åŠ¡å¯ä»¥ä½¿ç”¨æ›´ä¾¿å®œçš„æ¨¡åž‹å—ï¼Ÿ](#can-i-use-cheaper-models-for-personal-assistant-tasks)
  - [æˆ‘åœ¨ Telegram ä¸­è¿è¡Œäº† `/start` ä½†æ²¡æ”¶åˆ°é…å¯¹ç ](#i-ran-start-in-telegram-but-didnt-get-a-pairing-code)
  - [WhatsAppï¼šä¼šç»™æˆ‘çš„è”ç³»äººå‘æ¶ˆæ¯å—ï¼Ÿé…å¯¹å¦‚ä½•å·¥ä½œï¼Ÿ](#whatsapp-will-it-message-my-contacts-how-does-pairing-work)
- [èŠå¤©å‘½ä»¤ã€ä¸­æ­¢ä»»åŠ¡å’Œâ€œåœä¸ä¸‹æ¥â€](#chat-commands-aborting-tasks-and-it-wont-stop)
  - [å¦‚ä½•é˜»æ­¢å†…éƒ¨ç³»ç»Ÿæ¶ˆæ¯æ˜¾ç¤ºåœ¨èŠå¤©ä¸­ï¼Ÿ](#how-do-i-stop-internal-system-messages-from-showing-in-chat)
  - [å¦‚ä½•åœæ­¢/å–æ¶ˆæ­£åœ¨è¿è¡Œçš„ä»»åŠ¡ï¼Ÿ](#how-do-i-stopcancel-a-running-task)
  - [å¦‚ä½•ä»Ž Telegram å‘é€ Discord æ¶ˆæ¯ï¼Ÿï¼ˆ"Cross-context messaging denied"ï¼‰](#how-do-i-send-a-discord-message-from-telegram-crosscontext-messaging-denied)
  - [ä¸ºä»€ä¹ˆæ„Ÿè§‰æœºå™¨äººâ€œå¿½ç•¥â€äº†å¿«é€Ÿè¿žå‘çš„æ¶ˆæ¯ï¼Ÿ](#why-does-it-feel-like-the-bot-ignores-rapidfire-messages)

## å‡ºé—®é¢˜åŽçš„æœ€åˆå…­åç§’

1. **å¿«é€ŸçŠ¶æ€ï¼ˆé¦–å…ˆæ£€æŸ¥ï¼‰**

   ```bash
   krabkrab status
   ```

   å¿«é€Ÿæœ¬åœ°æ‘˜è¦ï¼šæ“ä½œç³»ç»Ÿ + æ›´æ–°ã€Gateway ç½‘å…³/æœåŠ¡å¯è¾¾æ€§ã€æ™ºèƒ½ä½“/ä¼šè¯ã€æä¾›å•†é…ç½® + è¿è¡Œæ—¶é—®é¢˜ï¼ˆGateway ç½‘å…³å¯è¾¾æ—¶ï¼‰ã€‚

2. **å¯ç²˜è´´çš„æŠ¥å‘Šï¼ˆå¯å®‰å…¨åˆ†äº«ï¼‰**

   ```bash
   krabkrab status --all
   ```

   åªè¯»è¯Šæ–­ï¼Œé™„å¸¦æ—¥å¿—å°¾éƒ¨ï¼ˆä»¤ç‰Œå·²è„±æ•ï¼‰ã€‚

3. **å®ˆæŠ¤è¿›ç¨‹ + ç«¯å£çŠ¶æ€**

   ```bash
   krabkrab gateway status
   ```

   æ˜¾ç¤º supervisor è¿è¡ŒçŠ¶æ€ä¸Ž RPC å¯è¾¾æ€§ã€æŽ¢æµ‹ç›®æ ‡ URLï¼Œä»¥åŠæœåŠ¡å¯èƒ½ä½¿ç”¨çš„é…ç½®ã€‚

4. **æ·±åº¦æŽ¢æµ‹**

   ```bash
   krabkrab status --deep
   ```

   è¿è¡Œ Gateway ç½‘å…³å¥åº·æ£€æŸ¥ + æä¾›å•†æŽ¢æµ‹ï¼ˆéœ€è¦å¯è¾¾çš„ Gateway ç½‘å…³ï¼‰ã€‚å‚é˜…[å¥åº·æ£€æŸ¥](/gateway/health)ã€‚

5. **è·Ÿè¸ªæœ€æ–°æ—¥å¿—**

   ```bash
   krabkrab logs --follow
   ```

   å¦‚æžœ RPC ä¸å¯ç”¨ï¼Œå›žé€€åˆ°ï¼š

   ```bash
   tail -f "$(ls -t /tmp/krabkrab/krabkrab-*.log | head -1)"
   ```

   æ–‡ä»¶æ—¥å¿—ä¸ŽæœåŠ¡æ—¥å¿—æ˜¯åˆ†å¼€çš„ï¼›å‚é˜…[æ—¥å¿—](/logging)å’Œ[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)ã€‚

6. **è¿è¡Œ doctorï¼ˆä¿®å¤ï¼‰**

   ```bash
   krabkrab doctor
   ```

   ä¿®å¤/è¿ç§»é…ç½®/çŠ¶æ€ + è¿è¡Œå¥åº·æ£€æŸ¥ã€‚å‚é˜… [Doctor](/gateway/doctor)ã€‚

7. **Gateway ç½‘å…³å¿«ç…§**
   ```bash
   krabkrab health --json
   krabkrab health --verbose   # å‡ºé”™æ—¶æ˜¾ç¤ºç›®æ ‡ URL + é…ç½®è·¯å¾„
   ```
   å‘è¿è¡Œä¸­çš„ Gateway ç½‘å…³è¯·æ±‚å®Œæ•´å¿«ç…§ï¼ˆä»… WSï¼‰ã€‚å‚é˜…[å¥åº·æ£€æŸ¥](/gateway/health)ã€‚

## å¿«é€Ÿå¼€å§‹ä¸Žé¦–æ¬¡è¿è¡Œè®¾ç½®

### æˆ‘å¡ä½äº†ï¼Œæœ€å¿«çš„æŽ’éšœæ–¹æ³•æ˜¯ä»€ä¹ˆ

ä½¿ç”¨èƒ½**çœ‹åˆ°ä½ æœºå™¨**çš„æœ¬åœ° AI æ™ºèƒ½ä½“ã€‚è¿™æ¯”åœ¨ Discord ä¸Šæé—®æœ‰æ•ˆå¾—å¤šï¼Œå› ä¸ºå¤§å¤šæ•°â€œå¡ä½äº†â€çš„æƒ…å†µéƒ½æ˜¯**æœ¬åœ°é…ç½®æˆ–çŽ¯å¢ƒé—®é¢˜**ï¼Œè¿œç¨‹å¸®åŠ©è€…æ— æ³•æ£€æŸ¥ã€‚

- **Claude Code**ï¼šhttps://www.anthropic.com/claude-code/
- **OpenAI Codex**ï¼šhttps://openai.com/codex/

è¿™äº›å·¥å…·å¯ä»¥è¯»å–ä»“åº“ã€è¿è¡Œå‘½ä»¤ã€æ£€æŸ¥æ—¥å¿—ï¼Œå¹¶å¸®åŠ©ä¿®å¤ä½ çš„æœºå™¨çº§åˆ«è®¾ç½®ï¼ˆPATHã€æœåŠ¡ã€æƒé™ã€è®¤è¯æ–‡ä»¶ï¼‰ã€‚é€šè¿‡å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…æä¾›**å®Œæ•´æºä»£ç **ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git
```

è¿™ä¼šä»Ž **git checkout** å®‰è£… KrabKrabï¼Œè¿™æ ·æ™ºèƒ½ä½“å¯ä»¥è¯»å–ä»£ç  + æ–‡æ¡£ï¼Œå¹¶æŽ¨ç†ä½ æ­£åœ¨è¿è¡Œçš„ç¡®åˆ‡ç‰ˆæœ¬ã€‚ä½ å¯ä»¥éšæ—¶é€šè¿‡ä¸å¸¦ `--install-method git` é‡æ–°è¿è¡Œå®‰è£…ç¨‹åºåˆ‡å›žç¨³å®šç‰ˆã€‚

æç¤ºï¼šè¦æ±‚æ™ºèƒ½ä½“**è®¡åˆ’å¹¶ç›‘ç£**ä¿®å¤ï¼ˆé€æ­¥è¿›è¡Œï¼‰ï¼Œç„¶åŽåªæ‰§è¡Œå¿…è¦çš„å‘½ä»¤ã€‚è¿™æ ·æ”¹åŠ¨è¾ƒå°ï¼Œæ›´å®¹æ˜“å®¡æŸ¥ã€‚

å¦‚æžœä½ å‘çŽ°äº†çœŸæ­£çš„ bug æˆ–ä¿®å¤æ–¹æ¡ˆï¼Œè¯·æäº¤ GitHub issue æˆ–å‘é€ PRï¼š
https://github.com/krabkrab/krabkrab/issues
https://github.com/krabkrab/krabkrab/pulls

ä»Žä»¥ä¸‹å‘½ä»¤å¼€å§‹ï¼ˆåœ¨å¯»æ±‚å¸®åŠ©æ—¶åˆ†äº«è¾“å‡ºï¼‰ï¼š

```bash
krabkrab status
krabkrab models status
krabkrab doctor
```

å®ƒä»¬çš„ä½œç”¨ï¼š

- `krabkrab status`ï¼šGateway ç½‘å…³/æ™ºèƒ½ä½“å¥åº·çŠ¶å†µ + åŸºæœ¬é…ç½®çš„å¿«é€Ÿå¿«ç…§ã€‚
- `krabkrab models status`ï¼šæ£€æŸ¥æä¾›å•†è®¤è¯ + æ¨¡åž‹å¯ç”¨æ€§ã€‚
- `krabkrab doctor`ï¼šéªŒè¯å¹¶ä¿®å¤å¸¸è§çš„é…ç½®/çŠ¶æ€é—®é¢˜ã€‚

å…¶ä»–æœ‰ç”¨çš„ CLI æ£€æŸ¥ï¼š`krabkrab status --all`ã€`krabkrab logs --follow`ã€
`krabkrab gateway status`ã€`krabkrab health --verbose`ã€‚

å¿«é€Ÿè°ƒè¯•æµç¨‹ï¼š[å‡ºé—®é¢˜åŽçš„æœ€åˆå…­åç§’](#first-60-seconds-if-somethings-broken)ã€‚
å®‰è£…æ–‡æ¡£ï¼š[å®‰è£…](/install)ã€[å®‰è£…ç¨‹åºæ ‡å¿—](/install/installer)ã€[æ›´æ–°](/install/updating)ã€‚

### å®‰è£…å’Œè®¾ç½® KrabKrab çš„æŽ¨èæ–¹å¼æ˜¯ä»€ä¹ˆ

ä»“åº“æŽ¨èä»Žæºç è¿è¡Œå¹¶ä½¿ç”¨æ–°æ‰‹å¼•å¯¼å‘å¯¼ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash
krabkrab onboard --install-daemon
```

å‘å¯¼è¿˜å¯ä»¥è‡ªåŠ¨æž„å»º UI èµ„æºã€‚æ–°æ‰‹å¼•å¯¼åŽï¼Œé€šå¸¸åœ¨ç«¯å£ **18789** ä¸Šè¿è¡Œ Gateway ç½‘å…³ã€‚

ä»Žæºç å®‰è£…ï¼ˆè´¡çŒ®è€…/å¼€å‘è€…ï¼‰ï¼š

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
pnpm install
pnpm build
pnpm ui:build # é¦–æ¬¡è¿è¡Œæ—¶è‡ªåŠ¨å®‰è£… UI ä¾èµ–
krabkrab onboard
```

å¦‚æžœä½ è¿˜æ²¡æœ‰å…¨å±€å®‰è£…ï¼Œé€šè¿‡ `pnpm krabkrab onboard` è¿è¡Œã€‚

### æ–°æ‰‹å¼•å¯¼åŽå¦‚ä½•æ‰“å¼€ä»ªè¡¨æ¿

å‘å¯¼çŽ°åœ¨ä¼šåœ¨æ–°æ‰‹å¼•å¯¼å®ŒæˆåŽç«‹å³ä½¿ç”¨å¸¦ä»¤ç‰Œçš„ä»ªè¡¨æ¿ URL æ‰“å¼€æµè§ˆå™¨ï¼Œå¹¶åœ¨æ‘˜è¦ä¸­æ‰“å°å®Œæ•´é“¾æŽ¥ï¼ˆå¸¦ä»¤ç‰Œï¼‰ã€‚ä¿æŒè¯¥æ ‡ç­¾é¡µæ‰“å¼€ï¼›å¦‚æžœæ²¡æœ‰è‡ªåŠ¨å¯åŠ¨ï¼Œè¯·åœ¨åŒä¸€å°æœºå™¨ä¸Šå¤åˆ¶/ç²˜è´´æ‰“å°çš„ URLã€‚ä»¤ç‰Œä¿æŒåœ¨æœ¬åœ°ä¸»æœºä¸Šâ€”â€”ä¸ä¼šä»Žæµè§ˆå™¨èŽ·å–ä»»ä½•å†…å®¹ã€‚

### å¦‚ä½•åœ¨æœ¬åœ°å’Œè¿œç¨‹çŽ¯å¢ƒä¸­éªŒè¯ä»ªè¡¨æ¿ä»¤ç‰Œ

**æœ¬åœ°ï¼ˆåŒä¸€å°æœºå™¨ï¼‰ï¼š**

- æ‰“å¼€ `http://127.0.0.1:18789/`ã€‚
- å¦‚æžœè¦æ±‚è®¤è¯ï¼Œè¿è¡Œ `krabkrab dashboard` å¹¶ä½¿ç”¨å¸¦ä»¤ç‰Œçš„é“¾æŽ¥ï¼ˆ`?token=...`ï¼‰ã€‚
- ä»¤ç‰Œä¸Ž `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰çš„å€¼ç›¸åŒï¼ŒUI åœ¨é¦–æ¬¡åŠ è½½åŽä¼šå­˜å‚¨å®ƒã€‚

**éžæœ¬åœ°çŽ¯å¢ƒï¼š**

- **Tailscale Serve**ï¼ˆæŽ¨èï¼‰ï¼šä¿æŒç»‘å®š loopbackï¼Œè¿è¡Œ `krabkrab gateway --tailscale serve`ï¼Œæ‰“å¼€ `https://<magicdns>/`ã€‚å¦‚æžœ `gateway.auth.allowTailscale` ä¸º `true`ï¼Œèº«ä»½æ ‡å¤´æ»¡è¶³è®¤è¯è¦æ±‚ï¼ˆæ— éœ€ä»¤ç‰Œï¼‰ã€‚
- **Tailnet ç»‘å®š**ï¼šè¿è¡Œ `krabkrab gateway --bind tailnet --token "<token>"`ï¼Œæ‰“å¼€ `http://<tailscale-ip>:18789/`ï¼Œåœ¨ä»ªè¡¨æ¿è®¾ç½®ä¸­ç²˜è´´ä»¤ç‰Œã€‚
- **SSH éš§é“**ï¼š`ssh -N -L 18789:127.0.0.1:18789 user@host`ï¼Œç„¶åŽä»Ž `krabkrab dashboard` æ‰“å¼€ `http://127.0.0.1:18789/?token=...`ã€‚

å‚é˜…[ä»ªè¡¨æ¿](/web/dashboard)å’Œ [Web ç•Œé¢](/web)äº†è§£ç»‘å®šæ¨¡å¼å’Œè®¤è¯è¯¦æƒ…ã€‚

### æˆ‘éœ€è¦ä»€ä¹ˆè¿è¡Œæ—¶

Node **>= 22** æ˜¯å¿…éœ€çš„ã€‚æŽ¨èä½¿ç”¨ `pnpm`ã€‚**ä¸æŽ¨è**ä½¿ç”¨ Bun è¿è¡Œ Gateway ç½‘å…³ã€‚

### èƒ½åœ¨ Raspberry Pi ä¸Šè¿è¡Œå—

å¯ä»¥ã€‚Gateway ç½‘å…³æ˜¯è½»é‡çº§çš„â€”â€”æ–‡æ¡£åˆ—å‡º **512MB-1GB RAM**ã€**1 æ ¸**å’Œçº¦ **500MB** ç£ç›˜ç©ºé—´è¶³å¤Ÿä¸ªäººä½¿ç”¨ï¼Œå¹¶æŒ‡å‡º **Raspberry Pi 4 å¯ä»¥è¿è¡Œ**ã€‚

å¦‚æžœä½ éœ€è¦é¢å¤–çš„ä½™é‡ï¼ˆæ—¥å¿—ã€åª’ä½“ã€å…¶ä»–æœåŠ¡ï¼‰ï¼Œ**æŽ¨è 2GB**ï¼Œä½†è¿™ä¸æ˜¯ç¡¬æ€§æœ€ä½Žè¦æ±‚ã€‚

æç¤ºï¼šå°åž‹ Pi/VPS å¯ä»¥æ‰˜ç®¡ Gateway ç½‘å…³ï¼Œä½ å¯ä»¥åœ¨ç¬”è®°æœ¬/æ‰‹æœºä¸Šé…å¯¹**èŠ‚ç‚¹**ä»¥èŽ·å–æœ¬åœ°å±å¹•/æ‘„åƒå¤´/ç”»å¸ƒæˆ–å‘½ä»¤æ‰§è¡Œèƒ½åŠ›ã€‚å‚é˜…[èŠ‚ç‚¹](/nodes)ã€‚

### Raspberry Pi å®‰è£…æœ‰ä»€ä¹ˆå»ºè®®

ç®€çŸ­å›žç­”ï¼šå¯ä»¥è¿è¡Œï¼Œä½†é¢„æœŸä¼šæœ‰ä¸€äº›ç²—ç³™ä¹‹å¤„ã€‚

- ä½¿ç”¨ **64 ä½**æ“ä½œç³»ç»Ÿå¹¶ä¿æŒ Node >= 22ã€‚
- ä¼˜å…ˆé€‰æ‹©**å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…**ï¼Œä»¥ä¾¿æŸ¥çœ‹æ—¥å¿—å’Œå¿«é€Ÿæ›´æ–°ã€‚
- å…ˆä¸å¯ç”¨æ¸ é“/Skillsï¼Œç„¶åŽé€ä¸ªæ·»åŠ ã€‚
- å¦‚æžœé‡åˆ°å¥‡æ€ªçš„äºŒè¿›åˆ¶é—®é¢˜ï¼Œé€šå¸¸æ˜¯ **ARM å…¼å®¹æ€§**é—®é¢˜ã€‚

æ–‡æ¡£ï¼š[Linux](/platforms/linux)ã€[å®‰è£…](/install)ã€‚

### å¡åœ¨ wake up my friend / æ–°æ‰‹å¼•å¯¼æ— æ³•å¯åŠ¨ï¼Œæ€Žä¹ˆåŠž

è¯¥ç•Œé¢ä¾èµ–äºŽ Gateway ç½‘å…³å¯è¾¾ä¸”å·²è®¤è¯ã€‚TUI ä¹Ÿä¼šåœ¨é¦–æ¬¡å¯åŠ¨æ—¶è‡ªåŠ¨å‘é€"Wake up, my friend!"ã€‚å¦‚æžœä½ çœ‹åˆ°è¯¥è¡Œä½†**æ²¡æœ‰å›žå¤**ä¸”ä»¤ç‰Œä¿æŒä¸º 0ï¼Œè¯´æ˜Žæ™ºèƒ½ä½“ä»Žæœªè¿è¡Œã€‚

1. é‡å¯ Gateway ç½‘å…³ï¼š

```bash
krabkrab gateway restart
```

2. æ£€æŸ¥çŠ¶æ€å’Œè®¤è¯ï¼š

```bash
krabkrab status
krabkrab models status
krabkrab logs --follow
```

3. å¦‚æžœä»ç„¶æŒ‚èµ·ï¼Œè¿è¡Œï¼š

```bash
krabkrab doctor
```

å¦‚æžœ Gateway ç½‘å…³åœ¨è¿œç¨‹ï¼Œç¡®ä¿éš§é“/Tailscale è¿žæŽ¥æ­£å¸¸ï¼Œä¸” UI æŒ‡å‘æ­£ç¡®çš„ Gateway ç½‘å…³ã€‚å‚é˜…[è¿œç¨‹è®¿é—®](/gateway/remote)ã€‚

### èƒ½å¦å°†æˆ‘çš„è®¾ç½®è¿ç§»åˆ°æ–°æœºå™¨ï¼ˆMac miniï¼‰è€Œä¸é‡æ–°è¿›è¡Œæ–°æ‰‹å¼•å¯¼

å¯ä»¥ã€‚å¤åˆ¶**çŠ¶æ€ç›®å½•**å’Œ**å·¥ä½œåŒº**ï¼Œç„¶åŽè¿è¡Œä¸€æ¬¡ Doctorã€‚åªè¦ä½ åŒæ—¶å¤åˆ¶**ä¸¤ä¸ª**ä½ç½®ï¼Œå°±èƒ½ä¿æŒä½ çš„æœºå™¨äººâ€œå®Œå…¨ä¸€æ ·â€ï¼ˆè®°å¿†ã€ä¼šè¯åŽ†å²ã€è®¤è¯å’Œæ¸ é“çŠ¶æ€ï¼‰ï¼š

1. åœ¨æ–°æœºå™¨ä¸Šå®‰è£… KrabKrabã€‚
2. ä»Žæ—§æœºå™¨å¤åˆ¶ `$krabkrab_STATE_DIR`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab`ï¼‰ã€‚
3. å¤åˆ¶ä½ çš„å·¥ä½œåŒºï¼ˆé»˜è®¤ï¼š`~/.krabkrab/workspace`ï¼‰ã€‚
4. è¿è¡Œ `krabkrab doctor` å¹¶é‡å¯ Gateway ç½‘å…³æœåŠ¡ã€‚

è¿™ä¼šä¿ç•™é…ç½®ã€è®¤è¯é…ç½®æ–‡ä»¶ã€WhatsApp å‡­æ®ã€ä¼šè¯å’Œè®°å¿†ã€‚å¦‚æžœä½ å¤„äºŽè¿œç¨‹æ¨¡å¼ï¼Œè¯·è®°ä½ Gateway ç½‘å…³ä¸»æœºæ‹¥æœ‰ä¼šè¯å­˜å‚¨å’Œå·¥ä½œåŒºã€‚

**é‡è¦ï¼š** å¦‚æžœä½ åªå°†å·¥ä½œåŒºæäº¤/æŽ¨é€åˆ° GitHubï¼Œä½ åªå¤‡ä»½äº†**è®°å¿† + å¼•å¯¼æ–‡ä»¶**ï¼Œä½†**ä¸åŒ…æ‹¬**ä¼šè¯åŽ†å²æˆ–è®¤è¯ã€‚å®ƒä»¬ä½äºŽ `~/.krabkrab/` ä¸‹ï¼ˆä¾‹å¦‚ `~/.krabkrab/agents/<agentId>/sessions/`ï¼‰ã€‚

ç›¸å…³ï¼š[è¿ç§»](/install/migrating)ã€[ç£ç›˜ä¸Šçš„æ–‡ä»¶ä½ç½®](/help/faq#where-does-krabkrab-store-its-data)ã€
[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace)ã€[Doctor](/gateway/doctor)ã€
[è¿œç¨‹æ¨¡å¼](/gateway/remote)ã€‚

### åœ¨å“ªé‡ŒæŸ¥çœ‹æœ€æ–°ç‰ˆæœ¬çš„æ›´æ–°å†…å®¹

æŸ¥çœ‹ GitHub å˜æ›´æ—¥å¿—ï¼š
https://github.com/krabkrab/krabkrab/blob/main/CHANGELOG.md

æœ€æ–°æ¡ç›®åœ¨é¡¶éƒ¨ã€‚å¦‚æžœé¡¶éƒ¨éƒ¨åˆ†æ ‡è®°ä¸º **Unreleased**ï¼Œåˆ™ä¸‹ä¸€ä¸ªå¸¦æ—¥æœŸçš„éƒ¨åˆ†æ˜¯æœ€æ–°å‘å¸ƒç‰ˆæœ¬ã€‚æ¡ç›®æŒ‰**äº®ç‚¹**ã€**å˜æ›´**å’Œ**ä¿®å¤**åˆ†ç»„ï¼ˆéœ€è¦æ—¶è¿˜æœ‰æ–‡æ¡£/å…¶ä»–éƒ¨åˆ†ï¼‰ã€‚

### æ— æ³•è®¿é—® docs.krabkrab.aiï¼ˆSSL é”™è¯¯ï¼‰ï¼Œæ€Žä¹ˆåŠž

ä¸€äº› Comcast/Xfinity è¿žæŽ¥é€šè¿‡ Xfinity Advanced Security é”™è¯¯åœ°æ‹¦æˆªäº† `docs.krabkrab.ai`ã€‚ç¦ç”¨è¯¥åŠŸèƒ½æˆ–å°† `docs.krabkrab.ai` åŠ å…¥ç™½åå•ï¼Œç„¶åŽé‡è¯•ã€‚æ›´å¤šè¯¦æƒ…ï¼š[æ•…éšœæŽ’é™¤](/help/troubleshooting#docskrabkrabai-shows-an-ssl-error-comcastxfinity)ã€‚
è¯·å¸®åŠ©æˆ‘ä»¬åœ¨æ­¤å¤„æŠ¥å‘Šä»¥è§£é™¤å°é”ï¼šhttps://spa.xfinity.com/check_url_statusã€‚

å¦‚æžœä»ç„¶æ— æ³•è®¿é—®è¯¥ç½‘ç«™ï¼Œæ–‡æ¡£åœ¨ GitHub ä¸Šæœ‰é•œåƒï¼š
https://github.com/krabkrab/krabkrab/tree/main/docs

### stable å’Œ beta æœ‰ä»€ä¹ˆåŒºåˆ«

**Stable** å’Œ **beta** æ˜¯ **npm dist-tags**ï¼Œä¸æ˜¯ç‹¬ç«‹çš„ä»£ç åˆ†æ”¯ï¼š

- `latest` = stable
- `beta` = ç”¨äºŽæµ‹è¯•çš„æ—©æœŸæž„å»º

æˆ‘ä»¬å°†æž„å»ºå‘å¸ƒåˆ° **beta**ï¼Œæµ‹è¯•åŽï¼Œä¸€æ—¦æž„å»ºç¨³å®šï¼Œå°±ä¼š**å°†åŒä¸€ç‰ˆæœ¬æå‡ä¸º `latest`**ã€‚è¿™å°±æ˜¯ä¸ºä»€ä¹ˆ beta å’Œ stable å¯ä»¥æŒ‡å‘**ç›¸åŒç‰ˆæœ¬**ã€‚

æŸ¥çœ‹å˜æ›´ï¼š
https://github.com/krabkrab/krabkrab/blob/main/CHANGELOG.md

### å¦‚ä½•å®‰è£… beta ç‰ˆæœ¬ï¼Œbeta å’Œ dev æœ‰ä»€ä¹ˆåŒºåˆ«

**Beta** æ˜¯ npm dist-tag `beta`ï¼ˆå¯èƒ½ä¸Ž `latest` ç›¸åŒï¼‰ã€‚
**Dev** æ˜¯ `main` çš„æ»šåŠ¨å¤´éƒ¨ï¼ˆgitï¼‰ï¼›å‘å¸ƒæ—¶ä½¿ç”¨ npm dist-tag `dev`ã€‚

ä¸€è¡Œå‘½ä»¤ï¼ˆmacOS/Linuxï¼‰ï¼š

```bash
curl -fsSL --proto '=https' --tlsv1.2 https://krabkrab.ai/install.sh | bash -s -- --beta
```

```bash
curl -fsSL --proto '=https' --tlsv1.2 https://krabkrab.ai/install.sh | bash -s -- --install-method git
```

Windows å®‰è£…ç¨‹åºï¼ˆPowerShellï¼‰ï¼š
https://krabkrab.ai/install.ps1

æ›´å¤šè¯¦æƒ…ï¼š[å¼€å‘æ¸ é“](/install/development-channels)å’Œ[å®‰è£…ç¨‹åºæ ‡å¿—](/install/installer)ã€‚

### å®‰è£…å’Œæ–°æ‰‹å¼•å¯¼é€šå¸¸éœ€è¦å¤šé•¿æ—¶é—´

å¤§è‡´æŒ‡å—ï¼š

- **å®‰è£…ï¼š** 2-5 åˆ†é’Ÿ
- **æ–°æ‰‹å¼•å¯¼ï¼š** 5-15 åˆ†é’Ÿï¼Œå–å†³äºŽé…ç½®å¤šå°‘æ¸ é“/æ¨¡åž‹

å¦‚æžœæŒ‚èµ·ï¼Œè¯·å‚é˜…[å®‰è£…ç¨‹åºå¡ä½](/help/faq#installer-stuck-how-do-i-get-more-feedback)å’Œ[æˆ‘å¡ä½äº†](/help/faq#im-stuck--whats-the-fastest-way-to-get-unstuck)ä¸­çš„å¿«é€Ÿè°ƒè¯•æµç¨‹ã€‚

### å¦‚ä½•è¯•ç”¨æœ€æ–°ä»£ç 

ä¸¤ä¸ªé€‰é¡¹ï¼š

1. **Dev æ¸ é“ï¼ˆgit checkoutï¼‰ï¼š**

```bash
krabkrab update --channel dev
```

è¿™ä¼šåˆ‡æ¢åˆ° `main` åˆ†æ”¯å¹¶ä»Žæºç æ›´æ–°ã€‚

2. **å¯ç¼–è¾‘å®‰è£…ï¼ˆä»Žå®‰è£…ç¨‹åºç½‘ç«™ï¼‰ï¼š**

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git
```

è¿™ä¼šç»™ä½ ä¸€ä¸ªå¯ç¼–è¾‘çš„æœ¬åœ°ä»“åº“ï¼Œç„¶åŽé€šè¿‡ git æ›´æ–°ã€‚

å¦‚æžœä½ æ›´å–œæ¬¢æ‰‹åŠ¨å…‹éš†ï¼Œä½¿ç”¨ï¼š

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
pnpm install
pnpm build
```

æ–‡æ¡£ï¼š[æ›´æ–°](/cli/update)ã€[å¼€å‘æ¸ é“](/install/development-channels)ã€
[å®‰è£…](/install)ã€‚

### å®‰è£…ç¨‹åºå¡ä½äº†ï¼Ÿå¦‚ä½•èŽ·å–æ›´å¤šåé¦ˆ

ä½¿ç”¨**è¯¦ç»†è¾“å‡º**é‡æ–°è¿è¡Œå®‰è£…ç¨‹åºï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --verbose
```

å¸¦è¯¦ç»†è¾“å‡ºçš„ Beta å®‰è£…ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --beta --verbose
```

å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…ï¼š

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git --verbose
```

æ›´å¤šé€‰é¡¹ï¼š[å®‰è£…ç¨‹åºæ ‡å¿—](/install/installer)ã€‚

### Windows å®‰è£…æç¤ºæ‰¾ä¸åˆ° git æˆ–æ— æ³•è¯†åˆ« krabkrab

ä¸¤ä¸ªå¸¸è§çš„ Windows é—®é¢˜ï¼š

**1) npm error spawn git / git not found**

- å®‰è£… **Git for Windows** å¹¶ç¡®ä¿ `git` åœ¨ä½ çš„ PATH ä¸­ã€‚
- å…³é—­å¹¶é‡æ–°æ‰“å¼€ PowerShellï¼Œç„¶åŽé‡æ–°è¿è¡Œå®‰è£…ç¨‹åºã€‚

**2) krabkrab is not recognizedï¼ˆå®‰è£…åŽï¼‰**

- ä½ çš„ npm å…¨å±€ bin æ–‡ä»¶å¤¹ä¸åœ¨ PATH ä¸­ã€‚
- æ£€æŸ¥è·¯å¾„ï¼š
  ```powershell
  npm config get prefix
  ```
- ç¡®ä¿ `<prefix>\\bin` åœ¨ PATH ä¸­ï¼ˆåœ¨å¤§å¤šæ•°ç³»ç»Ÿä¸Šæ˜¯ `%AppData%\\npm`ï¼‰ã€‚
- æ›´æ–° PATH åŽå…³é—­å¹¶é‡æ–°æ‰“å¼€ PowerShellã€‚

å¦‚æžœä½ æƒ³è¦æœ€é¡ºç•…çš„ Windows è®¾ç½®ï¼Œè¯·ä½¿ç”¨ **WSL2** è€Œä¸æ˜¯åŽŸç”Ÿ Windowsã€‚
æ–‡æ¡£ï¼š[Windows](/platforms/windows)ã€‚

### æ–‡æ¡£æ²¡æœ‰è§£ç­”æˆ‘çš„é—®é¢˜â€”â€”å¦‚ä½•èŽ·å¾—æ›´å¥½çš„ç­”æ¡ˆ

ä½¿ç”¨**å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…**ï¼Œè¿™æ ·ä½ åœ¨æœ¬åœ°æ‹¥æœ‰å®Œæ•´çš„æºç å’Œæ–‡æ¡£ï¼Œç„¶åŽä»Žè¯¥æ–‡ä»¶å¤¹å‘ä½ çš„æœºå™¨äººï¼ˆæˆ– Claude/Codexï¼‰æé—®ï¼Œè¿™æ ·å®ƒå¯ä»¥è¯»å–ä»“åº“å¹¶ç²¾ç¡®å›žç­”ã€‚

```bash
curl -fsSL https://krabkrab.ai/install.sh | bash -s -- --install-method git
```

æ›´å¤šè¯¦æƒ…ï¼š[å®‰è£…](/install)å’Œ[å®‰è£…ç¨‹åºæ ‡å¿—](/install/installer)ã€‚

### å¦‚ä½•åœ¨ Linux ä¸Šå®‰è£… KrabKrab

ç®€çŸ­å›žç­”ï¼šæŒ‰ç…§ Linux æŒ‡å—æ“ä½œï¼Œç„¶åŽè¿è¡Œæ–°æ‰‹å¼•å¯¼å‘å¯¼ã€‚

- Linux å¿«é€Ÿè·¯å¾„ + æœåŠ¡å®‰è£…ï¼š[Linux](/platforms/linux)ã€‚
- å®Œæ•´æŒ‡å—ï¼š[å…¥é—¨](/start/getting-started)ã€‚
- å®‰è£…å’Œæ›´æ–°ï¼š[å®‰è£…ä¸Žæ›´æ–°](/install/updating)ã€‚

### å¦‚ä½•åœ¨ VPS ä¸Šå®‰è£… KrabKrab

ä»»ä½• Linux VPS éƒ½å¯ä»¥ã€‚åœ¨æœåŠ¡å™¨ä¸Šå®‰è£…ï¼Œç„¶åŽä½¿ç”¨ SSH/Tailscale è®¿é—® Gateway ç½‘å…³ã€‚

æŒ‡å—ï¼š[exe.dev](/install/exe-dev)ã€[Hetzner](/install/hetzner)ã€[Fly.io](/install/fly)ã€‚
è¿œç¨‹è®¿é—®ï¼š[Gateway ç½‘å…³è¿œç¨‹](/gateway/remote)ã€‚

### äº‘/VPS å®‰è£…æŒ‡å—åœ¨å“ªé‡Œ

æˆ‘ä»¬ç»´æŠ¤äº†ä¸€ä¸ª**æ‰˜ç®¡ä¸­å¿ƒ**ï¼Œæ¶µç›–å¸¸è§æä¾›å•†ã€‚é€‰æ‹©ä¸€ä¸ªå¹¶æŒ‰æŒ‡å—æ“ä½œï¼š

- [VPS æ‰˜ç®¡](/vps)ï¼ˆæ‰€æœ‰æä¾›å•†æ±‡æ€»ï¼‰
- [Fly.io](/install/fly)
- [Hetzner](/install/hetzner)
- [exe.dev](/install/exe-dev)

åœ¨äº‘ç«¯çš„å·¥ä½œæ–¹å¼ï¼š**Gateway ç½‘å…³è¿è¡Œåœ¨æœåŠ¡å™¨ä¸Š**ï¼Œä½ é€šè¿‡æŽ§åˆ¶ UIï¼ˆæˆ– Tailscale/SSHï¼‰ä»Žç¬”è®°æœ¬/æ‰‹æœºè®¿é—®ã€‚ä½ çš„çŠ¶æ€ + å·¥ä½œåŒºä½äºŽæœåŠ¡å™¨ä¸Šï¼Œå› æ­¤å°†ä¸»æœºè§†ä¸ºæ•°æ®æ¥æºå¹¶åšå¥½å¤‡ä»½ã€‚

ä½ å¯ä»¥å°†**èŠ‚ç‚¹**ï¼ˆMac/iOS/Android/æ— å¤´ï¼‰é…å¯¹åˆ°äº‘ç«¯ Gateway ç½‘å…³ï¼Œä»¥è®¿é—®æœ¬åœ°å±å¹•/æ‘„åƒå¤´/ç”»å¸ƒæˆ–åœ¨ç¬”è®°æœ¬ä¸Šæ‰§è¡Œå‘½ä»¤ï¼ŒåŒæ—¶ Gateway ç½‘å…³ä¿æŒåœ¨äº‘ç«¯ã€‚

ä¸­å¿ƒï¼š[å¹³å°](/platforms)ã€‚è¿œç¨‹è®¿é—®ï¼š[Gateway ç½‘å…³è¿œç¨‹](/gateway/remote)ã€‚
èŠ‚ç‚¹ï¼š[èŠ‚ç‚¹](/nodes)ã€[èŠ‚ç‚¹ CLI](/cli/nodes)ã€‚

### å¯ä»¥è®© KrabKrab è‡ªè¡Œæ›´æ–°å—

ç®€çŸ­å›žç­”ï¼š**å¯ä»¥ï¼Œä½†ä¸æŽ¨è**ã€‚æ›´æ–°æµç¨‹å¯èƒ½é‡å¯ Gateway ç½‘å…³ï¼ˆè¿™ä¼šä¸­æ–­æ´»è·ƒä¼šè¯ï¼‰ï¼Œå¯èƒ½éœ€è¦å¹²å‡€çš„ git checkoutï¼Œå¹¶ä¸”å¯èƒ½æç¤ºç¡®è®¤ã€‚æ›´å®‰å…¨çš„åšæ³•ï¼šä½œä¸ºè¿ç»´äººå‘˜ä»Ž shell è¿è¡Œæ›´æ–°ã€‚

ä½¿ç”¨ CLIï¼š

```bash
krabkrab update
krabkrab update status
krabkrab update --channel stable|beta|dev
krabkrab update --tag <dist-tag|version>
krabkrab update --no-restart
```

å¦‚æžœå¿…é¡»ä»Žæ™ºèƒ½ä½“è‡ªåŠ¨åŒ–ï¼š

```bash
krabkrab update --yes --no-restart
krabkrab gateway restart
```

æ–‡æ¡£ï¼š[æ›´æ–°](/cli/update)ã€[æ›´æ–°æŒ‡å—](/install/updating)ã€‚

### æ–°æ‰‹å¼•å¯¼å‘å¯¼å…·ä½“åšäº†ä»€ä¹ˆ

`krabkrab onboard` æ˜¯æŽ¨èçš„è®¾ç½®è·¯å¾„ã€‚åœ¨**æœ¬åœ°æ¨¡å¼**ä¸‹ï¼Œå®ƒå¼•å¯¼ä½ å®Œæˆï¼š

- **æ¨¡åž‹/è®¤è¯è®¾ç½®**ï¼ˆæŽ¨èä½¿ç”¨ Anthropic **setup-token** è¿›è¡Œ Claude è®¢é˜…ï¼Œæ”¯æŒ OpenAI Codex OAuthï¼ŒAPI å¯†é’¥å¯é€‰ï¼Œæ”¯æŒ LM Studio æœ¬åœ°æ¨¡åž‹ï¼‰
- **å·¥ä½œåŒº**ä½ç½® + å¼•å¯¼æ–‡ä»¶
- **Gateway ç½‘å…³è®¾ç½®**ï¼ˆç»‘å®š/ç«¯å£/è®¤è¯/tailscaleï¼‰
- **æ¸ é“**ï¼ˆWhatsAppã€Telegramã€Discordã€Mattermostï¼ˆæ’ä»¶ï¼‰ã€Signalã€iMessageï¼‰
- **å®ˆæŠ¤è¿›ç¨‹å®‰è£…**ï¼ˆmacOS ä¸Šçš„ LaunchAgentï¼›Linux/WSL2 ä¸Šçš„ systemd ç”¨æˆ·å•å…ƒï¼‰
- **å¥åº·æ£€æŸ¥**å’Œ**Skills**é€‰æ‹©

å¦‚æžœä½ é…ç½®çš„æ¨¡åž‹æœªçŸ¥æˆ–ç¼ºå°‘è®¤è¯ï¼Œå®ƒè¿˜ä¼šå‘å‡ºè­¦å‘Šã€‚

### è¿è¡Œ KrabKrab éœ€è¦ Claude æˆ– OpenAI è®¢é˜…å—

ä¸éœ€è¦ã€‚ä½ å¯ä»¥ä½¿ç”¨ **API å¯†é’¥**ï¼ˆAnthropic/OpenAI/å…¶ä»–ï¼‰æˆ–**çº¯æœ¬åœ°æ¨¡åž‹**è¿è¡Œ KrabKrabï¼Œè¿™æ ·ä½ çš„æ•°æ®ç•™åœ¨ä½ çš„è®¾å¤‡ä¸Šã€‚è®¢é˜…ï¼ˆClaude Pro/Max æˆ– OpenAI Codexï¼‰æ˜¯è¿™äº›æä¾›å•†çš„å¯é€‰è®¤è¯æ–¹å¼ã€‚

æ–‡æ¡£ï¼š[Anthropic](/providers/anthropic)ã€[OpenAI](/providers/openai)ã€
[æœ¬åœ°æ¨¡åž‹](/gateway/local-models)ã€[æ¨¡åž‹](/concepts/models)ã€‚

### èƒ½å¦ä½¿ç”¨ Claude Max è®¢é˜…è€Œä¸éœ€è¦ API å¯†é’¥

å¯ä»¥ã€‚ä½ å¯ä»¥ä½¿ç”¨ **setup-token** ä»£æ›¿ API å¯†é’¥è¿›è¡Œè®¤è¯ã€‚è¿™æ˜¯è®¢é˜…è·¯å¾„ã€‚

Claude Pro/Max è®¢é˜…**ä¸åŒ…å« API å¯†é’¥**ï¼Œå› æ­¤è¿™æ˜¯è®¢é˜…è´¦æˆ·çš„æ­£ç¡®æ–¹å¼ã€‚é‡è¦æç¤ºï¼šä½ å¿…é¡»å‘ Anthropic ç¡®è®¤æ­¤ç”¨æ³•æ˜¯å¦ç¬¦åˆå…¶è®¢é˜…æ”¿ç­–å’Œæ¡æ¬¾ã€‚å¦‚æžœä½ æƒ³è¦æœ€æ˜Žç¡®ã€å—æ”¯æŒçš„æ–¹å¼ï¼Œè¯·ä½¿ç”¨ Anthropic API å¯†é’¥ã€‚

### Anthropic setup-token è®¤è¯å¦‚ä½•å·¥ä½œ

`claude setup-token` é€šè¿‡ Claude Code CLI ç”Ÿæˆä¸€ä¸ª**ä»¤ç‰Œå­—ç¬¦ä¸²**ï¼ˆåœ¨ Web æŽ§åˆ¶å°ä¸­ä¸å¯ç”¨ï¼‰ã€‚ä½ å¯ä»¥åœ¨**ä»»ä½•æœºå™¨**ä¸Šè¿è¡Œå®ƒã€‚åœ¨å‘å¯¼ä¸­é€‰æ‹© **Anthropic token (paste setup-token)** æˆ–ä½¿ç”¨ `krabkrab models auth paste-token --provider anthropic` ç²˜è´´ã€‚ä»¤ç‰Œä½œä¸º **anthropic** æä¾›å•†çš„è®¤è¯é…ç½®æ–‡ä»¶å­˜å‚¨ï¼Œåƒ API å¯†é’¥ä¸€æ ·ä½¿ç”¨ï¼ˆæ— è‡ªåŠ¨åˆ·æ–°ï¼‰ã€‚æ›´å¤šè¯¦æƒ…ï¼š[OAuth](/concepts/oauth)ã€‚

### åœ¨å“ªé‡ŒèŽ·å– Anthropic setup-token

å®ƒ**ä¸åœ¨** Anthropic Console ä¸­ã€‚setup-token ç”± **Claude Code CLI** åœ¨**ä»»ä½•æœºå™¨**ä¸Šç”Ÿæˆï¼š

```bash
claude setup-token
```

å¤åˆ¶å®ƒæ‰“å°çš„ä»¤ç‰Œï¼Œç„¶åŽåœ¨å‘å¯¼ä¸­é€‰æ‹© **Anthropic token (paste setup-token)**ã€‚å¦‚æžœä½ æƒ³åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè¿è¡Œï¼Œä½¿ç”¨ `krabkrab models auth setup-token --provider anthropic`ã€‚å¦‚æžœä½ åœ¨å…¶ä»–åœ°æ–¹è¿è¡Œäº† `claude setup-token`ï¼Œåœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šä½¿ç”¨ `krabkrab models auth paste-token --provider anthropic` ç²˜è´´ã€‚å‚é˜… [Anthropic](/providers/anthropic)ã€‚

### æ˜¯å¦æ”¯æŒ Claude è®¢é˜…è®¤è¯ï¼ˆClaude Pro/Maxï¼‰

æ˜¯çš„â€”â€”é€šè¿‡ **setup-token**ã€‚KrabKrab ä¸å†å¤ç”¨ Claude Code CLI OAuth ä»¤ç‰Œï¼›è¯·ä½¿ç”¨ setup-token æˆ– Anthropic API å¯†é’¥ã€‚åœ¨ä»»ä½•åœ°æ–¹ç”Ÿæˆä»¤ç‰Œå¹¶åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šç²˜è´´ã€‚å‚é˜… [Anthropic](/providers/anthropic) å’Œ [OAuth](/concepts/oauth)ã€‚

æ³¨æ„ï¼šClaude è®¢é˜…è®¿é—®å— Anthropic æ¡æ¬¾çº¦æŸã€‚å¯¹äºŽç”Ÿäº§æˆ–å¤šç”¨æˆ·å·¥ä½œè´Ÿè½½ï¼ŒAPI å¯†é’¥é€šå¸¸æ˜¯æ›´å®‰å…¨çš„é€‰æ‹©ã€‚

### ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ° HTTP 429 rate_limit_errorï¼ˆæ¥è‡ª Anthropicï¼‰

è¿™æ„å‘³ç€ä½ å½“å‰çª—å£çš„ **Anthropic é…é¢/é€ŸçŽ‡é™åˆ¶**å·²è€—å°½ã€‚å¦‚æžœä½ ä½¿ç”¨ **Claude è®¢é˜…**ï¼ˆsetup-token æˆ– Claude Code OAuthï¼‰ï¼Œè¯·ç­‰å¾…çª—å£é‡ç½®æˆ–å‡çº§ä½ çš„è®¡åˆ’ã€‚å¦‚æžœä½ ä½¿ç”¨ **Anthropic API å¯†é’¥**ï¼Œè¯·åœ¨ Anthropic Console ä¸­æ£€æŸ¥ä½¿ç”¨é‡/è®¡è´¹å¹¶æ ¹æ®éœ€è¦æé«˜é™åˆ¶ã€‚

æç¤ºï¼šè®¾ç½®ä¸€ä¸ª**å¤‡ç”¨æ¨¡åž‹**ï¼Œè¿™æ · KrabKrab åœ¨æŸä¸ªæä¾›å•†è¢«é™é€Ÿæ—¶ä»èƒ½ç»§ç»­å›žå¤ã€‚
å‚é˜…[æ¨¡åž‹](/cli/models)å’Œ [OAuth](/concepts/oauth)ã€‚

### æ”¯æŒ AWS Bedrock å—

æ˜¯çš„â€”â€”é€šè¿‡ pi-ai çš„ **Amazon Bedrock (Converse)** æä¾›å•†è¿›è¡Œ**æ‰‹åŠ¨é…ç½®**ã€‚ä½ å¿…é¡»åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šæä¾› AWS å‡­æ®/åŒºåŸŸï¼Œå¹¶åœ¨æ¨¡åž‹é…ç½®ä¸­æ·»åŠ  Bedrock æä¾›å•†æ¡ç›®ã€‚å‚é˜… [Amazon Bedrock](/providers/bedrock) å’Œ[æ¨¡åž‹æä¾›å•†](/providers/models)ã€‚å¦‚æžœä½ æ›´å–œæ¬¢æ‰˜ç®¡å¯†é’¥æµç¨‹ï¼Œåœ¨ Bedrock å‰é¢ä½¿ç”¨å…¼å®¹ OpenAI çš„ä»£ç†ä»ç„¶æ˜¯æœ‰æ•ˆé€‰é¡¹ã€‚

### Codex è®¤è¯å¦‚ä½•å·¥ä½œ

KrabKrab é€šè¿‡ OAuthï¼ˆChatGPT ç™»å½•ï¼‰æ”¯æŒ **OpenAI Code (Codex)**ã€‚å‘å¯¼å¯ä»¥è¿è¡Œ OAuth æµç¨‹ï¼Œå¹¶åœ¨é€‚å½“æ—¶å°†é»˜è®¤æ¨¡åž‹è®¾ç½®ä¸º `openai-codex/gpt-5.2`ã€‚å‚é˜…[æ¨¡åž‹æä¾›å•†](/concepts/model-providers)å’Œ[å‘å¯¼](/start/wizard)ã€‚

### æ˜¯å¦æ”¯æŒ OpenAI è®¢é˜…è®¤è¯ï¼ˆCodex OAuthï¼‰

æ˜¯çš„ã€‚KrabKrab å®Œå…¨æ”¯æŒ **OpenAI Code (Codex) è®¢é˜… OAuth**ã€‚æ–°æ‰‹å¼•å¯¼å‘å¯¼å¯ä»¥ä¸ºä½ è¿è¡Œ OAuth æµç¨‹ã€‚

å‚é˜… [OAuth](/concepts/oauth)ã€[æ¨¡åž‹æä¾›å•†](/concepts/model-providers)å’Œ[å‘å¯¼](/start/wizard)ã€‚

### å¦‚ä½•è®¾ç½® Gemini CLI OAuth

Gemini CLI ä½¿ç”¨**æ’ä»¶è®¤è¯æµç¨‹**ï¼Œè€Œä¸æ˜¯ `krabkrab.json` ä¸­çš„ client id æˆ– secretã€‚

æ­¥éª¤ï¼š

1. å¯ç”¨æ’ä»¶ï¼š`krabkrab plugins enable google-gemini-cli-auth`
2. ç™»å½•ï¼š`krabkrab models auth login --provider google-gemini-cli --set-default`

è¿™ä¼šåœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šå°† OAuth ä»¤ç‰Œå­˜å‚¨ä¸ºè®¤è¯é…ç½®æ–‡ä»¶ã€‚è¯¦æƒ…ï¼š[æ¨¡åž‹æä¾›å•†](/concepts/model-providers)ã€‚

### æœ¬åœ°æ¨¡åž‹é€‚åˆæ—¥å¸¸èŠå¤©å—

é€šå¸¸ä¸é€‚åˆã€‚KrabKrab éœ€è¦å¤§ä¸Šä¸‹æ–‡ + å¼ºå®‰å…¨æ€§ï¼›å°æ˜¾å¡ä¼šæˆªæ–­ä¸”æ³„æ¼ã€‚å¦‚æžœå¿…é¡»ä½¿ç”¨ï¼Œè¯·åœ¨æœ¬åœ°è¿è¡Œä½ èƒ½è¿è¡Œçš„**æœ€å¤§** MiniMax M2.1 ç‰ˆæœ¬ï¼ˆLM Studioï¼‰ï¼Œå‚é˜… [/gateway/local-models](/gateway/local-models)ã€‚è¾ƒå°/é‡åŒ–çš„æ¨¡åž‹ä¼šå¢žåŠ æç¤ºæ³¨å…¥é£Žé™©â€”â€”å‚é˜…[å®‰å…¨](/gateway/security)ã€‚

### å¦‚ä½•å°†æ‰˜ç®¡æ¨¡åž‹æµé‡é™åˆ¶åœ¨ç‰¹å®šåŒºåŸŸ

é€‰æ‹©åŒºåŸŸå›ºå®šçš„ç«¯ç‚¹ã€‚OpenRouter ä¸º MiniMaxã€Kimi å’Œ GLM æä¾›ç¾Žå›½æ‰˜ç®¡é€‰é¡¹ï¼›é€‰æ‹©ç¾Žå›½æ‰˜ç®¡å˜ä½“ä»¥ä¿æŒæ•°æ®åœ¨åŒºåŸŸå†…ã€‚ä½ ä»ç„¶å¯ä»¥é€šè¿‡ä½¿ç”¨ `models.mode: "merge"` åœ¨è¿™äº›æ—è¾¹åˆ—å‡º Anthropic/OpenAIï¼Œè¿™æ ·æ•…éšœè½¬ç§»ä¿æŒå¯ç”¨ï¼ŒåŒæ—¶å°Šé‡ä½ é€‰æ‹©çš„åŒºåŸŸæä¾›å•†ã€‚

### æˆ‘å¿…é¡»è´­ä¹° Mac Mini æ‰èƒ½å®‰è£…å—

ä¸éœ€è¦ã€‚KrabKrab è¿è¡Œåœ¨ macOS æˆ– Linux ä¸Šï¼ˆWindows é€šè¿‡ WSL2ï¼‰ã€‚Mac mini æ˜¯å¯é€‰çš„â€”â€”æœ‰äº›äººä¹°ä¸€å°ä½œä¸ºå¸¸å¼€ä¸»æœºï¼Œä½†å°åž‹ VPSã€å®¶åº­æœåŠ¡å™¨æˆ– Raspberry Pi çº§åˆ«çš„è®¾å¤‡ä¹Ÿå¯ä»¥ã€‚

ä½ åªæœ‰åœ¨ä½¿ç”¨ **macOS ä¸“ç”¨å·¥å…·**æ—¶æ‰éœ€è¦ Macã€‚å¯¹äºŽ iMessageï¼Œä½ å¯ä»¥å°† Gateway ç½‘å…³ä¿æŒåœ¨ Linux ä¸Šï¼Œé€šè¿‡å°† `channels.imessage.cliPath` æŒ‡å‘ SSH åŒ…è£…å™¨åœ¨ä»»ä½• Mac ä¸Šè¿è¡Œ `imsg`ã€‚å¦‚æžœä½ éœ€è¦å…¶ä»– macOS ä¸“ç”¨å·¥å…·ï¼Œåœ¨ Mac ä¸Šè¿è¡Œ Gateway ç½‘å…³æˆ–é…å¯¹ä¸€ä¸ª macOS èŠ‚ç‚¹ã€‚

æ–‡æ¡£ï¼š[iMessage](/channels/imessage)ã€[èŠ‚ç‚¹](/nodes)ã€[Mac è¿œç¨‹æ¨¡å¼](/platforms/mac/remote)ã€‚

### iMessage æ”¯æŒéœ€è¦ Mac mini å—

ä½ éœ€è¦**æŸå°ç™»å½•äº† Messages çš„ macOS è®¾å¤‡**ã€‚å®ƒ**ä¸ä¸€å®š**æ˜¯ Mac miniâ€”â€”ä»»ä½• Mac éƒ½å¯ä»¥ã€‚KrabKrab çš„ iMessage é›†æˆåœ¨ macOS ä¸Šè¿è¡Œï¼ˆBlueBubbles æˆ– `imsg`ï¼‰ï¼Œè€Œ Gateway ç½‘å…³å¯ä»¥åœ¨å…¶ä»–åœ°æ–¹è¿è¡Œã€‚

å¸¸è§è®¾ç½®ï¼š

- åœ¨ Linux/VPS ä¸Šè¿è¡Œ Gateway ç½‘å…³ï¼Œå°† `channels.imessage.cliPath` æŒ‡å‘åœ¨ Mac ä¸Šè¿è¡Œ `imsg` çš„ SSH åŒ…è£…å™¨ã€‚
- å¦‚æžœä½ æƒ³è¦æœ€ç®€å•çš„å•æœºè®¾ç½®ï¼Œåœ¨ Mac ä¸Šè¿è¡Œæ‰€æœ‰ç»„ä»¶ã€‚

æ–‡æ¡£ï¼š[iMessage](/channels/imessage)ã€[BlueBubbles](/channels/bluebubbles)ã€
[Mac è¿œç¨‹æ¨¡å¼](/platforms/mac/remote)ã€‚

### å¦‚æžœæˆ‘ä¹°äº† Mac mini è¿è¡Œ KrabKrabï¼Œèƒ½è¿žæŽ¥åˆ°æˆ‘çš„ MacBook Pro å—

å¯ä»¥ã€‚**Mac mini å¯ä»¥è¿è¡Œ Gateway ç½‘å…³**ï¼Œä½ çš„ MacBook Pro å¯ä»¥ä½œä¸º**èŠ‚ç‚¹**ï¼ˆä¼´éšè®¾å¤‡ï¼‰è¿žæŽ¥ã€‚èŠ‚ç‚¹ä¸è¿è¡Œ Gateway ç½‘å…³â€”â€”å®ƒä»¬æä¾›é¢å¤–åŠŸèƒ½ï¼Œå¦‚è¯¥è®¾å¤‡ä¸Šçš„å±å¹•/æ‘„åƒå¤´/ç”»å¸ƒå’Œ `system.run`ã€‚

å¸¸è§æ¨¡å¼ï¼š

- Gateway ç½‘å…³åœ¨ Mac mini ä¸Šï¼ˆå¸¸å¼€ï¼‰ã€‚
- MacBook Pro è¿è¡Œ macOS åº”ç”¨æˆ–èŠ‚ç‚¹ä¸»æœºå¹¶é…å¯¹åˆ° Gateway ç½‘å…³ã€‚
- ä½¿ç”¨ `krabkrab nodes status` / `krabkrab nodes list` æŸ¥çœ‹å®ƒã€‚

æ–‡æ¡£ï¼š[èŠ‚ç‚¹](/nodes)ã€[èŠ‚ç‚¹ CLI](/cli/nodes)ã€‚

### å¯ä»¥ä½¿ç”¨ Bun å—

Bun **ä¸æŽ¨è**ã€‚æˆ‘ä»¬è§‚å¯Ÿåˆ°è¿è¡Œæ—¶ bugï¼Œç‰¹åˆ«æ˜¯åœ¨ WhatsApp å’Œ Telegram æ–¹é¢ã€‚
ä½¿ç”¨ **Node** ä»¥èŽ·å¾—ç¨³å®šçš„ Gateway ç½‘å…³ã€‚

å¦‚æžœä½ ä»æƒ³å°è¯• Bunï¼Œè¯·åœ¨æ²¡æœ‰ WhatsApp/Telegram çš„éžç”Ÿäº§ Gateway ç½‘å…³ä¸Šè¿›è¡Œã€‚

### Telegramï¼šallowFrom å¡«ä»€ä¹ˆ

`channels.telegram.allowFrom` æ˜¯**äººç±»å‘é€è€…çš„ Telegram ç”¨æˆ· ID**ï¼ˆæ•°å­—ï¼ŒæŽ¨èï¼‰æˆ– `@username`ã€‚å®ƒä¸æ˜¯æœºå™¨äººç”¨æˆ·åã€‚

æ›´å®‰å…¨çš„æ–¹å¼ï¼ˆæ— éœ€ç¬¬ä¸‰æ–¹æœºå™¨äººï¼‰ï¼š

- ç»™ä½ çš„æœºå™¨äººå‘ç§ä¿¡ï¼Œç„¶åŽè¿è¡Œ `krabkrab logs --follow` å¹¶è¯»å– `from.id`ã€‚

å®˜æ–¹ Bot APIï¼š

- ç»™ä½ çš„æœºå™¨äººå‘ç§ä¿¡ï¼Œç„¶åŽè°ƒç”¨ `https://api.telegram.org/bot<bot_token>/getUpdates` å¹¶è¯»å– `message.from.id`ã€‚

ç¬¬ä¸‰æ–¹ï¼ˆéšç§æ€§è¾ƒä½Žï¼‰ï¼š

- ç»™ `@userinfobot` æˆ– `@getidsbot` å‘ç§ä¿¡ã€‚

å‚é˜… [/channels/telegram](/channels/telegram#access-control-dms--groups)ã€‚

### å¤šäººèƒ½å¦ä½¿ç”¨åŒä¸€ä¸ª WhatsApp å·ç é…åˆä¸åŒçš„ KrabKrab å®žä¾‹

å¯ä»¥ï¼Œé€šè¿‡**å¤šæ™ºèƒ½ä½“è·¯ç”±**ã€‚å°†æ¯ä¸ªå‘é€è€…çš„ WhatsApp **ç§ä¿¡**ï¼ˆpeer `kind: "dm"`ï¼Œå‘é€è€… E.164 æ ¼å¼å¦‚ `+15551234567`ï¼‰ç»‘å®šåˆ°ä¸åŒçš„ `agentId`ï¼Œè¿™æ ·æ¯ä¸ªäººèŽ·å¾—è‡ªå·±çš„å·¥ä½œåŒºå’Œä¼šè¯å­˜å‚¨ã€‚å›žå¤ä»ç„¶æ¥è‡ª**åŒä¸€ä¸ª WhatsApp è´¦æˆ·**ï¼Œä¸”ç§ä¿¡è®¿é—®æŽ§åˆ¶ï¼ˆ`channels.whatsapp.dmPolicy` / `channels.whatsapp.allowFrom`ï¼‰å¯¹æ¯ä¸ª WhatsApp è´¦æˆ·æ˜¯å…¨å±€çš„ã€‚å‚é˜…[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)å’Œ [WhatsApp](/channels/whatsapp)ã€‚

### èƒ½å¦åŒæ—¶è¿è¡Œä¸€ä¸ªâ€œå¿«é€ŸèŠå¤©â€æ™ºèƒ½ä½“å’Œä¸€ä¸ªâ€œç”¨ Opus ç¼–ç¨‹â€çš„æ™ºèƒ½ä½“

å¯ä»¥ã€‚ä½¿ç”¨å¤šæ™ºèƒ½ä½“è·¯ç”±ï¼šä¸ºæ¯ä¸ªæ™ºèƒ½ä½“è®¾ç½®è‡ªå·±çš„é»˜è®¤æ¨¡åž‹ï¼Œç„¶åŽå°†å…¥ç«™è·¯ç”±ï¼ˆæä¾›å•†è´¦æˆ·æˆ–ç‰¹å®šå¯¹ç­‰æ–¹ï¼‰ç»‘å®šåˆ°æ¯ä¸ªæ™ºèƒ½ä½“ã€‚ç¤ºä¾‹é…ç½®ä½äºŽ[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ã€‚å¦å‚é˜…[æ¨¡åž‹](/concepts/models)å’Œ[é…ç½®](/gateway/configuration)ã€‚

### Homebrew åœ¨ Linux ä¸Šå¯ç”¨å—

å¯ä»¥ã€‚Homebrew æ”¯æŒ Linuxï¼ˆLinuxbrewï¼‰ã€‚å¿«é€Ÿè®¾ç½®ï¼š

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> ~/.profile
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
brew install <formula>
```

å¦‚æžœä½ é€šè¿‡ systemd è¿è¡Œ KrabKrabï¼Œç¡®ä¿æœåŠ¡ PATH åŒ…å« `/home/linuxbrew/.linuxbrew/bin`ï¼ˆæˆ–ä½ çš„ brew å‰ç¼€ï¼‰ï¼Œä»¥ä¾¿ `brew` å®‰è£…çš„å·¥å…·åœ¨éžç™»å½• shell ä¸­å¯è§£æžã€‚
æœ€è¿‘çš„æž„å»ºè¿˜ä¼šåœ¨ Linux systemd æœåŠ¡ä¸Šè‡ªåŠ¨æ·»åŠ å¸¸è§çš„ç”¨æˆ· bin ç›®å½•ï¼ˆä¾‹å¦‚ `~/.local/bin`ã€`~/.npm-global/bin`ã€`~/.local/share/pnpm`ã€`~/.bun/bin`ï¼‰ï¼Œå¹¶åœ¨è®¾ç½®æ—¶å°Šé‡ `PNPM_HOME`ã€`NPM_CONFIG_PREFIX`ã€`BUN_INSTALL`ã€`VOLTA_HOME`ã€`ASDF_DATA_DIR`ã€`NVM_DIR` å’Œ `FNM_DIR`ã€‚

### å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…å’Œ npm å®‰è£…æœ‰ä»€ä¹ˆåŒºåˆ«

- **å¯ç¼–è¾‘ï¼ˆgitï¼‰å®‰è£…ï¼š** å®Œæ•´æºç  checkoutï¼Œå¯ç¼–è¾‘ï¼Œæœ€é€‚åˆè´¡çŒ®è€…ã€‚ä½ åœ¨æœ¬åœ°è¿è¡Œæž„å»ºå¹¶å¯ä»¥ä¿®è¡¥ä»£ç /æ–‡æ¡£ã€‚
- **npm å®‰è£…ï¼š** å…¨å±€ CLI å®‰è£…ï¼Œæ— ä»“åº“ï¼Œæœ€é€‚åˆâ€œç›´æŽ¥è¿è¡Œâ€ã€‚æ›´æ–°æ¥è‡ª npm dist-tagsã€‚

æ–‡æ¡£ï¼š[å…¥é—¨](/start/getting-started)ã€[æ›´æ–°](/install/updating)ã€‚

### ä¹‹åŽå¯ä»¥åœ¨ npm å’Œ git å®‰è£…ä¹‹é—´åˆ‡æ¢å—

å¯ä»¥ã€‚å®‰è£…å¦ä¸€ç§æ–¹å¼ï¼Œç„¶åŽè¿è¡Œ Doctor ä½¿ Gateway ç½‘å…³æœåŠ¡æŒ‡å‘æ–°çš„å…¥å£ç‚¹ã€‚
è¿™**ä¸ä¼šåˆ é™¤ä½ çš„æ•°æ®**â€”â€”å®ƒåªæ”¹å˜ KrabKrab ä»£ç çš„å®‰è£…ä½ç½®ã€‚ä½ çš„çŠ¶æ€
ï¼ˆ`~/.krabkrab`ï¼‰å’Œå·¥ä½œåŒºï¼ˆ`~/.krabkrab/workspace`ï¼‰ä¿æŒä¸å˜ã€‚

ä»Ž npm â†’ gitï¼š

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
pnpm install
pnpm build
krabkrab doctor
krabkrab gateway restart
```

ä»Ž git â†’ npmï¼š

```bash
npm install -g krabkrab@latest
krabkrab doctor
krabkrab gateway restart
```

Doctor ä¼šæ£€æµ‹ Gateway ç½‘å…³æœåŠ¡å…¥å£ç‚¹ä¸åŒ¹é…ï¼Œå¹¶æä¾›é‡å†™æœåŠ¡é…ç½®ä»¥åŒ¹é…å½“å‰å®‰è£…çš„é€‰é¡¹ï¼ˆåœ¨è‡ªåŠ¨åŒ–ä¸­ä½¿ç”¨ `--repair`ï¼‰ã€‚

å¤‡ä»½æç¤ºï¼šå‚é˜…[å¤‡ä»½ç­–ç•¥](/help/faq#whats-the-recommended-backup-strategy)ã€‚

### åº”è¯¥åœ¨ç¬”è®°æœ¬ç”µè„‘è¿˜æ˜¯ VPS ä¸Šè¿è¡Œ Gateway ç½‘å…³ç®€çŸ­å›žç­”ï¼š**å¦‚æžœä½ æƒ³è¦ 24/7 å¯é æ€§ï¼Œä½¿ç”¨ VPS**ã€‚å¦‚æžœä½ æƒ³è¦æœ€ä½Žæ‘©æ“¦ä¸”èƒ½æŽ¥å—ä¼‘çœ /é‡å¯ï¼Œåœ¨æœ¬åœ°è¿è¡Œã€‚

**ç¬”è®°æœ¬ï¼ˆæœ¬åœ° Gateway ç½‘å…³ï¼‰**

- **ä¼˜ç‚¹ï¼š** æ— æœåŠ¡å™¨æˆæœ¬ï¼Œç›´æŽ¥è®¿é—®æœ¬åœ°æ–‡ä»¶ï¼Œå®žæ—¶æµè§ˆå™¨çª—å£ã€‚
- **ç¼ºç‚¹ï¼š** ä¼‘çœ /ç½‘ç»œä¸­æ–­ = æ–­è¿žï¼Œæ“ä½œç³»ç»Ÿæ›´æ–°/é‡å¯ä¼šä¸­æ–­ï¼Œå¿…é¡»ä¿æŒå”¤é†’ã€‚

**VPS / äº‘**

- **ä¼˜ç‚¹ï¼š** å¸¸å¼€ï¼Œç½‘ç»œç¨³å®šï¼Œæ— ç¬”è®°æœ¬ä¼‘çœ é—®é¢˜ï¼Œæ›´å®¹æ˜“ä¿æŒè¿è¡Œã€‚
- **ç¼ºç‚¹ï¼š** é€šå¸¸æ— å¤´è¿è¡Œï¼ˆä½¿ç”¨æˆªå›¾ï¼‰ï¼Œä»…è¿œç¨‹æ–‡ä»¶è®¿é—®ï¼Œæ›´æ–°éœ€è¦ SSHã€‚

**KrabKrab ç‰¹å®šè¯´æ˜Žï¼š** WhatsApp/Telegram/Slack/Mattermostï¼ˆæ’ä»¶ï¼‰/Discord åœ¨ VPS ä¸Šéƒ½èƒ½æ­£å¸¸å·¥ä½œã€‚å”¯ä¸€çš„çœŸæ­£æƒè¡¡æ˜¯**æ— å¤´æµè§ˆå™¨**ä¸Žå¯è§çª—å£ã€‚å‚é˜…[æµè§ˆå™¨](/tools/browser)ã€‚

**æŽ¨èé»˜è®¤å€¼ï¼š** å¦‚æžœä¹‹å‰é‡åˆ°è¿‡ Gateway ç½‘å…³æ–­è¿žï¼Œä½¿ç”¨ VPSã€‚å½“ä½ æ­£åœ¨ç§¯æžä½¿ç”¨ Mac å¹¶ä¸”éœ€è¦æœ¬åœ°æ–‡ä»¶è®¿é—®æˆ–å¯è§æµè§ˆå™¨çš„ UI è‡ªåŠ¨åŒ–æ—¶ï¼Œæœ¬åœ°è¿è¡Œå¾ˆå¥½ã€‚

### åœ¨ä¸“ç”¨æœºå™¨ä¸Šè¿è¡Œ KrabKrab æœ‰å¤šé‡è¦

ä¸æ˜¯å¿…éœ€çš„ï¼Œä½†**æŽ¨èç”¨äºŽå¯é æ€§å’Œéš”ç¦»**ã€‚

- **ä¸“ç”¨ä¸»æœºï¼ˆVPS/Mac mini/Piï¼‰ï¼š** å¸¸å¼€ï¼Œæ›´å°‘çš„ä¼‘çœ /é‡å¯ä¸­æ–­ï¼Œæ›´å¹²å‡€çš„æƒé™ï¼Œæ›´å®¹æ˜“ä¿æŒè¿è¡Œã€‚
- **å…±äº«çš„ç¬”è®°æœ¬/å°å¼æœºï¼š** å®Œå…¨é€‚åˆæµ‹è¯•å’Œæ´»è·ƒä½¿ç”¨ï¼Œä½†å½“æœºå™¨ä¼‘çœ æˆ–æ›´æ–°æ—¶é¢„æœŸä¼šæœ‰æš‚åœã€‚

å¦‚æžœä½ æƒ³è¦ä¸¤å…¨å…¶ç¾Žï¼Œå°† Gateway ç½‘å…³ä¿æŒåœ¨ä¸“ç”¨ä¸»æœºä¸Šï¼Œå¹¶å°†ç¬”è®°æœ¬é…å¯¹ä¸º**èŠ‚ç‚¹**ä»¥èŽ·å–æœ¬åœ°å±å¹•/æ‘„åƒå¤´/æ‰§è¡Œå·¥å…·ã€‚å‚é˜…[èŠ‚ç‚¹](/nodes)ã€‚
å®‰å…¨æŒ‡å—è¯·é˜…è¯»[å®‰å…¨](/gateway/security)ã€‚

### VPS çš„æœ€ä½Žè¦æ±‚å’ŒæŽ¨èæ“ä½œç³»ç»Ÿæ˜¯ä»€ä¹ˆ

KrabKrab æ˜¯è½»é‡çº§çš„ã€‚å¯¹äºŽåŸºæœ¬çš„ Gateway ç½‘å…³ + ä¸€ä¸ªèŠå¤©æ¸ é“ï¼š

- **ç»å¯¹æœ€ä½Žï¼š** 1 vCPUï¼Œ1GB RAMï¼Œçº¦ 500MB ç£ç›˜ã€‚
- **æŽ¨èï¼š** 1-2 vCPUï¼Œ2GB RAM æˆ–æ›´å¤šä»¥ç•™æœ‰ä½™é‡ï¼ˆæ—¥å¿—ã€åª’ä½“ã€å¤šæ¸ é“ï¼‰ã€‚èŠ‚ç‚¹å·¥å…·å’Œæµè§ˆå™¨è‡ªåŠ¨åŒ–å¯èƒ½æ¶ˆè€—è¾ƒå¤šèµ„æºã€‚

æ“ä½œç³»ç»Ÿï¼šä½¿ç”¨ **Ubuntu LTS**ï¼ˆæˆ–ä»»ä½•çŽ°ä»£ Debian/Ubuntuï¼‰ã€‚Linux å®‰è£…è·¯å¾„åœ¨é‚£é‡Œæµ‹è¯•å¾—æœ€å……åˆ†ã€‚

æ–‡æ¡£ï¼š[Linux](/platforms/linux)ã€[VPS æ‰˜ç®¡](/vps)ã€‚

### å¯ä»¥åœ¨è™šæ‹Ÿæœºä¸­è¿è¡Œ KrabKrab å—ï¼Ÿæœ‰ä»€ä¹ˆè¦æ±‚

å¯ä»¥ã€‚å°†è™šæ‹Ÿæœºè§†ä¸ºä¸Ž VPS ç›¸åŒï¼šå®ƒéœ€è¦å¸¸å¼€ã€å¯è¾¾ï¼Œå¹¶æœ‰è¶³å¤Ÿçš„ RAM ç”¨äºŽ Gateway ç½‘å…³å’Œä½ å¯ç”¨çš„ä»»ä½•æ¸ é“ã€‚

åŸºå‡†æŒ‡å—ï¼š

- **ç»å¯¹æœ€ä½Žï¼š** 1 vCPUï¼Œ1GB RAMã€‚
- **æŽ¨èï¼š** 2GB RAM æˆ–æ›´å¤šï¼Œå¦‚æžœä½ è¿è¡Œå¤šä¸ªæ¸ é“ã€æµè§ˆå™¨è‡ªåŠ¨åŒ–æˆ–åª’ä½“å·¥å…·ã€‚
- **æ“ä½œç³»ç»Ÿï¼š** Ubuntu LTS æˆ–å…¶ä»–çŽ°ä»£ Debian/Ubuntuã€‚

å¦‚æžœä½ ä½¿ç”¨ Windowsï¼Œ**WSL2 æ˜¯æœ€ç®€å•çš„è™šæ‹Ÿæœºå¼è®¾ç½®**ï¼Œå…·æœ‰æœ€ä½³çš„å·¥å…·å…¼å®¹æ€§ã€‚å‚é˜… [Windows](/platforms/windows)ã€[VPS æ‰˜ç®¡](/vps)ã€‚
å¦‚æžœä½ åœ¨è™šæ‹Ÿæœºä¸­è¿è¡Œ macOSï¼Œå‚é˜… [macOS VM](/install/macos-vm)ã€‚

## ä»€ä¹ˆæ˜¯ KrabKrabï¼Ÿ

### ç”¨ä¸€æ®µè¯æè¿° KrabKrab

KrabKrab æ˜¯ä¸€ä¸ªè¿è¡Œåœ¨ä½ è‡ªå·±è®¾å¤‡ä¸Šçš„ä¸ªäºº AI åŠ©æ‰‹ã€‚å®ƒåœ¨ä½ å·²ç»ä½¿ç”¨çš„æ¶ˆæ¯å¹³å°ä¸Šå›žå¤ï¼ˆWhatsAppã€Telegramã€Slackã€Mattermostï¼ˆæ’ä»¶ï¼‰ã€Discordã€Google Chatã€Signalã€iMessageã€WebChatï¼‰ï¼Œè¿˜å¯ä»¥åœ¨æ”¯æŒçš„å¹³å°ä¸Šè¿›è¡Œè¯­éŸ³å’Œå®žæ—¶ Canvasã€‚**Gateway ç½‘å…³** æ˜¯å¸¸å¼€çš„æŽ§åˆ¶å¹³é¢ï¼›åŠ©æ‰‹æ˜¯äº§å“ã€‚

### ä»·å€¼ä¸»å¼ æ˜¯ä»€ä¹ˆ

KrabKrab ä¸æ˜¯â€œåªæ˜¯ä¸€ä¸ª Claude åŒ…è£…å™¨â€ã€‚å®ƒæ˜¯ä¸€ä¸ª**æœ¬åœ°ä¼˜å…ˆçš„æŽ§åˆ¶å¹³é¢**ï¼Œè®©ä½ åœ¨**è‡ªå·±çš„ç¡¬ä»¶**ä¸Šè¿è¡Œå¼ºå¤§çš„åŠ©æ‰‹ï¼Œå¯ä»Žä½ å·²ç»ä½¿ç”¨çš„èŠå¤©åº”ç”¨è®¿é—®ï¼Œå…·æœ‰æœ‰çŠ¶æ€ä¼šè¯ã€è®°å¿†å’Œå·¥å…·â€”â€”æ— éœ€å°†å·¥ä½œæµç¨‹çš„æŽ§åˆ¶æƒäº¤ç»™æ‰˜ç®¡ SaaSã€‚

äº®ç‚¹ï¼š

- **ä½ çš„è®¾å¤‡ï¼Œä½ çš„æ•°æ®ï¼š** åœ¨ä»»ä½•ä½ æƒ³è¦çš„åœ°æ–¹è¿è¡Œ Gateway ç½‘å…³ï¼ˆMacã€Linuxã€VPSï¼‰ï¼Œå¹¶å°†å·¥ä½œåŒº + ä¼šè¯åŽ†å²ä¿æŒåœ¨æœ¬åœ°ã€‚
- **çœŸå®žæ¸ é“ï¼Œè€Œéž Web æ²™ç®±ï¼š** WhatsApp/Telegram/Slack/Discord/Signal/iMessage/ç­‰ï¼ŒåŠ ä¸Šæ”¯æŒå¹³å°ä¸Šçš„ç§»åŠ¨è¯­éŸ³å’Œ Canvasã€‚
- **æ¨¡åž‹æ— å…³ï¼š** ä½¿ç”¨ Anthropicã€OpenAIã€MiniMaxã€OpenRouter ç­‰ï¼Œæ”¯æŒæŒ‰æ™ºèƒ½ä½“è·¯ç”±å’Œæ•…éšœè½¬ç§»ã€‚
- **çº¯æœ¬åœ°é€‰é¡¹ï¼š** è¿è¡Œæœ¬åœ°æ¨¡åž‹ï¼Œè®©**æ‰€æœ‰æ•°æ®éƒ½ä¿ç•™åœ¨ä½ çš„è®¾å¤‡ä¸Š**ã€‚
- **å¤šæ™ºèƒ½ä½“è·¯ç”±ï¼š** æŒ‰æ¸ é“ã€è´¦æˆ·æˆ–ä»»åŠ¡åˆ†é…ä¸åŒçš„æ™ºèƒ½ä½“ï¼Œæ¯ä¸ªéƒ½æœ‰è‡ªå·±çš„å·¥ä½œåŒºå’Œé»˜è®¤å€¼ã€‚
- **å¼€æºä¸”å¯ç¼–è¾‘ï¼š** æ— ä¾›åº”å•†é”å®šåœ°æ£€æŸ¥ã€æ‰©å±•å’Œè‡ªæ‰˜ç®¡ã€‚

æ–‡æ¡£ï¼š[Gateway ç½‘å…³](/gateway)ã€[æ¸ é“](/channels)ã€[å¤šæ™ºèƒ½ä½“](/concepts/multi-agent)ã€
[è®°å¿†](/concepts/memory)ã€‚

### åˆšè®¾ç½®å¥½ï¼Œåº”è¯¥å…ˆåšä»€ä¹ˆ

å¥½çš„å…¥é—¨é¡¹ç›®ï¼š

- å»ºä¸€ä¸ªç½‘ç«™ï¼ˆWordPressã€Shopify æˆ–ç®€å•çš„é™æ€ç«™ç‚¹ï¼‰ã€‚
- åšä¸€ä¸ªç§»åŠ¨åº”ç”¨åŽŸåž‹ï¼ˆå¤§çº²ã€ç•Œé¢ã€API è®¡åˆ’ï¼‰ã€‚
- æ•´ç†æ–‡ä»¶å’Œæ–‡ä»¶å¤¹ï¼ˆæ¸…ç†ã€å‘½åã€æ‰“æ ‡ç­¾ï¼‰ã€‚
- è¿žæŽ¥ Gmail å¹¶è‡ªåŠ¨åŒ–æ‘˜è¦æˆ–è·Ÿè¿›ã€‚

å®ƒå¯ä»¥å¤„ç†å¤§åž‹ä»»åŠ¡ï¼Œä½†æœ€å¥½å°†å…¶æ‹†åˆ†ä¸ºå¤šä¸ªé˜¶æ®µï¼Œå¹¶ä½¿ç”¨å­æ™ºèƒ½ä½“è¿›è¡Œå¹¶è¡Œå·¥ä½œã€‚

### KrabKrab æ—¥å¸¸æœ€å¸¸ç”¨çš„äº”ä¸ªåœºæ™¯æ˜¯ä»€ä¹ˆ

æ—¥å¸¸æ”¶ç›Šé€šå¸¸åŒ…æ‹¬ï¼š

- **ä¸ªäººç®€æŠ¥ï¼š** æ”¶ä»¶ç®±ã€æ—¥åŽ†å’Œä½ å…³å¿ƒçš„æ–°é—»æ‘˜è¦ã€‚
- **ç ”ç©¶å’Œèµ·è‰ï¼š** å¿«é€Ÿç ”ç©¶ã€æ‘˜è¦ä»¥åŠé‚®ä»¶æˆ–æ–‡æ¡£çš„åˆç¨¿ã€‚
- **æé†’å’Œè·Ÿè¿›ï¼š** å®šæ—¶ä»»åŠ¡æˆ–å¿ƒè·³é©±åŠ¨çš„æé†’å’Œæ£€æŸ¥æ¸…å•ã€‚
- **æµè§ˆå™¨è‡ªåŠ¨åŒ–ï¼š** å¡«å†™è¡¨å•ã€æ”¶é›†æ•°æ®å’Œé‡å¤æ€§ç½‘é¡µä»»åŠ¡ã€‚
- **è·¨è®¾å¤‡åè°ƒï¼š** ä»Žæ‰‹æœºå‘é€ä»»åŠ¡ï¼Œè®© Gateway ç½‘å…³åœ¨æœåŠ¡å™¨ä¸Šè¿è¡Œï¼Œç„¶åŽåœ¨èŠå¤©ä¸­èŽ·å–ç»“æžœã€‚

### KrabKrab èƒ½å¦å¸®åŠ© SaaS è¿›è¡ŒèŽ·å®¢ã€å¤–è”ã€å¹¿å‘Šå’Œåšå®¢

å¯ä»¥ç”¨äºŽ**è°ƒç ”ã€ç­›é€‰å’Œèµ·è‰**ã€‚å®ƒå¯ä»¥æ‰«æç½‘ç«™ã€å»ºç«‹å€™é€‰åå•ã€æ€»ç»“æ½œåœ¨å®¢æˆ·ï¼Œå¹¶æ’°å†™å¤–è”æˆ–å¹¿å‘Šæ–‡æ¡ˆè‰ç¨¿ã€‚

å¯¹äºŽ**å¤–è”æˆ–å¹¿å‘ŠæŠ•æ”¾**ï¼Œè¯·ä¿æŒäººå·¥å®¡æ ¸ã€‚é¿å…åžƒåœ¾é‚®ä»¶ï¼Œéµå®ˆå½“åœ°æ³•å¾‹å’Œå¹³å°æ”¿ç­–ï¼Œåœ¨å‘é€ä¹‹å‰å®¡æŸ¥æ‰€æœ‰å†…å®¹ã€‚æœ€å®‰å…¨çš„æ¨¡å¼æ˜¯è®© KrabKrab èµ·è‰ï¼Œç”±ä½ æ‰¹å‡†ã€‚

æ–‡æ¡£ï¼š[å®‰å…¨](/gateway/security)ã€‚

### ç›¸æ¯” Claude Codeï¼Œåœ¨ Web å¼€å‘æ–¹é¢æœ‰ä»€ä¹ˆä¼˜åŠ¿

KrabKrab æ˜¯ä¸€ä¸ª**ä¸ªäººåŠ©æ‰‹**å’Œåè°ƒå±‚ï¼Œä¸æ˜¯ IDE æ›¿ä»£å“ã€‚ä½¿ç”¨ Claude Code æˆ– Codex åœ¨ä»“åº“ä¸­è¿›è¡Œæœ€å¿«çš„ç›´æŽ¥ç¼–ç å¾ªçŽ¯ã€‚å½“ä½ éœ€è¦æŒä¹…è®°å¿†ã€è·¨è®¾å¤‡è®¿é—®å’Œå·¥å…·ç¼–æŽ’æ—¶ï¼Œä½¿ç”¨ KrabKrabã€‚

ä¼˜åŠ¿ï¼š

- è·¨ä¼šè¯çš„**æŒä¹…è®°å¿† + å·¥ä½œåŒº**
- **å¤šå¹³å°è®¿é—®**ï¼ˆWhatsAppã€Telegramã€TUIã€WebChatï¼‰
- **å·¥å…·ç¼–æŽ’**ï¼ˆæµè§ˆå™¨ã€æ–‡ä»¶ã€è°ƒåº¦ã€é’©å­ï¼‰
- **å¸¸å¼€ Gateway ç½‘å…³**ï¼ˆåœ¨ VPS ä¸Šè¿è¡Œï¼Œä»Žä»»ä½•åœ°æ–¹äº¤äº’ï¼‰
- ç”¨äºŽæœ¬åœ°æµè§ˆå™¨/å±å¹•/æ‘„åƒå¤´/æ‰§è¡Œçš„**èŠ‚ç‚¹**

å±•ç¤ºï¼šhttps://krabkrab.ai/showcase

## Skills ä¸Žè‡ªåŠ¨åŒ–

### å¦‚ä½•è‡ªå®šä¹‰ Skills è€Œä¸å¼„è„ä»“åº“

ä½¿ç”¨æ‰˜ç®¡è¦†ç›–è€Œä¸æ˜¯ç¼–è¾‘ä»“åº“å‰¯æœ¬ã€‚å°†ä½ çš„æ›´æ”¹æ”¾åœ¨ `~/.krabkrab/skills/<name>/SKILL.md`ï¼ˆæˆ–é€šè¿‡ `~/.krabkrab/krabkrab.json` ä¸­çš„ `skills.load.extraDirs` æ·»åŠ æ–‡ä»¶å¤¹ï¼‰ã€‚ä¼˜å…ˆçº§æ˜¯ `<workspace>/skills` > `~/.krabkrab/skills` > å†…ç½®ï¼Œæ‰€ä»¥æ‰˜ç®¡è¦†ç›–ä¼˜å…ˆç”Ÿæ•ˆè€Œä¸ä¼šä¿®æ”¹ gitã€‚åªæœ‰å€¼å¾—ä¸Šæ¸¸åˆå¹¶çš„ç¼–è¾‘æ‰åº”è¯¥æ”¾åœ¨ä»“åº“ä¸­å¹¶ä½œä¸º PR æäº¤ã€‚

### å¯ä»¥ä»Žè‡ªå®šä¹‰æ–‡ä»¶å¤¹åŠ è½½ Skills å—

å¯ä»¥ã€‚é€šè¿‡ `~/.krabkrab/krabkrab.json` ä¸­çš„ `skills.load.extraDirs` æ·»åŠ é¢å¤–ç›®å½•ï¼ˆæœ€ä½Žä¼˜å…ˆçº§ï¼‰ã€‚é»˜è®¤ä¼˜å…ˆçº§ä¿æŒä¸å˜ï¼š`<workspace>/skills` â†’ `~/.krabkrab/skills` â†’ å†…ç½® â†’ `skills.load.extraDirs`ã€‚`clawhub` é»˜è®¤å®‰è£…åˆ° `./skills`ï¼ŒKrabKrab å°†å…¶è§†ä¸º `<workspace>/skills`ã€‚

### å¦‚ä½•ä¸ºä¸åŒä»»åŠ¡ä½¿ç”¨ä¸åŒæ¨¡åž‹

ç›®å‰æ”¯æŒçš„æ¨¡å¼æœ‰ï¼š

- **å®šæ—¶ä»»åŠ¡**ï¼šéš”ç¦»çš„ä»»åŠ¡å¯ä»¥ä¸ºæ¯ä¸ªä»»åŠ¡è®¾ç½® `model` è¦†ç›–ã€‚
- **å­æ™ºèƒ½ä½“**ï¼šå°†ä»»åŠ¡è·¯ç”±åˆ°å…·æœ‰ä¸åŒé»˜è®¤æ¨¡åž‹çš„ç‹¬ç«‹æ™ºèƒ½ä½“ã€‚
- **æŒ‰éœ€åˆ‡æ¢**ï¼šä½¿ç”¨ `/model` éšæ—¶åˆ‡æ¢å½“å‰ä¼šè¯æ¨¡åž‹ã€‚

å‚é˜…[å®šæ—¶ä»»åŠ¡](/automation/cron-jobs)ã€[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)å’Œ[æ–œæ å‘½ä»¤](/tools/slash-commands)ã€‚

### æœºå™¨äººåœ¨æ‰§è¡Œç¹é‡å·¥ä½œæ—¶å¡ä½äº†ï¼Œå¦‚ä½•å¸è½½ä»»åŠ¡

ä½¿ç”¨**å­æ™ºèƒ½ä½“**å¤„ç†é•¿æ—¶é—´æˆ–å¹¶è¡Œä»»åŠ¡ã€‚å­æ™ºèƒ½ä½“åœ¨è‡ªå·±çš„ä¼šè¯ä¸­è¿è¡Œï¼Œè¿”å›žæ‘˜è¦ï¼Œå¹¶ä¿æŒä½ çš„ä¸»èŠå¤©å“åº”ã€‚

è¦æ±‚ä½ çš„æœºå™¨äººâ€œä¸ºè¿™ä¸ªä»»åŠ¡ç”Ÿæˆä¸€ä¸ªå­æ™ºèƒ½ä½“â€æˆ–ä½¿ç”¨ `/subagents`ã€‚
åœ¨èŠå¤©ä¸­ä½¿ç”¨ `/status` æŸ¥çœ‹ Gateway ç½‘å…³å½“å‰æ­£åœ¨åšä»€ä¹ˆï¼ˆä»¥åŠæ˜¯å¦å¿™ç¢Œï¼‰ã€‚

ä»¤ç‰Œæç¤ºï¼šé•¿ä»»åŠ¡å’Œå­æ™ºèƒ½ä½“éƒ½æ¶ˆè€—ä»¤ç‰Œã€‚å¦‚æžœå…³æ³¨æˆæœ¬ï¼Œé€šè¿‡ `agents.defaults.subagents.model` ä¸ºå­æ™ºèƒ½ä½“è®¾ç½®æ›´ä¾¿å®œçš„æ¨¡åž‹ã€‚

æ–‡æ¡£ï¼š[å­æ™ºèƒ½ä½“](/tools/subagents)ã€‚

### å®šæ—¶ä»»åŠ¡æˆ–æé†’æ²¡æœ‰è§¦å‘ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆ

å®šæ—¶ä»»åŠ¡åœ¨ Gateway ç½‘å…³è¿›ç¨‹å†…è¿è¡Œã€‚å¦‚æžœ Gateway ç½‘å…³æ²¡æœ‰æŒç»­è¿è¡Œï¼Œè®¡åˆ’ä»»åŠ¡å°†ä¸ä¼šè¿è¡Œã€‚

æ£€æŸ¥æ¸…å•ï¼š

- ç¡®è®¤ cron å·²å¯ç”¨ï¼ˆ`cron.enabled`ï¼‰ä¸”æœªè®¾ç½® `krabkrab_SKIP_CRON`ã€‚
- æ£€æŸ¥ Gateway ç½‘å…³æ˜¯å¦ 24/7 è¿è¡Œï¼ˆæ— ä¼‘çœ /é‡å¯ï¼‰ã€‚
- éªŒè¯ä»»åŠ¡çš„æ—¶åŒºè®¾ç½®ï¼ˆ`--tz` ä¸Žä¸»æœºæ—¶åŒºï¼‰ã€‚

è°ƒè¯•ï¼š

```bash
krabkrab cron run <jobId> --force
krabkrab cron runs --id <jobId> --limit 50
```

æ–‡æ¡£ï¼š[å®šæ—¶ä»»åŠ¡](/automation/cron-jobs)ã€[å®šæ—¶ä»»åŠ¡ vs å¿ƒè·³](/automation/cron-vs-heartbeat)ã€‚

### å¦‚ä½•åœ¨ Linux ä¸Šå®‰è£… Skills

ä½¿ç”¨ **ClawHub**ï¼ˆCLIï¼‰æˆ–å°† Skills æ”¾å…¥ä½ çš„å·¥ä½œåŒºã€‚macOS Skills UI åœ¨ Linux ä¸Šä¸å¯ç”¨ã€‚
æµè§ˆ Skillsï¼šhttps://clawhub.comã€‚

å®‰è£… ClawHub CLIï¼ˆé€‰æ‹©ä¸€ä¸ªåŒ…ç®¡ç†å™¨ï¼‰ï¼š

```bash
npm i -g clawhub
```

```bash
pnpm add -g clawhub
```

### KrabKrab èƒ½å¦æŒ‰è®¡åˆ’æˆ–åœ¨åŽå°æŒç»­è¿è¡Œä»»åŠ¡

å¯ä»¥ã€‚ä½¿ç”¨ Gateway ç½‘å…³è°ƒåº¦å™¨ï¼š

- **å®šæ—¶ä»»åŠ¡**ç”¨äºŽè®¡åˆ’æˆ–é‡å¤ä»»åŠ¡ï¼ˆè·¨é‡å¯æŒä¹…åŒ–ï¼‰ã€‚
- **å¿ƒè·³**ç”¨äºŽâ€œä¸»ä¼šè¯â€å®šæœŸæ£€æŸ¥ã€‚
- **éš”ç¦»ä»»åŠ¡**ç”¨äºŽè‡ªä¸»æ™ºèƒ½ä½“å‘å¸ƒæ‘˜è¦æˆ–æŠ•é€’åˆ°èŠå¤©ã€‚

æ–‡æ¡£ï¼š[å®šæ—¶ä»»åŠ¡](/automation/cron-jobs)ã€[å®šæ—¶ä»»åŠ¡ vs å¿ƒè·³](/automation/cron-vs-heartbeat)ã€
[å¿ƒè·³](/gateway/heartbeat)ã€‚

**èƒ½å¦ä»Ž Linux è¿è¡Œä»…é™ Apple/macOS çš„ Skills**

ä¸èƒ½ç›´æŽ¥è¿è¡Œã€‚macOS Skills å— `metadata.krabkrab.os` å’Œæ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶é™åˆ¶ï¼ŒSkills åªæœ‰åœ¨ **Gateway ç½‘å…³ä¸»æœº**ä¸Šç¬¦åˆæ¡ä»¶æ—¶æ‰ä¼šå‡ºçŽ°åœ¨ç³»ç»Ÿæç¤ºä¸­ã€‚åœ¨ Linux ä¸Šï¼Œ`darwin` ä¸“ç”¨ Skillsï¼ˆå¦‚ `apple-notes`ã€`apple-reminders`ã€`things-mac`ï¼‰ä¸ä¼šåŠ è½½ï¼Œé™¤éžä½ è¦†ç›–é™åˆ¶ã€‚

ä½ æœ‰ä¸‰ç§æ”¯æŒçš„æ¨¡å¼ï¼š

**æ–¹æ¡ˆ A - åœ¨ Mac ä¸Šè¿è¡Œ Gateway ç½‘å…³ï¼ˆæœ€ç®€å•ï¼‰ã€‚**
åœ¨ macOS äºŒè¿›åˆ¶æ–‡ä»¶æ‰€åœ¨çš„åœ°æ–¹è¿è¡Œ Gateway ç½‘å…³ï¼Œç„¶åŽä»Ž Linux é€šè¿‡[è¿œç¨‹æ¨¡å¼](#how-do-i-run-krabkrab-in-remote-mode-client-connects-to-a-gateway-elsewhere)æˆ– Tailscale è¿žæŽ¥ã€‚Skills æ­£å¸¸åŠ è½½ï¼Œå› ä¸º Gateway ç½‘å…³ä¸»æœºæ˜¯ macOSã€‚

**æ–¹æ¡ˆ B - ä½¿ç”¨ macOS èŠ‚ç‚¹ï¼ˆæ— éœ€ SSHï¼‰ã€‚**
åœ¨ Linux ä¸Šè¿è¡Œ Gateway ç½‘å…³ï¼Œé…å¯¹ä¸€ä¸ª macOS èŠ‚ç‚¹ï¼ˆèœå•æ åº”ç”¨ï¼‰ï¼Œå¹¶åœ¨ Mac ä¸Šå°†**èŠ‚ç‚¹è¿è¡Œå‘½ä»¤**è®¾ç½®ä¸ºâ€œå§‹ç»ˆè¯¢é—®â€æˆ–â€œå§‹ç»ˆå…è®¸â€ã€‚å½“æ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶å­˜åœ¨äºŽèŠ‚ç‚¹ä¸Šæ—¶ï¼ŒKrabKrab å¯ä»¥å°† macOS ä¸“ç”¨ Skills è§†ä¸ºç¬¦åˆæ¡ä»¶ã€‚æ™ºèƒ½ä½“é€šè¿‡ `nodes` å·¥å…·è¿è¡Œè¿™äº› Skillsã€‚å¦‚æžœä½ é€‰æ‹©â€œå§‹ç»ˆè¯¢é—®â€ï¼Œåœ¨æç¤ºä¸­æ‰¹å‡†â€œå§‹ç»ˆå…è®¸â€ä¼šå°†è¯¥å‘½ä»¤æ·»åŠ åˆ°å…è®¸åˆ—è¡¨ã€‚

**æ–¹æ¡ˆ C - é€šè¿‡ SSH ä»£ç† macOS äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆé«˜çº§ï¼‰ã€‚**
ä¿æŒ Gateway ç½‘å…³åœ¨ Linux ä¸Šï¼Œä½†ä½¿æ‰€éœ€çš„ CLI äºŒè¿›åˆ¶æ–‡ä»¶è§£æžä¸ºåœ¨ Mac ä¸Šè¿è¡Œçš„ SSH åŒ…è£…å™¨ã€‚ç„¶åŽè¦†ç›– Skills ä»¥å…è®¸ Linux ä½¿å…¶ä¿æŒç¬¦åˆæ¡ä»¶ã€‚

1. ä¸ºäºŒè¿›åˆ¶æ–‡ä»¶åˆ›å»º SSH åŒ…è£…å™¨ï¼ˆç¤ºä¾‹ï¼š`imsg`ï¼‰ï¼š
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail
   exec ssh -T user@mac-host /opt/homebrew/bin/imsg "$@"
   ```
2. å°†åŒ…è£…å™¨æ”¾åœ¨ Linux ä¸»æœºçš„ `PATH` ä¸Šï¼ˆä¾‹å¦‚ `~/bin/imsg`ï¼‰ã€‚
3. è¦†ç›– Skills å…ƒæ•°æ®ï¼ˆå·¥ä½œåŒºæˆ– `~/.krabkrab/skills`ï¼‰ä»¥å…è®¸ Linuxï¼š
   ```markdown
   ---
   name: imsg
   description: iMessage/SMS CLI for listing chats, history, watch, and sending.
   metadata: { "krabkrab": { "os": ["darwin", "linux"], "requires": { "bins": ["imsg"] } } }
   ---
   ```
4. å¼€å§‹æ–°ä¼šè¯ä»¥åˆ·æ–° Skills å¿«ç…§ã€‚

å¯¹äºŽ iMessageï¼Œä½ ä¹Ÿå¯ä»¥å°† `channels.imessage.cliPath` æŒ‡å‘ SSH åŒ…è£…å™¨ï¼ˆKrabKrab åªéœ€è¦ stdioï¼‰ã€‚å‚é˜… [iMessage](/channels/imessage)ã€‚

### æœ‰ Notion æˆ– HeyGen é›†æˆå—

ç›®å‰æ²¡æœ‰å†…ç½®é›†æˆã€‚

é€‰é¡¹ï¼š

- **è‡ªå®šä¹‰ Skills / æ’ä»¶ï¼š** æœ€é€‚åˆå¯é çš„ API è®¿é—®ï¼ˆNotion/HeyGen éƒ½æœ‰ APIï¼‰ã€‚
- **æµè§ˆå™¨è‡ªåŠ¨åŒ–ï¼š** æ— éœ€ç¼–ç ä½†æ›´æ…¢ä¸”æ›´è„†å¼±ã€‚

å¦‚æžœä½ æƒ³æŒ‰å®¢æˆ·ä¿ç•™ä¸Šä¸‹æ–‡ï¼ˆä»£ç†å·¥ä½œæµï¼‰ï¼Œä¸€ä¸ªç®€å•çš„æ¨¡å¼æ˜¯ï¼š

- æ¯ä¸ªå®¢æˆ·ä¸€ä¸ª Notion é¡µé¢ï¼ˆä¸Šä¸‹æ–‡ + åå¥½ + å½“å‰å·¥ä½œï¼‰ã€‚
- åœ¨ä¼šè¯å¼€å§‹æ—¶è¦æ±‚æ™ºèƒ½ä½“èŽ·å–è¯¥é¡µé¢ã€‚

å¦‚æžœä½ æƒ³è¦åŽŸç”Ÿé›†æˆï¼Œè¯·æäº¤åŠŸèƒ½è¯·æ±‚æˆ–æž„å»ºä¸€ä¸ªé’ˆå¯¹è¿™äº› API çš„ Skillsã€‚

å®‰è£… Skillsï¼š

```bash
clawhub install <skill-slug>
clawhub update --all
```

ClawHub å®‰è£…åˆ°å½“å‰ç›®å½•ä¸‹çš„ `./skills`ï¼ˆæˆ–å›žé€€åˆ°ä½ é…ç½®çš„ KrabKrab å·¥ä½œåŒºï¼‰ï¼›KrabKrab åœ¨ä¸‹ä¸€ä¸ªä¼šè¯ä¸­å°†å…¶è§†ä¸º `<workspace>/skills`ã€‚å¯¹äºŽè·¨æ™ºèƒ½ä½“å…±äº«çš„ Skillsï¼Œå°†å®ƒä»¬æ”¾åœ¨ `~/.krabkrab/skills/<name>/SKILL.md`ã€‚æŸäº› Skills æœŸæœ›é€šè¿‡ Homebrew å®‰è£…äºŒè¿›åˆ¶æ–‡ä»¶ï¼›åœ¨ Linux ä¸Šæ„å‘³ç€ Linuxbrewï¼ˆå‚é˜…ä¸Šé¢çš„ Homebrew Linux å¸¸è§é—®é¢˜æ¡ç›®ï¼‰ã€‚å‚é˜…[Skills](/tools/skills)å’Œ [ClawHub](/tools/clawhub)ã€‚

### å¦‚ä½•å®‰è£…ç”¨äºŽæµè§ˆå™¨æŽ¥ç®¡çš„ Chrome æ‰©å±•

ä½¿ç”¨å†…ç½®å®‰è£…ç¨‹åºï¼Œç„¶åŽåœ¨ Chrome ä¸­åŠ è½½æœªæ‰“åŒ…çš„æ‰©å±•ï¼š

```bash
krabkrab browser extension install
krabkrab browser extension path
```

ç„¶åŽ Chrome â†’ `chrome://extensions` â†’ å¯ç”¨â€œå¼€å‘è€…æ¨¡å¼â€ â†’ â€œåŠ è½½å·²è§£åŽ‹çš„æ‰©å±•ç¨‹åºâ€ â†’ é€‰æ‹©è¯¥æ–‡ä»¶å¤¹ã€‚

å®Œæ•´æŒ‡å—ï¼ˆåŒ…æ‹¬è¿œç¨‹ Gateway ç½‘å…³ + å®‰å…¨æ³¨æ„äº‹é¡¹ï¼‰ï¼š[Chrome æ‰©å±•](/tools/chrome-extension)

å¦‚æžœ Gateway ç½‘å…³è¿è¡Œåœ¨ä¸Ž Chrome åŒä¸€å°æœºå™¨ä¸Šï¼ˆé»˜è®¤è®¾ç½®ï¼‰ï¼Œä½ é€šå¸¸**ä¸éœ€è¦**é¢å¤–é…ç½®ã€‚
å¦‚æžœ Gateway ç½‘å…³è¿è¡Œåœ¨å…¶ä»–åœ°æ–¹ï¼Œåœ¨è¿è¡Œæµè§ˆå™¨çš„æœºå™¨ä¸Šè¿è¡Œä¸€ä¸ªèŠ‚ç‚¹ä¸»æœºï¼Œä»¥ä¾¿ Gateway ç½‘å…³å¯ä»¥ä»£ç†æµè§ˆå™¨æ“ä½œã€‚
ä½ ä»ç„¶éœ€è¦åœ¨è¦æŽ§åˆ¶çš„æ ‡ç­¾é¡µä¸Šç‚¹å‡»æ‰©å±•æŒ‰é’®ï¼ˆå®ƒä¸ä¼šè‡ªåŠ¨é™„åŠ ï¼‰ã€‚

## æ²™ç®±ä¸Žè®°å¿†

### æœ‰ä¸“é—¨çš„æ²™ç®±æ–‡æ¡£å—

æœ‰ã€‚å‚é˜…[æ²™ç®±](/gateway/sandboxing)ã€‚å¯¹äºŽ Docker ç‰¹å®šè®¾ç½®ï¼ˆå®Œæ•´ Gateway ç½‘å…³åœ¨ Docker ä¸­æˆ–æ²™ç®±é•œåƒï¼‰ï¼Œå‚é˜… [Docker](/install/docker)ã€‚

**èƒ½å¦è®©ç§ä¿¡ä¿æŒç§å¯†ï¼Œä½†ç¾¤ç»„ç”¨ä¸€ä¸ªæ™ºèƒ½ä½“å…¬å¼€æ²™ç®±éš”ç¦»**

å¯ä»¥â€”â€”å¦‚æžœä½ çš„ç§å¯†æµé‡æ˜¯**ç§ä¿¡**è€Œå…¬å¼€æµé‡æ˜¯**ç¾¤ç»„**ã€‚

ä½¿ç”¨ `agents.defaults.sandbox.mode: "non-main"`ï¼Œè¿™æ ·ç¾¤ç»„/é¢‘é“ä¼šè¯ï¼ˆéžä¸»é”®ï¼‰åœ¨ Docker ä¸­è¿è¡Œï¼Œè€Œä¸»ç§ä¿¡ä¼šè¯ä¿æŒåœ¨ä¸»æœºä¸Šã€‚ç„¶åŽé€šè¿‡ `tools.sandbox.tools` é™åˆ¶æ²™ç®±ä¼šè¯ä¸­å¯ç”¨çš„å·¥å…·ã€‚

è®¾ç½®æŒ‡å— + ç¤ºä¾‹é…ç½®ï¼š[ç¾¤ç»„ï¼šä¸ªäººç§ä¿¡ + å…¬å¼€ç¾¤ç»„](/channels/groups#pattern-personal-dms-public-groups-single-agent)

å…³é”®é…ç½®å‚è€ƒï¼š[Gateway ç½‘å…³é…ç½®](/gateway/configuration#agentsdefaultssandbox)

### å¦‚ä½•å°†ä¸»æœºæ–‡ä»¶å¤¹ç»‘å®šåˆ°æ²™ç®±ä¸­

å°† `agents.defaults.sandbox.docker.binds` è®¾ç½®ä¸º `["host:path:mode"]`ï¼ˆä¾‹å¦‚ `"/home/user/src:/src:ro"`ï¼‰ã€‚å…¨å±€ + æŒ‰æ™ºèƒ½ä½“çš„ç»‘å®šä¼šåˆå¹¶ï¼›å½“ `scope: "shared"` æ—¶æŒ‰æ™ºèƒ½ä½“çš„ç»‘å®šä¼šè¢«å¿½ç•¥ã€‚å¯¹äºŽæ•æ„Ÿå†…å®¹ä½¿ç”¨ `:ro`ï¼Œå¹¶è®°ä½ç»‘å®šä¼šç»•è¿‡æ²™ç®±æ–‡ä»¶ç³»ç»Ÿéš”ç¦»ã€‚å‚é˜…[æ²™ç®±](/gateway/sandboxing#custom-bind-mounts)å’Œ[æ²™ç®± vs å·¥å…·ç­–ç•¥ vs ææƒ](/gateway/sandbox-vs-tool-policy-vs-elevated#bind-mounts-security-quick-check)äº†è§£ç¤ºä¾‹å’Œå®‰å…¨æ³¨æ„äº‹é¡¹ã€‚

### è®°å¿†æ˜¯å¦‚ä½•å·¥ä½œçš„

KrabKrab è®°å¿†å°±æ˜¯æ™ºèƒ½ä½“å·¥ä½œåŒºä¸­çš„ Markdown æ–‡ä»¶ï¼š

- æ¯æ—¥ç¬”è®°åœ¨ `memory/YYYY-MM-DD.md`
- ç²¾é€‰çš„é•¿æœŸç¬”è®°åœ¨ `MEMORY.md`ï¼ˆä»…é™ä¸»/ç§å¯†ä¼šè¯ï¼‰

KrabKrab è¿˜ä¼šè¿è¡Œ**é™é»˜çš„é¢„åŽ‹ç¼©è®°å¿†åˆ·æ–°**ï¼Œä»¥æé†’æ¨¡åž‹åœ¨è‡ªåŠ¨åŽ‹ç¼©ä¹‹å‰å†™å…¥æŒä¹…ç¬”è®°ã€‚è¿™åªåœ¨å·¥ä½œåŒºå¯å†™æ—¶è¿è¡Œï¼ˆåªè¯»æ²™ç®±ä¼šè·³è¿‡ï¼‰ã€‚å‚é˜…[è®°å¿†](/concepts/memory)ã€‚

### è®°å¿†æ€»æ˜¯é—å¿˜ï¼Œå¦‚ä½•è®©å®ƒæŒä¹…ä¿å­˜

è¦æ±‚æœºå™¨äºº**å°†äº‹å®žå†™å…¥è®°å¿†**ã€‚é•¿æœŸç¬”è®°å±žäºŽ `MEMORY.md`ï¼ŒçŸ­æœŸä¸Šä¸‹æ–‡æ”¾å…¥ `memory/YYYY-MM-DD.md`ã€‚

è¿™ä»ç„¶æ˜¯æˆ‘ä»¬æ­£åœ¨æ”¹è¿›çš„é¢†åŸŸã€‚æé†’æ¨¡åž‹å­˜å‚¨è®°å¿†ä¼šæœ‰å¸®åŠ©ï¼›å®ƒä¼šçŸ¥é“å¦‚ä½•æ“ä½œã€‚å¦‚æžœå®ƒæŒç»­é—å¿˜ï¼ŒéªŒè¯ Gateway ç½‘å…³æ¯æ¬¡è¿è¡Œæ—¶æ˜¯å¦ä½¿ç”¨ç›¸åŒçš„å·¥ä½œåŒºã€‚

æ–‡æ¡£ï¼š[è®°å¿†](/concepts/memory)ã€[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace)ã€‚

### è¯­ä¹‰è®°å¿†æœç´¢éœ€è¦ OpenAI API å¯†é’¥å—

åªæœ‰åœ¨ä½¿ç”¨ **OpenAI embeddings** æ—¶æ‰éœ€è¦ã€‚Codex OAuth è¦†ç›– chat/completions ä½†**ä¸**æŽˆäºˆ embeddings è®¿é—®æƒé™ï¼Œå› æ­¤**ä½¿ç”¨ Codex ç™»å½•ï¼ˆOAuth æˆ– Codex CLI ç™»å½•ï¼‰**å¯¹è¯­ä¹‰è®°å¿†æœç´¢æ²¡æœ‰å¸®åŠ©ã€‚OpenAI embeddings ä»ç„¶éœ€è¦çœŸæ­£çš„ API å¯†é’¥ï¼ˆ`OPENAI_API_KEY` æˆ– `models.providers.openai.apiKey`ï¼‰ã€‚

å¦‚æžœä½ æ²¡æœ‰æ˜Žç¡®è®¾ç½®æä¾›å•†ï¼ŒKrabKrab ä¼šåœ¨èƒ½è§£æž API å¯†é’¥ï¼ˆè®¤è¯é…ç½®æ–‡ä»¶ã€`models.providers.*.apiKey` æˆ–çŽ¯å¢ƒå˜é‡ï¼‰æ—¶è‡ªåŠ¨é€‰æ‹©æä¾›å•†ã€‚å¦‚æžœ OpenAI å¯†é’¥å¯è§£æžåˆ™ä¼˜å…ˆä½¿ç”¨ OpenAIï¼Œå¦åˆ™å¦‚æžœ Gemini å¯†é’¥å¯è§£æžåˆ™ä½¿ç”¨ Geminiã€‚å¦‚æžœä¸¤ä¸ªå¯†é’¥éƒ½ä¸å¯ç”¨ï¼Œè®°å¿†æœç´¢ä¿æŒç¦ç”¨ç›´åˆ°ä½ é…ç½®å®ƒã€‚å¦‚æžœä½ é…ç½®äº†æœ¬åœ°æ¨¡åž‹è·¯å¾„ä¸”å­˜åœ¨ï¼ŒKrabKrab ä¼˜å…ˆä½¿ç”¨ `local`ã€‚

å¦‚æžœä½ æ›´æƒ³ä¿æŒæœ¬åœ°è¿è¡Œï¼Œè®¾ç½® `memorySearch.provider = "local"`ï¼ˆå¯é€‰ `memorySearch.fallback = "none"`ï¼‰ã€‚å¦‚æžœä½ æƒ³ä½¿ç”¨ Gemini embeddingsï¼Œè®¾ç½® `memorySearch.provider = "gemini"` å¹¶æä¾› `GEMINI_API_KEY`ï¼ˆæˆ– `memorySearch.remote.apiKey`ï¼‰ã€‚æˆ‘ä»¬æ”¯æŒ **OpenAIã€Gemini æˆ–æœ¬åœ°** embedding æ¨¡åž‹â€”â€”å‚é˜…[è®°å¿†](/concepts/memory)äº†è§£è®¾ç½®è¯¦æƒ…ã€‚

### è®°å¿†æ˜¯å¦æ°¸ä¹…ä¿ç•™ï¼Ÿæœ‰ä»€ä¹ˆé™åˆ¶

è®°å¿†æ–‡ä»¶ä¿å­˜åœ¨ç£ç›˜ä¸Šï¼ŒæŒä¹…å­˜åœ¨ç›´åˆ°ä½ åˆ é™¤å®ƒä»¬ã€‚é™åˆ¶æ˜¯ä½ çš„å­˜å‚¨ç©ºé—´ï¼Œè€Œä¸æ˜¯æ¨¡åž‹ã€‚**ä¼šè¯ä¸Šä¸‹æ–‡**ä»ç„¶å—æ¨¡åž‹ä¸Šä¸‹æ–‡çª—å£é™åˆ¶ï¼Œæ‰€ä»¥é•¿å¯¹è¯å¯èƒ½ä¼šåŽ‹ç¼©æˆ–æˆªæ–­ã€‚è¿™å°±æ˜¯è®°å¿†æœç´¢å­˜åœ¨çš„åŽŸå› â€”â€”å®ƒåªå°†ç›¸å…³éƒ¨åˆ†æ‹‰å›žä¸Šä¸‹æ–‡ã€‚

æ–‡æ¡£ï¼š[è®°å¿†](/concepts/memory)ã€[ä¸Šä¸‹æ–‡](/concepts/context)ã€‚

## ç£ç›˜ä¸Šçš„æ–‡ä»¶ä½ç½®

### KrabKrab ä½¿ç”¨çš„æ‰€æœ‰æ•°æ®éƒ½ä¿å­˜åœ¨æœ¬åœ°å—

ä¸æ˜¯â€”â€”**KrabKrab çš„çŠ¶æ€æ˜¯æœ¬åœ°çš„**ï¼Œä½†**å¤–éƒ¨æœåŠ¡ä»ç„¶ä¼šçœ‹åˆ°ä½ å‘é€ç»™å®ƒä»¬çš„å†…å®¹**ã€‚

- **é»˜è®¤æœ¬åœ°ï¼š** ä¼šè¯ã€è®°å¿†æ–‡ä»¶ã€é…ç½®å’Œå·¥ä½œåŒºä½äºŽ Gateway ç½‘å…³ä¸»æœºä¸Šï¼ˆ`~/.krabkrab` + ä½ çš„å·¥ä½œåŒºç›®å½•ï¼‰ã€‚
- **å¿…ç„¶è¿œç¨‹ï¼š** ä½ å‘é€ç»™æ¨¡åž‹æä¾›å•†ï¼ˆAnthropic/OpenAI/ç­‰ï¼‰çš„æ¶ˆæ¯ä¼šå‘é€åˆ°å®ƒä»¬çš„ APIï¼ŒèŠå¤©å¹³å°ï¼ˆWhatsApp/Telegram/Slack/ç­‰ï¼‰åœ¨å®ƒä»¬çš„æœåŠ¡å™¨ä¸Šå­˜å‚¨æ¶ˆæ¯æ•°æ®ã€‚
- **ä½ æŽ§åˆ¶èŒƒå›´ï¼š** ä½¿ç”¨æœ¬åœ°æ¨¡åž‹å¯ä»¥å°†æç¤ºä¿ç•™åœ¨ä½ çš„æœºå™¨ä¸Šï¼Œä½†æ¸ é“æµé‡ä»ç„¶é€šè¿‡æ¸ é“çš„æœåŠ¡å™¨ã€‚

ç›¸å…³ï¼š[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace)ã€[è®°å¿†](/concepts/memory)ã€‚

### KrabKrab å°†æ•°æ®å­˜å‚¨åœ¨å“ªé‡Œ

æ‰€æœ‰å†…å®¹ä½äºŽ `$krabkrab_STATE_DIR`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab`ï¼‰ä¸‹ï¼š

| è·¯å¾„                                                            | ç”¨é€”                                                 |
| --------------------------------------------------------------- | ---------------------------------------------------- |
| `$krabkrab_STATE_DIR/krabkrab.json`                             | ä¸»é…ç½®ï¼ˆJSON5ï¼‰                                      |
| `$krabkrab_STATE_DIR/credentials/oauth.json`                    | æ—§ç‰ˆ OAuth å¯¼å…¥ï¼ˆé¦–æ¬¡ä½¿ç”¨æ—¶å¤åˆ¶åˆ°è®¤è¯é…ç½®æ–‡ä»¶ï¼‰      |
| `$krabkrab_STATE_DIR/agents/<agentId>/agent/auth-profiles.json` | è®¤è¯é…ç½®æ–‡ä»¶ï¼ˆOAuth + API å¯†é’¥ï¼‰                     |
| `$krabkrab_STATE_DIR/agents/<agentId>/agent/auth.json`          | è¿è¡Œæ—¶è®¤è¯ç¼“å­˜ï¼ˆè‡ªåŠ¨ç®¡ç†ï¼‰                           |
| `$krabkrab_STATE_DIR/credentials/`                              | æä¾›å•†çŠ¶æ€ï¼ˆä¾‹å¦‚ `whatsapp/<accountId>/creds.json`ï¼‰ |
| `$krabkrab_STATE_DIR/agents/`                                   | æŒ‰æ™ºèƒ½ä½“çš„çŠ¶æ€ï¼ˆagentDir + ä¼šè¯ï¼‰                    |
| `$krabkrab_STATE_DIR/agents/<agentId>/sessions/`                | å¯¹è¯åŽ†å²å’ŒçŠ¶æ€ï¼ˆæŒ‰æ™ºèƒ½ä½“ï¼‰                           |
| `$krabkrab_STATE_DIR/agents/<agentId>/sessions/sessions.json`   | ä¼šè¯å…ƒæ•°æ®ï¼ˆæŒ‰æ™ºèƒ½ä½“ï¼‰                               |

æ—§ç‰ˆå•æ™ºèƒ½ä½“è·¯å¾„ï¼š`~/.krabkrab/agent/*`ï¼ˆé€šè¿‡ `krabkrab doctor` è¿ç§»ï¼‰ã€‚

ä½ çš„**å·¥ä½œåŒº**ï¼ˆAGENTS.mdã€è®°å¿†æ–‡ä»¶ã€Skills ç­‰ï¼‰æ˜¯ç‹¬ç«‹çš„ï¼Œé€šè¿‡ `agents.defaults.workspace` é…ç½®ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/workspace`ï¼‰ã€‚

### AGENTS.md / SOUL.md / USER.md / MEMORY.md åº”è¯¥æ”¾åœ¨å“ªé‡Œ

è¿™äº›æ–‡ä»¶ä½äºŽ**æ™ºèƒ½ä½“å·¥ä½œåŒº**ä¸­ï¼Œè€Œä¸æ˜¯ `~/.krabkrab`ã€‚

- **å·¥ä½œåŒºï¼ˆæŒ‰æ™ºèƒ½ä½“ï¼‰**ï¼š`AGENTS.md`ã€`SOUL.md`ã€`IDENTITY.md`ã€`USER.md`ã€
  `MEMORY.md`ï¼ˆæˆ– `memory.md`ï¼‰ã€`memory/YYYY-MM-DD.md`ã€å¯é€‰çš„ `HEARTBEAT.md`ã€‚
- **çŠ¶æ€ç›®å½•ï¼ˆ`~/.krabkrab`ï¼‰**ï¼šé…ç½®ã€å‡­æ®ã€è®¤è¯é…ç½®æ–‡ä»¶ã€ä¼šè¯ã€æ—¥å¿—å’Œå…±äº« Skillsï¼ˆ`~/.krabkrab/skills`ï¼‰ã€‚

é»˜è®¤å·¥ä½œåŒºæ˜¯ `~/.krabkrab/workspace`ï¼Œå¯é€šè¿‡ä»¥ä¸‹æ–¹å¼é…ç½®ï¼š

```json5
{
  agents: { defaults: { workspace: "~/.krabkrab/workspace" } },
}
```

å¦‚æžœæœºå™¨äººåœ¨é‡å¯åŽâ€œå¿˜è®°â€äº†å†…å®¹ï¼Œç¡®è®¤ Gateway ç½‘å…³æ¯æ¬¡å¯åŠ¨æ—¶éƒ½ä½¿ç”¨ç›¸åŒçš„å·¥ä½œåŒºï¼ˆè®°ä½ï¼šè¿œç¨‹æ¨¡å¼ä½¿ç”¨ **Gateway ç½‘å…³ä¸»æœºçš„**å·¥ä½œåŒºï¼Œè€Œä¸æ˜¯ä½ æœ¬åœ°ç¬”è®°æœ¬çš„ï¼‰ã€‚

æç¤ºï¼šå¦‚æžœä½ æƒ³è¦ä¸€ä¸ªæŒä¹…çš„è¡Œä¸ºæˆ–åå¥½ï¼Œè¦æ±‚æœºå™¨äºº**å°†å…¶å†™å…¥ AGENTS.md æˆ– MEMORY.md**ï¼Œè€Œä¸æ˜¯ä¾èµ–èŠå¤©åŽ†å²ã€‚

å‚é˜…[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace)å’Œ[è®°å¿†](/concepts/memory)ã€‚

### æŽ¨èçš„å¤‡ä»½ç­–ç•¥æ˜¯ä»€ä¹ˆ

å°†ä½ çš„**æ™ºèƒ½ä½“å·¥ä½œåŒº**æ”¾å…¥ä¸€ä¸ª**ç§æœ‰** git ä»“åº“ï¼Œå¹¶å¤‡ä»½åˆ°æŸä¸ªç§æœ‰ä½ç½®ï¼ˆä¾‹å¦‚ GitHub ç§æœ‰ä»“åº“ï¼‰ã€‚è¿™ä¼šæ•èŽ·è®°å¿† + AGENTS/SOUL/USER æ–‡ä»¶ï¼Œè®©ä½ ä»¥åŽå¯ä»¥æ¢å¤åŠ©æ‰‹çš„â€œæ€ç»´â€ã€‚

**ä¸è¦**æäº¤ `~/.krabkrab` ä¸‹çš„ä»»ä½•å†…å®¹ï¼ˆå‡­æ®ã€ä¼šè¯ã€ä»¤ç‰Œï¼‰ã€‚å¦‚æžœä½ éœ€è¦å®Œæ•´æ¢å¤ï¼Œå°†å·¥ä½œåŒºå’ŒçŠ¶æ€ç›®å½•åˆ†åˆ«å¤‡ä»½ï¼ˆå‚é˜…ä¸Šé¢çš„è¿ç§»é—®é¢˜ï¼‰ã€‚

æ–‡æ¡£ï¼š[æ™ºèƒ½ä½“å·¥ä½œåŒº](/concepts/agent-workspace)ã€‚

### å¦‚ä½•å®Œå…¨å¸è½½ KrabKrab

å‚é˜…ä¸“é—¨æŒ‡å—ï¼š[å¸è½½](/install/uninstall)ã€‚

### æ™ºèƒ½ä½“å¯ä»¥åœ¨å·¥ä½œåŒºå¤–å·¥ä½œå—

å¯ä»¥ã€‚å·¥ä½œåŒºæ˜¯**é»˜è®¤ cwd** å’Œè®°å¿†é”šç‚¹ï¼Œä¸æ˜¯ç¡¬æ²™ç®±ã€‚ç›¸å¯¹è·¯å¾„åœ¨å·¥ä½œåŒºå†…è§£æžï¼Œä½†ç»å¯¹è·¯å¾„å¯ä»¥è®¿é—®å…¶ä»–ä¸»æœºä½ç½®ï¼Œé™¤éžå¯ç”¨äº†æ²™ç®±ã€‚å¦‚æžœä½ éœ€è¦éš”ç¦»ï¼Œä½¿ç”¨ [`agents.defaults.sandbox`](/gateway/sandboxing) æˆ–æŒ‰æ™ºèƒ½ä½“çš„æ²™ç®±è®¾ç½®ã€‚å¦‚æžœä½ å¸Œæœ›æŸä¸ªä»“åº“ä½œä¸ºé»˜è®¤å·¥ä½œç›®å½•ï¼Œå°†è¯¥æ™ºèƒ½ä½“çš„ `workspace` æŒ‡å‘ä»“åº“æ ¹ç›®å½•ã€‚KrabKrab ä»“åº“åªæ˜¯æºä»£ç ï¼›é™¤éžä½ æœ‰æ„è¦è®©æ™ºèƒ½ä½“åœ¨å…¶ä¸­å·¥ä½œï¼Œå¦åˆ™ä¿æŒå·¥ä½œåŒºç‹¬ç«‹ã€‚

ç¤ºä¾‹ï¼ˆä»“åº“ä½œä¸ºé»˜è®¤ cwdï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      workspace: "~/Projects/my-repo",
    },
  },
}
```

### æˆ‘å¤„äºŽè¿œç¨‹æ¨¡å¼â€”â€”ä¼šè¯å­˜å‚¨åœ¨å“ªé‡Œ

ä¼šè¯çŠ¶æ€å½’ **Gateway ç½‘å…³ä¸»æœº**æ‰€æœ‰ã€‚å¦‚æžœä½ å¤„äºŽè¿œç¨‹æ¨¡å¼ï¼Œä½ å…³å¿ƒçš„ä¼šè¯å­˜å‚¨åœ¨è¿œç¨‹æœºå™¨ä¸Šï¼Œè€Œä¸æ˜¯ä½ çš„æœ¬åœ°ç¬”è®°æœ¬ä¸Šã€‚å‚é˜…[ä¼šè¯ç®¡ç†](/concepts/session)ã€‚

## é…ç½®åŸºç¡€

### é…ç½®æ–‡ä»¶æ˜¯ä»€ä¹ˆæ ¼å¼ï¼Ÿåœ¨å“ªé‡Œ

KrabKrab ä»Ž `$krabkrab_CONFIG_PATH`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/krabkrab.json`ï¼‰è¯»å–å¯é€‰çš„ **JSON5** é…ç½®ï¼š

```
$krabkrab_CONFIG_PATH
```

å¦‚æžœæ–‡ä»¶ä¸å­˜åœ¨ï¼Œä½¿ç”¨å®‰å…¨çš„é»˜è®¤å€¼ï¼ˆåŒ…æ‹¬é»˜è®¤å·¥ä½œåŒº `~/.krabkrab/workspace`ï¼‰ã€‚

### æˆ‘è®¾ç½®äº† gateway.bind: "lan"ï¼ˆæˆ– "tailnet"ï¼‰ï¼ŒçŽ°åœ¨ä»€ä¹ˆéƒ½ç›‘å¬ä¸äº† / UI æ˜¾ç¤ºæœªæŽˆæƒ

éž local loopback ç»‘å®š**éœ€è¦è®¤è¯**ã€‚é…ç½® `gateway.auth.mode` + `gateway.auth.token`ï¼ˆæˆ–ä½¿ç”¨ `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚

```json5
{
  gateway: {
    bind: "lan",
    auth: {
      mode: "token",
      token: "replace-me",
    },
  },
}
```

æ³¨æ„ï¼š

- `gateway.remote.token` ä»…ç”¨äºŽ**è¿œç¨‹ CLI è°ƒç”¨**ï¼›å®ƒä¸å¯ç”¨æœ¬åœ° Gateway ç½‘å…³è®¤è¯ã€‚
- æŽ§åˆ¶ UI é€šè¿‡ `connect.params.auth.token`ï¼ˆå­˜å‚¨åœ¨åº”ç”¨/UI è®¾ç½®ä¸­ï¼‰è¿›è¡Œè®¤è¯ã€‚é¿å…å°†ä»¤ç‰Œæ”¾åœ¨ URL ä¸­ã€‚

### ä¸ºä»€ä¹ˆçŽ°åœ¨åœ¨ localhost ä¹Ÿéœ€è¦ä»¤ç‰Œ

å‘å¯¼é»˜è®¤ç”Ÿæˆ Gateway ç½‘å…³ä»¤ç‰Œï¼ˆå³ä½¿åœ¨ local loopback ä¸Šï¼‰ï¼Œå› æ­¤**æœ¬åœ° WS å®¢æˆ·ç«¯å¿…é¡»è®¤è¯**ã€‚è¿™é˜»æ­¢äº†å…¶ä»–æœ¬åœ°è¿›ç¨‹è°ƒç”¨ Gateway ç½‘å…³ã€‚åœ¨æŽ§åˆ¶ UI è®¾ç½®ï¼ˆæˆ–ä½ çš„å®¢æˆ·ç«¯é…ç½®ï¼‰ä¸­ç²˜è´´ä»¤ç‰Œä»¥è¿žæŽ¥ã€‚

å¦‚æžœä½ **ç¡®å®ž**æƒ³è¦å¼€æ”¾ local loopbackï¼Œä»Žé…ç½®ä¸­ç§»é™¤ `gateway.auth`ã€‚Doctor å¯ä»¥éšæ—¶ä¸ºä½ ç”Ÿæˆä»¤ç‰Œï¼š`krabkrab doctor --generate-gateway-token`ã€‚

### æ›´æ”¹é…ç½®åŽéœ€è¦é‡å¯å—

Gateway ç½‘å…³ç›‘è§†é…ç½®æ–‡ä»¶å¹¶æ”¯æŒçƒ­é‡è½½ï¼š

- `gateway.reload.mode: "hybrid"`ï¼ˆé»˜è®¤ï¼‰ï¼šå®‰å…¨æ›´æ”¹çƒ­åº”ç”¨ï¼Œå…³é”®æ›´æ”¹é‡å¯
- ä¹Ÿæ”¯æŒ `hot`ã€`restart`ã€`off`

### å¦‚ä½•å¯ç”¨ç½‘ç»œæœç´¢ï¼ˆå’Œç½‘é¡µæŠ“å–ï¼‰

`web_fetch` æ— éœ€ API å¯†é’¥å³å¯å·¥ä½œã€‚`web_search` éœ€è¦ Brave Search API å¯†é’¥ã€‚**æŽ¨èï¼š** è¿è¡Œ `krabkrab configure --section web` å°†å…¶å­˜å‚¨åœ¨ `tools.web.search.apiKey` ä¸­ã€‚çŽ¯å¢ƒå˜é‡æ›¿ä»£æ–¹æ¡ˆï¼šä¸º Gateway ç½‘å…³è¿›ç¨‹è®¾ç½® `BRAVE_API_KEY`ã€‚

```json5
{
  tools: {
    web: {
      search: {
        enabled: true,
        apiKey: "BRAVE_API_KEY_HERE",
        maxResults: 5,
      },
      fetch: {
        enabled: true,
      },
    },
  },
}
```

æ³¨æ„ï¼š

- å¦‚æžœä½ ä½¿ç”¨å…è®¸åˆ—è¡¨ï¼Œæ·»åŠ  `web_search`/`web_fetch` æˆ– `group:web`ã€‚
- `web_fetch` é»˜è®¤å¯ç”¨ï¼ˆé™¤éžæ˜Žç¡®ç¦ç”¨ï¼‰ã€‚
- å®ˆæŠ¤è¿›ç¨‹ä»Ž `~/.krabkrab/.env`ï¼ˆæˆ–æœåŠ¡çŽ¯å¢ƒï¼‰è¯»å–çŽ¯å¢ƒå˜é‡ã€‚

æ–‡æ¡£ï¼š[Web å·¥å…·](/tools/web)ã€‚

### config.apply æ¸…ç©ºäº†æˆ‘çš„é…ç½®ï¼Œå¦‚ä½•æ¢å¤å’Œé¿å…

`config.apply` æ›¿æ¢**æ•´ä¸ªé…ç½®**ã€‚å¦‚æžœä½ å‘é€éƒ¨åˆ†å¯¹è±¡ï¼Œå…¶ä»–æ‰€æœ‰å†…å®¹éƒ½ä¼šè¢«ç§»é™¤ã€‚

æ¢å¤ï¼š

- ä»Žå¤‡ä»½æ¢å¤ï¼ˆgit æˆ–å¤åˆ¶çš„ `~/.krabkrab/krabkrab.json`ï¼‰ã€‚
- å¦‚æžœæ²¡æœ‰å¤‡ä»½ï¼Œé‡æ–°è¿è¡Œ `krabkrab doctor` å¹¶é‡æ–°é…ç½®æ¸ é“/æ¨¡åž‹ã€‚
- å¦‚æžœè¿™æ˜¯æ„å¤–æƒ…å†µï¼Œæäº¤ bug å¹¶é™„ä¸Šä½ æœ€åŽå·²çŸ¥çš„é…ç½®æˆ–ä»»ä½•å¤‡ä»½ã€‚
- æœ¬åœ°ç¼–ç æ™ºèƒ½ä½“é€šå¸¸å¯ä»¥ä»Žæ—¥å¿—æˆ–åŽ†å²ä¸­é‡å»ºå·¥ä½œé…ç½®ã€‚

é¿å…æ–¹æ³•ï¼š

- å¯¹å°æ›´æ”¹ä½¿ç”¨ `krabkrab config set`ã€‚
- å¯¹äº¤äº’å¼ç¼–è¾‘ä½¿ç”¨ `krabkrab configure`ã€‚

æ–‡æ¡£ï¼š[Config](/cli/config)ã€[Configure](/cli/configure)ã€[Doctor](/gateway/doctor)ã€‚

### å¦‚ä½•è¿è¡Œä¸€ä¸ªä¸­å¿ƒ Gateway ç½‘å…³é…åˆè·¨è®¾å¤‡çš„ä¸“ç”¨å·¥ä½œèŠ‚ç‚¹

å¸¸è§æ¨¡å¼æ˜¯**ä¸€ä¸ª Gateway ç½‘å…³**ï¼ˆä¾‹å¦‚ Raspberry Piï¼‰åŠ ä¸Š**èŠ‚ç‚¹**å’Œ**æ™ºèƒ½ä½“**ï¼š

- **Gateway ç½‘å…³ï¼ˆä¸­å¿ƒï¼‰ï¼š** æ‹¥æœ‰æ¸ é“ï¼ˆSignal/WhatsAppï¼‰ã€è·¯ç”±å’Œä¼šè¯ã€‚
- **èŠ‚ç‚¹ï¼ˆè®¾å¤‡ï¼‰ï¼š** Mac/iOS/Android ä½œä¸ºå¤–å›´è®¾å¤‡è¿žæŽ¥ï¼Œæš´éœ²æœ¬åœ°å·¥å…·ï¼ˆ`system.run`ã€`canvas`ã€`camera`ï¼‰ã€‚
- **æ™ºèƒ½ä½“ï¼ˆå·¥ä½œè€…ï¼‰ï¼š** ç”¨äºŽç‰¹æ®Šè§’è‰²çš„ç‹¬ç«‹å¤§è„‘/å·¥ä½œåŒºï¼ˆä¾‹å¦‚â€œHetzner è¿ç»´â€ã€â€œä¸ªäººæ•°æ®â€ï¼‰ã€‚
- **å­æ™ºèƒ½ä½“ï¼š** éœ€è¦å¹¶è¡Œå¤„ç†æ—¶ä»Žä¸»æ™ºèƒ½ä½“ç”ŸæˆåŽå°å·¥ä½œã€‚
- **TUIï¼š** è¿žæŽ¥åˆ° Gateway ç½‘å…³å¹¶åˆ‡æ¢æ™ºèƒ½ä½“/ä¼šè¯ã€‚

æ–‡æ¡£ï¼š[èŠ‚ç‚¹](/nodes)ã€[è¿œç¨‹è®¿é—®](/gateway/remote)ã€[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ã€[å­æ™ºèƒ½ä½“](/tools/subagents)ã€[TUI](/web/tui)ã€‚

### KrabKrab æµè§ˆå™¨å¯ä»¥æ— å¤´è¿è¡Œå—

å¯ä»¥ã€‚è¿™æ˜¯ä¸€ä¸ªé…ç½®é€‰é¡¹ï¼š

```json5
{
  browser: { headless: true },
  agents: {
    defaults: {
      sandbox: { browser: { headless: true } },
    },
  },
}
```

é»˜è®¤ä¸º `false`ï¼ˆæœ‰å¤´ï¼‰ã€‚æ— å¤´æ¨¡å¼åœ¨æŸäº›ç½‘ç«™ä¸Šæ›´å®¹æ˜“è§¦å‘åæœºå™¨äººæ£€æµ‹ã€‚å‚é˜…[æµè§ˆå™¨](/tools/browser)ã€‚

æ— å¤´æ¨¡å¼ä½¿ç”¨**ç›¸åŒçš„ Chromium å¼•æ“Ž**ï¼Œé€‚ç”¨äºŽå¤§å¤šæ•°è‡ªåŠ¨åŒ–ï¼ˆè¡¨å•ã€ç‚¹å‡»ã€æŠ“å–ã€ç™»å½•ï¼‰ã€‚ä¸»è¦åŒºåˆ«ï¼š

- æ²¡æœ‰å¯è§çš„æµè§ˆå™¨çª—å£ï¼ˆå¦‚æžœéœ€è¦è§†è§‰æ•ˆæžœä½¿ç”¨æˆªå›¾ï¼‰ã€‚
- æŸäº›ç½‘ç«™åœ¨æ— å¤´æ¨¡å¼ä¸‹å¯¹è‡ªåŠ¨åŒ–æ›´ä¸¥æ ¼ï¼ˆéªŒè¯ç ã€åæœºå™¨äººï¼‰ã€‚ä¾‹å¦‚ï¼ŒX/Twitter ç»å¸¸é˜»æ­¢æ— å¤´ä¼šè¯ã€‚

### å¦‚ä½•ä½¿ç”¨ Brave è¿›è¡Œæµè§ˆå™¨æŽ§åˆ¶

å°† `browser.executablePath` è®¾ç½®ä¸ºä½ çš„ Brave äºŒè¿›åˆ¶æ–‡ä»¶ï¼ˆæˆ–ä»»ä½•åŸºäºŽ Chromium çš„æµè§ˆå™¨ï¼‰å¹¶é‡å¯ Gateway ç½‘å…³ã€‚
å‚é˜…[æµè§ˆå™¨](/tools/browser#use-brave-or-another-chromium-based-browser)ä¸­çš„å®Œæ•´é…ç½®ç¤ºä¾‹ã€‚

## è¿œç¨‹ Gateway ç½‘å…³ä¸ŽèŠ‚ç‚¹

### å‘½ä»¤å¦‚ä½•åœ¨ Telegramã€Gateway ç½‘å…³å’ŒèŠ‚ç‚¹ä¹‹é—´ä¼ æ’­

Telegram æ¶ˆæ¯ç”± **Gateway ç½‘å…³** å¤„ç†ã€‚Gateway ç½‘å…³è¿è¡Œæ™ºèƒ½ä½“ï¼Œåªæœ‰åœ¨éœ€è¦èŠ‚ç‚¹å·¥å…·æ—¶æ‰é€šè¿‡ **Gateway ç½‘å…³ WebSocket** è°ƒç”¨èŠ‚ç‚¹ï¼š

Telegram â†’ Gateway ç½‘å…³ â†’ æ™ºèƒ½ä½“ â†’ `node.*` â†’ èŠ‚ç‚¹ â†’ Gateway ç½‘å…³ â†’ Telegram

èŠ‚ç‚¹ä¸ä¼šçœ‹åˆ°å…¥ç«™æä¾›å•†æµé‡ï¼›å®ƒä»¬åªæŽ¥æ”¶èŠ‚ç‚¹ RPC è°ƒç”¨ã€‚

### å¦‚æžœ Gateway ç½‘å…³æ‰˜ç®¡åœ¨è¿œç¨‹ï¼Œæˆ‘çš„æ™ºèƒ½ä½“å¦‚ä½•è®¿é—®æˆ‘çš„ç”µè„‘

ç®€çŸ­å›žç­”ï¼š**å°†ä½ çš„ç”µè„‘é…å¯¹ä¸ºèŠ‚ç‚¹**ã€‚Gateway ç½‘å…³è¿è¡Œåœ¨å…¶ä»–åœ°æ–¹ï¼Œä½†å®ƒå¯ä»¥é€šè¿‡ Gateway ç½‘å…³ WebSocket åœ¨ä½ çš„æœ¬åœ°æœºå™¨ä¸Šè°ƒç”¨ `node.*` å·¥å…·ï¼ˆå±å¹•ã€æ‘„åƒå¤´ã€ç³»ç»Ÿï¼‰ã€‚

å…¸åž‹è®¾ç½®ï¼š

1. åœ¨å¸¸å¼€ä¸»æœºï¼ˆVPS/å®¶åº­æœåŠ¡å™¨ï¼‰ä¸Šè¿è¡Œ Gateway ç½‘å…³ã€‚
2. å°† Gateway ç½‘å…³ä¸»æœºå’Œä½ çš„ç”µè„‘æ”¾åœ¨åŒä¸€ä¸ª tailnet ä¸Šã€‚
3. ç¡®ä¿ Gateway ç½‘å…³ WS å¯è¾¾ï¼ˆtailnet ç»‘å®šæˆ– SSH éš§é“ï¼‰ã€‚
4. åœ¨æœ¬åœ°æ‰“å¼€ macOS åº”ç”¨å¹¶ä»¥**è¿œç¨‹ over SSH** æ¨¡å¼è¿žæŽ¥ï¼ˆæˆ–ç›´æŽ¥ tailnetï¼‰ï¼Œä½¿å…¶å¯ä»¥æ³¨å†Œä¸ºèŠ‚ç‚¹ã€‚
5. åœ¨ Gateway ç½‘å…³ä¸Šæ‰¹å‡†èŠ‚ç‚¹ï¼š
   ```bash
   krabkrab nodes pending
   krabkrab nodes approve <requestId>
   ```

ä¸éœ€è¦å•ç‹¬çš„ TCP æ¡¥æŽ¥ï¼›èŠ‚ç‚¹é€šè¿‡ Gateway ç½‘å…³ WebSocket è¿žæŽ¥ã€‚

å®‰å…¨æé†’ï¼šé…å¯¹ macOS èŠ‚ç‚¹å…è®¸åœ¨è¯¥æœºå™¨ä¸Šæ‰§è¡Œ `system.run`ã€‚åªé…å¯¹ä½ ä¿¡ä»»çš„è®¾å¤‡ï¼Œå¹¶æŸ¥é˜…[å®‰å…¨](/gateway/security)ã€‚

æ–‡æ¡£ï¼š[èŠ‚ç‚¹](/nodes)ã€[Gateway ç½‘å…³åè®®](/gateway/protocol)ã€[macOS è¿œç¨‹æ¨¡å¼](/platforms/mac/remote)ã€[å®‰å…¨](/gateway/security)ã€‚

### Tailscale å·²è¿žæŽ¥ä½†æ”¶ä¸åˆ°å›žå¤ï¼Œæ€Žä¹ˆåŠž

æ£€æŸ¥åŸºç¡€é¡¹ï¼š

- Gateway ç½‘å…³æ­£åœ¨è¿è¡Œï¼š`krabkrab gateway status`
- Gateway ç½‘å…³å¥åº·ï¼š`krabkrab status`
- æ¸ é“å¥åº·ï¼š`krabkrab channels status`

ç„¶åŽéªŒè¯è®¤è¯å’Œè·¯ç”±ï¼š

- å¦‚æžœä½ ä½¿ç”¨ Tailscale Serveï¼Œç¡®ä¿ `gateway.auth.allowTailscale` è®¾ç½®æ­£ç¡®ã€‚
- å¦‚æžœä½ é€šè¿‡ SSH éš§é“è¿žæŽ¥ï¼Œç¡®è®¤æœ¬åœ°éš§é“å·²å¯åŠ¨å¹¶æŒ‡å‘æ­£ç¡®ç«¯å£ã€‚
- ç¡®è®¤ä½ çš„å…è®¸åˆ—è¡¨ï¼ˆç§ä¿¡æˆ–ç¾¤ç»„ï¼‰åŒ…å«ä½ çš„è´¦æˆ·ã€‚

æ–‡æ¡£ï¼š[Tailscale](/gateway/tailscale)ã€[è¿œç¨‹è®¿é—®](/gateway/remote)ã€[æ¸ é“](/channels)ã€‚

### ä¸¤ä¸ª KrabKrab å®žä¾‹ï¼ˆæœ¬åœ° + VPSï¼‰å¯ä»¥äº’ç›¸é€šä¿¡å—

å¯ä»¥ã€‚æ²¡æœ‰å†…ç½®çš„â€œæœºå™¨äººå¯¹æœºå™¨äººâ€æ¡¥æŽ¥ï¼Œä½†ä½ å¯ä»¥é€šè¿‡å‡ ç§å¯é çš„æ–¹å¼å®žçŽ°ï¼š

**æœ€ç®€å•ï¼š** ä½¿ç”¨ä¸¤ä¸ªæœºå™¨äººéƒ½èƒ½è®¿é—®çš„æ™®é€šèŠå¤©æ¸ é“ï¼ˆTelegram/Slack/WhatsAppï¼‰ã€‚è®©æœºå™¨äºº A ç»™æœºå™¨äºº B å‘æ¶ˆæ¯ï¼Œç„¶åŽè®©æœºå™¨äºº B æ­£å¸¸å›žå¤ã€‚

**CLI æ¡¥æŽ¥ï¼ˆé€šç”¨ï¼‰ï¼š** è¿è¡Œä¸€ä¸ªè„šæœ¬è°ƒç”¨å¦ä¸€ä¸ª Gateway ç½‘å…³ï¼Œä½¿ç”¨ `krabkrab agent --message ... --deliver`ï¼Œå®šå‘åˆ°å¦ä¸€ä¸ªæœºå™¨äººç›‘å¬çš„èŠå¤©ã€‚å¦‚æžœä¸€ä¸ªæœºå™¨äººåœ¨è¿œç¨‹ VPS ä¸Šï¼Œé€šè¿‡ SSH/Tailscale å°†ä½ çš„ CLI æŒ‡å‘è¯¥è¿œç¨‹ Gateway ç½‘å…³ï¼ˆå‚é˜…[è¿œç¨‹è®¿é—®](/gateway/remote)ï¼‰ã€‚

ç¤ºä¾‹æ¨¡å¼ï¼ˆä»Žèƒ½åˆ°è¾¾ç›®æ ‡ Gateway ç½‘å…³çš„æœºå™¨ä¸Šè¿è¡Œï¼‰ï¼š

```bash
krabkrab agent --message "Hello from local bot" --deliver --channel telegram --reply-to <chat-id>
```

æç¤ºï¼šæ·»åŠ æŠ¤æ é˜²æ­¢ä¸¤ä¸ªæœºå™¨äººæ— é™å¾ªçŽ¯ï¼ˆä»…æåŠã€æ¸ é“å…è®¸åˆ—è¡¨æˆ–â€œä¸å›žå¤æœºå™¨äººæ¶ˆæ¯â€è§„åˆ™ï¼‰ã€‚

æ–‡æ¡£ï¼š[è¿œç¨‹è®¿é—®](/gateway/remote)ã€[Agent CLI](/cli/agent)ã€[Agent send](/tools/agent-send)ã€‚

### å¤šä¸ªæ™ºèƒ½ä½“éœ€è¦ç‹¬ç«‹çš„ VPS å—

ä¸éœ€è¦ã€‚ä¸€ä¸ª Gateway ç½‘å…³å¯ä»¥æ‰˜ç®¡å¤šä¸ªæ™ºèƒ½ä½“ï¼Œæ¯ä¸ªéƒ½æœ‰è‡ªå·±çš„å·¥ä½œåŒºã€æ¨¡åž‹é»˜è®¤å€¼å’Œè·¯ç”±ã€‚è¿™æ˜¯æ­£å¸¸è®¾ç½®ï¼Œæ¯”æ¯ä¸ªæ™ºèƒ½ä½“ä¸€ä¸ª VPS ä¾¿å®œä¸”ç®€å•å¾—å¤šã€‚

åªæœ‰åœ¨éœ€è¦ç¡¬éš”ç¦»ï¼ˆå®‰å…¨è¾¹ç•Œï¼‰æˆ–éžå¸¸ä¸åŒçš„é…ç½®ï¼ˆä½ ä¸æƒ³å…±äº«ï¼‰æ—¶æ‰ä½¿ç”¨ç‹¬ç«‹çš„ VPSã€‚å¦åˆ™ä¿æŒä¸€ä¸ª Gateway ç½‘å…³å¹¶ä½¿ç”¨å¤šä¸ªæ™ºèƒ½ä½“æˆ–å­æ™ºèƒ½ä½“ã€‚

### åœ¨ä¸ªäººç¬”è®°æœ¬ç”µè„‘ä¸Šä½¿ç”¨èŠ‚ç‚¹è€Œä¸æ˜¯ä»Ž VPS SSH æœ‰ä»€ä¹ˆå¥½å¤„

æœ‰â€”â€”èŠ‚ç‚¹æ˜¯ä»Žè¿œç¨‹ Gateway ç½‘å…³åˆ°è¾¾ä½ ç¬”è®°æœ¬çš„é¦–é€‰æ–¹å¼ï¼Œå®ƒä»¬è§£é”çš„ä¸ä»…ä»…æ˜¯ shell è®¿é—®ã€‚Gateway ç½‘å…³è¿è¡Œåœ¨ macOS/Linuxï¼ˆWindows é€šè¿‡ WSL2ï¼‰ä¸Šä¸”æ˜¯è½»é‡çº§çš„ï¼ˆå°åž‹ VPS æˆ– Raspberry Pi çº§åˆ«çš„è®¾å¤‡å°±å¤Ÿç”¨ï¼›4 GB RAM è¶³å¤Ÿï¼‰ï¼Œæ‰€ä»¥å¸¸è§è®¾ç½®æ˜¯ä¸€ä¸ªå¸¸å¼€ä¸»æœºåŠ ä¸Šä½ çš„ç¬”è®°æœ¬ä½œä¸ºèŠ‚ç‚¹ã€‚

- **æ— éœ€å…¥ç«™ SSHã€‚** èŠ‚ç‚¹å‘ Gateway ç½‘å…³ WebSocket å‘èµ·å‡ºç«™è¿žæŽ¥å¹¶ä½¿ç”¨è®¾å¤‡é…å¯¹ã€‚
- **æ›´å®‰å…¨çš„æ‰§è¡ŒæŽ§åˆ¶ã€‚** `system.run` å—è¯¥ç¬”è®°æœ¬ä¸ŠèŠ‚ç‚¹å…è®¸åˆ—è¡¨/å®¡æ‰¹çš„é™åˆ¶ã€‚
- **æ›´å¤šè®¾å¤‡å·¥å…·ã€‚** èŠ‚ç‚¹é™¤äº† `system.run` è¿˜æš´éœ² `canvas`ã€`camera` å’Œ `screen`ã€‚
- **æœ¬åœ°æµè§ˆå™¨è‡ªåŠ¨åŒ–ã€‚** å°† Gateway ç½‘å…³ä¿æŒåœ¨ VPS ä¸Šï¼Œä½†åœ¨æœ¬åœ°è¿è¡Œ Chrome å¹¶é€šè¿‡ Chrome æ‰©å±• + ç¬”è®°æœ¬ä¸Šçš„èŠ‚ç‚¹ä¸»æœºä¸­ç»§æŽ§åˆ¶ã€‚

SSH å¯¹ä¸´æ—¶ shell è®¿é—®å¾ˆå¥½ï¼Œä½†èŠ‚ç‚¹å¯¹äºŽæŒç»­çš„æ™ºèƒ½ä½“å·¥ä½œæµå’Œè®¾å¤‡è‡ªåŠ¨åŒ–æ›´ç®€å•ã€‚

æ–‡æ¡£ï¼š[èŠ‚ç‚¹](/nodes)ã€[èŠ‚ç‚¹ CLI](/cli/nodes)ã€[Chrome æ‰©å±•](/tools/chrome-extension)ã€‚

### åº”è¯¥åœ¨ç¬¬äºŒå°ç¬”è®°æœ¬ä¸Šå®‰è£…è¿˜æ˜¯åªæ·»åŠ ä¸€ä¸ªèŠ‚ç‚¹

å¦‚æžœä½ åªéœ€è¦ç¬¬äºŒå°ç¬”è®°æœ¬ä¸Šçš„**æœ¬åœ°å·¥å…·**ï¼ˆå±å¹•/æ‘„åƒå¤´/æ‰§è¡Œï¼‰ï¼Œå°†å…¶æ·»åŠ ä¸º**èŠ‚ç‚¹**ã€‚è¿™ä¿æŒå•ä¸€ Gateway ç½‘å…³å¹¶é¿å…é‡å¤é…ç½®ã€‚æœ¬åœ°èŠ‚ç‚¹å·¥å…·ç›®å‰ä»…é™ macOSï¼Œä½†æˆ‘ä»¬è®¡åˆ’æ‰©å±•åˆ°å…¶ä»–æ“ä½œç³»ç»Ÿã€‚

åªæœ‰åœ¨éœ€è¦**ç¡¬éš”ç¦»**æˆ–ä¸¤ä¸ªå®Œå…¨ç‹¬ç«‹çš„æœºå™¨äººæ—¶æ‰å®‰è£…ç¬¬äºŒä¸ª Gateway ç½‘å…³ã€‚

æ–‡æ¡£ï¼š[èŠ‚ç‚¹](/nodes)ã€[èŠ‚ç‚¹ CLI](/cli/nodes)ã€[å¤š Gateway ç½‘å…³](/gateway/multiple-gateways)ã€‚

### èŠ‚ç‚¹ä¼šè¿è¡Œ Gateway ç½‘å…³æœåŠ¡å—

ä¸ä¼šã€‚æ¯å°ä¸»æœºä¸Šåº”è¯¥åªè¿è¡Œ**ä¸€ä¸ª Gateway ç½‘å…³**ï¼Œé™¤éžä½ æœ‰æ„è¿è¡Œéš”ç¦»çš„é…ç½®æ–‡ä»¶ï¼ˆå‚é˜…[å¤š Gateway ç½‘å…³](/gateway/multiple-gateways)ï¼‰ã€‚èŠ‚ç‚¹æ˜¯è¿žæŽ¥åˆ° Gateway ç½‘å…³çš„å¤–å›´è®¾å¤‡ï¼ˆiOS/Android èŠ‚ç‚¹ï¼Œæˆ– macOS èœå•æ åº”ç”¨çš„â€œèŠ‚ç‚¹æ¨¡å¼â€ï¼‰ã€‚å¯¹äºŽæ— å¤´èŠ‚ç‚¹ä¸»æœºå’Œ CLI æŽ§åˆ¶ï¼Œå‚é˜…[èŠ‚ç‚¹ä¸»æœº CLI](/cli/node)ã€‚

`gateway`ã€`discovery` å’Œ `canvasHost` çš„æ›´æ”¹éœ€è¦å®Œå…¨é‡å¯ã€‚

### æœ‰ API / RPC æ–¹å¼æ¥åº”ç”¨é…ç½®å—

æœ‰ã€‚`config.apply` éªŒè¯ + å†™å…¥å®Œæ•´é…ç½®ï¼Œå¹¶åœ¨æ“ä½œè¿‡ç¨‹ä¸­é‡å¯ Gateway ç½‘å…³ã€‚

### é¦–æ¬¡å®‰è£…çš„æœ€å°â€œåˆç†â€é…ç½®æ˜¯ä»€ä¹ˆ

```json5
{
  agents: { defaults: { workspace: "~/.krabkrab/workspace" } },
  channels: { whatsapp: { allowFrom: ["+15555550123"] } },
}
```

è¿™è®¾ç½®äº†ä½ çš„å·¥ä½œåŒºå¹¶é™åˆ¶è°å¯ä»¥è§¦å‘æœºå™¨äººã€‚

### å¦‚ä½•åœ¨ VPS ä¸Šè®¾ç½® Tailscale å¹¶ä»Ž Mac è¿žæŽ¥

æœ€ç®€æ­¥éª¤ï¼š

1. **åœ¨ VPS ä¸Šå®‰è£…å¹¶ç™»å½•**
   ```bash
   curl -fsSL https://tailscale.com/install.sh | sh
   sudo tailscale up
   ```
2. **åœ¨ Mac ä¸Šå®‰è£…å¹¶ç™»å½•**
   - ä½¿ç”¨ Tailscale åº”ç”¨å¹¶ç™»å½•åˆ°åŒä¸€ä¸ª tailnetã€‚
3. **å¯ç”¨ MagicDNSï¼ˆæŽ¨èï¼‰**
   - åœ¨ Tailscale ç®¡ç†æŽ§åˆ¶å°ä¸­å¯ç”¨ MagicDNSï¼Œè¿™æ · VPS æœ‰ä¸€ä¸ªç¨³å®šçš„åç§°ã€‚
4. **ä½¿ç”¨ tailnet ä¸»æœºå**
   - SSHï¼š`ssh user@your-vps.tailnet-xxxx.ts.net`
   - Gateway ç½‘å…³ WSï¼š`ws://your-vps.tailnet-xxxx.ts.net:18789`

å¦‚æžœä½ æƒ³è¦æ—  SSH çš„æŽ§åˆ¶ UIï¼Œåœ¨ VPS ä¸Šä½¿ç”¨ Tailscale Serveï¼š

```bash
krabkrab gateway --tailscale serve
```

è¿™ä¿æŒ Gateway ç½‘å…³ç»‘å®šåˆ° local loopback å¹¶é€šè¿‡ Tailscale æš´éœ² HTTPSã€‚å‚é˜… [Tailscale](/gateway/tailscale)ã€‚

### å¦‚ä½•å°† Mac èŠ‚ç‚¹è¿žæŽ¥åˆ°è¿œç¨‹ Gateway ç½‘å…³ï¼ˆTailscale Serveï¼‰

Serve æš´éœ² **Gateway ç½‘å…³æŽ§åˆ¶ UI + WS**ã€‚èŠ‚ç‚¹é€šè¿‡åŒä¸€ä¸ª Gateway ç½‘å…³ WS ç«¯ç‚¹è¿žæŽ¥ã€‚

æŽ¨èè®¾ç½®ï¼š

1. **ç¡®ä¿ VPS + Mac åœ¨åŒä¸€ä¸ª tailnet ä¸Š**ã€‚
2. **ä½¿ç”¨ macOS åº”ç”¨çš„è¿œç¨‹æ¨¡å¼**ï¼ˆSSH ç›®æ ‡å¯ä»¥æ˜¯ tailnet ä¸»æœºåï¼‰ã€‚åº”ç”¨ä¼šéš§é“ Gateway ç½‘å…³ç«¯å£å¹¶ä½œä¸ºèŠ‚ç‚¹è¿žæŽ¥ã€‚
3. **åœ¨ Gateway ç½‘å…³ä¸Šæ‰¹å‡†èŠ‚ç‚¹**ï¼š
   ```bash
   krabkrab nodes pending
   krabkrab nodes approve <requestId>
   ```

æ–‡æ¡£ï¼š[Gateway ç½‘å…³åè®®](/gateway/protocol)ã€[å‘çŽ°](/gateway/discovery)ã€[macOS è¿œç¨‹æ¨¡å¼](/platforms/mac/remote)ã€‚

## çŽ¯å¢ƒå˜é‡å’Œ .env åŠ è½½

### KrabKrab å¦‚ä½•åŠ è½½çŽ¯å¢ƒå˜é‡

KrabKrab ä»Žçˆ¶è¿›ç¨‹ï¼ˆshellã€launchd/systemdã€CI ç­‰ï¼‰è¯»å–çŽ¯å¢ƒå˜é‡ï¼Œå¹¶é¢å¤–åŠ è½½ï¼š

- å½“å‰å·¥ä½œç›®å½•ä¸‹çš„ `.env`
- `~/.krabkrab/.env`ï¼ˆå³ `$krabkrab_STATE_DIR/.env`ï¼‰çš„å…¨å±€å›žé€€ `.env`

ä¸¤ä¸ª `.env` æ–‡ä»¶éƒ½ä¸ä¼šè¦†ç›–å·²æœ‰çš„çŽ¯å¢ƒå˜é‡ã€‚

ä½ ä¹Ÿå¯ä»¥åœ¨é…ç½®ä¸­å®šä¹‰å†…è”çŽ¯å¢ƒå˜é‡ï¼ˆä»…åœ¨è¿›ç¨‹çŽ¯å¢ƒä¸­ç¼ºå¤±æ—¶åº”ç”¨ï¼‰ï¼š

```json5
{
  env: {
    OPENROUTER_API_KEY: "sk-or-...",
    vars: { GROQ_API_KEY: "gsk-..." },
  },
}
```

å‚é˜… [/environment](/help/environment) äº†è§£ä¼˜å…ˆçº§å’Œæ¥æºè¯¦æƒ…ã€‚

### æˆ‘é€šè¿‡æœåŠ¡å¯åŠ¨äº† Gateway ç½‘å…³ï¼Œä½†çŽ¯å¢ƒå˜é‡æ¶ˆå¤±äº†ï¼Œæ€Žä¹ˆåŠž

ä¸¤ä¸ªå¸¸è§ä¿®å¤æ–¹æ³•ï¼š

1. å°†ç¼ºå¤±çš„å¯†é’¥æ”¾åœ¨ `~/.krabkrab/.env` ä¸­ï¼Œè¿™æ ·å³ä½¿æœåŠ¡ä¸ç»§æ‰¿ä½ çš„ shell çŽ¯å¢ƒä¹Ÿèƒ½è¢«èŽ·å–ã€‚
2. å¯ç”¨ shell å¯¼å…¥ï¼ˆå¯é€‰çš„ä¾¿åˆ©åŠŸèƒ½ï¼‰ï¼š

```json5
{
  env: {
    shellEnv: {
      enabled: true,
      timeoutMs: 15000,
    },
  },
}
```

è¿™ä¼šè¿è¡Œä½ çš„ç™»å½• shell å¹¶ä»…å¯¼å…¥ç¼ºå¤±çš„é¢„æœŸé”®åï¼ˆä»Žä¸è¦†ç›–ï¼‰ã€‚çŽ¯å¢ƒå˜é‡ç­‰æ•ˆé¡¹ï¼š
`krabkrab_LOAD_SHELL_ENV=1`ã€`krabkrab_SHELL_ENV_TIMEOUT_MS=15000`ã€‚

### æˆ‘è®¾ç½®äº† COPILOT_GITHUB_TOKENï¼Œä½† models status æ˜¾ç¤º"Shell env: off"ï¼Œä¸ºä»€ä¹ˆ

`krabkrab models status` æŠ¥å‘Šçš„æ˜¯ **shell çŽ¯å¢ƒå¯¼å…¥**æ˜¯å¦å¯ç”¨ã€‚"Shell env: off"**ä¸**æ„å‘³ç€ä½ çš„çŽ¯å¢ƒå˜é‡ç¼ºå¤±â€”â€”å®ƒåªæ„å‘³ç€ KrabKrab ä¸ä¼šè‡ªåŠ¨åŠ è½½ä½ çš„ç™»å½• shellã€‚

å¦‚æžœ Gateway ç½‘å…³ä½œä¸ºæœåŠ¡ï¼ˆlaunchd/systemdï¼‰è¿è¡Œï¼Œå®ƒä¸ä¼šç»§æ‰¿ä½ çš„ shell çŽ¯å¢ƒã€‚é€šè¿‡ä»¥ä¸‹æ–¹å¼ä¹‹ä¸€ä¿®å¤ï¼š

1. å°†ä»¤ç‰Œæ”¾åœ¨ `~/.krabkrab/.env` ä¸­ï¼š
   ```
   COPILOT_GITHUB_TOKEN=...
   ```
2. æˆ–å¯ç”¨ shell å¯¼å…¥ï¼ˆ`env.shellEnv.enabled: true`ï¼‰ã€‚
3. æˆ–å°†å…¶æ·»åŠ åˆ°é…ç½®çš„ `env` å—ä¸­ï¼ˆä»…åœ¨ç¼ºå¤±æ—¶åº”ç”¨ï¼‰ã€‚

ç„¶åŽé‡å¯ Gateway ç½‘å…³å¹¶é‡æ–°æ£€æŸ¥ï¼š

```bash
krabkrab models status
```

Copilot ä»¤ç‰Œä»Ž `COPILOT_GITHUB_TOKEN` è¯»å–ï¼ˆä¹Ÿæ”¯æŒ `GH_TOKEN` / `GITHUB_TOKEN`ï¼‰ã€‚
å‚é˜… [/concepts/model-providers](/concepts/model-providers) å’Œ [/environment](/help/environment)ã€‚

## ä¼šè¯ä¸Žå¤šèŠå¤©

### å¦‚ä½•å¼€å§‹ä¸€ä¸ªæ–°å¯¹è¯

å‘é€ `/new` æˆ– `/reset` ä½œä¸ºç‹¬ç«‹æ¶ˆæ¯ã€‚å‚é˜…[ä¼šè¯ç®¡ç†](/concepts/session)ã€‚

### å¦‚æžœæˆ‘ä»Žä¸å‘é€ /newï¼Œä¼šè¯ä¼šè‡ªåŠ¨é‡ç½®å—

ä¼šã€‚ä¼šè¯åœ¨ `session.idleMinutes`ï¼ˆé»˜è®¤ **60**ï¼‰åŽè¿‡æœŸã€‚**ä¸‹ä¸€æ¡**æ¶ˆæ¯ä¼šä¸ºè¯¥èŠå¤©é”®å¼€å§‹ä¸€ä¸ªæ–°çš„ä¼šè¯ IDã€‚è¿™ä¸ä¼šåˆ é™¤è®°å½•â€”â€”åªæ˜¯å¼€å§‹ä¸€ä¸ªæ–°ä¼šè¯ã€‚

```json5
{
  session: {
    idleMinutes: 240,
  },
}
```

### èƒ½å¦åˆ›å»ºä¸€ä¸ª KrabKrab å®žä¾‹å›¢é˜Ÿâ€”â€”ä¸€ä¸ª CEO å’Œå¤šä¸ªæ™ºèƒ½ä½“

å¯ä»¥ï¼Œé€šè¿‡**å¤šæ™ºèƒ½ä½“è·¯ç”±**å’Œ**å­æ™ºèƒ½ä½“**ã€‚ä½ å¯ä»¥åˆ›å»ºä¸€ä¸ªåè°ƒå™¨æ™ºèƒ½ä½“å’Œå¤šä¸ªå·¥ä½œè€…æ™ºèƒ½ä½“ï¼Œæ¯ä¸ªéƒ½æœ‰è‡ªå·±çš„å·¥ä½œåŒºå’Œæ¨¡åž‹ã€‚

è¯è™½å¦‚æ­¤ï¼Œæœ€å¥½å°†å…¶è§†ä¸ºä¸€ä¸ª**æœ‰è¶£çš„å®žéªŒ**ã€‚å®ƒæ¶ˆè€—å¤§é‡ä»¤ç‰Œï¼Œé€šå¸¸ä¸å¦‚ä½¿ç”¨ä¸€ä¸ªæœºå™¨äººé…åˆä¸åŒä¼šè¯çš„æ•ˆçŽ‡é«˜ã€‚æˆ‘ä»¬è®¾æƒ³çš„å…¸åž‹æ¨¡åž‹æ˜¯ä¸€ä¸ªä½ ä¸Žä¹‹å¯¹è¯çš„æœºå™¨äººï¼Œç”¨ä¸åŒçš„ä¼šè¯è¿›è¡Œå¹¶è¡Œå·¥ä½œã€‚è¯¥æœºå™¨äººä¹Ÿå¯ä»¥åœ¨éœ€è¦æ—¶ç”Ÿæˆå­æ™ºèƒ½ä½“ã€‚

æ–‡æ¡£ï¼š[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ã€[å­æ™ºèƒ½ä½“](/tools/subagents)ã€[æ™ºèƒ½ä½“ CLI](/cli/agents)ã€‚

### ä¸ºä»€ä¹ˆä¸Šä¸‹æ–‡åœ¨ä»»åŠ¡ä¸­é€”è¢«æˆªæ–­äº†ï¼Ÿå¦‚ä½•é˜²æ­¢

ä¼šè¯ä¸Šä¸‹æ–‡å—æ¨¡åž‹çª—å£é™åˆ¶ã€‚é•¿å¯¹è¯ã€å¤§é‡å·¥å…·è¾“å‡ºæˆ–è®¸å¤šæ–‡ä»¶å¯èƒ½è§¦å‘åŽ‹ç¼©æˆ–æˆªæ–­ã€‚

æœ‰å¸®åŠ©çš„åšæ³•ï¼š

- è¦æ±‚æœºå™¨äººæ€»ç»“å½“å‰çŠ¶æ€å¹¶å†™å…¥æ–‡ä»¶ã€‚
- åœ¨é•¿ä»»åŠ¡ä¹‹å‰ä½¿ç”¨ `/compact`ï¼Œåˆ‡æ¢è¯é¢˜æ—¶ä½¿ç”¨ `/new`ã€‚
- å°†é‡è¦ä¸Šä¸‹æ–‡ä¿å­˜åœ¨å·¥ä½œåŒºä¸­ï¼Œè¦æ±‚æœºå™¨äººè¯»å–ã€‚
- å¯¹é•¿æ—¶é—´æˆ–å¹¶è¡Œå·¥ä½œä½¿ç”¨å­æ™ºèƒ½ä½“ï¼Œè¿™æ ·ä¸»èŠå¤©ä¿æŒè¾ƒå°ã€‚
- å¦‚æžœè¿™ç§æƒ…å†µç»å¸¸å‘ç”Ÿï¼Œé€‰æ‹©å…·æœ‰æ›´å¤§ä¸Šä¸‹æ–‡çª—å£çš„æ¨¡åž‹ã€‚

### å¦‚ä½•å®Œå…¨é‡ç½® KrabKrab ä½†ä¿ç•™å®‰è£…

ä½¿ç”¨é‡ç½®å‘½ä»¤ï¼š

```bash
krabkrab reset
```

éžäº¤äº’å¼å®Œæ•´é‡ç½®ï¼š

```bash
krabkrab reset --scope full --yes --non-interactive
```

ç„¶åŽé‡æ–°è¿è¡Œæ–°æ‰‹å¼•å¯¼ï¼š

```bash
krabkrab onboard --install-daemon
```

æ³¨æ„ï¼š

- æ–°æ‰‹å¼•å¯¼å‘å¯¼åœ¨çœ‹åˆ°çŽ°æœ‰é…ç½®æ—¶ä¹Ÿæä¾›**é‡ç½®**é€‰é¡¹ã€‚å‚é˜…[å‘å¯¼](/start/wizard)ã€‚
- å¦‚æžœä½ ä½¿ç”¨äº†é…ç½®æ–‡ä»¶ï¼ˆ`--profile` / `krabkrab_PROFILE`ï¼‰ï¼Œé‡ç½®æ¯ä¸ªçŠ¶æ€ç›®å½•ï¼ˆé»˜è®¤ä¸º `~/.krabkrab-<profile>`ï¼‰ã€‚
- å¼€å‘é‡ç½®ï¼š`krabkrab gateway --dev --reset`ï¼ˆä»…é™å¼€å‘ï¼›æ¸…é™¤å¼€å‘é…ç½® + å‡­æ® + ä¼šè¯ + å·¥ä½œåŒºï¼‰ã€‚

### æˆ‘é‡åˆ°äº† context too large é”™è¯¯â€”â€”å¦‚ä½•é‡ç½®æˆ–åŽ‹ç¼©

ä½¿ç”¨ä»¥ä¸‹æ–¹å¼ä¹‹ä¸€ï¼š

- **åŽ‹ç¼©**ï¼ˆä¿ç•™å¯¹è¯ä½†æ€»ç»“è¾ƒæ—©çš„è½®æ¬¡ï¼‰ï¼š

  ```
  /compact
  ```

  æˆ– `/compact <instructions>` æ¥å¼•å¯¼æ€»ç»“ã€‚

- **é‡ç½®**ï¼ˆä¸ºåŒä¸€èŠå¤©é”®å¼€å§‹æ–°çš„ä¼šè¯ IDï¼‰ï¼š
  ```
  /new
  /reset
  ```

å¦‚æžœæŒç»­å‡ºçŽ°ï¼š

- å¯ç”¨æˆ–è°ƒæ•´**ä¼šè¯ä¿®å‰ª**ï¼ˆ`agents.defaults.contextPruning`ï¼‰ä»¥è£å‰ªæ—§çš„å·¥å…·è¾“å‡ºã€‚
- ä½¿ç”¨å…·æœ‰æ›´å¤§ä¸Šä¸‹æ–‡çª—å£çš„æ¨¡åž‹ã€‚

æ–‡æ¡£ï¼š[åŽ‹ç¼©](/concepts/compaction)ã€[ä¼šè¯ä¿®å‰ª](/concepts/session-pruning)ã€[ä¼šè¯ç®¡ç†](/concepts/session)ã€‚

### ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ° LLM request rejected: messages.N.content.X.tool_use.input: Field required

è¿™æ˜¯ä¸€ä¸ªæä¾›å•†éªŒè¯é”™è¯¯ï¼šæ¨¡åž‹å‘å‡ºäº†ä¸€ä¸ªæ²¡æœ‰å¿…éœ€ `input` çš„ `tool_use` å—ã€‚é€šå¸¸æ„å‘³ç€ä¼šè¯åŽ†å²å·²è¿‡æ—¶æˆ–æŸåï¼ˆé€šå¸¸åœ¨é•¿çº¿ç¨‹æˆ–å·¥å…·/æ¨¡å¼å˜æ›´åŽå‘ç”Ÿï¼‰ã€‚

ä¿®å¤ï¼šä½¿ç”¨ `/new`ï¼ˆç‹¬ç«‹æ¶ˆæ¯ï¼‰å¼€å§‹æ–°ä¼šè¯ã€‚

### ä¸ºä»€ä¹ˆæ¯ 30 åˆ†é’Ÿæ”¶åˆ°ä¸€æ¬¡å¿ƒè·³æ¶ˆæ¯

å¿ƒè·³é»˜è®¤æ¯ **30 åˆ†é’Ÿ**è¿è¡Œä¸€æ¬¡ã€‚è°ƒæ•´æˆ–ç¦ç”¨ï¼š

```json5
{
  agents: {
    defaults: {
      heartbeat: {
        every: "2h", // æˆ– "0m" ç¦ç”¨
      },
    },
  },
}
```

å¦‚æžœ `HEARTBEAT.md` å­˜åœ¨ä½†å®žé™…ä¸Šä¸ºç©ºï¼ˆåªæœ‰ç©ºè¡Œå’Œ markdown æ ‡é¢˜å¦‚ `# Heading`ï¼‰ï¼ŒKrabKrab ä¼šè·³è¿‡å¿ƒè·³è¿è¡Œä»¥èŠ‚çœ API è°ƒç”¨ã€‚å¦‚æžœæ–‡ä»¶ä¸å­˜åœ¨ï¼Œå¿ƒè·³ä»ç„¶è¿è¡Œï¼Œç”±æ¨¡åž‹å†³å®šåšä»€ä¹ˆã€‚

æŒ‰æ™ºèƒ½ä½“è¦†ç›–ä½¿ç”¨ `agents.list[].heartbeat`ã€‚æ–‡æ¡£ï¼š[å¿ƒè·³](/gateway/heartbeat)ã€‚

### éœ€è¦åœ¨ WhatsApp ç¾¤ç»„ä¸­æ·»åŠ â€œæœºå™¨äººè´¦å·â€å—

ä¸éœ€è¦ã€‚KrabKrab è¿è¡Œåœ¨**ä½ è‡ªå·±çš„è´¦æˆ·**ä¸Šï¼Œæ‰€ä»¥å¦‚æžœä½ åœ¨ç¾¤ç»„ä¸­ï¼ŒKrabKrab å°±èƒ½çœ‹åˆ°å®ƒã€‚
é»˜è®¤æƒ…å†µä¸‹ï¼Œç¾¤ç»„å›žå¤è¢«é˜»æ­¢ï¼Œç›´åˆ°ä½ å…è®¸å‘é€è€…ï¼ˆ`groupPolicy: "allowlist"`ï¼‰ã€‚

å¦‚æžœä½ åªæƒ³**ä½ è‡ªå·±**èƒ½è§¦å‘ç¾¤ç»„å›žå¤ï¼š

```json5
{
  channels: {
    whatsapp: {
      groupPolicy: "allowlist",
      groupAllowFrom: ["+15551234567"],
    },
  },
}
```

### å¦‚ä½•èŽ·å– WhatsApp ç¾¤ç»„çš„ JID

æ–¹æ³• 1ï¼ˆæœ€å¿«ï¼‰ï¼šè·Ÿè¸ªæ—¥å¿—å¹¶åœ¨ç¾¤ç»„ä¸­å‘é€æµ‹è¯•æ¶ˆæ¯ï¼š

```bash
krabkrab logs --follow --json
```

æŸ¥æ‰¾ä»¥ `@g.us` ç»“å°¾çš„ `chatId`ï¼ˆæˆ– `from`ï¼‰ï¼Œå¦‚ï¼š
`1234567890-1234567890@g.us`ã€‚

æ–¹æ³• 2ï¼ˆå¦‚æžœå·²é…ç½®/åŠ å…¥å…è®¸åˆ—è¡¨ï¼‰ï¼šä»Žé…ç½®ä¸­åˆ—å‡ºç¾¤ç»„ï¼š

```bash
krabkrab directory groups list --channel whatsapp
```

æ–‡æ¡£ï¼š[WhatsApp](/channels/whatsapp)ã€[ç›®å½•](/cli/directory)ã€[æ—¥å¿—](/cli/logs)ã€‚

### ä¸ºä»€ä¹ˆ KrabKrab ä¸åœ¨ç¾¤ç»„ä¸­å›žå¤

ä¸¤ä¸ªå¸¸è§åŽŸå› ï¼š

- æåŠé™åˆ¶å·²å¼€å¯ï¼ˆé»˜è®¤ï¼‰ã€‚ä½ å¿…é¡» @æåŠæœºå™¨äººï¼ˆæˆ–åŒ¹é… `mentionPatterns`ï¼‰ã€‚
- ä½ é…ç½®äº† `channels.whatsapp.groups` ä½†æ²¡æœ‰ `"*"` ä¸”è¯¥ç¾¤ç»„æœªåŠ å…¥å…è®¸åˆ—è¡¨ã€‚

å‚é˜…[ç¾¤ç»„](/channels/groups)å’Œ[ç¾¤ç»„æ¶ˆæ¯](/channels/group-messages)ã€‚

### ç¾¤ç»„/çº¿ç¨‹ä¸Žç§èŠå…±äº«ä¸Šä¸‹æ–‡å—

ç›´æŽ¥èŠå¤©é»˜è®¤æŠ˜å åˆ°ä¸»ä¼šè¯ã€‚ç¾¤ç»„/é¢‘é“æœ‰è‡ªå·±çš„ä¼šè¯é”®ï¼ŒTelegram è¯é¢˜ / Discord çº¿ç¨‹æ˜¯ç‹¬ç«‹çš„ä¼šè¯ã€‚å‚é˜…[ç¾¤ç»„](/channels/groups)å’Œ[ç¾¤ç»„æ¶ˆæ¯](/channels/group-messages)ã€‚

### å¯ä»¥åˆ›å»ºå¤šå°‘ä¸ªå·¥ä½œåŒºå’Œæ™ºèƒ½ä½“

æ²¡æœ‰ç¡¬æ€§é™åˆ¶ã€‚å‡ åä¸ªï¼ˆç”šè‡³å‡ ç™¾ä¸ªï¼‰éƒ½æ²¡é—®é¢˜ï¼Œä½†è¯·æ³¨æ„ï¼š

- **ç£ç›˜å¢žé•¿ï¼š** ä¼šè¯ + è®°å½•ä½äºŽ `~/.krabkrab/agents/<agentId>/sessions/` ä¸‹ã€‚
- **ä»¤ç‰Œæˆæœ¬ï¼š** æ›´å¤šæ™ºèƒ½ä½“æ„å‘³ç€æ›´å¤šå¹¶å‘æ¨¡åž‹ä½¿ç”¨ã€‚
- **è¿ç»´å¼€é”€ï¼š** æŒ‰æ™ºèƒ½ä½“çš„è®¤è¯é…ç½®æ–‡ä»¶ã€å·¥ä½œåŒºå’Œæ¸ é“è·¯ç”±ã€‚

æç¤ºï¼š

- æ¯ä¸ªæ™ºèƒ½ä½“ä¿æŒä¸€ä¸ª**æ´»è·ƒ**å·¥ä½œåŒºï¼ˆ`agents.defaults.workspace`ï¼‰ã€‚
- å¦‚æžœç£ç›˜å¢žé•¿ï¼Œä¿®å‰ªæ—§ä¼šè¯ï¼ˆåˆ é™¤ JSONL æˆ–å­˜å‚¨æ¡ç›®ï¼‰ã€‚
- ä½¿ç”¨ `krabkrab doctor` å‘çŽ°æ— ç”¨çš„å·¥ä½œåŒºå’Œé…ç½®æ–‡ä»¶ä¸åŒ¹é…ã€‚

### å¯ä»¥åŒæ—¶è¿è¡Œå¤šä¸ªæœºå™¨äººæˆ–èŠå¤©ï¼ˆSlackï¼‰å—ï¼Ÿåº”è¯¥å¦‚ä½•è®¾ç½®

å¯ä»¥ã€‚ä½¿ç”¨**å¤šæ™ºèƒ½ä½“è·¯ç”±**è¿è¡Œå¤šä¸ªéš”ç¦»çš„æ™ºèƒ½ä½“ï¼Œå¹¶æŒ‰æ¸ é“/è´¦æˆ·/å¯¹ç­‰æ–¹è·¯ç”±å…¥ç«™æ¶ˆæ¯ã€‚Slack ä½œä¸ºæ¸ é“å—æ”¯æŒï¼Œå¯ä»¥ç»‘å®šåˆ°ç‰¹å®šæ™ºèƒ½ä½“ã€‚

æµè§ˆå™¨è®¿é—®åŠŸèƒ½å¼ºå¤§ï¼Œä½†ä¸æ˜¯â€œèƒ½åšäººç±»èƒ½åšçš„ä¸€åˆ‡â€â€”â€”åæœºå™¨äººã€éªŒè¯ç å’Œ MFA ä»ç„¶å¯ä»¥é˜»æ­¢è‡ªåŠ¨åŒ–ã€‚ä¸ºäº†æœ€å¯é çš„æµè§ˆå™¨æŽ§åˆ¶ï¼Œåœ¨è¿è¡Œæµè§ˆå™¨çš„æœºå™¨ä¸Šä½¿ç”¨ Chrome æ‰©å±•ä¸­ç»§ï¼ˆGateway ç½‘å…³å¯ä»¥åœ¨ä»»ä½•åœ°æ–¹ï¼‰ã€‚

æœ€ä½³å®žè·µè®¾ç½®ï¼š

- å¸¸å¼€ Gateway ç½‘å…³ä¸»æœºï¼ˆVPS/Mac miniï¼‰ã€‚
- æ¯ä¸ªè§’è‰²ä¸€ä¸ªæ™ºèƒ½ä½“ï¼ˆç»‘å®šï¼‰ã€‚
- Slack æ¸ é“ç»‘å®šåˆ°è¿™äº›æ™ºèƒ½ä½“ã€‚
- éœ€è¦æ—¶é€šè¿‡æ‰©å±•ä¸­ç»§ï¼ˆæˆ–èŠ‚ç‚¹ï¼‰ä½¿ç”¨æœ¬åœ°æµè§ˆå™¨ã€‚

æ–‡æ¡£ï¼š[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ã€[Slack](/channels/slack)ã€
[æµè§ˆå™¨](/tools/browser)ã€[Chrome æ‰©å±•](/tools/chrome-extension)ã€[èŠ‚ç‚¹](/nodes)ã€‚

## æ¨¡åž‹ï¼šé»˜è®¤å€¼ã€é€‰æ‹©ã€åˆ«åã€åˆ‡æ¢

### ä»€ä¹ˆæ˜¯â€œé»˜è®¤æ¨¡åž‹â€

KrabKrab çš„é»˜è®¤æ¨¡åž‹æ˜¯ä½ è®¾ç½®çš„ï¼š

```
agents.defaults.model.primary
```

æ¨¡åž‹ä»¥ `provider/model` å¼•ç”¨ï¼ˆç¤ºä¾‹ï¼š`anthropic/claude-opus-4-5`ï¼‰ã€‚å¦‚æžœä½ çœç•¥æä¾›å•†ï¼ŒKrabKrab ç›®å‰å‡è®¾ `anthropic` ä½œä¸ºä¸´æ—¶å¼ƒç”¨å›žé€€â€”â€”ä½†ä½ ä»ç„¶åº”è¯¥**æ˜Žç¡®**è®¾ç½® `provider/model`ã€‚

### æŽ¨èä»€ä¹ˆæ¨¡åž‹

**æŽ¨èé»˜è®¤ï¼š** `anthropic/claude-opus-4-5`ã€‚
**å¥½çš„æ›¿ä»£ï¼š** `anthropic/claude-sonnet-4-5`ã€‚
**å¯é ï¼ˆä¸ªæ€§è¾ƒå°‘ï¼‰ï¼š** `openai/gpt-5.2`â€”â€”å‡ ä¹Žå’Œ Opus ä¸€æ ·å¥½ï¼Œåªæ˜¯ä¸ªæ€§è¾ƒå°‘ã€‚
**ç»æµŽï¼š** `zai/glm-4.7`ã€‚

MiniMax M2.1 æœ‰è‡ªå·±çš„æ–‡æ¡£ï¼š[MiniMax](/providers/minimax) å’Œ
[æœ¬åœ°æ¨¡åž‹](/gateway/local-models)ã€‚

ç»éªŒæ³•åˆ™ï¼šé«˜é£Žé™©å·¥ä½œä½¿ç”¨ä½ èƒ½è´Ÿæ‹…çš„**æœ€å¥½çš„æ¨¡åž‹**ï¼Œæ—¥å¸¸èŠå¤©æˆ–æ‘˜è¦ä½¿ç”¨æ›´ä¾¿å®œçš„æ¨¡åž‹ã€‚ä½ å¯ä»¥æŒ‰æ™ºèƒ½ä½“è·¯ç”±æ¨¡åž‹å¹¶ä½¿ç”¨å­æ™ºèƒ½ä½“å¹¶è¡Œå¤„ç†é•¿ä»»åŠ¡ï¼ˆæ¯ä¸ªå­æ™ºèƒ½ä½“æ¶ˆè€—ä»¤ç‰Œï¼‰ã€‚å‚é˜…[æ¨¡åž‹](/concepts/models)å’Œ[å­æ™ºèƒ½ä½“](/tools/subagents)ã€‚

é‡è¦è­¦å‘Šï¼šè¾ƒå¼±/è¿‡åº¦é‡åŒ–çš„æ¨¡åž‹æ›´å®¹æ˜“å—åˆ°æç¤ºæ³¨å…¥å’Œä¸å®‰å…¨è¡Œä¸ºçš„å½±å“ã€‚å‚é˜…[å®‰å…¨](/gateway/security)ã€‚

æ›´å¤šä¸Šä¸‹æ–‡ï¼š[æ¨¡åž‹](/concepts/models)ã€‚

### å¯ä»¥ä½¿ç”¨è‡ªæ‰˜ç®¡æ¨¡åž‹ï¼ˆllama.cppã€vLLMã€Ollamaï¼‰å—

å¯ä»¥ã€‚å¦‚æžœä½ çš„æœ¬åœ°æœåŠ¡å™¨æš´éœ²äº†å…¼å®¹ OpenAI çš„ APIï¼Œä½ å¯ä»¥å°†è‡ªå®šä¹‰æä¾›å•†æŒ‡å‘å®ƒã€‚Ollama ç›´æŽ¥æ”¯æŒï¼Œæ˜¯æœ€ç®€å•çš„è·¯å¾„ã€‚

å®‰å…¨è¯´æ˜Žï¼šè¾ƒå°æˆ–å¤§å¹…é‡åŒ–çš„æ¨¡åž‹æ›´å®¹æ˜“å—åˆ°æç¤ºæ³¨å…¥çš„å½±å“ã€‚æˆ‘ä»¬å¼ºçƒˆå»ºè®®å¯¹ä»»ä½•å¯ä»¥ä½¿ç”¨å·¥å…·çš„æœºå™¨äººä½¿ç”¨**å¤§åž‹æ¨¡åž‹**ã€‚å¦‚æžœä½ ä»ç„¶æƒ³ä½¿ç”¨å°æ¨¡åž‹ï¼Œå¯ç”¨æ²™ç®±å’Œä¸¥æ ¼çš„å·¥å…·å…è®¸åˆ—è¡¨ã€‚

æ–‡æ¡£ï¼š[Ollama](/providers/ollama)ã€[æœ¬åœ°æ¨¡åž‹](/gateway/local-models)ã€
[æ¨¡åž‹æä¾›å•†](/concepts/model-providers)ã€[å®‰å…¨](/gateway/security)ã€
[æ²™ç®±](/gateway/sandboxing)ã€‚

### å¦‚ä½•åœ¨ä¸æ¸…ç©ºé…ç½®çš„æƒ…å†µä¸‹åˆ‡æ¢æ¨¡åž‹

ä½¿ç”¨**æ¨¡åž‹å‘½ä»¤**æˆ–åªç¼–è¾‘**æ¨¡åž‹**å­—æ®µã€‚é¿å…å®Œæ•´é…ç½®æ›¿æ¢ã€‚

å®‰å…¨é€‰é¡¹ï¼š

- èŠå¤©ä¸­çš„ `/model`ï¼ˆå¿«é€Ÿï¼ŒæŒ‰ä¼šè¯ï¼‰
- `krabkrab models set ...`ï¼ˆåªæ›´æ–°æ¨¡åž‹é…ç½®ï¼‰
- `krabkrab configure --section models`ï¼ˆäº¤äº’å¼ï¼‰
- ç¼–è¾‘ `~/.krabkrab/krabkrab.json` ä¸­çš„ `agents.defaults.model`

é¿å…ä½¿ç”¨éƒ¨åˆ†å¯¹è±¡æ‰§è¡Œ `config.apply`ï¼Œé™¤éžä½ æ‰“ç®—æ›¿æ¢æ•´ä¸ªé…ç½®ã€‚å¦‚æžœä½ ç¡®å®žè¦†ç›–äº†é…ç½®ï¼Œä»Žå¤‡ä»½æ¢å¤æˆ–é‡æ–°è¿è¡Œ `krabkrab doctor` æ¥ä¿®å¤ã€‚

æ–‡æ¡£ï¼š[æ¨¡åž‹](/concepts/models)ã€[Configure](/cli/configure)ã€[Config](/cli/config)ã€[Doctor](/gateway/doctor)ã€‚

### KrabKrabã€Flawd å’Œ Krill ä½¿ç”¨ä»€ä¹ˆæ¨¡åž‹

- **KrabKrab + Flawdï¼š** Anthropic Opusï¼ˆ`anthropic/claude-opus-4-5`ï¼‰â€”â€”å‚é˜… [Anthropic](/providers/anthropic)ã€‚
- **Krillï¼š** MiniMax M2.1ï¼ˆ`minimax/MiniMax-M2.1`ï¼‰â€”â€”å‚é˜… [MiniMax](/providers/minimax)ã€‚

### å¦‚ä½•åœ¨è¿è¡Œä¸­åˆ‡æ¢æ¨¡åž‹ï¼ˆæ— éœ€é‡å¯ï¼‰

ä½¿ç”¨ `/model` å‘½ä»¤ä½œä¸ºç‹¬ç«‹æ¶ˆæ¯ï¼š

```
/model sonnet
/model haiku
/model opus
/model gpt
/model gpt-mini
/model gemini
/model gemini-flash
```

ä½ å¯ä»¥ä½¿ç”¨ `/model`ã€`/model list` æˆ– `/model status` åˆ—å‡ºå¯ç”¨æ¨¡åž‹ã€‚

`/model`ï¼ˆå’Œ `/model list`ï¼‰æ˜¾ç¤ºç´§å‡‘çš„ç¼–å·é€‰æ‹©å™¨ã€‚æŒ‰ç¼–å·é€‰æ‹©ï¼š

```
/model 3
```

ä½ ä¹Ÿå¯ä»¥ä¸ºæä¾›å•†å¼ºåˆ¶æŒ‡å®šç‰¹å®šçš„è®¤è¯é…ç½®æ–‡ä»¶ï¼ˆæŒ‰ä¼šè¯ï¼‰ï¼š

```
/model opus@anthropic:default
/model opus@anthropic:work
```

æç¤ºï¼š`/model status` æ˜¾ç¤ºå“ªä¸ªæ™ºèƒ½ä½“æ˜¯æ´»è·ƒçš„ã€æ­£åœ¨ä½¿ç”¨å“ªä¸ª `auth-profiles.json` æ–‡ä»¶ï¼Œä»¥åŠæŽ¥ä¸‹æ¥å°†å°è¯•å“ªä¸ªè®¤è¯é…ç½®æ–‡ä»¶ã€‚
å®ƒè¿˜æ˜¾ç¤ºé…ç½®çš„æä¾›å•†ç«¯ç‚¹ï¼ˆ`baseUrl`ï¼‰å’Œ API æ¨¡å¼ï¼ˆ`api`ï¼‰ï¼ˆå¦‚æžœå¯ç”¨ï¼‰ã€‚

**å¦‚ä½•å–æ¶ˆç”¨ @profile è®¾ç½®çš„é…ç½®æ–‡ä»¶å›ºå®š**

é‡æ–°è¿è¡Œ `/model` ä½†**ä¸å¸¦** `@profile` åŽç¼€ï¼š

```
/model anthropic/claude-opus-4-5
```

å¦‚æžœä½ æƒ³è¿”å›žé»˜è®¤å€¼ï¼Œä»Ž `/model` ä¸­é€‰æ‹©ï¼ˆæˆ–å‘é€ `/model <default provider/model>`ï¼‰ã€‚
ä½¿ç”¨ `/model status` ç¡®è®¤å“ªä¸ªè®¤è¯é…ç½®æ–‡ä»¶æ˜¯æ´»è·ƒçš„ã€‚

### èƒ½å¦æ—¥å¸¸ä»»åŠ¡ç”¨ GPT 5.2ï¼Œç¼–ç¨‹ç”¨ Codex 5.2

å¯ä»¥ã€‚è®¾ç½®ä¸€ä¸ªä¸ºé»˜è®¤å¹¶æŒ‰éœ€åˆ‡æ¢ï¼š

- **å¿«é€Ÿåˆ‡æ¢ï¼ˆæŒ‰ä¼šè¯ï¼‰ï¼š** æ—¥å¸¸ä»»åŠ¡ç”¨ `/model gpt-5.2`ï¼Œç¼–ç¨‹ç”¨ `/model gpt-5.2-codex`ã€‚
- **é»˜è®¤ + åˆ‡æ¢ï¼š** å°† `agents.defaults.model.primary` è®¾ç½®ä¸º `openai-codex/gpt-5.2`ï¼Œç„¶åŽç¼–ç¨‹æ—¶åˆ‡æ¢åˆ° `openai-codex/gpt-5.2-codex`ï¼ˆæˆ–åè¿‡æ¥ï¼‰ã€‚
- **å­æ™ºèƒ½ä½“ï¼š** å°†ç¼–ç¨‹ä»»åŠ¡è·¯ç”±åˆ°å…·æœ‰ä¸åŒé»˜è®¤æ¨¡åž‹çš„å­æ™ºèƒ½ä½“ã€‚

å‚é˜…[æ¨¡åž‹](/concepts/models)å’Œ[æ–œæ å‘½ä»¤](/tools/slash-commands)ã€‚

### ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ°"Model â€¦ is not allowed"ç„¶åŽæ²¡æœ‰å›žå¤

å¦‚æžœè®¾ç½®äº† `agents.defaults.models`ï¼Œå®ƒæˆä¸º `/model` å’Œä»»ä½•ä¼šè¯è¦†ç›–çš„**å…è®¸åˆ—è¡¨**ã€‚é€‰æ‹©ä¸åœ¨è¯¥åˆ—è¡¨ä¸­çš„æ¨¡åž‹ä¼šè¿”å›žï¼š

```
Model "provider/model" is not allowed. Use /model to list available models.
```

è¯¥é”™è¯¯**ä»£æ›¿**æ­£å¸¸å›žå¤è¿”å›žã€‚ä¿®å¤ï¼šå°†æ¨¡åž‹æ·»åŠ åˆ° `agents.defaults.models`ï¼Œç§»é™¤å…è®¸åˆ—è¡¨ï¼Œæˆ–ä»Ž `/model list` ä¸­é€‰æ‹©ä¸€ä¸ªæ¨¡åž‹ã€‚

### ä¸ºä»€ä¹ˆæˆ‘çœ‹åˆ°"Unknown model: minimax/MiniMax-M2.1"

è¿™æ„å‘³ç€**æä¾›å•†æœªé…ç½®**ï¼ˆæœªæ‰¾åˆ° MiniMax æä¾›å•†é…ç½®æˆ–è®¤è¯é…ç½®æ–‡ä»¶ï¼‰ï¼Œå› æ­¤æ¨¡åž‹æ— æ³•è§£æžã€‚æ­¤æ£€æµ‹çš„ä¿®å¤åœ¨ **2026.1.12**ï¼ˆæ’°å†™æœ¬æ–‡æ—¶å°šæœªå‘å¸ƒï¼‰ä¸­ã€‚

ä¿®å¤æ¸…å•ï¼š

1. å‡çº§åˆ° **2026.1.12**ï¼ˆæˆ–ä»Žæºç  `main` è¿è¡Œï¼‰ï¼Œç„¶åŽé‡å¯ Gateway ç½‘å…³ã€‚
2. ç¡®ä¿ MiniMax å·²é…ç½®ï¼ˆå‘å¯¼æˆ– JSONï¼‰ï¼Œæˆ–è€… MiniMax API å¯†é’¥å­˜åœ¨äºŽçŽ¯å¢ƒ/è®¤è¯é…ç½®æ–‡ä»¶ä¸­ä»¥ä¾¿æä¾›å•†å¯ä»¥è¢«æ³¨å…¥ã€‚
3. ä½¿ç”¨ç²¾ç¡®çš„æ¨¡åž‹ IDï¼ˆåŒºåˆ†å¤§å°å†™ï¼‰ï¼š`minimax/MiniMax-M2.1` æˆ– `minimax/MiniMax-M2.1-lightning`ã€‚
4. è¿è¡Œï¼š
   ```bash
   krabkrab models list
   ```
   å¹¶ä»Žåˆ—è¡¨ä¸­é€‰æ‹©ï¼ˆæˆ–åœ¨èŠå¤©ä¸­ä½¿ç”¨ `/model list`ï¼‰ã€‚

å‚é˜… [MiniMax](/providers/minimax) å’Œ[æ¨¡åž‹](/concepts/models)ã€‚

### èƒ½å¦å°† MiniMax è®¾ä¸ºé»˜è®¤ï¼Œå¤æ‚ä»»åŠ¡ç”¨ OpenAI

å¯ä»¥ã€‚ä½¿ç”¨ **MiniMax ä½œä¸ºé»˜è®¤**ï¼Œéœ€è¦æ—¶**æŒ‰ä¼šè¯**åˆ‡æ¢æ¨¡åž‹ã€‚æ•…éšœè½¬ç§»ç”¨äºŽ**é”™è¯¯**ï¼Œè€Œéžâ€œå›°éš¾ä»»åŠ¡â€ï¼Œæ‰€ä»¥ä½¿ç”¨ `/model` æˆ–å•ç‹¬çš„æ™ºèƒ½ä½“ã€‚

**æ–¹æ¡ˆ Aï¼šæŒ‰ä¼šè¯åˆ‡æ¢**

```json5
{
  env: { MINIMAX_API_KEY: "sk-...", OPENAI_API_KEY: "sk-..." },
  agents: {
    defaults: {
      model: { primary: "minimax/MiniMax-M2.1" },
      models: {
        "minimax/MiniMax-M2.1": { alias: "minimax" },
        "openai/gpt-5.2": { alias: "gpt" },
      },
    },
  },
}
```

ç„¶åŽï¼š

```
/model gpt
```

**æ–¹æ¡ˆ Bï¼šåˆ†ç¦»æ™ºèƒ½ä½“**

- æ™ºèƒ½ä½“ A é»˜è®¤ï¼šMiniMax
- æ™ºèƒ½ä½“ B é»˜è®¤ï¼šOpenAI
- æŒ‰æ™ºèƒ½ä½“è·¯ç”±æˆ–ä½¿ç”¨ `/agent` åˆ‡æ¢

æ–‡æ¡£ï¼š[æ¨¡åž‹](/concepts/models)ã€[å¤šæ™ºèƒ½ä½“è·¯ç”±](/concepts/multi-agent)ã€[MiniMax](/providers/minimax)ã€[OpenAI](/providers/openai)ã€‚

### opus / sonnet / gpt æ˜¯å†…ç½®å¿«æ·æ–¹å¼å—

æ˜¯çš„ã€‚KrabKrab å†…ç½®äº†ä¸€äº›é»˜è®¤ç®€å†™ï¼ˆä»…åœ¨æ¨¡åž‹å­˜åœ¨äºŽ `agents.defaults.models` ä¸­æ—¶åº”ç”¨ï¼‰ï¼š

- `opus` â†’ `anthropic/claude-opus-4-5`
- `sonnet` â†’ `anthropic/claude-sonnet-4-5`
- `gpt` â†’ `openai/gpt-5.2`
- `gpt-mini` â†’ `openai/gpt-5-mini`
- `gemini` â†’ `google/gemini-3-pro-preview`
- `gemini-flash` â†’ `google/gemini-3-flash-preview`

å¦‚æžœä½ è®¾ç½®äº†åŒåçš„è‡ªå®šä¹‰åˆ«åï¼Œä½ çš„å€¼ä¼˜å…ˆã€‚

### å¦‚ä½•å®šä¹‰/è¦†ç›–æ¨¡åž‹å¿«æ·æ–¹å¼ï¼ˆåˆ«åï¼‰

åˆ«åæ¥è‡ª `agents.defaults.models.<modelId>.alias`ã€‚ç¤ºä¾‹ï¼š

```json5
{
  agents: {
    defaults: {
      model: { primary: "anthropic/claude-opus-4-5" },
      models: {
        "anthropic/claude-opus-4-5": { alias: "opus" },
        "anthropic/claude-sonnet-4-5": { alias: "sonnet" },
        "anthropic/claude-haiku-4-5": { alias: "haiku" },
      },
    },
  },
}
```

ç„¶åŽ `/model sonnet`ï¼ˆæˆ–æ”¯æŒæ—¶çš„ `/<alias>`ï¼‰è§£æžä¸ºè¯¥æ¨¡åž‹ IDã€‚

### å¦‚ä½•æ·»åŠ å…¶ä»–æä¾›å•†ï¼ˆå¦‚ OpenRouter æˆ– Z.AIï¼‰çš„æ¨¡åž‹

OpenRouterï¼ˆæŒ‰ä»¤ç‰Œä»˜è´¹ï¼›å¤šç§æ¨¡åž‹ï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      model: { primary: "openrouter/anthropic/claude-sonnet-4-5" },
      models: { "openrouter/anthropic/claude-sonnet-4-5": {} },
    },
  },
  env: { OPENROUTER_API_KEY: "sk-or-..." },
}
```

Z.AIï¼ˆGLM æ¨¡åž‹ï¼‰ï¼š

```json5
{
  agents: {
    defaults: {
      model: { primary: "zai/glm-4.7" },
      models: { "zai/glm-4.7": {} },
    },
  },
  env: { ZAI_API_KEY: "..." },
}
```

å¦‚æžœä½ å¼•ç”¨äº† provider/model ä½†ç¼ºå°‘æ‰€éœ€çš„æä¾›å•†å¯†é’¥ï¼Œä½ ä¼šæ”¶åˆ°è¿è¡Œæ—¶è®¤è¯é”™è¯¯ï¼ˆä¾‹å¦‚ `No API key found for provider "zai"`ï¼‰ã€‚

**æ·»åŠ æ–°æ™ºèƒ½ä½“åŽæç¤º No API key found for provider**

è¿™é€šå¸¸æ„å‘³ç€**æ–°æ™ºèƒ½ä½“**çš„è®¤è¯å­˜å‚¨ä¸ºç©ºã€‚è®¤è¯æ˜¯æŒ‰æ™ºèƒ½ä½“çš„ï¼Œå­˜å‚¨åœ¨ï¼š

```
~/.krabkrab/agents/<agentId>/agent/auth-profiles.json
```

ä¿®å¤é€‰é¡¹ï¼š

- è¿è¡Œ `krabkrab agents add <id>` å¹¶åœ¨å‘å¯¼ä¸­é…ç½®è®¤è¯ã€‚
- æˆ–ä»Žä¸»æ™ºèƒ½ä½“çš„ `agentDir` å¤åˆ¶ `auth-profiles.json` åˆ°æ–°æ™ºèƒ½ä½“çš„ `agentDir`ã€‚

**ä¸è¦**åœ¨æ™ºèƒ½ä½“ä¹‹é—´é‡ç”¨ `agentDir`ï¼›è¿™ä¼šå¯¼è‡´è®¤è¯/ä¼šè¯å†²çªã€‚

## æ¨¡åž‹æ•…éšœè½¬ç§»ä¸Ž"All models failed"

### æ•…éšœè½¬ç§»æ˜¯å¦‚ä½•å·¥ä½œçš„

æ•…éšœè½¬ç§»åˆ†ä¸¤ä¸ªé˜¶æ®µï¼š

1. åŒä¸€æä¾›å•†å†…çš„**è®¤è¯é…ç½®æ–‡ä»¶è½®æ¢**ã€‚
2. **æ¨¡åž‹å›žé€€**åˆ° `agents.defaults.model.fallbacks` ä¸­çš„ä¸‹ä¸€ä¸ªæ¨¡åž‹ã€‚

å†·å´æœŸé€‚ç”¨äºŽå¤±è´¥çš„é…ç½®æ–‡ä»¶ï¼ˆæŒ‡æ•°é€€é¿ï¼‰ï¼Œå› æ­¤ KrabKrab å³ä½¿åœ¨æä¾›å•†è¢«é™é€Ÿæˆ–ä¸´æ—¶å¤±è´¥æ—¶ä¹Ÿèƒ½ç»§ç»­å“åº”ã€‚

### è¿™ä¸ªé”™è¯¯æ˜¯ä»€ä¹ˆæ„æ€

```
No credentials found for profile "anthropic:default"
```

è¿™æ„å‘³ç€ç³»ç»Ÿå°è¯•ä½¿ç”¨è®¤è¯é…ç½®æ–‡ä»¶ ID `anthropic:default`ï¼Œä½†åœ¨é¢„æœŸçš„è®¤è¯å­˜å‚¨ä¸­æ‰¾ä¸åˆ°å®ƒçš„å‡­æ®ã€‚

### No credentials found for profile "anthropic:default" çš„ä¿®å¤æ¸…å•

- **ç¡®è®¤è®¤è¯é…ç½®æ–‡ä»¶çš„ä½ç½®**ï¼ˆæ–°è·¯å¾„ vs æ—§è·¯å¾„ï¼‰
  - å½“å‰ï¼š`~/.krabkrab/agents/<agentId>/agent/auth-profiles.json`
  - æ—§ç‰ˆï¼š`~/.krabkrab/agent/*`ï¼ˆé€šè¿‡ `krabkrab doctor` è¿ç§»ï¼‰
- **ç¡®è®¤çŽ¯å¢ƒå˜é‡è¢« Gateway ç½‘å…³åŠ è½½**
  - å¦‚æžœä½ åœ¨ shell ä¸­è®¾ç½®äº† `ANTHROPIC_API_KEY` ä½†é€šè¿‡ systemd/launchd è¿è¡Œ Gateway ç½‘å…³ï¼Œå®ƒå¯èƒ½ä¸ä¼šç»§æ‰¿ã€‚å°†å…¶æ”¾åœ¨ `~/.krabkrab/.env` ä¸­æˆ–å¯ç”¨ `env.shellEnv`ã€‚
- **ç¡®ä¿ä½ ç¼–è¾‘çš„æ˜¯æ­£ç¡®çš„æ™ºèƒ½ä½“**
  - å¤šæ™ºèƒ½ä½“è®¾ç½®æ„å‘³ç€å¯èƒ½æœ‰å¤šä¸ª `auth-profiles.json` æ–‡ä»¶ã€‚
- **å®Œæ•´æ€§æ£€æŸ¥æ¨¡åž‹/è®¤è¯çŠ¶æ€**
  - ä½¿ç”¨ `krabkrab models status` æŸ¥çœ‹å·²é…ç½®çš„æ¨¡åž‹ä»¥åŠæä¾›å•†æ˜¯å¦å·²è®¤è¯ã€‚

**No credentials found for profile "anthropic" çš„ä¿®å¤æ¸…å•**

è¿™æ„å‘³ç€è¿è¡Œå›ºå®šåˆ° Anthropic è®¤è¯é…ç½®æ–‡ä»¶ï¼Œä½† Gateway ç½‘å…³åœ¨å…¶è®¤è¯å­˜å‚¨ä¸­æ‰¾ä¸åˆ°å®ƒã€‚

- **ä½¿ç”¨ setup-token**
  - è¿è¡Œ `claude setup-token`ï¼Œç„¶åŽç”¨ `krabkrab models auth setup-token --provider anthropic` ç²˜è´´ã€‚
  - å¦‚æžœä»¤ç‰Œåœ¨å¦ä¸€å°æœºå™¨ä¸Šåˆ›å»ºï¼Œä½¿ç”¨ `krabkrab models auth paste-token --provider anthropic`ã€‚
- **å¦‚æžœä½ æƒ³ä½¿ç”¨ API å¯†é’¥**
  - åœ¨ **Gateway ç½‘å…³ä¸»æœº**ä¸Šå°† `ANTHROPIC_API_KEY` æ”¾å…¥ `~/.krabkrab/.env`ã€‚
  - æ¸…é™¤ä»»ä½•å¼ºåˆ¶ç¼ºå¤±é…ç½®æ–‡ä»¶çš„å›ºå®šé¡ºåºï¼š
    ```bash
    krabkrab models auth order clear --provider anthropic
    ```
- **ç¡®è®¤ä½ åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè¿è¡Œå‘½ä»¤**
  - åœ¨è¿œç¨‹æ¨¡å¼ä¸‹ï¼Œè®¤è¯é…ç½®æ–‡ä»¶ä½äºŽ Gateway ç½‘å…³æœºå™¨ä¸Šï¼Œè€Œä¸æ˜¯ä½ çš„ç¬”è®°æœ¬ä¸Šã€‚

### ä¸ºä»€ä¹ˆè¿˜å°è¯•äº† Google Gemini å¹¶ä¸”å¤±è´¥äº†

å¦‚æžœä½ çš„æ¨¡åž‹é…ç½®åŒ…å« Google Gemini ä½œä¸ºå›žé€€ï¼ˆæˆ–ä½ åˆ‡æ¢åˆ°äº† Gemini ç®€å†™ï¼‰ï¼ŒKrabKrab ä¼šåœ¨æ¨¡åž‹å›žé€€æœŸé—´å°è¯•å®ƒã€‚å¦‚æžœä½ æ²¡æœ‰é…ç½® Google å‡­æ®ï¼Œä½ ä¼šçœ‹åˆ° `No API key found for provider "google"`ã€‚

ä¿®å¤ï¼šæä¾› Google è®¤è¯ï¼Œæˆ–ä»Ž `agents.defaults.model.fallbacks` / åˆ«åä¸­ç§»é™¤/é¿å… Google æ¨¡åž‹ï¼Œè¿™æ ·å›žé€€ä¸ä¼šè·¯ç”±åˆ°é‚£é‡Œã€‚

**LLM request rejected message thinking signature required google antigravity**

åŽŸå› ï¼šä¼šè¯åŽ†å²åŒ…å«**æ²¡æœ‰ç­¾åçš„ thinking å—**ï¼ˆé€šå¸¸æ¥è‡ªä¸­æ­¢/éƒ¨åˆ†æµï¼‰ã€‚Google Antigravity è¦æ±‚ thinking å—æœ‰ç­¾åã€‚

ä¿®å¤ï¼šKrabKrab çŽ°åœ¨ä¸º Google Antigravity Claude å‰¥ç¦»æœªç­¾åçš„ thinking å—ã€‚å¦‚æžœä»ç„¶å‡ºçŽ°ï¼Œå¼€å§‹**æ–°ä¼šè¯**æˆ–ä¸ºè¯¥æ™ºèƒ½ä½“è®¾ç½® `/thinking off`ã€‚

## è®¤è¯é…ç½®æ–‡ä»¶ï¼šæ¦‚å¿µå’Œç®¡ç†æ–¹å¼

ç›¸å…³ï¼š[/concepts/oauth](/concepts/oauth)ï¼ˆOAuth æµç¨‹ã€ä»¤ç‰Œå­˜å‚¨ã€å¤šè´¦æˆ·æ¨¡å¼ï¼‰

### ä»€ä¹ˆæ˜¯è®¤è¯é…ç½®æ–‡ä»¶

è®¤è¯é…ç½®æ–‡ä»¶æ˜¯ç»‘å®šåˆ°æä¾›å•†çš„å‘½åå‡­æ®è®°å½•ï¼ˆOAuth æˆ– API å¯†é’¥ï¼‰ã€‚é…ç½®æ–‡ä»¶ä½äºŽï¼š

```
~/.krabkrab/agents/<agentId>/agent/auth-profiles.json
```

### å…¸åž‹çš„é…ç½®æ–‡ä»¶ ID æœ‰å“ªäº›

KrabKrab ä½¿ç”¨æä¾›å•†å‰ç¼€çš„ IDï¼Œå¦‚ï¼š

- `anthropic:default`ï¼ˆæ²¡æœ‰é‚®ç®±èº«ä»½æ—¶å¸¸è§ï¼‰
- `anthropic:<email>`ï¼ˆç”¨äºŽ OAuth èº«ä»½ï¼‰
- ä½ è‡ªå®šä¹‰çš„ IDï¼ˆä¾‹å¦‚ `anthropic:work`ï¼‰

### å¯ä»¥æŽ§åˆ¶é¦–å…ˆå°è¯•å“ªä¸ªè®¤è¯é…ç½®æ–‡ä»¶å—

å¯ä»¥ã€‚é…ç½®æ”¯æŒé…ç½®æ–‡ä»¶çš„å¯é€‰å…ƒæ•°æ®å’ŒæŒ‰æä¾›å•†çš„æŽ’åºï¼ˆ`auth.order.<provider>`ï¼‰ã€‚è¿™**ä¸**å­˜å‚¨å¯†é’¥ï¼›å®ƒå°† ID æ˜ å°„åˆ° provider/mode å¹¶è®¾ç½®è½®æ¢é¡ºåºã€‚

å¦‚æžœæŸä¸ªé…ç½®æ–‡ä»¶å¤„äºŽçŸ­æœŸ**å†·å´**ï¼ˆé€ŸçŽ‡é™åˆ¶/è¶…æ—¶/è®¤è¯å¤±è´¥ï¼‰æˆ–è¾ƒé•¿çš„**ç¦ç”¨**çŠ¶æ€ï¼ˆè®¡è´¹/é¢åº¦ä¸è¶³ï¼‰ï¼ŒKrabKrab å¯èƒ½ä¼šä¸´æ—¶è·³è¿‡å®ƒã€‚è¦æ£€æŸ¥è¿™ä¸€ç‚¹ï¼Œè¿è¡Œ `krabkrab models status --json` å¹¶æŸ¥çœ‹ `auth.unusableProfiles`ã€‚è°ƒä¼˜ï¼š`auth.cooldowns.billingBackoffHours*`ã€‚

ä½ ä¹Ÿå¯ä»¥é€šè¿‡ CLI è®¾ç½®**æŒ‰æ™ºèƒ½ä½“**çš„é¡ºåºè¦†ç›–ï¼ˆå­˜å‚¨åœ¨è¯¥æ™ºèƒ½ä½“çš„ `auth-profiles.json` ä¸­ï¼‰ï¼š

```bash
# é»˜è®¤ä¸ºé…ç½®çš„é»˜è®¤æ™ºèƒ½ä½“ï¼ˆçœç•¥ --agentï¼‰
krabkrab models auth order get --provider anthropic

# å°†è½®æ¢é”å®šåˆ°å•ä¸ªé…ç½®æ–‡ä»¶ï¼ˆåªå°è¯•è¿™ä¸€ä¸ªï¼‰
krabkrab models auth order set --provider anthropic anthropic:default

# æˆ–è®¾ç½®æ˜Žç¡®çš„é¡ºåºï¼ˆæä¾›å•†å†…å›žé€€ï¼‰
krabkrab models auth order set --provider anthropic anthropic:work anthropic:default

# æ¸…é™¤è¦†ç›–ï¼ˆå›žé€€åˆ°é…ç½® auth.order / è½®æ¢ï¼‰
krabkrab models auth order clear --provider anthropic
```

è¦é’ˆå¯¹ç‰¹å®šæ™ºèƒ½ä½“ï¼š

```bash
krabkrab models auth order set --provider anthropic --agent main anthropic:default
```

### OAuth ä¸Ž API å¯†é’¥ï¼šæœ‰ä»€ä¹ˆåŒºåˆ«

KrabKrab ä¸¤è€…éƒ½æ”¯æŒï¼š

- **OAuth** é€šå¸¸åˆ©ç”¨è®¢é˜…è®¿é—®ï¼ˆå¦‚é€‚ç”¨ï¼‰ã€‚
- **API å¯†é’¥** ä½¿ç”¨æŒ‰ä»¤ç‰Œä»˜è´¹çš„è®¡è´¹ã€‚

å‘å¯¼æ˜Žç¡®æ”¯æŒ Anthropic setup-token å’Œ OpenAI Codex OAuthï¼Œä¹Ÿå¯ä»¥ä¸ºä½ å­˜å‚¨ API å¯†é’¥ã€‚

## Gateway ç½‘å…³ï¼šç«¯å£ã€â€œå·²åœ¨è¿è¡Œâ€å’Œè¿œç¨‹æ¨¡å¼

### Gateway ç½‘å…³ä½¿ç”¨ä»€ä¹ˆç«¯å£

`gateway.port` æŽ§åˆ¶ç”¨äºŽ WebSocket + HTTPï¼ˆæŽ§åˆ¶ UIã€é’©å­ç­‰ï¼‰çš„å•ä¸ªå¤ç”¨ç«¯å£ã€‚

ä¼˜å…ˆçº§ï¼š

```
--port > krabkrab_GATEWAY_PORT > gateway.port > é»˜è®¤ 18789
```

### ä¸ºä»€ä¹ˆ krabkrab gateway status æ˜¾ç¤º Runtime: running ä½† RPC probe: failed

å› ä¸º"running"æ˜¯ **supervisor** çš„è§†è§’ï¼ˆlaunchd/systemd/schtasksï¼‰ã€‚RPC æŽ¢æµ‹æ˜¯ CLI å®žé™…è¿žæŽ¥åˆ° Gateway ç½‘å…³ WebSocket å¹¶è°ƒç”¨ `status`ã€‚

ä½¿ç”¨ `krabkrab gateway status` å¹¶å…³æ³¨è¿™äº›è¡Œï¼š

- `Probe target:`ï¼ˆæŽ¢æµ‹å®žé™…ä½¿ç”¨çš„ URLï¼‰
- `Listening:`ï¼ˆç«¯å£ä¸Šå®žé™…ç»‘å®šçš„å†…å®¹ï¼‰
- `Last gateway error:`ï¼ˆè¿›ç¨‹å­˜æ´»ä½†ç«¯å£æœªç›‘å¬æ—¶çš„å¸¸è§æ ¹å› ï¼‰

### ä¸ºä»€ä¹ˆ krabkrab gateway status æ˜¾ç¤º Config (cli) å’Œ Config (service) ä¸åŒ

ä½ æ­£åœ¨ç¼–è¾‘ä¸€ä¸ªé…ç½®æ–‡ä»¶ï¼Œè€ŒæœåŠ¡è¿è¡Œçš„æ˜¯å¦ä¸€ä¸ªï¼ˆé€šå¸¸æ˜¯ `--profile` / `krabkrab_STATE_DIR` ä¸åŒ¹é…ï¼‰ã€‚

ä¿®å¤ï¼š

```bash
krabkrab gateway install --force
```

ä»Žä½ å¸Œæœ›æœåŠ¡ä½¿ç”¨çš„ç›¸åŒ `--profile` / çŽ¯å¢ƒè¿è¡Œè¯¥å‘½ä»¤ã€‚

### "another gateway instance is already listening"æ˜¯ä»€ä¹ˆæ„æ€

KrabKrab é€šè¿‡åœ¨å¯åŠ¨æ—¶ç«‹å³ç»‘å®š WebSocket ç›‘å¬å™¨æ¥å¼ºåˆ¶è¿è¡Œæ—¶é”ï¼ˆé»˜è®¤ `ws://127.0.0.1:18789`ï¼‰ã€‚å¦‚æžœç»‘å®šå›  `EADDRINUSE` å¤±è´¥ï¼Œå®ƒä¼šæŠ›å‡º `GatewayLockError` è¡¨ç¤ºå¦ä¸€ä¸ªå®žä¾‹å·²åœ¨ç›‘å¬ã€‚

ä¿®å¤ï¼šåœæ­¢å¦ä¸€ä¸ªå®žä¾‹ï¼Œé‡Šæ”¾ç«¯å£ï¼Œæˆ–ä½¿ç”¨ `krabkrab gateway --port <port>` è¿è¡Œã€‚

### å¦‚ä½•ä»¥è¿œç¨‹æ¨¡å¼è¿è¡Œ KrabKrabï¼ˆå®¢æˆ·ç«¯è¿žæŽ¥åˆ°å…¶ä»–ä½ç½®çš„ Gateway ç½‘å…³ï¼‰

è®¾ç½® `gateway.mode: "remote"` å¹¶æŒ‡å‘è¿œç¨‹ WebSocket URLï¼Œå¯é€‰å¸¦ä»¤ç‰Œ/å¯†ç ï¼š

```json5
{
  gateway: {
    mode: "remote",
    remote: {
      url: "ws://gateway.tailnet:18789",
      token: "your-token",
      password: "your-password",
    },
  },
}
```

æ³¨æ„ï¼š

- `krabkrab gateway` ä»…åœ¨ `gateway.mode` ä¸º `local` æ—¶å¯åŠ¨ï¼ˆæˆ–ä½ ä¼ é€’è¦†ç›–æ ‡å¿—ï¼‰ã€‚
- macOS åº”ç”¨ç›‘è§†é…ç½®æ–‡ä»¶ï¼Œå½“è¿™äº›å€¼æ›´æ”¹æ—¶å®žæ—¶åˆ‡æ¢æ¨¡å¼ã€‚

### æŽ§åˆ¶ UI æ˜¾ç¤º"unauthorized"æˆ–æŒç»­é‡è¿žï¼Œæ€Žä¹ˆåŠž

ä½ çš„ Gateway ç½‘å…³è¿è¡Œæ—¶å¯ç”¨äº†è®¤è¯ï¼ˆ`gateway.auth.*`ï¼‰ï¼Œä½† UI æ²¡æœ‰å‘é€åŒ¹é…çš„ä»¤ç‰Œ/å¯†ç ã€‚

äº‹å®žï¼ˆæ¥è‡ªä»£ç ï¼‰ï¼š

- æŽ§åˆ¶ UI å°†ä»¤ç‰Œå­˜å‚¨åœ¨æµè§ˆå™¨ localStorage é”® `krabkrab.control.settings.v1` ä¸­ã€‚
- UI å¯ä»¥å¯¼å…¥ä¸€æ¬¡ `?token=...`ï¼ˆå’Œ/æˆ– `?password=...`ï¼‰ï¼Œç„¶åŽä»Ž URL ä¸­å‰¥ç¦»ã€‚

ä¿®å¤ï¼š

- æœ€å¿«ï¼š`krabkrab dashboard`ï¼ˆæ‰“å° + å¤åˆ¶å¸¦ä»¤ç‰Œçš„é“¾æŽ¥ï¼Œå°è¯•æ‰“å¼€ï¼›å¦‚æžœæ— å¤´åˆ™æ˜¾ç¤º SSH æç¤ºï¼‰ã€‚
- å¦‚æžœä½ è¿˜æ²¡æœ‰ä»¤ç‰Œï¼š`krabkrab doctor --generate-gateway-token`ã€‚
- å¦‚æžœæ˜¯è¿œç¨‹ï¼Œå…ˆå»ºéš§é“ï¼š`ssh -N -L 18789:127.0.0.1:18789 user@host` ç„¶åŽæ‰“å¼€ `http://127.0.0.1:18789/?token=...`ã€‚
- åœ¨ Gateway ç½‘å…³ä¸»æœºä¸Šè®¾ç½® `gateway.auth.token`ï¼ˆæˆ– `krabkrab_GATEWAY_TOKEN`ï¼‰ã€‚
- åœ¨æŽ§åˆ¶ UI è®¾ç½®ä¸­ç²˜è´´ç›¸åŒçš„ä»¤ç‰Œï¼ˆæˆ–ä½¿ç”¨ä¸€æ¬¡æ€§ `?token=...` é“¾æŽ¥åˆ·æ–°ï¼‰ã€‚
- ä»ç„¶å¡ä½ï¼Ÿè¿è¡Œ `krabkrab status --all` å¹¶æŒ‰[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)æ“ä½œã€‚å‚é˜…[ä»ªè¡¨æ¿](/web/dashboard)äº†è§£è®¤è¯è¯¦æƒ…ã€‚

### æˆ‘è®¾ç½®äº† gateway.bind: "tailnet" ä½†æ— æ³•ç»‘å®š / ä»€ä¹ˆéƒ½æ²¡ç›‘å¬

`tailnet` ç»‘å®šä»Žä½ çš„ç½‘ç»œæŽ¥å£ä¸­é€‰æ‹© Tailscale IPï¼ˆ100.64.0.0/10ï¼‰ã€‚å¦‚æžœæœºå™¨æ²¡æœ‰åœ¨ Tailscale ä¸Šï¼ˆæˆ–æŽ¥å£å·²å…³é—­ï¼‰ï¼Œå°±æ²¡æœ‰å¯ç»‘å®šçš„åœ°å€ã€‚

ä¿®å¤ï¼š

- åœ¨è¯¥ä¸»æœºä¸Šå¯åŠ¨ Tailscaleï¼ˆä½¿å…¶æ‹¥æœ‰ 100.x åœ°å€ï¼‰ï¼Œæˆ–
- åˆ‡æ¢åˆ° `gateway.bind: "loopback"` / `"lan"`ã€‚

æ³¨æ„ï¼š`tailnet` æ˜¯æ˜Žç¡®çš„ã€‚`auto` ä¼˜å…ˆ local loopbackï¼›å½“ä½ æƒ³è¦ä»… tailnet ç»‘å®šæ—¶ä½¿ç”¨ `gateway.bind: "tailnet"`ã€‚

### å¯ä»¥åœ¨åŒä¸€ä¸»æœºä¸Šè¿è¡Œå¤šä¸ª Gateway ç½‘å…³å—

é€šå¸¸ä¸éœ€è¦â€”â€”ä¸€ä¸ª Gateway ç½‘å…³å¯ä»¥è¿è¡Œå¤šä¸ªæ¶ˆæ¯æ¸ é“å’Œæ™ºèƒ½ä½“ã€‚ä»…åœ¨éœ€è¦å†—ä½™ï¼ˆä¾‹å¦‚æ•‘æ´æœºå™¨äººï¼‰æˆ–ç¡¬éš”ç¦»æ—¶ä½¿ç”¨å¤šä¸ª Gateway ç½‘å…³ã€‚

å¯ä»¥ï¼Œä½†ä½ å¿…é¡»éš”ç¦»ï¼š

- `krabkrab_CONFIG_PATH`ï¼ˆæ¯å®žä¾‹é…ç½®ï¼‰
- `krabkrab_STATE_DIR`ï¼ˆæ¯å®žä¾‹çŠ¶æ€ï¼‰
- `agents.defaults.workspace`ï¼ˆå·¥ä½œåŒºéš”ç¦»ï¼‰
- `gateway.port`ï¼ˆå”¯ä¸€ç«¯å£ï¼‰

å¿«é€Ÿè®¾ç½®ï¼ˆæŽ¨èï¼‰ï¼š

- æ¯å®žä¾‹ä½¿ç”¨ `krabkrab --profile <name> â€¦`ï¼ˆè‡ªåŠ¨åˆ›å»º `~/.krabkrab-<name>`ï¼‰ã€‚
- åœ¨æ¯ä¸ªé…ç½®æ–‡ä»¶é…ç½®ä¸­è®¾ç½®å”¯ä¸€çš„ `gateway.port`ï¼ˆæˆ–æ‰‹åŠ¨è¿è¡Œæ—¶ä¼  `--port`ï¼‰ã€‚
- å®‰è£…æ¯é…ç½®æ–‡ä»¶çš„æœåŠ¡ï¼š`krabkrab --profile <name> gateway install`ã€‚

é…ç½®æ–‡ä»¶è¿˜ä¼šä¸ºæœåŠ¡åç§°æ·»åŠ åŽç¼€ï¼ˆ`bot.molt.<profile>`ï¼›æ—§ç‰ˆ `com.krabkrab.*`ã€`krabkrab-gateway-<profile>.service`ã€`KrabKrab Gateway ç½‘å…³ (<profile>)`ï¼‰ã€‚
å®Œæ•´æŒ‡å—ï¼š[å¤š Gateway ç½‘å…³](/gateway/multiple-gateways)ã€‚

### "invalid handshake" / code 1008 æ˜¯ä»€ä¹ˆæ„æ€

Gateway ç½‘å…³æ˜¯ä¸€ä¸ª **WebSocket æœåŠ¡å™¨**ï¼Œå®ƒæœŸæœ›ç¬¬ä¸€æ¡æ¶ˆæ¯æ˜¯ `connect` å¸§ã€‚å¦‚æžœæ”¶åˆ°å…¶ä»–å†…å®¹ï¼Œå®ƒä¼šä»¥ **code 1008**ï¼ˆç­–ç•¥è¿è§„ï¼‰å…³é—­è¿žæŽ¥ã€‚

å¸¸è§åŽŸå› ï¼š

- ä½ åœ¨æµè§ˆå™¨ä¸­æ‰“å¼€äº† **HTTP** URLï¼ˆ`http://...`ï¼‰è€Œä¸æ˜¯ WS å®¢æˆ·ç«¯ã€‚
- ä½ ä½¿ç”¨äº†é”™è¯¯çš„ç«¯å£æˆ–è·¯å¾„ã€‚
- ä»£ç†æˆ–éš§é“å‰¥ç¦»äº†è®¤è¯å¤´æˆ–å‘é€äº†éž Gateway ç½‘å…³è¯·æ±‚ã€‚

å¿«é€Ÿä¿®å¤ï¼š

1. ä½¿ç”¨ WS URLï¼š`ws://<host>:18789`ï¼ˆæˆ– `wss://...` å¦‚æžœ HTTPSï¼‰ã€‚
2. ä¸è¦åœ¨æ™®é€šæµè§ˆå™¨æ ‡ç­¾é¡µä¸­æ‰“å¼€ WS ç«¯å£ã€‚
3. å¦‚æžœè®¤è¯å·²å¯ç”¨ï¼Œåœ¨ `connect` å¸§ä¸­åŒ…å«ä»¤ç‰Œ/å¯†ç ã€‚

å¦‚æžœä½ ä½¿ç”¨ CLI æˆ– TUIï¼ŒURL åº”è¯¥ç±»ä¼¼ï¼š

```
krabkrab tui --url ws://<host>:18789 --token <token>
```

åè®®è¯¦æƒ…ï¼š[Gateway ç½‘å…³åè®®](/gateway/protocol)ã€‚

## æ—¥å¿—ä¸Žè°ƒè¯•

### æ—¥å¿—åœ¨å“ªé‡Œ

æ–‡ä»¶æ—¥å¿—ï¼ˆç»“æž„åŒ–ï¼‰ï¼š

```
/tmp/krabkrab/krabkrab-YYYY-MM-DD.log
```

ä½ å¯ä»¥é€šè¿‡ `logging.file` è®¾ç½®ç¨³å®šè·¯å¾„ã€‚æ–‡ä»¶æ—¥å¿—çº§åˆ«ç”± `logging.level` æŽ§åˆ¶ã€‚æŽ§åˆ¶å°è¯¦ç»†åº¦ç”± `--verbose` å’Œ `logging.consoleLevel` æŽ§åˆ¶ã€‚

æœ€å¿«çš„æ—¥å¿—è·Ÿè¸ªï¼š

```bash
krabkrab logs --follow
```

æœåŠ¡/supervisor æ—¥å¿—ï¼ˆå½“ Gateway ç½‘å…³é€šè¿‡ launchd/systemd è¿è¡Œæ—¶ï¼‰ï¼š

- macOSï¼š`$krabkrab_STATE_DIR/logs/gateway.log` å’Œ `gateway.err.log`ï¼ˆé»˜è®¤ï¼š`~/.krabkrab/logs/...`ï¼›é…ç½®æ–‡ä»¶ä½¿ç”¨ `~/.krabkrab-<profile>/logs/...`ï¼‰
- Linuxï¼š`journalctl --user -u krabkrab-gateway[-<profile>].service -n 200 --no-pager`
- Windowsï¼š`schtasks /Query /TN "KrabKrab Gateway ç½‘å…³ (<profile>)" /V /FO LIST`

å‚é˜…[æ•…éšœæŽ’é™¤](/gateway/troubleshooting#log-locations)äº†è§£æ›´å¤šã€‚

### å¦‚ä½•å¯åŠ¨/åœæ­¢/é‡å¯ Gateway ç½‘å…³æœåŠ¡

ä½¿ç”¨ Gateway ç½‘å…³è¾…åŠ©å‘½ä»¤ï¼š

```bash
krabkrab gateway status
krabkrab gateway restart
```

å¦‚æžœä½ æ‰‹åŠ¨è¿è¡Œ Gateway ç½‘å…³ï¼Œ`krabkrab gateway --force` å¯ä»¥å›žæ”¶ç«¯å£ã€‚å‚é˜… [Gateway ç½‘å…³](/gateway)ã€‚

### æˆ‘åœ¨ Windows ä¸Šå…³é—­äº†ç»ˆç«¯â€”â€”å¦‚ä½•é‡å¯ KrabKrab

æœ‰**ä¸¤ç§ Windows å®‰è£…æ¨¡å¼**ï¼š

**1) WSL2ï¼ˆæŽ¨èï¼‰ï¼š** Gateway ç½‘å…³è¿è¡Œåœ¨ Linux å†…éƒ¨ã€‚

æ‰“å¼€ PowerShellï¼Œè¿›å…¥ WSLï¼Œç„¶åŽé‡å¯ï¼š

```powershell
wsl
krabkrab gateway status
krabkrab gateway restart
```

å¦‚æžœä½ ä»Žæœªå®‰è£…æœåŠ¡ï¼Œåœ¨å‰å°å¯åŠ¨ï¼š

```bash
krabkrab gateway run
```

**2) åŽŸç”Ÿ Windowsï¼ˆä¸æŽ¨èï¼‰ï¼š** Gateway ç½‘å…³ç›´æŽ¥åœ¨ Windows ä¸­è¿è¡Œã€‚

æ‰“å¼€ PowerShell å¹¶è¿è¡Œï¼š

```powershell
krabkrab gateway status
krabkrab gateway restart
```

å¦‚æžœä½ æ‰‹åŠ¨è¿è¡Œï¼ˆæ— æœåŠ¡ï¼‰ï¼Œä½¿ç”¨ï¼š

```powershell
krabkrab gateway run
```

æ–‡æ¡£ï¼š[Windows (WSL2)](/platforms/windows)ã€[Gateway ç½‘å…³æœåŠ¡è¿ç»´æ‰‹å†Œ](/gateway)ã€‚

### Gateway ç½‘å…³å·²å¯åŠ¨ä½†å›žå¤å§‹ç»ˆä¸åˆ°è¾¾ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆ

ä»Žå¿«é€Ÿå¥åº·æ‰«æå¼€å§‹ï¼š

```bash
krabkrab status
krabkrab models status
krabkrab channels status
krabkrab logs --follow
```

å¸¸è§åŽŸå› ï¼š

- æ¨¡åž‹è®¤è¯æœªåœ¨ **Gateway ç½‘å…³ä¸»æœº**ä¸ŠåŠ è½½ï¼ˆæ£€æŸ¥ `models status`ï¼‰ã€‚
- æ¸ é“é…å¯¹/å…è®¸åˆ—è¡¨é˜»æ­¢å›žå¤ï¼ˆæ£€æŸ¥æ¸ é“é…ç½® + æ—¥å¿—ï¼‰ã€‚
- WebChat/ä»ªè¡¨æ¿æ‰“å¼€ä½†æ²¡æœ‰æ­£ç¡®çš„ä»¤ç‰Œã€‚

å¦‚æžœä½ åœ¨è¿œç¨‹ï¼Œç¡®è®¤éš§é“/Tailscale è¿žæŽ¥æ­£å¸¸ä¸” Gateway ç½‘å…³ WebSocket å¯è¾¾ã€‚

æ–‡æ¡£ï¼š[æ¸ é“](/channels)ã€[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)ã€[è¿œç¨‹è®¿é—®](/gateway/remote)ã€‚

### "Disconnected from gateway: no reason"â€”â€”æ€Žä¹ˆåŠž

è¿™é€šå¸¸æ„å‘³ç€ UI ä¸¢å¤±äº† WebSocket è¿žæŽ¥ã€‚æ£€æŸ¥ï¼š

1. Gateway ç½‘å…³åœ¨è¿è¡Œå—ï¼Ÿ`krabkrab gateway status`
2. Gateway ç½‘å…³å¥åº·å—ï¼Ÿ`krabkrab status`
3. UI æœ‰æ­£ç¡®çš„ä»¤ç‰Œå—ï¼Ÿ`krabkrab dashboard`
4. å¦‚æžœæ˜¯è¿œç¨‹ï¼Œéš§é“/Tailscale é“¾æŽ¥æ­£å¸¸å—ï¼Ÿ

ç„¶åŽè·Ÿè¸ªæ—¥å¿—ï¼š

```bash
krabkrab logs --follow
```

æ–‡æ¡£ï¼š[ä»ªè¡¨æ¿](/web/dashboard)ã€[è¿œç¨‹è®¿é—®](/gateway/remote)ã€[æ•…éšœæŽ’é™¤](/gateway/troubleshooting)ã€‚

### Telegram setMyCommands å› ç½‘ç»œé”™è¯¯å¤±è´¥ï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆ

ä»Žæ—¥å¿—å’Œæ¸ é“çŠ¶æ€å¼€å§‹ï¼š

```bash
krabkrab channels status
krabkrab channels logs --channel telegram
```

å¦‚æžœä½ åœ¨ VPS ä¸Šæˆ–ä»£ç†åŽé¢ï¼Œç¡®è®¤å‡ºç«™ HTTPS è¢«å…è®¸ä¸” DNS æ­£å¸¸å·¥ä½œã€‚
å¦‚æžœ Gateway ç½‘å…³åœ¨è¿œç¨‹ï¼Œç¡®ä¿ä½ åœ¨ Gateway ç½‘å…³ä¸»æœºä¸ŠæŸ¥çœ‹æ—¥å¿—ã€‚

æ–‡æ¡£ï¼š[Telegram](/channels/telegram)ã€[æ¸ é“æ•…éšœæŽ’é™¤](/channels/troubleshooting)ã€‚

### TUI æ²¡æœ‰è¾“å‡ºï¼Œåº”è¯¥æ£€æŸ¥ä»€ä¹ˆ

é¦–å…ˆç¡®è®¤ Gateway ç½‘å…³å¯è¾¾ä¸”æ™ºèƒ½ä½“å¯ä»¥è¿è¡Œï¼š

```bash
krabkrab status
krabkrab models status
krabkrab logs --follow
```

åœ¨ TUI ä¸­ï¼Œä½¿ç”¨ `/status` æŸ¥çœ‹å½“å‰çŠ¶æ€ã€‚å¦‚æžœä½ æœŸæœ›åœ¨èŠå¤©æ¸ é“ä¸­æ”¶åˆ°å›žå¤ï¼Œç¡®ä¿æŠ•é€’å·²å¯ç”¨ï¼ˆ`/deliver on`ï¼‰ã€‚

æ–‡æ¡£ï¼š[TUI](/web/tui)ã€[æ–œæ å‘½ä»¤](/tools/slash-commands)ã€‚

### å¦‚ä½•å®Œå…¨åœæ­¢ç„¶åŽå¯åŠ¨ Gateway ç½‘å…³å¦‚æžœä½ å®‰è£…äº†æœåŠ¡ï¼š

```bash
krabkrab gateway stop
krabkrab gateway start
```

è¿™ä¼šåœæ­¢/å¯åŠ¨**å—ç›‘ç®¡çš„æœåŠ¡**ï¼ˆmacOS ä¸Šçš„ launchdï¼ŒLinux ä¸Šçš„ systemdï¼‰ã€‚
å½“ Gateway ç½‘å…³ä½œä¸ºå®ˆæŠ¤è¿›ç¨‹åœ¨åŽå°è¿è¡Œæ—¶ä½¿ç”¨æ­¤å‘½ä»¤ã€‚

å¦‚æžœä½ åœ¨å‰å°è¿è¡Œï¼Œç”¨ Ctrlâ€‘C åœæ­¢ï¼Œç„¶åŽï¼š

```bash
krabkrab gateway run
```

æ–‡æ¡£ï¼š[Gateway ç½‘å…³æœåŠ¡è¿ç»´æ‰‹å†Œ](/gateway)ã€‚

### é€šä¿—è§£é‡Šï¼škrabkrab gateway restart ä¸Ž krabkrab gateway

- `krabkrab gateway restart`ï¼šé‡å¯**åŽå°æœåŠ¡**ï¼ˆlaunchd/systemdï¼‰ã€‚
- `krabkrab gateway`ï¼šåœ¨è¿™ä¸ªç»ˆç«¯ä¼šè¯ä¸­**å‰å°**è¿è¡Œ Gateway ç½‘å…³ã€‚

å¦‚æžœä½ å®‰è£…äº†æœåŠ¡ï¼Œä½¿ç”¨ Gateway ç½‘å…³å‘½ä»¤ã€‚æƒ³è¦ä¸€æ¬¡æ€§å‰å°è¿è¡Œæ—¶ä½¿ç”¨ `krabkrab gateway`ã€‚

### å‡ºçŽ°æ•…éšœæ—¶èŽ·å–æ›´å¤šè¯¦æƒ…çš„æœ€å¿«æ–¹æ³•æ˜¯ä»€ä¹ˆ

ä½¿ç”¨ `--verbose` å¯åŠ¨ Gateway ç½‘å…³ä»¥èŽ·å–æ›´å¤šæŽ§åˆ¶å°è¯¦æƒ…ã€‚ç„¶åŽæ£€æŸ¥æ—¥å¿—æ–‡ä»¶ä¸­çš„æ¸ é“è®¤è¯ã€æ¨¡åž‹è·¯ç”±å’Œ RPC é”™è¯¯ã€‚

## åª’ä½“ä¸Žé™„ä»¶

### æˆ‘çš„ Skills ç”Ÿæˆäº†å›¾ç‰‡/PDFï¼Œä½†ä»€ä¹ˆéƒ½æ²¡å‘é€

æ™ºèƒ½ä½“çš„å‡ºç«™é™„ä»¶å¿…é¡»åŒ…å« `MEDIA:<path-or-url>` è¡Œï¼ˆç‹¬å ä¸€è¡Œï¼‰ã€‚å‚é˜… [KrabKrab åŠ©æ‰‹è®¾ç½®](/start/krabkrab)å’Œ [Agent send](/tools/agent-send)ã€‚

CLI å‘é€ï¼š

```bash
krabkrab message send --target +15555550123 --message "Here you go" --media /path/to/file.png
```

è¿˜è¦æ£€æŸ¥ï¼š

- ç›®æ ‡æ¸ é“æ”¯æŒå‡ºç«™åª’ä½“ä¸”æœªè¢«å…è®¸åˆ—è¡¨é˜»æ­¢ã€‚
- æ–‡ä»¶åœ¨æä¾›å•†çš„å¤§å°é™åˆ¶å†…ï¼ˆå›¾ç‰‡ä¼šè°ƒæ•´åˆ°æœ€å¤§ 2048pxï¼‰ã€‚

å‚é˜…[å›¾ç‰‡](/nodes/images)ã€‚

## å®‰å…¨ä¸Žè®¿é—®æŽ§åˆ¶

### å°† KrabKrab æš´éœ²ç»™å…¥ç«™ç§ä¿¡å®‰å…¨å—

å°†å…¥ç«™ç§ä¿¡è§†ä¸ºä¸å¯ä¿¡è¾“å…¥ã€‚é»˜è®¤è®¾è®¡æ—¨åœ¨é™ä½Žé£Žé™©ï¼š

- æ”¯æŒç§ä¿¡çš„æ¸ é“ä¸Šçš„é»˜è®¤è¡Œä¸ºæ˜¯**é…å¯¹**ï¼š
  - æœªçŸ¥å‘é€è€…ä¼šæ”¶åˆ°é…å¯¹ç ï¼›æœºå™¨äººä¸å¤„ç†ä»–ä»¬çš„æ¶ˆæ¯ã€‚
  - æ‰¹å‡†æ–¹å¼ï¼š`krabkrab pairing approve <channel> <code>`
  - æ¯ä¸ªæ¸ é“çš„å¾…å¤„ç†è¯·æ±‚ä¸Šé™ä¸º **3 ä¸ª**ï¼›å¦‚æžœæ²¡æ”¶åˆ°ä»£ç ï¼Œæ£€æŸ¥ `krabkrab pairing list <channel>`ã€‚
- å…¬å¼€å¼€æ”¾ç§ä¿¡éœ€è¦æ˜Žç¡®é€‰æ‹©åŠ å…¥ï¼ˆ`dmPolicy: "open"` ä¸”å…è®¸åˆ—è¡¨ `"*"`ï¼‰ã€‚

è¿è¡Œ `krabkrab doctor` ä»¥å‘çŽ°æœ‰é£Žé™©çš„ç§ä¿¡ç­–ç•¥ã€‚

### æç¤ºæ³¨å…¥åªå¯¹å…¬å¼€æœºå™¨äººæœ‰å½±å“å—

ä¸æ˜¯ã€‚æç¤ºæ³¨å…¥æ˜¯å…³äºŽ**ä¸å¯ä¿¡å†…å®¹**ï¼Œä¸ä»…ä»…æ˜¯è°èƒ½ç»™æœºå™¨äººå‘ç§ä¿¡ã€‚å¦‚æžœä½ çš„åŠ©æ‰‹è¯»å–å¤–éƒ¨å†…å®¹ï¼ˆç½‘ç»œæœç´¢/æŠ“å–ã€æµè§ˆå™¨é¡µé¢ã€é‚®ä»¶ã€æ–‡æ¡£ã€é™„ä»¶ã€ç²˜è´´çš„æ—¥å¿—ï¼‰ï¼Œè¿™äº›å†…å®¹å¯èƒ½åŒ…å«è¯•å›¾åŠ«æŒæ¨¡åž‹çš„æŒ‡ä»¤ã€‚å³ä½¿**ä½ æ˜¯å”¯ä¸€çš„å‘é€è€…**ï¼Œè¿™ä¹Ÿå¯èƒ½å‘ç”Ÿã€‚

æœ€å¤§çš„é£Žé™©æ˜¯åœ¨å¯ç”¨å·¥å…·æ—¶ï¼šæ¨¡åž‹å¯èƒ½è¢«è¯±å¯¼æ³„éœ²ä¸Šä¸‹æ–‡æˆ–ä»£è¡¨ä½ è°ƒç”¨å·¥å…·ã€‚é€šè¿‡ä»¥ä¸‹æ–¹å¼å‡å°‘å½±å“èŒƒå›´ï¼š

- ä½¿ç”¨åªè¯»æˆ–ç¦ç”¨å·¥å…·çš„â€œé˜…è¯»å™¨â€æ™ºèƒ½ä½“æ¥æ€»ç»“ä¸å¯ä¿¡å†…å®¹
- å¯¹å¯ç”¨å·¥å…·çš„æ™ºèƒ½ä½“å…³é—­ `web_search` / `web_fetch` / `browser`
- æ²™ç®±éš”ç¦»å’Œä¸¥æ ¼çš„å·¥å…·å…è®¸åˆ—è¡¨

è¯¦æƒ…ï¼š[å®‰å…¨](/gateway/security)ã€‚

### æˆ‘çš„æœºå™¨äººåº”è¯¥æœ‰è‡ªå·±çš„é‚®ç®±ã€GitHub è´¦æˆ·æˆ–ç”µè¯å·ç å—

æ˜¯çš„ï¼Œå¯¹äºŽå¤§å¤šæ•°è®¾ç½®æ¥è¯´ã€‚ç”¨ç‹¬ç«‹çš„è´¦æˆ·å’Œç”µè¯å·ç éš”ç¦»æœºå™¨äººå¯ä»¥åœ¨å‡ºé—®é¢˜æ—¶å‡å°‘å½±å“èŒƒå›´ã€‚è¿™ä¹Ÿä½¿å¾—è½®æ¢å‡­æ®æˆ–æ’¤é”€è®¿é—®æ›´å®¹æ˜“ï¼Œè€Œä¸å½±å“ä½ çš„ä¸ªäººè´¦æˆ·ã€‚

ä»Žå°å¤„å¼€å§‹ã€‚åªæŽˆäºˆä½ å®žé™…éœ€è¦çš„å·¥å…·å’Œè´¦æˆ·çš„è®¿é—®æƒé™ï¼Œä»¥åŽéœ€è¦æ—¶å†æ‰©å±•ã€‚

æ–‡æ¡£ï¼š[å®‰å…¨](/gateway/security)ã€[é…å¯¹](/channels/pairing)ã€‚

### æˆ‘èƒ½è®©å®ƒè‡ªä¸»ç®¡ç†æˆ‘çš„çŸ­ä¿¡å—ï¼Ÿè¿™å®‰å…¨å—

æˆ‘ä»¬**ä¸å»ºè®®**å®Œå…¨è‡ªä¸»ç®¡ç†ä½ çš„ä¸ªäººæ¶ˆæ¯ã€‚æœ€å®‰å…¨çš„æ¨¡å¼æ˜¯ï¼š

- å°†ç§ä¿¡ä¿æŒåœ¨**é…å¯¹æ¨¡å¼**æˆ–ä¸¥æ ¼çš„å…è®¸åˆ—è¡¨ä¸­ã€‚
- å¦‚æžœä½ å¸Œæœ›å®ƒä»£è¡¨ä½ å‘æ¶ˆæ¯ï¼Œä½¿ç”¨**ç‹¬ç«‹çš„å·ç æˆ–è´¦æˆ·**ã€‚
- è®©å®ƒèµ·è‰ï¼Œç„¶åŽ**å‘é€å‰æ‰¹å‡†**ã€‚

å¦‚æžœä½ æƒ³å®žéªŒï¼Œåœ¨ä¸“ç”¨è´¦æˆ·ä¸Šè¿›è¡Œå¹¶ä¿æŒéš”ç¦»ã€‚å‚é˜…[å®‰å…¨](/gateway/security)ã€‚

### ä¸ªäººåŠ©ç†ä»»åŠ¡å¯ä»¥ä½¿ç”¨æ›´ä¾¿å®œçš„æ¨¡åž‹å—

å¯ä»¥ï¼Œ**å¦‚æžœ**æ™ºèƒ½ä½“ä»…ç”¨äºŽèŠå¤©ä¸”è¾“å…¥æ˜¯å¯ä¿¡çš„ã€‚è¾ƒå°çš„æ¨¡åž‹æ›´å®¹æ˜“å—åˆ°æŒ‡ä»¤åŠ«æŒï¼Œå› æ­¤é¿å…å°†å®ƒä»¬ç”¨äºŽå¯ç”¨å·¥å…·çš„æ™ºèƒ½ä½“æˆ–è¯»å–ä¸å¯ä¿¡å†…å®¹æ—¶ã€‚å¦‚æžœä½ å¿…é¡»ä½¿ç”¨è¾ƒå°çš„æ¨¡åž‹ï¼Œé”å®šå·¥å…·å¹¶åœ¨æ²™ç®±ä¸­è¿è¡Œã€‚å‚é˜…[å®‰å…¨](/gateway/security)ã€‚

### æˆ‘åœ¨ Telegram ä¸­è¿è¡Œäº† /start ä½†æ²¡æ”¶åˆ°é…å¯¹ç 

é…å¯¹ç **ä»…åœ¨**æœªçŸ¥å‘é€è€…å‘æœºå™¨äººå‘æ¶ˆæ¯ä¸” `dmPolicy: "pairing"` å¯ç”¨æ—¶å‘é€ã€‚`/start` æœ¬èº«ä¸ä¼šç”Ÿæˆä»£ç ã€‚

æ£€æŸ¥å¾…å¤„ç†è¯·æ±‚ï¼š

```bash
krabkrab pairing list telegram
```

å¦‚æžœä½ æƒ³ç«‹å³èŽ·å¾—è®¿é—®æƒé™ï¼Œå°†ä½ çš„å‘é€è€… ID åŠ å…¥å…è®¸åˆ—è¡¨æˆ–ä¸ºè¯¥è´¦æˆ·è®¾ç½® `dmPolicy: "open"`ã€‚

### WhatsAppï¼šä¼šç»™æˆ‘çš„è”ç³»äººå‘æ¶ˆæ¯å—ï¼Ÿé…å¯¹å¦‚ä½•å·¥ä½œ

ä¸ä¼šã€‚WhatsApp çš„é»˜è®¤ç§ä¿¡ç­–ç•¥æ˜¯**é…å¯¹**ã€‚æœªçŸ¥å‘é€è€…åªä¼šæ”¶åˆ°é…å¯¹ç ï¼Œä»–ä»¬çš„æ¶ˆæ¯**ä¸ä¼šè¢«å¤„ç†**ã€‚KrabKrab åªå›žå¤å®ƒæ”¶åˆ°çš„èŠå¤©æˆ–ä½ æ˜Žç¡®è§¦å‘çš„å‘é€ã€‚

æ‰¹å‡†é…å¯¹ï¼š

```bash
krabkrab pairing approve whatsapp <code>
```

åˆ—å‡ºå¾…å¤„ç†è¯·æ±‚ï¼š

```bash
krabkrab pairing list whatsapp
```

å‘å¯¼ç”µè¯å·ç æç¤ºï¼šå®ƒç”¨äºŽè®¾ç½®ä½ çš„**å…è®¸åˆ—è¡¨/æ‰€æœ‰è€…**ï¼Œä»¥ä¾¿ä½ è‡ªå·±çš„ç§ä¿¡è¢«å…è®¸ã€‚å®ƒä¸ç”¨äºŽè‡ªåŠ¨å‘é€ã€‚å¦‚æžœä½ åœ¨ä¸ªäºº WhatsApp å·ç ä¸Šè¿è¡Œï¼Œä½¿ç”¨è¯¥å·ç å¹¶å¯ç”¨ `channels.whatsapp.selfChatMode`ã€‚

## èŠå¤©å‘½ä»¤ã€ä¸­æ­¢ä»»åŠ¡å’Œâ€œåœä¸ä¸‹æ¥â€

### å¦‚ä½•é˜»æ­¢å†…éƒ¨ç³»ç»Ÿæ¶ˆæ¯æ˜¾ç¤ºåœ¨èŠå¤©ä¸­

å¤§å¤šæ•°å†…éƒ¨æˆ–å·¥å…·æ¶ˆæ¯åªåœ¨è¯¥ä¼šè¯å¯ç”¨äº† **verbose** æˆ– **reasoning** æ—¶æ‰å‡ºçŽ°ã€‚

åœ¨ä½ çœ‹åˆ°å®ƒçš„èŠå¤©ä¸­ä¿®å¤ï¼š

```
/verbose off
/reasoning off
```

å¦‚æžœä»ç„¶å˜ˆæ‚ï¼Œæ£€æŸ¥æŽ§åˆ¶ UI ä¸­çš„ä¼šè¯è®¾ç½®å¹¶å°† verbose è®¾ä¸º**ç»§æ‰¿**ã€‚åŒæ—¶ç¡®è®¤ä½ æ²¡æœ‰ä½¿ç”¨åœ¨é…ç½®ä¸­å°† `verboseDefault` è®¾ä¸º `on` çš„æœºå™¨äººé…ç½®æ–‡ä»¶ã€‚

æ–‡æ¡£ï¼š[æ€è€ƒå’Œè¯¦ç»†è¾“å‡º](/tools/thinking)ã€[å®‰å…¨](/gateway/security#reasoning--verbose-output-in-groups)ã€‚

### å¦‚ä½•åœæ­¢/å–æ¶ˆæ­£åœ¨è¿è¡Œçš„ä»»åŠ¡

å‘é€ä»¥ä¸‹ä»»ä¸€å†…å®¹ä½œä¸º**ç‹¬ç«‹æ¶ˆæ¯**ï¼ˆä¸å¸¦æ–œæ ï¼‰ï¼š

```
stop
abort
esc
wait
exit
interrupt
```

è¿™äº›æ˜¯ä¸­æ­¢è§¦å‘å™¨ï¼ˆä¸æ˜¯æ–œæ å‘½ä»¤ï¼‰ã€‚

å¯¹äºŽåŽå°è¿›ç¨‹ï¼ˆæ¥è‡ª exec å·¥å…·ï¼‰ï¼Œä½ å¯ä»¥è¦æ±‚æ™ºèƒ½ä½“è¿è¡Œï¼š

```
process action:kill sessionId:XXX
```

æ–œæ å‘½ä»¤æ¦‚è§ˆï¼šå‚é˜…[æ–œæ å‘½ä»¤](/tools/slash-commands)ã€‚

å¤§å¤šæ•°å‘½ä»¤å¿…é¡»ä½œä¸ºä»¥ `/` å¼€å¤´çš„**ç‹¬ç«‹**æ¶ˆæ¯å‘é€ï¼Œä½†ä¸€äº›å¿«æ·æ–¹å¼ï¼ˆå¦‚ `/status`ï¼‰å¯¹å…è®¸åˆ—è¡¨ä¸­çš„å‘é€è€…ä¹Ÿæ”¯æŒå†…è”ä½¿ç”¨ã€‚

### å¦‚ä½•ä»Ž Telegram å‘é€ Discord æ¶ˆæ¯ï¼Ÿï¼ˆ"Cross-context messaging denied"ï¼‰

KrabKrab é»˜è®¤é˜»æ­¢**è·¨æä¾›å•†**æ¶ˆæ¯ã€‚å¦‚æžœå·¥å…·è°ƒç”¨ç»‘å®šåˆ° Telegramï¼Œé™¤éžä½ æ˜Žç¡®å…è®¸ï¼Œå¦åˆ™ä¸ä¼šå‘é€åˆ° Discordã€‚

ä¸ºæ™ºèƒ½ä½“å¯ç”¨è·¨æä¾›å•†æ¶ˆæ¯ï¼š

```json5
{
  agents: {
    defaults: {
      tools: {
        message: {
          crossContext: {
            allowAcrossProviders: true,
            marker: { enabled: true, prefix: "[from {channel}] " },
          },
        },
      },
    },
  },
}
```

ç¼–è¾‘é…ç½®åŽé‡å¯ Gateway ç½‘å…³ã€‚å¦‚æžœä½ åªæƒ³ä¸ºå•ä¸ªæ™ºèƒ½ä½“è®¾ç½®ï¼Œå°†å…¶æ”¾åœ¨ `agents.list[].tools.message` ä¸‹ã€‚

### ä¸ºä»€ä¹ˆæ„Ÿè§‰æœºå™¨äººâ€œå¿½ç•¥â€äº†å¿«é€Ÿè¿žå‘çš„æ¶ˆæ¯

é˜Ÿåˆ—æ¨¡å¼æŽ§åˆ¶æ–°æ¶ˆæ¯å¦‚ä½•ä¸Žæ­£åœ¨è¿›è¡Œçš„è¿è¡Œäº¤äº’ã€‚ä½¿ç”¨ `/queue` æ›´æ”¹æ¨¡å¼ï¼š

- `steer` - æ–°æ¶ˆæ¯é‡å®šå‘å½“å‰ä»»åŠ¡
- `followup` - é€æ¡å¤„ç†æ¶ˆæ¯
- `collect` - æ‰¹é‡æ¶ˆæ¯å¹¶å›žå¤ä¸€æ¬¡ï¼ˆé»˜è®¤ï¼‰
- `steer-backlog` - ç«‹å³è½¬å‘ï¼Œç„¶åŽå¤„ç†ç§¯åŽ‹
- `interrupt` - ä¸­æ­¢å½“å‰è¿è¡Œå¹¶é‡æ–°å¼€å§‹

ä½ å¯ä»¥ä¸º followup æ¨¡å¼æ·»åŠ é€‰é¡¹å¦‚ `debounce:2s cap:25 drop:summarize`ã€‚

## ä»Žæˆªå›¾/èŠå¤©è®°å½•ä¸­å‡†ç¡®å›žç­”é—®é¢˜

**é—®ï¼šâ€œä½¿ç”¨ API å¯†é’¥æ—¶ Anthropic çš„é»˜è®¤æ¨¡åž‹æ˜¯ä»€ä¹ˆï¼Ÿâ€**

**ç­”ï¼š** åœ¨ KrabKrab ä¸­ï¼Œå‡­æ®å’Œæ¨¡åž‹é€‰æ‹©æ˜¯åˆ†å¼€çš„ã€‚è®¾ç½® `ANTHROPIC_API_KEY`ï¼ˆæˆ–åœ¨è®¤è¯é…ç½®æ–‡ä»¶ä¸­å­˜å‚¨ Anthropic API å¯†é’¥ï¼‰å¯ç”¨è®¤è¯ï¼Œä½†å®žé™…çš„é»˜è®¤æ¨¡åž‹æ˜¯ä½ åœ¨ `agents.defaults.model.primary` ä¸­é…ç½®çš„ï¼ˆä¾‹å¦‚ `anthropic/claude-sonnet-4-5` æˆ– `anthropic/claude-opus-4-5`ï¼‰ã€‚å¦‚æžœä½ çœ‹åˆ° `No credentials found for profile "anthropic:default"`ï¼Œæ„å‘³ç€ Gateway ç½‘å…³åœ¨æ­£åœ¨è¿è¡Œçš„æ™ºèƒ½ä½“çš„é¢„æœŸ `auth-profiles.json` ä¸­æ‰¾ä¸åˆ° Anthropic å‡­æ®ã€‚

---

ä»ç„¶å¡ä½ï¼Ÿåœ¨ [Discord](https://discord.com/invite/clawd) ä¸­æé—®æˆ–å‘èµ· [GitHub è®¨è®º](https://github.com/krabkrab/krabkrab/discussions)ã€‚

