use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccessTokenClaimsRole {
    SuperAdmin,
    Admin,
    User,
    Public,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,                 // User ID
    pub exp: usize,                  // Expiration time for access token
    pub role: AccessTokenClaimsRole, // User role
}

impl AccessTokenClaims {
    pub fn new(user_id: String, role: AccessTokenClaimsRole) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15)) // Short-lived access token (15 min)
            .expect("valid timestamp")
            .timestamp() as usize;

        AccessTokenClaims {
            sub: user_id,
            exp: expiration,
            role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time for refresh token
}

impl RefreshTokenClaims {
    pub fn new(user_id: String) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7)) // Long-lived refresh token (7 days)
            .expect("valid timestamp")
            .timestamp() as usize;

        RefreshTokenClaims { sub: user_id, exp: expiration }
    }
}

pub fn generate_access_token(user_id: String, role: AccessTokenClaimsRole, secret: &[u8]) -> Result<String> {
    let claims = AccessTokenClaims::new(user_id, role);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn generate_refresh_token(user_id: String, secret: &[u8]) -> Result<String> {
    let claims = RefreshTokenClaims::new(user_id);
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn validate_access_token(token: &str, secret: &[u8]) -> Result<AccessTokenClaims> {
    let validation = Validation::default();
    let token_data = decode::<AccessTokenClaims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}

pub fn validate_refresh_token(token: &str, secret: &[u8]) -> Result<RefreshTokenClaims> {
    let validation = Validation::default();
    let token_data = decode::<RefreshTokenClaims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}

pub fn refresh_access_token(refresh_token: &str, secret: &[u8], user_role: AccessTokenClaimsRole) -> Result<String> {
    match validate_refresh_token(refresh_token, secret) {
        Ok(claims) => {
            // Generate a new access token
            generate_access_token(claims.sub, user_role, secret)
        }
        Err(e) => Err(e),
    }
}
