use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    interface::country_repository::CountryRepository,
    models::country::{Country, CountryCreate, CountryUpdate},
};

pub struct SqlxCountryRepository;

impl CountryRepository for SqlxCountryRepository {
    async fn get_countries(&self, pool: &PgPool) -> Result<Vec<Country>, Error> {
        let countries = sqlx::query_as!(
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

        Ok(countries)
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
        country_create: CountryCreate,
    ) -> Result<Country, Error> {
        // Generate UUID v7 id
        let ts = Timestamp::now(NoContext);
        let id = Uuid::new_v7(ts);
        let country = sqlx::query_as!(
            Country,
            r#"
            INSERT INTO countries (id, iso_name, iso_alpha_2, iso_alpha_3)
            VALUES ($1, $2, $3, $4)
            RETURNING id, iso_name, iso_alpha_2, iso_alpha_3
            "#,
            id,
            country_create.iso_name,
            country_create.iso_alpha_2,
            country_create.iso_alpha_3
        )
        .fetch_one(conn)
        .await?;

        Ok(country)
    }

    async fn delete_country(&self, conn: &PgPool, country_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM countries WHERE id = $1", country_id)
            .execute(conn)
            .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    async fn update_country(
        &self,
        conn: &PgPool,
        country_id: Uuid,
        country_update: CountryUpdate,
    ) -> Result<Country, Error> {
        let country = sqlx::query_as!(
            Country,
            r#"
            UPDATE countries
            SET iso_name = $1, iso_alpha_2 = $2, iso_alpha_3 = $3
            WHERE id = $4
            RETURNING id, iso_name, iso_alpha_2, iso_alpha_3
            "#,
            country_update.iso_name,
            country_update.iso_alpha_2,
            country_update.iso_alpha_3,
            country_id
        )
        .fetch_one(conn)
        .await?;
        Ok(country)
    }

    async fn get_country_by_code(
        &self,
        pool: &PgPool,
        country_code: String,
    ) -> Result<Country, Error> {
        // Check if the country code is 2 or 3 characters long and contains only ASCII alphabetic characters
        if (country_code.len() != 2 && country_code.len() != 3)
            || !country_code.chars().all(|c| c.is_ascii_alphabetic())
        {
            return Err(Error::ColumnNotFound("Invalid country code".to_string()));
        }

        // Compose the SQL query to fetch the country by code
        // Use the appropriate column based on the length of the country code
        let query = if country_code.len() == 2 {
            r#"
            SELECT  
                id,
                iso_name,
                iso_alpha_2,
                iso_alpha_3
            FROM countries 
            WHERE iso_alpha_2 = $1
            "#
        } else {
            r#"
            SELECT  
                id,
                iso_name,
                iso_alpha_2,
                iso_alpha_3
            FROM countries 
            WHERE iso_alpha_3 = $1
            "#
        };

        let code = country_code.to_ascii_uppercase();
        let country = sqlx::query_as::<_, Country>(query)
            .bind(code)
            .fetch_one(pool)
            .await?;
        Ok(country)
    }
}
