use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_invalid_credentials_error() -> Failure {
    return Failure {
        args: None,
        code: "error.invalid_credentials".to_string(),
        message: "Invalid credentials error".to_string(),
        error_type: FailureType::Authentication,
    };
}
