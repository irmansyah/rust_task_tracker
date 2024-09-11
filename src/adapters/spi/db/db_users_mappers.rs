use uuid::Uuid;

use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::user_entity::*;

use super::user_model::User;

pub struct UserDbMapper {}

impl DbMapper<UserEntity, User> for UserDbMapper {
    fn to_db(entity: UserEntity) -> User {
        User {
            id: Uuid::parse_str(&entity.id).unwrap_or_default(),
            username: entity.username,
            email: entity.email,
            password_hash: entity.password,
            role: entity.role,
            access_token: entity.access_token,
            fcm_token: entity.fcm_token,
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
            access_token: model.access_token,
            fcm_token: model.fcm_token,
            last_login: model.last_login,
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}
