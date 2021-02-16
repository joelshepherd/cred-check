use crate::{error, Db};
use serde::Serialize;

#[derive(Serialize)]
pub struct Vote {
    pub id: i32,
    pub opinion_id: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateVote {
    pub opinion_id: i32,
    pub user_id: i32,
}

/// Count votes for source
pub async fn count_by_position(db: &Db, source_id: i32) -> error::Result<(i64, i64)> {
    let rows = sqlx::query!(
        "select position, count(*) from vote inner join opinion on vote.opinion_id = opinion.id where opinion.source_id = $1 group by opinion.position",
        &source_id
    )
    .fetch_all(db)
    .await?;

    let mut votes = (0, 0);

    for row in rows.iter() {
        if row.position == true {
            votes.0 = row.count.unwrap_or(0);
        } else {
            votes.1 = row.count.unwrap_or(0);
        }
    }

    Ok(votes)
}

/// Create a new vote
pub async fn create(db: &Db, input: CreateVote) -> error::Result<Vote> {
    let opinion = super::opinion::find(db, input.opinion_id).await?;

    // Delete any existing votes
    sqlx::query!(
        "delete from vote where user_id = $1 and opinion_id in (select id from opinion where source_id = $2)",
        &input.user_id,
        &opinion.source_id,
    )
    .execute(db)
    .await?;

    let vote = sqlx::query_as!(
        Vote,
        "insert into vote (opinion_id, user_id) values ($1, $2) returning *",
        input.opinion_id,
        input.user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(vote)
}
