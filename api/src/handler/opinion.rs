use crate::{model, Context};
use serde::Deserialize;
use warp::{reject, reply};

#[derive(Deserialize)]
pub struct CreateRequest {
    source_id: i32,
    position: bool,
    body: String,
}

pub async fn create(
    context: Context,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the source exists
    model::source::find(&context.db, request.source_id)
        .await
        .map_err(|_| reject())?;

    let input = model::opinion::OpinionInput {
        user_id: context.user.id,
        source_id: request.source_id,
        position: request.position,
        body: request.body,
    };

    let opinion = model::opinion::create(&context.db, input)
        .await
        .map_err(|_| reject())?;

    Ok(reply::json(&opinion))
}
