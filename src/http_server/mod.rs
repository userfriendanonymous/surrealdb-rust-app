mod api;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web::Data};
use crate::{db_pool::DbPool, session::Session};
pub struct AppState {
    db_pool: DbPool,
    pub session: Mutex<Option<Session>>
}

pub async fn launch(db_pool: DbPool){
    let app_state = Data::new(AppState {
        db_pool,
        session: Mutex::new(None)
    });

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .service(api::service())
    })
    .bind(("127.0.0.1", 5000)).unwrap()
    .run()
    .await.unwrap();
}