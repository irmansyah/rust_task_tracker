use chrono::NaiveDateTime;

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
            role: entity.role,
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
        }
    }

    fn to_entity(_payload: UserRegisterPayload) -> UserEntity {
        panic!("not implemented");
    }
}

fn naive_datetime_to_unixtimemillis(datetime: NaiveDateTime) -> i64 {
    // Get the Unix timestamp in seconds and convert to milliseconds
    let millis = datetime.and_utc().timestamp_millis();
    millis
}
