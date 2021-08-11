use crate::{
    common::failure::domain::failure::Failure,
    config::auth_config::AuthConfig,
    features::auth::{
        domain::{auth_data::AuthData, user_role::UserRole},
        infrastructure::entities::tokens_pair::TokensPair,
    },
};

use async_trait::async_trait;

pub trait TokenProvider {
    fn generate_token(&self, uuid: &String, role: &UserRole) -> Result<TokensPair, Failure>;

    fn validate_access_token(&self, access_token: &String) -> Result<(String, UserRole), Failure>;

    fn validate_refresh_token(&self, refresh_token: &String) -> Result<(), Failure>;
}

#[async_trait]
pub trait AuthDataRepository {
    async fn get_auth_data(&self, email: &String, username: &String) -> Result<AuthData, Failure>;
}

pub trait PasswordManager {
    fn hash_password(&self, password: &String) -> Result<String, Failure>;

    fn verify_password(&self, password: &String, hash: &String) -> Result<bool, Failure>;
}

pub struct AuthInteractor<T, Y, U> {
    password_manager: T,
    token_provider: Y,
    auth_data_respository: U,
}

impl<T, Y, U> AuthInteractor<T, Y, U>
where
    T: PasswordManager,
    Y: TokenProvider,
    U: AuthDataRepository,
{
    pub fn new(
        password_manager: T,
        token_provider: Y,
        auth_data_respository: U,
    ) -> AuthInteractor<T, Y, U> {
        return AuthInteractor {
            password_manager,
            token_provider,
            auth_data_respository,
        };
    }
}
