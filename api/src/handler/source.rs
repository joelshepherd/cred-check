use crate::{model, Db};
use serde::Serialize;

#[derive(Serialize)]
struct SourceExt {
    source: model::source::Source,
    opinions: Vec<model::opinion::Opinion>,
    votes: (i64, i64),
}

impl From<model::source::Source> for SourceExt {
    fn from(source: model::source::Source) -> Self {
        Self {
            source,
            opinions: Vec::new(),
            votes: (0, 0),
        }
    }
}

pub async fn find(db: Db, url: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url.as_str().to_owned();

    let source = model::source::find_by_url(&db, url).await?;
    let opinions = model::opinion::find_by_source(&db, source.id).await?;
    let votes = model::vote::count_by_position(&db, source.id).await?;

    let data = SourceExt {
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

    let data: SourceExt = source.into();

    Ok(warp::reply::with_status(
        warp::reply::json(&data),
        warp::http::status::StatusCode::CREATED,
    ))
}
