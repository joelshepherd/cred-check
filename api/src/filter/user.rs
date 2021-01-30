use crate::{handler, Context};
use warp::Filter;

pub fn find(
    context: impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    context
        .and(warp::path("user"))
        .and(warp::path::param())
        .and_then(handler::user::find)
}

pub fn create(
    context: impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    context
        .and(warp::path("user"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::user::create)
}
