---
read_when:
  - ä½ æƒ³åœ¨ GCP ä¸Š 24/7 è¿è¡Œ KrabKrab
  - ä½ æƒ³è¦åœ¨è‡ªå·±çš„ VM ä¸Šè¿è¡Œç”Ÿäº§çº§ã€å¸¸é©»çš„ Gateway ç½‘å…³
  - ä½ æƒ³å®Œå…¨æŽ§åˆ¶æŒä¹…åŒ–ã€äºŒè¿›åˆ¶æ–‡ä»¶å’Œé‡å¯è¡Œä¸º
summary: åœ¨ GCP Compute Engine VMï¼ˆDockerï¼‰ä¸Š 24/7 è¿è¡Œ KrabKrab Gateway ç½‘å…³å¹¶æŒä¹…åŒ–çŠ¶æ€
title: GCP
x-i18n:
  generated_at: "2026-02-03T07:52:50Z"
  model: claude-opus-4-5
  provider: pi
  source_hash: abb236dd421505d307bb3869340c9a0c940de095b93af9ad1f130cc08a9deadb
  source_path: platforms/gcp.md
  workflow: 15
---

# åœ¨ GCP Compute Engine ä¸Šè¿è¡Œ KrabKrabï¼ˆDockerï¼Œç”Ÿäº§ VPS æŒ‡å—ï¼‰

## ç›®æ ‡

ä½¿ç”¨ Docker åœ¨ GCP Compute Engine VM ä¸Šè¿è¡ŒæŒä¹…åŒ–çš„ KrabKrab Gateway ç½‘å…³ï¼Œå…·æœ‰æŒä¹…çŠ¶æ€ã€å†…ç½®äºŒè¿›åˆ¶æ–‡ä»¶å’Œå®‰å…¨çš„é‡å¯è¡Œä¸ºã€‚

å¦‚æžœä½ æƒ³è¦"KrabKrab 24/7 å¤§çº¦ $5-12/æœˆ"ï¼Œè¿™æ˜¯åœ¨ Google Cloud ä¸Šçš„å¯é è®¾ç½®ã€‚
ä»·æ ¼å› æœºå™¨ç±»åž‹å’ŒåŒºåŸŸè€Œå¼‚ï¼›é€‰æ‹©é€‚åˆä½ å·¥ä½œè´Ÿè½½çš„æœ€å° VMï¼Œå¦‚æžœé‡åˆ° OOM åˆ™æ‰©å®¹ã€‚

## æˆ‘ä»¬åœ¨åšä»€ä¹ˆï¼ˆç®€å•è¯´æ˜Žï¼‰ï¼Ÿ

- åˆ›å»º GCP é¡¹ç›®å¹¶å¯ç”¨è®¡è´¹
- åˆ›å»º Compute Engine VM
- å®‰è£… Dockerï¼ˆéš”ç¦»çš„åº”ç”¨è¿è¡Œæ—¶ï¼‰
- åœ¨ Docker ä¸­å¯åŠ¨ KrabKrab Gateway ç½‘å…³
- åœ¨ä¸»æœºä¸ŠæŒä¹…åŒ– `~/.krabkrab` + `~/.krabkrab/workspace`ï¼ˆé‡å¯/é‡å»ºåŽä»ä¿ç•™ï¼‰
- é€šè¿‡ SSH éš§é“ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è®¿é—®æŽ§åˆ¶ UI

Gateway ç½‘å…³å¯ä»¥é€šè¿‡ä»¥ä¸‹æ–¹å¼è®¿é—®ï¼š

- ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è¿›è¡Œ SSH ç«¯å£è½¬å‘
- å¦‚æžœä½ è‡ªå·±ç®¡ç†é˜²ç«å¢™å’Œä»¤ç‰Œï¼Œå¯ä»¥ç›´æŽ¥æš´éœ²ç«¯å£

æœ¬æŒ‡å—ä½¿ç”¨ GCP Compute Engine ä¸Šçš„ Debianã€‚
Ubuntu ä¹Ÿå¯ä»¥ï¼›è¯·ç›¸åº”åœ°æ˜ å°„è½¯ä»¶åŒ…ã€‚
æœ‰å…³é€šç”¨ Docker æµç¨‹ï¼Œè¯·å‚é˜… [Docker](/install/docker)ã€‚

---

## å¿«é€Ÿè·¯å¾„ï¼ˆæœ‰ç»éªŒçš„è¿ç»´äººå‘˜ï¼‰

1. åˆ›å»º GCP é¡¹ç›® + å¯ç”¨ Compute Engine API
2. åˆ›å»º Compute Engine VMï¼ˆe2-smallï¼ŒDebian 12ï¼Œ20GBï¼‰
3. SSH è¿›å…¥ VM
4. å®‰è£… Docker
5. å…‹éš† KrabKrab ä»“åº“
6. åˆ›å»ºæŒä¹…åŒ–ä¸»æœºç›®å½•
7. é…ç½® `.env` å’Œ `docker-compose.yml`
8. å†…ç½®æ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶ã€æž„å»ºå¹¶å¯åŠ¨

---

## ä½ éœ€è¦ä»€ä¹ˆ

- GCP è´¦æˆ·ï¼ˆe2-micro ç¬¦åˆå…è´¹å±‚æ¡ä»¶ï¼‰
- å·²å®‰è£… gcloud CLIï¼ˆæˆ–ä½¿ç”¨ Cloud Consoleï¼‰
- ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘ SSH è®¿é—®
- å¯¹ SSH + å¤åˆ¶/ç²˜è´´æœ‰åŸºæœ¬äº†è§£
- çº¦ 20-30 åˆ†é’Ÿ
- Docker å’Œ Docker Compose
- æ¨¡åž‹è®¤è¯å‡­è¯
- å¯é€‰çš„æä¾›å•†å‡­è¯
  - WhatsApp QR
  - Telegram bot token
  - Gmail OAuth

---

## 1) å®‰è£… gcloud CLIï¼ˆæˆ–ä½¿ç”¨ Consoleï¼‰

**é€‰é¡¹ Aï¼šgcloud CLI**ï¼ˆæŽ¨èç”¨äºŽè‡ªåŠ¨åŒ–ï¼‰

ä»Ž https://cloud.google.com/sdk/docs/install å®‰è£…

åˆå§‹åŒ–å¹¶è®¤è¯ï¼š

```bash
gcloud init
gcloud auth login
```

**é€‰é¡¹ Bï¼šCloud Console**

æ‰€æœ‰æ­¥éª¤éƒ½å¯ä»¥é€šè¿‡ https://console.cloud.google.com çš„ Web UI å®Œæˆ

---

## 2) åˆ›å»º GCP é¡¹ç›®

**CLIï¼š**

```bash
gcloud projects create my-krabkrab-project --name="KrabKrab Gateway"
gcloud config set project my-krabkrab-project
```

åœ¨ https://console.cloud.google.com/billing å¯ç”¨è®¡è´¹ï¼ˆCompute Engine å¿…éœ€ï¼‰ã€‚

å¯ç”¨ Compute Engine APIï¼š

```bash
gcloud services enable compute.googleapis.com
```

**Consoleï¼š**

1. è½¬åˆ° IAM & Admin > Create Project
2. å‘½åå¹¶åˆ›å»º
3. ä¸ºé¡¹ç›®å¯ç”¨è®¡è´¹
4. å¯¼èˆªåˆ° APIs & Services > Enable APIs > æœç´¢ "Compute Engine API" > Enable

---

## 3) åˆ›å»º VM

**æœºå™¨ç±»åž‹ï¼š**

| ç±»åž‹     | é…ç½®                    | æˆæœ¬       | è¯´æ˜Ž           |
| -------- | ----------------------- | ---------- | -------------- |
| e2-small | 2 vCPUï¼Œ2GB RAM         | ~$12/æœˆ    | æŽ¨è           |
| e2-micro | 2 vCPUï¼ˆå…±äº«ï¼‰ï¼Œ1GB RAM | ç¬¦åˆå…è´¹å±‚ | è´Ÿè½½ä¸‹å¯èƒ½ OOM |

**CLIï¼š**

```bash
gcloud compute instances create krabkrab-gateway \
  --zone=us-central1-a \
  --machine-type=e2-small \
  --boot-disk-size=20GB \
  --image-family=debian-12 \
  --image-project=debian-cloud
```

**Consoleï¼š**

1. è½¬åˆ° Compute Engine > VM instances > Create instance
2. Nameï¼š`krabkrab-gateway`
3. Regionï¼š`us-central1`ï¼ŒZoneï¼š`us-central1-a`
4. Machine typeï¼š`e2-small`
5. Boot diskï¼šDebian 12ï¼Œ20GB
6. Create

---

## 4) SSH è¿›å…¥ VM

**CLIï¼š**

```bash
gcloud compute ssh krabkrab-gateway --zone=us-central1-a
```

**Consoleï¼š**

åœ¨ Compute Engine ä»ªè¡¨æ¿ä¸­ç‚¹å‡» VM æ—è¾¹çš„"SSH"æŒ‰é’®ã€‚

æ³¨æ„ï¼šVM åˆ›å»ºåŽ SSH å¯†é’¥ä¼ æ’­å¯èƒ½éœ€è¦ 1-2 åˆ†é’Ÿã€‚å¦‚æžœè¿žæŽ¥è¢«æ‹’ç»ï¼Œè¯·ç­‰å¾…å¹¶é‡è¯•ã€‚

---

## 5) å®‰è£… Dockerï¼ˆåœ¨ VM ä¸Šï¼‰

```bash
sudo apt-get update
sudo apt-get install -y git curl ca-certificates
curl -fsSL https://get.docker.com | sudo sh
sudo usermod -aG docker $USER
```

æ³¨é”€å¹¶é‡æ–°ç™»å½•ä»¥ä½¿ç»„æ›´æ”¹ç”Ÿæ•ˆï¼š

```bash
exit
```

ç„¶åŽé‡æ–° SSH ç™»å½•ï¼š

```bash
gcloud compute ssh krabkrab-gateway --zone=us-central1-a
```

éªŒè¯ï¼š

```bash
docker --version
docker compose version
```

---

## 6) å…‹éš† KrabKrab ä»“åº“

```bash
git clone https://github.com/krabkrab/krabkrab.git
cd krabkrab
```

æœ¬æŒ‡å—å‡è®¾ä½ å°†æž„å»ºè‡ªå®šä¹‰é•œåƒä»¥ä¿è¯äºŒè¿›åˆ¶æ–‡ä»¶æŒä¹…åŒ–ã€‚

---

## 7) åˆ›å»ºæŒä¹…åŒ–ä¸»æœºç›®å½•

Docker å®¹å™¨æ˜¯ä¸´æ—¶çš„ã€‚
æ‰€æœ‰é•¿æœŸçŠ¶æ€å¿…é¡»å­˜åœ¨äºŽä¸»æœºä¸Šã€‚

```bash
mkdir -p ~/.krabkrab
mkdir -p ~/.krabkrab/workspace
```

---

## 8) é…ç½®çŽ¯å¢ƒå˜é‡

åœ¨ä»“åº“æ ¹ç›®å½•åˆ›å»º `.env`ã€‚

```bash
krabkrab_IMAGE=krabkrab:latest
krabkrab_GATEWAY_TOKEN=change-me-now
krabkrab_GATEWAY_BIND=lan
krabkrab_GATEWAY_PORT=18789

krabkrab_CONFIG_DIR=/home/$USER/.krabkrab
krabkrab_WORKSPACE_DIR=/home/$USER/.krabkrab/workspace

GOG_KEYRING_PASSWORD=change-me-now
XDG_CONFIG_HOME=/home/node/.krabkrab
```

ç”Ÿæˆå¼ºå¯†é’¥ï¼š

```bash
openssl rand -hex 32
```

**ä¸è¦æäº¤æ­¤æ–‡ä»¶ã€‚**

---

## 9) Docker Compose é…ç½®

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
      # æŽ¨èï¼šåœ¨ VM ä¸Šä¿æŒ Gateway ç½‘å…³ä»…ç»‘å®š loopbackï¼›é€šè¿‡ SSH éš§é“è®¿é—®ã€‚
      # è¦å…¬å¼€æš´éœ²ï¼Œç§»é™¤ `127.0.0.1:` å‰ç¼€å¹¶ç›¸åº”é…ç½®é˜²ç«å¢™ã€‚
      - "127.0.0.1:${krabkrab_GATEWAY_PORT}:18789"

      # å¯é€‰ï¼šä»…å½“ä½ é’ˆå¯¹æ­¤ VM è¿è¡Œ iOS/Android èŠ‚ç‚¹å¹¶éœ€è¦ Canvas ä¸»æœºæ—¶ã€‚
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

## 10) å°†æ‰€éœ€äºŒè¿›åˆ¶æ–‡ä»¶å†…ç½®åˆ°é•œåƒä¸­ï¼ˆå…³é”®ï¼‰

åœ¨è¿è¡Œä¸­çš„å®¹å™¨å†…å®‰è£…äºŒè¿›åˆ¶æ–‡ä»¶æ˜¯ä¸€ä¸ªé™·é˜±ã€‚
ä»»ä½•åœ¨è¿è¡Œæ—¶å®‰è£…çš„å†…å®¹åœ¨é‡å¯åŽéƒ½ä¼šä¸¢å¤±ã€‚

æ‰€æœ‰ Skills æ‰€éœ€çš„å¤–éƒ¨äºŒè¿›åˆ¶æ–‡ä»¶å¿…é¡»åœ¨é•œåƒæž„å»ºæ—¶å®‰è£…ã€‚

ä»¥ä¸‹ç¤ºä¾‹ä»…æ˜¾ç¤ºä¸‰ä¸ªå¸¸è§çš„äºŒè¿›åˆ¶æ–‡ä»¶ï¼š

- `gog` ç”¨äºŽ Gmail è®¿é—®
- `goplaces` ç”¨äºŽ Google Places
- `wacli` ç”¨äºŽ WhatsApp

è¿™äº›æ˜¯ç¤ºä¾‹ï¼Œä¸æ˜¯å®Œæ•´åˆ—è¡¨ã€‚
ä½ å¯ä»¥ä½¿ç”¨ç›¸åŒçš„æ¨¡å¼å®‰è£…ä»»æ„æ•°é‡çš„äºŒè¿›åˆ¶æ–‡ä»¶ã€‚

å¦‚æžœä½ ä»¥åŽæ·»åŠ ä¾èµ–é¢å¤–äºŒè¿›åˆ¶æ–‡ä»¶çš„æ–° Skillsï¼Œä½ å¿…é¡»ï¼š

1. æ›´æ–° Dockerfile
2. é‡å»ºé•œåƒ
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

# ä½¿ç”¨ç›¸åŒçš„æ¨¡å¼åœ¨ä¸‹é¢æ·»åŠ æ›´å¤šäºŒè¿›åˆ¶æ–‡ä»¶

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

## 11) æž„å»ºå¹¶å¯åŠ¨

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

## 12) éªŒè¯ Gateway ç½‘å…³

```bash
docker compose logs -f krabkrab-gateway
```

æˆåŠŸï¼š

```
[gateway] listening on ws://0.0.0.0:18789
```

---

## 13) ä»Žä½ çš„ç¬”è®°æœ¬ç”µè„‘è®¿é—®

åˆ›å»º SSH éš§é“ä»¥è½¬å‘ Gateway ç½‘å…³ç«¯å£ï¼š

```bash
gcloud compute ssh krabkrab-gateway --zone=us-central1-a -- -L 18789:127.0.0.1:18789
```

åœ¨æµè§ˆå™¨ä¸­æ‰“å¼€ï¼š

`http://127.0.0.1:18789/`

ç²˜è´´ä½ çš„ Gateway ç½‘å…³ä»¤ç‰Œã€‚

---

## ä»€ä¹ˆæŒä¹…åŒ–åœ¨å“ªé‡Œï¼ˆçœŸå®žæ¥æºï¼‰

KrabKrab åœ¨ Docker ä¸­è¿è¡Œï¼Œä½† Docker ä¸æ˜¯çœŸå®žæ¥æºã€‚
æ‰€æœ‰é•¿æœŸçŠ¶æ€å¿…é¡»åœ¨é‡å¯ã€é‡å»ºå’Œé‡å¯åŽä»ç„¶å­˜åœ¨ã€‚

| ç»„ä»¶             | ä½ç½®                              | æŒä¹…åŒ–æœºåˆ¶    | è¯´æ˜Ž                        |
| ---------------- | --------------------------------- | ------------- | --------------------------- |
| Gateway ç½‘å…³é…ç½® | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | åŒ…æ‹¬ `krabkrab.json`ã€ä»¤ç‰Œ  |
| æ¨¡åž‹è®¤è¯é…ç½®æ–‡ä»¶ | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | OAuth ä»¤ç‰Œã€API å¯†é’¥        |
| Skill é…ç½®       | `/home/node/.krabkrab/skills/`    | ä¸»æœºå·æŒ‚è½½    | Skill çº§åˆ«çŠ¶æ€              |
| æ™ºèƒ½ä½“å·¥ä½œåŒº     | `/home/node/.krabkrab/workspace/` | ä¸»æœºå·æŒ‚è½½    | ä»£ç å’Œæ™ºèƒ½ä½“äº§ç‰©            |
| WhatsApp ä¼šè¯    | `/home/node/.krabkrab/`           | ä¸»æœºå·æŒ‚è½½    | ä¿ç•™ QR ç™»å½•                |
| Gmail å¯†é’¥çŽ¯     | `/home/node/.krabkrab/`           | ä¸»æœºå· + å¯†ç  | éœ€è¦ `GOG_KEYRING_PASSWORD` |
| å¤–éƒ¨äºŒè¿›åˆ¶æ–‡ä»¶   | `/usr/local/bin/`                 | Docker é•œåƒ   | å¿…é¡»åœ¨æž„å»ºæ—¶å†…ç½®            |
| Node è¿è¡Œæ—¶      | å®¹å™¨æ–‡ä»¶ç³»ç»Ÿ                      | Docker é•œåƒ   | æ¯æ¬¡é•œåƒæž„å»ºæ—¶é‡å»º          |
| OS åŒ…            | å®¹å™¨æ–‡ä»¶ç³»ç»Ÿ                      | Docker é•œåƒ   | ä¸è¦åœ¨è¿è¡Œæ—¶å®‰è£…            |
| Docker å®¹å™¨      | ä¸´æ—¶                              | å¯é‡å¯        | å¯ä»¥å®‰å…¨é”€æ¯                |

---

## æ›´æ–°

åœ¨ VM ä¸Šæ›´æ–° KrabKrabï¼š

```bash
cd ~/krabkrab
git pull
docker compose build
docker compose up -d
```

---

## æ•…éšœæŽ’é™¤

**SSH è¿žæŽ¥è¢«æ‹’ç»**

VM åˆ›å»ºåŽ SSH å¯†é’¥ä¼ æ’­å¯èƒ½éœ€è¦ 1-2 åˆ†é’Ÿã€‚ç­‰å¾…å¹¶é‡è¯•ã€‚

**OS Login é—®é¢˜**

æ£€æŸ¥ä½ çš„ OS Login é…ç½®æ–‡ä»¶ï¼š

```bash
gcloud compute os-login describe-profile
```

ç¡®ä¿ä½ çš„è´¦æˆ·å…·æœ‰æ‰€éœ€çš„ IAM æƒé™ï¼ˆCompute OS Login æˆ– Compute OS Admin Loginï¼‰ã€‚

**å†…å­˜ä¸è¶³ï¼ˆOOMï¼‰**

å¦‚æžœä½¿ç”¨ e2-micro å¹¶é‡åˆ° OOMï¼Œå‡çº§åˆ° e2-small æˆ– e2-mediumï¼š

```bash
# é¦–å…ˆåœæ­¢ VM
gcloud compute instances stop krabkrab-gateway --zone=us-central1-a

# æ›´æ”¹æœºå™¨ç±»åž‹
gcloud compute instances set-machine-type krabkrab-gateway \
  --zone=us-central1-a \
  --machine-type=e2-small

# å¯åŠ¨ VM
gcloud compute instances start krabkrab-gateway --zone=us-central1-a
```

---

## æœåŠ¡è´¦æˆ·ï¼ˆå®‰å…¨æœ€ä½³å®žè·µï¼‰

å¯¹äºŽä¸ªäººä½¿ç”¨ï¼Œä½ çš„é»˜è®¤ç”¨æˆ·è´¦æˆ·å°±å¯ä»¥ã€‚

å¯¹äºŽè‡ªåŠ¨åŒ–æˆ– CI/CD ç®¡é“ï¼Œåˆ›å»ºå…·æœ‰æœ€å°æƒé™çš„ä¸“ç”¨æœåŠ¡è´¦æˆ·ï¼š

1. åˆ›å»ºæœåŠ¡è´¦æˆ·ï¼š

   ```bash
   gcloud iam service-accounts create krabkrab-deploy \
     --display-name="KrabKrab Deployment"
   ```

2. æŽˆäºˆ Compute Instance Admin è§’è‰²ï¼ˆæˆ–æ›´çª„çš„è‡ªå®šä¹‰è§’è‰²ï¼‰ï¼š
   ```bash
   gcloud projects add-iam-policy-binding my-krabkrab-project \
     --member="serviceAccount:krabkrab-deploy@my-krabkrab-project.iam.gserviceaccount.com" \
     --role="roles/compute.instanceAdmin.v1"
   ```

é¿å…ä¸ºè‡ªåŠ¨åŒ–ä½¿ç”¨ Owner è§’è‰²ã€‚ä½¿ç”¨æœ€å°æƒé™åŽŸåˆ™ã€‚

å‚é˜… https://cloud.google.com/iam/docs/understanding-roles äº†è§£ IAM è§’è‰²è¯¦æƒ…ã€‚

---

## ä¸‹ä¸€æ­¥

- è®¾ç½®æ¶ˆæ¯æ¸ é“ï¼š[æ¸ é“](/channels)
- å°†æœ¬åœ°è®¾å¤‡é…å¯¹ä¸ºèŠ‚ç‚¹ï¼š[èŠ‚ç‚¹](/nodes)
- é…ç½® Gateway ç½‘å…³ï¼š[Gateway ç½‘å…³é…ç½®](/gateway/configuration)

