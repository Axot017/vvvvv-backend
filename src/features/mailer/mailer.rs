use crate::{
    common::failure::domain::failure::Failure,
    features::profile::interactors::profile_interactor::VerificationMailer,
};
use async_trait::async_trait;

pub struct Mailer;

impl Mailer {
    pub fn new() -> Mailer {
        return Mailer {};
    }
}

#[async_trait]
impl VerificationMailer for Mailer {
    async fn send_verification_code(&self, email: &String, code: &String) -> Result<(), Failure> {
        return Ok(());
    }
}
