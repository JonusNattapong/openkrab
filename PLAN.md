# OpenKrab Development Plan

This document outlines the planned features and roadmap for OpenKrab.

## Current Status

**Version:** 2026.2.20  
**Status:** Production Ready  
**Lines of Code:** ~56,276 Rust  
**Test Coverage:** 410+ tests

### Completed Features (100%)

- ✅ Multi-channel gateway (18 platforms)
- ✅ AI agent runtime with tools
- ✅ Memory system (vector + text search)
- ✅ Voice system (wake word, VAD, TTS)
- ✅ Plugin system (WASM runtime)
- ✅ Browser automation (CDP with pooling)
- ✅ Canvas/A2UI (Agent-to-UI protocol)
- ✅ Hooks system (Event-driven architecture)
- ✅ Security (pairing, sandboxing, audit)
- ✅ Web dashboard
- ✅ CLI commands (40+)

### Recently Completed (2026-02-20)

- ✅ Transcript compaction (`agents/compaction.rs`) — context window management
- ✅ Block/tool streaming (`agents/streaming.rs`) — paragraph-aware response streaming
- ✅ Binding system (`routing/bindings.rs`) — channel-to-agent mapping
- ✅ Complex session keys (`routing/session_key.rs`) — composite agent:id:rest format with DM scoping
- ✅ Role-based routing (`routing/role_routing.rs`) — allowlist/blocklist/priority role rules
- ✅ Session management tools (`agents/session_tools.rs`) — spawn/send/list/history
- ✅ Heartbeat runner (`gateway/heartbeat.rs`) — periodic health checks
- ✅ Gateway config hot reload (`gateway/config_reload.rs`) — file-watch with diff-based reload plans
- ✅ Transcript update events (`sessions/transcript_events.rs`) — pub/sub event bus

---

## Future Releases

### Planned Features

#### Platform Support

- [ ] Native WhatsApp SDK parity extras
- [ ] Native LINE SDK parity extras
- [ ] Native macOS notifications (via `cocoa` crate)

#### Deployment & Distribution

- [x] Docker container images (multi-arch: amd64 + arm64)
- [x] Cross-compiled binaries for more platforms
  - [x] ARM32 (Raspberry Pi — armv7)
  - [x] FreeBSD (x86_64)
  - [x] OpenBSD (x86_64 compat via static musl)
  - [x] Linux musl (fully static binary)

#### Voice & Communication

- [ ] WebRTC support for voice calls
- [ ] SIP integration
- [ ] Video call support

#### AI & ML Enhancements

- [ ] Local LLM inference (llama.cpp integration)
- [ ] Multi-modal support (vision, audio)
- [ ] Agent chaining and workflows
- [ ] Custom model fine-tuning pipeline

#### Enterprise Features

- [ ] LDAP/Active Directory integration
- [ ] SAML SSO support
- [ ] Audit log export (SIEM integration)
- [ ] Compliance reporting (GDPR, SOC2)

#### Developer Experience

- [ ] Plugin SDK documentation
- [ ] Plugin marketplace
- [ ] GraphQL API
- [ ] Webhook management UI

---

## Version Roadmap

### v2026.3.x (Q1 2026)

- ✅ Docker container images (Dockerfile + docker-compose)
- ✅ Cross-compilation for ARM32, FreeBSD, OpenBSD
- Native macOS notifications

### v2026.4.x (Q2 2026)

- WebRTC voice calls
- Local LLM inference
- Plugin marketplace beta

### v2026.5.x (Q3 2026)

- Multi-modal AI support
- Enterprise SSO (SAML/LDAP)
- GraphQL API

### v2026.6.x (Q4 2026)

- Video calls
- Agent workflows
- Compliance reporting

---

## Long-term Vision

### Goals

- [ ] Mobile apps (iOS/Android) using React Native or native
- [ ] Desktop apps (Electron or Tauri)
- [ ] Cloud-hosted option (managed service)
- [ ] Enterprise certification (SOC2, ISO 27001)
- [ ] AI agent marketplace

### Research Areas

- [ ] Federated learning for privacy-preserving AI
- [ ] Homomorphic encryption for secure computation
- [ ] Edge AI deployment on low-power devices
- [ ] Quantum-resistant cryptography

---

## Contributing

Want to help implement these features? See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Priority areas for contributors:

1. ~~Docker containerization~~ ✅
2. WebRTC implementation
3. Additional channel connectors
4. Plugin SDK improvements
5. Documentation and tutorials

---

## Feature Requests

To request a feature:

1. Check if it's already on this list
2. Open an issue on GitHub with the `enhancement` label
3. Discuss in GitHub Discussions

---

Last updated: 2026-02-20
