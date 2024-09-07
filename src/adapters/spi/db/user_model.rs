use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserRegister<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdate<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
