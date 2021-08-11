use crate::{
    common::failure::domain::failure::Failure,
    features::auth::{domain::user_role::UserRole, interactors::auth_interactor::TokenProvider},
};

use super::entities::tokens_pair::TokensPair;

pub struct JwtTokenProvider;

impl JwtTokenProvider {
    pub fn new() -> JwtTokenProvider {
        return JwtTokenProvider {};
    }
}

impl TokenProvider for JwtTokenProvider {
    fn generate_token(uuid: &String, role: &UserRole) -> Result<TokensPair, Failure> {
        todo!()
    }

    fn validate_access_token(access_token: &String) -> Result<(String, UserRole), Failure> {
        todo!()
    }

    fn validate_refresh_token(refresh_token: &String) -> Result<(), Failure> {
        todo!()
    }
}
