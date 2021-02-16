use crate::{error, Db};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Source {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSource {
    pub url: String,
}

/// Find a source by id
pub async fn find(db: &Db, id: i32) -> error::Result<Source> {
    let source = sqlx::query_as!(Source, "select * from source where id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(source)
}

/// Search for a source by url
pub async fn find_by_url(db: &Db, url: String) -> error::Result<Source> {
    let source = sqlx::query_as!(Source, "select * from source where url = $1", url)
        .fetch_one(db)
        .await?;

    Ok(source)
}

/// Create a new source
pub async fn create(db: &Db, input: CreateSource) -> error::Result<Source> {
    // TODO: parse url and pull title
    let title = "TODO";
    let url = input.url;

    let source = sqlx::query_as!(
        Source,
        "insert into source (title, url) values ($1, $2) returning *",
        title,
        url
    )
    .fetch_one(db)
    .await?;

    Ok(source)
}
