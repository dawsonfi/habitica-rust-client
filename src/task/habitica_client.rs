use task::api_credentials::ApiCredentials;
use task::rest_client::{RestClient, RestClientError};
use task::tasks::Tasks;
use serde_json::Value;

pub struct HabiticaClient {
    rest_client: RestClient,
    url: &'static str
}

impl HabiticaClient {
    pub fn new(api_credentials: ApiCredentials) -> HabiticaClient {
        let rest_client = RestClient::new(api_credentials);

        HabiticaClient { rest_client, url: "https://habitica.com/api/v3/tasks/user" }
    }

    pub fn get_all_tasks(&self) -> Result<Tasks, RestClientError> {
        let response = self.rest_client.get::<&str, Value>(self.url);

        Ok(Tasks::new(response?))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_tasks_from_habitica() {

        let api_credentials = ApiCredentials::new("user".to_string(), "key".to_string());

        let client = HabiticaClient::new(api_credentials);

        let tasks = client.get_all_tasks();
    }

}
