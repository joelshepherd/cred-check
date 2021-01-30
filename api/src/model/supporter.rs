use crate::Db;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Supporter {
    pub id: i32,
    pub opinion_id: i32,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct SupporterInput {
    pub opinion_id: i32,
    pub user_id: i32,
}

/// Count supporters by opinion
pub async fn count_by_opinion(db: &Db, opinion_id: i32) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "select count(*) from supporter where opinion_id = $1",
        opinion_id
    )
    .map(|row| row.count.unwrap_or(0))
    .fetch_one(db)
    .await
}

/// Create a new supporter
pub async fn create(db: &Db, input: SupporterInput) -> Result<Supporter, sqlx::Error> {
    sqlx::query_as!(
        Supporter,
        "insert into supporter (opinion_id, user_id) values ($1, $2) returning *",
        input.opinion_id,
        input.user_id,
    )
    .fetch_one(db)
    .await
}
