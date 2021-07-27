use std::collections::HashMap;

#[allow(dead_code)]
pub enum FailureType {
    Validation,
    Authentication,
    Forbidden,
    NotFound,
    Conflict,
    Unknown,
}

pub struct Failure {
    pub error_type: FailureType,
    pub message: String,
    pub code: String,
    pub args: Option<HashMap<String, String>>,
}
