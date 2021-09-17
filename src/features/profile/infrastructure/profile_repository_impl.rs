use crate::{
    common::failure::domain::failure::Failure,
    features::profile::{
        domain::{create_user_model::CreateUserModel, user::User},
        errors::profile_errors::{get_db_connection_error, get_user_not_found_error},
        infrastructure::entities::profile_entity::ProfileEntity,
        interactors::profile_interactor::ProfileRepository,
    },
    schema,
};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};
use schema::profile::dsl::*;

pub struct ProfileRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ProfileRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> ProfileRepositoryImpl {
        ProfileRepositoryImpl { pool }
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
impl ProfileRepository for ProfileRepositoryImpl {
    async fn save_user(&self, user: &CreateUserModel) -> Result<(), Failure> {
        let _ = user;
        Ok(())
    }

    async fn get_user_by_id(&self, profile_id: &i64) -> Result<User, Failure> {
        let connection = self.get_connection()?;
        let result = profile
            .filter(id.eq(profile_id))
            .get_result::<ProfileEntity>(&connection);

        return match result {
            Ok(entity) => Ok(entity.into()),
            Err(_) => Err(get_user_not_found_error()),
        };
    }

    async fn get_user_by_email(&self, user_email: &String) -> Result<User, Failure> {
        let connection = self.get_connection()?;
        let result = profile
            .filter(email.eq(user_email))
            .get_result::<ProfileEntity>(&connection);

        return match result {
            Ok(entity) => Ok(entity.into()),
            Err(_) => Err(get_user_not_found_error()),
        };
    }

    async fn update_user(&self, user: &User) -> Result<(), Failure> {
        let _ = user;
        todo!()
    }
}
