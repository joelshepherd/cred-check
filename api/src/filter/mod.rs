mod opinion;
mod source;
mod supporter;
mod user;

use crate::{Context, Db};
use warp::Filter;

pub fn filters(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let context = warp::any().map(move || Context::new(db.clone()));

    let index = warp::path::end().map(|| warp::reply());

    index
        .or(opinion::create(context.clone()))
        .or(source::create(context.clone()))
        .or(source::find(context.clone()))
        .or(supporter::create(context.clone()))
        .or(user::create(context.clone()))
        .or(user::find(context))
}
