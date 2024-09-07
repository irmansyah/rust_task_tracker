use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::UserPayload,
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserEntity},
};

pub struct UpdateOneUserUseCase<'a> {
    user_payload: &'a UserPayload,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> UpdateOneUserUseCase<'a> {
    pub fn new(user_payload: &'a UserPayload, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        UpdateOneUserUseCase { user_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for UpdateOneUserUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.update_one_user(&self.user_payload).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot update single user", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::application::{repositories::users_repository_abstract::MockUsersRepositoryAbstract, usecases::update_one_user_usecase::UpdateOneUserUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all user users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserPayload::new("Username 1".to_string(), "Test1@gmail.com".to_string(), "Test1234".to_string());
        user_repository
            .expect_update_one_user()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let update_one_user_usecase = UpdateOneUserUseCase::new(&payload, &user_repository);
        let data = update_one_user_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot update one user", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one user user by id" usecase repo returning one result
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserPayload::new("Username 1".to_string(), "Test1@gmail.com".to_string(), "Test1234".to_string());
        user_repository.expect_update_one_user().times(1).returning(|_| {
            Ok(UserEntity {
                id: 1,
                username: String::from("Username 1"),
                email: String::from("Test1@gmail.com"),
                password: String::from("test1234"),
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = UpdateOneUserUseCase::new(&payload, &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, 1);
        assert_eq!(data.email, "Test1@gmail.com");
    }
}