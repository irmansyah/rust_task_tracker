use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::*,
    domain::user_entity::{UserEntity, UserAllEntity, UserAccessTokenEntity},
};

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UsersRepositoryAbstract {
    async fn get_refresh(&self, user_payload: &UserRefreshTokenPayload) -> Result<UserAccessTokenEntity, Box<dyn Error>>;
    async fn register_user(&self, user_payload: &UserRegisterPayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn login_user(&self, user_payload: &UserLoginPayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn update_one_user(&self, user_payload: &UserUpdatePayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn get_all_users(&self) -> Result<Vec<UserAllEntity>, Box<dyn Error>>;
    async fn get_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>>;
    async fn delete_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>>;
}
