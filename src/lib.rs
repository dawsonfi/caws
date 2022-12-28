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
    pub fn new() -> Self {
        Kraken {}
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_execute_enabled_tentacles() {
        let mut enabled_tentacles: HashMap<String, Vec<String>> = HashMap::new();
        enabled_tentacles.insert("test".to_owned(), vec!["dummy".to_owned()]);

        let kraken = Kraken::new();
        let destruction_results = kraken
            .release(Key {
                agents: enabled_tentacles,
            })
            .await
            .unwrap();

        let result_group = destruction_results.execution_status.get("test");
        assert!(result_group.is_some());
        
        let results = result_group.unwrap();
        assert_eq!(1, results.len());

        let result = &results[0];
        assert_eq!("dummy", result.name);
        assert_eq!("Hit Water", result.result);
        assert!(result.success);
    }
}
