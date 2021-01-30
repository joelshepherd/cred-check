mod context;
mod database;
mod filter;
mod handler;
mod model;

use context::Context;
use sqlx::PgPool;

type Db = PgPool;

#[tokio::main]
async fn main() {
    let db = database::connect().await;
    let routes = filter::filters(db);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
