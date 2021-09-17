use crate::{features::profile::domain::create_user_model::CreateUserModel, schema::profile};

#[derive(Insertable, Debug, PartialEq, Clone)]
#[table_name = "profile"]
pub struct NewProfile {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<CreateUserModel> for NewProfile {
    fn from(user: CreateUserModel) -> Self {
        return NewProfile {
            email: user.email,
            name: user.username,
            password: user.password,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_map_model_to_entity() {
        let model = CreateUserModel {
            email: "email".to_string(),
            password: "password".to_string(),
            username: "username".to_string(),
        };

        let result = NewProfile::from(model);

        assert_eq!(result.email, "email".to_string());
        assert_eq!(result.password, "password".to_string());
        assert_eq!(result.name, "username".to_string());
    }
}
