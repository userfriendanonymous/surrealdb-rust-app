use serde::{Serialize, Deserialize};
use rand::Rng;
use jsonwebtoken::{DecodingKey, Validation, Algorithm, EncodingKey};
use pwhash::bcrypt;

pub struct Tokens {
    pub access: String,
    pub key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub name: String,
    pub key: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct KeyClaims {
    pub key: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct Info {
    pub name: String,
}

pub enum Auth {
    Valid {
        info: Info
    },
    Invalid(String)
}

impl From<Tokens> for Auth {
    fn from(tokens: Tokens) -> Self {
        let keys = get_keys();
        let access_claims = match jsonwebtoken::decode::<AccessClaims>(
            &tokens.access,
            &DecodingKey::from_secret(&keys.access.as_bytes()),
            &Validation::new(Algorithm::HS512)
        ) {
            Ok(claims) => claims,
            Err(error) => return Auth::Invalid(error.to_string())
        }.claims;

        let key_claims = match jsonwebtoken::decode::<KeyClaims>(
            &tokens.key,
            &DecodingKey::from_secret(&keys.key.as_bytes()),
            &Validation::new(Algorithm::HS512)
        ) {
            Ok(claims) => claims,
            Err(error) => return Auth::Invalid(error.to_string())
        }.claims;

        if key_claims.key != access_claims.key {
            return Auth::Invalid("Keys don't match".to_string());
        }

        Auth::Valid {
            info: Info {
                name: access_claims.name
            }
        }
    }
}

mod keys {
    use std::sync::Mutex;

    #[derive(Clone)]
    pub struct Object {
        pub access: String,
        pub key: String,
    }

    pub static OBJECT: Mutex<Option<Object>> = Mutex::new(None);
}

pub fn get_keys() -> keys::Object {
    match &*keys::OBJECT.lock().unwrap() {
        Some(keys) => keys.clone(),
        None => {
            dotenv::dotenv().ok();
            keys::Object {
                access: std::env::var("ACCESS_KEY").unwrap(),
                key: std::env::var("KEY_KEY").unwrap()
            }
        }
    }
}

pub fn hash_password(password: String) -> Result<String, String> {
    bcrypt::hash(password)
    .map_err(|error| error.to_string())
}

pub fn compare_password(password: String, hash: String) -> bool {
    bcrypt::verify(password.as_str(), hash.as_str())
}