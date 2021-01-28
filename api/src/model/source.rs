use serde::Serialize;

#[derive(Serialize)]
pub struct Source {
    pub id: i64,
    pub url: String,
}

/// Find a source by type and path
pub async fn find(path: String, db: &crate::Db) -> Result<Source, sqlx::Error> {
    sqlx::query_as!(Source, "select * from source where url = ?", path)
        .fetch_one(db)
        .await
}
