use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserRolePayload {
    SuperAdmin,
    Admin,
    Author,
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
            UserRolePayload::Author => write!(f, "author"),
            UserRolePayload::User => write!(f, "user"),
        }
    }
}
impl UserRolePayload {
    pub fn next(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::User => Some(UserRolePayload::Author),
            UserRolePayload::Author => Some(UserRolePayload::Author),
            UserRolePayload::Admin => Some(UserRolePayload::Admin),
            UserRolePayload::SuperAdmin => None,  // No further promotion
        }
    }

    pub fn previous(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::SuperAdmin => Some(UserRolePayload::Admin),
            UserRolePayload::Admin => Some(UserRolePayload::Admin),
            UserRolePayload::Author => Some(UserRolePayload::User),
            UserRolePayload::User => None,  // No demotion from User
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserIdPayload {
    pub user_id: String,
}

impl UserIdPayload {
    pub fn new(
        user_id: String, 
    ) -> Self {
        UserIdPayload {
            user_id,
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRefreshTokenPayload {
    pub refresh_token: String,
}

impl UserRefreshTokenPayload {
    pub fn new(
        refresh_token: String, 
    ) -> Self {
        UserRefreshTokenPayload {
            refresh_token,
        }
    }
}

pub struct UserPayload {}
