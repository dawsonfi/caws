use caws::{Kraken, Key};
use std::collections::HashMap;

#[tokio::test]
async fn should_execute_test_agents() {
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