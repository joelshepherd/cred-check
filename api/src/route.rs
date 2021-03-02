use crate::{error, handler, model, Db};
use warp::Filter;

/// Init routes
pub fn init(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(warp::reply)
        .or(warp::path("login")
            .and(warp::post())
            .and(with_db(db.clone()))
            .and(warp::body::json())
            .and_then(handler::auth::login))
        .or(warp::path("signup")
            .and(warp::post())
            .and(with_db(db.clone()))
            .and(warp::body::json())
            .and_then(handler::auth::signup))
        .or(warp::path("opinion")
            .and(warp::post())
            .and(with_db_and_user(db.clone()))
            .and(warp::body::json())
            .and_then(handler::opinion::create))
        .or(warp::path("source").and(
            warp::get()
                .and(with_db(db.clone()))
                .and(warp::path::tail())
                .and_then(handler::source::find)
                .or(warp::post()
                    .and(with_db(db.clone()))
                    .and(warp::body::json())
                    .and_then(handler::source::create)),
        ))
        .or(warp::path("vote").and(
            warp::post()
                .and(with_db_and_user(db.clone()))
                .and(warp::body::json())
                .and_then(handler::vote::create),
        ))
        .or(warp::path("user").and(
            warp::get()
                .and(with_db(db.clone()))
                .and(warp::path::param())
                .and_then(handler::user::find),
        ))
        .recover(map_error)
        .with(
            warp::cors()
                .allow_methods(vec!["GET", "POST"])
                .allow_headers(vec!["authorization", "content-type"])
                .allow_any_origin()
                .allow_credentials(true)
                .build(),
        )
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_db_and_user(
    db: Db,
) -> impl Filter<Extract = (Db, model::user::User), Error = warp::Rejection> + Clone {
    warp::header("authorization")
        .and_then(move |token| map_user(db.clone(), token))
        .untuple_one()
}

// Separate function because you cannot async closure in stable yet
async fn map_user(db: Db, token: String) -> Result<(Db, model::user::User), warp::Rejection> {
    let user = match model::user::find_from_token(&db, &token).await {
        Some(user) => user,
        None => return Err(error::Error::Unauthorised.into()),
    };

    Ok((db, user))
}

async fn map_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:#?}", err);

    if let Some(err) = err.find::<error::Error>() {
        let status = match err {
            error::Error::Internal => warp::http::status::StatusCode::INTERNAL_SERVER_ERROR,
            error::Error::NotFound => warp::http::status::StatusCode::NOT_FOUND,
            error::Error::Unauthorised => warp::http::status::StatusCode::UNAUTHORIZED,
        };

        return Ok(warp::reply::with_status(warp::reply::html(""), status));
    }

    Err(err)
}
