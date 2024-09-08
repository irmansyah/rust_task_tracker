use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
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
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    ) -> Self {
        UserEntity {
            id,
            username,
            email,
            password,
            role,
            updated_at,
            created_at,
        }
    }
}
