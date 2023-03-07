use actix_web::{web::scope, Scope, Responder, post, get, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{http_server::AppStateData, session::Session};

pub fn service() -> Scope {
    scope("/posts")
    .service(create)
    .service(get_one)
}

#[derive(Deserialize)]
pub struct CreateBody {
    title: String,
    content: String
}

#[post("/")]
pub async fn create(app_state: AppStateData, body: web::Json<CreateBody>) -> impl Responder {
    let Some(ref session) = *app_state.session.lock().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "session not found"
        }));
    };

    match session.create_post(body.title.clone(), body.content.clone()).await {
        Ok(id) => HttpResponse::Created().json(json!({
            "id": id
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": error
        }))
    }
}

#[get("/{id}")]
pub async fn get_one(app_state: AppStateData, id: web::Path<i32>) -> impl Responder {
    let Some(ref session) = *app_state.session.lock().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "session not found"
        }));
    };

    match session.get_post(*id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": error
        }))
    }
}