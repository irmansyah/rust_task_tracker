use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::UserIdPayload,
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserEntity},
};

pub struct DeleteOneUserByIdUseCase<'a> {
    user_payload: &'a UserIdPayload,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> DeleteOneUserByIdUseCase<'a> {
    pub fn new(user_payload: &'a UserIdPayload, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        DeleteOneUserByIdUseCase { user_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for DeleteOneUserByIdUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.delete_user_by_id(&self.user_payload).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot delete single user", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{adapters::api::users::users_payloads::UserRolePayload, application::repositories::users_repository_abstract::MockUsersRepositoryAbstract, domain::user_entity::UserEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserIdPayload::new(String::from("id1"));
        user_repository
            .expect_update_one_user()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let delete_one_user_by_id_usecase = DeleteOneUserByIdUseCase::new(&payload, &user_repository);
        let data = delete_one_user_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single user", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog user by id" usecase repo returning one result
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserIdPayload::new(String::from("id1"));
        user_repository.expect_get_user_by_id().times(1).returning(|_| {
            Ok(UserEntity {
                id: String::from("id1"),
                username: String::from("User 1"),
                email: String::from("test1@gmail.com"),
                password: String::from("Test1234"),
                role: UserRolePayload::User.to_string(),
                access_token: String::from("thisisaccesstoken123"),
                fcm_token: String::from("thisisfcmtoken123"),
                last_login: todo!(),
                updated_at: todo!(),
                created_at: todo!(),
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = DeleteOneUserByIdUseCase::new(&payload, &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, String::from("id1"));
        assert_eq!(data.email, "test1@gmail.com");
    }
}
