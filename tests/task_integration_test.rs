extern crate habitica_rust_client;

use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use std::env;

#[test]
fn should_get_tasks_for_user() {
    let api_credentials = ApiCredentials::new(
        env::var("API_USER").unwrap().to_string(),
        env::var("API_KEY").unwrap().to_string(),
    );
    let habitica_client = HabiticaClient::new(api_credentials);

    let tasks = habitica_client.get_all_tasks();

    assert!(tasks.unwrap().get_tasks().len() > 1);
}
