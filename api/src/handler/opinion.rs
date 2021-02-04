use crate::{model, Db};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRequest {
    source_id: i32,
    position: bool,
    body: String,
}

pub async fn create(
    db: Db,
    user: model::user::User,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the source exists
    model::source::find(&db, request.source_id).await?;

    let input = model::opinion::CreateOpinion {
        user_id: user.id,
        source_id: request.source_id,
        position: request.position,
        body: request.body,
    };

    let opinion = model::opinion::create(&db, input).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&opinion),
        warp::http::status::StatusCode::CREATED,
    ))
}
