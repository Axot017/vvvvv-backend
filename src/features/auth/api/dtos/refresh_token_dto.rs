use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenDto {
    pub token: String,
    pub client_secret: String,
}
