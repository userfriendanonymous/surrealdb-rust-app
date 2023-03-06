use std::collections::BTreeMap;
use surrealdb::{sql::{self, Value, Object}, Response};
use super::DbPool;

macro_rules! bTreeMap {
    (
        $(($name:expr, $value:expr)),*
        $(,)? // for trailing commas
    ) => {
        [
            $(
                ($name.into(), $value.into()),
            )*
        ].into()
    };
}

fn query_result_into_objects(responses: Vec<Response>) -> Result<impl Iterator<Item = Result<Object, String>>, String> {
    let r = responses.into_iter().next().map(|response| response.result);

    let Some(Ok(Value::Array(sql::Array(values)))) = r else {
        return Err("NO.".to_string())
    };

    Ok(values.into_iter().map(|value| match value {
        Value::Object(value) => Ok(value),
        _ => Err("lol".to_string())
    }))
}

fn extract_single_object(mut objects: impl Iterator<Item = Result<Object, String>>) -> Result<Object, String> {
    match objects.next() {
        Some(object) => {
            object.map_err(|error| error.to_string())
        },

        None => Err("not found".to_string())
    }
}

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl TryFrom<Object> for User {
    type Error = String;
    
    fn try_from(object: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: object.get("id").ok_or("Id not found")?.clone().as_int() as i32,
            name: object.get("name").ok_or("Name not found")?.clone().as_string(),
            email: object.get("email").ok_or("Email not found")?.clone().as_string(),
            password_hash: object.get("password_hash").ok_or("Password hash not found")?.clone().as_string()
        })
    }
}

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
    pub async fn create_user(&self, name: impl Into<String>, email: impl Into<String>, password_hash: impl Into<String>) -> Result<User, String> {
        let data: BTreeMap<String, Value> = bTreeMap!(
            ("name", name.into() as String),
            ("email", email.into() as String),
            ("password_hash", password_hash.into() as String),
        );
        
        let vars: BTreeMap<String, Value> = bTreeMap!(
            ("data", data)
        );

        let responses = self.datastore.execute(
            "CREATE user CONTENT $data",
            &self.session,
            Some(vars),
            false
        ).await
        .map_err(|error| error.to_string())?;

        let mut objects = query_result_into_objects(responses)?;

        User::try_from(extract_single_object(objects)?)
    }

    pub async fn create_post(&self, title: impl Into<String>, content: impl Into<String>) -> Result<i32, String> {
        let data: BTreeMap<String, Value> = bTreeMap!(
            ("title", title.into() as String),
            ("content", content.into() as String)
        );
        
        let vars: BTreeMap<String, Value> = bTreeMap!(
            ("data", data)
        );

        let responses = self.datastore.execute(
            "CREATE post CONTENT $data",
            &self.session,
            Some(vars),
            false
        ).await
        .map_err(|error| error.to_string())?;

        let mut objects = query_result_into_objects(responses)?;

        Ok(extract_single_object(objects)?.get("id").ok_or("id field missing")?.clone().as_int() as i32) // as whatever you want
    }

}