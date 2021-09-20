use super::user_role::UserRole;

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentUserData {
    pub id: i64,
    pub role: UserRole,
}
