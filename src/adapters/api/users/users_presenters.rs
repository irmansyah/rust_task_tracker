use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
