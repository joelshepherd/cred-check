use crate::{model, Db};
use serde::Serialize;
use warp::{path::Tail, reject, reply};

#[derive(Serialize)]
struct FindData {
    source: model::source::Source,
    opinions: Vec<model::opinion::Opinion>,
}

pub async fn find(db: Db, url: Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url.as_str().to_owned();

    let source = model::source::find(&db, url).await.map_err(|_| reject())?;
    let opinions = model::opinion::find_by_source(&db, source.id)
        .await
        .map_err(|_| reject())?;

    let data = FindData { source, opinions };

    Ok(reply::json(&data))
}

pub async fn create(
    db: Db,
    input: model::source::SourceInput,
) -> Result<impl warp::Reply, warp::Rejection> {
    let source = model::source::create(&db, input)
        .await
        .map_err(|_| reject())?;

    Ok(reply::json(&source))
}
