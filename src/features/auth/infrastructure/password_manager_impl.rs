use crate::{
    common::failure::domain::failure::Failure,
    features::auth::{
        errors::password_errors::{get_password_hashing_error, get_password_verification_error},
        interactors::auth_interactor::PasswordManager,
    },
};

use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordManagerImpl;

impl PasswordManagerImpl {
    pub fn new() -> PasswordManagerImpl {
        return PasswordManagerImpl {};
    }
}

impl PasswordManager for PasswordManagerImpl {
    fn hash_password(password: &String) -> Result<String, Failure> {
        let hashed = hash(password, DEFAULT_COST);

        return match hashed {
            Ok(hashed) => Ok(hashed),
            Err(_) => Err(get_password_verification_error()),
        };
    }

    fn verify_password(password: &String, hash: &String) -> Result<bool, Failure> {
        let result = verify(password, hash);

        return match result {
            Ok(result) => Ok(result),
            Err(_) => Err(get_password_hashing_error()),
        };
    }
}
