use async_trait::async_trait;

use crate::{
    adapters::api::dog_facts::dog_facts_payloads::DogFactPayload,
    applidogion::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, usecases::interfaces::AbstractPayloadUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};

pub struct PostOneDogFactUseCase<'a> {
    repository: &'a dyn DogFactsRepositoryAbstract,
}

impl<'a> PostOneDogFactUseCase<'a> {
    pub fn new(repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        PostOneDogFactUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractPayloadUseCase<DogFactPayload, DogFactEntity> for PostOneDogFactUseCase<'a> {
    async fn execute(&self, dog_fact_payload: DogFactPayload) -> Result<DogFactEntity, ApiError> {
        let dog_fact = self.repository.post_one_dog_fact(&dog_fact_payload).await;

        match dog_fact {
            Ok(fact) => Ok(fact),
            Err(e) => Err(ErrorHandlingUtils::applidogion_error("Cannot get random dog fact", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::applidogion::{repositories::dog_facts_repository_abstract::MockDogFactsRepositoryAbstract, usecases::post_one_dog_fact_usecase::PostOneDogFactUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected error
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        dog_fact_repository
            .expect_get_random_dog_fact()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let post_one_dog_fact_usecase = PostOneDogFactUseCase::new(&dog_fact_repository);
        let new_dog_fact = DogFactPayload::new("1".to_string(), 1);
        let data = post_one_dog_fact_usecase.execute(new_dog_fact).await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get random dog fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one random dog fact" usecase repo returning one result
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        dog_fact_repository.expect_get_random_dog_fact().with().times(1).returning(|| {
            Ok(DogFactEntity {
                fact_txt: String::from("fact1"),
                fact_length: 1,
            })
        });

        // when calling usecase
        let post_one_dog_fact_usecase = PostOneDogFactUseCase::new(&dog_fact_repository);
        let new_dog_fact = DogFactPayload::new("1".to_string(), 1);
        let data = post_one_dog_fact_usecase.execute(new_dog_fact).await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_txt, "fact1");
        assert_eq!(data.fact_length, 1);
    }
}
