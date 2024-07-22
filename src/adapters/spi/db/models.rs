use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName, Insertable)]
#[table_name = "dog_facts"]
pub struct DogFact {
    pub id: i32,
    pub fact: String,
}

impl DogFact {
    pub fn new(id: i32, fact: String) -> Self {
        DogFact { id, fact }
    }
}
