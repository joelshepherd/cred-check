use crate::{model, Db};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRequest {
    source_id: i64,
    position: bool,
    body: String,
}

#[derive(Serialize)]
pub struct OpinionReply {
    id: i64,
    position: bool,
    body: String,
}

impl From<model::opinion::Opinion> for OpinionReply {
    fn from(opinion: model::opinion::Opinion) -> Self {
        Self {
            id: opinion.id,
            position: opinion.position,
            body: opinion.body,
        }
    }
}

pub async fn create(
    db: Db,
    user: model::user::User,
    request: CreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the source exists
    model::source::find(&db, &request.source_id).await?;

    let input = model::opinion::CreateOpinion {
        user_id: user.id,
        source_id: request.source_id,
        position: request.position,
        body: request.body,
    };
    let opinion = model::opinion::create(&db, input).await?;
    let reply = OpinionReply::from(opinion);

    Ok(warp::reply::with_status(
        warp::reply::json(&reply),
        warp::http::status::StatusCode::CREATED,
    ))
}
