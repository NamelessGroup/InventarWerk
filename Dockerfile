# Build-Stage
FROM rust:latest as builder
WORKDIR /usr/src/app

# Diesel CLI installieren
RUN cargo install diesel_cli --no-default-features --features sqlite

# Rust-Abhängigkeiten cachen
COPY backend/Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -r src

# Anwendung bauen
COPY ./backend/ ./
RUN cargo build --release

# 2. Frontend Build Stage
FROM node:22 AS frontend-builder
WORKDIR /frontend

#Copy Frontend source code
COPY frontend/package.json /frontend/
RUN npm install

COPY frontend/ /frontend/
RUN npm run build


# Runtime-Stage
FROM debian:bookworm-slim
WORKDIR /usr/src/app

# Laufzeitabhängigkeiten installieren
RUN apt-get update && apt-get install -y libsqlite3-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Diesel CLI aus dem Build-Container kopieren
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Anwendung und Migrations kopieren
COPY --from=builder /usr/src/app/target/release/backend ./
COPY --from=builder /usr/src/app/migrations ./migrations

COPY ./backend/static ./static

#COPY --from=frontend-builder /frontend/dist ./static
# Startskript hinzufügen
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh
RUN ls -la
# Standard-Befehl
ENTRYPOINT ["./entrypoint.sh"]
EXPOSE 8000
