use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::adapters::api::dog_facts::dog_facts_payloads::DogFactPayload;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::DogFactDbMapper, models::DogFact, schema::dog_facts::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract};
use crate::domain::dog_fact_entity::DogFactEntity;

pub struct DogFactsRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl DogFactsRepositoryAbstract for DogFactsRepository {
    async fn post_one_dog_fact(&self, cat_fact_payload: &DogFactPayload) -> Result<DogFactEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        // Assuming `cat_facts` is your database table/model for CatFacts
        let cat_fact = DogFactPayload::new(
            cat_fact_payload.fact.clone(), // Extract data from payload
            cat_fact_payload.fact_length.clone(),
        );

        let result = dog_facts.get_result::<DogFact>(&mut conn);

        // Use your ORM/SQL to insert the data
        cat_facts.insert(&mut conn, &cat_fact)?;
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
