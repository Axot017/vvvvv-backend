use crate::{
    common::failure::domain::failure::Failure,
    features::{
        auth::{
            errors::password_errors::{
                get_password_hashing_error, get_password_verification_error,
            },
            interactors::auth_interactor::PasswordVerifier,
        },
        profile::interactors::profile_interactor::PasswordHasher,
    },
};

use async_trait::async_trait;

use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordManagerImpl;

impl PasswordManagerImpl {
    pub fn new() -> PasswordManagerImpl {
        return PasswordManagerImpl {};
    }
}

#[async_trait]
impl PasswordHasher for PasswordManagerImpl {
    async fn hash_password(&self, password: &String) -> Result<String, Failure> {
        let hashed = hash(password, DEFAULT_COST);

        return match hashed {
            Ok(hashed) => Ok(hashed),
            Err(_) => Err(get_password_hashing_error()),
        };
    }
}

#[async_trait]
impl PasswordVerifier for PasswordManagerImpl {
    async fn verify_password(&self, password: &String, hash: &String) -> Result<bool, Failure> {
        let result = verify(password, hash);

        return match result {
            Ok(result) => Ok(result),
            Err(_) => Err(get_password_verification_error()),
        };
    }
}
