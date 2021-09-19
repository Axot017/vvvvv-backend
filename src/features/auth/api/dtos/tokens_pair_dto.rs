use serde::Serialize;

use crate::features::auth::domain::tokens_pair::TokensPair;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TokensPairDto {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_exp: u64,
    pub refresh_token_exp: u64,
}

impl From<TokensPair> for TokensPairDto {
    fn from(pair: TokensPair) -> Self {
        TokensPairDto {
            access_token: pair.access_token,
            refresh_token: pair.refresh_token,
            access_token_exp: pair.access_token_exp,
            refresh_token_exp: pair.refresh_token_exp,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_map_model_to_dto() {
        let token = TokensPairDto {
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            access_token_exp: 1,
            refresh_token_exp: 2,
        };

        let result = TokensPairDto::from(token);

        assert_eq!(result.access_token, "access".to_string());
        assert_eq!(result.refresh_token, "refresh".to_string());
        assert_eq!(result.access_token_exp, 1);
        assert_eq!(result.refresh_token_exp, 2);
    }
}
