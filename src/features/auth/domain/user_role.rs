#[derive(Debug, PartialEq)]
pub enum UserRole {
    USER,
    MODERATOR,
    ADMIN,
}

impl From<&str> for UserRole {
    fn from(str: &str) -> Self {
        return match str {
            "USER" => UserRole::USER,
            "MODERATOR" => UserRole::MODERATOR,
            "ADMIN" => UserRole::ADMIN,
            _ => UserRole::USER,
        };
    }
}

impl Into<String> for UserRole {
    fn into(self) -> String {
        return match self {
            UserRole::USER => "USER".to_string(),
            UserRole::MODERATOR => "MODERATOR".to_string(),
            UserRole::ADMIN => "ADMIN".to_string(),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_get_role_from_str() {
        let user: UserRole = UserRole::from("USER");
        let moderator: UserRole = UserRole::from("MODERATOR");
        let admin: UserRole = UserRole::from("ADMIN");

        assert_eq!(user, UserRole::USER);
        assert_eq!(moderator, UserRole::MODERATOR);
        assert_eq!(admin, UserRole::ADMIN);
    }

    #[test]
    fn should_turn_into_string() {
        let user: String = UserRole::USER.into();
        let moderator: String = UserRole::MODERATOR.into();
        let admin: String = UserRole::ADMIN.into();

        assert_eq!(user, "USER");
        assert_eq!(moderator, "MODERATOR");
        assert_eq!(admin, "ADMIN");
    }
}
