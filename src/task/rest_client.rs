use reqwest;
use reqwest::header::Headers;
use reqwest::Client;
use serde_json::Value;
use std::fmt;
use task::api_credentials::ApiCredentials;

pub trait RestOperations {
    fn get(&self, url: &str) -> Result<Value, RestClientError>;
}

pub struct RestClient {
    api_credentials: ApiCredentials,
    client: Client,
}

impl RestClient {
    pub fn new(api_credentials: ApiCredentials) -> Box<RestOperations> {
        let client = reqwest::Client::new();
        Box::new(RestClient {
            api_credentials,
            client,
        })
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

impl RestOperations for RestClient {
    fn get(&self, url: &str) -> Result<Value, RestClientError> {
        let headers = self.build_auth_headers();

        Ok(self
            .get_client()
            .get(url)
            .headers(headers)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap())
    }
}

pub struct RestClientError {
    message: String,
}

impl fmt::Debug for RestClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error")
    }
}

impl RestClientError {
    pub fn new(message: String) -> RestClientError {
        RestClientError { message }
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}
