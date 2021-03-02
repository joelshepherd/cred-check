use crate::{model, Db};
use serde::Serialize;

#[derive(Serialize)]
struct UserReply {
    id: i64,
    name: String,
}

impl From<model::user::User> for UserReply {
    fn from(user: model::user::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

pub async fn find(db: Db, id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::find(&db, &id).await?;
    let reply = UserReply::from(user);

    Ok(warp::reply::json(&reply))
}
