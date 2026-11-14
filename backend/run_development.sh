#!/bin/bash

# Beim Beenden (Strg+C, SIGTERM oder normales Ende) alle gestarteten APIs mitbeenden
cleanup() {
    trap - INT TERM EXIT
    kill $(jobs -p) 2>/dev/null
    wait
}
trap cleanup INT TERM EXIT

# APIs gleichzeitig starten
cargo run --bin inventarwerk_api --features=dev &
cargo run --bin zauberwerk_api --features=dev &
cargo run --bin last_changes_api --features=dev &

# Warten, bis alle beendet werden
wait
