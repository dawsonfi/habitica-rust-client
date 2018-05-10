#[cfg(test)]
use mocktopus::macros::*;

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

#[cfg_attr(test, mockable)]
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

pub struct RestClientError {
    message: String
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