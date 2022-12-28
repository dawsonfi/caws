use crate::tentacles::{CrashResult, Tentacle};
use async_trait::async_trait;
use std::io::Error;

pub struct DummyTentacle {}

impl DummyTentacle {
    pub fn new() -> Self {
        DummyTentacle {}
    }
}

#[async_trait]
impl Tentacle for DummyTentacle {
    fn group(&self) -> String {
        "test".to_owned()
    }

    fn name(&self) -> String {
        "dummy".to_owned()
    }

    async fn crash(&self) -> Result<CrashResult, Error> {
        Ok(CrashResult {
            success: true,
            result: "Hit Water".to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_group() {
        let dummy_tentacle = DummyTentacle::new();

        assert_eq!("test".to_owned(), dummy_tentacle.group())
    }

    #[test]
    fn should_return_name() {
        let dummy_tentacle = DummyTentacle::new();

        assert_eq!("dummy".to_owned(), dummy_tentacle.name())
    }

    #[tokio::test]
    async fn should_return_crash_result() {
        let dummy_tentacle = DummyTentacle::new();

        let crash_result = dummy_tentacle.crash().await.unwrap();

        assert!(crash_result.success);
        assert_eq!("Hit Water".to_owned(), crash_result.result);
    }
}
