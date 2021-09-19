use std::collections::HashMap;

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
        code: "error.unknown_error".to_string(),
        message: "Failed to connect to db".to_string(),
    };
}

pub fn get_unknown_user_creation_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.unknown_user_creation_error".to_string(),
        message: "Unknown user creation error".to_string(),
    };
}

pub fn get_unknown_user_update_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.unknown_user_update_error".to_string(),
        message: "Unknown user update error".to_string(),
    };
}

pub fn get_unique_violation_error(field: String) -> Failure {
    let mut args = HashMap::new();
    args.insert("field".to_string(), field.clone());

    return Failure {
        error_type: FailureType::Conflict,
        code: "error.unique_violation_error".to_string(),
        message: format!("Field '{}' must be unique", field).to_string(),
        args: Some(args),
    };
}

pub fn get_redis_connection_error() -> Failure {
    return Failure {
        error_type: FailureType::Unknown,
        args: None,
        code: "error.unknown_error".to_string(),
        message: "Failed to connect to redis".to_string(),
    };
}

pub fn get_invalid_verification_code() -> Failure {
    return Failure {
        error_type: FailureType::NotFound,
        args: None,
        code: "error.invalid_verification_code".to_string(),
        message: "Invalide email verification code".to_string(),
    };
}
