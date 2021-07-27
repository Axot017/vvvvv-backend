use crate::{
    common::failure::domain::failure::Failure,
    features::profile::interactors::profile_interactor::VerificationKeysStorage,
};
use async_trait::async_trait;

pub struct VerificationKeysStorageImpl;

impl VerificationKeysStorageImpl {
    pub fn new() -> VerificationKeysStorageImpl {
        return VerificationKeysStorageImpl {};
    }
}

#[async_trait]
impl VerificationKeysStorage for VerificationKeysStorageImpl {
    async fn save_verification_code(
        &self,
        email: &String,
        verification_code: &String,
    ) -> Result<(), Failure> {
        let _ = email;
        let _ = verification_code;
        Ok(())
    }

    async fn get_verification_code(&self, email: &String) -> Result<String, Failure> {
        let _ = email;
        Ok("test".to_string())
    }
}
