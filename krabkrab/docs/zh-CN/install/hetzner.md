---
read_when:
  - ä½ æƒ³è®© KrabKrab åœ¨äº‘ VPS ä¸Š 24/7 è¿è¡Œï¼ˆè€Œä¸æ˜¯ä½ çš„ç¬”è®°æœ¬ç”µè„‘ï¼‰
  - ä½ æƒ³åœ¨è‡ªå·±çš„ VPS ä¸Šè¿è¡Œç”Ÿäº§çº§ã€æ°¸ä¹…åœ¨çº¿çš„ Gateway ç½‘å…³
  - ä½ æƒ³å®Œå…¨æŽ§åˆ¶æŒä¹…åŒ–ã€äºŒè¿›åˆ¶æ–‡ä»¶å’Œé‡å¯è¡Œä¸º
  - ä½ åœ¨ Hetzner æˆ–ç±»ä¼¼æä¾›å•†ä¸Šç”¨ Docker è¿è¡Œ KrabKrab
summary: åœ¨å»‰ä»·çš„ Hetzner VPSï¼ˆDockerï¼‰ä¸Š 24/7 è¿è¡Œ KrabKrab Gateway ç½‘å…³ï¼Œå¸¦æŒä¹…çŠ¶æ€å’Œå†…ç½®äºŒè¿›åˆ¶æ–‡ä»¶
title: Hetzner
x-i18n:
  generated_at: "2026-02-03T07:52:17Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: 84d9f24f1a803aa15faa52a05f25fe557ec3e2c2f48a00c701d49764bd3bc21a
  source_path: platforms/hetzner.md
  workflow: 15
---

# åœ¨ Hetzner ä¸Šè¿è¡Œ KrabKrabï¼ˆDockerï¼Œç”Ÿäº§ VPS æŒ‡å—ï¼‰

## ç›®æ ‡

ä½¿ç”¨ Docker åœ¨ Hetzner VPS ä¸Šè¿è¡ŒæŒä¹…çš„ KrabKrab Gateway ç½‘å…³ï¼Œå¸¦æŒä¹…çŠ¶æ€ã€å†…ç½®äºŒè¿›åˆ¶æ–‡ä»¶å’Œå®‰å…¨çš„é‡å¯è¡Œä¸ºã€‚

å¦‚æžœä½ æƒ³è¦"çº¦ $5 å®žçŽ° KrabKrab 24/7"ï¼Œè¿™æ˜¯æœ€ç®€å•å¯é çš„è®¾ç½®ã€‚
Hetzner å®šä»·ä¼šå˜åŒ–ï¼›é€‰æ‹©æœ€å°çš„ Debian/Ubuntu VPSï¼Œå¦‚æžœé‡åˆ° OOM å†æ‰©å®¹ã€‚

## æˆ‘ä»¬åœ¨åšä»€ä¹ˆï¼ˆç®€å•è¯´æ˜Žï¼‰ï¼Ÿ

- ç§Ÿç”¨ä¸€å°å°åž‹ Linux æœåŠ¡å™¨ï¼ˆHetzner VPSï¼‰
- å®‰è£… Dockerï¼ˆéš”ç¦»çš„åº”ç”¨è¿è¡Œæ—¶ï¼‰
- åœ¨ Docker ä¸­å¯åŠ¨ KrabKrab Gateway ç½‘å…³
- åœ¨ä¸»æœºä¸ŠæŒä¹…åŒ– `~/.krabkrab` + `~/.krabkrab/workspace`ï¼ˆé‡å¯/é‡å»ºåŽä¿ç•™ï¼‰
- é€šè¿‡ SSH éš§é“ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è®¿é—®æŽ§åˆ¶ UI

Gateway ç½‘å…³å¯ä»¥é€šè¿‡ä»¥ä¸‹æ–¹å¼è®¿é—®ï¼š

- ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è¿›è¡Œ SSH ç«¯å£è½¬å‘
- å¦‚æžœä½ è‡ªå·±ç®¡ç†é˜²ç«å¢™å’Œä»¤ç‰Œï¼Œå¯ä»¥ç›´æŽ¥æš´éœ²ç«¯å£

æœ¬æŒ‡å—å‡è®¾åœ¨ Hetzner ä¸Šä½¿ç”¨ Ubuntu æˆ– Debianã€‚
å¦‚æžœä½ ä½¿ç”¨å…¶ä»– Linux VPSï¼Œè¯·ç›¸åº”åœ°æ˜ å°„è½¯ä»¶åŒ…ã€‚
é€šç”¨ Docker æµç¨‹è¯·å‚è§ [Docker](/install/docker)ã€‚

---

## å¿«é€Ÿè·¯å¾„ï¼ˆæœ‰ç»éªŒçš„è¿ç»´äººå‘˜ï¼‰

1. é…ç½® Hetzner VPS
2. å®‰è£… Docker
3. å…‹éš† KrabKrab ä»“åº“
4. åˆ›å»ºæŒä¹…åŒ–ä¸»æœºç›®å½•
5. é…ç½® `.env` å’Œ `docker-compose.yml`
6. å°†æ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶çƒ˜ç„™åˆ°é•œåƒä¸­
7. `docker compose up -d`
8. éªŒè¯æŒä¹…åŒ–å’Œ Gateway ç½‘å…³è®¿é—®

---

## ä½ éœ€è¦ä»€ä¹ˆ

- å…·æœ‰ root è®¿é—®æƒé™çš„ Hetzner VPS
- ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è¿›è¡Œ SSH è®¿é—®
- åŸºæœ¬ç†Ÿæ‚‰ SSH + å¤åˆ¶/ç²˜è´´
- çº¦ 20 åˆ†é’Ÿ
- Docker å’Œ Docker Compose
- æ¨¡åž‹è®¤è¯å‡­è¯
- å¯é€‰çš„æä¾›å•†å‡­è¯
  - WhatsApp äºŒç»´ç 
  - Telegram æœºå™¨äººä»¤ç‰Œ
  - Gmail OAuth

---

## 1) é…ç½® VPS

åœ¨ Hetzner ä¸­åˆ›å»ºä¸€ä¸ª Ubuntu æˆ– Debian VPSã€‚

ä»¥ root èº«ä»½è¿žæŽ¥ï¼š

```bash
ssh root@YOUR_VPS_IP
```

æœ¬æŒ‡å—å‡è®¾ VPS æ˜¯æœ‰çŠ¶æ€çš„ã€‚
ä¸è¦å°†å…¶è§†ä¸ºä¸€æ¬¡æ€§åŸºç¡€è®¾æ–½ã€‚

---

## 2) å®‰è£… Dockerï¼ˆåœ¨ VPS ä¸Šï¼‰

```bash
apt-get update
apt-get install -y git curl ca-certificates
curl -fsSL https://get.docker.com | sh
```

éªŒè¯ï¼š

```bash
docker --version
docker compose version
```

---

## 3) å…‹éš† KrabKrab ä»“åº“

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
```

æœ¬æŒ‡å—å‡è®¾ä½ å°†æž„å»ºè‡ªå®šä¹‰é•œåƒä»¥ä¿è¯äºŒè¿›åˆ¶æ–‡ä»¶æŒä¹…åŒ–ã€‚

---

## 4) åˆ›å»ºæŒä¹…åŒ–ä¸»æœºç›®å½•

Docker å®¹å™¨æ˜¯ä¸´æ—¶çš„ã€‚
æ‰€æœ‰é•¿æœŸçŠ¶æ€å¿…é¡»å­˜å‚¨åœ¨ä¸»æœºä¸Šã€‚

```bash
mkdir -p /root/.krabkrab
mkdir -p /root/.krabkrab/workspace

# å°†æ‰€æœ‰æƒè®¾ç½®ä¸ºå®¹å™¨ç”¨æˆ·ï¼ˆuid 1000ï¼‰ï¼š
chown -R 1000:1000 /root/.krabkrab
chown -R 1000:1000 /root/.krabkrab/workspace
```

---

## 5) é…ç½®çŽ¯å¢ƒå˜é‡

åœ¨ä»“åº“æ ¹ç›®å½•åˆ›å»º `.env`ã€‚

```bash
krabkrab_IMAGE=krabkrab:latest
krabkrab_GATEWAY_TOKEN=change-me-now
krabkrab_GATEWAY_BIND=lan
krabkrab_GATEWAY_PORT=18789

krabkrab_CONFIG_DIR=/root/.krabkrab
krabkrab_WORKSPACE_DIR=/root/.krabkrab/workspace

GOG_KEYRING_PASSWORD=change-me-now
XDG_CONFIG_HOME=/home/node/.krabkrab
```

ç”Ÿæˆå¼ºå¯†é’¥ï¼š

```bash
openssl rand -hex 32
```

**ä¸è¦æäº¤æ­¤æ–‡ä»¶ã€‚**

---

## 6) Docker Compose é…ç½®

åˆ›å»ºæˆ–æ›´æ–° `docker-compose.yml`ã€‚

```yaml
services:
  krabkrab-gateway:
    image: ${krabkrab_IMAGE}
    build: .
    restart: unless-stopped
    env_file:
      - .env
    environment:
      - HOME=/home/node
      - NODE_ENV=production
      - TERM=xterm-256color
      - krabkrab_GATEWAY_BIND=${krabkrab_GATEWAY_BIND}
      - krabkrab_GATEWAY_PORT=${krabkrab_GATEWAY_PORT}
      - krabkrab_GATEWAY_TOKEN=${krabkrab_GATEWAY_TOKEN}
      - GOG_KEYRING_PASSWORD=${GOG_KEYRING_PASSWORD}
      - XDG_CONFIG_HOME=${XDG_CONFIG_HOME}
      - PATH=/home/linuxbrew/.linuxbrew/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
    volumes:
      - ${krabkrab_CONFIG_DIR}:/home/node/.krabkrab
      - ${krabkrab_WORKSPACE_DIR}:/home/node/.krabkrab/workspace
    ports:
      # æŽ¨èï¼šåœ¨ VPS ä¸Šä¿æŒ Gateway ç½‘å…³ä»…é™ loopbackï¼›é€šè¿‡ SSH éš§é“è®¿é—®ã€‚
      # è¦å…¬å¼€æš´éœ²ï¼Œç§»é™¤ `127.0.0.1:` å‰ç¼€å¹¶ç›¸åº”é…ç½®é˜²ç«å¢™ã€‚
      - "127.0.0.1:${krabkrab_GATEWAY_PORT}:18789"

      # å¯é€‰ï¼šä»…å½“ä½ å¯¹æ­¤ VPS è¿è¡Œ iOS/Android èŠ‚ç‚¹å¹¶éœ€è¦ Canvas ä¸»æœºæ—¶ã€‚
      # å¦‚æžœä½ å…¬å¼€æš´éœ²æ­¤ç«¯å£ï¼Œè¯·é˜…è¯» /gateway/security å¹¶ç›¸åº”é…ç½®é˜²ç«å¢™ã€‚
      # - "18793:18793"
    command:
      [
        "node",
        "dist/index.js",
        "gateway",
        "--bind",
        "${krabkrab_GATEWAY_BIND}",
        "--port",
        "${krabkrab_GATEWAY_PORT}",
      ]
```

---

## 7) å°†æ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶çƒ˜ç„™åˆ°é•œåƒä¸­ï¼ˆå…³é”®ï¼‰

åœ¨è¿è¡Œä¸­çš„å®¹å™¨å†…å®‰è£…äºŒè¿›åˆ¶æ–‡ä»¶æ˜¯ä¸€ä¸ªé™·é˜±ã€‚
ä»»ä½•åœ¨è¿è¡Œæ—¶å®‰è£…çš„ä¸œè¥¿éƒ½ä¼šåœ¨é‡å¯æ—¶ä¸¢å¤±ã€‚

æ‰€æœ‰ skills æ‰€éœ€çš„å¤–éƒ¨äºŒè¿›åˆ¶æ–‡ä»¶å¿…é¡»åœ¨é•œåƒæž„å»ºæ—¶å®‰è£…ã€‚

ä»¥ä¸‹ç¤ºä¾‹ä»…å±•ç¤ºä¸‰ä¸ªå¸¸è§äºŒè¿›åˆ¶æ–‡ä»¶ï¼š

- `gog` ç”¨äºŽ Gmail è®¿é—®
- `goplaces` ç”¨äºŽ Google Places
- `wacli` ç”¨äºŽ WhatsApp

è¿™äº›æ˜¯ç¤ºä¾‹ï¼Œä¸æ˜¯å®Œæ•´åˆ—è¡¨ã€‚
ä½ å¯ä»¥ä½¿ç”¨ç›¸åŒçš„æ¨¡å¼å®‰è£…ä»»æ„æ•°é‡çš„äºŒè¿›åˆ¶æ–‡ä»¶ã€‚

å¦‚æžœä½ ä»¥åŽæ·»åŠ ä¾èµ–é¢å¤–äºŒè¿›åˆ¶æ–‡ä»¶çš„æ–° skillsï¼Œä½ å¿…é¡»ï¼š

1. æ›´æ–° Dockerfile
2. é‡æ–°æž„å»ºé•œåƒ
3. é‡å¯å®¹å™¨

**ç¤ºä¾‹ Dockerfile**

```dockerfile
FROM node:22-bookworm

RUN apt-get update && apt-get install -y socat && rm -rf /var/lib/apt/lists/*

# ç¤ºä¾‹äºŒè¿›åˆ¶æ–‡ä»¶ 1ï¼šGmail CLI
RUN curl -L https://github.com/steipete/gog/releases/latest/download/gog_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/gog

# ç¤ºä¾‹äºŒè¿›åˆ¶æ–‡ä»¶ 2ï¼šGoogle Places CLI
RUN curl -L https://github.com/steipete/goplaces/releases/latest/download/goplaces_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/goplaces

# ç¤ºä¾‹äºŒè¿›åˆ¶æ–‡ä»¶ 3ï¼šWhatsApp CLI
RUN curl -L https://github.com/steipete/wacli/releases/latest/download/wacli_Linux_x86_64.tar.gz \
  | tar -xz -C /usr/local/bin && chmod +x /usr/local/bin/wacli

# ä½¿ç”¨ç›¸åŒæ¨¡å¼åœ¨ä¸‹æ–¹æ·»åŠ æ›´å¤šäºŒè¿›åˆ¶æ–‡ä»¶

WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml .npmrc ./
COPY ui/package.json ./ui/package.json
COPY scripts ./scripts

RUN corepack enable
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build
RUN pnpm ui:install
RUN pnpm ui:build

ENV NODE_ENV=production

CMD ["node","dist/index.js"]
```

---

## 8) æž„å»ºå¹¶å¯åŠ¨

```bash
docker compose build
docker compose up -d krabkrab-gateway
```

éªŒè¯äºŒè¿›åˆ¶æ–‡ä»¶ï¼š

```bash
docker compose exec krabkrab-gateway which gog
docker compose exec krabkrab-gateway which goplaces
docker compose exec krabkrab-gateway which wacli
```

é¢„æœŸè¾“å‡ºï¼š

```
/usr/local/bin/gog
/usr/local/bin/goplaces
/usr/local/bin/wacli
```

---

## 9) éªŒè¯ Gateway ç½‘å…³

```bash
docker compose logs -f krabkrab-gateway
```

æˆåŠŸï¼š

```
[gateway] listening on ws://0.0.0.0:18789
```

ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘ï¼š

```bash
ssh -N -L 18789:127.0.0.1:18789 root@YOUR_VPS_IP
```

æ‰“å¼€ï¼š

`http://127.0.0.1:18789/`

ç²˜è´´ä½ çš„ Gateway ç½‘å…³ä»¤ç‰Œã€‚

---

## æŒä¹…åŒ–ä½ç½®ï¼ˆäº‹å®žæ¥æºï¼‰

KrabKrab åœ¨ Docker ä¸­è¿è¡Œï¼Œä½† Docker ä¸æ˜¯äº‹å®žæ¥æºã€‚
æ‰€æœ‰é•¿æœŸçŠ¶æ€å¿…é¡»åœ¨é‡å¯ã€é‡å»ºå’Œé‡å¯åŽä¿ç•™ã€‚

| ç»„ä»¶             | ä½ç½®                              | æŒä¹…åŒ–æœºåˆ¶    | è¯´æ˜Ž                        |
| ---------------- | --------------------------------- | ------------- | --------------------------- |
| Gateway ç½‘å…³é…ç½® | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | åŒ…æ‹¬ `krabkrab.json`ã€ä»¤ç‰Œ  |
| æ¨¡åž‹è®¤è¯é…ç½®æ–‡ä»¶ | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | OAuth ä»¤ç‰Œã€API å¯†é’¥        |
| Skill é…ç½®       | `/home/node/.krabkrab/skills/`    | ä¸»æœºå·æŒ‚è½½    | Skill çº§åˆ«çŠ¶æ€              |
| æ™ºèƒ½ä½“å·¥ä½œåŒº     | `/home/node/.krabkrab/workspace/` | ä¸»æœºå·æŒ‚è½½    | ä»£ç å’Œæ™ºèƒ½ä½“äº§ç‰©            |
| WhatsApp ä¼šè¯    | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | ä¿ç•™äºŒç»´ç ç™»å½•              |
| Gmail å¯†é’¥çŽ¯     | `/home/node/.krabkrab/`           | ä¸»æœºå· + å¯†ç  | éœ€è¦ `GOG_KEYRING_PASSWORD` |
| å¤–éƒ¨äºŒè¿›åˆ¶æ–‡ä»¶   | `/usr/local/bin/`                 | Docker é•œåƒ   | å¿…é¡»åœ¨æž„å»ºæ—¶çƒ˜ç„™            |
| Node è¿è¡Œæ—¶      | å®¹å™¨æ–‡ä»¶ç³»ç»Ÿ                      | Docker é•œåƒ   | æ¯æ¬¡é•œåƒæž„å»ºæ—¶é‡å»º          |
| æ“ä½œç³»ç»ŸåŒ…       | å®¹å™¨æ–‡ä»¶ç³»ç»Ÿ                      | Docker é•œåƒ   | ä¸è¦åœ¨è¿è¡Œæ—¶å®‰è£…            |
| Docker å®¹å™¨      | ä¸´æ—¶çš„                            | å¯é‡å¯        | å¯ä»¥å®‰å…¨é”€æ¯                |

