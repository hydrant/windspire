use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use super::models::country::{Country, CountryCreate};


pub(crate) trait CountryRepository {
    fn get_country_by_id(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;

    fn get_countries(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<Country>, Error>>;

    fn insert_country(
        &self,
        conn: &PgPool,
        user: CountryCreate,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;
}
