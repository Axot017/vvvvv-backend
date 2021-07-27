#[derive(Debug, PartialEq, Clone)]
pub struct CreateUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
}
