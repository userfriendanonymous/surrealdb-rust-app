use serde::Serialize;

use crate::db_pool::DbPool;
use crate::db_pool::Post as DbPost;

#[derive(Serialize)]
pub struct Post {
    title: String,
    content: String,
    id: i32,
}

impl From<DbPost> for Post {
    fn from(value: DbPost) -> Self {
        Self {
            title: value.title,
            content: value.content,
            id: value.id
        }
    }
}

pub async fn create(db_pool: &mut DbPool, title: String, content: String) -> Result<i32, String> {
    db_pool.create_post(title, content).await
}

pub async fn get(db_pool: &mut DbPool, id: i32) -> Result<Post, String> {
    Ok(Post::from(db_pool.get_post(id).await?))
}

pub async fn delete(db_pool: &mut DbPool, id: i32) -> Result<Post, String> {
    Ok(Post::from(db_pool.delete_post(id).await?))
}

pub async fn update(db_pool: &mut DbPool, id: i32, title: Option<String>, content: Option<String>) -> Result<Post, String> {
    Ok(Post::from(db_pool.update_post(id, title, content).await?))
}