use crate::{
    application::{
        http_response::{internal_server_error_json_response, json_response},
        state::AppState,
    },
    domain::{
        interface::boat_repository::BoatRepository,
        models::{auth::AuthContext, boat::BoatCreate},
        repositories::boat_owner_repository::BoatOwnerRepository,
    },
    infrastructure::repositories::sqlx_boat_repository::SqlxBoatRepository,
};
use axum::{
    Extension,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use validator::Validate;

pub async fn create_user_boat_command(
    State(app_state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
    Json(boat_create): Json<BoatCreate>,
) -> impl IntoResponse {
    // Validate the boat_create data
    if let Err(e) = boat_create.validate() {
        return json_response(
            StatusCode::BAD_REQUEST,
            json!({ "success": false, "message": e }),
        );
    }

    let boat_repository = SqlxBoatRepository;
    let owner_repository = BoatOwnerRepository::new(&app_state.db_pool);

    // Create the boat
    match boat_repository
        .insert(&app_state.db_pool, boat_create)
        .await
    {
        Ok(boat) => {
            // Assign ownership to the creating user
            if let Err(e) = owner_repository
                .add_owner_to_boat(boat.id, auth_context.user.id)
                .await
            {
                // If ownership assignment fails, we should probably delete the boat
                // For now, just log the error and continue
                eprintln!("Failed to assign boat ownership: {}", e);
            }

            json_response(
                StatusCode::CREATED,
                json!({ "success": true, "data": boat }),
            )
        }
        Err(e) => internal_server_error_json_response(e),
    }
}
