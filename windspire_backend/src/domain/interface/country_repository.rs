use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::models::country::{Country, CountryCreate};



pub(crate) trait CountryRepository {
    fn get_country_by_id(
        &self,
        pool: &PgPool,
        country_id: Uuid,
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

}
