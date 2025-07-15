use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::config::JwtConfig;
use crate::domain::models::auth::{AuthUser, Claims};

#[derive(Debug)]
pub enum JwtError {
    TokenCreation(jsonwebtoken::errors::Error),
    TokenValidation(jsonwebtoken::errors::Error),
    InvalidToken,
    ExpiredToken,
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::TokenCreation(e) => write!(f, "Token creation error: {}", e),
            JwtError::TokenValidation(e) => write!(f, "Token validation error: {}", e),
            JwtError::InvalidToken => write!(f, "Invalid token"),
            JwtError::ExpiredToken => write!(f, "Token has expired"),
        }
    }
}

impl std::error::Error for JwtError {}

pub struct JwtService {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.secret.as_ref());
        let decoding_key = DecodingKey::from_secret(config.secret.as_ref());

        Self {
            config,
            encoding_key,
            decoding_key,
        }
    }

    pub fn generate_token(&self, user: &AuthUser) -> Result<String, JwtError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.config.expiration_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            name: format!("{} {}", user.first_name, user.last_name),
            roles: user.roles.clone(),
            permissions: user.permissions.clone(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        };

        let header = Header::new(Algorithm::HS256);

        encode(&header, &claims, &self.encoding_key).map_err(JwtError::TokenCreation)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.config.issuer]);

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
                _ => JwtError::TokenValidation(e),
            }
        })?;

        Ok(token_data.claims)
    }

    pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }

    pub fn refresh_token(&self, claims: &Claims) -> Result<String, JwtError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.config.expiration_hours);

        let new_claims = Claims {
            sub: claims.sub.clone(),
            email: claims.email.clone(),
            name: claims.name.clone(),
            roles: claims.roles.clone(),
            permissions: claims.permissions.clone(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        };

        let header = Header::new(Algorithm::HS256);

        encode(&header, &new_claims, &self.encoding_key).map_err(JwtError::TokenCreation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret-key".to_string(),
            expiration_hours: 1,
            issuer: "test".to_string(),
        }
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

    #[test]
    fn test_generate_and_validate_token() {
        let config = create_test_config();
        let jwt_service = JwtService::new(config);
        let user = create_test_user();

        let token = jwt_service.generate_token(&user).unwrap();
        let claims = jwt_service.validate_token(&token).unwrap();

        assert_eq!(claims.email, user.email);
        assert_eq!(claims.roles, user.roles);
        assert_eq!(claims.permissions, user.permissions);
    }

    #[test]
    fn test_extract_bearer_token() {
        let auth_header = "Bearer abc123";
        let token = JwtService::extract_bearer_token(auth_header);
        assert_eq!(token, Some("abc123"));

        let invalid_header = "Basic abc123";
        let token = JwtService::extract_bearer_token(invalid_header);
        assert_eq!(token, None);
    }
}
