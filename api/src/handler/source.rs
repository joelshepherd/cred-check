use crate::{model, Db};
use serde::Serialize;

#[derive(Serialize)]
struct FindReply {
    source: model::source::Source,
    opinions: Vec<model::opinion::Opinion>,
    votes: (i64, i64),
}

pub async fn find(db: Db, url: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url.as_str().to_owned();

    let source = model::source::find_by_url(&db, url).await?;
    let opinions = model::opinion::find_by_source(&db, source.id).await?;

    let mut votes = (0, 0);

    // TODO: fix n+1 issue
    for opinion in opinions.iter() {
        let supporters = model::supporter::count_by_opinion(&db, opinion.id).await?;

        if opinion.position {
            votes.0 += 1 + supporters;
        } else {
            votes.1 += 1 + supporters;
        }
    }

    let data = FindReply {
        source,
        opinions,
        votes,
    };

    Ok(warp::reply::json(&data))
}

pub async fn create(
    db: Db,
    input: model::source::CreateSource,
) -> Result<impl warp::Reply, warp::Rejection> {
    let source = model::source::create(&db, input).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&source),
        warp::http::status::StatusCode::CREATED,
    ))
}
