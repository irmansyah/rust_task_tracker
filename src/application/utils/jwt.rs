use jsonwebtoken;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Error};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: i64,
}

pub fn decode(token: String) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").unwrap();

    match jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    ) {
        Ok(token) => Ok(token.claims.id),
        Err(_) => Err(Error),
    }
}

pub fn encode(id: String) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").unwrap();
    
    match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims { id, exp: chrono::Utc::now().timestamp() + 3600 * 24 },
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(Error),
    }
}
