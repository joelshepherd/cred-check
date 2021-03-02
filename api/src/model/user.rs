use crate::{error, Db};

pub struct User {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateUser {
    pub name: String,
    pub username: String,
}

/// Find a user by id
pub async fn find(db: &Db, id: &i64) -> error::Result<User> {
    let user = sqlx::query_as!(User, "select * from \"user\" where id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(user)
}

/// Find a user by username
pub async fn find_by_username(db: &Db, username: &str) -> error::Result<User> {
    let user = sqlx::query_as!(User, "select * from \"user\" where username = $1", username)
        .fetch_one(db)
        .await?;

    Ok(user)
}

/// Find a user by token
pub async fn find_from_token(db: &Db, token: &str) -> Option<User> {
    let token = crate::DECODER.decode::<String>(&token).ok()?;
    find_by_username(db, &token).await.ok()
}

/// Create a new user
pub async fn create(db: &Db, input: CreateUser) -> error::Result<User> {
    let user = sqlx::query_as!(
        User,
        "insert into \"user\" (name, username) values ($1, $2) returning *",
        input.name,
        input.username
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}
