use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterPayload {
    // implement for POST/UPDATE requests
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserRegisterPayload {
    pub fn new(
        username: String, 
        email: String, 
        password: String, 
    ) -> Self {
        UserRegisterPayload {
            username,
            email,
            password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginPayload {
    // implement for POST/UPDATE requests
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
    // implement for POST/UPDATE requests
    pub user_id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UserUpdatePayload {
    pub fn new(
        user_id: i32, 
        username: Option<String>, 
        email: Option<String>, 
        password: Option<String>, 
    ) -> Self {
        UserUpdatePayload {
            user_id,
            username,
            email,
            password,
        }
    }
}
