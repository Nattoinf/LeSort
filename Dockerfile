# ---------- Build Stage ----------
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/lesort /usr/local/bin/lesort

ENTRYPOINT ["lesort"]
