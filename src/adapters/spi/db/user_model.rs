use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Selectable, AsChangeset, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub refresh_token: String,
    pub fcm_token: String,
    pub last_login: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserRegister<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: &'a str,
}

