# Backend Dockerfile for Rust (Actix + SQLx)

FROM rust:1.77 AS builder

# Create app directory
WORKDIR /app

# Install dependencies first (for caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy source and rebuild
COPY . .
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

ENV RUST_LOG=info
CMD ["backend"]

