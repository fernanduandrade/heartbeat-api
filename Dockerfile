FROM rust:1.80-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd --system --uid 10001 --create-home appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/heartbeat-api /usr/local/bin/heartbeat-api

USER appuser
EXPOSE 8080

ENV PORT=8080

CMD ["heartbeat-api"]
