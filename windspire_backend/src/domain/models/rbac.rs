use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub granted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRoles {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}

// DTOs for creating roles and permissions
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRole {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePermission {
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

// Common permission patterns
pub const PERMISSION_USERS_READ: &str = "users:read";
pub const PERMISSION_USERS_WRITE: &str = "users:write";
pub const PERMISSION_USERS_DELETE: &str = "users:delete";
pub const PERMISSION_USERS_READ_OWN: &str = "users:read_own";
pub const PERMISSION_USERS_WRITE_OWN: &str = "users:write_own";

pub const PERMISSION_COUNTRIES_READ: &str = "countries:read";
pub const PERMISSION_COUNTRIES_WRITE: &str = "countries:write";
pub const PERMISSION_COUNTRIES_DELETE: &str = "countries:delete";

pub const PERMISSION_BOATS_READ: &str = "boats:read";
pub const PERMISSION_BOATS_WRITE: &str = "boats:write";
pub const PERMISSION_BOATS_DELETE: &str = "boats:delete";

// Default roles
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_MODERATOR: &str = "moderator";
pub const ROLE_USER: &str = "user";
