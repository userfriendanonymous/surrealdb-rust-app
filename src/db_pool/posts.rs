use std::collections::BTreeMap;
use surrealdb::sql::{Value, Object};
use super::DbPool;
use super::utils::{b_tree_map, extract_single_object, query_result_into_objects};

pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl TryFrom<Object> for Post {
    type Error = String;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: object.get("id").ok_or("Id not found")?.clone().as_int() as i32,
            title: object.get("title").ok_or("title not found")?.clone().as_string(),
            content: object.get("content").ok_or("content not found")?.clone().as_string(),
        })
    }
}

impl DbPool {
    pub async fn create_post(&self, title: impl Into<String>, content: impl Into<String>) -> Result<i32, String> {
        let data: BTreeMap<String, Value> = b_tree_map!(
            ("title", title.into() as String),
            ("content", content.into() as String)
        );
        
        let vars: BTreeMap<String, Value> = b_tree_map!(
            ("data", data)
        );

        let responses = self.datastore.execute(
            "CREATE post CONTENT $data",
            &self.session,
            Some(vars),
            false
        ).await
        .map_err(|error| error.to_string())?;

        let objects = query_result_into_objects(responses)?;

        Ok(extract_single_object(objects)?.get("id").ok_or("id field missing")?.clone().as_int() as i32) // as whatever you want
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, String> {
        let vars: BTreeMap<String, Value> = b_tree_map!(
            ("id", id)
        );

        let responses = self.datastore.execute(
            "SELECT * FROM post WHERE id = $1",
            &self.session,
            Some(vars),
            false
        ).await
        .map_err(|error| error.to_string())?;

        let objects = query_result_into_objects(responses)?;
        Post::try_from(extract_single_object(objects)?)
    }
}