use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    application::{
        http_response::{
            internal_server_error_json_response, json_response, row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::repositories::{
        boat_owner_repository::BoatOwnerRepository, user_repository::UserRepository,
    },
    infrastructure::repositories::sqlx_user_repository::SqlxUserRepository,
};

pub async fn get_user_profile_query(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let user_repository = SqlxUserRepository;
    let boat_repository = BoatOwnerRepository::new(&app_state.db_pool);

    // Get user data
    let user = match user_repository
        .get_user_by_id(&app_state.db_pool, user_id)
        .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return row_not_found_error_json_response("User not found");
        }
        Err(err) => return internal_server_error_json_response(err),
    };

    // Get user's boats
    let boats = match boat_repository
        .get_boats_with_details_for_user(user_id)
        .await
    {
        Ok(boats) => boats,
        Err(err) => {
            // Log error but don't fail the whole request if boats can't be loaded
            eprintln!("Failed to load boats for user {}: {}", user_id, err);
            Vec::new()
        }
    };

    let user_profile = json!({
        "user": user,
        "boats": boats,
        "boat_count": boats.len()
    });

    json_response(
        StatusCode::OK,
        json!({ "success": true, "data": user_profile }),
    )
}
