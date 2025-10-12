use anyhow::Result;
use sqlx::{Error, PgPool};
use std::future::Future;
use uuid::Uuid;

use crate::domain::models::rbac::UserWithRoles;
use crate::domain::models::user::{
    OAuthUserCreate, User, UserByEmail, UserCreate, UserUpdate, UserWithCountry,
};

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

    // OAuth-related methods
    fn get_user_by_email(
        &self,
        pool: &PgPool,
        email: &str,
    ) -> impl Future<Output = Result<UserByEmail, Error>>;

    fn get_user_by_provider_id(
        &self,
        pool: &PgPool,
        provider_id: &str,
        provider_name: &str,
    ) -> impl Future<Output = Result<UserByEmail, Error>>;

    fn create_oauth_user(
        &self,
        pool: &PgPool,
        user: &OAuthUserCreate,
    ) -> impl Future<Output = Result<User, Error>>;

    fn update_oauth_info(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        provider_id: &str,
        provider_name: &str,
    ) -> impl Future<Output = Result<(), Error>>;

    // RBAC-related methods
    fn get_user_with_roles(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> impl Future<Output = Result<UserWithRoles, Error>>;
}
