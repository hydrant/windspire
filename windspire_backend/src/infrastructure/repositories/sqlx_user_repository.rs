use anyhow::Result;
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    models::rbac::{Permission, Role, UserWithRoles},
    models::user::{OAuthUserCreate, User, UserByEmail, UserCreate, UserUpdate, UserWithCountry},
    repositories::user_repository::UserRepository,
};

pub struct SqlxUserRepository;

impl UserRepository for SqlxUserRepository {
    async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<User, Error> {
        let row = sqlx::query!(
            r#"
            SELECT
                id as "id!: Uuid",
                first_name as "first_name!",
                last_name as "last_name!",
                email as "email!",
                phone,
                country_id as "country_id!: Uuid",
                provider_id,
                provider_name,
                avatar_url,
                created_at,
                updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email: row.email,
            phone: row.phone,
            country_id: row.country_id,
            provider_id: row.provider_id,
            provider_name: row.provider_name,
            avatar_url: row.avatar_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        Ok(user)
    }

    async fn get_users(&self, pool: &PgPool) -> Result<Vec<UserWithCountry>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                u.id as "id!: Uuid", 
                u.first_name as "first_name!: String", 
                u.last_name as "last_name!: String", 
                u.email as "email!: String", 
                u.phone, 
                u.country_id as "country_id!: Uuid", 
                c.iso_name as "iso_name?: String", 
                u.provider_id, 
                u.provider_name, 
                u.avatar_url 
            FROM public.users u
            LEFT JOIN public.countries c ON c.id = u.country_id
            "#
        )
        .fetch_all(pool)
        .await?;

        let users = rows
            .into_iter()
            .map(|row| UserWithCountry {
                id: row.id,
                first_name: row.first_name,
                last_name: row.last_name,
                email: row.email,
                phone: row.phone,
                country_id: row.country_id,
                iso_name: row.iso_name,
                provider_id: row.provider_id,
                provider_name: row.provider_name,
                avatar_url: row.avatar_url,
            })
            .collect();

        Ok(users)
    }

    async fn insert_user(
        &self,
        conn: &PgPool,
        user_create: UserCreate,
    ) -> Result<User, sqlx::Error> {
        // Generate UUID v7 id
        let ts = Timestamp::now(&NoContext);
        let id = Uuid::new_v7(ts);
        let now = Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, first_name, last_name, email, phone, country_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, first_name, last_name, email, phone, country_id, provider_id, provider_name, avatar_url, created_at, updated_at
            "#,
            id,
            user_create.first_name,
            user_create.last_name,
            user_create.email,
            user_create.phone,
            user_create.country_id,
            Some(now),
            Some(now)
        )
        .fetch_one(conn)
        .await?;

        Ok(user)
    }

    async fn delete_user(&self, conn: &PgPool, user_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(conn)
            .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    async fn update_user(
        &self,
        conn: &PgPool,
        user_id: Uuid,
        user_update: UserUpdate,
    ) -> Result<User, Error> {
        let now = Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name = $1, last_name = $2, email = $3, phone = $4, country_id = $5, updated_at = $6
            WHERE id = $7
            RETURNING id, first_name, last_name, email, phone, country_id, provider_id, provider_name, avatar_url, created_at, updated_at
            "#,
            user_update.first_name,
            user_update.last_name,
            user_update.email,
            user_update.phone,
            user_update.country_id,
            Some(now),
            user_id
        )
        .fetch_one(conn)
        .await?;
        Ok(user)
    }

    // OAuth-related methods
    async fn get_user_by_email(&self, pool: &PgPool, email: &str) -> Result<UserByEmail, Error> {
        let user = sqlx::query_as!(
            UserByEmail,
            r#"
            SELECT id, email, first_name, last_name, provider_id, provider_name, country_id
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_provider_id(
        &self,
        pool: &PgPool,
        provider_id: &str,
        provider_name: &str,
    ) -> Result<UserByEmail, Error> {
        let user = sqlx::query_as!(
            UserByEmail,
            r#"
            SELECT id, email, first_name, last_name, provider_id, provider_name, country_id
            FROM users
            WHERE provider_id = $1 AND provider_name = $2
            "#,
            provider_id,
            provider_name
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn create_oauth_user(
        &self,
        pool: &PgPool,
        user: &OAuthUserCreate,
    ) -> Result<User, Error> {
        let ts = Timestamp::now(&NoContext);
        let id = Uuid::new_v7(ts);
        let now = Utc::now();

        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                id, first_name, last_name, email, phone, country_id,
                provider_id, provider_name, avatar_url, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, first_name, last_name, email, phone, country_id,
                      provider_id, provider_name, avatar_url, created_at, updated_at
            "#,
            id,
            user.first_name,
            user.last_name,
            user.email,
            None::<String>, // phone
            user.country_id,
            Some(user.provider_id.clone()),
            Some(user.provider_name.clone()),
            user.avatar_url,
            Some(now),
            Some(now)
        )
        .fetch_one(pool)
        .await?;

        // Assign default user role
        let default_role_id = sqlx::query!("SELECT id FROM roles WHERE name = 'user' LIMIT 1")
            .fetch_one(pool)
            .await?;

        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
            created_user.id,
            default_role_id.id
        )
        .execute(pool)
        .await?;

        Ok(created_user)
    }

    async fn update_oauth_info(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        provider_id: &str,
        provider_name: &str,
    ) -> Result<(), Error> {
        let now = Utc::now();

        sqlx::query!(
            r#"
            UPDATE users
            SET provider_id = $1, provider_name = $2, updated_at = $3
            WHERE id = $4
            "#,
            Some(provider_id),
            Some(provider_name),
            Some(now),
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // RBAC-related methods
    async fn get_user_with_roles(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<UserWithRoles, Error> {
        // Get user basic info
        let user = sqlx::query!(
            r#"
            SELECT id, email, first_name, last_name
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        // Get user roles
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT r.id, r.name, r.description, r.created_at
            FROM roles r
            INNER JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        // Get user permissions through roles
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            SELECT DISTINCT p.id, p.name, p.description, p.resource, p.action, p.created_at
            FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            INNER JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(UserWithRoles {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            roles,
            permissions,
        })
    }

    async fn assign_role_to_user(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id,
            role_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn remove_role_from_user(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2",
            user_id,
            role_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
