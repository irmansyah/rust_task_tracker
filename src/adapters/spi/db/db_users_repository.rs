use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

use crate::adapters::api::users::users_payloads::{UserLoginPayload, UserPayload, UserRegisterPayload};
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::UserDbMapper, schema::users::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::users_repository_abstract::UsersRepositoryAbstract};
use crate::domain::user_entity::UserEntity;

use crate::adapters::spi::db::schema::users;

use super::user_model::{RegisterNewUser, User};

pub struct UsersRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl UsersRepositoryAbstract for UsersRepository {
    async fn register_user(&self, user_payload: &UserRegisterPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_username = user_payload.username.clone().unwrap_or_default();
        let data_typ = user_payload.typ.clone().unwrap_or_default().to_string();
        let data_priority = user_payload.priority.clone().unwrap_or_default().to_string();
        let data_status = user_payload.status.clone().unwrap_or_default().to_string();

        let new_user = NewUser {
            title: Some(&data_title),
            typ: Some(&data_typ),
            priority: Some(&data_priority),
            status: Some(&data_status),
            description: Some(&data_description),
            duration: Some(data_duration),
            due_date: Some(data_due_date),
            project_id: Some(data_project_id),
            user_list: data_user_list,
        };

        let result = diesel::insert_into(users::table).values(&new_user).returning(User::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn login_user(&self, user_payload: &UserLoginPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_username = user_payload.username.clone().unwrap_or_default();
        let data_email = user_payload.email.clone().unwrap_or_default().to_string();
        let data_password = user_payload.password.clone().unwrap_or_default().to_string();

        let new_user = NewUser {
            title: Some(&data_title),
            typ: Some(&data_typ),
            priority: Some(&data_priority),
            status: Some(&data_status),
            description: Some(&data_description),
            duration: Some(data_duration),
            due_date: Some(data_due_date),
            project_id: Some(data_project_id),
            user_list: data_user_list,
        };

        let result = diesel::insert_into(users::table).values(&new_user).returning(User::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_user(&self, user_payload: &UserPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let data_user_list: Option<Vec<&str>> = user_payload.user_list.as_ref().map(|vec| vec.iter().map(|s| s.as_str()).collect());
        let target = users.filter(id.eq(user_payload.user_id));
        let result = diesel::update(target)
            .set((
                user_payload.title.clone().map(|data| title.eq(data)),
                user_payload.typ.clone().map(|data| typ.eq(data.to_string())),
                user_payload.priority.clone().map(|data| priority.eq(data.to_string())),
                user_payload.status.clone().map(|data| priority.eq(data.to_string())),
                user_payload.description.clone().map(|data| description.eq(data)),
                user_payload.duration.map(|data| duration.eq(data)),
                user_payload.due_date.map(|data| due_date.eq(data)),
                user_payload.project_id.map(|data| project_id.eq(data)),
                data_user_list.map(|data| user_list.eq(data)),
            ))
            .returning(User::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = users.filter(id.eq(user_id)).get_result::<User>(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
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
