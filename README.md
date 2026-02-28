# InventarWerk

<p align="center"> 
	<img alt="Logo" src="Logo.png" height="200px">
</p>

A tool to manage your inventory in a tabletop roleplaying game. It allows you to create different inventories, share them with your friends and manage the items in them. It also allows you to keep track of the money in the inventory and write notes for the dungeon master.

## Deployment

I recommend deploying InventarWerk behind a reverse proxy with Docker Compose:

```yaml
services:
  db_inv:
    image: postgres:15
    container_name: inventarwerk_postgres
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=inventarwerk
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  inventarwerk:
    image: ghcr.io/namelessgroup/inventarwerk:latest
    networks:
      - default
      - nginxbridge
    container_name: inventarwerk_nameless
    depends_on:
      - db_inv
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db_inv:5432/inventarwerk
      - DISCORD_CLIENT_ID=<Your-Client-ID>
      - DISCORD_CLIENT_SECRET=<Your-Client-Secret>
      - DISCORD_REDIRECT_URI=https://<Your-URL>/account/oauth/callback
      - ROCKET_ADDRESS=0.0.0.0
      - ROCKET_PORT=8000
    restart: unless-stopped

networks:
  nginxbridge:
    name: nginxbridge
    external: true
```

## Developing

### Backend 

For building the backend and the sqlx-cli you may need following packages (depending on the operating system):
- perl
- openssl (and the dev version, e.g., `libssl-dev`)
- pkg-config

First, install Rust: [https://rust-lang.org/learn/get-started/](https://rust-lang.org/learn/get-started/)

Then install sqlx-cli:
```bash
cargo install sqlx-cli
```

Fill the `.env` file located in the `backend` directory as following:
```env
DATABASE_URL=<Your-Postgres-URL>
DISCORD_CLIENT_ID=<Your-Client-ID>
DISCORD_CLIENT_SECRET=<Your-Client-Secret>
DISCORD_REDIRECT_URI=http://localhost:8000/account/oauth/callback
ROCKET_ADDRESS=127.0.0.1
ROCKET_PORT=8000
```

Now apply the migrations to the DB (run from `backend/repositories`):
```bash
sqlx migrate run
```

Finally, run the backend (from the `backend` folder):
```bash
cargo run --features=dev
```

#### Scripts
For resetting the DB (you may need to when switching branches with differing migrations) you can use the following script (use with caution):
`backend/reset_db.sh`

### Frontend

For running the frontend you need `node` and `npm`.

Switch to the `frontend` folder.

Install the requirements with:
```bash
npm install
```

And run the development server:
```bash
npm run dev
```

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
