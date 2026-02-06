FROM rust:1.84-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace
COPY Cargo.toml ./
COPY crates/ crates/

# Build release binary
RUN cargo build --release -p tennis-scorer-api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tennis-scorer-api /usr/local/bin/tennis-scorer-api
COPY --from=builder /app/crates/tennis-scorer-api/migrations /app/migrations

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["tennis-scorer-api"]
