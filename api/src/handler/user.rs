use crate::{model, Db};

pub async fn find(db: Db, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::find(&db, id).await?;

    Ok(warp::reply::json(&user))
}
