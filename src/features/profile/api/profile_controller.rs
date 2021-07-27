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
    features::profile::{
        infrastructure::profile_repository_impl::ProfileRepositoryImpl,
        interactors::profile_interactor::ProfileInteractor,
    },
};

use super::dtos::create_user_dto::CreateUserDto;

type Interceptor = ProfileInteractor<ProfileRepositoryImpl>;

pub fn configure_profile_controller(interactor: Arc<Interceptor>, config: &mut ServiceConfig) {
    config.service(
        web::scope("/profile")
            .app_data(Data::from(interactor))
            .service(get_current_user)
            .service(create_user),
    );
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
