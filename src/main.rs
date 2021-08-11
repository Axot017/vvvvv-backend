mod common;
mod config;
mod features;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use config::{auth_config::AuthConfig, common_config::CommonConfig};
use features::{
    auth::{
        api::auth_controller::configure_auth_controller,
        infrastructure::{
            auth_data_repository_impl::AuthDataRepositoryImpl,
            jwt_token_provider::JwtTokenProvider, password_manager_impl::PasswordManagerImpl,
        },
        interactors::auth_interactor::AuthInteractor,
    },
    mailer::mailer::Mailer,
    profile::{
        api::profile_controller::configure_profile_controller,
        infrastructure::{
            profile_repository_impl::ProfileRepositoryImpl,
            verification_keys_storage_impl::VerificationKeysStorageImpl,
        },
        interactors::profile_interactor::ProfileInteractor,
        utils::code_generator::VerificationCodeGenerator,
    },
};

type Profile = ProfileInteractor<
    ProfileRepositoryImpl,
    VerificationCodeGenerator,
    VerificationKeysStorageImpl,
    Mailer,
>;

type Auth = AuthInteractor<PasswordManagerImpl, JwtTokenProvider, AuthDataRepositoryImpl>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let common_config = CommonConfig::new();
    let profile_interactor = get_profile_interactor();
    let auth_interactor = get_auth_interactor();
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .configure(|cfg| {
                    configure_profile_controller(profile_interactor.clone(), cfg);
                })
                .configure(|cfg| configure_auth_controller(auth_interactor.clone(), cfg)),
        )
    })
    .bind(format!("127.0.0.1:{}", common_config.port))?
    .run()
    .await
}

fn get_auth_interactor() -> Arc<Auth> {
    let interactor = AuthInteractor::new(
        PasswordManagerImpl::new(),
        JwtTokenProvider::new(AuthConfig::new()),
        AuthDataRepositoryImpl::new(),
    );

    Arc::new(interactor)
}

fn get_profile_interactor() -> Arc<Profile> {
    let code_generator = VerificationCodeGenerator::new();
    let verification_keys_storage = VerificationKeysStorageImpl::new();
    let profile_repository = ProfileRepositoryImpl::new();
    let mailer = Mailer::new();
    let interactor = ProfileInteractor::new(
        profile_repository,
        code_generator,
        verification_keys_storage,
        mailer,
    );

    Arc::new(interactor)
}
