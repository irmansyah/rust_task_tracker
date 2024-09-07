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
pub struct UserPayload {
    // implement for POST/UPDATE requests
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserPayload {
    pub fn new(
        username: String, 
        email: String, 
        password: String, 
    ) -> Self {
        UserPayload {
            username,
            email,
            password,
        }
    }
}
