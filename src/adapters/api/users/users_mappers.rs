use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::user_entity::UserEntity;

use super::users_payloads::UserRegisterPayload;
use super::users_presenters::UserPresenter;

pub struct UserPresenterMapper {}

impl ApiMapper<UserEntity, UserPresenter, UserRegisterPayload> for UserPresenterMapper {
    fn to_api(entity: UserEntity) -> UserPresenter {
        UserPresenter {
            username: entity.username,
            email: entity.email,
            password: entity.password,
        }
    }

    fn to_entity(_payload: UserRegisterPayload) -> UserEntity {
        panic!("not implemented");
    }
}
