use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    application::{
        http_response::{
            bad_request_error_json_response, internal_server_error_json_response, json_response,
            row_not_found_error_json_response,
        },
        state::AppState,
    },
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_country_by_code_query(
    State(app_state): State<AppState>,
    Path(country_code): Path<String>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository
        .get_country_by_code(&app_state.db_pool, country_code)
        .await
    {
        Ok(country) => json_response(StatusCode::OK, json!({ "success": true, "data": country })),
        Err(sqlx::Error::ColumnNotFound(msg)) => bad_request_error_json_response(msg),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Country not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
