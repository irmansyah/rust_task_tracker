use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub access_token: String,
    pub fcm_token: String,
    pub last_login: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAllPresenter {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}
