mod common;
mod config;
mod features;
mod schema;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate r2d2;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use config::{auth_config::AuthConfig, common_config::CommonConfig, profile_config::ProfileConfig};
use diesel::{r2d2::ConnectionManager, PgConnection};
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
use r2d2::Pool;
use redis::aio::MultiplexedConnection;

type Profile = ProfileInteractor<
    ProfileRepositoryImpl,
    VerificationCodeGenerator,
    VerificationKeysStorageImpl,
    Mailer,
>;

type Auth =
    AuthInteractor<PasswordManagerImpl, JwtTokenProvider, AuthDataRepositoryImpl, AuthConfig>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let common_config = CommonConfig::new();

    let manager = ConnectionManager::<PgConnection>::new(common_config.db_url);
    let pool = r2d2::Pool::new(manager).unwrap();
    let redis_client = redis::Client::open(common_config.redis_url).unwrap();
    let redis_connection = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .unwrap();

    let profile_interactor = get_profile_interactor(pool.clone(), redis_connection.clone());
    let auth_interactor = get_auth_interactor(pool.clone());

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

fn get_auth_interactor(pool: Pool<ConnectionManager<PgConnection>>) -> Arc<Auth> {
    let interactor = AuthInteractor::new(
        PasswordManagerImpl::new(),
        JwtTokenProvider::new(AuthConfig::new()),
        AuthDataRepositoryImpl::new(pool),
        AuthConfig::new(),
    );

    Arc::new(interactor)
}

fn get_profile_interactor(
    pool: Pool<ConnectionManager<PgConnection>>,
    redis_connection: MultiplexedConnection,
) -> Arc<Profile> {
    let config = ProfileConfig::new();
    let code_generator = VerificationCodeGenerator::new();
    let verification_keys_storage = VerificationKeysStorageImpl::new(redis_connection, config);
    let profile_repository = ProfileRepositoryImpl::new(pool);
    let mailer = Mailer::new();
    let interactor = ProfileInteractor::new(
        profile_repository,
        code_generator,
        verification_keys_storage,
        mailer,
    );

    Arc::new(interactor)
}
