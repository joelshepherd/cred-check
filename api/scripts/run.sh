#!/bin/bash -e
source .env
sqlx migrate run
cargo run $@
