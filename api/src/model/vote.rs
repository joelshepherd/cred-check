use crate::{error, Db};

pub struct Vote {
    pub id: i64,
    pub source_id: i64,
    pub opinion_id: i64,
    pub user_id: i64,
    pub position: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateVote {
    pub opinion_id: i64,
    pub user_id: i64,
}

/// Count votes for source
pub async fn count_by_position(db: &Db, source_id: &i64) -> error::Result<(i64, i64)> {
    let rows = sqlx::query!(
        "select position, count(*) from vote where source_id = $1 group by position",
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
    let opinion = super::opinion::find(db, &input.opinion_id).await?;

    let vote = sqlx::query_as!(
        Vote,
        "insert into vote (source_id, user_id, opinion_id, position) values ($1, $2, $3, $4)
        on conflict (source_id, user_id) do update set opinion_id = $3, position = $4, created_at = now()
        returning *",
        opinion.source_id,
        input.user_id,
        input.opinion_id,
        opinion.position
    )
    .fetch_one(db)
    .await?;

    Ok(vote)
}
