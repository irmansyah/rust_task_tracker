use async_trait::async_trait;

use crate::{
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{user_entity::UserEntity, error::ApiError},
};

pub struct DeleteOneUserByIdUseCase<'a> {
    user_id: &'a i32,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> DeleteOneUserByIdUseCase<'a> {
    pub fn new(user_id: &'a i32, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        DeleteOneUserByIdUseCase { user_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for DeleteOneUserByIdUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.delete_user_by_id(*self.user_id).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot delete single user", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use crate::{application::repositories::users_repository_abstract::MockUsersRepositoryAbstract, domain::user_entity::UserEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository
            .expect_delete_user_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let delete_one_user_by_id_usecase = DeleteOneUserByIdUseCase::new(&1, &user_repository);
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
        user_repository.expect_get_user_by_id().with(eq(1)).times(1).returning(|_| {
            Ok(UserEntity {
                id: 1,
                username: String::from("user1"),
                email: String::from("test1@gmail.com"),
                password: String::from("Test1234"),
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = DeleteOneUserByIdUseCase::new(&1, &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, 1);
        assert_eq!(data.email, "test1@gmail.com");
    }
}
