use task::api_credentials::ApiCredentials;
use task::tasks::Tasks;

pub struct HabiticaClient {
    api_credentials: ApiCredentials,
}

impl HabiticaClient {
    pub fn new(api_credentials: ApiCredentials) -> HabiticaClient {
        HabiticaClient { api_credentials }
    }

    pub fn get_api_credentials(&self) -> &ApiCredentials {
        &self.api_credentials
    }

    pub fn get_all_tasks(&self) -> Tasks {
        Tasks::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_client_with_api_credentials() {
        let api_credentials = ApiCredentials::new("raw".to_string(), "potato".to_string());

        let client = HabiticaClient::new(api_credentials);

        assert_eq!(client.get_api_credentials().get_user(), &"raw".to_string());
    }

}
