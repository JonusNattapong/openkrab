# 🎯 Agent Task Board

> **กระดานงานกลาง** — ทุก Agent ต้องอ่านก่อนเริ่มงาน และอัปเดตเมื่อเปลี่ยนสถานะ
>
> **Last Updated**: 2026-02-17T13:00:00+07:00
> **Phase**: 3 — Storage & Channels
> **Overall Progress**: ✅ Done (54%): core, errors, config, storage, gateway, cli, telegram, discord, agents, tools, auth, storage-integration
> **In Progress** (8%): discord tests, ci-cd, telegram-mocks
> **Not Started** (38%): whatsapp-bridge, slack, google-chat, signal, voice-wake, talk-mode, browser, canvas, media, model-failover, oauth, cron, webhooks, docker, skills, mobile-ffi

---

## 📋 Rules (กฎการใช้งาน)

1. **อ่านก่อนทำ** — ก่อนเริ่มงานใหม่ให้อ่าน Board นี้ก่อนเสมอ
2. **ล็อคไฟล์** — ถ้าจะแก้ไฟล์ ต้องประกาศใน File Locks ก่อน
3. **อัปเดตสถานะ** — เมื่อเริ่ม/เสร็จ/ติดปัญหา ให้อัปเดตทันที
4. **ห้ามแก้ไฟล์ที่ถูกล็อค** — ถ้าไฟล์ถูก Agent อื่นล็อคอยู่ ห้ามแตะ
5. **ใช้ MESSAGES.md** — ถ้าต้องการคุยกับ Agent อื่น

---

## 👥 Agent Registry

| Agent | Role | Status | Current Task |
|-------|------|--------|--------------|
| Agent-1 | Lead Developer — Gateway, Core, CLI, Auth | 🔵 Working | Gateway-Storage Integration + Auth Middleware |
| Agent-2 | Backend Engineer — Telegram, Storage, Tooling | 🔵 Working | Telegram Mocks & Integration Tests |
| Agent-3 | Channel Specialist — Discord, Slack, WhatsApp, Signal | 🔵 Working | Protocol pre-check — preparing to follow PROTOCOL.md |
| Agent-4 | Async & Storage Specialist — Storage, SQLite/PG, CI/CD | 🔵 Working | Rust migration verification + compile triage |

**Status Legend:**

- 🟢 Idle — ว่าง พร้อมรับงาน
- 🔵 Working — กำลังทำงาน
- 🟡 Blocked — ติดปัญหา รอความช่วยเหลือ
- 🔴 Error — เกิดข้อผิดพลาด ต้องแก้ไข
- ⚪ Offline — ไม่ได้ทำงาน

---

## 🔒 File Locks

> ประกาศไฟล์ที่กำลังแก้ไข เพื่อป้องกัน conflict

| File Path | Locked By | Since | Purpose |
|-----------|-----------|-------|---------|
| crates/openclaw-storage/src/backends/sqlite.rs | Agent-3 (released 2026-02-17T13:11:00+07:00) | 2026-02-17T12:40:00 | Storage repair & compile fixes (completed) |
| .agents/comms/AGENT_3.md | Agent-3 | 2026-02-17T12:25:00 | Protocol pre-check & status update |
| .agents/comms/MESSAGES.md | Agent-3 | 2026-02-17T12:25:00 | Posting protocol pre-check message |
| docs/channels/discord.md | Agent-3 | 2026-02-17T12:26:00 | Writing user docs & examples |
| crates/openclaw-channels/discord/src/lib.rs | Agent-3 | 2026-02-17T12:30:00 | Running integration & unit tests |
| crates/openclaw-channels/discord/src/mocks.rs | Agent-3 | 2026-02-17T12:30:00 | Updating mocks for tests |
| crates/openclaw-gateway/src/auth.rs | Agent-3 | 2026-02-17T12:32:00 | Reviewing & testing auth middleware |
| crates/openclaw-gateway/src/lib.rs | Agent-3 | 2026-02-17T12:32:00 | Gateway build & tests (auth) |
| crates/openclaw-channels/telegram/src/lib.rs | Agent-3 | 2026-02-17T12:33:00 | Fixing closure FnOnce -> Fn issue (compile)

### วิธีใช้ File Locks

```markdown
<!-- เมื่อเริ่มแก้ไฟล์ — เพิ่มบรรทัด -->
| src/example.ts | Agent-1 | 2026-02-17T10:40:00 | Refactor function X |

| Issue | Agents Involved | Status | Resolution |
|-------|----------------|--------|------------|
| Discord real token unavailable | Agent-3 | 🟡 Workaround | Using mock mode for development |
| WhatsApp — no mature Rust lib | Team | 🔴 Blocked | Use Baileys bridge (TypeScript) for v1 |

---

## 📝 Decisions Log (บันทึกการตัดสินใจ)

> การตัดสินใจสำคัญที่ทุก agent ต้องรู้

| Date | Decision | Made By | Affects |
|------|----------|---------|---------|
| 2026-02-17 | Auth: API key first, JWT later for web UI | Agent-1 | Agent-1, Agent-4 |
| 2026-02-17 | Storage: SQLite default, PostgreSQL for production | Agent-4 | All |
| 2026-02-17 | WhatsApp: Bridge approach with Baileys first | Team | Agent-3 |
| 2026-02-17 | Agent framework: Custom impl (not Pi), use rig or similar | Team | Agent-1, Agent-2 |
| 2026-02-17 | Config format: TOML (replacing JSON) | Agent-1 | All |
| 2026-02-17 | Discord: Use mock if token unavailable | Agent-4 | Agent-3 |
| 2026-02-17 | CLI bootstrap: Rust-only runtime; Node kept as reference file | Agent-4 | All |
