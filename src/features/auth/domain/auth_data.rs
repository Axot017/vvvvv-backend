use chrono::{DateTime, Utc};

use super::user_role::UserRole;

#[derive(Debug, PartialEq, Clone)]
pub struct AuthData {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_role: UserRole,
    pub verified_at: Option<DateTime<Utc>>,
}
