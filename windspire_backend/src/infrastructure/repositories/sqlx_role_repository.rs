use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::{NoContext, Timestamp, Uuid};

use crate::domain::{
    models::rbac::{Permission, Role},
    repositories::role_repository::RoleRepository,
};

pub struct SqlxRoleRepository;

#[async_trait]
impl RoleRepository for SqlxRoleRepository {
    async fn get_role_by_id(&self, pool: &PgPool, role_id: Uuid) -> Result<Role, Error> {
        let role = sqlx::query_as!(
            Role,
            r#"
            SELECT id, name, description, created_at
            FROM roles 
            WHERE id = $1
            "#,
            role_id
        )
        .fetch_one(pool)
        .await?;

        Ok(role)
    }

    async fn get_role_by_name(&self, pool: &PgPool, name: &str) -> Result<Role, Error> {
        let role = sqlx::query_as!(
            Role,
            r#"
            SELECT id, name, description, created_at
            FROM roles 
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(role)
    }

    async fn get_all_roles(&self, pool: &PgPool) -> Result<Vec<Role>, Error> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT id, name, description, created_at
            FROM roles 
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(roles)
    }

    async fn create_role(
        &self,
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<Role, Error> {
        let ts = Timestamp::now(NoContext);
        let id = Uuid::new_v7(ts);
        let now = Utc::now();

        let role = sqlx::query_as!(
            Role,
            r#"
            INSERT INTO roles (id, name, description, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, created_at
            "#,
            id,
            name,
            description,
            Some(now)
        )
        .fetch_one(pool)
        .await?;

        Ok(role)
    }

    async fn update_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<Role, Error> {
        let role = sqlx::query_as!(
            Role,
            r#"
            UPDATE roles
            SET name = $1, description = $2
            WHERE id = $3
            RETURNING id, name, description, created_at
            "#,
            name,
            description,
            role_id
        )
        .fetch_one(pool)
        .await?;

        Ok(role)
    }

    async fn delete_role(&self, pool: &PgPool, role_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM roles WHERE id = $1", role_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    // Role-Permission management
    async fn get_role_permissions(
        &self,
        pool: &PgPool,
        role_id: Uuid,
    ) -> Result<Vec<Permission>, Error> {
        let permissions = sqlx::query_as!(
            Permission,
            r#"
            SELECT p.id, p.name, p.description, p.resource, p.action, p.created_at
            FROM permissions p
            INNER JOIN role_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1
            ORDER BY p.resource, p.action
            "#,
            role_id
        )
        .fetch_all(pool)
        .await?;

        Ok(permissions)
    }

    async fn assign_permission_to_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            role_id,
            permission_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn remove_permission_from_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM role_permissions WHERE role_id = $1 AND permission_id = $2",
            role_id,
            permission_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
