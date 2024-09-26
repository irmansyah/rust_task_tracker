use uuid::Uuid;

use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::user_entity::*;

use super::user_model::User;

pub struct UserDbMapper {}

pub struct UserAllDbMapper {}

pub struct UserAccessTokenDbMapper {}

impl DbMapper<UserEntity, User> for UserDbMapper {
    fn to_db(entity: UserEntity) -> User {
        User {
            id: Uuid::parse_str(&entity.id).unwrap_or_default(),
            username: entity.username,
            email: entity.email,
            password_hash: entity.password,
            role: entity.role,
            // refresh_token: entity.access_token,
            fcm_token: entity.fcm_token,
            refresh_token: todo!(),
            last_login: todo!(),
            updated_at: todo!(),
            created_at: todo!(),
        }
    }

    fn to_entity(model: User) -> UserEntity {
        UserEntity {
            id: model.id.to_string(),
            username: model.username,
            email: model.email,
            password: model.password_hash,
            role: model.role,
            refresh_token: Some(model.refresh_token),
            fcm_token: model.fcm_token,
            last_login: model.last_login,
            updated_at: model.updated_at,
            created_at: model.created_at,
            access_token: Some("".to_string()),
        }
    }
}

impl DbMapper<UserAllEntity, User> for UserAllDbMapper {
    fn to_db(entity: UserAllEntity) -> User {
        User {
            id: Uuid::parse_str(&entity.id).unwrap_or_default(),
            username: entity.username,
            email: entity.email,
            role: entity.role,
            password_hash: todo!(),
            refresh_token: todo!(),
            fcm_token: todo!(),
            last_login: todo!(),
            updated_at: todo!(),
            created_at: todo!(),
        }
    }

    fn to_entity(model: User) -> UserAllEntity {
        UserAllEntity {
            id: model.id.to_string(),
            username: model.username,
            email: model.email,
            role: model.role,
        }
    }
}

impl DbMapper<UserAccessTokenEntity, String> for UserAccessTokenDbMapper {
    fn to_db(entity: UserAccessTokenEntity) -> String {
        entity.access_token.to_string()
    }

    fn to_entity(access_token: String) -> UserAccessTokenEntity {
        UserAccessTokenEntity {
            access_token: access_token.to_string(),
        }
    }
}
