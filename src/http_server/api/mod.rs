pub mod posts;
pub mod auth;
pub mod users;

use actix_web::{web::{scope}, Scope};

pub fn service() -> Scope {
    scope("/api")
    .service(posts::service())
    .service(auth::service())
    .service(users::service())
}