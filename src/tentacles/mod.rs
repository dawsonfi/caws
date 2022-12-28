use async_trait::async_trait;
use std::collections::HashMap;
use std::io::Error;

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
    pub fn build(_: HashMap<String, Vec<String>>) -> Vec<Box<dyn Tentacle>> {
        vec![]
    }
}
