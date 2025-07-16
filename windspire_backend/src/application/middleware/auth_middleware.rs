use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::application::services::jwt_service::{JwtError, JwtService};
use crate::domain::models::auth::{AuthContext, Claims};

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken(JwtError),
    InternalError,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingToken => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            AuthError::MissingToken => "Missing authorization token",
            AuthError::InvalidToken(_) => "Invalid or expired token",
            AuthError::InternalError => "Internal server error",
        }
    }
}

pub async fn jwt_auth_middleware(
    State(app_state): State<crate::application::state::AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

    // Extract Bearer token
    let token = JwtService::extract_bearer_token(auth_header)
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization format"))?;

    // Validate token
    let claims = app_state
        .jwt_service
        .validate_token(token)
        .map_err(|e| match e {
            JwtError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired"),
            _ => (StatusCode::UNAUTHORIZED, "Invalid token"),
        })?;

    // Create auth context and add to request extensions
    let auth_context = AuthContext {
        user: claims_to_auth_user(&claims),
        token: token.to_string(),
    };

    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

// Optional middleware that doesn't fail if no token is provided
pub async fn optional_jwt_auth_middleware(
    State(app_state): State<crate::application::state::AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        if let Some(token) = JwtService::extract_bearer_token(auth_header) {
            if let Ok(claims) = app_state.jwt_service.validate_token(token) {
                let auth_context = AuthContext {
                    user: claims_to_auth_user(&claims),
                    token: token.to_string(),
                };
                request.extensions_mut().insert(auth_context);
            }
        }
    }

    next.run(request).await
}

fn claims_to_auth_user(claims: &Claims) -> crate::domain::models::auth::AuthUser {
    use uuid::Uuid;

    crate::domain::models::auth::AuthUser {
        id: Uuid::parse_str(&claims.sub).unwrap_or_default(),
        email: claims.email.clone(),
        first_name: claims
            .name
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string(),
        last_name: claims
            .name
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" "),
        provider_id: "".to_string(), // This would be populated from database
        provider_name: "".to_string(), // This would be populated from database
        roles: claims.roles.clone(),
        permissions: claims.permissions.clone(),
    }
}

// Helper function to extract auth context from request
pub fn extract_auth_context(request: &Request) -> Option<&AuthContext> {
    request.extensions().get::<AuthContext>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::config::JwtConfig;
    use crate::application::services::jwt_service::JwtService;
    use crate::domain::models::auth::AuthUser;
    use axum::{body::Body, http::Request};
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_test_jwt_service() -> Arc<JwtService> {
        let config = JwtConfig {
            secret: "test-secret".to_string(),
            expiration_hours: 1,
            issuer: "test".to_string(),
        };
        Arc::new(JwtService::new(config))
    }

    fn create_test_user() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            provider_id: "123".to_string(),
            provider_name: "google".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["users:read_own".to_string()],
        }
    }

    #[tokio::test]
    async fn test_valid_token_middleware() {
        let jwt_service = create_test_jwt_service();
        let user = create_test_user();
        let token = jwt_service.generate_token(&user).unwrap();

        let mut request = Request::builder()
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        // Simulate middleware processing
        let claims = jwt_service.validate_token(&token).unwrap();
        let auth_context = AuthContext {
            user: claims_to_auth_user(&claims),
            token: token.clone(),
        };
        request.extensions_mut().insert(auth_context);

        let extracted_context = extract_auth_context(&request);
        assert!(extracted_context.is_some());
        assert_eq!(extracted_context.unwrap().user.email, user.email);
    }

    #[test]
    fn test_claims_to_auth_user() {
        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            email: "test@example.com".to_string(),
            name: "John Doe".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["users:read_own".to_string()],
            iat: 1234567890,
            exp: 1234567890 + 3600,
        };

        let auth_user = claims_to_auth_user(&claims);
        assert_eq!(auth_user.email, "test@example.com");
        assert_eq!(auth_user.first_name, "John");
        assert_eq!(auth_user.last_name, "Doe");
        assert_eq!(auth_user.roles, vec!["user"]);
    }
}
