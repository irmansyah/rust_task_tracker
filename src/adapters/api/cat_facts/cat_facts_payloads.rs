use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactPayload {
    // implement for POST/UPDATE requests
    pub fact: String,
    pub fact_length: i32,
}

impl CatFactPayload {
    pub fn new(fact: String, fact_length: i32) -> Self {
        CatFactPayload { fact, fact_length }
    }
}
