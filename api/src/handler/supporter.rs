use crate::{model, Db};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRequest {
    opinion_id: i32,
}

pub async fn create(
    db: Db,
    user: model::user::User,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the opinion exists
    model::opinion::find(&db, request.opinion_id).await?;

    let input = model::supporter::CreateSupporter {
        opinion_id: request.opinion_id,
        user_id: user.id,
    };

    let supporter = model::supporter::create(&db, input).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&supporter),
        warp::http::status::StatusCode::CREATED,
    ))
}
