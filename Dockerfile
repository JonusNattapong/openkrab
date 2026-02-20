# ╔══════════════════════════════════════════════════════════════════════╗
# ║  krabkrab — Multi-stage Docker Build                                ║
# ║  Produces a minimal (~80 MB) container with the compiled binary.    ║
# ╚══════════════════════════════════════════════════════════════════════╝

# ── Stage 1: Builder ─────────────────────────────────────────────────
FROM rust:1.83-bookworm AS builder

# Build arguments
ARG FEATURES="default"
ARG PROFILE="release"

WORKDIR /build

# Install system dependencies needed for compilation
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependency compilation:
# Copy manifests first so deps are cached unless Cargo.toml changes
COPY Cargo.toml Cargo.lock ./
COPY bin/krabkrab-cli/Cargo.toml bin/krabkrab-cli/Cargo.toml

# Create stub source so cargo can resolve deps
RUN mkdir -p src && echo "pub fn stub() {}" > src/lib.rs && \
    mkdir -p bin/krabkrab-cli/src && echo "fn main() {}" > bin/krabkrab-cli/src/main.rs

# Download and compile dependencies only (cached layer)
RUN cargo build --workspace --${PROFILE} --features "${FEATURES}" 2>/dev/null || true

# Now copy real source code
COPY src/ src/
COPY bin/ bin/
COPY tests/ tests/
COPY examples/ examples/

# Touch source files to invalidate the stub build
RUN touch src/lib.rs bin/krabkrab-cli/src/main.rs

# Build the real binary
RUN cargo build --workspace --${PROFILE} --features "${FEATURES}" && \
    cp target/${PROFILE}/krabkrab-cli /build/krabkrab-cli-bin 2>/dev/null || \
    cp target/release/krabkrab-cli /build/krabkrab-cli-bin

# ── Stage 2: Runtime ─────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Labels
LABEL org.opencontainers.image.title="krabkrab"
LABEL org.opencontainers.image.description="Personal AI assistant — Rust edition"
LABEL org.opencontainers.image.source="https://github.com/openkrab/openkrab"
LABEL org.opencontainers.image.licenses="MIT"

# Install minimal runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    tini \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid 1000 krab && \
    useradd --uid 1000 --gid krab --shell /bin/bash --create-home krab

# Create data directories
RUN mkdir -p /data/config /data/memory /data/logs /data/plugins && \
    chown -R krab:krab /data

# Copy binary from builder
COPY --from=builder /build/krabkrab-cli-bin /usr/local/bin/krabkrab

# Use non-root user
USER krab
WORKDIR /home/krab

# Environment
ENV KRABKRAB_DATA_DIR="/data"
ENV KRABKRAB_CONFIG_DIR="/data/config"
ENV KRABKRAB_LOG_DIR="/data/logs"
ENV RUST_LOG="info"
ENV RUST_BACKTRACE="1"

# Gateway default port
EXPOSE 4120

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:4120/health || exit 1

# Use tini for proper PID 1 signal handling
ENTRYPOINT ["tini", "--"]
CMD ["krabkrab", "gateway", "start"]
