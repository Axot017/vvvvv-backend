use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar_id: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
