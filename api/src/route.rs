use crate::{handler, Db};
use warp::Filter;

pub fn get(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let with_db = warp::any().map(move || db.clone());

    let index = warp::path::end().map(|| warp::reply());

    let source_find = with_db
        .clone()
        .and(warp::path("source"))
        .and(warp::path::tail())
        .and_then(handler::source::find);

    let source_create = with_db
        .clone()
        .and(warp::path("source"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::source::create);

    index.or(source_find).or(source_create)
}
