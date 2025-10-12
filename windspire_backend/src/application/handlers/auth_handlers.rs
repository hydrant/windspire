use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::application::http_response::ok_json_response;

use crate::domain::models::auth::AuthUser;
use crate::domain::models::user::{OAuthUserCreate, User};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::repositories::sqlx_user_repository::SqlxUserRepository;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct FirebaseAuthRequest {
    pub id_token: String,
    pub display_name: Option<String>, // Optional display name for registration
}

#[derive(Debug, Serialize)]
pub struct FirebaseAuthResponse {
    pub success: bool,
    pub data: Option<AuthTokenData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthTokenData {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
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
    let auth_context = match request
        .extensions()
        .get::<crate::domain::models::auth::AuthContext>()
    {
        Some(ctx) => ctx,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Authentication required"
                })),
            )
                .into_response();
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

pub async fn firebase_auth_handler(
    State(app_state): State<crate::application::state::AppState>,
    Json(payload): Json<FirebaseAuthRequest>,
) -> impl IntoResponse {
    tracing::info!("Processing Firebase authentication");
    tracing::info!(
        "Received payload - display_name: {:?}",
        payload.display_name
    );

    // Verify Firebase ID token
    #[cfg(debug_assertions)]
    let firebase_result = app_state
        .firebase_service
        .verify_id_token_unsafe(&payload.id_token)
        .await;

    #[cfg(not(debug_assertions))]
    let firebase_result = app_state
        .firebase_service
        .verify_id_token(&payload.id_token)
        .await;

    let firebase_user = match firebase_result {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Firebase token verification failed: {}", e);
            return (
                StatusCode::UNAUTHORIZED,
                Json(FirebaseAuthResponse {
                    success: false,
                    data: None,
                    message: Some("Invalid Firebase token".to_string()),
                }),
            )
                .into_response();
        }
    };

    let user_repository = SqlxUserRepository;

    // Check if user exists by Firebase UID first, then fall back to email
    let existing_user = user_repository
        .get_user_by_provider_id(&app_state.db_pool, &firebase_user.uid, "firebase")
        .await;

    let user = match existing_user {
        Ok(user_by_provider) => {
            tracing::info!(
                "Existing user found by Firebase UID: {}",
                user_by_provider.email
            );

            // Check if we should update the user's name from display_name
            let (first_name, last_name) = if let Some(display_name_str) = &payload.display_name {
                if !display_name_str.trim().is_empty()
                    && (user_by_provider.first_name == "Firebase"
                        || user_by_provider.last_name == "User")
                {
                    tracing::info!(
                        "Updating existing user's name from '{}' '{}' to '{}'",
                        user_by_provider.first_name,
                        user_by_provider.last_name,
                        display_name_str
                    );

                    let name_parts: Vec<&str> = display_name_str.split_whitespace().collect();
                    let new_first_name = name_parts.first().unwrap_or(&"Firebase").to_string();
                    let new_last_name = name_parts
                        .get(1..)
                        .map(|parts| parts.join(" "))
                        .unwrap_or_else(|| "User".to_string());

                    // Update the user in the database with new names
                    tracing::info!(
                        "Updating user {} with first_name: '{}', last_name: '{}'",
                        user_by_provider.id,
                        new_first_name,
                        new_last_name
                    );

                    // Get the full user data first to preserve all fields
                    match user_repository
                        .get_user_by_id(&app_state.db_pool, user_by_provider.id)
                        .await
                    {
                        Ok(full_user) => {
                            // Create user update with new names but keeping existing data
                            let user_update = crate::domain::models::user::UserUpdate {
                                first_name: new_first_name.clone(),
                                last_name: new_last_name.clone(),
                                email: full_user.email,
                                phone: full_user.phone,
                                country_id: full_user.country_id,
                            };

                            // Update the user in the database
                            if let Err(e) = user_repository
                                .update_user(&app_state.db_pool, user_by_provider.id, user_update)
                                .await
                            {
                                tracing::error!("Failed to update user names: {}", e);
                            } else {
                                tracing::info!("Successfully updated user names in database");
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to get full user data for update: {}", e);
                        }
                    }

                    (new_first_name, new_last_name)
                } else {
                    (
                        user_by_provider.first_name.clone(),
                        user_by_provider.last_name.clone(),
                    )
                }
            } else {
                (
                    user_by_provider.first_name.clone(),
                    user_by_provider.last_name.clone(),
                )
            };

            User {
                id: user_by_provider.id,
                first_name,
                last_name,
                email: user_by_provider.email,
                phone: None,
                country_id: user_by_provider.country_id,
                provider_id: user_by_provider.provider_id,
                provider_name: user_by_provider.provider_name,
                avatar_url: firebase_user.picture.clone(),
                created_at: None,
                updated_at: None,
            }
        }
        Err(_) => {
            // Check if user exists by email
            let existing_user_by_email = user_repository
                .get_user_by_email(
                    &app_state.db_pool,
                    &firebase_user.email.clone().unwrap_or_default(),
                )
                .await;

            match existing_user_by_email {
                Ok(user_by_email) => {
                    tracing::info!(
                        "Existing user found by email, updating Firebase info: {}",
                        user_by_email.email
                    );

                    // Update Firebase provider info
                    if let Err(e) = user_repository
                        .update_oauth_info(
                            &app_state.db_pool,
                            user_by_email.id,
                            &firebase_user.uid,
                            "firebase",
                        )
                        .await
                    {
                        tracing::error!("Failed to update Firebase info: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(FirebaseAuthResponse {
                                success: false,
                                data: None,
                                message: Some(
                                    "Failed to update user Firebase information".to_string(),
                                ),
                            }),
                        )
                            .into_response();
                    }

                    User {
                        id: user_by_email.id,
                        first_name: user_by_email.first_name,
                        last_name: user_by_email.last_name,
                        email: user_by_email.email,
                        phone: None,
                        country_id: user_by_email.country_id,
                        provider_id: Some(firebase_user.uid.clone()),
                        provider_name: Some("firebase".to_string()),
                        avatar_url: firebase_user.picture.clone(),
                        created_at: None,
                        updated_at: None,
                    }
                }
                Err(_) => {
                    tracing::info!(
                        "Creating new user from Firebase: {}",
                        firebase_user
                            .email
                            .as_ref()
                            .unwrap_or(&"unknown".to_string())
                    );

                    // Get default country ID
                    let country_id = match get_default_country_id(&app_state.db_pool).await {
                        Ok(id) => id,
                        Err(response) => return response.into_response(),
                    };

                    // Extract name parts - prefer display_name from request, fall back to Firebase token
                    tracing::info!("Firebase user name field: {:?}", firebase_user.name);
                    tracing::info!("Request display_name field: {:?}", payload.display_name);

                    let display_name = payload
                        .display_name
                        .as_deref()
                        .or(firebase_user.name.as_deref())
                        .unwrap_or("Firebase User");

                    tracing::info!("Using display_name: {}", display_name);

                    let name_parts: Vec<&str> = display_name.split_whitespace().collect();

                    let first_name = name_parts.first().unwrap_or(&"Firebase").to_string();
                    let last_name = name_parts
                        .get(1..)
                        .map(|parts| parts.join(" "))
                        .unwrap_or_else(|| "User".to_string());

                    tracing::info!(
                        "Parsed names - first_name: {}, last_name: {}",
                        first_name,
                        last_name
                    );

                    // Create new user
                    let new_user = OAuthUserCreate {
                        email: firebase_user
                            .email
                            .clone()
                            .unwrap_or_else(|| format!("{}@firebase.local", firebase_user.uid)),
                        first_name,
                        last_name,
                        provider_id: firebase_user.uid.clone(),
                        provider_name: "firebase".to_string(),
                        avatar_url: firebase_user.picture.clone(),
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
                                Json(FirebaseAuthResponse {
                                    success: false,
                                    data: None,
                                    message: Some("Failed to create user account".to_string()),
                                }),
                            )
                                .into_response();
                        }
                    }
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
                Json(FirebaseAuthResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to get user permissions".to_string()),
                }),
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
            tracing::error!("Failed to generate JWT token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FirebaseAuthResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to generate authentication token".to_string()),
                }),
            )
                .into_response();
        }
    };

    let response = FirebaseAuthResponse {
        success: true,
        data: Some(AuthTokenData {
            token: jwt_token,
            user: UserInfo {
                id: user.id.to_string(),
                email: user.email,
                name: format!("{} {}", user.first_name, user.last_name),
                picture: user.avatar_url,
            },
        }),
        message: None,
    };

    (StatusCode::OK, Json(response)).into_response()
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
