use crate::features::profile::interactors::profile_interactor::CodeGenerator;
use async_trait::async_trait;
use nanoid::nanoid;

pub struct VerificationCodeGenerator;

impl VerificationCodeGenerator {
    pub fn new() -> VerificationCodeGenerator {
        return VerificationCodeGenerator {};
    }
}

#[async_trait]
impl CodeGenerator for VerificationCodeGenerator {
    async fn generate(&self) -> String {
        return nanoid!();
    }
}
