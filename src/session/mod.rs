use ::core::fmt;
use std::sync::{Arc, Mutex};

use crate::db_pool::DbPool;
use crate::core;
use crate::core::{
    posts::Post,
    users::User,
    auth::{Auth, Tokens, Info as AuthInfo}
};

pub type DbPoolState = Arc<Mutex<DbPool>>;

pub struct Session {
    db_pool: DbPoolState,
    auth: Auth
}

macro_rules! db {
    ($self:expr) => {
        &mut (*$self.db_pool.lock().unwrap())
    };
}

pub enum RegisterError {
    NameTaken,
    EmailTaken,
    InvalidData(String),
    Internal(String),
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterError::EmailTaken => write!(f, "Email is already taken"),
            RegisterError::NameTaken => write!(f, "Name is already taken"),
            RegisterError::InvalidData(message) => write!(f, "Invalid Data: {message}"),
            RegisterError::Internal(message) => write!(f, "Internal Error: {message}"),
        }
    }
}

impl Session {
    pub fn new(db_pool: DbPoolState, tokens: Tokens) -> Self {
        Self {
            db_pool,
            auth: Auth::from(tokens)
        }
    }

    pub async fn create_post(&self, title: String, content: String) ->  Result<i32, String> {
        core::posts::create(db!(self), title, content).await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, String> {
        core::posts::get(db!(self), id).await
    }

    pub async fn delete_post(&self, id: i32) -> Result<Post, String> {
        core::posts::delete(db!(self), id).await
    }

    pub async fn update_post(&self, id: i32, title: Option<String>, content: Option<String>) -> Result<Post, String> {
        core::posts::update(db!(self), id, title, content).await
    }

    pub async fn get_user(&self, name: &str) -> Result<User, String> {
        core::users::get(db!(self), name).await
    }

    pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<Tokens, RegisterError> {
        core::auth::validate_credentials_data(&name, &email, &password).map_err(|error| RegisterError::InvalidData(error))?;
        let tokens = core::auth::Tokens::try_from(AuthInfo {
            name: name.clone().to_owned()
        }).map_err(|error| RegisterError::Internal(error))?;

        let db_pool = self.db_pool.lock().unwrap();
        let uniqueness = db_pool.check_if_unique_credentials(name, email).await
        .map_err(|error| RegisterError::Internal(error))?;
        
        if !uniqueness.name {
            return Err(RegisterError::NameTaken)
        }

        if !uniqueness.email {
            return Err(RegisterError::EmailTaken)
        }

        let password_hash = core::auth::hash_password(password.clone()).map_err(|error| RegisterError::Internal(error))?;

        core::users::create(db!(self), name.clone().to_owned(), email.clone().to_owned(), password_hash).await
        .map_err(|error| RegisterError::Internal(error))?;
        Ok(tokens)
    }

    pub async fn login(&self, name: &str, password: &str) -> Result<Tokens, String> {
        let db_pool = self.db_pool.lock().unwrap();
        let user = db_pool.get_user(name).await?;
        if !core::auth::compare_password(password, user.password_hash.as_str()) {
            return Err("Invalid credentials".to_owned());
        }

        Tokens::try_from(AuthInfo {
            name: name.clone().to_owned()
        })
    }
}