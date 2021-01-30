use crate::Db;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Source {
    pub id: i32,
    pub url: String,
}

/// Find a source by type and path
pub async fn find(db: &Db, path: String) -> Result<Source, sqlx::Error> {
    sqlx::query_as!(Source, "select * from source where url = $1", path)
        .fetch_one(db)
        .await
}

#[derive(Deserialize)]
pub struct SourceInput {
    url: String,
}

pub async fn create(db: &Db, input: SourceInput) -> Result<Source, sqlx::Error> {
    sqlx::query!("insert into source (url) values ($1)", input.url)
        .execute(db)
        .await?;

    find(db, input.url).await
}
