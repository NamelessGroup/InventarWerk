# InventarWerk

<p align="center"> 
	<img alt="Logo" src="Logo.png" height="200px">
</p>

A tool to manage your inventory in a tabletop roleplaying game. It allows you to create different inventories, share them with your friends and manage the items in them. It also allows you to keep track of the money in the inventory and write notes for the dungeon master.

## Deployment
I recommand deploying the Inventarwerk with docker simply build the Docker image i.e with the following command:
```bash
docker build -t inventarwerk .
```
and run it:
```bash
docker run -d -p "80:8000" --env-file .env inventarwerk
```
I suggest keeping the database persistant by setting the db path to `db/database.db` and mounting the `/app/src/db` directory (but this seems to not function in wsl, i suggest running it in pure linux):
```bash
docker run -d -p "80:8000" -v ./db:/app/src/db --env-file .env inventarwerk
```
I would also suggest running it behind a reverse proxy that use ssl. My docker compose setup looks like:
```yaml
services:
  inventarwerk:
    networks:
      ngninxbridge:
        ipv4_address: xxx.xxx.xxx.xxx
    volumes:
      - ./db:/usr/src/app/db
    container_name: inventarwerk
    env_file: .env
    image: inventarwerk
    restart: unless-stopped

networks:
  ngninxbridge:
    name: ngninxbridge
    external: true
```
Nginx runs in another container, that routes the traffic over the nginxbridge to the container.
## Dockerfile
The Dockerfile supports build args i.e. "--build-arg FEATURES="--features dev-deploy""


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
