use async_trait::async_trait;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use std::error::Error;
use std::sync::Arc;

use crate::adapters::api::dog_facts::dog_facts_payloads::DogFactPayload;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::DogFactDbMapper, models::DogFact, schema::dog_facts::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract};
use crate::domain::dog_fact_entity::DogFactEntity;

use crate::adapters::spi::db::schema::dog_facts;
use crate::adapters::spi::db::schema::dog_facts::id;

pub struct DogFactsRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl DogFactsRepositoryAbstract for DogFactsRepository {
    async fn post_one_dog_fact(&self, dog_fact_payload: &DogFactPayload) -> Result<DogFactEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let new_dog_fact = DogFact::new(dog_fact_payload.fact_id, dog_fact_payload.fact.clone());
        let result = diesel::insert_into(dog_facts::table).values(&new_dog_fact).get_result::<DogFact>(&mut conn);

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = dog_facts.filter(id.eq(dog_fact_id)).get_result::<DogFact>(&mut conn);

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = dog_facts.load::<DogFact>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(DogFactDbMapper::to_entity).collect::<Vec<DogFactEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
