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

---

## Future Releases

### Planned Features

#### Platform Support

- [ ] Native WhatsApp SDK parity extras
- [ ] Native LINE SDK parity extras
- [ ] Native macOS notifications (via `cocoa` crate)

#### Deployment & Distribution

- [ ] Docker container images
- [ ] Cross-compiled binaries for more platforms
  - [ ] ARM32 (Raspberry Pi)
  - [ ] FreeBSD
  - [ ] OpenBSD

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

- Docker container images
- Cross-compilation for ARM32
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

1. Docker containerization
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
