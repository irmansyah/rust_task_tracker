use async_trait::async_trait;

use crate::{
    adapters::api::users::users_payloads::UserRegisterPayload,
    application::{repositories::users_repository_abstract::UsersRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, user_entity::UserEntity},
};

pub struct RegisterUserUseCase<'a> {
    user_payload: &'a UserRegisterPayload,
    repository: &'a dyn UsersRepositoryAbstract,
}

impl<'a> RegisterUserUseCase<'a> {
    pub fn new(user_payload: &'a UserRegisterPayload, repository: &'a dyn UsersRepositoryAbstract) -> Self {
        RegisterUserUseCase { user_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for RegisterUserUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.register_user(&self.user_payload).await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot post user", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{adapters::api::users::users_payloads::UserRolePayload, application::repositories::users_repository_abstract::MockUsersRepositoryAbstract};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all user users" usecase repo with an unexpected random error
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserRegisterPayload::new(String::from("user1"), String::from("test@gmail.com"), String::from("test1234"), Some(UserRolePayload::User));
        user_repository
            .expect_register_user()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let post_one_user_usecase = RegisterUserUseCase::new(&payload, &user_repository);
        let data = post_one_user_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single user", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one user user by id" usecase repo returning one result
        let mut user_repository = MockUsersRepositoryAbstract::new();
        let payload = UserRegisterPayload::new(String::from("User1"), String::from("user1@gmail.com"), String::from("test1234"), Some(UserRolePayload::User));

        // 1725862140
        user_repository.expect_register_user().times(1).returning(|_| {
            Ok(UserEntity {
                id: String::from("id1"),
                username: String::from("Username 1"),
                email: String::from("Test1@gmail.com"),
                password: String::from("test1234"),
                role: UserRolePayload::User.to_string(),
                updated_at: todo!(), 
                created_at: todo!()
            })
        });

        // when calling usecase
        let get_one_user_by_id_usecase = RegisterUserUseCase::new(&payload, &user_repository);
        let data = get_one_user_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, String::from("id1"));
        assert_eq!(data.username, "User 1");
    }
}
