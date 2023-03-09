use actix_web::{Responder, Scope, web::{self, Json}, post, HttpResponse, cookie::Cookie};
use serde::Deserialize;
use serde_json::json;

use crate::http_server::AppStateData;

pub fn service() -> Scope {
    web::scope("/auth")
    .service(login)
    .service(register)
}

#[derive(Deserialize)]
pub struct LoginBody {
    name: String,
    password: String
}

#[derive(Deserialize)]
pub struct RegisterBody {
    name: String,
    password: String,
    email: String
}

#[post("/login")]
pub async fn login(app_state: AppStateData, body: Json<LoginBody>) -> impl Responder {
    let Some(ref session) = *app_state.session.lock().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "session not found"
        }));
    };

    match session.login(body.name.as_str(), body.password.as_str()).await {
        Ok(tokens) => HttpResponse::Ok()
        .cookie(
            Cookie::build("access-token", tokens.access)
            .http_only(true)
            .finish()
        )
        .cookie(
            Cookie::build("key-token", tokens.key)
            .finish()
        )
        .json(json!({
            "message": "success"
        })),

        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": error
        }))
    }
}

#[post("/register")]
pub async fn register(app_state: AppStateData, body: Json<RegisterBody>) -> impl Responder {
    let Some(ref session) = *app_state.session.lock().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "session not found"
        }));
    };

    println!("Got register request: {}, {}", body.name, body.email);

    match session.register(body.name.as_str(), body.email.as_str(), body.password.as_str()).await {
        Ok(tokens) => HttpResponse::Ok()
        .cookie(
            Cookie::build("access-token", tokens.access)
            .http_only(true)
            .finish()
        )
        .cookie(
            Cookie::build("key-token", tokens.key)
            .finish()
        )
        .json(json!({
            "message": "success"
        })),

        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": error.to_string()
        }))
    }
}