use sqlx::PgPool;
use anyhow::Result;
use crate::domain::models::User;

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User, 
        r#"
        SELECT  
            users.id,
            users.first_name,
            users.last_name,
            users.email,
            users.phone,
            countries.iso_name as country
        FROM public.users 
        JOIN countries ON users.country_id = countries.id;
        "#
    )
        .fetch_all(pool)
        .await
        .expect("Failed to fetch users from the database");
    
    Ok(users)
}