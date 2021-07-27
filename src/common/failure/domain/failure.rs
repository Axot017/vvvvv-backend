use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum FailureType {
    Validation,
    Authentication,
    Forbidden,
    NotFound,
    Conflict,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Failure {
    pub error_type: FailureType,
    pub message: String,
    pub code: String,
    pub args: Option<HashMap<String, String>>,
}
