use serde::{Deserialize, Serialize};

use std::{fmt, str::FromStr};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserRolePayload {
    SuperAdmin,
    Admin,
    Author,
    User,
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
    pub fn promote_by_superadmin(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::User => Some(UserRolePayload::Author),
            UserRolePayload::Author => Some(UserRolePayload::Admin),
            UserRolePayload::Admin => Some(UserRolePayload::SuperAdmin),
            UserRolePayload::SuperAdmin => None,
        }
    }

    pub fn promote(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::User => Some(UserRolePayload::Author),
            UserRolePayload::Author => None,
            UserRolePayload::Admin => None,
            UserRolePayload::SuperAdmin => None,
        }
    }

    pub fn demote_by_superadmin(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::SuperAdmin => Some(UserRolePayload::Admin),
            UserRolePayload::Admin => Some(UserRolePayload::Author),
            UserRolePayload::Author => Some(UserRolePayload::User),
            UserRolePayload::User => None, // No demotion from User
        }
    }

    pub fn demote(self) -> Option<UserRolePayload> {
        match self {
            UserRolePayload::SuperAdmin => None,
            UserRolePayload::Admin => Some(UserRolePayload::Author),
            UserRolePayload::Author => Some(UserRolePayload::User),
            UserRolePayload::User => None, // No demotion from User
        }
    }
}

impl FromStr for UserRolePayload {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "super_admin" => Ok(UserRolePayload::SuperAdmin),
            "admin" => Ok(UserRolePayload::Admin),
            "author" => Ok(UserRolePayload::Author),
            "user" => Ok(UserRolePayload::User),
            _ => Err(format!("UserRolePayload role: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserRolePromotePayload {
    Promote,
    Demote,
    None,
}

impl Default for UserRolePromotePayload {
    fn default() -> Self {
        UserRolePromotePayload::None
    }
}

impl FromStr for UserRolePromotePayload {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "promote" => Ok(UserRolePromotePayload::Promote),
            "demote" => Ok(UserRolePromotePayload::Demote),
            "none" => Ok(UserRolePromotePayload::None),
            _ => Ok(UserRolePromotePayload::None),
        }
    }
}

impl UserRolePromotePayload {
    pub fn apply_role_change(&self, data_my_role: UserRolePayload, mut data_user_role: UserRolePayload) -> UserRolePayload {
        match self {
            UserRolePromotePayload::Promote => {
                data_user_role = match data_my_role {
                    UserRolePayload::SuperAdmin => data_user_role.clone().promote_by_superadmin().unwrap_or(data_user_role),
                    UserRolePayload::Admin => data_user_role.clone().promote().unwrap_or(data_user_role),
                    _ => data_user_role, // No promotion for other roles
                };
            }
            UserRolePromotePayload::Demote => {
                // if let Some(next_role) = data_user_role.clone().demote() {
                //     data_user_role = next_role;
                // }
                data_user_role = match data_my_role {
                    UserRolePayload::SuperAdmin => data_user_role.clone().demote_by_superadmin().unwrap_or(data_user_role),
                    UserRolePayload::Admin => data_user_role.clone().demote().unwrap_or(data_user_role),
                    _ => data_user_role, // No promotion for other roles
                };
            }
            UserRolePromotePayload::None => { /* No changes */ }
        }
        data_user_role
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserIdPayload {
    pub user_id: String,
}

impl UserIdPayload {
    pub fn new(user_id: String) -> Self {
        UserIdPayload { user_id }
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
    pub fn new(username: String, email: String, password: String, role: Option<UserRolePayload>) -> Self {
        UserRegisterPayload { username, email, password, role }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginPayload {
    pub email: String,
    pub password: String,
}

impl UserLoginPayload {
    pub fn new(email: String, password: String) -> Self {
        UserLoginPayload { email, password }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserUpdatePayload {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRolePayload>,
    pub role_promote: Option<UserRolePromotePayload>,
}

impl UserUpdatePayload {
    pub fn new(
        user_id: Option<String>,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        role: Option<UserRolePayload>,
        role_promote: Option<UserRolePromotePayload>,
    ) -> Self {
        UserUpdatePayload {
            user_id,
            username,
            email,
            password,
            role,
            role_promote,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRefreshTokenPayload {
    pub refresh_token: String,
}

impl UserRefreshTokenPayload {
    pub fn new(refresh_token: String) -> Self {
        UserRefreshTokenPayload { refresh_token }
    }
}

pub struct UserPayload {}
