use serde_json::Value;
use task::api_credentials::ApiCredentials;
use task::rest_client::{RestClient, RestClientError};
use task::tasks::Tasks;

pub struct HabiticaClient {
    rest_client: RestClient,
    url: &'static str,
}

impl HabiticaClient {
    pub fn new(api_credentials: ApiCredentials) -> HabiticaClient {
        let rest_client = RestClient::new(api_credentials);

        HabiticaClient {
            rest_client,
            url: "https://habitica.com/api/v3/tasks/user",
        }
    }

    pub fn get_all_tasks(&self) -> Result<Tasks, RestClientError> {
        let response = self.rest_client.get::<&str, Value>(self.url);

        Ok(Tasks::new(response?))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use mocktopus::mocking::*;

    #[test]
    #[ignore] //returning exception
    fn return_sucess_when_rest_call_succeeds() {
        let api_credentials = ApiCredentials::new("user".to_string(), "key".to_string());
        RestClient::get::<&str, Value>
            .mock_safe(|_, _| MockResult::Return(Ok(json!({"data": [{"text": "Todo"}]}))));
        let client = HabiticaClient::new(api_credentials);

        let tasks = client.get_all_tasks();

        assert!(tasks.is_ok());
    }

    #[test]
    fn return_err_when_rest_call_fails() {
        let api_credentials = ApiCredentials::new("user".to_string(), "key".to_string());
        let client = HabiticaClient::new(api_credentials);
        RestClient::get::<&str, Value>
            .mock_safe(|_, _| MockResult::Return(Err(RestClientError::new("failed".to_string()))));

        let tasks = client.get_all_tasks();

        assert!(tasks.is_err());
    }

}
