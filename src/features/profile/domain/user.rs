use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub avatar_code: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
