use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::user_entity::UserEntity;

use super::users_payloads::UserRegisterPayload;
use super::users_presenters::UserPresenter;

pub struct UserPresenterMapper {}

pub struct UserAllPresenterMapper {}

impl ApiMapper<UserEntity, UserPresenter, UserRegisterPayload> for UserPresenterMapper {
    fn to_api(entity: UserEntity) -> UserPresenter {
        UserPresenter {
            user_id: entity.id,
            username: entity.username,
            email: entity.email,
            password: entity.password,
        }
    }

    fn to_entity(_payload: UserRegisterPayload) -> UserEntity {
        panic!("not implemented");
    }
}
