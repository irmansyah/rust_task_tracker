use async_trait::async_trait;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashSet;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::api::users::users_payloads::*;
use crate::application::mappers::db_mapper::DbMapper;
use crate::application::utils::access_control::extractors::claims::{generate_token, Claims, Permission, Role};
use crate::application::utils::validate_params;
use crate::{application::repositories::users_repository_abstract::UsersRepositoryAbstract, domain::user_entity::UserEntity};

use super::db_users_mappers::UserDbMapper;
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
        // let data_access_token = match jwt::encode(user.id.to_string()) {
        //     Ok(token) => Ok(token),
        //     Err(_) => Err(String::from("Error generating token!")),
        // };
        //
        // let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!").as_bytes()).unwrap();
        // let claims = AccessTokenClaims {
        //     id: user.id.to_string(),
        //     exp: chrono::Utc::now().timestamp() + 3600 * 24,
        // };
        // let data_access_token = claims.sign_with_key(&jwt_secret).unwrap_or_default();

        let jwt_secret_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
        let jwt_secret = jwt_secret_string.as_bytes();

        // Define some specific permissions
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read("admin-messages".to_string()));
        permissions.insert(Permission::Read("author-messages".to_string()));
        permissions.insert(Permission::Read("user-messages".to_string()));

        let data_role = Role::Admin;

        let data_access_token = generate_token(jwt_secret, &user.id.to_string(), data_role.clone(), Some(permissions.clone())).unwrap_or_default();

        let target = users.filter(id.eq(user.id));
        let result = diesel::update(target)
            .set((access_token.eq(data_access_token), last_login.eq(Utc::now().naive_utc().clone())))
            .returning(User::as_returning())
            .get_result(&mut conn);

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
                // user_payload.password.clone().map(|data| password_hash.eq(data.to_string())),
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

    async fn get_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id).unwrap();
        let result = users.filter(id.eq(user_id)).get_result::<User>(&mut conn);

        match result {
            Ok(model) => Ok(UserDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_user_by_id(&self, user_payload: &UserIdPayload) -> Result<UserEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id = Uuid::parse_str(&user_payload.user_id).unwrap_or_default();
        // let user_id = match decode(user_payload.user_id) {
        //     Ok(data_id) => data_id,
        //     Err(_) => Err(String::from("Invalid token!")),
        // };
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
