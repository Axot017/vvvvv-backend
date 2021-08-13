use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{
    common::failure::domain::failure::Failure,
    config::auth_config::AuthConfig,
    features::auth::{
        domain::{tokens_pair::TokensPair, user_role::UserRole},
        errors::token_errors::{
            get_invalid_access_token_error, get_invalid_refresh_token_error,
            get_token_generating_error,
        },
        infrastructure::entities::token_claims::TokenClaims,
        interactors::auth_interactor::TokenProvider,
    },
};

pub struct JwtTokenProvider {
    auth_config: AuthConfig,
}

impl JwtTokenProvider {
    pub fn new(auth_config: AuthConfig) -> JwtTokenProvider {
        return JwtTokenProvider { auth_config };
    }
}

impl TokenProvider for JwtTokenProvider {
    fn generate_token(&self, uuid: &String, role: &UserRole) -> Result<TokensPair, Failure> {
        let access_token_claims = TokenClaims {
            authorized: true,
            user_role: String::from(role),
            user_uuid: uuid.clone(),
            exp: (Utc::now().timestamp().unsigned_abs() + self.auth_config.access_token_exp)
                as usize,
        };
        let refresh_token_claims = TokenClaims {
            authorized: true,
            user_role: String::from(role),
            user_uuid: uuid.clone(),
            exp: (Utc::now().timestamp().unsigned_abs() + self.auth_config.refresh_token_exp)
                as usize,
        };

        let access_token = encode(
            &Header::default(),
            &access_token_claims,
            &EncodingKey::from_secret(self.auth_config.private_key.as_ref()),
        );
        let refresh_token = encode(
            &Header::default(),
            &refresh_token_claims,
            &EncodingKey::from_secret(self.auth_config.private_key.as_ref()),
        );

        return match (access_token, refresh_token) {
            (Ok(access), Ok(refresh)) => Ok(TokensPair {
                access_token: access,
                refresh_token: refresh,
                access_token_exp: self.auth_config.access_token_exp,
                refresh_token_exp: self.auth_config.refresh_token_exp,
            }),
            _ => Err(get_token_generating_error()),
        };
    }

    fn validate_access_token(&self, access_token: &String) -> Result<(String, UserRole), Failure> {
        let token = decode::<TokenClaims>(
            access_token,
            &DecodingKey::from_secret(self.auth_config.private_key.as_ref()),
            &Validation::default(),
        );

        return match token {
            Ok(token) => {
                return if token.claims.authorized {
                    Ok((
                        token.claims.user_uuid,
                        UserRole::from(token.claims.user_role.as_str()),
                    ))
                } else {
                    Err(get_invalid_access_token_error())
                }
            }
            _ => Err(get_invalid_access_token_error()),
        };
    }

    fn validate_refresh_token(
        &self,
        refresh_token: &String,
    ) -> Result<(String, UserRole), Failure> {
        let token = decode::<TokenClaims>(
            refresh_token,
            &DecodingKey::from_secret(self.auth_config.private_key.as_ref()),
            &Validation::default(),
        );

        return match token {
            Ok(token) => {
                return if !token.claims.authorized {
                    Ok((
                        token.claims.user_uuid,
                        UserRole::from(token.claims.user_role.as_str()),
                    ))
                } else {
                    Err(get_invalid_refresh_token_error())
                }
            }
            _ => Err(get_invalid_refresh_token_error()),
        };
    }
}
