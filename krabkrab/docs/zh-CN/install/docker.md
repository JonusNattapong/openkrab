---
read_when:
  - ä½ æƒ³è¦å®¹å™¨åŒ–çš„ Gateway ç½‘å…³è€Œä¸æ˜¯æœ¬åœ°å®‰è£…
  - ä½ æ­£åœ¨éªŒè¯ Docker æµç¨‹
summary: KrabKrab çš„å¯é€‰ Docker è®¾ç½®å’Œæ–°æ‰‹å¼•å¯¼
title: Docker
x-i18n:
  generated_at: "2026-02-03T07:51:20Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: bd823e49b6ce76fe1136a42bf48f436b316ed1cd2f9612e3f4919f1e6b2cdee9
  source_path: install/docker.md
  workflow: 15
---

# Dockerï¼ˆå¯é€‰ï¼‰

Docker æ˜¯**å¯é€‰çš„**ã€‚ä»…å½“ä½ æƒ³è¦å®¹å™¨åŒ–çš„ Gateway ç½‘å…³æˆ–éªŒè¯ Docker æµç¨‹æ—¶æ‰ä½¿ç”¨å®ƒã€‚

## Docker é€‚åˆæˆ‘å—ï¼Ÿ

- **æ˜¯**ï¼šä½ æƒ³è¦ä¸€ä¸ªéš”ç¦»çš„ã€å¯ä¸¢å¼ƒçš„ Gateway ç½‘å…³çŽ¯å¢ƒï¼Œæˆ–åœ¨æ²¡æœ‰æœ¬åœ°å®‰è£…çš„ä¸»æœºä¸Šè¿è¡Œ KrabKrabã€‚
- **å¦**ï¼šä½ åœ¨è‡ªå·±çš„æœºå™¨ä¸Šè¿è¡Œï¼Œåªæƒ³è¦æœ€å¿«çš„å¼€å‘å¾ªçŽ¯ã€‚è¯·æ”¹ç”¨æ­£å¸¸çš„å®‰è£…æµç¨‹ã€‚
- **æ²™ç®±æ³¨æ„äº‹é¡¹**ï¼šæ™ºèƒ½ä½“æ²™ç®±éš”ç¦»ä¹Ÿä½¿ç”¨ Dockerï¼Œä½†å®ƒ**ä¸éœ€è¦**å®Œæ•´çš„ Gateway ç½‘å…³åœ¨ Docker ä¸­è¿è¡Œã€‚å‚é˜…[æ²™ç®±éš”ç¦»](/gateway/sandboxing)ã€‚

æœ¬æŒ‡å—æ¶µç›–ï¼š

- å®¹å™¨åŒ– Gateway ç½‘å…³ï¼ˆå®Œæ•´çš„ KrabKrab åœ¨ Docker ä¸­ï¼‰
- æ¯ä¼šè¯æ™ºèƒ½ä½“æ²™ç®±ï¼ˆä¸»æœº Gateway ç½‘å…³ + Docker éš”ç¦»çš„æ™ºèƒ½ä½“å·¥å…·ï¼‰

æ²™ç®±éš”ç¦»è¯¦æƒ…ï¼š[æ²™ç®±éš”ç¦»](/gateway/sandboxing)

## è¦æ±‚

- Docker Desktopï¼ˆæˆ– Docker Engineï¼‰+ Docker Compose v2
- è¶³å¤Ÿçš„ç£ç›˜ç©ºé—´ç”¨äºŽé•œåƒ + æ—¥å¿—

## å®¹å™¨åŒ– Gateway ç½‘å…³ï¼ˆDocker Composeï¼‰

### å¿«é€Ÿå¼€å§‹ï¼ˆæŽ¨èï¼‰

ä»Žä»“åº“æ ¹ç›®å½•ï¼š

```bash
./docker-setup.sh
```

æ­¤è„šæœ¬ï¼š

- æž„å»º Gateway ç½‘å…³é•œåƒ
- è¿è¡Œæ–°æ‰‹å¼•å¯¼å‘å¯¼
- æ‰“å°å¯é€‰çš„æä¾›å•†è®¾ç½®æç¤º
- é€šè¿‡ Docker Compose å¯åŠ¨ Gateway ç½‘å…³
- ç”Ÿæˆ Gateway ç½‘å…³ä»¤ç‰Œå¹¶å†™å…¥ `.env`

å¯é€‰çŽ¯å¢ƒå˜é‡ï¼š

- `krabkrab_DOCKER_APT_PACKAGES` â€” åœ¨æž„å»ºæœŸé—´å®‰è£…é¢å¤–çš„ apt åŒ…
- `krabkrab_EXTRA_MOUNTS` â€” æ·»åŠ é¢å¤–çš„ä¸»æœºç»‘å®šæŒ‚è½½
- `krabkrab_HOME_VOLUME` â€” åœ¨å‘½åå·ä¸­æŒä¹…åŒ– `/home/node`

å®ŒæˆåŽï¼š

- åœ¨æµè§ˆå™¨ä¸­æ‰“å¼€ `http://127.0.0.1:18789/`ã€‚
- å°†ä»¤ç‰Œç²˜è´´åˆ°æŽ§åˆ¶ UIï¼ˆè®¾ç½® â†’ tokenï¼‰ã€‚
- éœ€è¦å†æ¬¡èŽ·å–å¸¦ä»¤ç‰Œçš„ URLï¼Ÿè¿è¡Œ `docker compose run --rm krabkrab-cli dashboard --no-open`ã€‚

å®ƒåœ¨ä¸»æœºä¸Šå†™å…¥é…ç½®/å·¥ä½œåŒºï¼š

- `~/.krabkrab/`
- `~/.krabkrab/workspace`

åœ¨ VPS ä¸Šè¿è¡Œï¼Ÿå‚é˜… [Hetznerï¼ˆDocker VPSï¼‰](/install/hetzner)ã€‚

### æ‰‹åŠ¨æµç¨‹ï¼ˆcomposeï¼‰

```bash
docker build -t krabkrab:local -f Dockerfile .
docker compose run --rm krabkrab-cli onboard
docker compose up -d krabkrab-gateway
```

æ³¨æ„ï¼šä»Žä»“åº“æ ¹ç›®å½•è¿è¡Œ `docker compose ...`ã€‚å¦‚æžœä½ å¯ç”¨äº† `krabkrab_EXTRA_MOUNTS` æˆ– `krabkrab_HOME_VOLUME`ï¼Œè®¾ç½®è„šæœ¬ä¼šå†™å…¥ `docker-compose.extra.yml`ï¼›åœ¨å…¶ä»–åœ°æ–¹è¿è¡Œ Compose æ—¶åŒ…å«å®ƒï¼š

```bash
docker compose -f docker-compose.yml -f docker-compose.extra.yml <command>
```

### æŽ§åˆ¶ UI ä»¤ç‰Œ + é…å¯¹ï¼ˆDockerï¼‰

å¦‚æžœä½ çœ‹åˆ°"unauthorized"æˆ–"disconnected (1008): pairing required"ï¼ŒèŽ·å–æ–°çš„ä»ªè¡¨æ¿é“¾æŽ¥å¹¶æ‰¹å‡†æµè§ˆå™¨è®¾å¤‡ï¼š

```bash
docker compose run --rm krabkrab-cli dashboard --no-open
docker compose run --rm krabkrab-cli devices list
docker compose run --rm krabkrab-cli devices approve <requestId>
```

æ›´å¤šè¯¦æƒ…ï¼š[ä»ªè¡¨æ¿](/web/dashboard)ï¼Œ[è®¾å¤‡](/cli/devices)ã€‚

### é¢å¤–æŒ‚è½½ï¼ˆå¯é€‰ï¼‰

å¦‚æžœä½ æƒ³å°†é¢å¤–çš„ä¸»æœºç›®å½•æŒ‚è½½åˆ°å®¹å™¨ä¸­ï¼Œåœ¨è¿è¡Œ `docker-setup.sh` ä¹‹å‰è®¾ç½® `krabkrab_EXTRA_MOUNTS`ã€‚è¿™æŽ¥å—é€—å·åˆ†éš”çš„ Docker ç»‘å®šæŒ‚è½½åˆ—è¡¨ï¼Œå¹¶é€šè¿‡ç”Ÿæˆ `docker-compose.extra.yml` å°†å®ƒä»¬åº”ç”¨åˆ° `krabkrab-gateway` å’Œ `krabkrab-cli`ã€‚

ç¤ºä¾‹ï¼š

```bash
export krabkrab_EXTRA_MOUNTS="$HOME/.codex:/home/node/.codex:ro,$HOME/github:/home/node/github:rw"
./docker-setup.sh
```

æ³¨æ„ï¼š

- è·¯å¾„å¿…é¡»åœ¨ macOS/Windows ä¸Šä¸Ž Docker Desktop å…±äº«ã€‚
- å¦‚æžœä½ ç¼–è¾‘ `krabkrab_EXTRA_MOUNTS`ï¼Œé‡æ–°è¿è¡Œ `docker-setup.sh` ä»¥é‡æ–°ç”Ÿæˆé¢å¤–çš„ compose æ–‡ä»¶ã€‚
- `docker-compose.extra.yml` æ˜¯ç”Ÿæˆçš„ã€‚ä¸è¦æ‰‹åŠ¨ç¼–è¾‘å®ƒã€‚

### æŒä¹…åŒ–æ•´ä¸ªå®¹å™¨ homeï¼ˆå¯é€‰ï¼‰

å¦‚æžœä½ æƒ³è®© `/home/node` åœ¨å®¹å™¨é‡å»ºåŽæŒä¹…åŒ–ï¼Œé€šè¿‡ `krabkrab_HOME_VOLUME` è®¾ç½®ä¸€ä¸ªå‘½åå·ã€‚è¿™ä¼šåˆ›å»ºä¸€ä¸ª Docker å·å¹¶å°†å…¶æŒ‚è½½åˆ° `/home/node`ï¼ŒåŒæ—¶ä¿æŒæ ‡å‡†çš„é…ç½®/å·¥ä½œåŒºç»‘å®šæŒ‚è½½ã€‚è¿™é‡Œä½¿ç”¨å‘½åå·ï¼ˆä¸æ˜¯ç»‘å®šè·¯å¾„ï¼‰ï¼›å¯¹äºŽç»‘å®šæŒ‚è½½ï¼Œä½¿ç”¨ `krabkrab_EXTRA_MOUNTS`ã€‚

ç¤ºä¾‹ï¼š

```bash
export krabkrab_HOME_VOLUME="krabkrab_home"
./docker-setup.sh
```

ä½ å¯ä»¥å°†å…¶ä¸Žé¢å¤–æŒ‚è½½ç»“åˆä½¿ç”¨ï¼š

```bash
export krabkrab_HOME_VOLUME="krabkrab_home"
export krabkrab_EXTRA_MOUNTS="$HOME/.codex:/home/node/.codex:ro,$HOME/github:/home/node/github:rw"
./docker-setup.sh
```

æ³¨æ„ï¼š

- å¦‚æžœä½ æ›´æ”¹ `krabkrab_HOME_VOLUME`ï¼Œé‡æ–°è¿è¡Œ `docker-setup.sh` ä»¥é‡æ–°ç”Ÿæˆé¢å¤–çš„ compose æ–‡ä»¶ã€‚
- å‘½åå·ä¼šæŒä¹…åŒ–ç›´åˆ°ä½¿ç”¨ `docker volume rm <name>` åˆ é™¤ã€‚

### å®‰è£…é¢å¤–çš„ apt åŒ…ï¼ˆå¯é€‰ï¼‰

å¦‚æžœä½ éœ€è¦é•œåƒå†…çš„ç³»ç»ŸåŒ…ï¼ˆä¾‹å¦‚æž„å»ºå·¥å…·æˆ–åª’ä½“åº“ï¼‰ï¼Œåœ¨è¿è¡Œ `docker-setup.sh` ä¹‹å‰è®¾ç½® `krabkrab_DOCKER_APT_PACKAGES`ã€‚è¿™ä¼šåœ¨é•œåƒæž„å»ºæœŸé—´å®‰è£…åŒ…ï¼Œå› æ­¤å³ä½¿å®¹å™¨è¢«åˆ é™¤å®ƒä»¬ä¹Ÿä¼šæŒä¹…åŒ–ã€‚

ç¤ºä¾‹ï¼š

```bash
export krabkrab_DOCKER_APT_PACKAGES="ffmpeg build-essential"
./docker-setup.sh
```

æ³¨æ„ï¼š

- è¿™æŽ¥å—ç©ºæ ¼åˆ†éš”çš„ apt åŒ…åç§°åˆ—è¡¨ã€‚
- å¦‚æžœä½ æ›´æ”¹ `krabkrab_DOCKER_APT_PACKAGES`ï¼Œé‡æ–°è¿è¡Œ `docker-setup.sh` ä»¥é‡å»ºé•œåƒã€‚

### é«˜çº§ç”¨æˆ·/åŠŸèƒ½å®Œæ•´çš„å®¹å™¨ï¼ˆé€‰æ‹©åŠ å…¥ï¼‰

é»˜è®¤çš„ Docker é•œåƒæ˜¯**å®‰å…¨ä¼˜å…ˆ**çš„ï¼Œä»¥éž root çš„ `node` ç”¨æˆ·è¿è¡Œã€‚è¿™ä¿æŒäº†è¾ƒå°çš„æ”»å‡»é¢ï¼Œä½†è¿™æ„å‘³ç€ï¼š

- è¿è¡Œæ—¶æ— æ³•å®‰è£…ç³»ç»ŸåŒ…
- é»˜è®¤æ²¡æœ‰ Homebrew
- æ²¡æœ‰æ†ç»‘çš„ Chromium/Playwright æµè§ˆå™¨

å¦‚æžœä½ æƒ³è¦åŠŸèƒ½æ›´å®Œæ•´çš„å®¹å™¨ï¼Œä½¿ç”¨è¿™äº›é€‰æ‹©åŠ å…¥é€‰é¡¹ï¼š

1. **æŒä¹…åŒ– `/home/node`** ä»¥ä¾¿æµè§ˆå™¨ä¸‹è½½å’Œå·¥å…·ç¼“å­˜èƒ½å¤Ÿä¿ç•™ï¼š

```bash
export krabkrab_HOME_VOLUME="krabkrab_home"
./docker-setup.sh
```

2. **å°†ç³»ç»Ÿä¾èµ–çƒ˜ç„™åˆ°é•œåƒä¸­**ï¼ˆå¯é‡å¤ + æŒä¹…åŒ–ï¼‰ï¼š

```bash
export krabkrab_DOCKER_APT_PACKAGES="git curl jq"
./docker-setup.sh
```

3. **ä¸ä½¿ç”¨ `npx` å®‰è£… Playwright æµè§ˆå™¨**ï¼ˆé¿å… npm è¦†ç›–å†²çªï¼‰ï¼š

```bash
docker compose run --rm krabkrab-cli \
  node /app/node_modules/playwright-core/cli.js install chromium
```

å¦‚æžœä½ éœ€è¦ Playwright å®‰è£…ç³»ç»Ÿä¾èµ–ï¼Œä½¿ç”¨ `krabkrab_DOCKER_APT_PACKAGES` é‡å»ºé•œåƒï¼Œè€Œä¸æ˜¯åœ¨è¿è¡Œæ—¶ä½¿ç”¨ `--with-deps`ã€‚

4. **æŒä¹…åŒ– Playwright æµè§ˆå™¨ä¸‹è½½**ï¼š

- åœ¨ `docker-compose.yml` ä¸­è®¾ç½® `PLAYWRIGHT_BROWSERS_PATH=/home/node/.cache/ms-playwright`ã€‚
- ç¡®ä¿ `/home/node` é€šè¿‡ `krabkrab_HOME_VOLUME` æŒä¹…åŒ–ï¼Œæˆ–é€šè¿‡ `krabkrab_EXTRA_MOUNTS` æŒ‚è½½ `/home/node/.cache/ms-playwright`ã€‚

### æƒé™ + EACCES

é•œåƒä»¥ `node`ï¼ˆuid 1000ï¼‰è¿è¡Œã€‚å¦‚æžœä½ åœ¨ `/home/node/.krabkrab` ä¸Šçœ‹åˆ°æƒé™é”™è¯¯ï¼Œç¡®ä¿ä½ çš„ä¸»æœºç»‘å®šæŒ‚è½½ç”± uid 1000 æ‹¥æœ‰ã€‚

ç¤ºä¾‹ï¼ˆLinux ä¸»æœºï¼‰ï¼š

```bash
sudo chown -R 1000:1000 /path/to/krabkrab-config /path/to/krabkrab-workspace
```

å¦‚æžœä½ é€‰æ‹©ä»¥ root è¿è¡Œä»¥æ–¹ä¾¿ä½¿ç”¨ï¼Œä½ æŽ¥å—äº†å®‰å…¨æƒè¡¡ã€‚

### æ›´å¿«çš„é‡å»ºï¼ˆæŽ¨èï¼‰

è¦åŠ é€Ÿé‡å»ºï¼ŒæŽ’åºä½ çš„ Dockerfile ä»¥ä¾¿ä¾èµ–å±‚è¢«ç¼“å­˜ã€‚è¿™é¿å…äº†é™¤éžé”æ–‡ä»¶æ›´æ”¹å¦åˆ™é‡æ–°è¿è¡Œ `pnpm install`ï¼š

```dockerfile
FROM node:22-bookworm

# å®‰è£… Bunï¼ˆæž„å»ºè„šæœ¬éœ€è¦ï¼‰
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

RUN corepack enable

WORKDIR /app

# ç¼“å­˜ä¾èµ–ï¼Œé™¤éžåŒ…å…ƒæ•°æ®æ›´æ”¹
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml .npmrc ./
COPY ui/package.json ./ui/package.json
COPY scripts ./scripts

RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build
RUN pnpm ui:install
RUN pnpm ui:build

ENV NODE_ENV=production

CMD ["node","dist/index.js"]
```

### æ¸ é“è®¾ç½®ï¼ˆå¯é€‰ï¼‰

ä½¿ç”¨ CLI å®¹å™¨é…ç½®æ¸ é“ï¼Œç„¶åŽåœ¨éœ€è¦æ—¶é‡å¯ Gateway ç½‘å…³ã€‚

WhatsAppï¼ˆQRï¼‰ï¼š

```bash
docker compose run --rm krabkrab-cli channels login
```

Telegramï¼ˆbot tokenï¼‰ï¼š

```bash
docker compose run --rm krabkrab-cli channels add --channel telegram --token "<token>"
```

Discordï¼ˆbot tokenï¼‰ï¼š

```bash
docker compose run --rm krabkrab-cli channels add --channel discord --token "<token>"
```

æ–‡æ¡£ï¼š[WhatsApp](/channels/whatsapp)ï¼Œ[Telegram](/channels/telegram)ï¼Œ[Discord](/channels/discord)

### OpenAI Codex OAuthï¼ˆæ— å¤´ Dockerï¼‰

å¦‚æžœä½ åœ¨å‘å¯¼ä¸­é€‰æ‹© OpenAI Codex OAuthï¼Œå®ƒä¼šæ‰“å¼€æµè§ˆå™¨ URL å¹¶å°è¯•åœ¨ `http://127.0.0.1:1455/auth/callback` æ•èŽ·å›žè°ƒã€‚åœ¨ Docker æˆ–æ— å¤´è®¾ç½®ä¸­ï¼Œè¯¥å›žè°ƒå¯èƒ½æ˜¾ç¤ºæµè§ˆå™¨é”™è¯¯ã€‚å¤åˆ¶ä½ åˆ°è¾¾çš„å®Œæ•´é‡å®šå‘ URL å¹¶å°†å…¶ç²˜è´´å›žå‘å¯¼ä»¥å®Œæˆè®¤è¯ã€‚

### å¥åº·æ£€æŸ¥

```bash
docker compose exec krabkrab-gateway node dist/index.js health --token "$krabkrab_GATEWAY_TOKEN"
```

### E2E å†’çƒŸæµ‹è¯•ï¼ˆDockerï¼‰

```bash
scripts/e2e/onboard-docker.sh
```

### QR å¯¼å…¥å†’çƒŸæµ‹è¯•ï¼ˆDockerï¼‰

```bash
pnpm test:docker:qr
```

### æ³¨æ„

- Gateway ç½‘å…³ç»‘å®šé»˜è®¤ä¸º `lan` ç”¨äºŽå®¹å™¨ä½¿ç”¨ã€‚
- Dockerfile CMD ä½¿ç”¨ `--allow-unconfigured`ï¼›æŒ‚è½½çš„é…ç½®å¦‚æžœ `gateway.mode` ä¸æ˜¯ `local` ä»ä¼šå¯åŠ¨ã€‚è¦†ç›– CMD ä»¥å¼ºåˆ¶æ‰§è¡Œæ£€æŸ¥ã€‚
- Gateway ç½‘å…³å®¹å™¨æ˜¯ä¼šè¯çš„çœŸå®žæ¥æºï¼ˆ`~/.krabkrab/agents/<agentId>/sessions/`ï¼‰ã€‚

## æ™ºèƒ½ä½“æ²™ç®±ï¼ˆä¸»æœº Gateway ç½‘å…³ + Docker å·¥å…·ï¼‰

æ·±å…¥äº†è§£ï¼š[æ²™ç®±éš”ç¦»](/gateway/sandboxing)

### å®ƒåšä»€ä¹ˆ

å½“å¯ç”¨ `agents.defaults.sandbox` æ—¶ï¼Œ**éžä¸»ä¼šè¯**åœ¨ Docker å®¹å™¨å†…è¿è¡Œå·¥å…·ã€‚Gateway ç½‘å…³ä¿æŒåœ¨ä½ çš„ä¸»æœºä¸Šï¼Œä½†å·¥å…·æ‰§è¡Œæ˜¯éš”ç¦»çš„ï¼š

- scopeï¼šé»˜è®¤ä¸º `"agent"`ï¼ˆæ¯ä¸ªæ™ºèƒ½ä½“ä¸€ä¸ªå®¹å™¨ + å·¥ä½œåŒºï¼‰
- scopeï¼š`"session"` ç”¨äºŽæ¯ä¼šè¯éš”ç¦»
- æ¯ä½œç”¨åŸŸå·¥ä½œåŒºæ–‡ä»¶å¤¹æŒ‚è½½åœ¨ `/workspace`
- å¯é€‰çš„æ™ºèƒ½ä½“å·¥ä½œåŒºè®¿é—®ï¼ˆ`agents.defaults.sandbox.workspaceAccess`ï¼‰
- å…è®¸/æ‹’ç»å·¥å…·ç­–ç•¥ï¼ˆæ‹’ç»ä¼˜å…ˆï¼‰
- å…¥ç«™åª’ä½“è¢«å¤åˆ¶åˆ°æ´»åŠ¨æ²™ç®±å·¥ä½œåŒºï¼ˆ`media/inbound/*`ï¼‰ï¼Œä»¥ä¾¿å·¥å…·å¯ä»¥è¯»å–å®ƒï¼ˆä½¿ç”¨ `workspaceAccess: "rw"` æ—¶ï¼Œè¿™ä¼šè½åœ¨æ™ºèƒ½ä½“å·¥ä½œåŒºä¸­ï¼‰

è­¦å‘Šï¼š`scope: "shared"` ç¦ç”¨è·¨ä¼šè¯éš”ç¦»ã€‚æ‰€æœ‰ä¼šè¯å…±äº«ä¸€ä¸ªå®¹å™¨å’Œä¸€ä¸ªå·¥ä½œåŒºã€‚

### æ¯æ™ºèƒ½ä½“æ²™ç®±é…ç½®æ–‡ä»¶ï¼ˆå¤šæ™ºèƒ½ä½“ï¼‰

å¦‚æžœä½ ä½¿ç”¨å¤šæ™ºèƒ½ä½“è·¯ç”±ï¼Œæ¯ä¸ªæ™ºèƒ½ä½“å¯ä»¥è¦†ç›–æ²™ç®± + å·¥å…·è®¾ç½®ï¼š`agents.list[].sandbox` å’Œ `agents.list[].tools`ï¼ˆåŠ ä¸Š `agents.list[].tools.sandbox.tools`ï¼‰ã€‚è¿™è®©ä½ å¯ä»¥åœ¨ä¸€ä¸ª Gateway ç½‘å…³ä¸­è¿è¡Œæ··åˆè®¿é—®çº§åˆ«ï¼š

- å®Œå…¨è®¿é—®ï¼ˆä¸ªäººæ™ºèƒ½ä½“ï¼‰
- åªè¯»å·¥å…· + åªè¯»å·¥ä½œåŒºï¼ˆå®¶åº­/å·¥ä½œæ™ºèƒ½ä½“ï¼‰
- æ— æ–‡ä»¶ç³»ç»Ÿ/shell å·¥å…·ï¼ˆå…¬å…±æ™ºèƒ½ä½“ï¼‰

å‚é˜…[å¤šæ™ºèƒ½ä½“æ²™ç®±ä¸Žå·¥å…·](/tools/multi-agent-sandbox-tools)äº†è§£ç¤ºä¾‹ã€ä¼˜å…ˆçº§å’Œæ•…éšœæŽ’é™¤ã€‚

### é»˜è®¤è¡Œä¸º

- é•œåƒï¼š`krabkrab-sandbox:bookworm-slim`
- æ¯ä¸ªæ™ºèƒ½ä½“ä¸€ä¸ªå®¹å™¨
- æ™ºèƒ½ä½“å·¥ä½œåŒºè®¿é—®ï¼š`workspaceAccess: "none"`ï¼ˆé»˜è®¤ï¼‰ä½¿ç”¨ `~/.krabkrab/sandboxes`
  - `"ro"` ä¿æŒæ²™ç®±å·¥ä½œåŒºåœ¨ `/workspace` å¹¶å°†æ™ºèƒ½ä½“å·¥ä½œåŒºåªè¯»æŒ‚è½½åœ¨ `/agent`ï¼ˆç¦ç”¨ `write`/`edit`/`apply_patch`ï¼‰
  - `"rw"` å°†æ™ºèƒ½ä½“å·¥ä½œåŒºè¯»å†™æŒ‚è½½åœ¨ `/workspace`
- è‡ªåŠ¨æ¸…ç†ï¼šç©ºé—² > 24h æˆ– å¹´é¾„ > 7d
- ç½‘ç»œï¼šé»˜è®¤ä¸º `none`ï¼ˆå¦‚æžœéœ€è¦å‡ºç«™åˆ™æ˜Žç¡®é€‰æ‹©åŠ å…¥ï¼‰
- é»˜è®¤å…è®¸ï¼š`exec`ã€`process`ã€`read`ã€`write`ã€`edit`ã€`sessions_list`ã€`sessions_history`ã€`sessions_send`ã€`sessions_spawn`ã€`session_status`
- é»˜è®¤æ‹’ç»ï¼š`browser`ã€`canvas`ã€`nodes`ã€`cron`ã€`discord`ã€`gateway`

### å¯ç”¨æ²™ç®±éš”ç¦»

å¦‚æžœä½ è®¡åˆ’åœ¨ `setupCommand` ä¸­å®‰è£…åŒ…ï¼Œè¯·æ³¨æ„ï¼š

- é»˜è®¤ `docker.network` æ˜¯ `"none"`ï¼ˆæ— å‡ºç«™ï¼‰ã€‚
- `readOnlyRoot: true` é˜»æ­¢åŒ…å®‰è£…ã€‚
- `user` å¿…é¡»æ˜¯ root æ‰èƒ½è¿è¡Œ `apt-get`ï¼ˆçœç•¥ `user` æˆ–è®¾ç½® `user: "0:0"`ï¼‰ã€‚
  å½“ `setupCommand`ï¼ˆæˆ– docker é…ç½®ï¼‰æ›´æ”¹æ—¶ï¼ŒKrabKrab ä¼šè‡ªåŠ¨é‡å»ºå®¹å™¨ï¼Œé™¤éžå®¹å™¨æ˜¯**æœ€è¿‘ä½¿ç”¨çš„**ï¼ˆåœ¨çº¦ 5 åˆ†é’Ÿå†…ï¼‰ã€‚çƒ­å®¹å™¨ä¼šè®°å½•è­¦å‘Šï¼ŒåŒ…å«ç¡®åˆ‡çš„ `krabkrab sandbox recreate ...` å‘½ä»¤ã€‚

```json5
{
  agents: {
    defaults: {
      sandbox: {
        mode: "non-main", // off | non-main | all
        scope: "agent", // session | agent | sharedï¼ˆé»˜è®¤ä¸º agentï¼‰
        workspaceAccess: "none", // none | ro | rw
        workspaceRoot: "~/.krabkrab/sandboxes",
        docker: {
          image: "krabkrab-sandbox:bookworm-slim",
          workdir: "/workspace",
          readOnlyRoot: true,
          tmpfs: ["/tmp", "/var/tmp", "/run"],
          network: "none",
          user: "1000:1000",
          capDrop: ["ALL"],
          env: { LANG: "C.UTF-8" },
          setupCommand: "apt-get update && apt-get install -y git curl jq",
          pidsLimit: 256,
          memory: "1g",
          memorySwap: "2g",
          cpus: 1,
          ulimits: {
            nofile: { soft: 1024, hard: 2048 },
            nproc: 256,
          },
          seccompProfile: "/path/to/seccomp.json",
          apparmorProfile: "krabkrab-sandbox",
          dns: ["1.1.1.1", "8.8.8.8"],
          extraHosts: ["internal.service:10.0.0.5"],
        },
        prune: {
          idleHours: 24, // 0 ç¦ç”¨ç©ºé—²æ¸…ç†
          maxAgeDays: 7, // 0 ç¦ç”¨æœ€å¤§å¹´é¾„æ¸…ç†
        },
      },
    },
  },
  tools: {
    sandbox: {
      tools: {
        allow: [
          "exec",
          "process",
          "read",
          "write",
          "edit",
          "sessions_list",
          "sessions_history",
          "sessions_send",
          "sessions_spawn",
          "session_status",
        ],
        deny: ["browser", "canvas", "nodes", "cron", "discord", "gateway"],
      },
    },
  },
}
```

åŠ å›ºé€‰é¡¹ä½äºŽ `agents.defaults.sandbox.docker` ä¸‹ï¼š`network`ã€`user`ã€`pidsLimit`ã€`memory`ã€`memorySwap`ã€`cpus`ã€`ulimits`ã€`seccompProfile`ã€`apparmorProfile`ã€`dns`ã€`extraHosts`ã€‚

å¤šæ™ºèƒ½ä½“ï¼šé€šè¿‡ `agents.list[].sandbox.{docker,browser,prune}.*` æŒ‰æ™ºèƒ½ä½“è¦†ç›– `agents.defaults.sandbox.{docker,browser,prune}.*`ï¼ˆå½“ `agents.defaults.sandbox.scope` / `agents.list[].sandbox.scope` æ˜¯ `"shared"` æ—¶å¿½ç•¥ï¼‰ã€‚

### æž„å»ºé»˜è®¤æ²™ç®±é•œåƒ

```bash
scripts/sandbox-setup.sh
```

è¿™ä½¿ç”¨ `Dockerfile.sandbox` æž„å»º `krabkrab-sandbox:bookworm-slim`ã€‚

### æ²™ç®±é€šç”¨é•œåƒï¼ˆå¯é€‰ï¼‰

å¦‚æžœä½ æƒ³è¦ä¸€ä¸ªå¸¦æœ‰å¸¸è§æž„å»ºå·¥å…·ï¼ˆNodeã€Goã€Rust ç­‰ï¼‰çš„æ²™ç®±é•œåƒï¼Œæž„å»ºé€šç”¨é•œåƒï¼š

```bash
scripts/sandbox-common-setup.sh
```

è¿™æž„å»º `krabkrab-sandbox-common:bookworm-slim`ã€‚è¦ä½¿ç”¨å®ƒï¼š

```json5
{
  agents: {
    defaults: {
      sandbox: { docker: { image: "krabkrab-sandbox-common:bookworm-slim" } },
    },
  },
}
```

### æ²™ç®±æµè§ˆå™¨é•œåƒ

è¦åœ¨æ²™ç®±å†…è¿è¡Œæµè§ˆå™¨å·¥å…·ï¼Œæž„å»ºæµè§ˆå™¨é•œåƒï¼š

```bash
scripts/sandbox-browser-setup.sh
```

è¿™ä½¿ç”¨ `Dockerfile.sandbox-browser` æž„å»º `krabkrab-sandbox-browser:bookworm-slim`ã€‚å®¹å™¨è¿è¡Œå¯ç”¨ CDP çš„ Chromium å’Œå¯é€‰çš„ noVNC è§‚å¯Ÿå™¨ï¼ˆé€šè¿‡ Xvfb æœ‰å¤´ï¼‰ã€‚

æ³¨æ„ï¼š

- æœ‰å¤´ï¼ˆXvfbï¼‰æ¯”æ— å¤´å‡å°‘æœºå™¨äººé˜»æ­¢ã€‚
- é€šè¿‡è®¾ç½® `agents.defaults.sandbox.browser.headless=true` ä»ç„¶å¯ä»¥ä½¿ç”¨æ— å¤´æ¨¡å¼ã€‚
- ä¸éœ€è¦å®Œæ•´çš„æ¡Œé¢çŽ¯å¢ƒï¼ˆGNOMEï¼‰ï¼›Xvfb æä¾›æ˜¾ç¤ºã€‚

ä½¿ç”¨é…ç½®ï¼š

```json5
{
  agents: {
    defaults: {
      sandbox: {
        browser: { enabled: true },
      },
    },
  },
}
```

è‡ªå®šä¹‰æµè§ˆå™¨é•œåƒï¼š

```json5
{
  agents: {
    defaults: {
      sandbox: { browser: { image: "my-krabkrab-browser" } },
    },
  },
}
```

å¯ç”¨åŽï¼Œæ™ºèƒ½ä½“æŽ¥æ”¶ï¼š

- æ²™ç®±æµè§ˆå™¨æŽ§åˆ¶ URLï¼ˆç”¨äºŽ `browser` å·¥å…·ï¼‰
- noVNC URLï¼ˆå¦‚æžœå¯ç”¨ä¸” headless=falseï¼‰

è®°ä½ï¼šå¦‚æžœä½ ä½¿ç”¨å·¥å…·å…è®¸åˆ—è¡¨ï¼Œæ·»åŠ  `browser`ï¼ˆå¹¶ä»Žæ‹’ç»ä¸­ç§»é™¤å®ƒï¼‰å¦åˆ™å·¥å…·ä»ç„¶è¢«é˜»æ­¢ã€‚
æ¸…ç†è§„åˆ™ï¼ˆ`agents.defaults.sandbox.prune`ï¼‰ä¹Ÿé€‚ç”¨äºŽæµè§ˆå™¨å®¹å™¨ã€‚

### è‡ªå®šä¹‰æ²™ç®±é•œåƒ

æž„å»ºä½ è‡ªå·±çš„é•œåƒå¹¶å°†é…ç½®æŒ‡å‘å®ƒï¼š

```bash
docker build -t my-krabkrab-sbx -f Dockerfile.sandbox .
```

```json5
{
  agents: {
    defaults: {
      sandbox: { docker: { image: "my-krabkrab-sbx" } },
    },
  },
}
```

### å·¥å…·ç­–ç•¥ï¼ˆå…è®¸/æ‹’ç»ï¼‰

- `deny` ä¼˜å…ˆäºŽ `allow`ã€‚
- å¦‚æžœ `allow` ä¸ºç©ºï¼šæ‰€æœ‰å·¥å…·ï¼ˆé™¤äº† denyï¼‰éƒ½å¯ç”¨ã€‚
- å¦‚æžœ `allow` éžç©ºï¼šåªæœ‰ `allow` ä¸­çš„å·¥å…·å¯ç”¨ï¼ˆå‡åŽ» denyï¼‰ã€‚

### æ¸…ç†ç­–ç•¥

ä¸¤ä¸ªé€‰é¡¹ï¼š

- `prune.idleHours`ï¼šç§»é™¤ X å°æ—¶æœªä½¿ç”¨çš„å®¹å™¨ï¼ˆ0 = ç¦ç”¨ï¼‰
- `prune.maxAgeDays`ï¼šç§»é™¤è¶…è¿‡ X å¤©çš„å®¹å™¨ï¼ˆ0 = ç¦ç”¨ï¼‰

ç¤ºä¾‹ï¼š

- ä¿ç•™ç¹å¿™ä¼šè¯ä½†é™åˆ¶ç”Ÿå‘½å‘¨æœŸï¼š
  `idleHours: 24`ã€`maxAgeDays: 7`
- æ°¸ä¸æ¸…ç†ï¼š
  `idleHours: 0`ã€`maxAgeDays: 0`

### å®‰å…¨æ³¨æ„äº‹é¡¹

- ç¡¬éš”ç¦»ä»…é€‚ç”¨äºŽ**å·¥å…·**ï¼ˆexec/read/write/edit/apply_patchï¼‰ã€‚
- ä»…ä¸»æœºå·¥å…·å¦‚ browser/camera/canvas é»˜è®¤è¢«é˜»æ­¢ã€‚
- åœ¨æ²™ç®±ä¸­å…è®¸ `browser` **ä¼šç ´åéš”ç¦»**ï¼ˆæµè§ˆå™¨åœ¨ä¸»æœºä¸Šè¿è¡Œï¼‰ã€‚

## æ•…éšœæŽ’é™¤

- é•œåƒç¼ºå¤±ï¼šä½¿ç”¨ [`scripts/sandbox-setup.sh`](https://github.com/krabkrab/krabkrab/blob/main/scripts/sandbox-setup.sh) æž„å»ºæˆ–è®¾ç½® `agents.defaults.sandbox.docker.image`ã€‚
- å®¹å™¨æœªè¿è¡Œï¼šå®ƒä¼šæŒ‰éœ€ä¸ºæ¯ä¸ªä¼šè¯è‡ªåŠ¨åˆ›å»ºã€‚
- æ²™ç®±ä¸­çš„æƒé™é”™è¯¯ï¼šå°† `docker.user` è®¾ç½®ä¸ºä¸Žä½ æŒ‚è½½çš„å·¥ä½œåŒºæ‰€æœ‰æƒåŒ¹é…çš„ UID:GIDï¼ˆæˆ– chown å·¥ä½œåŒºæ–‡ä»¶å¤¹ï¼‰ã€‚
- æ‰¾ä¸åˆ°è‡ªå®šä¹‰å·¥å…·ï¼šKrabKrab ä½¿ç”¨ `sh -lc`ï¼ˆç™»å½• shellï¼‰è¿è¡Œå‘½ä»¤ï¼Œè¿™ä¼š source `/etc/profile` å¹¶å¯èƒ½é‡ç½® PATHã€‚è®¾ç½® `docker.env.PATH` ä»¥åœ¨å‰é¢æ·»åŠ ä½ çš„è‡ªå®šä¹‰å·¥å…·è·¯å¾„ï¼ˆä¾‹å¦‚ `/custom/bin:/usr/local/share/npm-global/bin`ï¼‰ï¼Œæˆ–åœ¨ä½ çš„ Dockerfile ä¸­åœ¨ `/etc/profile.d/` ä¸‹æ·»åŠ è„šæœ¬ã€‚

