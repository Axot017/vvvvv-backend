use crate::schema::profile::dsl::*;
use crate::{
    common::failure::domain::failure::Failure,
    features::profile::{
        domain::{create_user_model::CreateUserModel, user::User},
        errors::profile_errors::{
            get_db_connection_error, get_unique_violation_error, get_unknown_user_creation_error,
            get_unknown_user_update_error, get_user_not_found_error,
        },
        infrastructure::entities::profile_entity::ProfileEntity,
        interactors::profile_interactor::ProfileRepository,
    },
};
use async_trait::async_trait;
use diesel::result::Error::DatabaseError;
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

use super::entities::{new_profile::NewProfile, profile_changeset::ProfileChangeset};

const UNIQUE_USERNAME_CONSTRAINT: &str = "profile_name_key";
const UNIQUE_EMAIL_CONSTRAINT: &str = "profile_email_key";

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
        use crate::schema::profile;

        let new_user = NewProfile::from(user.to_owned());
        let connection = self.get_connection()?;
        let result = diesel::insert_into(profile::table)
            .values(&new_user)
            .execute(&connection);

        return match result {
            Ok(_) => Ok(()),
            Err(error) => match error {
                DatabaseError(error, info) => match error {
                    DatabaseErrorKind::UniqueViolation => {
                        let column = match info.constraint_name().unwrap_or("") {
                            UNIQUE_USERNAME_CONSTRAINT => "username",
                            UNIQUE_EMAIL_CONSTRAINT => "email",
                            _ => "",
                        };
                        Err(get_unique_violation_error(column.to_string()))
                    }
                    _ => Err(get_unknown_user_creation_error()),
                },
                _ => Err(get_unknown_user_creation_error()),
            },
        };
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
        let connection = self.get_connection()?;
        let changeset = ProfileChangeset::from(user.to_owned());
        let target = profile.filter(id.eq(user.id));
        let result = diesel::update(target).set(&changeset).execute(&connection);

        return match result {
            Ok(_) => Ok(()),
            Err(_) => Err(get_unknown_user_update_error()),
        };
    }
}
