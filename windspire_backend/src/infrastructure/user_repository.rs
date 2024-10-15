use sqlx::PgPool;
use anyhow::Result;
use crate::domain::models::User;

pub async fn get_users(
    pool: &PgPool
) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User, 
        r#"
        SELECT  
            users.id,
            users.first_name,
            users.last_name,
            users.email,
            users.phone,
            users.country_id
        FROM public.users 
        "#
    )
        .fetch_all(pool)
        .await?;
    
    Ok(users)
}


async fn insert_user(
    user: User, 
    conn: &PgPool,
) -> Result<(), sqlx::Error> {
    let insert_user_q = r#"
    INSERT INTO users (id, first_name, last_name, email, phone, country_id)
    VALUES (DEFAULT, $1, $2, $3, $4, $5)
    "#;

    sqlx::query(insert_user_q)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.country_id)
        .execute(conn) // Pass conn directly, no need to dereference
        .await?;

    Ok(())
}