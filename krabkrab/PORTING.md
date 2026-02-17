Porting plan — openclaw (TypeScript) → krabkrab (Rust)

สรุปสั้น ๆ
- เป้าหมาย: full rewrite แบบ incremental — ย้ายโค้ดหลักจาก `openclaw/src` มาเป็น Rust crate ภายใต้ `krabkrab`
- วิธีทำ: แบ่งเป็น milestones (core types → runtime/CLI → connectors → providers → web/UI)

สถาปัตยกรรม (ข้อเสนอ)
- runtime: tokio (async)
- logging: tracing + tracing-subscriber
- errors: thiserror / anyhow
- config/serialization: serde + serde_json
- http: reqwest (client), axum/warp (server ถ้าต้องการ web API)
- testing: tokio::test, assert_json

โมดูลหลัก (mapping — priority สูง → ต่ำ)
- openclaw/src/utils, types → krabkrab::common (high)
- openclaw/src/logging* → krabkrab::logging (high)
- openclaw/src/commands, src/cli → krabkrab::commands + `krabkrab-cli` (binary) (high)
- openclaw/src/slack → krabkrab::connectors::slack (high)
- openclaw/src/telegram → krabkrab::connectors::telegram (high)
- openclaw/src/channels, providers → krabkrab::channels / krabkrab::providers (high)
- openclaw/src/media, media-understanding → krabkrab::media (medium)
- openclaw/src/web, ui → krabkrab-web (WASM or separate crate) (medium)
- openclaw/src/plugin-sdk → krabkrab::plugin (deferred)
- Swabble (Swift) → keep separate (not port for initial pass)

Incremental milestones
1. Core primitives: types, utils, logging, config
2. CLI + command runner + basic runtime loop
3. Connectors: telegram + slack (event handling + send)
4. Channels/providers + media handling
5. Web UI / WASM / plugin system
6. Tests + CI + release

Roadmap (auto phases - committed)

Phase 1 — Core parity (กำลังทำ)
- utils parity: normalize helpers, safe json wrappers, small string/path utilities
- version parity: fallback resolution behavior คล้าย `openclaw/src/version.ts`
- status/health parity: status summary + health command surface
- config parity (minimal): typed config + defaults + validation skeleton
- tests: unit tests สำหรับทุกโมดูลด้านบน

Phase 2 — Channel parity
- telegram parity: inbound normalization, outbound formatting, command surface
- slack parity: inbound normalization, outbound formatting, command surface
- behavior tests เทียบเคสหลักจากของเดิม

Phase 3 — Ops parity
- doctor/configure/onboard command flows (incremental)
- daemon/gateway command-path skeleton และ integration hooks
- regression tests สำหรับ CLI command routes

Phase 4 — Full feature hardening
- เพิ่ม integration tests ต่อ command chain
- เพิ่ม docs/release notes และ migration notes
- cleanup API naming ให้สอดคล้อง Rust idioms

Immediate execution order (เริ่มทันที)
1) ทำ Phase 1 ให้ compile-level complete
2) เติม parity tests
3) เดิน Phase 2 + 3 ต่อเนื่อง

ไฟล์ที่สร้างไว้แล้ว
- [`krabkrab/Cargo.toml`](krabkrab/Cargo.toml:1)
- [`krabkrab/src/lib.rs`](krabkrab/src/lib.rs:1)

หมายเหตุ
- ขั้นตอนนี้เป็นแผนงานเชิงวิศวกรรม — port ทีละโมดูล ไว้ทดสอบและรีแฟกเตอร์ระหว่างทาง
- บล็อกหลักตอนนี้: เครื่องมือ Rust (cargo) ยังไม่ติดตั้งใน environment ของคุณ (ผมบอกวิธีติดตั้งได้ถ้าต้องการ)
