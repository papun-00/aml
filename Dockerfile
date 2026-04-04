# ─────────────────────────────────────────────────────────────
# Alashore Marine Exports — Production Dockerfile
# Multi-stage: Rust backend + Dioxus WASM frontend
# ─────────────────────────────────────────────────────────────

# Stage 1: Build Rust backend
FROM rust:1.79-slim AS backend-builder
WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev sqlite3 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
COPY shared/     ./shared/
COPY backend/    ./backend/
COPY frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir -p frontend/src && echo "fn main(){}" > frontend/src/main.rs

RUN cargo build --release --package backend

# Stage 2: Build Dioxus frontend (WASM)
FROM rust:1.79 AS frontend-builder
WORKDIR /build

RUN rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli --version 0.6.0 --locked && \
    cargo install wasm-bindgen-cli --version 0.2.92 --locked

COPY . .
WORKDIR /build/frontend
RUN dx build --release

# Stage 3: Final runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates sqlite3 && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -r -s /bin/false alashore

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /build/target/release/alashore-api ./

# Copy frontend dist (served by Axum as static files)
COPY --from=frontend-builder /build/frontend/dist ./frontend/dist/

# Copy migrations
COPY backend/migrations/ ./migrations/

# Create data directory for SQLite
RUN mkdir -p /app/data && chown alashore:alashore /app/data

USER alashore

EXPOSE 3000

ENV HOST=0.0.0.0
ENV PORT=3000
ENV DATABASE_URL=sqlite:///app/data/alashore.db

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s \
  CMD curl -f http://localhost:3000/api/v1/health || exit 1

CMD ["./alashore-api"]
