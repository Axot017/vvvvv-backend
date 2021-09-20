use actix_web::{http::StatusCode, ResponseError};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

use super::{
    domain::failure::{Failure, FailureType},
    failure_handler::handle_failure,
};

#[derive(Debug, Serialize)]
pub struct FailureDto {
    message: String,
    code: String,
    args: HashMap<String, String>,
}

impl FailureDto {
    pub fn from_failure(failure: Failure) -> FailureDto {
        let args = failure.args.unwrap_or(HashMap::new());
        FailureDto {
            message: failure.message,
            code: failure.code,
            args,
        }
    }
}

impl Display for Failure {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Ok(());
    }
}

impl ResponseError for Failure {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            FailureType::Validation => StatusCode::BAD_REQUEST,
            FailureType::Authentication => StatusCode::UNAUTHORIZED,
            FailureType::Forbidden => StatusCode::FORBIDDEN,
            FailureType::NotFound => StatusCode::NOT_FOUND,
            FailureType::Conflict => StatusCode::CONFLICT,
            FailureType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        handle_failure(self.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::common::failure::domain::failure::FailureType;

    use super::*;

    #[test]
    fn should_get_dto_from_failure() {
        let mut args: HashMap<String, String> = HashMap::new();
        args.insert("test_key".to_string(), "test_value".to_string());

        let failure = Failure {
            code: "test_code".to_string(),
            error_type: FailureType::Conflict,
            message: "test_message".to_string(),
            args: Some(args),
        };

        let result = FailureDto::from_failure(failure);

        assert_eq!(result.code, "test_code");
        assert_eq!(result.message, "test_message");
        assert_eq!(result.args["test_key"], "test_value");
    }

    #[test]
    fn should_return_empty_map_of_args() {
        let failure = Failure {
            code: "test_code".to_string(),
            error_type: FailureType::Conflict,
            message: "test_message".to_string(),
            args: None,
        };

        let result = FailureDto::from_failure(failure);

        assert_eq!(result.args, HashMap::new());
    }
}
