# Multi-stage build for Rust backend
# Stage 1: Builder
# Using Rust 1.90.0 which supports dependencies that require edition2024
FROM rust:1.90.0-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Copy SQLx query cache for offline mode
COPY .sqlx ./.sqlx

# Build with SQLx offline mode and static linking
ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM scratch

# Copy CA certificates for HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/windspire_backend /windspire_backend

# Expose port
EXPOSE 8080

# Run the binary
ENTRYPOINT ["/windspire_backend"]
