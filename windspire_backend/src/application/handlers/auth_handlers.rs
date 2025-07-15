use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::config::AppConfig;
use crate::application::http_response::{json_response, ok_json_response};
use crate::application::services::{jwt_service::JwtService, oauth_service::OAuthService};
use crate::domain::models::auth::{AuthUser, JwtTokenResponse};
use crate::domain::models::user::OAuthUserCreate;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::repositories::sqlx_user_repository::SqlxUserRepository;

#[derive(Debug, Deserialize)]
pub struct AuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct AuthLoginResponse {
    pub authorization_url: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

pub struct AuthHandlers {
    oauth_service: Arc<OAuthService>,
    jwt_service: Arc<JwtService>,
    user_repository: SqlxUserRepository,
    config: AppConfig,
}

impl AuthHandlers {
    pub fn new(
        oauth_service: Arc<OAuthService>,
        jwt_service: Arc<JwtService>,
        config: AppConfig,
    ) -> Self {
        Self {
            oauth_service,
            jwt_service,
            user_repository: SqlxUserRepository,
            config,
        }
    }

    pub async fn login(&self) -> impl IntoResponse {
        tracing::info!("Starting OAuth login flow");

        let auth_url = self.oauth_service.get_authorization_url();

        let response = AuthLoginResponse {
            authorization_url: auth_url.url,
            state: auth_url.state,
        };

        ok_json_response(response)
    }

    pub async fn callback(
        &self,
        pool: &sqlx::PgPool,
        query: Query<AuthCallbackQuery>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        tracing::info!("Processing OAuth callback with code: {}", query.code);

        // Exchange authorization code for access token
        let access_token = self
            .oauth_service
            .exchange_code_for_token(&query.code, &query.state, &query.state)
            .await
            .map_err(|e| {
                tracing::error!("Token exchange failed: {}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Failed to exchange authorization code"
                    })),
                )
            })?;

        // Get user info from Google
        let google_user = self
            .oauth_service
            .get_user_info(&access_token)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get user info: {}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Failed to get user information"
                    })),
                )
            })?;

        // Check if user exists by email
        let existing_user = self
            .user_repository
            .get_user_by_email(pool, &google_user.email)
            .await;

        let user = match existing_user {
            Ok(user) => {
                tracing::info!("Existing user found: {}", user.email);
                // Update OAuth provider info if needed
                if user.provider_id.is_none() || user.provider_name.is_none() {
                    self.user_repository
                        .update_oauth_info(pool, user.id, &google_user.id, "google")
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to update OAuth info: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(serde_json::json!({
                                    "success": false,
                                    "message": "Failed to update user OAuth information"
                                })),
                            )
                        })?;
                }
                user
            }
            Err(_) => {
                tracing::info!("Creating new user: {}", google_user.email);
                // Create new user
                let new_user = OAuthUserCreate {
                    email: google_user.email.clone(),
                    first_name: google_user.given_name.clone(),
                    last_name: google_user.family_name.clone(),
                    provider_id: google_user.id.clone(),
                    provider_name: "google".to_string(),
                    avatar_url: google_user.picture.clone(),
                    country_id: self.get_default_country_id(pool).await?,
                };

                self.user_repository
                    .create_oauth_user(pool, &new_user)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to create user: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "success": false,
                                "message": "Failed to create user account"
                            })),
                        )
                    })?
            }
        };

        // Get user roles and permissions
        let user_with_roles = self
            .user_repository
            .get_user_with_roles(pool, user.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get user roles: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Failed to get user permissions"
                    })),
                )
            })?;

        // Create AuthUser for JWT
        let auth_user = AuthUser {
            id: user.id,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            provider_id: user.provider_id.unwrap_or_default(),
            provider_name: user.provider_name.unwrap_or_default(),
            roles: user_with_roles
                .roles
                .iter()
                .map(|r| r.name.clone())
                .collect(),
            permissions: user_with_roles
                .permissions
                .iter()
                .map(|p| p.name.clone())
                .collect(),
        };

        // Generate JWT token
        let jwt_token = self.jwt_service.generate_token(&auth_user).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to generate authentication token"
                })),
            )
        })?;

        tracing::info!("Successfully authenticated user: {}", user.email);

        let response = JwtTokenResponse {
            access_token: jwt_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt.expiration_hours * 3600,
            user: auth_user,
        };

        Ok(ok_json_response(response))
    }

    pub async fn refresh_token(
        &self,
        Json(request): Json<RefreshTokenRequest>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        // For now, we'll treat the refresh token as a JWT token
        // In a production system, you'd want separate refresh tokens
        let claims = self
            .jwt_service
            .validate_token(&request.refresh_token)
            .map_err(|e| {
                tracing::error!("Invalid refresh token: {}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "success": false,
                        "message": "Invalid refresh token"
                    })),
                )
            })?;

        let new_token = self.jwt_service.refresh_token(&claims).map_err(|e| {
            tracing::error!("Failed to refresh token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to refresh token"
                })),
            )
        })?;

        let response = serde_json::json!({
            "access_token": new_token,
            "token_type": "Bearer",
            "expires_in": self.config.jwt.expiration_hours * 3600
        });

        Ok(ok_json_response(response))
    }

    pub async fn logout(&self) -> impl IntoResponse {
        // For JWT tokens, logout is typically handled client-side
        // by removing the token from storage
        tracing::info!("User logout requested");

        ok_json_response(serde_json::json!({
            "message": "Successfully logged out"
        }))
    }

    async fn get_default_country_id(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<Uuid, (StatusCode, Json<serde_json::Value>)> {
        // Get Norway as default country (or first available country)
        let country = sqlx::query!(
            "SELECT id FROM countries WHERE iso_alpha2 = 'NO' OR iso_alpha3 = 'NOR' LIMIT 1"
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get default country: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to get default country"
                })),
            )
        })?;

        if let Some(country) = country {
            Ok(country.id)
        } else {
            // Fallback: get any country
            let any_country = sqlx::query!("SELECT id FROM countries LIMIT 1")
                .fetch_optional(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to get any country: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "success": false,
                            "message": "No countries available"
                        })),
                    )
                })?;

            any_country.map(|c| c.id).ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "No countries found in database"
                })),
            ))
        }
    }
}
