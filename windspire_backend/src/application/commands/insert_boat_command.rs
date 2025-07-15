use crate::{
    application::{
        http_response::{internal_server_error_json_response, json_response},
        state::AppState,
    },
    domain::{interface::boat_repository::BoatRepository, models::boat::BoatCreate},
    infrastructure::repositories::sqlx_boat_repository::SqlxBoatRepository,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use validator::Validate;

pub async fn insert_boat_command(
    State(app_state): State<AppState>,
    Json(boat_create): Json<BoatCreate>,
) -> impl IntoResponse {
    // Validate the user_create data
    match boat_create.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };

    let repository = SqlxBoatRepository;
    match repository.insert(&app_state.db_pool, boat_create).await {
        Ok(users) => json_response(StatusCode::OK, json!({ "success": true, "data": users })),
        Err(e) => internal_server_error_json_response(e),
    }
}
