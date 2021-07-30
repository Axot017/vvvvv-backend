use crate::{
    common::failure::domain::failure::Failure,
    features::profile::{
        domain::{create_user_model::CreateUserModel, user::User},
        interactors::profile_interactor::ProfileRepository,
    },
};
use async_trait::async_trait;

pub struct ProfileRepositoryImpl;

impl ProfileRepositoryImpl {
    pub fn new() -> ProfileRepositoryImpl {
        ProfileRepositoryImpl {}
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure> {
        let _ = user;
        Ok(())
    }

    async fn get_user_by_uuid(&self, uuid: &String) -> Result<User, Failure> {
        let _ = uuid;
        todo!()
    }
}
