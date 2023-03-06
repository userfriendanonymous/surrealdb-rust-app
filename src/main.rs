use db_pool::DbPool;

mod http_server;
mod core;
mod db_pool;
mod session;

#[actix_web::main]
async fn main() {
    let db_pool = DbPool::new().await;

    http_server::launch(db_pool).await;
}
