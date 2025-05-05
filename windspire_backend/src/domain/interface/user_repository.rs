use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::models::user::{User, UserCreate, UserUpdate, UserWithCountry};

pub(crate) trait UserRepository {
    fn get_user_by_id(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> impl Future<Output = Result<User, Error>>;

    fn get_users(&self, pool: &PgPool)
    -> impl Future<Output = Result<Vec<UserWithCountry>, Error>>;

    fn insert_user(
        &self,
        conn: &PgPool,
        user: UserCreate,
    ) -> impl Future<Output = Result<User, Error>>;

    fn delete_user(&self, conn: &PgPool, user_id: Uuid) -> impl Future<Output = Result<(), Error>>;

    fn update_user(
        &self,
        conn: &PgPool,
        user_id: Uuid,
        user: UserUpdate,
    ) -> impl Future<Output = Result<User, Error>>;
}
