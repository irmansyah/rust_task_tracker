use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::Config;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{adapters::api::shared::error_presenter::ErrorResponse, infrastructure::env_source};

use super::extractors::claims::{Claims, ClientError, Permission, Role};

pub struct AuthUseCase;
pub struct AuthCheckUseCase;

impl AuthUseCase {
    pub fn check_role(role: &str) -> HashSet<Permission> {
        let mut permissions = AuthUseCase::no();

        match Role::from_str(role) {
            Ok(Role::SuperAdmin) => {
                permissions = AuthUseCase::all_read_down_to_superadmin();
            }
            Ok(Role::Admin) => {
                permissions = AuthUseCase::all_read_down_to_admin();
            }
            Ok(Role::Author) => {
                permissions = AuthUseCase::all_read_down_to_author();
            }
            Ok(Role::User) => {
                permissions = AuthUseCase::all_read_down_to_user();
            }
            Err(_) => {
                println!("Invalid role: {}", role);
            }
        }

        permissions
    }

    pub fn all_read_down_to_superadmin() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read("superadmin-tasks".to_string()));
        permissions.insert(Permission::Read("admin-tasks".to_string()));
        permissions.insert(Permission::Read("author-tasks".to_string()));
        permissions.insert(Permission::Read("user-tasks".to_string()));
        permissions
    }

    pub fn all_read_down_to_admin() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read("admin-tasks".to_string()));
        permissions.insert(Permission::Read("author-tasks".to_string()));
        permissions.insert(Permission::Read("user-tasks".to_string()));
        permissions
    }

    pub fn all_read_down_to_author() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read("author-tasks".to_string()));
        permissions.insert(Permission::Read("user-tasks".to_string()));
        permissions
    }

    pub fn all_read_down_to_user() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read("user-tasks".to_string()));
        permissions
    }

    // pub fn all_read_up_to_admin() -> HashSet<Permission> {
    //     let mut permissions = HashSet::new();
    //     permissions.insert(Permission::Read("superadmin-tasks".to_string()));
    //     permissions.insert(Permission::Read("admin-tasks".to_string()));
    //     permissions
    // }

    pub fn no() -> HashSet<Permission> {
        let permissions = HashSet::new();
        permissions
    }

    // Generate a token with a secret key, including roles and permissions
    pub fn generate_token(user_id: &str, role_str: &str, expiration: usize, permissions: Option<HashSet<Permission>>) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env_source::get_jwt_secret();
        let expiration_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(expiration as u64); // 1-hour expiration

        let role = match Role::from_str(role_str) {
            Ok(it) => it,
            Err(_) => Role::User,
        };
        let claims = Claims {
            sub: user_id.to_owned(),
            role,
            permissions,
            exp: expiration_time.as_secs() as usize,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(&secret))
    }

    // Validate the token with the secret key
    pub fn validate_token(token: &str) -> Result<Claims, ClientError> {
        let secret = env_source::get_jwt_secret();
        let validation = Validation::new(Algorithm::HS256);
        decode::<Claims>(token, &DecodingKey::from_secret(&secret), &validation)
            .map_err(ClientError::Decode)
            .map(|data| data.claims)
    }
}

impl AuthCheckUseCase {
    pub fn check_permission_up_to_user(claims: Claims) -> Result<(), ErrorResponse> {
        let allowed_roles = vec![Role::SuperAdmin, Role::Admin, Role::Author, Role::User];
        if !claims.validate_roles(&allowed_roles) {
            return Err(ErrorResponse::default());
        }
        println!("Role validated: SuperAdmin, Admin, Author, User");

        if !claims.validate_permissions(&AuthUseCase::all_read_down_to_user()) {
            return Err(ErrorResponse::default());
        }

        println!("Role validated: Success");
        Ok(())
    }

    pub fn check_permission_up_to_author(claims: Claims) -> Result<(), ErrorResponse> {
        let allowed_roles = vec![Role::SuperAdmin, Role::Admin, Role::Author];
        if !claims.validate_roles(&allowed_roles) {
            return Err(ErrorResponse::default());
        }
        println!("Role validated: Admin, Author, User");

        if !claims.validate_permissions(&AuthUseCase::all_read_down_to_author()) {
            return Err(ErrorResponse::default());
        }

        println!("Role validated: Success");
        Ok(())
    }

    pub fn check_permission_up_to_admin(claims: Claims) -> Result<(), ErrorResponse> {
        let allowed_roles = vec![Role::SuperAdmin, Role::Admin];
        if !claims.validate_roles(&allowed_roles) {
            return Err(ErrorResponse::default());
        }
        println!("Role validated: Author, User");

        if !claims.validate_permissions(&AuthUseCase::all_read_down_to_admin()) {
            return Err(ErrorResponse::default());
        }

        println!("Role validated: Success");
        Ok(())
    }

    pub fn check_permission_up_to_superadmin(claims: Claims) -> Result<(), ErrorResponse> {
        let allowed_roles = vec![Role::SuperAdmin];
        if !claims.validate_roles(&allowed_roles) {
            return Err(ErrorResponse::default());
        }
        println!("Role validated: User");

        if !claims.validate_permissions(&AuthUseCase::all_read_down_to_superadmin()) {
            return Err(ErrorResponse::default());
        }

        println!("Role validated: Success");
        Ok(())
    }
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
                match AuthUseCase::validate_token(&token) {
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
