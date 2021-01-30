use crate::{model, Context};
use serde::Serialize;
use warp::{http::status, path::Tail, reject, reply};

#[derive(Serialize)]
struct FindReply {
    source: model::source::Source,
    opinions: Vec<model::opinion::Opinion>,
    votes: (i64, i64),
}

pub async fn find(context: Context, url: Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url.as_str().to_owned();

    let source = model::source::search(&context.db, url)
        .await
        .map_err(|_| reject())?;

    let opinions = model::opinion::find_by_source(&context.db, source.id)
        .await
        .map_err(|_| reject())?;

    let mut votes = (0, 0);

    for opinion in opinions.iter() {
        let supporters = model::supporter::count_by_opinion(&context.db, opinion.id)
            .await
            .map_err(|_| reject())?;

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

    Ok(reply::json(&data))
}

pub async fn create(
    context: Context,
    input: model::source::SourceInput,
) -> Result<impl warp::Reply, warp::Rejection> {
    let source = model::source::create(&context.db, input)
        .await
        .map_err(|_| reject())?;

    Ok(reply::with_status(
        warp::reply::json(&source),
        status::StatusCode::CREATED,
    ))
}
