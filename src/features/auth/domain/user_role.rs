#[derive(Debug, PartialEq, Clone)]
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

impl From<&UserRole> for String {
    fn from(role: &UserRole) -> Self {
        return match role {
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
    fn should_get_string_from_role() {
        let user: String = String::from(&UserRole::USER);
        let moderator: String = String::from(&UserRole::MODERATOR);
        let admin: String = String::from(&UserRole::ADMIN);

        assert_eq!(user, "USER");
        assert_eq!(moderator, "MODERATOR");
        assert_eq!(admin, "ADMIN");
    }
}
