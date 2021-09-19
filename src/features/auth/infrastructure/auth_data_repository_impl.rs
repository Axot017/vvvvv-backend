use crate::features::auth::errors::auth_errors::get_invalid_credentials_error;
use crate::features::profile::infrastructure::entities::profile_entity::ProfileEntity;
use crate::schema::profile::dsl::*;
use crate::{
    common::failure::domain::failure::Failure,
    features::auth::{
        domain::auth_data::AuthData, errors::auth_errors::get_db_connection_error,
        interactors::auth_interactor::AuthDataRepository,
    },
};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel::{QueryDsl, RunQueryDsl};
use r2d2::{Pool, PooledConnection};

pub struct AuthDataRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AuthDataRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> AuthDataRepositoryImpl {
        return AuthDataRepositoryImpl { pool };
    }

    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Failure> {
        let pool = self.pool.get();
        return match pool {
            Ok(pool) => Ok(pool),
            _ => Err(get_db_connection_error()),
        };
    }
}

#[async_trait]
impl AuthDataRepository for AuthDataRepositoryImpl {
    async fn get_auth_data(&self, login: &String) -> Result<AuthData, Failure> {
        let connection = self.get_connection()?;
        let result = profile
            .filter(email.eq(login))
            .or_filter(name.eq(login))
            .get_result::<ProfileEntity>(&connection);

        return match result {
            Ok(user) => Ok(user.into()),
            Err(_) => Err(get_invalid_credentials_error()),
        };
    }
}
