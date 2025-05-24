# Build-Stage
FROM rust:latest as builder
WORKDIR /usr/src/app

ARG FEATURES=""
ARG POSTGRES_URI=""

# Anwendung bauen
COPY ./backend/ ./

RUN cargo install sqlx-cli --no-default-features --features postgres

ENV DATABASE_URL=${POSTGRES_URI}

RUN cargo sqlx migrate run --source repositories/migrations


RUN cargo build ${FEATURES} --release

# 2. Frontend Build Stage
FROM node:23 AS frontend-builder
WORKDIR /frontend

#Copy Frontend source code
COPY frontend/package.json /frontend/
RUN npm install

COPY .git/ /frontend/.git/
COPY frontend/ /frontend/
RUN npm run build:prod


# Runtime-Stage
FROM debian:bookworm-slim
WORKDIR /usr/src/app

# Laufzeitabhängigkeiten installieren
RUN apt-get update && apt-get install -y libsqlite3-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Anwendung und Migrations kopieren
COPY --from=builder /usr/src/app/target/release/backend ./

#COPY ./backend/static ./static

COPY --from=frontend-builder /frontend/dist ./static
# Startskript hinzufügen
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh
RUN ls -la
# Standard-Befehl
ENTRYPOINT ["./entrypoint.sh"]
EXPOSE 8000
