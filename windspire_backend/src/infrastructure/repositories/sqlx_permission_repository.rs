use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    models::rbac::Permission, repositories::permission_repository::PermissionRepository,
};

pub struct SqlxPermissionRepository;

#[async_trait]
impl PermissionRepository for SqlxPermissionRepository {
    async fn get_permission_by_id(
        &self,
        pool: &PgPool,
        permission_id: Uuid,
    ) -> Result<Permission, Error> {
        let permission = sqlx::query_as!(
            Permission,
            r#"
            SELECT id, name, description, resource, action, created_at
            FROM permissions 
            WHERE id = $1
            "#,
            permission_id
        )
        .fetch_one(pool)
        .await?;

        Ok(permission)
    }

    async fn get_permission_by_name(&self, pool: &PgPool, name: &str) -> Result<Permission, Error> {
        let permission = sqlx::query_as!(
            Permission,
            r#"
            SELECT id, name, description, resource, action, created_at
            FROM permissions 
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(permission)
    }

    async fn get_all_permissions(&self, pool: &PgPool) -> Result<Vec<Permission>, Error> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            SELECT id, name, description, resource, action, created_at
            FROM permissions 
            ORDER BY resource, action
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(permissions)
    }

    async fn get_permissions_by_resource(
        &self,
        pool: &PgPool,
        resource: &str,
    ) -> Result<Vec<Permission>, Error> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            SELECT id, name, description, resource, action, created_at
            FROM permissions 
            WHERE resource = $1
            ORDER BY action
            "#,
            resource
        )
        .fetch_all(pool)
        .await?;

        Ok(permissions)
    }

    async fn create_permission(
        &self,
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
        resource: &str,
        action: &str,
    ) -> Result<Permission, Error> {
        let ts = Timestamp::now(NoContext);
        let id = Uuid::new_v7(ts);
        let now = Utc::now();

        let permission = sqlx::query_as!(
            Permission,
            r#"
            INSERT INTO permissions (id, name, description, resource, action, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, description, resource, action, created_at
            "#,
            id,
            name,
            description,
            resource,
            action,
            Some(now)
        )
        .fetch_one(pool)
        .await?;

        Ok(permission)
    }

    async fn update_permission(
        &self,
        pool: &PgPool,
        permission_id: Uuid,
        name: &str,
        description: Option<&str>,
        resource: &str,
        action: &str,
    ) -> Result<Permission, Error> {
        let permission = sqlx::query_as!(
            Permission,
            r#"
            UPDATE permissions
            SET name = $1, description = $2, resource = $3, action = $4
            WHERE id = $5
            RETURNING id, name, description, resource, action, created_at
            "#,
            name,
            description,
            resource,
            action,
            permission_id
        )
        .fetch_one(pool)
        .await?;

        Ok(permission)
    }

    async fn delete_permission(&self, pool: &PgPool, permission_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM permissions WHERE id = $1", permission_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
