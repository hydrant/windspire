use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::rbac::{Permission, Role};
use crate::infrastructure::error::Error;

#[async_trait]
pub trait RoleRepository {
    async fn get_role_by_id(&self, pool: &PgPool, role_id: Uuid) -> Result<Role, Error>;
    async fn get_role_by_name(&self, pool: &PgPool, name: &str) -> Result<Role, Error>;
    async fn get_all_roles(&self, pool: &PgPool) -> Result<Vec<Role>, Error>;
    async fn create_role(
        &self,
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<Role, Error>;
    async fn update_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<Role, Error>;
    async fn delete_role(&self, pool: &PgPool, role_id: Uuid) -> Result<(), Error>;

    // Role-Permission management
    async fn get_role_permissions(
        &self,
        pool: &PgPool,
        role_id: Uuid,
    ) -> Result<Vec<Permission>, Error>;
    async fn assign_permission_to_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<(), Error>;
    async fn remove_permission_from_role(
        &self,
        pool: &PgPool,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<(), Error>;
}
