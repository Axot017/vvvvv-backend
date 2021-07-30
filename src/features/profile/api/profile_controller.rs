use std::sync::Arc;

use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder,
};

use crate::{
    common::failure::failure_handler::handle_failure,
    features::{
        mailer::mailer::Mailer,
        profile::{
            infrastructure::{
                profile_repository_impl::ProfileRepositoryImpl,
                verification_keys_storage_impl::VerificationKeysStorageImpl,
            },
            interactors::profile_interactor::ProfileInteractor,
            utils::code_generator::VerificationCodeGenerator,
        },
    },
};

use super::dtos::{
    create_user_dto::CreateUserDto, resend_email_dto::ResendEmailDto,
    verify_email_dto::VerifyEmailDto,
};

type Interceptor = ProfileInteractor<
    ProfileRepositoryImpl,
    VerificationCodeGenerator,
    VerificationKeysStorageImpl,
    Mailer,
>;

pub fn configure_profile_controller(interactor: Arc<Interceptor>, config: &mut ServiceConfig) {
    config.service(
        web::scope("/profile")
            .app_data(Data::from(interactor))
            .service(get_current_user)
            .service(resend_email)
            .service(create_user),
    );
}

#[post("/email/resend")]
async fn resend_email(
    interactor: web::Data<Interceptor>,
    dto: web::Json<ResendEmailDto>,
) -> impl Responder {
    let result = interactor.resend_email(&dto.email).await;
    match result {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(err) => handle_failure(err),
    }
}

#[post("/email/verify")]
async fn verify_user(
    interactor: web::Data<Interceptor>,
    dto: web::Json<VerifyEmailDto>,
) -> impl Responder {
    let result = interactor.verify_email(&dto.code).await;
    match result {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(err) => handle_failure(err),
    }
}

#[get("/current")]
async fn get_current_user() -> impl Responder {
    HttpResponse::Ok()
}

#[post("")]
async fn create_user(
    interactor: web::Data<Interceptor>,
    dto: web::Json<CreateUserDto>,
) -> impl Responder {
    let user = dto.to_model();
    let result = interactor.create_user(&user).await;
    match result {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(err) => handle_failure(err),
    }
}
