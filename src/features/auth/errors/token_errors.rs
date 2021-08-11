use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_token_generating_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.token_generating".to_string(),
        message: "Unknown error when generating token".to_string(),
    };
}

pub fn get_invalid_access_token_error() -> Failure {
    return Failure {
        error_type: FailureType::Authentication,
        args: None,
        code: "error.invalid_access_token".to_string(),
        message: "Invalid access token".to_string(),
    };
}

pub fn get_invalid_refresh_token_error() -> Failure {
    return Failure {
        error_type: FailureType::Authentication,
        args: None,
        code: "error.invalid_refresh_token".to_string(),
        message: "Invalid refresh token".to_string(),
    };
}
