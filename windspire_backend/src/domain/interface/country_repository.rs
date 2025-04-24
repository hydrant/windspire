use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::models::country::{Country, CountryCreate, CountryUpdate};

pub(crate) trait CountryRepository {
    fn get_country_by_id(
        &self,
        pool: &PgPool,
        country_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;

    fn get_country_by_code(
        &self,
        pool: &PgPool,
        country_code: String,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;

    fn get_countries(
        &self,
        pool: &PgPool,
    ) -> impl std::future::Future<Output = Result<Vec<Country>, Error>>;

    fn insert_country(
        &self,
        conn: &PgPool,
        country: CountryCreate,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;

    fn delete_country(
        &self,
        conn: &PgPool,
        country_id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn update_country(
        &self,
        conn: &PgPool,
        country_id: Uuid,
        country_update: CountryUpdate,
    ) -> impl std::future::Future<Output = Result<Country, Error>>;
}
