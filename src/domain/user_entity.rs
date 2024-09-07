#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserEntity {
    pub fn new(
        id: i32,
        username: String,
        email: String,
        password: String,
    ) -> Self {
        UserEntity {
            id,
            username,
            email,
            password,
        }
    }
}
