use task::api_credentials::ApiCredentials;
use reqwest;
use reqwest::{IntoUrl, Client};
use reqwest::header::Headers;
use serde::de::DeserializeOwned;
use std::fmt;

pub struct RestClient {
    api_credentials: ApiCredentials,
    client: Client
}

impl RestClient {

    pub fn new(api_credentials: ApiCredentials) -> RestClient {
        let client = reqwest::Client::new();
        RestClient { api_credentials, client}
    }

    pub fn get<T: IntoUrl, D: DeserializeOwned>(&self, url: T) -> Result<D, RestClientError> {
        let headers = self.build_auth_headers();

        Ok(self.get_client()
            .get(url)
            .headers(headers)
            .send()
            .unwrap()
            .json::<D>()
            .unwrap())
    }

    fn build_auth_headers(&self) -> Headers {
        let api_credentials = self.get_api_credentials();
        let mut headers = Headers::new();

        headers.set_raw("x-api-user", api_credentials.get_user().to_owned());
        headers.set_raw("x-api-key", api_credentials.get_key().to_owned());

        headers
    }

    fn get_api_credentials(&self) -> &ApiCredentials {
        &self.api_credentials
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

}

pub struct RestClientError;

impl fmt::Debug for RestClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error")
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