use chrono::NaiveDateTime;

use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::user_entity::{UserAllEntity, UserEntity};

use super::users_payloads::{UserPayload, UserRegisterPayload};
use super::users_presenters::{UserAllPresenter, UserPresenter};

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
            access_token: entity.access_token,
            fcm_token: entity.fcm_token,
            last_login: naive_datetime_to_unixtimemillis(entity.last_login),
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
        }
    }

    fn to_entity(_payload: UserRegisterPayload) -> UserEntity {
        panic!("not implemented");
    }
}

impl ApiMapper<UserAllEntity, UserAllPresenter, UserPayload> for UserAllPresenterMapper {
    fn to_api(entity: UserAllEntity) -> UserAllPresenter {
        UserAllPresenter {
            user_id: entity.id,
            username: entity.username,
            email: entity.email,
            role: entity.role,
        }
    }

    fn to_entity(_payload: UserPayload) -> UserAllEntity {
        panic!("not implemented");
    }
}


fn naive_datetime_to_unixtimemillis(datetime: NaiveDateTime) -> i64 {
    // Get the Unix timestamp in seconds and convert to milliseconds
    let millis = datetime.and_utc().timestamp_millis();
    millis
}
