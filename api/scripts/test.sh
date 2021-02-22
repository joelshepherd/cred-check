#!/bin/bash -e
source tests/.env
sqlx db reset
sqlx migrate --source tests/migrations run
cargo test $@
