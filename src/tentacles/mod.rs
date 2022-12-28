mod test_tentacles;

use async_trait::async_trait;
use std::collections::HashMap;
use std::io::Error;
use test_tentacles::DummyTentacle;

pub struct CrashResult {
    pub success: bool,
    pub result: String,
}

#[async_trait]
pub trait Tentacle {
    fn group(&self) -> String;
    fn name(&self) -> String;
    async fn crash(&self) -> Result<CrashResult, Error>;
}

pub struct TentacleFactory {}

impl TentacleFactory {
    pub fn build(enabled_tentacles: HashMap<String, Vec<String>>) -> Vec<Box<dyn Tentacle>> {
        let tentacles: Vec<Box<dyn Tentacle>> = vec![Box::new(DummyTentacle::new())];

        tentacles
            .into_iter()
            .filter(|tentacle| TentacleFactory::is_enabled(&enabled_tentacles, tentacle))
            .collect()
    }

    fn is_enabled(
        enabled_tentacles: &HashMap<String, Vec<String>>,
        tentacle: &Box<dyn Tentacle>,
    ) -> bool {
        enabled_tentacles.contains_key(tentacle.group().as_str())
            && enabled_tentacles
                .get(tentacle.group().as_str())
                .unwrap()
                .contains(&tentacle.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_enabled_tentacles() {
        let mut enabled_tentacles: HashMap<String, Vec<String>> = HashMap::new();
        enabled_tentacles.insert("test".to_owned(), vec!["dummy".to_owned()]);

        let tentacles = TentacleFactory::build(enabled_tentacles.clone());

        for tentacle in tentacles {
            let group = enabled_tentacles.get(tentacle.group().as_str());
            assert!(group.is_some());
            assert!(group.unwrap().contains(&tentacle.name()))
        }
    }

    #[test]
    fn should_return_empty_list_when_no_tentacle_is_enabled() {
        let enabled_tentacles: HashMap<String, Vec<String>> = HashMap::new();

        let tentacles = TentacleFactory::build(enabled_tentacles.clone());

        assert!(tentacles.is_empty());
    }
}
