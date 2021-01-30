use crate::{model, Context};
use warp::{reject, reply};

pub async fn find(context: Context, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::find(&context.db, id)
        .await
        .map_err(|_| reject())?;

    Ok(reply::json(&user))
}

pub async fn create(
    context: Context,
    input: model::user::UserInput,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::create(&context.db, input)
        .await
        .map_err(|_| reject())?;

    Ok(reply::json(&user))
}
