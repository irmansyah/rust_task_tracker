use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::users::users_payloads::*;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::UserDbMapper, schema::users::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::users_repository_abstract::UsersRepositoryAbstract};
use crate::domain::user_entity::UserEntity;

use crate::adapters::spi::db::schema::users;

use super::user_model::*;

pub struct UsersRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl UsersRepositoryAbstract for UsersRepository {
    async fn register_user(&self, user_payload: &UserRegisterPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_username = user_payload.username.clone();
        let data_email = user_payload.email.clone();
        let data_password = user_payload.password.clone();
        let data_role = user_payload.role.clone().unwrap_or_default().to_string();

        let new_user = UserRegister {
            username: &data_username,
            email: &data_email,
            password_hash: &data_password,
            role: &data_role,
        };

        let result = diesel::insert_into(users::table).values(&new_user).returning(User::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn login_user(&self, user_payload: &UserLoginPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_email = user_payload.email.clone();
        let data_password = user_payload.password.clone();

        let user = UserLogin {
            email: &data_email,
            password_hash: &data_password,
        };

        let result = diesel::insert_into(users::table).values(&user).returning(User::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_user(&self, user_payload: &UserUpdatePayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id).unwrap();
        let target = users.filter(id.eq(user_id));

        let result = diesel::update(target)
            .set((
                user_payload.username.clone().map(|data| username.eq(data.to_string())),
                user_payload.password.clone().map(|data| password_hash.eq(data.to_string())),
                user_payload.role.clone().map(|data| role.eq(data.to_string())),
                updated_at.eq(Utc::now().naive_utc().clone()),
            ))
            .returning(User::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_id).unwrap();
        let result = users.filter(id.eq(user_id)).get_result::<User>(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_user_by_id(&self, user_id: &String) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_id).unwrap();
        let target_user = users::table.filter(users::id.eq(user_id));
        let result = diesel::delete(target_user).get_result::<User>(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<UserEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = users.load::<User>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(UserDbMapper::to_entity).collect::<Vec<UserEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
