use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::application::http_response::ok_json_response;

use crate::domain::models::auth::{AuthUser, JwtTokenResponse};
use crate::domain::models::user::{OAuthUserCreate, User};
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

// Standalone handler functions for use with Axum router
pub async fn login_handler(
    State(app_state): State<crate::application::state::AppState>,
) -> impl IntoResponse {
    tracing::info!("Starting OAuth login flow");

    let auth_url = app_state.oauth_service.get_authorization_url();

    let response = AuthLoginResponse {
        authorization_url: auth_url.url,
        state: auth_url.state,
    };

    ok_json_response(response)
}

pub async fn oauth_callback_handler(
    State(app_state): State<crate::application::state::AppState>,
    query: Query<AuthCallbackQuery>,
) -> impl IntoResponse {
    tracing::info!("Processing OAuth callback with code: {}", query.code);

    // Exchange authorization code for access token
    let access_token = match app_state
        .oauth_service
        .exchange_code_for_token(&query.code, &query.state, &query.state)
        .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Token exchange failed: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to exchange authorization code"
                })),
            )
                .into_response();
        }
    };

    // Get user info from Google
    let google_user = match app_state.oauth_service.get_user_info(&access_token).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to get user info: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to get user information"
                })),
            )
                .into_response();
        }
    };

    let user_repository = SqlxUserRepository;

    // Check if user exists by email
    let existing_user = user_repository
        .get_user_by_email(&app_state.db_pool, &google_user.email)
        .await;

    let user = match existing_user {
        Ok(user_by_email) => {
            tracing::info!("Existing user found: {}", user_by_email.email);
            // Update OAuth provider info if needed
            if user_by_email.provider_id.is_none() || user_by_email.provider_name.is_none() {
                if let Err(e) = user_repository
                    .update_oauth_info(
                        &app_state.db_pool,
                        user_by_email.id,
                        &google_user.id,
                        "google",
                    )
                    .await
                {
                    tracing::error!("Failed to update OAuth info: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "success": false,
                            "message": "Failed to update user OAuth information"
                        })),
                    )
                        .into_response();
                }
            }

            // Convert UserByEmail to User-like structure for consistency
            User {
                id: user_by_email.id,
                first_name: user_by_email.first_name,
                last_name: user_by_email.last_name,
                email: user_by_email.email,
                phone: None, // Not available in UserByEmail
                country_id: user_by_email.country_id,
                provider_id: user_by_email.provider_id,
                provider_name: user_by_email.provider_name,
                avatar_url: google_user.picture.clone(), // Use Google profile picture
                created_at: None, // Not available in UserByEmail
                updated_at: None, // Not available in UserByEmail
            }
        }
        Err(_) => {
            tracing::info!("Creating new user: {}", google_user.email);

            // Get default country ID
            let country_id = match get_default_country_id(&app_state.db_pool).await {
                Ok(id) => id,
                Err(response) => return response.into_response(),
            };

            // Create new user
            let new_user = OAuthUserCreate {
                email: google_user.email.clone(),
                first_name: google_user.given_name.clone(),
                last_name: google_user.family_name.clone(),
                provider_id: google_user.id.clone(),
                provider_name: "google".to_string(),
                avatar_url: google_user.picture.clone(),
                country_id,
            };

            match user_repository
                .create_oauth_user(&app_state.db_pool, &new_user)
                .await
            {
                Ok(created_user) => created_user,
                Err(e) => {
                    tracing::error!("Failed to create user: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "success": false,
                            "message": "Failed to create user account"
                        })),
                    )
                        .into_response();
                }
            }
        }
    };

    // Get user roles and permissions
    let user_with_roles = match user_repository
        .get_user_with_roles(&app_state.db_pool, user.id)
        .await
    {
        Ok(user_with_roles) => user_with_roles,
        Err(e) => {
            tracing::error!("Failed to get user roles: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to get user permissions"
                })),
            )
                .into_response();
        }
    };

    // Create AuthUser for JWT
    let auth_user = AuthUser {
        id: user.id,
        email: user.email.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        provider_id: user.provider_id.unwrap_or_default(),
        provider_name: user.provider_name.unwrap_or_default(),
        avatar_url: user.avatar_url.clone(),
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
    let jwt_token = match app_state.jwt_service.generate_token(&auth_user) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to generate JWT: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to generate authentication token"
                })),
            )
                .into_response();
        }
    };

    tracing::info!("Successfully authenticated user: {}", user.email);

    // Instead of returning JSON, redirect to frontend with token
    let frontend_url = format!("http://localhost:5173/auth/callback?token={}", jwt_token);
    
    (
        StatusCode::FOUND,
        [("Location", frontend_url.as_str())],
        "Redirecting to frontend..."
    ).into_response()
}

pub async fn refresh_token_handler(
    State(app_state): State<crate::application::state::AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    // For now, we'll treat the refresh token as a JWT token
    // In a production system, you'd want separate refresh tokens
    let claims = match app_state.jwt_service.validate_token(&request.refresh_token) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::error!("Invalid refresh token: {}", e);
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Invalid refresh token"
                })),
            )
                .into_response();
        }
    };

    let new_token = match app_state.jwt_service.refresh_token(&claims) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to refresh token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Failed to refresh token"
                })),
            )
                .into_response();
        }
    };

    let response = serde_json::json!({
        "access_token": new_token,
        "token_type": "Bearer",
        "expires_in": app_state.config.jwt.expiration_hours * 3600
    });

    ok_json_response(response).into_response()
}

pub async fn logout_handler(
    State(_app_state): State<crate::application::state::AppState>,
) -> impl IntoResponse {
    // For JWT tokens, logout is typically handled client-side
    // by removing the token from storage
    tracing::info!("User logout requested");

    ok_json_response(serde_json::json!({
        "message": "Successfully logged out"
    }))
}

pub async fn me_handler(
    State(_app_state): State<crate::application::state::AppState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    // Extract auth context from request extensions
    let auth_context = match request.extensions().get::<crate::domain::models::auth::AuthContext>() {
        Some(ctx) => ctx,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Authentication required"
                })),
            ).into_response();
        }
    };

    tracing::info!("Getting current user info for: {}", auth_context.user.email);

    // Return the user info from the JWT token
    let user_info = serde_json::json!({
        "id": auth_context.user.id,
        "email": auth_context.user.email,
        "name": format!("{} {}", auth_context.user.first_name, auth_context.user.last_name),
        "first_name": auth_context.user.first_name,
        "last_name": auth_context.user.last_name,
        "picture": auth_context.user.avatar_url,
        "roles": auth_context.user.roles,
        "permissions": auth_context.user.permissions
    });

    ok_json_response(user_info)
}

async fn get_default_country_id(
    pool: &sqlx::PgPool,
) -> Result<Uuid, (StatusCode, Json<serde_json::Value>)> {
    // Get Norway as default country (or first available country)
    let country = sqlx::query!(
        "SELECT id FROM countries WHERE iso_alpha_2 = 'NO' OR iso_alpha_3 = 'NOR' LIMIT 1"
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
