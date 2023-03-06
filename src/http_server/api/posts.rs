use actix_web::{web::scope, Scope};

pub fn service() -> Scope {
    scope("/posts")
}