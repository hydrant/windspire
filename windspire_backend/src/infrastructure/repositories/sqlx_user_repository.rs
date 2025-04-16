use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::{models::user::{User, UserCreate}, user_repository::UserRepository};


pub struct SqlxUserRepository;

impl UserRepository for SqlxUserRepository {
    async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT  
                id,
                first_name,
                last_name,
                email,
                phone,
                country_id
            FROM users 
            WHERE id = $1
            "#,
            user_id // Pass the id to the query
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn get_users(&self, pool: &PgPool) -> Result<Vec<User>, Error> {
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

    async fn insert_user(&self, conn: &PgPool, user: UserCreate) -> Result<(), sqlx::Error> {
        let insert_user_q = r#"
        INSERT INTO users (id, first_name, last_name, email, phone, country_id)
        VALUES (DEFAULT, $1, $2, $3, $4, $5)
        "#;

        sqlx::query(insert_user_q)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.email)
            .bind(&user.phone.as_deref())
            .bind(&user.country_id)
            .execute(conn) // Pass conn directly, no need to dereference
            .await?;

        Ok(())
    }
}
