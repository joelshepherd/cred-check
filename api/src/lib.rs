#[macro_use]
extern crate lazy_static;

mod error;
mod handler;
mod model;
mod route;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use warp::Filter;

type Db = PgPool;

lazy_static! {
    static ref SECRET: mwt::SecretKey = mwt::SecretKey::default();
    static ref ENCODER: mwt::Encoder<'static> = mwt::Encoder::new(&SECRET, 8600);
    static ref DECODER: mwt::Decoder<'static> = mwt::Decoder::new(&SECRET, 0);
}

/// Init api
pub async fn init() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_config = env::var("DATABASE_URL").expect("Could not find database config.");
    let db = PgPoolOptions::new()
        .connect(&db_config)
        .await
        .expect("Could not connect to database.");

    route::init(db)
}
