use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::{UserLoginPayload, UserRegisterPayload, UserUpdatePayload},
    domain::user_entity::UserEntity,
};

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UsersRepositoryAbstract {
    async fn register_user(&self, user_payload: &UserRegisterPayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn login_user(&self, user_payload: &UserLoginPayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn update_one_user(&self, user_payload: &UserUpdatePayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<UserEntity, Box<dyn Error>>;
    async fn get_all_users(&self) -> Result<Vec<UserEntity>, Box<dyn Error>>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<UserEntity, Box<dyn Error>>;
}
