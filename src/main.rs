mod common;
mod config;
mod features;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use features::{
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let code_generator = VerificationCodeGenerator::new();
    let verification_keys_storage = VerificationKeysStorageImpl::new();
    let profile_repository = ProfileRepositoryImpl::new();
    let mailer = Mailer::new();
    let profile_interactor = Arc::new(ProfileInteractor::new(
        profile_repository,
        code_generator,
        verification_keys_storage,
        mailer,
    ));
    HttpServer::new(move || {
        App::new().service(web::scope("/api").configure(|cfg| {
            configure_profile_controller(profile_interactor.clone(), cfg);
        }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
