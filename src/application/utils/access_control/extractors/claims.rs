use actix_web::{dev::Payload, error::ResponseError, http::StatusCode, Error, FromRequest, HttpRequest, HttpResponse};
use actix_web_httpauth::{extractors::bearer::Config, headers::www_authenticate::bearer::Bearer};
use derive_more::Display;
use jsonwebtoken::{decode, encode, jwk::AlgorithmParameters, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::application::utils::types::ErrorMessage;

// Define roles with their associated permissions
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin,
    Admin,
    Author,
    User,
}

// Permissions like read, write, delete actions for resources
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Permission {
    Read(String),   // e.g., "read:admin-messages"
    Write(String),  // e.g., "write:admin-messages"
    Delete(String), // e.g., "delete:admin-messages"
}

// JWT Claims structure to include both role and permissions
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,                              // Subject (user ID)
    role: Role,                               // User role
    permissions: Option<HashSet<Permission>>, // Optional specific permissions
    exp: usize,                               // Expiry timestamp
}

impl Claims {
    // // Check if the user has the required role and permissions
    // pub fn validate_role(&self, required_role: &Role) -> bool {
    //     &self.role == required_role
    // }
    pub fn validate_roles(&self, allowed_roles: &[Role]) -> bool {
        allowed_roles.contains(&self.role)
    }

    // Check if the user has the required permissions
    pub fn validate_permissions(&self, required_permissions: &HashSet<Permission>) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| permissions.is_superset(required_permissions))
    }
}

#[derive(Debug, Display)]
enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some("Authorization header value must follow this format: Bearer access-token".to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgortithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!("Unsupported encryption algortithm expected RSA got {:?}", alg)),
                message: "Bad credentials".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

// Generate a token with a secret key, including roles and permissions
pub fn generate_token(secret: &[u8], user_id: &str, role: Role, permissions: Option<HashSet<Permission>>) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() + 3600; // 1-hour expiration

    let claims = Claims {
        sub: user_id.to_owned(),
        role,
        permissions,
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

// Validate the token with the secret key
fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, ClientError> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map_err(ClientError::Decode)
        .map(|data| data.claims)
}

// Implement FromRequest for Claims to handle JWT extraction from the request
impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(String::from);

        Box::pin(async move {
            if let Some(token) = auth_header {
                let jwt_secret_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
                let secret: &[u8] = jwt_secret_string.as_bytes();
                match validate_token(&token, secret) {
                    Ok(claims) => Ok(claims),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Correctly create AuthenticationError using Config
                let config = Config::default();
                Err(ClientError::Authentication(actix_web_httpauth::extractors::AuthenticationError::from(config)).into())
            }
        })
    }
}

// // Example permissions and roles
// pub fn permissions_and_roles_admin() -> Result<String, jsonwebtoken::errors::Error> {
//     // Define some specific permissions
//     let mut permissions = HashSet::new();
//     permissions.insert(Permission::Read("admin-messages".to_string()));
//     permissions.insert(Permission::Read("author-messages".to_string()));
//     permissions.insert(Permission::Read("user-messages".to_string()));
//     // permissions.insert(Permission::Write("admin-messages".to_string()));

//     // Example of generating a token for an admin
//     let secret = b"your_secret_key_111";
//     let role = Role::Admin;

//     match generate_token(secret, "admin_admin", role.clone(), Some(permissions.clone())) {
//         Ok(token) => {
//             println!("Generated token for {:?}: {}", role, token);
//             Ok(token)
//         }
//         Err(e) => {
//             eprintln!("Error generating token: {:?}", e);
//             Err(e)
//         }
//     }
// }

// pub fn permissions_and_roles_user() -> Result<String, jsonwebtoken::errors::Error> {
//     // Define some specific permissions
//     let mut permissions = HashSet::new();
//     permissions.insert(Permission::Read("author-messages".to_string()));
//     permissions.insert(Permission::Read("user-messages".to_string()));
//     // permissions.insert(Permission::Write("user-messages".to_string()));

//     // Example of generating a token for an user
//     let secret = b"your_secret_key_111";
//     let role = Role::User;

//     match generate_token(secret, "user_user", role.clone(), Some(permissions.clone())) {
//         Ok(token) => {
//             println!("Generated token for {:?}: {}", role, token);
//             Ok(token)
//         }
//         Err(e) => {
//             eprintln!("Error generating token: {:?}", e);
//             Err(e)
//         }
//     }
// }
