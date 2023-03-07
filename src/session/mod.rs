use std::sync::{Arc, Mutex};

use crate::core::auth::{Auth, Tokens};
use crate::db_pool::DbPool;
use crate::core;
use crate::core::posts::Post;

pub type DbPoolState = Arc<Mutex<DbPool>>;

pub struct Session {
    db_pool: DbPoolState,
    auth: Auth
}

impl Session {
    pub fn new(db_pool: DbPoolState, tokens: Tokens) -> Self {
        Self {
            db_pool,
            auth: Auth::from(tokens)
        }
    }

    pub async fn create_post(&self, title: String, content: String) ->  Result<i32, String> {
        core::posts::create(&mut (*self.db_pool.lock().unwrap()), title, content).await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, String> {
        core::posts::get(&mut (*self.db_pool.lock().unwrap()), id).await
    }
}