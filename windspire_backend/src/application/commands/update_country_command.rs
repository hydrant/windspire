use crate::{
    application::{
        http_response::{
            internal_server_error_json_response, json_response, row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::{interface::country_repository::CountryRepository, models::country::CountryUpdate},
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use uuid::Uuid;
use validator::Validate;

pub async fn update_country_command(
    State(app_state): State<AppState>,
    Path(country_id): Path<Uuid>,
    Json(country_update): Json<CountryUpdate>,
) -> impl IntoResponse {
    // Validate the country_update data
    match country_update.validate() {
        Ok(_) => (),
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": e }),
            );
        }
    };

    let repository = SqlxCountryRepository;
    match repository
        .update_country(&app_state.db_pool, country_id, country_update)
        .await
    {
        Ok(country) => json_response(StatusCode::OK, json!({ "success": true, "data": country })),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Country not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
