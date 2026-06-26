# ---------- Build Stage ----------
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/lesort /usr/local/bin/lesort

ENTRYPOINT ["lesort"]
