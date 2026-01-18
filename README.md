# InventarWerk

<p align="center"> 
	<img alt="Logo" src="Logo.png" height="200px">
</p>

A tool to manage your inventory in a tabletop roleplaying game. It allows you to create different inventories, share them with your friends and manage the items in them. It also allows you to keep track of the money in the inventory and write notes for the dungeon master.

## Deployment
I recommand deploying the Inventarwerk behind a reverse proxy with docker compose:

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
      - internalnginx
    container_name: inventarwerk_nameless
    depends_on:
      - db_inv_nameless
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db_inv:5432/inventarwerk
      - DISCORD_CLIENT_ID=<Your-Client-ID>
      - DISCORD_CLIENT_SECRET=<Your-Client-Secret>
      - DISCORD_REDIRECT_URI=https://<Your-URL>/account/oauth/callback

      - ROCKET_ADDRESS=0.0.0.0
      - ROCKET_PORT=8000
    restart: unless-stopped


networks:
  ngninxbridge:
    name: ngninxbridge
    external: true
```

## Developing
### Backend 
For building the backend and the sqlx-cli you may need following packages (depending on the operating system):
- pearl
- openssl (and the dev version)
- pkg-config

First install rust (https://rust-lang.org/learn/get-started/)
Than install sqlx-cli (`cargo install sqlx-cli`)
Fill the .env file located in the backend as following:
```
DATABASE_URL=<Your-Postgres-URL>
DISCORD_CLIENT_ID=<Your-Client-ID>
DISCORD_CLIENT_SECRET=<Your-Client-Secret>
DISCORD_REDIRECT_URI=http://localhost:8000/account/oauth/callback
ROCKET_ADDRESS=127.0.0.1
ROCKET_PORT=8000
```

Now apply the migrations to the db (run from `backend/repositories`): `sqlx migrate run`

Finally run the backend (from the `backend` folder): `cargo run --features=dev`

#### Scripts
For reseting the DB (you may need to when switching branches with differing migrations) you can use the following script (use with caution):
`backend/reset_db.sh`

### Frontend

For running the frontend you need `node` and `npm`.

Now switch to the frontend folder.

Install the requirements with: `npm i`

And run the development server: `npm run dev`

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
