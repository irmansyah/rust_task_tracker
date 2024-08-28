use async_trait::async_trait;

use crate::{
    adapters::api::dog_facts::dog_facts_payloads::DogFactPayload,
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};

pub struct PostOneDogFactUseCase<'a> {
    dog_fact_payload: &'a DogFactPayload,
    repository: &'a dyn DogFactsRepositoryAbstract,
}

impl<'a> PostOneDogFactUseCase<'a> {
    pub fn new(dog_fact_payload: &'a DogFactPayload, repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        PostOneDogFactUseCase { dog_fact_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<DogFactEntity> for PostOneDogFactUseCase<'a> {
    async fn execute(&self) -> Result<DogFactEntity, ApiError> {
        let dog_fact = self.repository.post_one_dog_fact(&self.dog_fact_payload).await;

        match dog_fact {
            Ok(dog_fact) => Ok(dog_fact),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get random dog fact", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::application::{repositories::dog_facts_repository_abstract::MockDogFactsRepositoryAbstract, usecases::post_one_dog_fact_usecase::PostOneDogFactUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        let payload = DogFactPayload::new(1, "This is text fact".to_string());
        dog_fact_repository
            .expect_get_dog_fact_by_id()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let post_one_dog_fact_usecase = PostOneDogFactUseCase::new(&payload, &dog_fact_repository);
        let data = post_one_dog_fact_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog fact by id" usecase repo returning one result
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        let payload = DogFactPayload::new(1, "This is text fact".to_string());
        dog_fact_repository.expect_post_one_dog_fact().times(1).returning(|_| {
            Ok(DogFactEntity {
                fact_id: 1,
                fact: String::from("fact1"),
            })
        });

        // when calling usecase
        let get_one_dog_fact_by_id_usecase = PostOneDogFactUseCase::new(&payload, &dog_fact_repository);
        let data = get_one_dog_fact_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_id, 1);
        assert_eq!(data.fact, "fact1");
    }
}
