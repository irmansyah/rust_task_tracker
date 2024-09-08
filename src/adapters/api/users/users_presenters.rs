use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
}
