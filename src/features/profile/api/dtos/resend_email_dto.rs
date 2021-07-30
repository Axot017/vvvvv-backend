use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResendEmailDto {
    pub email: String,
}
