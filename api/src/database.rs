use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn connect() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("Could not find database config.");
    let db = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Could not connect to database.");

    db
}
