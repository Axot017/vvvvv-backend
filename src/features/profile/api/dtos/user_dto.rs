use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::features::profile::domain::user::User;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    id: i64,
    username: String,
    email: String,
    avatar_id: Option<String>,
    birthday: Option<DateTime<Utc>>,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        return UserDto {
            id: user.id,
            avatar_id: user.avatar_id,
            created_at: user.created_at,
            email: user.email,
            birthday: user.birthday,
            updated_at: user.updated_at,
            username: user.username,
            verified_at: user.verified_at,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_dto_to_model() {
        let now = Utc::now();
        let user = User {
            id: 1,
            email: "email".to_string(),
            username: "username".to_string(),
            birthday: None,
            avatar_id: None,
            verified_at: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let dto = UserDto::from(user);

        assert_eq!(dto.avatar_id, None);
        assert_eq!(dto.verified_at, None);
        assert_eq!(dto.birthday, None);
        assert_eq!(dto.created_at, now);
        assert_eq!(dto.updated_at, now);
        assert_eq!(dto.username, "username".to_string());
        assert_eq!(dto.email, "email".to_string());
        assert_eq!(dto.id, 1);
    }
}
