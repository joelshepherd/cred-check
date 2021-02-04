use crate::{error, Db};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Supporter {
    pub id: i32,
    pub opinion_id: i32,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct CreateSupporter {
    pub opinion_id: i32,
    pub user_id: i32,
}

/// Count supporters by opinion
pub async fn count_by_opinion(db: &Db, opinion_id: i32) -> error::Result<i64> {
    let count = sqlx::query_scalar!(
        "select count(*) from supporter where opinion_id = $1",
        opinion_id
    )
    .fetch_one(db)
    .await?
    .unwrap_or(0);

    Ok(count)
}

/// Create a new supporter
pub async fn create(db: &Db, input: CreateSupporter) -> error::Result<Supporter> {
    let supporter = sqlx::query_as!(
        Supporter,
        "insert into supporter (opinion_id, user_id) values ($1, $2) returning *",
        input.opinion_id,
        input.user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(supporter)
}
