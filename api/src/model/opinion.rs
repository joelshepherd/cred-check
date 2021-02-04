use crate::{error, Db};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Opinion {
    pub id: i32,
    pub source_id: i32,
    pub user_id: i32,
    pub position: bool,
    pub body: String,
}

#[derive(Deserialize)]
pub struct CreateOpinion {
    pub source_id: i32,
    pub user_id: i32,
    pub position: bool,
    pub body: String,
}

/// Find a opinion by id
pub async fn find(db: &Db, id: i32) -> error::Result<Opinion> {
    let opinion = sqlx::query_as!(Opinion, "select * from opinion where id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(opinion)
}

/// Find opinions by source
pub async fn find_by_source(db: &Db, source_id: i32) -> error::Result<Vec<Opinion>> {
    let opinions = sqlx::query_as!(
        Opinion,
        "select * from opinion where source_id = $1",
        source_id
    )
    .fetch_all(db)
    .await?;

    Ok(opinions)
}

/// Create a new opinion
pub async fn create(db: &Db, input: CreateOpinion) -> error::Result<Opinion> {
    let opinion = sqlx::query_as!(
        Opinion,
        "insert into opinion (source_id, user_id, position, body) values ($1, $2, $3, $4) returning *",
        input.source_id,
        input.user_id,
        input.position, input.body
    )
    .fetch_one(db)
    .await?;

    Ok(opinion)
}
