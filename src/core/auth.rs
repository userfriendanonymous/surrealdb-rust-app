use serde::{Serialize, Deserialize};
use rand::Rng;
use jsonwebtoken::{DecodingKey, Validation, Algorithm, EncodingKey};
use pwhash::bcrypt;

const NAME_CHARS: &str = "qazwsxedcrfvtgbyhnujmikolpQAZWSXEDCRFVTGBYHNUJMIKOLP1234567890_";

#[derive(Default)]
pub struct Tokens {
    pub access: String,
    pub key: String
}

impl TryFrom<Info> for Tokens {
    type Error = String;
    fn try_from(info: Info) -> Result<Self, Self::Error> {
        let keys = get_keys();

        let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

        let mut rng = rand::thread_rng();
        let key: String = (0..20).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
        let header = jsonwebtoken::Header::new(Algorithm::HS512);

        let access_claims = AccessClaims {
            exp,
            name: info.name,
            key: key.clone()
        };
        let key_claims = KeyClaims {
            exp,
            key,
        };
        let access_token = jsonwebtoken::encode(&header, &access_claims, &EncodingKey::from_secret(keys.access.as_bytes()))
        .map_err(|error| error.to_string())?;
        let key_token = jsonwebtoken::encode(&header, &key_claims, &EncodingKey::from_secret(keys.key.as_bytes()))
        .map_err(|error| error.to_string())?;

        Ok(Tokens {
            access: access_token,
            key: key_token
        })
    }
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

fn get_keys() -> keys::Object {
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

pub fn hash_password(password: &str) -> Result<String, String> {
    bcrypt::hash(password)
    .map_err(|error| error.to_string())
}

pub fn compare_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}

pub fn validate_credentials_data(name: &str, email: &str, password: &str) -> Result<(), String> {
    for char in name.chars() {
        if !NAME_CHARS.contains(char) {
            return Err("Invalid characters in username".to_owned())
        }
    }

    if name.len() < 3 || name.len() > 20 {
        return Err("Username length too short or too large".to_owned())
    }

    if password.len() < 7 || password.len() > 50 {
        return Err("Password length too short or too large".to_owned())
    }

    Ok(())
}