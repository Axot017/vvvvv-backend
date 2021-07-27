mod common;
mod config;
mod features;

use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use features::profile::{
    api::profile_controller::configure_profile_controller,
    infrastructure::profile_repository_impl::ProfileRepositoryImpl,
    interactors::profile_interactor::ProfileInteractor,
};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let profile_repostory = ProfileRepositoryImpl::new();
    let profile_interactor = Arc::new(ProfileInteractor::new(profile_repostory));
    HttpServer::new(move || {
        App::new().service(web::scope("/api").configure(|cfg| {
            configure_profile_controller(profile_interactor.clone(), cfg);
        }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
