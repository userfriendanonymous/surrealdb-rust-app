mod api;
mod middleware;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web::Data};
use crate::session::{Session, DbPoolState};

pub struct AppState {
    db_pool: DbPoolState,
    pub session: Mutex<Option<Session>>
}

pub type AppStateData = Data<AppState>;

pub async fn launch(db_pool: DbPoolState){
    let app_state = Data::new(AppState {
        db_pool,
        session: Mutex::new(None)
    });

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::session::Factory::new(app_state.clone()))
        .app_data(app_state.clone())
        .service(api::service())
    })
    .bind(("127.0.0.1", 5000)).unwrap()
    .run()
    .await.unwrap();
}