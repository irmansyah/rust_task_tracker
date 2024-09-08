use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

// #[derive(Queryable, Identifiable, Selectable]
#[derive(Queryable, Insertable, Selectable, AsChangeset, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserLogin<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdate<'a> {
    pub username: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    pub updated_at: NaiveDateTime,
}
