use crate::{model, Db};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRequest {
    opinion_id: i64,
}

pub async fn create(
    db: Db,
    user: model::user::User,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the opinion exists
    model::opinion::find(&db, &request.opinion_id).await?;

    let input = model::vote::CreateVote {
        opinion_id: request.opinion_id,
        user_id: user.id,
    };

    model::vote::create(&db, input).await?;

    Ok(warp::reply())
}
