use chrono::{DateTime, Utc};

use crate::features::{
    auth::domain::{auth_data::AuthData, user_role::UserRole},
    profile::domain::user::User,
};

#[derive(Queryable, Debug, PartialEq, Clone)]
pub struct ProfileEntity {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub avatar_id: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Into<User> for ProfileEntity {
    fn into(self) -> User {
        return User {
            id: self.id,
            username: self.name,
            email: self.email,
            avatar_id: self.avatar_id,
            birthday: self.birthday,
            verified_at: self.email_confirmed_at,
            updated_at: self.updated_at,
            created_at: self.created_at,
        };
    }
}

impl Into<AuthData> for ProfileEntity {
    fn into(self) -> AuthData {
        let role = match self.role.as_str() {
            "ADMIN" => UserRole::ADMIN,
            "MODERATOR" => UserRole::MODERATOR,
            _ => UserRole::USER,
        };
        return AuthData {
            email: self.email,
            id: self.id,
            password: self.password,
            user_role: role,
            username: self.name,
            verified_at: self.email_confirmed_at,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_entity_to_auth_data_model() {
        let now = Utc::now();
        let profile_entity = ProfileEntity {
            id: 1,
            email: "email".to_string(),
            name: "name".to_string(),
            password: "password".to_string(),
            role: "ADMIN".to_string(),
            avatar_id: None,
            birthday: None,
            email_confirmed_at: Some(now.clone()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let result: AuthData = profile_entity.into();

        assert_eq!(result.id, 1);
        assert_eq!(result.email.as_str(), "email");
        assert_eq!(result.username.as_str(), "name");
        assert_eq!(result.password.as_str(), "password");
        assert_eq!(result.user_role, UserRole::ADMIN)
    }

    #[test]
    fn should_map_entity_to_model() {
        let now = Utc::now();
        let profile_entity = ProfileEntity {
            id: 1,
            email: "email".to_string(),
            name: "name".to_string(),
            password: "password".to_string(),
            role: "USER".to_string(),
            avatar_id: None,
            birthday: None,
            email_confirmed_at: Some(now.clone()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let user: User = profile_entity.into();

        assert_eq!(user.avatar_id, None);
        assert_eq!(user.created_at, now);
        assert_eq!(user.updated_at, now);
        assert_eq!(user.birthday, None);
        assert_eq!(user.verified_at, Some(now));
        assert_eq!(user.email, "email".to_string());
        assert_eq!(user.username, "name".to_string());
    }
}
