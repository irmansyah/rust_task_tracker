use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub access_token: String,
    pub fcm_token: String,
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
        access_token: String,
        fcm_token: String,
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
            access_token,
            fcm_token,
            last_login,
            updated_at,
            created_at,
        }
    }
}
