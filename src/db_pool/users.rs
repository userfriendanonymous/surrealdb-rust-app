use std::collections::BTreeMap;
use surrealdb::sql::{Value, Object};
use super::DbPool;
use super::utils::{b_tree_map, extract_single_object, query_result_into_objects};

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct CredentialUniqueness {
    pub name: bool,
    pub email: bool
}

impl Default for CredentialUniqueness {
    fn default() -> Self {
        Self {
            name: true,
            email: true
        }
    }
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

impl DbPool {
    pub async fn create_user(&self, name: impl Into<String>, email: impl Into<String>, password_hash: impl Into<String>) -> Result<User, String> {
        let data: BTreeMap<String, Value> = b_tree_map!(
            ("name", name.into() as String),
            ("email", email.into() as String),
            ("password_hash", password_hash.into() as String),
        );
        
        let vars: BTreeMap<String, Value> = b_tree_map!(
            ("data", data)
        );

        let responses = self.datastore.execute(
            "CREATE user CONTENT $data",
            &self.session,
            Some(vars),
            false
        ).await
        .map_err(|error| error.to_string())?;

        let objects = query_result_into_objects(responses)?;

        User::try_from(extract_single_object(objects)?)
    }

    pub async fn get_user(&self, name: &str) -> Result<User, String> {
        let vars: BTreeMap<String, Value> = b_tree_map!(
            ("name", name.to_string())
        );

        let responses = self.datastore.execute(
            "SELECT * FROM user WHERE name = $name",
            &self.session, Some(vars), false
        ).await
        .map_err(|error| error.to_string())?;

        let objects = query_result_into_objects(responses)?;
        User::try_from(extract_single_object(objects)?)
    }

    pub async fn check_if_unique_credentials(&self, name: &str, email: &str) -> Result<CredentialUniqueness, String> {
        let vars: BTreeMap<String, Value> = b_tree_map!(
            ("name", name.to_string()),
            ("email", email.to_string())
        );

        let responses = self.datastore.execute(
            "SELECT name, email FROM user WHERE name = $name OR email = $email",
            &self.session, Some(vars), false
        ).await
        .map_err(|error| error.to_string())?;
        
        let mut objects = query_result_into_objects(responses)?;
        
        match objects.next() {
            Some(object_result) => match object_result {
                Ok(object) => Ok(CredentialUniqueness {
                    name: object.get("name").ok_or("name field doesn't exist".to_owned())?.to_string().as_str() != name,
                    email: object.get("email").ok_or("email field doesn't exist".to_owned())?.to_string().as_str() != email
                }),
                Err(error) => Err(error)
            }
            None => Ok(CredentialUniqueness::default())
        }
    }
}