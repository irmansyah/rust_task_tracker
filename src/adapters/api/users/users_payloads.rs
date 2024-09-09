use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserRolePayload {
    SuperAdmin,
    Admin,
    User
}

impl Default for UserRolePayload {
    fn default() -> Self {
        UserRolePayload::User
    }
}

impl fmt::Display for UserRolePayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRolePayload::SuperAdmin => write!(f, "super_admin"),
            UserRolePayload::Admin => write!(f, "admin"),
            UserRolePayload::User => write!(f, "user"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRolePayload>,
}

impl UserRegisterPayload {
    pub fn new(
        username: String, 
        email: String, 
        password: String, 
        role: Option<UserRolePayload>, 
    ) -> Self {
        UserRegisterPayload {
            username,
            email,
            password,
            role,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginPayload {
    pub email: String,
    pub password: String,
}

impl UserLoginPayload {
    pub fn new(
        email: String, 
        password: String, 
    ) -> Self {
        UserLoginPayload {
            email,
            password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdatePayload {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRolePayload>,
    // pub updated_at: NaiveDateTime,
}

impl UserUpdatePayload {
    pub fn new(
        user_id: String, 
        username: Option<String>, 
        email: Option<String>, 
        password: Option<String>, 
        role: Option<UserRolePayload>, 
    ) -> Self {
        UserUpdatePayload {
            user_id,
            username,
            email,
            password,
            role,
            // updated_at: Utc::now().naive_utc(),
        }
    }
}
