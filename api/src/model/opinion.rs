use serde::Serialize;

#[derive(Serialize)]
pub struct Opinion {
    id: i64,
    source_id: i64,
    position: bool,
    body: String,
}

/// Find opinions by source
pub async fn find_by_source(source_id: i64, db: &crate::Db) -> Result<Vec<Opinion>, sqlx::Error> {
    sqlx::query_as!(
        Opinion,
        "select * from opinion where source_id = ?",
        source_id
    )
    .fetch_all(db)
    .await
}
