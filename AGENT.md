# AGENT Guide for `krabkrab`

## Mission
Port `openclaw` TypeScript runtime into Rust incrementally while keeping behavior parity, test coverage, and clear phase tracking in `PORTING.md`.

## Source of truth
- Porting status and roadmap: `PORTING.md`
- Rust modules: `src/`
- Tests: `tests/`
- Upstream reference: `../openclaw/`

## Working rules
- Keep changes phase-scoped. Do not mix unrelated refactors in the same commit.
- For each ported feature, include:
  - Source mapping comment in file header.
  - Rust unit/integration tests for happy path + error path.
  - Config and runtime wiring (avoid dead modules).
- Preserve existing user-facing behavior unless the phase explicitly changes it.
- Prefer deterministic logic and explicit error messages over silent fallbacks.

## Porting checklist (per module)
1. Identify exact upstream source files in `../openclaw`.
2. Port data models and parsing first.
3. Port runtime logic and integrate into registry/router/commands.
4. Add tests for parsing, policy decisions, and failure handling.
5. Update `PORTING.md` status table and phase notes.

## Priority order
1. Complete current phase from `PORTING.md`.
2. Close stubs/TODOs in already-ported modules.
3. Add next connector/provider by roadmap priority.
4. Finalize hardening and release-readiness phase.

## Local validation
Run when toolchain is available:

```bash
cd krabkrab
cargo fmt
cargo test
```

If full tests are expensive, run targeted tests for touched modules first, then full suite before closing the phase.

## Out of scope by default
- Native app stacks under `openclaw/apps/` (iOS/macOS/Android)
- Browser extension assets
- Infra deployment files (Docker/Fly/Render)

Only include out-of-scope items if project scope is explicitly changed.
