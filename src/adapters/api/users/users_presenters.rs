use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub username: String,
    pub email: String,
    pub password: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserLoginPresenter {
//     pub email: String,
//     pub password: String,
// }
