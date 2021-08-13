use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_client_secret_error() -> Failure {
    return Failure {
        args: None,
        error_type: FailureType::Forbidden,
        code: "error.invalid_client_secret".to_string(),
        message: "Invalid client secret".to_string(),
    };
}
