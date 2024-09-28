use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_token: Option<String>,
    pub last_login: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl UserEntity {
    pub fn new(
        id: String,
        username: String,
        email: String,
        password: String,
        role: String,
        refresh_token: Option<String>,
        access_token: Option<String>,
        fcm_token: Option<String>,
        last_login: NaiveDateTime,
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    ) -> Self {
        UserEntity {
            id,
            username,
            email,
            password,
            role,
            refresh_token,
            access_token,
            fcm_token,
            last_login,
            updated_at,
            created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserAllEntity {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl UserAllEntity {
    pub fn new(
        id: String,
        username: String,
        email: String,
        role: String,
    ) -> Self {
        UserAllEntity {
            id,
            username,
            email,
            role,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserAccessTokenEntity {
    pub access_token: String,
}

impl UserAccessTokenEntity {
    pub fn new(
        access_token: String,
    ) -> Self {
        UserAccessTokenEntity {
            access_token,
        }
    }
}
