use async_trait::async_trait;

use crate::{
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserEntity},
};

pub struct GetOneUserByIdUseCase<'a> {
    user_id: &'a String,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> GetOneUserByIdUseCase<'a> {
    pub fn new(user_id: &'a String, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        GetOneUserByIdUseCase { user_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for GetOneUserByIdUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.get_user_by_id(&self.user_id).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get single user", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use crate::{adapters::api::users::users_payloads::UserRolePayload, application::repositories::users_repository_abstract::MockUsersRepositoryAbstract, domain::user_entity::UserEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository
            .expect_get_user_by_id()
            .with(eq(1.to_string()))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&"1".to_string(), &user_repository);
        let data = get_one_user_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single user", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one user by id" usecase repo returning one result
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository.expect_get_user_by_id().with(eq(String::from('1'))).times(1).returning(|_| {
            Ok(UserEntity {
                id: String::from("userid1"),
                username: String::from("user1"),
                email: String::from("test1@gmail.com"),
                password: String::from("Test1234"),
                role: UserRolePayload::User.to_string(),
                updated_at: todo!(),
                created_at: todo!(),
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = GetOneUserByIdUseCase::new(&"userid1".to_string(), &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, String::from("userid1"));
        assert_eq!(data.username, "User 1");
    }
}
