use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_password_verification_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.password_verification".to_string(),
        message: "Unknown error when veryfing password".to_string(),
    };
}

pub fn get_password_hashing_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.password_hashing".to_string(),
        message: "Unknown error when hashing password".to_string(),
    };
}
