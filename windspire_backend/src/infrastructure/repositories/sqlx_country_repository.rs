use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::{interface::country_repository::CountryRepository, models::country::Country};

pub struct SqlxCountryRepository;

impl CountryRepository for SqlxCountryRepository {
    async fn get_countries(&self, pool: &PgPool) -> Result<Vec<Country>, Error> {
        let users = sqlx::query_as!(
            Country,
            r#"
            SELECT  
                countries.id,
                countries.iso_name,
                countries.iso_alpha_2,
                countries.iso_alpha_3
            FROM public.countries
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    async fn get_country_by_id(&self, pool: &PgPool, country_id: Uuid) -> Result<Country, Error> {
        let country = sqlx::query_as!(
            Country,
            r#"
        SELECT  
            id,
            iso_name,
            iso_alpha_2,
            iso_alpha_3
        FROM countries 
        WHERE id = $1
        "#,
            country_id
        )
        .fetch_one(pool)
        .await?;

        Ok(country)
    }

    async fn insert_country(
        &self,
        conn: &PgPool,
        country: crate::domain::models::country::CountryCreate,
    ) -> Result<Country, Error> {
        todo!()
    }
}

//pub async fn get_by_id(
//    pool: &PgPool,
//    id: Uuid, // Accept the id as a parameter
//) -> Result<Country, sqlx::Error> {
//    let country = sqlx::query_as!(
//        Country,
//        r#"
//        SELECT
//            id,
//            iso_name,
//            iso_alpha_2,
//            iso_alpha_3
//        FROM countries
//        WHERE id = $1
//        "#,
//        id // Pass the id to the query
//    )
//    .fetch_one(pool)
//    .await?;
//
//    Ok(country)
//}
//
