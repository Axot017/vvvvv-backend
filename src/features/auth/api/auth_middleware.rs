use actix_web::{dev::ServiceRequest, web::Data, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    config::auth_config::AuthConfig,
    features::auth::{
        errors::auth_errors::get_unauthenticated_error,
        infrastructure::{
            auth_data_repository_impl::AuthDataRepositoryImpl,
            jwt_token_provider::JwtTokenProvider, password_manager_impl::PasswordManagerImpl,
        },
        interactors::auth_interactor::AuthInteractor,
    },
};

type Auth =
    AuthInteractor<PasswordManagerImpl, JwtTokenProvider, AuthDataRepositoryImpl, AuthConfig>;

// pub async fn inject_current_user_data(
//     req: ServiceRequest,
//     credentails: BearerAuth,
// ) -> Result<ServiceRequest, Error> {
//     let interactor = req.app_data::<Auth>();

//     match interactor {
//         None => Ok(req),
//         Some(interacotor) => {
//             let result = interacotor
//                 .validate_access_token(&credentails.token().to_string())
//                 .await;
//             match result {
//                 Err(_) => Ok(req),
//                 Ok(data) => {
//                     req.extensions_mut().insert(data);
//                     Ok(req)
//                 }
//             }
//         }
//     }
// }

pub async fn verify_current_user(
    req: ServiceRequest,
    credentails: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let interactor = req.app_data::<Data<Auth>>();
    match interactor {
        None => Err(Error::from(get_unauthenticated_error())),
        Some(interacotor) => {
            let result = interacotor
                .validate_access_token(&credentails.token().to_string())
                .await;
            match result {
                Err(_) => Err(Error::from(get_unauthenticated_error())),
                Ok(data) => {
                    req.extensions_mut().insert(data);
                    Ok(req)
                }
            }
        }
    }
}
