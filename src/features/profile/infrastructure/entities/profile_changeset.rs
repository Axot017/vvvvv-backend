use crate::{features::profile::domain::user::User, schema::profile};
use chrono::{DateTime, Utc};

#[derive(AsChangeset)]
#[table_name = "profile"]
pub struct ProfileChangeset {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_id: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
}

impl From<User> for ProfileChangeset {
    fn from(user: User) -> Self {
        ProfileChangeset {
            avatar_id: user.avatar_id,
            birthday: user.birthday,
            email: Some(user.email),
            email_confirmed_at: user.verified_at,
            name: Some(user.username),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_map_model_to_changeset() {
        let now = Utc::now();
        let user = User {
            avatar_id: None,
            birthday: None,
            created_at: now,
            updated_at: now,
            email: "email".to_string(),
            id: 1,
            username: "username".to_string(),
            verified_at: Some(now),
        };

        let result: ProfileChangeset = user.into();

        assert_eq!(result.avatar_id, None);
        assert_eq!(result.birthday, None);
        assert_eq!(result.email, Some("email".to_string()));
        assert_eq!(result.email_confirmed_at, Some(now));
        assert_eq!(result.name, Some("username".to_string()));
    }
}
