use crate::db_pool::DbPool;

pub async fn create_post(db_pool: DbPool, title: String, content: String) -> Result<i32, String> {
    db_pool.create_post(title, content).await
}