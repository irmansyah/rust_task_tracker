use async_trait::async_trait;

use crate::{
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserEntity},
};

pub struct GetAllUsersUseCase<'a> {
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> GetAllUsersUseCase<'a> {
    pub fn new(repository: &'a dyn UsersRepositoryAbstract) -> Self {
        GetAllUsersUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<UserEntity>> for GetAllUsersUseCase<'a> {
    async fn execute(&self) -> Result<Vec<UserEntity>, ApiError> {
        let users = self.repository.get_all_users().await;

        match users {
            Ok(users) => Ok(users),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all users", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{application::repositories::users_repository_abstract::MockUsersRepositoryAbstract, domain::user_entity::UserEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository
            .expect_get_all_users()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_users_usecase = GetAllUsersUseCase::new(&user_repository);
        let data = get_all_users_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog users", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog users" usecase repo returning an empty list
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository.expect_get_all_users().with().times(1).returning(|| Ok(Vec::<UserEntity>::new()));

        // when calling usecase
        let get_all_users_usecase = GetAllUsersUseCase::new(&user_repository);
        let data = get_all_users_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog users" usecase repo returning a list of 2 entities
        let mut user_repository = MockUsersRepositoryAbstract::new();
        user_repository.expect_get_all_users().with().times(1).returning(|| {
            Ok(vec![
                UserEntity {
                    id: 1,
                    username: String::from("user1"),
                    email: String::from("test1@gmail.com"),
                    password: String::from("Test1234"),
                },
                UserEntity {
                    id: 2,
                    username: String::from("user2"),
                    email: String::from("test2@gmail.com"),
                    password: String::from("Test1234"),
                },
            ])
        });

        // when calling usecase
        let get_all_users_usecase = GetAllUsersUseCase::new(&user_repository);
        let data = get_all_users_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}