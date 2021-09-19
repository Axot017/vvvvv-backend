use crate::{
    common::failure::domain::failure::Failure,
    features::auth::{
        domain::{auth_data::AuthData, tokens_pair::TokensPair, user_role::UserRole},
        errors::{
            auth_errors::get_invalid_credentials_error,
            client_secret_errors::get_client_secret_error,
        },
    },
};

use async_trait::async_trait;

pub trait TokenProvider {
    fn generate_token(&self, id: &i64, role: &UserRole) -> Result<TokensPair, Failure>;

    fn validate_access_token(&self, access_token: &String) -> Result<(i64, UserRole), Failure>;

    fn validate_refresh_token(&self, refresh_token: &String) -> Result<(i64, UserRole), Failure>;
}

pub trait AuthConfigProvider {
    fn get_client_secret(&self) -> String;
}

#[async_trait]
pub trait AuthDataRepository {
    async fn get_auth_data(&self, login: &String) -> Result<AuthData, Failure>;
}

#[async_trait]
pub trait PasswordVerifier {
    async fn verify_password(&self, password: &String, hash: &String) -> Result<bool, Failure>;
}

pub struct AuthInteractor<T, Y, U, I> {
    password_manager: T,
    token_provider: Y,
    auth_data_repository: U,
    auth_config_provider: I,
}

impl<T, Y, U, I> AuthInteractor<T, Y, U, I>
where
    T: PasswordVerifier,
    Y: TokenProvider,
    U: AuthDataRepository,
    I: AuthConfigProvider,
{
    pub fn new(
        password_manager: T,
        token_provider: Y,
        auth_data_repository: U,
        auth_config_provider: I,
    ) -> AuthInteractor<T, Y, U, I> {
        return AuthInteractor {
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        };
    }

    pub async fn validate_client_secret(&self, client_secret: &String) -> Result<(), Failure> {
        if client_secret == &self.auth_config_provider.get_client_secret() {
            Ok(())
        } else {
            Err(get_client_secret_error())
        }
    }

    pub async fn login(&self, login: &String, password: &String) -> Result<TokensPair, Failure> {
        let profile = self.auth_data_repository.get_auth_data(&login).await?;
        let is_password_valid = self
            .password_manager
            .verify_password(&password, &profile.password)
            .await?;
        if is_password_valid {
            let tokens = self
                .token_provider
                .generate_token(&profile.id, &profile.user_role)?;

            Ok(tokens)
        } else {
            Err(get_invalid_credentials_error())
        }
    }

    pub async fn refresh(&self, refresh_token: &String) -> Result<TokensPair, Failure> {
        let (id, role) = self.token_provider.validate_refresh_token(&refresh_token)?;

        let tokens = self.token_provider.generate_token(&id, &role)?;

        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use chrono::Utc;
    use mockall::{mock, predicate};

    use crate::{
        common::failure::domain::failure::FailureType,
        features::auth::errors::client_secret_errors::get_client_secret_error,
    };

    use super::*;

    mock! {
        PasswordManager {}

        #[async_trait]
        impl PasswordVerifier for PasswordManager {
            async fn verify_password(&self, password: &String, hash: &String) -> Result<bool, Failure>;
        }
    }

    mock! {
        TokenProvider {}

        impl TokenProvider for TokenProvider {
            fn generate_token(&self, id: &i64, role: &UserRole) -> Result<TokensPair, Failure>;

            fn validate_access_token(&self, access_token: &String) -> Result<(i64, UserRole), Failure>;

            fn validate_refresh_token(&self, refresh_token: &String) -> Result<(i64, UserRole), Failure>;
        }
    }

    mock! {
        AuthDataRepository {}

        #[async_trait]
        impl AuthDataRepository for AuthDataRepository {
            async fn get_auth_data(&self, login: &String) -> Result<AuthData, Failure>;
        }
    }

    mock! {
        AuthConfigProvider {}

        impl AuthConfigProvider for AuthConfigProvider {
            fn get_client_secret(&self) -> String;
        }
    }

    #[actix_rt::test]
    async fn should_return_error_if_client_secret_invalid() {
        let password_manager = MockPasswordManager::new();
        let auth_data_repository = MockAuthDataRepository::new();
        let mut auth_config_provider = MockAuthConfigProvider::new();
        let token_provider = MockTokenProvider::new();
        auth_config_provider
            .expect_get_client_secret()
            .return_once(|| "test_client_secret".to_string());
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .validate_client_secret(&"invalid_client_secret".to_string())
            .await;

        assert_eq!(result, Err(get_client_secret_error()));
    }

    #[actix_rt::test]
    async fn should_return_ok_when_secret_is_valid() {
        let password_manager = MockPasswordManager::new();
        let auth_data_repository = MockAuthDataRepository::new();
        let mut auth_config_provider = MockAuthConfigProvider::new();
        let token_provider = MockTokenProvider::new();
        auth_config_provider
            .expect_get_client_secret()
            .return_once(|| "test_client_secret".to_string());
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .validate_client_secret(&"test_client_secret".to_string())
            .await;

        assert_eq!(result, Ok(()));
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_get_profile() {
        let failure = Failure {
            args: None,
            code: "Test".to_string(),
            message: "Test".to_string(),
            error_type: FailureType::Authentication,
        };
        let failure_clone = failure.clone();

        let password_manager = MockPasswordManager::new();
        let mut auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let token_provider = MockTokenProvider::new();
        auth_data_repository
            .expect_get_auth_data()
            .with(predicate::eq("login".to_string()))
            .return_once(move |_| Err(failure_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .login(&"login".to_string(), &"password".to_string())
            .await;

        assert_eq!(result, Err(failure));
    }

    #[actix_rt::test]
    async fn should_return_error_if_fail_to_verify_password() {
        let failure = Failure {
            args: None,
            code: "Test".to_string(),
            message: "Test".to_string(),
            error_type: FailureType::Authentication,
        };
        let profile = AuthData {
            email: "email".to_string(),
            password: "hash".to_string(),
            user_role: UserRole::USER,
            username: "username".to_string(),
            id: 1,
            verified_at: Some(Utc::now()),
        };
        let failure_clone = failure.clone();

        let mut password_manager = MockPasswordManager::new();
        let mut auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let token_provider = MockTokenProvider::new();
        auth_data_repository
            .expect_get_auth_data()
            .with(predicate::eq("login".to_string()))
            .return_once(move |_| Ok(profile));
        password_manager
            .expect_verify_password()
            .with(
                predicate::eq("password".to_string()),
                predicate::eq("hash".to_string()),
            )
            .return_once(move |_, __| Err(failure_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .login(&"login".to_string(), &"password".to_string())
            .await;

        assert_eq!(result, Err(failure));
    }

    #[actix_rt::test]
    async fn should_return_error_if_password_invalid() {
        let profile = AuthData {
            email: "email".to_string(),
            password: "hash".to_string(),
            user_role: UserRole::USER,
            username: "username".to_string(),
            id: 1,
            verified_at: Some(Utc::now()),
        };

        let mut password_manager = MockPasswordManager::new();
        let mut auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let token_provider = MockTokenProvider::new();
        auth_data_repository
            .expect_get_auth_data()
            .with(predicate::eq("login".to_string()))
            .return_once(move |_| Ok(profile));
        password_manager
            .expect_verify_password()
            .with(
                predicate::eq("password".to_string()),
                predicate::eq("hash".to_string()),
            )
            .return_once(move |_, __| Ok(false));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .login(&"login".to_string(), &"password".to_string())
            .await;

        assert_eq!(result, Err(get_invalid_credentials_error()));
    }

    #[actix_rt::test]
    async fn should_return_error_if_fail_to_generate_token() {
        let failure = Failure {
            args: None,
            code: "Test".to_string(),
            message: "Test".to_string(),
            error_type: FailureType::Authentication,
        };
        let profile = AuthData {
            email: "email".to_string(),
            password: "hash".to_string(),
            user_role: UserRole::USER,
            username: "username".to_string(),
            id: 1,
            verified_at: Some(Utc::now()),
        };
        let failure_clone = failure.clone();

        let mut password_manager = MockPasswordManager::new();
        let mut auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let mut token_provider = MockTokenProvider::new();
        auth_data_repository
            .expect_get_auth_data()
            .with(predicate::eq("login".to_string()))
            .return_once(move |_| Ok(profile));
        password_manager
            .expect_verify_password()
            .with(
                predicate::eq("password".to_string()),
                predicate::eq("hash".to_string()),
            )
            .return_once(move |_, __| Ok(true));
        token_provider
            .expect_generate_token()
            .with(predicate::eq(1), predicate::eq(UserRole::USER))
            .return_once(move |_, __| Err(failure_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .login(&"login".to_string(), &"password".to_string())
            .await;

        assert_eq!(result, Err(failure));
    }

    #[actix_rt::test]
    async fn should_return_return_tokens_pair() {
        let profile = AuthData {
            email: "email".to_string(),
            password: "hash".to_string(),
            user_role: UserRole::USER,
            username: "username".to_string(),
            id: 1,
            verified_at: Some(Utc::now()),
        };
        let tokens_pair = TokensPair {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
            access_token_exp: 1,
            refresh_token_exp: 2,
        };
        let tokens_pair_clone = tokens_pair.clone();

        let mut password_manager = MockPasswordManager::new();
        let mut auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let mut token_provider = MockTokenProvider::new();
        auth_data_repository
            .expect_get_auth_data()
            .with(predicate::eq("login".to_string()))
            .return_once(move |_| Ok(profile));
        password_manager
            .expect_verify_password()
            .with(
                predicate::eq("password".to_string()),
                predicate::eq("hash".to_string()),
            )
            .return_once(move |_, __| Ok(true));
        token_provider
            .expect_generate_token()
            .with(predicate::eq(1), predicate::eq(UserRole::USER))
            .return_once(move |_, __| Ok(tokens_pair_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor
            .login(&"login".to_string(), &"password".to_string())
            .await;

        assert_eq!(result, Ok(tokens_pair));
    }

    #[actix_rt::test]
    async fn should_return_error_if_refresh_token_is_invalid() {
        let failure = Failure {
            args: None,
            code: "Test".to_string(),
            message: "Test".to_string(),
            error_type: FailureType::Authentication,
        };
        let failure_clone = failure.clone();

        let password_manager = MockPasswordManager::new();
        let auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let mut token_provider = MockTokenProvider::new();
        token_provider
            .expect_validate_refresh_token()
            .with(predicate::eq("refresh_token".to_string()))
            .return_once(move |_| Err(failure_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor.refresh(&"refresh_token".to_string()).await;

        assert_eq!(result, Err(failure));
    }

    #[actix_rt::test]
    async fn should_return_error_when_failed_to_generate_new_tokens() {
        let failure = Failure {
            args: None,
            code: "Test".to_string(),
            message: "Test".to_string(),
            error_type: FailureType::Authentication,
        };
        let failure_clone = failure.clone();

        let password_manager = MockPasswordManager::new();
        let auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let mut token_provider = MockTokenProvider::new();
        token_provider
            .expect_validate_refresh_token()
            .with(predicate::eq("refresh_token".to_string()))
            .return_once(|_| Ok((1, UserRole::USER)));
        token_provider
            .expect_generate_token()
            .with(predicate::eq(1), predicate::eq(UserRole::USER))
            .return_once(move |_, __| Err(failure_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor.refresh(&"refresh_token".to_string()).await;

        assert_eq!(result, Err(failure));
    }

    #[actix_rt::test]
    async fn should_return_new_token_pair_if_refresh_token_is_valid() {
        let tokens_pair = TokensPair {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
            access_token_exp: 1,
            refresh_token_exp: 2,
        };
        let tokens_pair_clone = tokens_pair.clone();

        let password_manager = MockPasswordManager::new();
        let auth_data_repository = MockAuthDataRepository::new();
        let auth_config_provider = MockAuthConfigProvider::new();
        let mut token_provider = MockTokenProvider::new();
        token_provider
            .expect_validate_refresh_token()
            .with(predicate::eq("refresh_token".to_string()))
            .return_once(|_| Ok((1, UserRole::USER)));
        token_provider
            .expect_generate_token()
            .with(predicate::eq(1), predicate::eq(UserRole::USER))
            .return_once(move |_, __| Ok(tokens_pair_clone));
        let interactor = AuthInteractor::new(
            password_manager,
            token_provider,
            auth_data_repository,
            auth_config_provider,
        );

        let result = interactor.refresh(&"refresh_token".to_string()).await;

        assert_eq!(result, Ok(tokens_pair));
    }
}
