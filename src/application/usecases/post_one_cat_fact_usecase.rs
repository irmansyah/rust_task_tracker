use async_trait::async_trait;

use crate::{
    adapters::api::cat_facts::cat_facts_payloads::CatFactPayload,
    application::{repositories::cat_facts_repository_abstract::CatFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{cat_fact_entity::CatFactEntity, error::ApiError},
};

use super::interfaces::AbstractUseCase;

pub struct PostOneCatFactUseCase<'a> {
    cat_fact_payload: &'a CatFactPayload,
    repository: &'a dyn CatFactsRepositoryAbstract,
}

impl<'a> PostOneCatFactUseCase<'a> {
    pub fn new(cat_fact_payload: &'a CatFactPayload, repository: &'a dyn CatFactsRepositoryAbstract) -> Self {
        PostOneCatFactUseCase { cat_fact_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<CatFactEntity> for PostOneCatFactUseCase<'a> {
    async fn execute(&self) -> Result<CatFactEntity, ApiError> {
        let cat_fact = self.repository.post_one_cat_fact(&self.cat_fact_payload).await;

        match cat_fact {
            Ok(fact) => Ok(fact),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get random cat fact", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::application::{repositories::cat_facts_repository_abstract::MockCatFactsRepositoryAbstract, usecases::post_one_cat_fact_usecase::PostOneCatFactUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all cat facts" usecase repo with an unexpected error
        let mut cat_fact_repository = MockCatFactsRepositoryAbstract::new();
        cat_fact_repository
            .expect_get_random_cat_fact()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let cat_fact_payload = CatFactPayload::new("That is cat fact 1".to_string(), 2);
        let post_one_cat_fact_usecase = PostOneCatFactUseCase::new(&cat_fact_payload, &cat_fact_repository);
        let data = post_one_cat_fact_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get random cat fact", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one random cat fact" usecase repo returning one result
        let mut cat_fact_repository = MockCatFactsRepositoryAbstract::new();
        cat_fact_repository.expect_get_random_cat_fact().with().times(1).returning(|| {
            Ok(CatFactEntity {
                fact_txt: String::from("fact1"),
                fact_length: 1,
            })
        });

        // when calling usecase
        let cat_fact_payload = CatFactPayload::new("That is cat fact 1".to_string(), 2);
        let post_one_cat_fact_usecase = PostOneCatFactUseCase::new(&cat_fact_payload, &cat_fact_repository);
        let data = post_one_cat_fact_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.fact_txt, "fact1");
        assert_eq!(data.fact_length, 1);
    }
}
