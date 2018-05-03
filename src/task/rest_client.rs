use task::api_credentials::ApiCredentials;

pub struct RestClient {
    api_credentials: ApiCredentials,
}

impl RestClient {

    pub fn new(api_credentials: ApiCredentials) -> RestClient {
        RestClient { api_credentials }
    }

    pub fn get_api_credentials(&self) -> &ApiCredentials {
        &self.api_credentials
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_client_with_credentials() {
        let api_credentials = ApiCredentials::new("raw".to_string(), "potato".to_string());

        let client = RestClient::new(api_credentials);

        assert_eq!(client.get_api_credentials().get_user(), &"raw".to_string())
    }

}