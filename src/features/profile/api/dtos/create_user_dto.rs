use serde::Deserialize;

use crate::features::profile::domain::create_user_model::CreateUserModel;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    username: String,
    email: String,
    password: String,
}

impl Into<CreateUserModel> for CreateUserDto {
    fn into(self) -> CreateUserModel {
        return CreateUserModel {
            username: self.username,
            email: self.email,
            password: self.password,
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
        let dto_clone = dto.clone();

        let result: CreateUserModel = dto.into();

        assert_eq!(dto_clone.password, result.password);
        assert_eq!(dto_clone.username, result.username);
        assert_eq!(dto_clone.email, result.email);
    }
}
