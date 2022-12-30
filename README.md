## Caws

Chaos on AWS Library and Lambda

## Usage

### Rust

* Create a new Rust project with `cargo new --bin PROJECT_NAME`
* Add the necessary dependencies to you `Cargo.toml`:
```
[dependencies]
tokio = { version = "1.23.0", features = ["full"] }
serde_json = "1.0.91"
lambda_runtime = "0.7.2"
caws = "1.0.0"
```

* Change `main.rs` to the following code:

```
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

    let kraken = Kraken::new();
    let destruction_results = kraken.release(key).await?;

    Ok(to_value(DestructionResults {
        execution_status: destruction_results.execution_status,
    })?)
}

```
* Build and package your code with: `cargo lambda build --release --x86-64 --output-format zip` (you might need to install [cargo-lambda first](https://github.com/cargo-lambda/cargo-lambda))

### CDK

* Add the dependency to your cdk project

```
"caws-constructs": "^0.1.4",
```

* Import the `CawsLambda` construct:

```
import { CawsLambda } from 'caws-constructs';
```

* Create the Lambda:

```
new CawsLambda(this, 'lambda-id', {
    functionName: 'function-name',
    brazilPackagePath: 'path/to/bootstrap.zip',
    env: [
        "agents": {
            //Configure here the agents that you want to enable
            "test": ["dummy"]
        }
    ]
})
```

* Run cdk deploy to create the lambda and eventbridge schedule