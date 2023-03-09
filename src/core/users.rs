use serde::Serialize;

use crate::db_pool::DbPool;
use crate::db_pool::User as DbUser;

#[derive(Serialize)]
pub struct User {
    name: String,
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        Self {
            name: value.name
        }
    }
}

pub async fn create(db_pool: &mut DbPool, name: String, email: String, password_hash: String) -> Result<User, String> {
    Ok(User::from(db_pool.create_user(name, email, password_hash).await?))
}

pub async fn get(db_pool: &mut DbPool, name: &str) -> Result<User, String> {
    Ok(User::from(db_pool.get_user(name).await?))
}