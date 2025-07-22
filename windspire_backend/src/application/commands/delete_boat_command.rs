use crate::{
    application::{
        http_response::{
            internal_server_error_json_response, json_response, row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::interface::boat_repository::BoatRepository,
    infrastructure::repositories::sqlx_boat_repository::SqlxBoatRepository,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

pub async fn delete_boat_command(
    State(app_state): State<AppState>,
    Path(boat_id): Path<Uuid>,
) -> impl IntoResponse {
    let repository = SqlxBoatRepository;
    match repository.delete(&app_state.db_pool, boat_id).await {
        Ok(_) => json_response(
            StatusCode::OK,
            json!({ "success": true, "message": "Boat deleted successfully" }),
        ),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Boat not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
