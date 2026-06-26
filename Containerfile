# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.88 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM debian:bookworm-slim

LABEL org.opencontainers.image.title="peekls" \
      org.opencontainers.image.description="A CLI tool that reimagines ls with smarter file listing and lightweight previews" \
      org.opencontainers.image.source="https://github.com/kawai-225/peekls" \
      org.opencontainers.image.licenses="MIT"

COPY --from=builder /app/target/release/peekls /app/peekls

WORKDIR /work

ENTRYPOINT ["/app/peekls"]