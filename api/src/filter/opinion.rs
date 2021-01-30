use crate::{handler, Context};
use warp::Filter;

pub fn create(
    context: impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    context
        .and(warp::path("opinion"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::opinion::create)
}
