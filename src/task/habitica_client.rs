use task::api_credentials::ApiCredentials;
use task::rest_client::{RestOperations, RestClient, RestClientError};
use task::tasks::Tasks;

pub struct HabiticaClient {
    rest_client: Box<RestOperations>,
    url: &'static str,
}

impl HabiticaClient {
    /// Creates a new HabiticaClient
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate habitica_rust_client;
    ///
    /// use habitica_rust_client::task::api_credentials::ApiCredentials;
    /// use habitica_rust_client::task::habitica_client::HabiticaClient;
    ///
    /// let api_credentials = ApiCredentials::new("user_id".to_string(), "api_token".to_string());
    ///
    /// HabiticaClient::new(api_credentials);
    ///
    /// ```
    pub fn new(api_credentials: ApiCredentials) -> HabiticaClient {
        let rest_client = RestClient::new(api_credentials);

        HabiticaClient {
            rest_client,
            url: "https://habitica.com/api/v3/tasks/user",
        }
    }

    /// Returns all of users tasks (habits, dailies, to-dos)
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate habitica_rust_client;
    ///
    /// use habitica_rust_client::task::api_credentials::ApiCredentials;
    /// use habitica_rust_client::task::habitica_client::HabiticaClient;
    /// use std::env;
    ///
    /// let api_credentials = ApiCredentials::new(
    ///        env::var("API_USER").unwrap().to_string(),
    ///        env::var("API_KEY").unwrap().to_string(),
    /// );
    ///
    /// HabiticaClient::new(api_credentials).get_all_tasks();
    ///
    /// ```
    ///
    /// # Errors
    ///
    /// If the REST call to Habitica Api does not succeed (status code diferrent from 200) it will return an error with a String that contains what happened
    ///
    pub fn get_all_tasks(&self) -> Result<Tasks, RestClientError> {
        let response = self.rest_client.get(self.url);

        Ok(Tasks::new(response?))
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::Value;

    #[test]
    fn return_tasks_when_rest_call_succeds() {
        let client = HabiticaClient {
            rest_client: Box::new(SuccessRestClient {}),
            url: "https://habitica.com/api/v3/tasks/user"
        };

        let tasks = client.get_all_tasks();

        assert!(tasks.is_ok());
    }

    #[test]
    fn return_err_when_rest_call_fails() {
        let client = HabiticaClient {
            rest_client: Box::new(ErrorRestClient {}),
            url: "https://habitica.com/api/v3/tasks/user"
        };

        let tasks = client.get_all_tasks();

        assert!(tasks.is_err());
    }

    struct ErrorRestClient;
    struct SuccessRestClient;

    impl RestOperations for ErrorRestClient {
        fn get(&self, _url: &str) -> Result<Value, RestClientError> {
            Err(RestClientError::new("deu ruim".to_string()))
        }
    }

    impl RestOperations for SuccessRestClient {
        fn get(&self, _url: &str) -> Result<Value, RestClientError> {
            Ok(json!({"data": [{"text": "Todo"}]}))
        }
    }

}
