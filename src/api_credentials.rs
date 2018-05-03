pub struct ApiCredentials {
    api_user: String,
    api_key: String,
}

impl ApiCredentials {
    pub fn new(api_user: String, api_key: String) -> ApiCredentials {
        ApiCredentials {
            api_user: api_user,
            api_key: api_key,
        }
    }

    pub fn get_user(&self) -> &String {
        &self.api_user
    }

    pub fn get_key(&self) -> &String {
        &self.api_key
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn return_api_credentials_with_user_and_key() {
        let api_credentials = ApiCredentials::new("french".to_string(), "fries".to_string());

        assert_eq!(api_credentials.get_user(), &"french".to_string());
        assert_eq!(api_credentials.get_key(), &"fries".to_string());
    }

}
