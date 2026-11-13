# InventarWerk

<p align="center"> 
	<img alt="Logo" src="Logo.png" height="200px">
</p>

A tool to manage your inventory in a tabletop roleplaying game. It allows you to create different inventories, share them with your friends and manage the items in them. It also allows you to keep track of the money in the inventory and write notes for the dungeon master.

**ZauberWerk** is the spell companion: spell lists, spell presets, spell slots, concentration and spell attack modifiers.

## Architecture

InventarWerk consists of three Rust services (Rocket), a Vue frontend and a PostgreSQL database:

| Image | Source | Responsible for |
|---|---|---|
| `inventarwerk-api` | `backend/inventarwerk_api` | `/account`, `/inventory`, `/itemPreset`, `/backup`, combined Swagger UI |
| `zauberwerk-api` | `backend/zauberwerk_api` | `/spellList`, `/spellPreset`, `/spellSlot`, `/spellAttackModifier`, `/concentration` |
| `last-changes-api` | `backend/last_changes_api` | `/lastChanges/stream` (server-sent events from Postgres `NOTIFY`) |
| `web` | `frontend/` | nginx: serves the frontend and routes the API paths above to the services |

```
          your reverse proxy (TLS)
                    │
            ┌───────▼───────┐
            │      web      │  frontend + routing by path
            └─┬─────┬─────┬─┘
   inventarwerk-api │   last-changes-api
             zauberwerk-api
                    │
               PostgreSQL
```

The frontend sends all requests to its own origin, so the services must always be reachable through `web` (or an equivalent proxy) under one domain. This is also required for the session cookies, which every service decrypts with the shared `SECRET_KEY`.

Database migrations live in `backend/repositories/migrations` and are applied automatically by every service on startup.

## Deployment

The images are published to the GitHub Container Registry:

```
ghcr.io/namelessgroup/inventarwerk/inventarwerk-api
ghcr.io/namelessgroup/inventarwerk/zauberwerk-api
ghcr.io/namelessgroup/inventarwerk/last-changes-api
ghcr.io/namelessgroup/inventarwerk/web
```

Tags:

| Tag | Built from |
|---|---|
| `main` | latest commit on the main branch |
| `latest`, `v1.2.0` (git tag name) | releases (`latest` only for non-prereleases) |
| `pr-<number>` | pull requests against main |

### Environment

Create a `.env` file next to your `docker-compose.yml`:

```env
# PostgreSQL connection
DATABASE_URL=postgres://inventarwerk:<password>@postgres:5432/inventarwerk

# Key for encrypting session cookies. Must be identical for all services
# and at least 32 characters long. Generate one with e.g. `openssl rand -base64 32`.
SECRET_KEY=<random string>

# Discord OAuth (only used by inventarwerk-api)
DISCORD_CLIENT_ID=<client id>
DISCORD_CLIENT_SECRET=<client secret>
DISCORD_REDIRECT_URI=https://your.domain/account/oauth/callback
```

Rocket can be configured further with `ROCKET_*` variables. The images already set `ROCKET_ADDRESS=0.0.0.0` and `ROCKET_PORT=8000`.

### docker compose

```yaml
services:
  inventarwerk-api:
    image: ghcr.io/namelessgroup/inventarwerk/inventarwerk-api:latest
    env_file: .env
    restart: unless-stopped

  zauberwerk-api:
    image: ghcr.io/namelessgroup/inventarwerk/zauberwerk-api:latest
    env_file: .env
    restart: unless-stopped

  last-changes-api:
    image: ghcr.io/namelessgroup/inventarwerk/last-changes-api:latest
    env_file: .env
    restart: unless-stopped

  web:
    image: ghcr.io/namelessgroup/inventarwerk/web:latest
    depends_on:
      - inventarwerk-api
      - zauberwerk-api
      - last-changes-api
    restart: unless-stopped
    ports:
      - "8000:80"
```

The service names `inventarwerk-api`, `zauberwerk-api` and `last-changes-api` are used by the nginx configuration in `web`. Don't rename them.

If PostgreSQL runs in the same compose project, add it as a service and point `DATABASE_URL` to it.

### Backups

DMs can download a complete SQL dump of the database at `/backup/database` (e.g. through the Swagger UI). The `inventarwerk-api` image contains `pg_dump` for PostgreSQL 15, 16, 17 and 18 and uses the one matching your server, so upgrading PostgreSQL within these versions needs no change. For other versions, build the image with e.g. `--build-arg PG_VERSIONS="15 16 17 18 19"`.

If `pg_dump` fails before the download starts, the endpoint answers with an error. If it fails while streaming, the file ends with a line starting with `-- ERROR: THIS BACKUP IS INCOMPLETE`.

Restore a backup with:

```bash
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f inventarwerk-backup.sql
# or without a local psql, using the image of your server version:
docker run --rm -i postgres:15 psql "$DATABASE_URL" -v ON_ERROR_STOP=1 < inventarwerk-backup.sql
```

Use a current `psql` (any major version, but a minor release from August 2025 or later): the dumps contain the `\restrict` meta-command, which older `psql` releases don't know.

The dump drops and recreates all tables (`--clean --if-exists`), so restoring overwrites the current data.

### Behind a reverse proxy

We recommend running `web` behind a reverse proxy that terminates TLS. Instead of publishing a port, put `web` into the proxy's network, e.g.:

```yaml
  web:
    image: ghcr.io/namelessgroup/inventarwerk/web:latest
    networks:
      default:
      ngninxbridge:
        ipv4_address: xxx.xxx.xxx.xxx

networks:
  ngninxbridge:
    name: ngninxbridge
    external: true
```

Only `web` needs to be in that network; the APIs stay in the internal compose network.

The proxy must pass `X-Forwarded-Proto` and must not buffer server-sent events. For nginx:

```nginx
location / {
    proxy_pass http://xxx.xxx.xxx.xxx;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    # server-sent events (/lastChanges/stream, /spellPreset/cast/stream)
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    proxy_buffering off;
    proxy_read_timeout 1h;
}
```

## Building the images

The backend uses compile-time checked sqlx queries. The Docker build runs in offline mode and needs the query data in `backend/.sqlx`, which you generate against a database with all migrations applied:

```bash
cd backend
export DATABASE_URL=postgres://...      # or put it into backend/.env
sqlx migrate run --source repositories/migrations
cargo sqlx prepare --workspace
cd ..
```

Then build all images with [Bake](https://docs.docker.com/build/bake/):

```bash
docker buildx bake                          # all images
docker buildx bake backend                  # only the three API images
docker buildx bake web --load               # a single image, loaded into the local docker
VERSION=$(git rev-parse --short=6 HEAD) TAG=dev docker buildx bake --load
```

`VERSION` is the version shown in the frontend, `TAG` the image tag and `REGISTRY` the image prefix (see `docker-bake.hcl`).

A single image can also be built directly; the build context is always the repository root:

```bash
docker build -f backend/Dockerfile --target zauberwerk_api -t zauberwerk-api .
docker build -f frontend/Dockerfile -t inventarwerk-web .
```

### CI

`.github/workflows/docker-publish.yml` starts a PostgreSQL 15 service, applies all migrations, type-checks every sqlx query against it (`cargo sqlx prepare --workspace`) and then builds all four images with Bake. The images are pushed to ghcr.io (including pull requests) and get a build provenance attestation.

## Development

Requirements: Rust (stable), Node.js 24, a PostgreSQL database and [sqlx-cli](https://crates.io/crates/sqlx-cli) (`cargo install sqlx-cli --no-default-features --features rustls,postgres`).

### Backend

Create `backend/.env` with the variables from [Environment](#environment), with `DATABASE_URL` pointing to your development database. For local development set `DISCORD_REDIRECT_URI=http://localhost:8000/account/oauth/callback`.

The sqlx macros check every query against `DATABASE_URL` while compiling, so the database must be reachable and up to date. Apply new migrations with:

```bash
cd backend
sqlx migrate run --source repositories/migrations
```

Start all three services with CORS enabled:

```bash
cd backend
./run_development.sh
```

| Service | Port |
|---|---|
| inventarwerk_api | 8000 |
| zauberwerk_api | 8001 |
| last_changes_api | 8002 |

The `dev` feature binds to `127.0.0.1`, enables CORS for all origins and sets the ports above. Never enable it in production.

### Frontend

```bash
cd frontend
npm install
npm run dev
```

In development mode the frontend talks to the services on `localhost:8000`, `:8001` and `:8002` directly.

### API documentation

Every service serves a Swagger UI at `/swagger-ui/`. In production, `https://your.domain/swagger-ui/` is a combined UI with the API docs of all three services (select them in the top right). In development, each service only shows its own API docs, e.g. `http://localhost:8001/swagger-ui/` for ZauberWerk.

<details>
<summary><h2>Initial Requirements</h2></summary>
<ul>
<li>Different inventories
<ul><li>Sub-levels: private (shared only with you), shared (with read/write access), public</li></ul>
</li>
<li>Inventory manages items</li>
<li>Each inventory has money</li>
<li>Math in amount fields</li>
<li>DM notes</li>
<li>Account system (Discord)</li>
<li>Save item presets</li>
<li>Each item has: name, value, text</li>
</ul>
</details>
