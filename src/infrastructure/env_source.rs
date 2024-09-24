use std::env;

pub fn get_jwt_secret() -> Vec<u8> {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set!").into_bytes()
}

// let jwt_secret_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
// let jwt_secret = jwt_secret_string.as_bytes();
