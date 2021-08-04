use serde::Deserialize;

use crate::features::profile::domain::create_user_model::CreateUserModel;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}

impl CreateUserDto {
    pub fn to_model(&self) -> CreateUserModel {
        return CreateUserModel {
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_dto_to_model() {
        let username = "TestUsername".to_string();
        let email = "test@email.com".to_string();
        let password = "testPassword".to_string();
        let dto = CreateUserDto {
            username,
            email,
            password,
        };

        let result = dto.to_model();

        assert_eq!(dto.username, result.username);
        assert_eq!(dto.email, result.email);
        assert_eq!(dto.password, result.password);
    }
}
