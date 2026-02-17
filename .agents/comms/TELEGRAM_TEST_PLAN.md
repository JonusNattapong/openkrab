# Telegram Media Download Test Plan

Purpose
-------
Prepare tests, fixtures and test-support helpers so the Telegram media download flow
can be hardened once `crates/openclaw-channels/telegram/src/lib.rs` is unlocked.
All artifacts in this file-tree are non-invasive (under `.agents/`) and do **not** edit
locked crates. The plan describes the approach, test cases, mocking strategy and
developer instructions to run the tests locally after the locked file is available.

High-level goals
----------------
- Validate handling of teloxide message shapes (PhotoSize, Document, Sticker) where
  file path fields may be `None` or `Some(String)` depending on teloxide version.
- Exercise streaming downloads using a shared `reqwest::Client` and assert max-size
  caps and graceful failures on oversize or partial reads.
- Test retry/backoff behavior on transient HTTP errors and non-200 responses.
- Ensure no real Telegram token is required: mock Bot::get_file and HTTP file server.

Recommended crates for tests (add to crate under test when unlocked)
------------------------------------------------------------------
- `httptest` or `wiremock` for mocking HTTP file responses (server + expectations)
- `serde_json` for constructing teloxide-like fixtures
- `tokio` (already in workspace) for async test runtime
- `bytes` for working with streamed buffers

Reqwest Client configuration (recommended)
---------------------------------------
- Single `reqwest::Client` created once per handler/worker (reuse across downloads)
- Timeouts: connect 10s, overall 60s (adjustable)
- Use `reqwest::Client::builder().pool_max_idle_per_host(0)` if desired for tests
- For real use: enable TLS with rustls (tokio-rustls) via default features

Behavioral expectations
-----------------------
- If teloxide `File.path` is `None` -> return a clear OpenClawError::Network/Storage
  mapping (no panic).
- If content-length > configured MAX_BYTES -> abort early with `Err` and no OOM.
- On transient 5xx or timeouts -> retry with exponential backoff (configurable attempts).
- Properly close stream on early abort and do not buffer the entire file in memory
  unless size is under the allowed threshold.

Test cases
----------
1) Photo download success
   - Mock Bot::get_file to return a `file_path` string.
   - Mock HTTP server returns 200 with small image bytes and `content-length` header.
   - Assert: download completes, returns bytes <= MAX_BYTES, no error.

2) Photo missing path (None)
   - Bot::get_file returns a file struct without `path`.
   - Assert: function returns a recoverable error describing missing file path.

3) Document oversize
   - Mock server sets `content-length` > MAX_BYTES.
   - Assert: download aborts early with oversize error and does not allocate large buffer.

4) Server transient failures -> retry then succeed
   - First two attempts return 502/timeout, final returns 200.
   - Assert: code retries according to policy and ultimately succeeds.

5) Partial read / truncated body
   - Server returns 200 but closes connection early (less bytes than content-length).
   - Assert: code detects truncated read and returns an error.

6) Sticker (webp) path variants
   - Test both Document-like and Photo-like sticker shapes where the file field differs.

Mocking strategy
----------------
- Mock Bot::get_file: create a small wrapper trait in the test that mimics the subset
  of the teloxide API used (e.g., `async fn get_file(file_id) -> Option<String>`).
- HTTP mock: use `httptest::Server` or `wiremock::MockServer` to respond to GET
  requests for `/file/bot<token>/<file_path>` with configurable status, headers, and body.
- Keep the token value in tests a fake static string (e.g. `TEST_TOKEN`) so URL
  construction remains deterministic.

Test scaffolding (non-invasive files)
-------------------------------------
- `.agents/test-utils/telegram_fixtures.json` — sample teloxide message shapes to use
  when building unit tests.
- `.agents/test-utils/http_helpers.md` — small notes/commands for running mock server

Developer steps to run tests once telegram crate is unlocked
----------------------------------------------------------
1. Add `httptest` (or `wiremock`) as a [dev-dependencies] entry in
   `crates/openclaw-channels/telegram/Cargo.toml`.
2. Add `#[cfg(test)] mod tests { ... }` inside `telegram/src/lib.rs` or
   create `crates/openclaw-channels/telegram/tests/download_tests.rs`.
3. Implement a thin `BotClient` trait to wrap `Bot::get_file` and in tests provide
   a mock implementation that returns fixtures from `.agents/test-utils/telegram_fixtures.json`.
4. Use `httptest::Server` to create endpoints and set expectations for status/headers/body.
5. Run `cargo test -p openclaw-channels-telegram --test download_tests`.

Next steps (what I will do now)
------------------------------
- Create these non-invasive artifacts (this file and fixtures) — done.
- When Agent-3 releases the telegram crate lock I will:
  1) Open PR with changes to `crates/openclaw-channels/telegram/src/lib.rs` to use a shared
     `reqwest::Client`, streaming with max-size, and the BotClient trait for testability.
  2) Add tests as described and iterate until `cargo test` passes.

If you want me to proceed differently, reply with one of:
1) "Proceed to take lock now" — I will claim and update BOARD.md + MESSAGES.md and start
   implementing code changes.
2) "Wait for Agent-3" — I will hold and only prepare more docs/fixtures.

