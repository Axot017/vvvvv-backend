#[derive(Debug, PartialEq, Clone)]
pub struct TokensPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_exp: u64,
    pub refresh_token_exp: u64,
}
