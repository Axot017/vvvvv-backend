use crate::{
    common::failure::domain::failure::Failure,
    features::profile::domain::create_user_model::CreateUserModel,
};
use async_trait::async_trait;

#[async_trait]
pub trait ProfileRepository {
    async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure>;
}

#[async_trait]
pub trait VerificationKeysStorage {
    async fn save_verification_code(
        &self,
        email: &String,
        verification_code: &String,
    ) -> Result<(), Failure>;

    async fn get_verification_code(&self, email: &String) -> Result<String, Failure>;
}

#[async_trait]
pub trait CodeGenerator {
    async fn generate(&self) -> String;
}

pub struct ProfileInteractor<T, Y, U> {
    profile_repository: T,
    code_generator: Y,
    verification_keys_storage: U,
}

impl<T, Y, U> ProfileInteractor<T, Y, U>
where
    T: 'static + ProfileRepository,
    Y: 'static + CodeGenerator,
    U: 'static + VerificationKeysStorage,
{
    pub fn new(
        profile_repository: T,
        code_generator: Y,
        verification_keys_storage: U,
    ) -> ProfileInteractor<T, Y, U> {
        ProfileInteractor {
            profile_repository,
            code_generator,
            verification_keys_storage,
        }
    }

    pub async fn create_user(&self, user: &CreateUserModel) -> Result<(), Failure> {
        self.profile_repository.save_user(user).await?;
        let code = self.code_generator.generate().await;
        self.verification_keys_storage
            .save_verification_code(&user.email, &code)
            .await
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::*;
    use mockall::*;

    use crate::common::failure::domain::failure::FailureType;

    use super::*;

    mock! {
        CodeGenerator {}

        #[async_trait]
        impl CodeGenerator for CodeGenerator {
            async fn generate(&self) -> String;
        }
    }

    mock! {
        VerificationKeysStorage {}

        #[async_trait]
        impl VerificationKeysStorage for VerificationKeysStorage {
            async fn save_verification_code(
                &self,
                email: &String,
                verification_code: &String,
            ) -> Result<(), Failure>;

            async fn get_verification_code(&self, email: &String) -> Result<String, Failure>;
        }
    }

    mock! {
        ProfileRespository {}

        #[async_trait]
        impl ProfileRepository for ProfileRespository {
            async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure>;
        }
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_save_user() {
        let mut repo = MockProfileRespository::new();
        let storage = MockVerificationKeysStorage::new();
        let code_generator = MockCodeGenerator::new();
        let user = CreateUserModel {
            email: "test@test.com".to_string(),
            password: "testPassword".to_string(),
            username: "testName".to_string(),
        };
        let failure = Failure {
            error_type: FailureType::Authentication,
            args: None,
            code: "cdsaf".to_string(),
            message: "as".to_string(),
        };
        let clone = failure.clone();
        repo.expect_save_user()
            .with(predicate::eq(user.clone()))
            .return_once(move |_| Err(clone));
        let interactor = ProfileInteractor::new(repo, code_generator, storage);

        let result = interactor.create_user(&user).await;

        assert_eq!(result, Err(failure.clone()))
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_save_code() {
        let mut repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let mut code_generator = MockCodeGenerator::new();
        let test_code = "test_code".to_string();
        let test_code_clone = test_code.clone();
        let user = CreateUserModel {
            email: "test@test.com".to_string(),
            password: "testPassword".to_string(),
            username: "testName".to_string(),
        };
        let failure = Failure {
            error_type: FailureType::Authentication,
            args: None,
            code: "cdsaf".to_string(),
            message: "as".to_string(),
        };
        let copy = failure.clone();
        repo.expect_save_user()
            .with(predicate::eq(user.clone()))
            .return_once(move |_| Ok(()));
        code_generator
            .expect_generate()
            .return_once(move || test_code_clone);

        storage
            .expect_save_verification_code()
            .with(predicate::eq(user.clone().email), predicate::eq(test_code))
            .return_once(move |_, __| Err(copy));
        let interactor = ProfileInteractor::new(repo, code_generator, storage);

        let result = interactor.create_user(&user).await;

        assert_eq!(result, Err(failure.clone()))
    }
}
