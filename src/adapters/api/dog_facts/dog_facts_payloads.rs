use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DogFactPayload {
    // implement for POST/UPDATE requests
    pub fact: String,
    pub fact_length: i32,
}
impl DogFactPayload {
    pub fn new(fact: String, fact_length: i32) -> Self {
        DogFactPayload { fact, fact_length }
    }
}
