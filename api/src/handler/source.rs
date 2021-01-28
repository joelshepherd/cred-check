use crate::model;
use serde::Serialize;
use warp::{reject, reply};

#[derive(Serialize)]
struct FindData {
    source: model::source::Source,
    opinions: Vec<model::opinion::Opinion>,
}

pub async fn find(url: String, db: crate::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let source = model::source::find(url, &db).await.map_err(|_| reject())?;
    let opinions = model::opinion::find_by_source(source.id, &db)
        .await
        .map_err(|_| reject())?;

    let data = FindData { source, opinions };

    Ok(reply::json(&data))
}
