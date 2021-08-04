use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder,
};

use crate::features::auth::interactors::auth_interactor::AuthInteractor;

use super::dtos::{login_dto::LoginDto, refresh_token_dto::RefreshTokenDto};

type Interactor = AuthInteractor;

pub fn configure_auth_controller(interactor: Arc<Interactor>, config: &mut ServiceConfig) {
    config.service(
        web::scope("/auth")
            .app_data(Data::from(interactor))
            .service(login)
            .service(refresh),
    );
}

#[post("/login")]
async fn login(dto: web::Form<LoginDto>) -> impl Responder {
    let _ = dto;
    HttpResponse::Ok()
}

#[post("/refresh")]
async fn refresh(dto: web::Form<RefreshTokenDto>) -> impl Responder {
    let _ = dto;
    HttpResponse::Ok()
}
