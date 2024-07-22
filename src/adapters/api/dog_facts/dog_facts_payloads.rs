use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DogFactPayload {
    // implement for POST/UPDATE requests
    pub fact_id: i32,
    pub fact: String,
}

impl DogFactPayload {
    pub fn new(fact_id: i32, fact: String) -> Self {
        DogFactPayload { fact_id, fact }
    }
}
