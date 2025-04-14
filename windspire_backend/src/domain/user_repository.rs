use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use super::models::user::User;

pub(crate) trait UserRepository {
    fn get_user_by_id(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<User, Error>>;

    fn get_users(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<User>, Error>>;

    fn insert_user(
        &self,
        conn: &PgPool,
        user: User,
    ) -> impl std::future::Future<Output = Result<(), Error>>;
}
