use crate::{
    common::failure::domain::failure::Failure,
    features::profile::domain::create_user_model::CreateUserModel,
};
use async_trait::async_trait;

#[async_trait]
pub trait ProfileRepository {
    async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure>;
}

pub struct ProfileInteractor<T> {
    profile_repository: T,
}

impl<T> ProfileInteractor<T>
where
    T: 'static + ProfileRepository,
{
    pub fn new(profile_repository: T) -> ProfileInteractor<T> {
        ProfileInteractor { profile_repository }
    }

    pub async fn create_user(&self, user: &CreateUserModel) -> Result<(), Failure> {
        self.profile_repository.save_user(user).await
    }
}
