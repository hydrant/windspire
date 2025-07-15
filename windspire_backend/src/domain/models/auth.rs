use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub provider_id: String,
    pub provider_name: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub email: String,
    pub name: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub iat: i64, // Issued at
    pub exp: i64, // Expiration time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: AuthUser,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
    pub verified_email: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user: AuthUser,
    pub token: String,
}
