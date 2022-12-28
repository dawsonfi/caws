use caws::{DestructionResults, Key, Kraken};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{from_value, to_value, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(func)).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let event = event.payload;
    let key: Key = from_value(event)?;

    let kraken = Kraken {};
    let destruction_results = kraken.release(key).await?;

    Ok(to_value(DestructionResults {
        execution_status: destruction_results.execution_status,
    })?)
}
