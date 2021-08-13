use std::env;

use crate::features::auth::interactors::auth_interactor::AuthConfigProvider;

pub struct AuthConfig {
    pub client_secret: String,
    pub private_key: String,
    pub access_token_exp: u64,
    pub refresh_token_exp: u64,
}

impl AuthConfig {
    pub fn new() -> AuthConfig {
        let client_secret = env::var("CLIENT_SECRET").unwrap_or("test_client_secret".to_string());
        let private_key = env::var("PRIVATE_KEY").unwrap_or("test_private_key".to_string());
        let access_token_exp = env::var("ACCESS_TOKEN_EXP")
            .unwrap_or("".to_string())
            .parse::<u64>()
            .unwrap_or(420);
        let refresh_token_exp = env::var("REFRESH_TOKEN_EXP")
            .unwrap_or("".to_string())
            .parse::<u64>()
            .unwrap_or(604_800);
        return AuthConfig {
            client_secret,
            private_key,
            access_token_exp,
            refresh_token_exp,
        };
    }
}

impl AuthConfigProvider for AuthConfig {
    fn get_client_secret(&self) -> String {
        return self.client_secret.clone();
    }
}
