mod error;
mod handler;
mod model;
mod route;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use warp::Filter;

type Db = PgPool;

/// Init api
pub async fn init() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = env::var("DATABASE_URL").expect("Could not find database config.");
    let db = PgPoolOptions::new()
        .connect(&config)
        .await
        .expect("Could not connect to database.");

    route::init(db)
}
