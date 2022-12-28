use std::collections::HashMap;
use std::io::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Key {
    pub agents: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct DestructionResult {
    pub execution_status: HashMap<String, Vec<String>>,
}

pub struct Kraken {}

impl Kraken {
    pub async fn release(self, _: Key) -> Result<DestructionResult, Error> {
        Ok(DestructionResult {
            execution_status: HashMap::new(),
        })
    }
}
