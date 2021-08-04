use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub login: String,
    pub password: String,
    pub client_secret: String,
}
