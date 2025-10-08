use crate::application::state::AppState;
use crate::domain::models::boat::Boat;
use crate::domain::models::user::UserWithCountry;
use crate::domain::repositories::boat_owner_repository::BoatOwnerRepository;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json;
use uuid::Uuid;

pub async fn add_owner_to_boat(
    State(state): State<AppState>,
    Path((boat_id, user_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let repo = BoatOwnerRepository::new(&state.db_pool);
    match repo.add_owner_to_boat(boat_id, user_id).await {
        Ok(()) => {
            let response = serde_json::json!({
                "success": true,
                "message": "Owner added successfully"
            });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = serde_json::json!({
                "success": false,
                "message": e.to_string()
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn remove_owner_from_boat(
    State(state): State<AppState>,
    Path((boat_id, user_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let repo = BoatOwnerRepository::new(&state.db_pool);
    match repo.remove_owner_from_boat(boat_id, user_id).await {
        Ok(()) => {
            let response = serde_json::json!({
                "success": true,
                "message": "Owner removed successfully"
            });
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = serde_json::json!({
                "success": false,
                "message": e.to_string()
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_boats_for_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = BoatOwnerRepository::new(&state.db_pool);
    let boats = repo
        .get_boats_with_details_for_user(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok::<Json<Vec<Boat>>, (StatusCode, String)>(Json(boats))
}

pub async fn get_owners_for_boat(
    State(state): State<AppState>,
    Path(boat_id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = BoatOwnerRepository::new(&state.db_pool);
    let owners = repo
        .get_owners_with_details_for_boat(boat_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok::<Json<Vec<UserWithCountry>>, (StatusCode, String)>(Json(owners))
}
