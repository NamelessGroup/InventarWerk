#!/bin/bash

# APIs gleichzeitig starten
cargo run --bin inventarwerk_api --features=dev&
cargo run --bin zauberwerk_api --features=dev&

# Warten, bis beide beendet werden
wait
