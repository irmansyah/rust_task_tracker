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
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}
