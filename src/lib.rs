mod tentacles;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use tentacles::TentacleFactory;

#[derive(Deserialize)]
pub struct Key {
    pub agents: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct DestructionResults {
    pub execution_status: HashMap<String, Vec<DestructionResult>>,
}

#[derive(Serialize)]
pub struct DestructionResult {
    pub name: String,
    pub success: bool,
    pub result: String,
}

pub struct Kraken {}

impl Kraken {
    pub async fn release(self, key: Key) -> Result<DestructionResults, Error> {
        let tentacles = TentacleFactory::build(key.agents);
        let mut crash_results: HashMap<String, Vec<DestructionResult>> = HashMap::new();

        for tentacle in tentacles {
            let crash_result = tentacle.crash().await?;

            if !crash_results.contains_key(tentacle.group().as_str()) {
                crash_results.insert(tentacle.group(), vec![]);
            }

            crash_results
                .get_mut(tentacle.group().as_str())
                .unwrap()
                .push(DestructionResult {
                    name: tentacle.name(),
                    success: crash_result.success,
                    result: crash_result.result,
                });
        }

        Ok(DestructionResults {
            execution_status: crash_results,
        })
    }
}
