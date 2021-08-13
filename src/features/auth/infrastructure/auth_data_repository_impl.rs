use crate::{
    common::failure::domain::failure::Failure,
    features::auth::{
        domain::auth_data::AuthData, interactors::auth_interactor::AuthDataRepository,
    },
};

use async_trait::async_trait;

pub struct AuthDataRepositoryImpl;

impl AuthDataRepositoryImpl {
    pub fn new() -> AuthDataRepositoryImpl {
        return AuthDataRepositoryImpl {};
    }
}

#[async_trait]
impl AuthDataRepository for AuthDataRepositoryImpl {
    async fn get_auth_data(&self, login: &String) -> Result<AuthData, Failure> {
        let _ = login;
        todo!()
    }
}
