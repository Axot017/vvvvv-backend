use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyEmailDto {
    pub code: String,
}
