use crate::Db;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Source {
    pub id: i32,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SourceInput {
    pub url: String,
}

/// Search for a source by url
pub async fn search(db: &Db, url: String) -> Result<Source, sqlx::Error> {
    sqlx::query_as!(Source, "select * from source where url = $1", url)
        .fetch_one(db)
        .await
}

/// Find a source by id
pub async fn find(db: &Db, id: i32) -> Result<Source, sqlx::Error> {
    sqlx::query_as!(Source, "select * from source where id = $1", id)
        .fetch_one(db)
        .await
}

/// Create a new source
pub async fn create(db: &Db, input: SourceInput) -> Result<Source, sqlx::Error> {
    // TODO: parse url and pull title
    let title = "TODO";
    let url = input.url;

    sqlx::query_as!(
        Source,
        "insert into source (title, url) values ($1, $2) returning *",
        title,
        url
    )
    .fetch_one(db)
    .await
}
