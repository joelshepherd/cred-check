use crate::{model, Db};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    name: String,
    username: String,
}

#[derive(Serialize)]
pub struct TokenReply {
    token: String,
}

pub async fn login(db: Db, request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate the source exists
    let user = model::user::find_by_username(&db, request.username).await?;
    let token = crate::ENCODER.encode(&user.username).unwrap();
    let body = TokenReply { token };

    Ok(warp::reply::json(&body))
}

pub async fn signup(db: Db, input: SignupRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let data = model::user::CreateUser {
        name: input.name,
        username: input.username,
    };
    let user = model::user::create(&db, data).await?;
    let token = crate::ENCODER.encode(&user.username).unwrap();
    let body = TokenReply { token };

    Ok(warp::reply::with_status(
        warp::reply::json(&body),
        warp::http::status::StatusCode::CREATED,
    ))
}
