use crate::{handler, Db};
use warp::{any, path, reply, Filter};

pub fn get(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = path::end().map(|| reply());

    let source_find = path("source")
        .and(path::param())
        .and(with_db(db.clone()))
        .and_then(handler::source::find);

    index.or(source_find)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    any().map(move || db.clone())
}
