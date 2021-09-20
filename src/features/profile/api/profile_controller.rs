use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    common::failure::failure_handler::handle_failure,
    features::{
        auth::{
            api::auth_middleware::verify_current_user, domain::current_user_data::CurrentUserData,
            infrastructure::password_manager_impl::PasswordManagerImpl,
        },
        mailer::mailer::Mailer,
        profile::{
            domain::create_user_model::CreateUserModel,
            errors::profile_errors::get_user_not_found_error,
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
    create_user_dto::CreateUserDto, resend_email_dto::ResendEmailDto, user_dto::UserDto,
    verify_email_dto::VerifyEmailDto,
};

type Interactor = ProfileInteractor<
    ProfileRepositoryImpl,
    VerificationCodeGenerator,
    VerificationKeysStorageImpl,
    Mailer,
    PasswordManagerImpl,
>;

pub fn configure_profile_controller(config: &mut ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(verify_current_user);
    config.service(
        web::scope("/profile")
            .service(
                web::scope("/me")
                    .wrap(auth_middleware)
                    .service(get_current_user),
            )
            .service(resend_email)
            .service(verify_user)
            .service(create_user),
    );
}

#[post("/email/resend")]
async fn resend_email(
    interactor: web::Data<Interactor>,
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
    interactor: web::Data<Interactor>,
    dto: web::Json<VerifyEmailDto>,
) -> impl Responder {
    let result = interactor.verify_email(&dto.code).await;
    match result {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(err) => handle_failure(err),
    }
}

#[get("")]
async fn get_current_user(
    interactor: web::Data<Interactor>,
    request: HttpRequest,
) -> impl Responder {
    let ext = request.extensions();
    let current_user = ext.get::<CurrentUserData>();
    match current_user {
        None => handle_failure(get_user_not_found_error()),
        Some(current_user) => {
            let result = interactor.get_user(&current_user.id).await;
            match result {
                Ok(user) => HttpResponse::Ok().json(UserDto::from(user)),
                Err(err) => handle_failure(err),
            }
        }
    }
}

#[post("")]
async fn create_user(
    interactor: web::Data<Interactor>,
    dto: web::Json<CreateUserDto>,
) -> impl Responder {
    let mut user: CreateUserModel = dto.into_inner().into();
    let result = interactor.create_user(&mut user).await;
    match result {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(err) => handle_failure(err),
    }
}
