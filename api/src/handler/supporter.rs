use crate::{model, Context};
use serde::Deserialize;
use warp::{reject, reply};

#[derive(Deserialize)]
pub struct CreateRequest {
    opinion_id: i32,
}

pub async fn create(
    context: Context,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the opinion exists
    model::opinion::find(&context.db, request.opinion_id)
        .await
        .map_err(|_| reject())?;

    let input = model::supporter::SupporterInput {
        opinion_id: request.opinion_id,
        user_id: context.user.id,
    };

    let supporter = model::supporter::create(&context.db, input)
        .await
        .map_err(|_| reject())?;

    Ok(reply::json(&supporter))
}
