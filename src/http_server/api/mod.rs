pub mod posts;

use actix_web::{web::{scope}, Scope};

pub fn service() -> Scope {
    scope("/api")
    .service(posts::service())
}