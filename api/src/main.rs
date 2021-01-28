mod handler;
mod model;
mod route;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;

type Db = Pool<Sqlite>;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("Could not find database config.");
    let db = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Could not connect to database.");

    let routes = route::get(db);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
