# fact-checker/api

## Setup

```sh
$ export DATABASE_URL="postgres:localhost:5432/fact_checker"`
$ sqlx db setup
```

## Test

```sh
$ export DATABASE_URL="postgres:localhost:5432/fact_checker_test"`
$ sqlx db reset
$ cargo test
```
