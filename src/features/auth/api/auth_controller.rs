use actix_web::{
    post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::{
    common::failure::failure_handler::handle_failure,
    config::auth_config::AuthConfig,
    features::auth::{
        infrastructure::{
            auth_data_repository_impl::AuthDataRepositoryImpl,
            jwt_token_provider::JwtTokenProvider, password_manager_impl::PasswordManagerImpl,
        },
        interactors::auth_interactor::AuthInteractor,
    },
};

use super::dtos::{
    login_dto::LoginDto, refresh_token_dto::RefreshTokenDto, tokens_pair_dto::TokensPairDto,
};

type Interactor =
    AuthInteractor<PasswordManagerImpl, JwtTokenProvider, AuthDataRepositoryImpl, AuthConfig>;

pub fn configure_auth_controller(config: &mut ServiceConfig) {
    config.service(web::scope("/auth").service(login).service(refresh));
}

#[post("/login")]
async fn login(interactor: web::Data<Interactor>, dto: web::Form<LoginDto>) -> impl Responder {
    let result = interactor.validate_client_secret(&dto.client_secret).await;

    if let Err(failure) = result {
        return handle_failure(failure);
    }

    let result = interactor.login(&dto.login, &dto.password).await;

    match result {
        Ok(tokens) => HttpResponse::Ok().json(TokensPairDto::from(tokens)),
        Err(err) => handle_failure(err),
    }
}

#[post("/refresh")]
async fn refresh(
    interactor: web::Data<Interactor>,
    dto: web::Form<RefreshTokenDto>,
) -> impl Responder {
    let result = interactor.validate_client_secret(&dto.client_secret).await;

    if let Err(failure) = result {
        return handle_failure(failure);
    }

    let result = interactor.refresh(&dto.token).await;

    match result {
        Ok(tokens) => HttpResponse::Ok().json(TokensPairDto::from(tokens)),
        Err(err) => handle_failure(err),
    }
}
