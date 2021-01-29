use serde::Serialize;

#[derive(Serialize)]
pub struct Opinion {
    pub id: i64,
    pub source_id: i64,
    pub position: bool,
    pub body: String,
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
