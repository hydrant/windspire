use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::rbac::Permission;
use crate::infrastructure::error::Error;

#[async_trait]
pub trait PermissionRepository {
    async fn get_permission_by_id(
        &self,
        pool: &PgPool,
        permission_id: Uuid,
    ) -> Result<Permission, Error>;
    async fn get_permission_by_name(&self, pool: &PgPool, name: &str) -> Result<Permission, Error>;
    async fn get_all_permissions(&self, pool: &PgPool) -> Result<Vec<Permission>, Error>;
    async fn get_permissions_by_resource(
        &self,
        pool: &PgPool,
        resource: &str,
    ) -> Result<Vec<Permission>, Error>;
    async fn create_permission(
        &self,
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
        resource: &str,
        action: &str,
    ) -> Result<Permission, Error>;
    async fn update_permission(
        &self,
        pool: &PgPool,
        permission_id: Uuid,
        name: &str,
        description: Option<&str>,
        resource: &str,
        action: &str,
    ) -> Result<Permission, Error>;
    async fn delete_permission(&self, pool: &PgPool, permission_id: Uuid) -> Result<(), Error>;
}
