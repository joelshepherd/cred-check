use crate::{error, parser, Db};

pub struct Source {
    pub id: i64,
    pub title: String,
    pub canonical_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateSource {
    pub url: String,
}

/// Find a source by id
pub async fn find(db: &Db, id: i64) -> error::Result<Source> {
    let source = sqlx::query_as!(Source, "select * from source where id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(source)
}

/// Search for a source by url
pub async fn find_by_url(db: &Db, url: String) -> error::Result<Source> {
    let source = sqlx::query_as!(Source, "select source.* from source inner join alternative on source.id = alternative.source_id where alternative.url = $1", url)
        .fetch_one(db)
        .await?;

    Ok(source)
}

/// Create a new source
pub async fn create(db: &Db, input: CreateSource) -> error::Result<Source> {
    // Fetch canonical url and title
    let (canonical_url, title) = parser::parse(input.url.clone()).await?;

    let source = sqlx::query_as!(
        Source,
        "select * from source where canonical_url = $1",
        &canonical_url
    )
    .fetch_optional(db)
    .await?;

    let source = match source {
        Some(source) => source,
        None => {
            // TODO: transaction for insert source and alternative

            let new_source = sqlx::query_as!(
                Source,
                "insert into source (title, canonical_url) values ($1, $2) returning *",
                title,
                canonical_url
            )
            .fetch_one(db)
            .await?;

            // Save the canonical url
            create_alternative(db, &new_source.id, &canonical_url).await?;

            new_source
        }
    };

    // Save the input url if it's different
    if input.url != canonical_url {
        create_alternative(db, &source.id, &input.url).await?;
    }

    Ok(source)
}

/// Create a new alternative url for a source
async fn create_alternative(db: &Db, source_id: &i64, url: &str) -> error::Result<()> {
    sqlx::query!(
        "insert into alternative (source_id, url) values ($1, $2)",
        &source_id,
        &url
    )
    .execute(db)
    .await?;

    Ok(())
}
