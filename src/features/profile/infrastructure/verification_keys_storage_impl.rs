use crate::features::profile::errors::profile_errors::{
    get_invalid_verification_code, get_redis_connection_error,
};
use crate::{
    common::failure::domain::failure::Failure, config::profile_config::ProfileConfig,
    features::profile::interactors::profile_interactor::VerificationKeysStorage,
};
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisError};

pub struct VerificationKeysStorageImpl {
    redis_connection: MultiplexedConnection,
    config: ProfileConfig,
}

impl VerificationKeysStorageImpl {
    pub fn new(redis: MultiplexedConnection, config: ProfileConfig) -> VerificationKeysStorageImpl {
        return VerificationKeysStorageImpl {
            redis_connection: redis,
            config,
        };
    }
}

#[async_trait]
impl VerificationKeysStorage for VerificationKeysStorageImpl {
    async fn save_verification_code(
        &self,
        email: &String,
        verification_code: &String,
    ) -> Result<(), Failure> {
        let exp = self.config.email_verification_key_exp;

        let result: Result<(), RedisError> = self
            .redis_connection
            .clone()
            .set_ex(verification_code, email, exp as usize)
            .await;

        return match result {
            Ok(_) => Ok(()),
            Err(_) => Err(get_redis_connection_error()),
        };
    }

    async fn get_email_by_code(&self, verification_code: &String) -> Result<String, Failure> {
        let result: Result<String, RedisError> =
            self.redis_connection.clone().get(verification_code).await;

        return match result {
            Ok(email) => Ok(email),
            Err(_) => Err(get_invalid_verification_code()),
        };
    }
}
