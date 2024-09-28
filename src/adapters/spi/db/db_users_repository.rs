use async_trait::async_trait;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::shared::error_presenter::ErrorResponse;
use crate::adapters::api::users::users_payloads::*;
use crate::application::mappers::db_mapper::DbMapper;
use crate::application::utils::access_control::auth_usecase::AuthUseCase;
use crate::application::utils::validate_params;
use crate::{application::repositories::users_repository_abstract::UsersRepositoryAbstract, domain::user_entity::{UserEntity, UserAllEntity, UserAccessTokenEntity}};

use super::db_users_mappers::{UserAccessTokenDbMapper, UserAllDbMapper, UserDbMapper};
use super::schema::users::{self, *};
use super::user_model::*;
use crate::adapters::spi::db::{db_connection::DbConnection, schema::users::dsl::*};

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

        let valid_email = validate_params::is_email(&data_email);
        let valid_password = validate_params::is_password(&data_password);

        if !valid_email || !valid_password {
            return Err(String::from("Invalid Email or Password!").into());
        }

        let email_already_exists = users::table.filter(users::email.eq(&data_email)).select(users::id).first::<Uuid>(&mut conn).is_ok();

        if email_already_exists {
            return Err(String::from("Invalid email or password!").into());
        }

        let hashed_password = hash(data_password.as_str(), DEFAULT_COST).unwrap();

        let new_user = UserRegister {
            username: &data_username,
            email: &data_email,
            password_hash: &hashed_password,
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

        let user = match users::table.filter(users::email.eq(&data_email)).select(User::as_select()).first::<User>(&mut conn) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Invalid email or password!").into()),
        };

        if user.email != data_email {
            return Err(String::from("Invalid email or password!").into());
        };

        if !bcrypt::verify(data_password, &user.password_hash).unwrap() {
            return Err(String::from("Invalid email or password!").into());
        }
        let permissions = AuthUseCase::check_role(&user.role);
        let data_refresh_token = AuthUseCase::generate_token(&user.id.to_string(), &user.role, 3600 * 24, Some(permissions.clone())).unwrap_or_default();
        let data_access_token = AuthUseCase::generate_token(&user.id.to_string(), &user.role, 3600, Some(permissions.clone())).unwrap_or_default();

        let target = users.filter(id.eq(user.id));
        let result = diesel::update(target)
            .set((last_login.eq(Utc::now().naive_utc().clone()), refresh_token.eq(data_refresh_token.clone())))
            .returning(User::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) => {
                let mut entity = UserDbMapper::to_entity(model);
                entity.access_token = Some(data_access_token);
                Ok(entity)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_refresh(&self, user_payload: &UserRefreshTokenPayload) -> Result<UserAccessTokenEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let claims = match AuthUseCase::validate_token(&user_payload.refresh_token) {
            Ok(data) => data,
            Err(_) => return Err(Box::new(ErrorResponse::default())), // Box the ClientError
        };

        let user_id = Uuid::parse_str(&claims.sub)?;
        let target_user = users::table.filter(id.eq(user_id)).filter(refresh_token.eq(&user_payload.refresh_token));

        let user = match target_user.select(User::as_select()).first::<User>(&mut conn) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Failed refresh token").into()),
        };

        let permissions = AuthUseCase::check_role(&user.role);
        let new_refresh_token = AuthUseCase::generate_token(&user.id.to_string(), &user.role, 3600 * 24, Some(permissions.clone())).unwrap_or_default();

        let result = diesel::update(target_user)
            .set(refresh_token.eq(new_refresh_token))
            .returning(refresh_token)
            .get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserAccessTokenDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_user(&self, user_payload: &UserUpdatePayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id.clone().unwrap_or_default()).unwrap_or_default();
        let target = users::table.filter(id.eq(user_id));

        let user = match target.select(User::as_select()).first::<User>(&mut conn) {
            Ok(user) => user,
            Err(_) => return Err(String::from("Failed refresh token").into()),
        };

        let data_my_role = user_payload.role.clone().unwrap_or_default();
        let data_user_role = UserRolePayload::from_str(&user.role).clone().unwrap_or_default();
        let promote_role = &user_payload.role_promote.clone().unwrap_or_default();

        let result = diesel::update(target)
            .set((
                role.eq(promote_role.apply_role_change(data_my_role, data_user_role).to_string()),
                user_payload.username.clone().map(|data| username.eq(data.to_string())),
                user_payload.email.clone().map(|data| email.eq(data.to_string())),
                user_payload.password.clone().map(|data| password_hash.eq(data.to_string())),
                updated_at.eq(Utc::now().naive_utc().clone()),
            ))
            .returning(User::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id)?;
        let result = users.filter(id.eq(user_id)).get_result::<User>(&mut conn);

        match result {
            // Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Ok(model) => {
                let mut entity = UserDbMapper::to_entity(model);
                entity.refresh_token = None;
                entity.access_token = None;
                entity.fcm_token = None;
                Ok(entity)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id)?;
        let target_user = users::table.filter(users::id.eq(user_id));
        let result = diesel::delete(target_user).get_result::<User>(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<UserAllEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = users.load::<User>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(UserAllDbMapper::to_entity).collect::<Vec<UserAllEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
