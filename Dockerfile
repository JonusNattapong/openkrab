# â•”â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•—
# â•‘  openkrab â€” Multi-stage Docker Build                                â•‘
# â•‘  Produces a minimal (~80 MB) container with the compiled binary.    â•‘
# â•šâ•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

# â”€â”€ Stage 1: Builder â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
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
COPY bin/openkrab-cli/Cargo.toml bin/openkrab-cli/Cargo.toml

# Create stub source so cargo can resolve deps
RUN mkdir -p src && echo "pub fn stub() {}" > src/lib.rs && \
    mkdir -p bin/openkrab-cli/src && echo "fn main() {}" > bin/openkrab-cli/src/main.rs

# Download and compile dependencies only (cached layer)
RUN cargo build --workspace --${PROFILE} --features "${FEATURES}" 2>/dev/null || true

# Now copy real source code
COPY src/ src/
COPY bin/ bin/
COPY tests/ tests/
COPY examples/ examples/

# Touch source files to invalidate the stub build
RUN touch src/lib.rs bin/openkrab-cli/src/main.rs

# Build the real binary
RUN cargo build --workspace --${PROFILE} --features "${FEATURES}" && \
    cp target/${PROFILE}/openkrab-cli /build/openkrab-cli-bin 2>/dev/null || \
    cp target/release/openkrab-cli /build/openkrab-cli-bin

# â”€â”€ Stage 2: Runtime â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
FROM debian:bookworm-slim AS runtime

# Labels
LABEL org.opencontainers.image.title="openkrab"
LABEL org.opencontainers.image.description="Personal AI assistant â€” Rust edition"
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
COPY --from=builder /build/openkrab-cli-bin /usr/local/bin/openkrab

# Use non-root user
USER krab
WORKDIR /home/krab

# Environment
ENV OPENKRAB_DATA_DIR="/data"
ENV OPENKRAB_CONFIG_DIR="/data/config"
ENV OPENKRAB_LOG_DIR="/data/logs"
ENV RUST_LOG="info"
ENV RUST_BACKTRACE="1"

# Gateway default port
EXPOSE 4120

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:4120/health || exit 1

# Use tini for proper PID 1 signal handling
ENTRYPOINT ["tini", "--"]
CMD ["openkrab", "gateway", "start"]


