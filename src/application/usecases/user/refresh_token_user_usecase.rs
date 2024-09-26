use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::UserRefreshTokenPayload,
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserAccessTokenEntity},
};

pub struct RefreshTokenUserUseCase<'a> {
    user_payload: &'a UserRefreshTokenPayload,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> RefreshTokenUserUseCase<'a> {
    pub fn new(user_payload: &'a UserRefreshTokenPayload, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        RefreshTokenUserUseCase { user_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserAccessTokenEntity> for RefreshTokenUserUseCase<'a> {
    async fn execute(&self) -> Result<UserAccessTokenEntity, ApiError> {
        let token = self.repository.get_refresh(&self.user_payload).await;

        match token {
            Ok(token) => Ok(token),
            Err(e) => Err(ErrorHandlingUtils::application_error(&e.to_string(), Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{adapters::api::users::users_payloads::UserRefreshTokenPayload, application::repositories::users_repository_abstract::MockUsersRepositoryAbstract};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all user users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserRefreshTokenPayload::new(String::from("thisistoken123"));
        user_repository
            .expect_get_refresh()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let refresh_token_user_usecase = RefreshTokenUserUseCase::new(&payload, &user_repository);
        let data = refresh_token_user_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single user", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one user user by id" usecase repo returning one result
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserRefreshTokenPayload::new(String::from("thisisrefreshtoken123"));
        user_repository.expect_get_refresh().times(1).returning(|_| {
            Ok(UserAccessTokenEntity {
                access_token: String::from("thisisaccesstoken123"),
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = RefreshTokenUserUseCase::new(&payload, &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.access_token, String::from("thisisaccesstoken123"));
    }
}
