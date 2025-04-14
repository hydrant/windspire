use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::country::Country;

pub async fn get_by_id(
    pool: &PgPool,
    id: Uuid, // Accept the id as a parameter
) -> Result<Country, sqlx::Error> {
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
        id // Pass the id to the query
    )
    .fetch_one(pool)
    .await?;

    Ok(country)
}
