use crate::Db;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserInput {
    name: String,
}

/// Find a user by id
pub async fn find(db: &Db, path: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "select * from \"user\" where id = $1", path)
        .fetch_one(db)
        .await
}
/// Create a new user
pub async fn create(db: &Db, input: UserInput) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "insert into \"user\" (name) values ($1) returning *",
        input.name
    )
    .fetch_one(db)
    .await
}
