use crate::{model::user::User, Db};

pub struct Context {
    pub db: Db,
    pub user: User,
}

impl Context {
    pub fn new(db: Db) -> Self {
        let user = User {
            id: 1,
            name: "TODO".to_string(),
        };

        Self { db, user }
    }
}
