use crate::{
    application::{
        http_response::{internal_server_error_json_response, json_response},
        state::AppState,
    },
    domain::{models::auth::AuthContext, repositories::boat_owner_repository::BoatOwnerRepository},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};
use serde_json::json;

pub async fn get_my_boats_query(
    State(app_state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
) -> impl IntoResponse {
    let repo = BoatOwnerRepository::new(&app_state.db_pool);

    match repo
        .get_boats_with_details_for_user(auth_context.user.id)
        .await
    {
        Ok(boats) => json_response(StatusCode::OK, json!({ "success": true, "data": boats })),
        Err(e) => internal_server_error_json_response(e),
    }
}
