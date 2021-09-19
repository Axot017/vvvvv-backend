use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub user_id: i64,
    pub user_role: String,
    pub authorized: bool,
}
