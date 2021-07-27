use actix_web::{http::StatusCode, HttpResponse};

use crate::common::failure::domain::failure::FailureType;

use super::{domain::failure::Failure, failure_dto::FailureDto};

pub fn handle_failure(failure: Failure) -> HttpResponse {
    let code = match failure.error_type {
        FailureType::Validation => StatusCode::BAD_REQUEST,
        FailureType::Authentication => StatusCode::UNAUTHORIZED,
        FailureType::Forbidden => StatusCode::FORBIDDEN,
        FailureType::NotFound => StatusCode::NOT_FOUND,
        FailureType::Conflict => StatusCode::CONFLICT,
        FailureType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
    };
    let dto = FailureDto::from_failure(failure);
    HttpResponse::build(code).json(dto)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_map_failure_to_response() {
        let failure = Failure {
            code: "test_code".to_string(),
            error_type: FailureType::Conflict,
            message: "test_message".to_string(),
            args: None,
        };

        let result = handle_failure(failure);

        assert_eq!(result.status(), StatusCode::CONFLICT)
    }
}
