use crate::{model, Db};

pub async fn find(db: Db, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::find(&db, id).await?;

    Ok(warp::reply::json(&user))
}

pub async fn create(
    db: Db,
    input: model::user::CreateUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user = model::user::create(&db, input).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::status::StatusCode::CREATED,
    ))
}
