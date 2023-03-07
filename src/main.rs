mod http_server;
mod core;
mod db_pool;
mod session;

use db_pool::DbPool;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() {
    let db_pool_state = Arc::new(Mutex::new(DbPool::new().await));

    http_server::launch(db_pool_state.clone()).await;
}
