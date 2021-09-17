use crate::{
    common::failure::domain::failure::Failure,
    features::profile::{
        domain::{create_user_model::CreateUserModel, user::User},
        errors::profile_errors::get_user_already_verified_error,
    },
};
use async_trait::async_trait;
use chrono::Utc;

#[async_trait]
pub trait ProfileRepository {
    async fn get_user_by_id(&self, uuid: &i64) -> Result<User, Failure>;

    async fn get_user_by_email(&self, email: &String) -> Result<User, Failure>;

    async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure>;

    async fn update_user(&self, user: &User) -> Result<(), Failure>;
}

#[async_trait]
pub trait VerificationMailer {
    async fn send_verification_code(&self, email: &String, code: &String) -> Result<(), Failure>;
}

#[async_trait]
pub trait VerificationKeysStorage {
    async fn save_verification_code(
        &self,
        email: &String,
        verification_code: &String,
    ) -> Result<(), Failure>;

    async fn get_email_by_code(&self, verification_code: &String) -> Result<String, Failure>;
}

#[async_trait]
pub trait CodeGenerator {
    async fn generate(&self) -> String;
}

pub struct ProfileInteractor<T, Y, U, I> {
    profile_repository: T,
    code_generator: Y,
    verification_keys_storage: U,
    mailer: I,
}

impl<T, Y, U, I> ProfileInteractor<T, Y, U, I>
where
    T: ProfileRepository,
    Y: CodeGenerator,
    U: VerificationKeysStorage,
    I: VerificationMailer,
{
    pub fn new(
        profile_repository: T,
        code_generator: Y,
        verification_keys_storage: U,
        mailer: I,
    ) -> ProfileInteractor<T, Y, U, I> {
        ProfileInteractor {
            profile_repository,
            code_generator,
            verification_keys_storage,
            mailer,
        }
    }

    pub async fn create_user(&self, user: &CreateUserModel) -> Result<(), Failure> {
        self.profile_repository.save_user(user).await?;
        self.send_verification_email(&user.email).await
    }

    pub async fn verify_email(&self, code: &String) -> Result<(), Failure> {
        let email = self
            .verification_keys_storage
            .get_email_by_code(code)
            .await?;
        let mut user = self.profile_repository.get_user_by_email(&email).await?;
        user.verified_at = Some(Utc::now());
        self.profile_repository.update_user(&user).await
    }

    pub async fn get_user(&self, id: &i64) -> Result<User, Failure> {
        self.profile_repository.get_user_by_id(id).await
    }

    pub async fn resend_email(&self, email: &String) -> Result<(), Failure> {
        let user = self.profile_repository.get_user_by_email(email).await?;
        if user.verified_at.is_some() {
            return Err(get_user_already_verified_error());
        }
        self.send_verification_email(&user.email).await
    }

    async fn send_verification_email(&self, email: &String) -> Result<(), Failure> {
        let code = self.code_generator.generate().await;
        self.verification_keys_storage
            .save_verification_code(email, &code)
            .await?;
        self.mailer.send_verification_code(email, &code).await
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
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

            async fn get_email_by_code(&self, verification_code: &String) -> Result<String, Failure>;
        }
    }

    mock! {
        ProfileRespository {}

        #[async_trait]
        impl ProfileRepository for ProfileRespository {
            async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure>;

            async fn get_user_by_id(&self, uuid: &i64) -> Result<User, Failure>;

            async fn get_user_by_email(&self, email: &String) -> Result<User, Failure>;

            async fn update_user(&self, user: &User) -> Result<(), Failure>;
        }
    }

    mock! {
        VerificationMailer {}

        #[async_trait]
        impl VerificationMailer for VerificationMailer {
            async fn send_verification_code(&self, email: &String, code: &String) -> Result<(), Failure>;
        }
    }

    #[actix_rt::test]
    async fn should_update_user_verification_date() {
        let test_code = "test_code".to_string();
        let user = User {
            verified_at: None,
            avatar_id: None,
            id: 1,
            email: "test_email".to_string(),
            username: "test_username".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let user_clone = user.clone();
        let email = (&user).email.clone();

        let mut repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let code_generator = MockCodeGenerator::new();
        let mailer = MockVerificationMailer::new();

        repo.expect_get_user_by_email()
            .with(predicate::eq(email.clone()))
            .return_once(move |_| Ok(user_clone));
        storage
            .expect_get_email_by_code()
            .with(predicate::eq(test_code.clone()))
            .return_once(move |_| Ok(email));
        repo.expect_update_user()
            .with(predicate::function(|user: &User| {
                user.verified_at.is_some()
            }))
            .return_once(|_| Ok(()));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.verify_email(&test_code).await;

        assert_eq!(result, Ok(()));
    }

    #[actix_rt::test]
    async fn should_return_user() {
        let user = User {
            verified_at: None,
            avatar_id: None,
            id: 1,
            email: "test_email".to_string(),
            username: "test_username".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let user_clone = user.clone();

        let mut repo = MockProfileRespository::new();
        let storage = MockVerificationKeysStorage::new();
        let code_generator = MockCodeGenerator::new();
        let mailer = MockVerificationMailer::new();

        repo.expect_get_user_by_id()
            .with(predicate::eq((&user).id.clone()))
            .return_once(|_| Ok(user_clone));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.get_user(&user.id).await;

        assert_eq!(result, Ok(user));
    }

    #[actix_rt::test]
    async fn should_resend_email() {
        let test_code = "test_code".to_string();
        let test_code_clone = test_code.clone();
        let user = User {
            verified_at: None,
            avatar_id: None,
            id: 1,
            email: "test_email".to_string(),
            username: "test_username".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let user_clone = user.clone();

        let mut repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let mut code_generator = MockCodeGenerator::new();
        let mut mailer = MockVerificationMailer::new();

        repo.expect_get_user_by_email()
            .with(predicate::eq(user.email.clone()))
            .return_once(move |_| Ok(user_clone));
        code_generator
            .expect_generate()
            .return_once(move || test_code_clone);
        storage
            .expect_save_verification_code()
            .with(
                predicate::eq((&user).email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Ok(()));
        mailer
            .expect_send_verification_code()
            .with(
                predicate::eq((&user).email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Ok(()));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.resend_email(&user.email).await;

        assert_eq!(result, Ok(()));
    }

    #[actix_rt::test]
    async fn should_return_error_if_email_is_alredy_verified() {
        let user = User {
            verified_at: Some(Utc::now()),
            avatar_id: None,
            id: 1,
            email: "test_email".to_string(),
            username: "test_username".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let user_clone = user.clone();

        let mut repo = MockProfileRespository::new();
        let storage = MockVerificationKeysStorage::new();
        let code_generator = MockCodeGenerator::new();
        let mailer = MockVerificationMailer::new();

        repo.expect_get_user_by_email()
            .with(predicate::eq(user.email.clone()))
            .return_once(move |_| Ok(user_clone));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.resend_email(&user.email).await;

        assert_eq!(result, Err(get_user_already_verified_error()));
    }

    #[actix_rt::test]
    async fn should_return_ok() {
        let test_code = "test_code".to_string();
        let test_code_clone = test_code.clone();
        let user = CreateUserModel {
            email: "test@test.com".to_string(),
            password: "testPassword".to_string(),
            username: "testName".to_string(),
        };

        let mut repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let mut code_generator = MockCodeGenerator::new();
        let mut mailer = MockVerificationMailer::new();

        repo.expect_save_user()
            .with(predicate::eq(user.clone()))
            .return_once(|_| Ok(()));
        code_generator
            .expect_generate()
            .return_once(move || test_code_clone);
        storage
            .expect_save_verification_code()
            .with(
                predicate::eq((&user).email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Ok(()));
        mailer
            .expect_send_verification_code()
            .with(
                predicate::eq((&user).email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Ok(()));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.create_user(&user).await;

        assert_eq!(result, Ok(()));
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_save_user() {
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

        let mut repo = MockProfileRespository::new();
        let storage = MockVerificationKeysStorage::new();
        let code_generator = MockCodeGenerator::new();
        let mailer = MockVerificationMailer::new();

        repo.expect_save_user()
            .with(predicate::eq(user.clone()))
            .return_once(move |_| Err(clone));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.create_user(&user).await;

        assert_eq!(result, Err(failure.clone()))
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_save_code() {
        let test_code = "test_code".to_string();
        let test_code_clone = test_code.clone();
        let email = "test@email.com".to_string();
        let failure = Failure {
            error_type: FailureType::Authentication,
            args: None,
            code: "cdsaf".to_string(),
            message: "as".to_string(),
        };
        let copy = failure.clone();

        let repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let mut code_generator = MockCodeGenerator::new();
        let mailer = MockVerificationMailer::new();

        code_generator
            .expect_generate()
            .return_once(move || test_code_clone);
        storage
            .expect_save_verification_code()
            .with(predicate::eq(email.clone()), predicate::eq(test_code))
            .return_once(move |_, __| Err(copy));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.send_verification_email(&email).await;

        assert_eq!(result, Err(failure.clone()))
    }

    #[actix_rt::test]
    async fn should_return_error_if_cannot_send_email() {
        let test_code = "test_code".to_string();
        let test_code_clone = test_code.clone();
        let email = "test@email.com".to_string();
        let failure = Failure {
            error_type: FailureType::Authentication,
            args: None,
            code: "cdsaf".to_string(),
            message: "as".to_string(),
        };
        let copy = failure.clone();

        let repo = MockProfileRespository::new();
        let mut storage = MockVerificationKeysStorage::new();
        let mut code_generator = MockCodeGenerator::new();
        let mut mailer = MockVerificationMailer::new();

        code_generator
            .expect_generate()
            .return_once(move || test_code_clone);
        storage
            .expect_save_verification_code()
            .with(
                predicate::eq(email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Ok(()));
        mailer
            .expect_send_verification_code()
            .with(
                predicate::eq(email.clone()),
                predicate::eq(test_code.clone()),
            )
            .return_once(move |_, __| Err(copy));

        let interactor = ProfileInteractor::new(repo, code_generator, storage, mailer);

        let result = interactor.send_verification_email(&email).await;

        assert_eq!(result, Err(failure.clone()))
    }
}
