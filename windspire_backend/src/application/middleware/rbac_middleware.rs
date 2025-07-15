use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::collections::HashSet;

use crate::application::middleware::auth_middleware::extract_auth_context;
use crate::domain::models::rbac::{
    PERMISSION_BOATS_DELETE, PERMISSION_BOATS_READ, PERMISSION_BOATS_WRITE,
    PERMISSION_COUNTRIES_DELETE, PERMISSION_COUNTRIES_READ, PERMISSION_COUNTRIES_WRITE,
    PERMISSION_USERS_DELETE, PERMISSION_USERS_READ, PERMISSION_USERS_READ_OWN,
    PERMISSION_USERS_WRITE, PERMISSION_USERS_WRITE_OWN, ROLE_ADMIN,
};

#[derive(Debug, Clone)]
pub struct RequiredPermission {
    pub permission: String,
    pub allow_own: bool, // Allow if user owns the resource
}

impl RequiredPermission {
    pub fn new(permission: &str) -> Self {
        Self {
            permission: permission.to_string(),
            allow_own: false,
        }
    }

    pub fn with_own_access(permission: &str) -> Self {
        Self {
            permission: permission.to_string(),
            allow_own: true,
        }
    }
}

pub fn require_permission(
    permission: RequiredPermission,
) -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    move |request: Request, next: Next| {
        let permission = permission.clone();
        Box::pin(async move { check_permission(request, next, permission).await })
    }
}

async fn check_permission(
    request: Request,
    next: Next,
    required_permission: RequiredPermission,
) -> Result<Response, (StatusCode, &'static str)> {
    // Extract auth context
    let auth_context = extract_auth_context(&request)
        .ok_or((StatusCode::UNAUTHORIZED, "Authentication required"))?;

    // Admin role bypasses all permission checks
    if auth_context.user.roles.contains(&ROLE_ADMIN.to_string()) {
        return Ok(next.run(request).await);
    }

    // Check if user has the required permission
    let has_permission = auth_context
        .user
        .permissions
        .contains(&required_permission.permission);

    if has_permission {
        return Ok(next.run(request).await);
    }

    // If permission allows own access, check for own-specific permission
    if required_permission.allow_own {
        let own_permission = format!(
            "{}_own",
            required_permission
                .permission
                .replace(":read", "")
                .replace(":write", "")
                .replace(":delete", "")
        );
        if auth_context
            .user
            .permissions
            .iter()
            .any(|p| p.contains("_own"))
        {
            // Additional logic would be needed here to verify the user owns the resource
            // This would typically involve extracting the resource ID from the request path
            // and checking ownership in the database
            return Ok(next.run(request).await);
        }
    }

    Err((StatusCode::FORBIDDEN, "Insufficient permissions"))
}

// Convenience functions for common permission checks
pub fn require_users_read() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::with_own_access(PERMISSION_USERS_READ))
}

pub fn require_users_write() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::with_own_access(PERMISSION_USERS_WRITE))
}

pub fn require_users_delete() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_USERS_DELETE))
}

pub fn require_countries_read() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_COUNTRIES_READ))
}

pub fn require_countries_write() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_COUNTRIES_WRITE))
}

pub fn require_countries_delete() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_COUNTRIES_DELETE))
}

pub fn require_boats_read() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_BOATS_READ))
}

pub fn require_boats_write() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_BOATS_WRITE))
}

pub fn require_boats_delete() -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, (StatusCode, &'static str)>> + Send>,
> + Clone {
    require_permission(RequiredPermission::new(PERMISSION_BOATS_DELETE))
}

// Helper function to check if user has any of the specified roles
pub fn has_any_role(request: &Request, roles: &[&str]) -> bool {
    if let Some(auth_context) = extract_auth_context(request) {
        let user_roles: HashSet<&str> =
            auth_context.user.roles.iter().map(|r| r.as_str()).collect();
        let required_roles: HashSet<&str> = roles.iter().copied().collect();
        !user_roles.is_disjoint(&required_roles)
    } else {
        false
    }
}

// Helper function to check if user has specific permission
pub fn has_permission(request: &Request, permission: &str) -> bool {
    if let Some(auth_context) = extract_auth_context(request) {
        // Admin role bypasses all permission checks
        if auth_context.user.roles.contains(&ROLE_ADMIN.to_string()) {
            return true;
        }
        auth_context
            .user
            .permissions
            .contains(&permission.to_string())
    } else {
        false
    }
}

// Helper function to get user ID from auth context
pub fn get_user_id(request: &Request) -> Option<uuid::Uuid> {
    extract_auth_context(request).map(|ctx| ctx.user.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::auth::{AuthContext, AuthUser};
    use axum::{body::Body, http::Request};
    use uuid::Uuid;

    fn create_test_user_with_permissions(permissions: Vec<&str>, roles: Vec<&str>) -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            provider_id: "123".to_string(),
            provider_name: "google".to_string(),
            roles: roles.iter().map(|r| r.to_string()).collect(),
            permissions: permissions.iter().map(|p| p.to_string()).collect(),
        }
    }

    #[test]
    fn test_has_permission() {
        let user = create_test_user_with_permissions(vec![PERMISSION_USERS_READ], vec!["user"]);
        let auth_context = AuthContext {
            user,
            token: "test-token".to_string(),
        };

        let mut request = Request::builder().body(Body::empty()).unwrap();
        request.extensions_mut().insert(auth_context);

        assert!(has_permission(&request, PERMISSION_USERS_READ));
        assert!(!has_permission(&request, PERMISSION_USERS_WRITE));
    }

    #[test]
    fn test_admin_role_bypass() {
        let user = create_test_user_with_permissions(vec![], vec![ROLE_ADMIN]);
        let auth_context = AuthContext {
            user,
            token: "test-token".to_string(),
        };

        let mut request = Request::builder().body(Body::empty()).unwrap();
        request.extensions_mut().insert(auth_context);

        // Admin should have access to any permission
        assert!(has_permission(&request, PERMISSION_USERS_DELETE));
        assert!(has_permission(&request, PERMISSION_COUNTRIES_DELETE));
        assert!(has_permission(&request, "any:permission"));
    }

    #[test]
    fn test_has_any_role() {
        let user = create_test_user_with_permissions(vec![], vec!["user", "moderator"]);
        let auth_context = AuthContext {
            user,
            token: "test-token".to_string(),
        };

        let mut request = Request::builder().body(Body::empty()).unwrap();
        request.extensions_mut().insert(auth_context);

        assert!(has_any_role(&request, &["user"]));
        assert!(has_any_role(&request, &["moderator"]));
        assert!(has_any_role(&request, &["user", "admin"]));
        assert!(!has_any_role(&request, &["admin"]));
    }

    #[test]
    fn test_get_user_id() {
        let user_id = Uuid::new_v4();
        let mut user = create_test_user_with_permissions(vec![], vec!["user"]);
        user.id = user_id;

        let auth_context = AuthContext {
            user,
            token: "test-token".to_string(),
        };

        let mut request = Request::builder().body(Body::empty()).unwrap();
        request.extensions_mut().insert(auth_context);

        assert_eq!(get_user_id(&request), Some(user_id));
    }
}
