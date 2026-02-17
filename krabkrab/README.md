# KrabKrab

KrabKrab is a Rust workspace scaffold for rebuilding the original project in Rust,
with the new brand name `KrabKrab`.

## Workspace Crates

- `cli` - command line binary (`krabkrab`)
- `common` - shared types/logging/helpers
- `agents` - agent runtime stubs
- `gateway` - gateway runtime stubs
- `channels` - channel adapter stubs
- `providers` - provider abstraction stubs
- `media` - media subsystem stubs
- `sessions` - session subsystem stubs
- `memory` - memory subsystem stubs
- `plugins` - plugin subsystem stubs
- `web` - web API/control UI stubs
- `infra` - infrastructure helpers
- `wizard` - onboarding wizard scaffold

## Quick Start

```powershell
cd D:\Projects\Github\openkrab\krabkrab
cargo run -p krabkrab_cli -- start
cargo run -p krabkrab_cli -- send user123 "hello"
cargo run -p krabkrab_cli -- onboard
```

Note: this is a scaffold. Runtime features are placeholders and will be implemented incrementally.
