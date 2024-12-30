# Build-Stage
FROM rust:latest as builder
WORKDIR /usr/src/app

# Diesel CLI installieren
RUN cargo install diesel_cli --no-default-features --features sqlite

# Rust-Abhängigkeiten cachen
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -r src

# Anwendung bauen
COPY ./backend/ ./
RUN cargo build --release

# Runtime-Stage
FROM debian:bookworm-slim
WORKDIR /usr/src/app

# Laufzeitabhängigkeiten installieren
RUN apt-get update && apt-get install -y libsqlite3-dev && rm -rf /var/lib/apt/lists/*

# Diesel CLI aus dem Build-Container kopieren
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Anwendung und Migrations kopieren
COPY --from=builder /usr/src/app/target/release/backend ./
COPY --from=builder /usr/src/app/migrations ./migrations

# TODO: Add frontend here
COPY /backend/static ./static

# Startskript hinzufügen
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh
RUN ls -la
# Standard-Befehl
ENTRYPOINT ["./entrypoint.sh"]
EXPOSE 8000
