use crate::common::failure::domain::failure::{Failure, FailureType};

pub fn get_user_already_verified_error() -> Failure {
    return Failure {
        error_type: FailureType::Conflict,
        args: None,
        code: "error.email_already_verified".to_string(),
        message: "Email is already verified".to_string(),
    };
}
