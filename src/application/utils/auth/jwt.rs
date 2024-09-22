use hmac::{Hmac, Mac};
use jsonwebtoken;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{env, fmt::Error};

use actix_web::{dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};

use jwt::VerifyWithKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub id: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub id: String,
    pub exp: usize,
}

// pub fn decode(token: String) -> Result<String, Error> {
//     let secret = env::var("JWT_SECRET").unwrap();

//     match jsonwebtoken::decode::<AccessTokenClaims>(&token, &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()), &jsonwebtoken::Validation::default()) {
//         Ok(token) => Ok(token.claims.id),
//         Err(_) => Err(Error),
//     }
// }

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<AccessTokenClaims, &str> = token_string.verify_with_key(&key).map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

// pub fn encode(id: String) -> Result<String, Error> {
//     let secret = env::var("JWT_SECRET").unwrap();
//     match jsonwebtoken::encode(
//         &jsonwebtoken::Header::default(),
//         &AccessTokenClaims {
//             id,
//             exp: chrono::Utc::now().timestamp() + 3600 * 24,
//         },
//         &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
//     ) {
//         Ok(token) => Ok(token),
//         Err(_) => Err(Error),
//     }
// }

// pub fn encode(id: String) -> Result<String, Error> {
//     // let secret = env::var("JWT_SECRET").unwrap();

//     let secret: Hmac<Sha256> = Hmac::new_from_slice(std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!").as_bytes()).unwrap();

//     // let claims = Claims { id: id, exp: todo!() };
//     let token_str = claims.sign_with_key(&secret).unwrap();
//     Ok(token_str)
// }

