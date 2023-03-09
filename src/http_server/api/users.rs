use actix_web::{Scope, web::{self, Path}, get, Responder, HttpResponse};
use serde_json::json;

use crate::http_server::AppStateData;

pub fn service() -> Scope {
    web::scope("/users")
    .service(get_one)
}

#[get("/{name}")]
pub async fn get_one(app_state: AppStateData, name: Path<String>) -> impl Responder {
    let Some(ref session) = *app_state.session.lock().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "session not found"
        }));
    };

    match session.get_user((*name).as_str()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::NotFound().json(json!({
            "message": error
        }))
    }
}