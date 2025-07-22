use crate::{
    application::{
        http_response::{
            internal_server_error_json_response, json_response, row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::{interface::boat_repository::BoatRepository, models::boat::BoatUpdate},
    infrastructure::repositories::sqlx_boat_repository::SqlxBoatRepository,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

pub async fn update_boat_command(
    State(app_state): State<AppState>,
    Path(boat_id): Path<Uuid>,
    Json(boat_update): Json<BoatUpdate>,
) -> impl IntoResponse {
    // Validate the boat_update data
    match boat_update.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };

    let repository = SqlxBoatRepository;
    match repository
        .update(&app_state.db_pool, boat_id, boat_update)
        .await
    {
        Ok(boat) => json_response(StatusCode::OK, json!({ "success": true, "data": boat })),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Boat not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
