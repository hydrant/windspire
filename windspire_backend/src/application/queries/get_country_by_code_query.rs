use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    application::http_response::{
        bad_request_error_json_response, internal_server_error_json_response, json_response,
        row_not_found_error_json_response,
    },
    domain::interface::country_repository::CountryRepository,
    infrastructure::repositories::sqlx_country_repository::SqlxCountryRepository,
};

pub async fn get_country_by_code_query(
    State(pg_pool): State<PgPool>,
    Path(country_code): Path<String>,
) -> impl IntoResponse {
    let repository = SqlxCountryRepository;
    match repository.get_country_by_code(&pg_pool, country_code).await {
        Ok(country) => json_response(StatusCode::OK, json!({ "success": true, "data": country })),
        Err(sqlx::Error::ColumnNotFound(msg)) => bad_request_error_json_response(msg),
        Err(sqlx::Error::RowNotFound) => row_not_found_error_json_response("Country not found"),
        Err(e) => internal_server_error_json_response(e),
    }
}
