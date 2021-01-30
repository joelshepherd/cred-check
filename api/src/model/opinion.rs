use crate::Db;
use serde::Serialize;

#[derive(Serialize)]
pub struct Opinion {
    pub id: i32,
    pub source_id: i32,
    pub position: bool,
    pub body: String,
}

/// Find opinions by source
pub async fn find_by_source(db: &Db, source_id: i32) -> Result<Vec<Opinion>, sqlx::Error> {
    sqlx::query_as!(
        Opinion,
        "select * from opinion where source_id = $1",
        source_id
    )
    .fetch_all(db)
    .await
}
