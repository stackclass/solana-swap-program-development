# syntax=docker/dockerfile:1.7-labs
FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

# .git & README.md are unique per-repository. We ignore them on first copy to prevent cache misses
COPY --exclude=.git --exclude=README.md . /app

# This runs cargo build
RUN .stackclass/compile.sh

# Final minimal image
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder --exclude=target /app ./
COPY --from=builder /app/target/release/stackclass-solana-voting-program ./target/release/

# Set default startup command
CMD ["./.stackclass/run.sh"]
