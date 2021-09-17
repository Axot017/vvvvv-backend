use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_user_already_verified_error() -> Failure {
    return Failure {
        error_type: FailureType::Conflict,
        args: None,
        code: "error.email_already_verified".to_string(),
        message: "Email is already verified".to_string(),
    };
}

pub fn get_user_not_found_error() -> Failure {
    return Failure {
        error_type: FailureType::NotFound,
        args: None,
        code: "error.user_not_found".to_string(),
        message: "User not found".to_string(),
    };
}

pub fn get_db_connection_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.failed_to_connect_to_db".to_string(),
        message: "Failed to connect to db".to_string(),
    };
}
